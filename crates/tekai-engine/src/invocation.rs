//! Invocation configuration and argument ownership.

use std::ffi::{c_char, c_int, OsString};
use std::fmt;
use std::path::PathBuf;
use std::ptr;

/// Per-call settings that do not mutate the host's directory or environment.
#[derive(Clone, Debug, Default)]
pub struct InvocationOptions {
    /// Relative paths are resolved against the host directory at entry.
    pub working_directory: Option<PathBuf>,
    /// Apply these overrides to the invocation's parent-environment snapshot.
    /// A `None` value removes the variable for this call only.
    pub environment: Vec<(OsString, Option<OsString>)>,
    pub interrupt: crate::InterruptToken,
    /// Use `SupervisedProcess` only for an isolated engine worker whose parent
    /// owns its whole process group and stops descendants before reaping it.
    pub pipe_process_policy: crate::PipeProcessPolicy,
}

/// The result of the engine's normal completion path.
///
/// Fatal diagnostic exits return Errors. Help/version exits return Clean.
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

/// A failure while preparing the invocation or tearing down its streams.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InvocationError {
    MissingProgramName,
    TooManyArguments {
        count: usize,
    },
    ArgumentContainsNul {
        index: usize,
        position: usize,
    },
    NestedInvocation,
    ResourceSetup {
        resource: &'static str,
        message: String,
    },
    StreamTeardown {
        message: String,
    },
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
            Self::NestedInvocation => {
                formatter.write_str("nested engine invocation on this thread")
            }
            Self::ResourceSetup { resource, message } => {
                write!(formatter, "engine {resource} setup failed: {message}")
            }
            Self::StreamTeardown { message } => {
                write!(formatter, "engine stream teardown failed: {message}")
            }
        }
    }
}

impl std::error::Error for InvocationError {}

pub(crate) fn resource<T, E: fmt::Display>(
    resource: &'static str,
    result: Result<T, E>,
) -> Result<T, InvocationError> {
    result.map_err(|error| InvocationError::ResourceSetup {
        resource,
        message: error.to_string(),
    })
}

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

pub(crate) struct OwnedArguments {
    argc: c_int,
    // getopt may reorder or replace argv entries. Own the byte buffers
    // separately so that the table never becomes their deallocation authority.
    _storage: Vec<Vec<u8>>,
    argv: Vec<*mut c_char>,
}

impl OwnedArguments {
    pub(crate) fn new(arguments: Vec<Vec<u8>>) -> Result<Self, InvocationError> {
        let argc = validate_arguments(&arguments)?;
        Ok(Self::from_validated(arguments, argc))
    }

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

    pub(crate) fn engine_args(&mut self) -> (c_int, *mut *mut c_char) {
        (self.argc, self.argv.as_mut_ptr())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::ffi::CStr;

    #[test]
    fn invalid_inputs_do_not_affect_later_arguments() {
        assert!(matches!(
            OwnedArguments::new(Vec::new()),
            Err(InvocationError::MissingProgramName)
        ));
        assert!(matches!(
            OwnedArguments::new(vec![b"pdflatex".to_vec(), b"bad\0argument".to_vec()]),
            Err(InvocationError::ArgumentContainsNul {
                index: 1,
                position: 3,
            })
        ));
        assert!(OwnedArguments::new(vec![b"pdflatex".to_vec()]).is_ok());
    }

    #[test]
    fn independent_arguments_cannot_replace_argument_backing() {
        let mut retained =
            Some(OwnedArguments::new(vec![b"pdflatex".to_vec(), b"--fmt=latex".to_vec()]).unwrap());
        let (argc, argv) = retained.as_mut().unwrap().engine_args();
        // SAFETY: this pointer is within the retained second argument buffer.
        let borrowed_format = unsafe { (*argv.add(1)).add(6) };
        for _ in 0..8 {
            let _other = OwnedArguments::new(vec![b"replacement".to_vec()]).unwrap();
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
        let mut owner =
            OwnedArguments::new(vec![b"pdflatex".to_vec(), b"input.tex".to_vec()]).unwrap();
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
            let mut owner = OwnedArguments::new(arguments).unwrap();
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
        fn nul_rejection_reports_exact_position(
            prefix in prop::collection::vec(1u8..=u8::MAX, 0..64),
            suffix in prop::collection::vec(any::<u8>(), 0..64),
        ) {
            let position = prefix.len();
            let mut invalid = prefix;
            invalid.push(0);
            invalid.extend(suffix);
            prop_assert!(matches!(
                OwnedArguments::new(vec![b"pdflatex".to_vec(), invalid]),
                Err(InvocationError::ArgumentContainsNul { index: 1, position: found })
                    if found == position
            ), "the first NUL must be rejected at its exact byte position");
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
