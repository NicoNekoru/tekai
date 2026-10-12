//! Invocation-owned regex results routed through a non-owning thread pointer.
//!
//! The scope API keeps its unregister guard private, so safe callers cannot
//! forget a public guard and leave a dangling pointer. A separate busy guard
//! rejects recursive mutable access and resets even when the action unwinds.

#![warn(clippy::all)]

use crate::posix_regex::{Match, Regex, RegexError};
use std::cell::Cell;
use std::fmt;
use std::marker::{PhantomData, PhantomPinned};
use std::pin::Pin;
use std::ptr::NonNull;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SessionError {
    Missing,
    Nested,
    Busy,
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Missing => "no active regex invocation scope",
            Self::Nested => "nested regex invocation scope",
            Self::Busy => "recursive regex state access",
        })
    }
}

impl std::error::Error for SessionError {}

/// Pure owned result state. Requested slots never determine allocation size.
#[derive(Default)]
pub(crate) struct MatchState {
    requested: usize,
    text: Vec<u8>,
    matched: Option<Match>,
}

impl MatchState {
    pub(crate) fn clear(&mut self) {
        self.requested = 0;
        self.text.clear();
        self.matched = None;
    }

    /// Compile before the boundary renders text into makecstring's reusable
    /// backing. The resulting Regex owns all pattern-dependent storage.
    pub(crate) fn prepare(
        &mut self,
        pattern: &[u8],
        subcount: i32,
        icase: bool,
    ) -> Result<Regex, RegexError> {
        // Deliberately correct the old stale-result behavior on compile error.
        self.clear();
        let regex = Regex::compile(pattern, icase)?;
        self.requested = if subcount < 0 { 10 } else { subcount as usize };
        Ok(regex)
    }

    pub(crate) fn execute(&mut self, regex: &Regex, text: &[u8]) -> Result<bool, RegexError> {
        // Every admission, matching, or allocation error invalidates the last
        // result. Only fully owned successful data can become observable.
        self.text.clear();
        self.matched = None;
        let Some(matched) = regex.captures(text, self.requested)? else {
            return Ok(false);
        };
        self.text
            .try_reserve_exact(text.len())
            .map_err(|_| RegexError::Allocation)?;
        self.text.extend_from_slice(text);
        self.matched = Some(matched);
        Ok(true)
    }

    pub(crate) fn capture(&self, index: i32) -> Option<(usize, &[u8])> {
        let index = usize::try_from(index).ok()?;
        if index >= self.requested {
            return None;
        }
        let span = self.matched.as_ref()?.captures.get(index)?.as_ref()?;
        Some((span.start, self.text.get(span.clone())?))
    }
}

pub(crate) struct RegexSession {
    state: MatchState,
    _thread_confined: PhantomData<Rc<()>>,
    _pinned: PhantomPinned,
}

thread_local! {
    static ACTIVE: Active = const { Active {
        pointer: Cell::new(None),
        busy: Cell::new(false),
    } };
}

struct Active {
    pointer: Cell<Option<NonNull<RegexSession>>>,
    busy: Cell<bool>,
}

struct ScopeGuard(NonNull<RegexSession>);

impl Drop for ScopeGuard {
    fn drop(&mut self) {
        let _ = ACTIVE.try_with(|active| {
            if active.pointer.get() == Some(self.0) {
                active.pointer.set(None);
                // SAFETY This private guard drops after action returns or
                // unwinds, including after its inner mutable-access guards.
                // The original pinned owner is still exclusively borrowed.
                unsafe { self.0.as_mut().state.clear() };
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

impl RegexSession {
    pub(crate) fn new() -> Pin<Box<Self>> {
        Box::pin(Self {
            state: MatchState::default(),
            _thread_confined: PhantomData,
            _pinned: PhantomPinned,
        })
    }

    pub(crate) fn scope<T>(
        session: Pin<&mut Self>,
        action: impl FnOnce() -> T,
    ) -> Result<T, SessionError> {
        let pointer = ACTIVE.with(|active| {
            if active.busy.get() {
                return Err(SessionError::Busy);
            }
            if active.pointer.get().is_some() {
                return Err(SessionError::Nested);
            }
            // SAFETY The pinned exclusive borrow remains alive through action
            // and the private guard. Neither function moves session fields.
            let pointer = NonNull::from(unsafe { session.get_unchecked_mut() });
            active.pointer.set(Some(pointer));
            Ok(pointer)
        })?;
        let _scope = ScopeGuard(pointer);
        Ok(action())
    }
}

pub(crate) fn with_state<T>(action: impl FnOnce(&mut MatchState) -> T) -> Result<T, SessionError> {
    ACTIVE.with(|active| {
        if active.busy.get() {
            return Err(SessionError::Busy);
        }
        let mut pointer = active.pointer.get().ok_or(SessionError::Missing)?;
        active.busy.set(true);
        let _busy = BusyGuard(&active.busy);
        // SAFETY Only the private scope API can register a pointer. It keeps
        // the pinned thread-confined owner exclusively borrowed until this
        // call returns. The busy guard excludes aliased mutable callbacks.
        let state = unsafe { &mut pointer.as_mut().state };
        Ok(action(state))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::posix_regex::{MAX_PATTERN_BYTES, MAX_TEXT_BYTES};
    use proptest::prelude::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    fn matched(state: &mut MatchState, pattern: &[u8], text: &[u8], requested: i32) {
        let regex = state.prepare(pattern, requested, false).unwrap();
        assert!(state.execute(&regex, text).unwrap());
    }

    #[test]
    fn slots_follow_real_groups_without_request_proportional_storage() {
        let mut state = MatchState::default();
        matched(&mut state, br"\([()]", b"((", i32::MAX);
        assert_eq!(state.matched.as_ref().unwrap().captures.len(), 1);
        assert_eq!(state.capture(0), Some((0, b"((".as_slice())));
        assert_eq!(state.capture(1), None);
        matched(&mut state, b"(a)", b"a", i32::MAX);
        assert_eq!(state.matched.as_ref().unwrap().captures.len(), 2);
        matched(&mut state, b"(a)", b"a", 0);
        assert!(state.matched.as_ref().unwrap().captures.is_empty());
        assert_eq!(state.capture(0), None);
    }

    #[test]
    fn defaults_empty_unmatched_and_byte_offsets_are_preserved() {
        let mut state = MatchState::default();
        matched(&mut state, b"(a)?()", b"", -1);
        assert_eq!(state.requested, 10);
        assert_eq!(state.capture(0), Some((0, b"".as_slice())));
        assert_eq!(state.capture(1), None);
        assert_eq!(state.capture(2), Some((0, b"".as_slice())));
        assert_eq!(state.capture(-1), None);
        matched(&mut state, &[b'(', 0x80, b')'], &[b'z', 0x80], 2);
        assert_eq!(state.capture(1), Some((1, [0x80].as_slice())));
    }

    #[test]
    fn unsuccessful_calls_clear_stale_results() {
        let mut state = MatchState::default();
        matched(&mut state, b"(a)", b"a", 2);
        assert!(state.prepare(b"[", 2, false).is_err());
        assert_eq!(state.capture(1), None);
        matched(&mut state, b"(a)", b"a", 2);
        let regex = state.prepare(b"(b)", 2, false).unwrap();
        assert!(!state.execute(&regex, b"a").unwrap());
        assert_eq!(state.capture(1), None);
        matched(&mut state, b"(a)", b"a", 2);
        assert!(state
            .prepare(&vec![b'a'; MAX_PATTERN_BYTES + 1], 2, false)
            .is_err());
        assert_eq!(state.capture(1), None);
        let regex = state.prepare(b"(a)", 2, false).unwrap();
        assert!(state
            .execute(&regex, &vec![b'a'; MAX_TEXT_BYTES + 1])
            .is_err());
        assert_eq!(state.capture(1), None);
        assert!(state.execute(&regex, b"a\0b").is_err());
        assert_eq!(state.capture(1), None);
    }

    #[test]
    fn compiled_pattern_survives_reused_boundary_backing() {
        let mut state = MatchState::default();
        let mut backing = b"(a)".to_vec();
        let regex = state.prepare(&backing, 2, false).unwrap();
        backing.clear();
        backing.extend_from_slice(b"za");
        assert!(state.execute(&regex, &backing).unwrap());
        backing.fill(b'x');
        assert_eq!(state.capture(1), Some((1, b"a".as_slice())));
    }

    #[test]
    fn ambiguous_matching_budget_error_clears_a_prior_successful_execution() {
        let mut state = MatchState::default();
        let regex = state.prepare(b"((a|aa)*)", 3, false).unwrap();
        assert!(state.execute(&regex, b"a").unwrap());
        assert!(state.capture(1).is_some());
        assert_eq!(
            state.execute(&regex, &vec![b'a'; 40_000]),
            Err(RegexError::MatchLimit)
        );
        assert_eq!(state.capture(1), None);
    }

    #[test]
    fn scopes_reject_nesting_and_recursion_without_aliasing() {
        assert_eq!(with_state(|_| ()), Err(SessionError::Missing));
        let mut outer = RegexSession::new();
        let mut inner = RegexSession::new();
        RegexSession::scope(outer.as_mut(), || {
            assert_eq!(
                RegexSession::scope(inner.as_mut(), || ()),
                Err(SessionError::Nested)
            );
            with_state(|_| {
                assert_eq!(with_state(|_| ()), Err(SessionError::Busy));
                assert_eq!(
                    RegexSession::scope(inner.as_mut(), || ()),
                    Err(SessionError::Busy)
                );
            })
            .unwrap();
            with_state(|_| ()).unwrap();
        })
        .unwrap();
        assert_eq!(with_state(|_| ()), Err(SessionError::Missing));
    }

    #[test]
    fn both_routing_guards_clear_on_unwind_and_owners_do_not_leak_results() {
        let mut first = RegexSession::new();
        RegexSession::scope(first.as_mut(), || {
            let unwind = catch_unwind(AssertUnwindSafe(|| {
                let _ = with_state::<()>(|_| panic!("callback failed"));
            }));
            assert!(unwind.is_err());
            with_state(|state| matched(state, b"(a)", b"a", 2)).unwrap();
        })
        .unwrap();
        let unwind = catch_unwind(AssertUnwindSafe(|| {
            let _ = RegexSession::scope(first.as_mut(), || panic!("invocation failed"));
        }));
        assert!(unwind.is_err());
        assert_eq!(with_state(|_| ()), Err(SessionError::Missing));
        drop(first);
        let mut second = RegexSession::new();
        RegexSession::scope(second.as_mut(), || {
            with_state(|state| assert_eq!(state.capture(1), None)).unwrap();
        })
        .unwrap();
    }

    #[test]
    fn active_state_is_thread_local() {
        let mut session = RegexSession::new();
        RegexSession::scope(session.as_mut(), || {
            std::thread::spawn(|| {
                assert_eq!(with_state(|_| ()), Err(SessionError::Missing));
                let mut other = RegexSession::new();
                RegexSession::scope(other.as_mut(), || with_state(|_| ()).unwrap()).unwrap();
            })
            .join()
            .unwrap();
            with_state(|_| ()).unwrap();
        })
        .unwrap();
    }

    proptest! {
        #[test]
        fn capture_storage_uses_compiled_groups_and_requested_count(
            requested in 0i32..=i32::MAX,
            groups in 0usize..32,
        ) {
            let pattern = format!("{}a{}", "(".repeat(groups), ")".repeat(groups));
            let mut state = MatchState::default();
            matched(&mut state, pattern.as_bytes(), b"a", requested);
            prop_assert_eq!(state.matched.as_ref().unwrap().captures.len(),
                (requested as usize).min(groups + 1));
        }
    }
}
