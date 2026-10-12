//! Stop translated control flow without terminating the embedding process.
//!
//! Internal engine calls use Rust's ABI, so a private unwind payload can leave
//! deeply nested fatal and help/version paths while running resource destructors.
//! It is caught only at the invocation boundary. Unexpected panics propagate.

#![warn(clippy::all)]

use crate::invocation::EngineCompletion;
use std::ffi::{c_char, c_int, CStr};
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

struct EngineExit(c_int);

pub(crate) fn exit(status: c_int) -> ! {
    // Unlike panic_any, resume_unwind does not run the application's panic hook
    // for an ordinary engine diagnostic or a successful help/version request.
    resume_unwind(Box::new(EngineExit(status)))
}

pub(crate) fn abort() -> ! {
    panic!("engine requested an abort for an internal failure")
}

pub(crate) fn catch<T>(operation: impl FnOnce() -> T) -> Result<T, EngineCompletion> {
    match catch_unwind(AssertUnwindSafe(operation)) {
        Ok(value) => Ok(value),
        Err(payload) => match payload.downcast::<EngineExit>() {
            Ok(exit) if exit.0 == 0 => Err(EngineCompletion::Clean),
            Ok(_) => Err(EngineCompletion::Errors),
            Err(unexpected) => resume_unwind(unexpected),
        },
    }
}

/// The generated assertion sites pass readable, NUL-terminated literals.
/// This is a programming failure, not a successful or recoverable engine exit.
pub(crate) unsafe fn __assert_rtn(
    function: *const c_char,
    file: *const c_char,
    line: c_int,
    expression: *const c_char,
) -> ! {
    // SAFETY: callers provide static C string literals for all three pointers.
    let (function, file, expression) = unsafe {
        (
            CStr::from_ptr(function).to_string_lossy(),
            CStr::from_ptr(file).to_string_lossy(),
            CStr::from_ptr(expression).to_string_lossy(),
        )
    };
    panic!("engine assertion failed at {file}:{line} in {function}: {expression}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    struct MarkDrop<'a>(&'a Cell<bool>);
    impl Drop for MarkDrop<'_> {
        fn drop(&mut self) {
            self.0.set(true);
        }
    }

    #[test]
    fn deliberate_exit_runs_destructors_and_can_be_followed_by_success() {
        for status in [0, 1, 2, -1] {
            let dropped = Cell::new(false);
            let result = catch(|| {
                let _resource = MarkDrop(&dropped);
                exit(status);
            })
            .unwrap_err();
            assert!(dropped.get());
            assert_eq!(result.exit_code(), i32::from(status != 0));
            assert_eq!(
                catch(|| EngineCompletion::Clean),
                Ok(EngineCompletion::Clean)
            );
        }
    }

    #[test]
    fn unexpected_panic_is_not_misreported_as_engine_completion() {
        let dropped = Cell::new(false);
        let result = catch_unwind(AssertUnwindSafe(|| {
            catch(|| {
                let _resource = MarkDrop(&dropped);
                panic!("unexpected implementation failure");
            })
        }));
        assert!(dropped.get());
        assert_eq!(
            result.unwrap_err().downcast_ref::<&str>(),
            Some(&"unexpected implementation failure")
        );
    }

    #[test]
    fn assertion_is_a_programming_failure_with_source_context() {
        let error = catch_unwind(|| unsafe {
            __assert_rtn(
                c"caller".as_ptr(),
                c"source.rs".as_ptr(),
                17,
                c"valid".as_ptr(),
            );
        })
        .unwrap_err();
        let message = error.downcast_ref::<String>().unwrap();
        assert!(message.contains("source.rs:17"));
        assert!(message.contains("caller: valid"));
    }
}
