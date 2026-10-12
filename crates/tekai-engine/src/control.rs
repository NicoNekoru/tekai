//! Invocation-owned interrupt requests. Signal registration belongs to the CLI.

#![warn(clippy::all)]

use std::cell::Cell;
use std::marker::{PhantomData, PhantomPinned};
use std::pin::Pin;
use std::ptr;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// A caller may request an interrupt without touching engine memory or signal
/// policy. Polling uses the engine's existing TeX interrupt/interaction rules.
#[derive(Clone, Debug, Default)]
pub struct InterruptToken(Arc<AtomicBool>);

impl InterruptToken {
    pub fn request(&self) {
        self.0.store(true, Ordering::Release);
    }

    /// The CLI may register this flag with its signal-handling adapter. The
    /// engine library itself never installs or replaces a host signal handler.
    pub fn signal_flag(&self) -> Arc<AtomicBool> {
        self.0.clone()
    }
}

thread_local! {
    static ACTIVE: Cell<*const ControlSession> = const { Cell::new(ptr::null()) };
}

pub(crate) struct ControlSession {
    interrupt: InterruptToken,
    profile: Option<Rc<crate::engine_profile::Collector>>,
    _thread_confined: PhantomData<Rc<()>>,
    _pinned: PhantomPinned,
}

impl ControlSession {
    pub(crate) fn new(interrupt: InterruptToken) -> Pin<Box<Self>> {
        Self::new_with_profile(interrupt, None)
    }

    pub(crate) fn new_with_profile(
        interrupt: InterruptToken,
        profile: Option<Rc<crate::engine_profile::Collector>>,
    ) -> Pin<Box<Self>> {
        Box::pin(Self {
            interrupt,
            profile,
            _thread_confined: PhantomData,
            _pinned: PhantomPinned,
        })
    }

    pub(crate) fn scope<T>(
        self: Pin<&mut Self>,
        operation: impl FnOnce() -> T,
    ) -> Result<T, &'static str> {
        struct Guard;
        impl Drop for Guard {
            fn drop(&mut self) {
                ACTIVE.with(|active| active.set(ptr::null()));
            }
        }
        ACTIVE.with(|active| {
            if !active.get().is_null() {
                return Err("nested engine interrupt scope");
            }
            active.set(self.as_ref().get_ref());
            Ok(())
        })?;
        let _guard = Guard;
        Ok(operation())
    }
}

pub(crate) fn profile_collector() -> Option<Rc<crate::engine_profile::Collector>> {
    ACTIVE.with(|active| {
        let pointer = active.get();
        if pointer.is_null() {
            None
        } else {
            // SAFETY The existing private scope holds the pinned owner. Only
            // the optional Rc handle is cloned, never a mutable session borrow.
            unsafe { &*pointer }.profile.clone()
        }
    })
}

fn take_request() -> bool {
    ACTIVE.with(|active| {
        let pointer = active.get();
        if pointer.is_null() {
            return false;
        }
        // SAFETY: the private guard borrows this pinned thread-confined owner.
        unsafe { &*pointer }
            .interrupt
            .0
            .swap(false, Ordering::AcqRel)
    })
}

pub(crate) unsafe fn interrupted() -> bool {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    // SAFETY: engine callers execute within the core state's exclusive scope.
    // The other thread only writes the AtomicBool, never this ordinary field.
    unsafe {
        if take_request() {
            (*__tekai_state_generated_pdftexextra).interrupt = 1;
        }
        (*__tekai_state_generated_pdftexextra).interrupt != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn scoped_requests_are_consumed_once_and_do_not_leak_to_later_owners() {
        let token = InterruptToken::default();
        let mut owner = ControlSession::new(token.clone());
        owner
            .as_mut()
            .scope(|| {
                assert!(!take_request());
                std::thread::spawn(move || token.request()).join().unwrap();
                assert!(take_request());
                assert!(!take_request());
            })
            .unwrap();
        let mut next = ControlSession::new(InterruptToken::default());
        next.as_mut().scope(|| assert!(!take_request())).unwrap();
    }

    #[test]
    fn scope_is_cleared_after_unwind_and_rejects_nesting() {
        let mut owner = ControlSession::new(InterruptToken::default());
        let mut nested = ControlSession::new(InterruptToken::default());
        assert!(catch_unwind(AssertUnwindSafe(|| {
            owner
                .as_mut()
                .scope(|| {
                    assert!(nested.as_mut().scope(|| unreachable!()).is_err());
                    panic!("engine failure");
                })
                .unwrap();
        }))
        .is_err());
        assert!(!take_request());
        nested.as_mut().scope(|| assert!(!take_request())).unwrap();
    }
}
