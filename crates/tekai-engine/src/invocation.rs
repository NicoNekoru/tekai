//! Argument ownership for the process-isolated engine entry.
//!
//! Several generated globals retain pointers into the argument buffers after
//! normal completion. Keep the one accepted argument set for the rest of the
//! process until those aliases and the engine's other allocations have owners.

use std::ffi::{c_char, c_int};
use std::fmt;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};

/// The result of the engine's normal completion path.
///
/// Fatal engine paths still terminate the process and do not return this value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EngineCompletion {
    Clean,
    Warnings,
    Errors,
}

impl EngineCompletion {
    pub(crate) fn from_history(history: u8) -> Self {
        match history {
            0 => Self::Clean,
            1 => Self::Warnings,
            _ => Self::Errors,
        }
    }

    /// Preserve the CLI's existing success status for warnings.
    pub fn exit_code(self) -> c_int {
        match self {
            Self::Clean | Self::Warnings => 0,
            Self::Errors => 1,
        }
    }
}

/// A failure before the engine's initialization or reset routines run.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InvocationError {
    MissingProgramName,
    TooManyArguments { count: usize },
    ArgumentContainsNul { index: usize, position: usize },
    AlreadyInvoked,
}

impl fmt::Display for InvocationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingProgramName => {
                formatter.write_str("engine arguments require a program name")
            }
            Self::TooManyArguments { count } => {
                write!(
                    formatter,
                    "engine argument count {count} exceeds its supported range"
                )
            }
            Self::ArgumentContainsNul { index, position } => {
                write!(
                    formatter,
                    "engine argument {index} contains NUL at byte {position}"
                )
            }
            Self::AlreadyInvoked => formatter.write_str("engine already entered in this process"),
        }
    }
}

impl std::error::Error for InvocationError {}

fn checked_count(count: usize) -> Result<c_int, InvocationError> {
    if count == 0 {
        return Err(InvocationError::MissingProgramName);
    }
    c_int::try_from(count).map_err(|_| InvocationError::TooManyArguments { count })
}

fn validate_arguments(arguments: &[Vec<u8>]) -> Result<c_int, InvocationError> {
    let argc = checked_count(arguments.len())?;
    for (index, argument) in arguments.iter().enumerate() {
        if let Some(position) = argument.iter().position(|&byte| byte == 0) {
            return Err(InvocationError::ArgumentContainsNul { index, position });
        }
    }
    Ok(argc)
}

struct OwnedArguments {
    argc: c_int,
    // getopt may reorder or replace argv entries. Own the byte buffers
    // separately so that the table never becomes their deallocation authority.
    _storage: Vec<Vec<u8>>,
    argv: Vec<*mut c_char>,
}

impl OwnedArguments {
    fn from_validated(arguments: Vec<Vec<u8>>, argc: c_int) -> Self {
        let mut storage = arguments
            .into_iter()
            .map(|mut argument| {
                argument.push(0);
                argument
            })
            .collect::<Vec<_>>();
        let argv = storage
            .iter_mut()
            .map(|argument| argument.as_mut_ptr().cast::<c_char>())
            .chain(std::iter::once(ptr::null_mut()))
            .collect::<Vec<_>>();
        Self {
            argc,
            _storage: storage,
            argv,
        }
    }

    fn engine_args(&mut self) -> (c_int, *mut *mut c_char) {
        (self.argc, self.argv.as_mut_ptr())
    }
}

struct EntryLatch(AtomicBool);

impl EntryLatch {
    const fn new() -> Self {
        Self(AtomicBool::new(false))
    }

    fn prepare(&self, arguments: Vec<Vec<u8>>) -> Result<OwnedArguments, InvocationError> {
        // Reject repeats before validation or allocating engine-owned storage.
        if self.0.load(Ordering::Acquire) {
            return Err(InvocationError::AlreadyInvoked);
        }
        let argc = validate_arguments(&arguments)?;
        self.0
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| InvocationError::AlreadyInvoked)?;
        // The latch is permanent even if initialization never reaches normal
        // completion. Unwinding does not make the global engine reusable.
        Ok(OwnedArguments::from_validated(arguments, argc))
    }
}

static ENTRY_LATCH: EntryLatch = EntryLatch::new();
static mut RETAINED_ARGUMENTS: Option<OwnedArguments> = None;

pub(crate) unsafe fn retain_for_entry(
    arguments: Vec<Vec<u8>>,
) -> Result<(c_int, *mut *mut c_char), InvocationError> {
    let mut owner = ENTRY_LATCH.prepare(arguments)?;
    let engine_args = owner.engine_args();
    // SAFETY: the permanent latch admits exactly one writer. No code reads,
    // replaces, or drops this slot. Neither the buffers nor the argv table grow
    // after construction. Moving their Vec owners preserves the allocations,
    // including pointers retained by generated globals.
    unsafe {
        ptr::write(&raw mut RETAINED_ARGUMENTS, Some(owner));
    }
    Ok(engine_args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::ffi::CStr;
    use std::sync::{Arc, Barrier};

    #[test]
    fn invalid_inputs_do_not_claim_entry() {
        let latch = EntryLatch::new();
        assert!(matches!(
            latch.prepare(Vec::new()),
            Err(InvocationError::MissingProgramName)
        ));
        assert!(matches!(
            latch.prepare(vec![b"pdflatex".to_vec(), b"bad\0argument".to_vec()]),
            Err(InvocationError::ArgumentContainsNul {
                index: 1,
                position: 3,
            })
        ));
        assert!(!latch.0.load(Ordering::Acquire));
        assert!(latch.prepare(vec![b"pdflatex".to_vec()]).is_ok());
    }

    #[test]
    fn repeat_entry_cannot_replace_argument_backing() {
        let latch = EntryLatch::new();
        let mut retained = Some(
            latch
                .prepare(vec![b"pdflatex".to_vec(), b"--fmt=latex".to_vec()])
                .unwrap(),
        );
        let (argc, argv) = retained.as_mut().unwrap().engine_args();
        // SAFETY: this pointer is within the retained second argument buffer.
        let borrowed_format = unsafe { (*argv.add(1)).add(6) };
        for _ in 0..8 {
            assert!(matches!(
                latch.prepare(vec![b"replacement\0".to_vec()]),
                Err(InvocationError::AlreadyInvoked)
            ));
            assert_eq!(retained.as_mut().unwrap().engine_args(), (argc, argv));
        }
        // SAFETY: the retained owner still holds the NUL-terminated buffer.
        assert_eq!(
            unsafe { CStr::from_ptr(borrowed_format) }.to_bytes(),
            b"latex"
        );
    }

    #[test]
    fn argv_mutations_do_not_change_buffer_ownership() {
        let latch = EntryLatch::new();
        let mut owner = latch
            .prepare(vec![b"pdflatex".to_vec(), b"input.tex".to_vec()])
            .unwrap();
        let original_buffers = owner
            ._storage
            .iter()
            .map(|buffer| buffer.as_ptr())
            .collect::<Vec<_>>();
        let (_, argv) = owner.engine_args();
        // SAFETY: the owned table has two non-null entries and a sentinel.
        // Generated filename normalization can replace an entry in this way.
        unsafe {
            ptr::swap(argv, argv.add(1));
            *argv.add(1) = c"normalized.tex".as_ptr().cast_mut();
        }
        assert_eq!(
            owner
                ._storage
                .iter()
                .map(|buffer| buffer.as_ptr())
                .collect::<Vec<_>>(),
            original_buffers
        );
        drop(owner);
    }

    #[test]
    fn concurrent_claims_admit_one_mutation() {
        let latch = Arc::new(EntryLatch::new());
        let start = Arc::new(Barrier::new(8));
        let mutations = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let threads = (0..8)
            .map(|_| {
                let latch = Arc::clone(&latch);
                let start = Arc::clone(&start);
                let mutations = Arc::clone(&mutations);
                std::thread::spawn(move || {
                    start.wait();
                    if latch.prepare(vec![b"pdflatex".to_vec()]).is_ok() {
                        mutations.fetch_add(1, Ordering::Relaxed);
                    }
                })
            })
            .collect::<Vec<_>>();
        for thread in threads {
            thread.join().unwrap();
        }
        assert_eq!(mutations.load(Ordering::Relaxed), 1);
        assert!(matches!(
            latch.prepare(vec![b"pdflatex".to_vec()]),
            Err(InvocationError::AlreadyInvoked)
        ));
    }

    proptest! {
        #[test]
        fn checked_count_never_truncates(count in any::<usize>()) {
            match checked_count(count) {
                Ok(argc) => {
                    prop_assert!(count > 0);
                    prop_assert_eq!(usize::try_from(argc).unwrap(), count);
                }
                Err(InvocationError::MissingProgramName) => prop_assert_eq!(count, 0),
                Err(InvocationError::TooManyArguments { count: rejected }) => {
                    prop_assert_eq!(rejected, count);
                    prop_assert!(count > c_int::MAX as usize);
                }
                Err(other) => prop_assert!(false, "unexpected count error {other:?}"),
            }
        }

        #[test]
        fn owned_arguments_preserve_bytes_across_moves(
            arguments in prop::collection::vec(
                prop::collection::vec(1u8..=u8::MAX, 0..64),
                1..32,
            ),
        ) {
            let expected = arguments.clone();
            let latch = EntryLatch::new();
            let mut owner = latch.prepare(arguments).unwrap();
            let (argc, argv) = owner.engine_args();
            let retained = Some(owner);
            prop_assert_eq!(argc as usize, expected.len());
            for (index, bytes) in expected.iter().enumerate() {
                // SAFETY: moving the owner does not move its Vec allocations.
                let actual = unsafe { CStr::from_ptr(*argv.add(index)) }.to_bytes();
                prop_assert_eq!(actual, bytes);
            }
            // SAFETY: the table includes the sentinel after its argc entries.
            prop_assert!(unsafe { (*argv.add(expected.len())).is_null() },
                "argv must end with a null pointer");
            drop(retained);
        }

        #[test]
        fn nul_rejection_precedes_claim(
            prefix in prop::collection::vec(1u8..=u8::MAX, 0..64),
            suffix in prop::collection::vec(any::<u8>(), 0..64),
        ) {
            let position = prefix.len();
            let mut invalid = prefix;
            invalid.push(0);
            invalid.extend(suffix);
            let latch = EntryLatch::new();
            prop_assert!(matches!(
                latch.prepare(vec![b"pdflatex".to_vec(), invalid]),
                Err(InvocationError::ArgumentContainsNul { index: 1, position: found })
                    if found == position
            ), "the first NUL must be rejected at its exact byte position");
            prop_assert!(!latch.0.load(Ordering::Acquire));
        }

        #[test]
        fn completion_preserves_legacy_exit_status(history in any::<u8>()) {
            let completion = EngineCompletion::from_history(history);
            prop_assert_eq!(completion.exit_code(), if history <= 1 { 0 } else { 1 });
            if history == 0 {
                prop_assert_eq!(completion, EngineCompletion::Clean);
            } else if history == 1 {
                prop_assert_eq!(completion, EngineCompletion::Warnings);
            } else {
                prop_assert_eq!(completion, EngineCompletion::Errors);
            }
        }
    }
}
