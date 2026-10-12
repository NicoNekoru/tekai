//! Invocation-owned parent snapshot, environment overlay, and cnf overrides.
//!
//! Host environment variables are never changed. Legacy C-string views retain
//! their owned historical payload until the private invocation scope ends.
//! Safe reads outside an invocation fall back to the read-only parent state.

#![warn(clippy::all)]

use std::cell::Cell;
use std::collections::HashMap;
use std::ffi::{c_char, CStr, CString, OsStr, OsString};
use std::fmt;
use std::marker::{PhantomData, PhantomPinned};
use std::os::unix::ffi::OsStrExt;
use std::pin::Pin;
use std::process::Command;
use std::ptr::{self, NonNull};
use std::rc::Rc;

const MAX_STORAGE: usize = 32 * 1024 * 1024;
const MAX_ENTRIES: usize = 65_536;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EnvironmentError {
    Missing,
    Nested,
    Busy,
    InvalidName,
    NulValue,
    ResourceLimit,
    Allocation,
}

impl fmt::Display for EnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Missing => "no active environment invocation scope",
            Self::Nested => "nested environment invocation scope",
            Self::Busy => "recursive environment state access",
            Self::InvalidName => "invalid environment variable name",
            Self::NulValue => "NUL in environment variable value",
            Self::ResourceLimit => "invocation environment exceeds its storage limits",
            Self::Allocation => "unable to allocate invocation environment storage",
        })
    }
}

impl std::error::Error for EnvironmentError {}

fn valid_name(name: &OsStr) -> bool {
    !name.is_empty() && !name.as_bytes().iter().any(|&b| b == 0 || b == b'=')
}

struct Overlay {
    values: HashMap<OsString, OsString>,
    cnf: HashMap<OsString, OsString>,
    views: HashMap<OsString, usize>,
    retained: Vec<CString>,
    stored_bytes: usize,
}

impl Overlay {
    fn new() -> Result<Self, EnvironmentError> {
        let mut overlay = Self {
            values: HashMap::new(),
            cnf: HashMap::new(),
            views: HashMap::new(),
            retained: Vec::new(),
            stored_bytes: 0,
        };
        for (key, value) in std::env::vars_os() {
            overlay.set(&key, &value, false)?;
        }
        Ok(overlay)
    }

    fn set(&mut self, name: &OsStr, value: &OsStr, cnf: bool) -> Result<(), EnvironmentError> {
        if !valid_name(name) {
            return Err(EnvironmentError::InvalidName);
        }
        if value.as_bytes().contains(&0) {
            return Err(EnvironmentError::NulValue);
        }
        let entries = self.values.len() + self.cnf.len();
        let values = if cnf { &mut self.cnf } else { &mut self.values };
        if values.get(name).is_some_and(|old| old == value) {
            return Ok(());
        }
        if !values.contains_key(name) && entries >= MAX_ENTRIES {
            return Err(EnvironmentError::ResourceLimit);
        }
        let old_size = values.get(name).map_or(0, |old| name.len() + old.len());
        let new_size = name
            .len()
            .checked_add(value.len())
            .ok_or(EnvironmentError::ResourceLimit)?;
        let stored = self
            .stored_bytes
            .checked_sub(old_size)
            .and_then(|size| size.checked_add(new_size))
            .filter(|&size| size <= MAX_STORAGE)
            .ok_or(EnvironmentError::ResourceLimit)?;
        values
            .try_reserve(1)
            .map_err(|_| EnvironmentError::Allocation)?;
        values.insert(name.to_owned(), value.to_owned());
        self.stored_bytes = stored;
        if !cnf {
            self.views.remove(name);
        }
        Ok(())
    }

    fn view(&mut self, name: &OsStr) -> Result<*mut c_char, EnvironmentError> {
        if !valid_name(name) {
            return Ok(ptr::null_mut());
        }
        if let Some(&index) = self.views.get(name) {
            return Ok(self.retained[index].as_ptr().cast_mut());
        }
        let Some(value) = self.values.get(name) else {
            return Ok(ptr::null_mut());
        };
        if self.retained.len() >= MAX_ENTRIES {
            return Err(EnvironmentError::ResourceLimit);
        }
        let bytes = value.as_bytes();
        // Also account for the copied lookup key. Old payloads stay alive after
        // set(), even though only the newest view has a lookup-table entry.
        let stored = self
            .stored_bytes
            .checked_add(bytes.len())
            .and_then(|size| size.checked_add(name.len() + 1))
            .filter(|&size| size <= MAX_STORAGE)
            .ok_or(EnvironmentError::ResourceLimit)?;
        let mut terminated = Vec::new();
        terminated
            .try_reserve_exact(bytes.len() + 1)
            .map_err(|_| EnvironmentError::Allocation)?;
        terminated.extend_from_slice(bytes);
        terminated.push(0);
        let view =
            CString::from_vec_with_nul(terminated).map_err(|_| EnvironmentError::NulValue)?;
        self.retained
            .try_reserve(1)
            .map_err(|_| EnvironmentError::Allocation)?;
        self.views
            .try_reserve(1)
            .map_err(|_| EnvironmentError::Allocation)?;
        let pointer = view.as_ptr().cast_mut();
        self.views.insert(name.to_owned(), self.retained.len());
        self.retained.push(view);
        self.stored_bytes = stored;
        Ok(pointer)
    }
}

pub(crate) struct EnvironmentSession {
    overlay: Overlay,
    _thread_confined: PhantomData<Rc<()>>,
    _pinned: PhantomPinned,
}

struct Active {
    pointer: Cell<Option<NonNull<EnvironmentSession>>>,
    busy: Cell<bool>,
}

thread_local! {
    static ACTIVE: Active = const { Active { pointer: Cell::new(None), busy: Cell::new(false) } };
}

struct ScopeGuard(NonNull<EnvironmentSession>);
impl Drop for ScopeGuard {
    fn drop(&mut self) {
        let _ = ACTIVE.try_with(|active| {
            if active.pointer.get() == Some(self.0) {
                active.pointer.set(None);
            }
        });
    }
}

struct BusyGuard<'a>(&'a Cell<bool>);
impl Drop for BusyGuard<'_> {
    fn drop(&mut self) {
        self.0.set(false);
    }
}

impl EnvironmentSession {
    pub(crate) fn new() -> Result<Pin<Box<Self>>, EnvironmentError> {
        Ok(Box::pin(Self {
            overlay: Overlay::new()?,
            _thread_confined: PhantomData,
            _pinned: PhantomPinned,
        }))
    }

    pub(crate) fn scope<T>(
        session: Pin<&mut Self>,
        action: impl FnOnce() -> T,
    ) -> Result<T, EnvironmentError> {
        let pointer = ACTIVE.with(|active| {
            if active.busy.get() {
                return Err(EnvironmentError::Busy);
            }
            if active.pointer.get().is_some() {
                return Err(EnvironmentError::Nested);
            }
            // SAFETY The exclusive pinned borrow lasts through action and the
            // private guard. No safe caller can retain the unregister guard.
            let pointer = NonNull::from(unsafe { session.get_unchecked_mut() });
            active.pointer.set(Some(pointer));
            Ok(pointer)
        })?;
        let _scope = ScopeGuard(pointer);
        Ok(action())
    }
}

fn with_overlay<T>(action: impl FnOnce(&mut Overlay) -> T) -> Result<T, EnvironmentError> {
    ACTIVE.with(|active| {
        if active.busy.get() {
            return Err(EnvironmentError::Busy);
        }
        let mut pointer = active.pointer.get().ok_or(EnvironmentError::Missing)?;
        active.busy.set(true);
        let _busy = BusyGuard(&active.busy);
        // SAFETY The private scope owns the thread-confined pinned borrow, and
        // the busy guard excludes any recursive alias of this mutable state.
        Ok(action(unsafe { &mut pointer.as_mut().overlay }))
    })
}

pub(crate) fn report(error: &impl fmt::Display) {
    use std::io::Write;
    let _ = writeln!(std::io::stderr().lock(), "tekai environment: {error}");
}

pub(crate) fn var_os(name: impl AsRef<OsStr>) -> Option<OsString> {
    let name = name.as_ref();
    match with_overlay(|overlay| overlay.values.get(name).cloned()) {
        Ok(value) => value,
        Err(EnvironmentError::Missing) => std::env::var_os(name),
        Err(error) => {
            report(&error);
            None
        }
    }
}

pub(crate) fn var(name: &str) -> Result<String, std::env::VarError> {
    var_os(name)
        .ok_or(std::env::VarError::NotPresent)?
        .into_string()
        .map_err(std::env::VarError::NotUnicode)
}

pub(crate) fn cnf_value(name: impl AsRef<OsStr>) -> Option<OsString> {
    match with_overlay(|overlay| overlay.cnf.get(name.as_ref()).cloned()) {
        Ok(value) => value,
        Err(EnvironmentError::Missing) => None,
        Err(error) => {
            report(&error);
            None
        }
    }
}

pub(crate) fn set(name: &OsStr, value: &OsStr) -> Result<(), EnvironmentError> {
    with_overlay(|overlay| overlay.set(name, value, false))?
}

pub(crate) fn set_cnf(name: &OsStr, value: &OsStr) -> Result<(), EnvironmentError> {
    with_overlay(|overlay| overlay.set(name, value, true))?
}

pub(crate) fn configure_command(command: &mut Command) -> Result<(), EnvironmentError> {
    match with_overlay(|overlay| {
        command.env_clear().envs(&overlay.values);
    }) {
        Err(EnvironmentError::Missing) => Ok(()),
        result => result,
    }
}

/// # Safety
/// name must be a terminated C string. Returned pointers are read-only legacy
/// views valid until the active EnvironmentSession owner is dropped. A scope
/// must be active, and generated callers must not modify or free these views.
pub(crate) unsafe fn getenv(name: *const c_char) -> *mut c_char {
    if name.is_null() {
        return ptr::null_mut();
    }
    let name = OsStr::from_bytes(unsafe { CStr::from_ptr(name) }.to_bytes());
    match with_overlay(|overlay| overlay.view(name)) {
        Ok(Ok(pointer)) => pointer,
        Ok(Err(error)) | Err(error) => {
            report(&error);
            ptr::null_mut()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::ffi::OsStringExt;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    fn scoped(action: impl FnOnce()) {
        let mut session = EnvironmentSession::new().unwrap();
        EnvironmentSession::scope(session.as_mut(), action).unwrap();
    }

    #[test]
    fn two_invocations_do_not_leak_overlay_or_cnf_into_parent_or_each_other() {
        let before = std::env::var_os("HOME");
        for value in ["first", "second"] {
            scoped(|| {
                assert_eq!(var_os("HOME"), before);
                assert_eq!(cnf_value("shell_escape"), None);
                set(OsStr::new("HOME"), OsStr::new(value)).unwrap();
                set_cnf(OsStr::new("shell_escape"), OsStr::new(value)).unwrap();
                assert_eq!(var_os("HOME"), Some(value.into()));
                assert_eq!(cnf_value("shell_escape"), Some(value.into()));
                assert_eq!(std::env::var_os("HOME"), before);
            });
        }
        assert_eq!(var_os("HOME"), before);
        assert_eq!(std::env::var_os("HOME"), before);
        assert_eq!(cnf_value("shell_escape"), None);
    }

    #[test]
    fn historical_views_survive_updates_and_non_utf8_values() {
        scoped(|| {
            let name = OsStr::from_bytes(b"TEKAI_RAW_\xff");
            let first = OsStr::from_bytes(b"first\xff");
            set(name, first).unwrap();
            let c_name = CString::new(name.as_bytes()).unwrap();
            let old = unsafe { getenv(c_name.as_ptr()) };
            assert_eq!(unsafe { CStr::from_ptr(old) }.to_bytes(), first.as_bytes());
            set(name, OsStr::from_bytes(b"second\x80")).unwrap();
            let new = unsafe { getenv(c_name.as_ptr()) };
            assert_ne!(new, old);
            assert_eq!(unsafe { CStr::from_ptr(old) }.to_bytes(), first.as_bytes());
            assert_eq!(unsafe { CStr::from_ptr(new) }.to_bytes(), b"second\x80");
            assert_eq!(
                var_os(name),
                Some(OsString::from_vec(b"second\x80".to_vec()))
            );
            set(OsStr::new("TEKAI_UTF8_TEST"), first).unwrap();
            assert!(matches!(
                var("TEKAI_UTF8_TEST"),
                Err(std::env::VarError::NotUnicode(_))
            ));
        });
    }

    #[test]
    fn unwind_unregisters_without_changing_parent() {
        let before = std::env::var_os("HOME");
        let unwind = catch_unwind(AssertUnwindSafe(|| {
            scoped(|| {
                set(OsStr::new("HOME"), OsStr::new("unwound")).unwrap();
                panic!("scope failed");
            })
        }));
        assert!(unwind.is_err());
        assert_eq!(var_os("HOME"), before);
        assert_eq!(
            set(OsStr::new("HOME"), OsStr::new("absent")),
            Err(EnvironmentError::Missing)
        );
    }

    #[test]
    fn concurrent_threads_have_independent_parent_snapshots_and_overrides() {
        let before = std::env::var_os("HOME");
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
        let threads = ["thread-one", "thread-two"].map(|value| {
            let barrier = barrier.clone();
            let before = before.clone();
            std::thread::spawn(move || {
                scoped(|| {
                    assert_eq!(var_os("HOME"), before);
                    set(OsStr::new("HOME"), OsStr::new(value)).unwrap();
                    barrier.wait();
                    assert_eq!(var_os("HOME"), Some(value.into()));
                    assert_eq!(std::env::var_os("HOME"), before);
                })
            })
        });
        for thread in threads {
            thread.join().unwrap();
        }
        assert_eq!(std::env::var_os("HOME"), before);
    }

    #[test]
    fn child_command_observes_overlay_and_non_utf8_bytes() {
        scoped(|| {
            set(
                OsStr::new("TEKAI_ENV_TEST"),
                OsStr::from_bytes(b"child\xff"),
            )
            .unwrap();
            let mut command = Command::new("/bin/sh");
            command.arg("-c").arg("printf '%s' \"$TEKAI_ENV_TEST\"");
            configure_command(&mut command).unwrap();
            let output = command.output().unwrap();
            assert!(output.status.success());
            assert_eq!(output.stdout, b"child\xff");
        });
    }

    #[test]
    fn scope_and_alias_rejections_are_fallible_and_recover_after_unwind() {
        let mut first = EnvironmentSession::new().unwrap();
        let mut second = EnvironmentSession::new().unwrap();
        EnvironmentSession::scope(first.as_mut(), || {
            assert_eq!(
                EnvironmentSession::scope(second.as_mut(), || ()),
                Err(EnvironmentError::Nested)
            );
            with_overlay(|_| {
                assert_eq!(with_overlay(|_| ()), Err(EnvironmentError::Busy));
                assert_eq!(
                    EnvironmentSession::scope(second.as_mut(), || ()),
                    Err(EnvironmentError::Busy)
                );
            })
            .unwrap();
            let unwind = catch_unwind(AssertUnwindSafe(|| {
                let _ = with_overlay::<()>(|_| panic!("callback failed"));
            }));
            assert!(unwind.is_err());
            assert_eq!(var_os("HOME"), std::env::var_os("HOME"));
        })
        .unwrap();
    }

    #[test]
    fn invalid_data_and_storage_limit_are_ordinary_errors() {
        scoped(|| {
            assert_eq!(
                set(OsStr::new(""), OsStr::new("x")),
                Err(EnvironmentError::InvalidName)
            );
            assert_eq!(
                set(OsStr::new("a=b"), OsStr::new("x")),
                Err(EnvironmentError::InvalidName)
            );
            assert_eq!(
                set(OsStr::new("a"), OsStr::from_bytes(b"x\0y")),
                Err(EnvironmentError::NulValue)
            );
            assert_eq!(
                set(OsStr::new("a"), OsStr::from_bytes(&vec![b'x'; MAX_STORAGE])),
                Err(EnvironmentError::ResourceLimit)
            );
        });
    }
}
