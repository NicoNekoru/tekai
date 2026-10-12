//! Invocation-owned storage for the former mutable engine statics.

#![warn(clippy::all)]

use core::cell::{Cell, UnsafeCell};
use core::marker::{PhantomData, PhantomPinned};
use core::pin::Pin;
use core::ptr::NonNull;
use std::rc::Rc;

// No Box or whole-Storage reference exists while engine pointers are live.
// Raw ownership also keeps a move or borrow of the small handle below from
// changing the allocation provenance of its separate inline core storage.
pub(crate) struct Storage {
    pub(crate) generated_backend_avl: UnsafeCell<crate::generated::backend::avl::State>,
    pub(crate) generated_backend_avlstuff: UnsafeCell<crate::generated::backend::avlstuff::State>,
    pub(crate) generated_backend_mapfile: UnsafeCell<crate::generated::backend::mapfile::State>,
    pub(crate) generated_backend_pkin: UnsafeCell<crate::generated::backend::pkin::State>,
    pub(crate) generated_backend_subfont: UnsafeCell<crate::generated::backend::subfont::State>,
    pub(crate) generated_backend_tounicode: UnsafeCell<crate::generated::backend::tounicode::State>,
    pub(crate) generated_backend_vfpacket: UnsafeCell<crate::generated::backend::vfpacket::State>,
    pub(crate) generated_backend_writeenc: UnsafeCell<crate::generated::backend::writeenc::State>,
    pub(crate) generated_backend_writefont: UnsafeCell<crate::generated::backend::writefont::State>,
    pub(crate) generated_backend_writeimg: UnsafeCell<crate::generated::backend::writeimg::State>,
    pub(crate) generated_backend_writejbig2:
        UnsafeCell<crate::generated::backend::writejbig2::State>,
    pub(crate) generated_backend_writepng: UnsafeCell<crate::generated::backend::writepng::State>,
    pub(crate) generated_backend_writet1: UnsafeCell<crate::generated::backend::writet1::State>,
    pub(crate) generated_backend_writet3: UnsafeCell<crate::generated::backend::writet3::State>,
    pub(crate) generated_backend_writettf: UnsafeCell<crate::generated::backend::writettf::State>,
    pub(crate) generated_pdftex0: UnsafeCell<crate::generated::pdftex0::State>,
    pub(crate) generated_pdftex_pool: UnsafeCell<crate::generated::pdftex_pool::State>,
    pub(crate) generated_pdftexextra: UnsafeCell<crate::generated::pdftexextra::State>,
    pub(crate) kpathsea: UnsafeCell<crate::kpathsea::State>,
    pub(crate) openclose: UnsafeCell<crate::openclose::State>,
    pub(crate) support: UnsafeCell<crate::support::State>,
    pub(crate) utils: UnsafeCell<crate::utils::State>,
    _thread_confined: PhantomData<Rc<()>>,
    _pinned: PhantomPinned,
}

pub(crate) struct InvocationState {
    storage: NonNull<Storage>,
    _thread_confined: PhantomData<Rc<()>>,
    _pinned: PhantomPinned,
}

impl Drop for InvocationState {
    fn drop(&mut self) {
        // All private scopes have ended and no engine aliases remain. This
        // is the sole reconstruction of the fully initialized storage Box.
        unsafe {
            drop(Box::from_raw(self.storage.as_ptr()));
        }
    }
}

struct PartialAllocation {
    pointer: *mut core::mem::MaybeUninit<Storage>,
    armed: bool,
}
impl Drop for PartialAllocation {
    fn drop(&mut self) {
        if self.armed {
            // Module field guards are declared after this guard and therefore
            // drop first. MaybeUninit then deallocates without double-dropping
            // any completed or incomplete State field.
            unsafe {
                drop(Box::from_raw(self.pointer));
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NestedState;
impl core::fmt::Display for NestedState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("nested engine state invocation")
    }
}
impl std::error::Error for NestedState {}

std::thread_local! {
    static ACTIVE: Cell<*const Storage> = const { Cell::new(core::ptr::null()) };
}
struct ScopeGuard;
impl Drop for ScopeGuard {
    fn drop(&mut self) {
        ACTIVE.with(|active| active.set(core::ptr::null()));
    }
}

impl InvocationState {
    pub(crate) fn new() -> Pin<Box<Self>> {
        // Relinquish Box's unique payload borrow BEFORE obtaining any field
        // address. Only the raw allocation guard owns storage during placement.
        let mut allocation = PartialAllocation {
            pointer: Box::into_raw(Box::<Storage>::new_uninit()),
            armed: true,
        };
        let invocation = allocation.pointer.cast::<Storage>();
        // Storage is private raw-owned memory at a fixed address. Nothing
        // creates an initialized whole-Storage reference or moves its fields.
        unsafe {
            let mut __tekai_init_generated_backend_avl =
                crate::generated::backend::avl::StateInitialization::begin(UnsafeCell::raw_get(
                    core::ptr::addr_of!((*invocation).generated_backend_avl),
                ));
            let mut __tekai_init_generated_backend_avlstuff =
                crate::generated::backend::avlstuff::StateInitialization::begin(
                    UnsafeCell::raw_get(core::ptr::addr_of!(
                        (*invocation).generated_backend_avlstuff
                    )),
                );
            let mut __tekai_init_generated_backend_mapfile =
                crate::generated::backend::mapfile::StateInitialization::begin(
                    UnsafeCell::raw_get(core::ptr::addr_of!(
                        (*invocation).generated_backend_mapfile
                    )),
                );
            let mut __tekai_init_generated_backend_pkin =
                crate::generated::backend::pkin::StateInitialization::begin(UnsafeCell::raw_get(
                    core::ptr::addr_of!((*invocation).generated_backend_pkin),
                ));
            let mut __tekai_init_generated_backend_subfont =
                crate::generated::backend::subfont::StateInitialization::begin(
                    UnsafeCell::raw_get(core::ptr::addr_of!(
                        (*invocation).generated_backend_subfont
                    )),
                );
            let mut __tekai_init_generated_backend_tounicode =
                crate::generated::backend::tounicode::StateInitialization::begin(
                    UnsafeCell::raw_get(core::ptr::addr_of!(
                        (*invocation).generated_backend_tounicode
                    )),
                );
            let mut __tekai_init_generated_backend_vfpacket =
                crate::generated::backend::vfpacket::StateInitialization::begin(
                    UnsafeCell::raw_get(core::ptr::addr_of!(
                        (*invocation).generated_backend_vfpacket
                    )),
                );
            let mut __tekai_init_generated_backend_writeenc =
                crate::generated::backend::writeenc::StateInitialization::begin(
                    UnsafeCell::raw_get(core::ptr::addr_of!(
                        (*invocation).generated_backend_writeenc
                    )),
                );
            let mut __tekai_init_generated_backend_writefont =
                crate::generated::backend::writefont::StateInitialization::begin(
                    UnsafeCell::raw_get(core::ptr::addr_of!(
                        (*invocation).generated_backend_writefont
                    )),
                );
            let mut __tekai_init_generated_backend_writeimg =
                crate::generated::backend::writeimg::StateInitialization::begin(
                    UnsafeCell::raw_get(core::ptr::addr_of!(
                        (*invocation).generated_backend_writeimg
                    )),
                );
            let mut __tekai_init_generated_backend_writejbig2 =
                crate::generated::backend::writejbig2::StateInitialization::begin(
                    UnsafeCell::raw_get(core::ptr::addr_of!(
                        (*invocation).generated_backend_writejbig2
                    )),
                );
            let mut __tekai_init_generated_backend_writepng =
                crate::generated::backend::writepng::StateInitialization::begin(
                    UnsafeCell::raw_get(core::ptr::addr_of!(
                        (*invocation).generated_backend_writepng
                    )),
                );
            let mut __tekai_init_generated_backend_writet1 =
                crate::generated::backend::writet1::StateInitialization::begin(
                    UnsafeCell::raw_get(core::ptr::addr_of!(
                        (*invocation).generated_backend_writet1
                    )),
                );
            let mut __tekai_init_generated_backend_writet3 =
                crate::generated::backend::writet3::StateInitialization::begin(
                    UnsafeCell::raw_get(core::ptr::addr_of!(
                        (*invocation).generated_backend_writet3
                    )),
                );
            let mut __tekai_init_generated_backend_writettf =
                crate::generated::backend::writettf::StateInitialization::begin(
                    UnsafeCell::raw_get(core::ptr::addr_of!(
                        (*invocation).generated_backend_writettf
                    )),
                );
            let mut __tekai_init_generated_pdftex0 =
                crate::generated::pdftex0::StateInitialization::begin(UnsafeCell::raw_get(
                    core::ptr::addr_of!((*invocation).generated_pdftex0),
                ));
            let mut __tekai_init_generated_pdftex_pool =
                crate::generated::pdftex_pool::StateInitialization::begin(UnsafeCell::raw_get(
                    core::ptr::addr_of!((*invocation).generated_pdftex_pool),
                ));
            let mut __tekai_init_generated_pdftexextra =
                crate::generated::pdftexextra::StateInitialization::begin(UnsafeCell::raw_get(
                    core::ptr::addr_of!((*invocation).generated_pdftexextra),
                ));
            let mut __tekai_init_kpathsea = crate::kpathsea::StateInitialization::begin(
                UnsafeCell::raw_get(core::ptr::addr_of!((*invocation).kpathsea)),
            );
            let mut __tekai_init_openclose = crate::openclose::StateInitialization::begin(
                UnsafeCell::raw_get(core::ptr::addr_of!((*invocation).openclose)),
            );
            let mut __tekai_init_support = crate::support::StateInitialization::begin(
                UnsafeCell::raw_get(core::ptr::addr_of!((*invocation).support)),
            );
            let mut __tekai_init_utils = crate::utils::StateInitialization::begin(
                UnsafeCell::raw_get(core::ptr::addr_of!((*invocation).utils)),
            );
            __tekai_init_generated_backend_avl.initialize_fields(invocation);
            __tekai_init_generated_backend_avlstuff.initialize_fields(invocation);
            __tekai_init_generated_backend_mapfile.initialize_fields(invocation);
            __tekai_init_generated_backend_pkin.initialize_fields(invocation);
            __tekai_init_generated_backend_subfont.initialize_fields(invocation);
            __tekai_init_generated_backend_tounicode.initialize_fields(invocation);
            __tekai_init_generated_backend_vfpacket.initialize_fields(invocation);
            __tekai_init_generated_backend_writeenc.initialize_fields(invocation);
            __tekai_init_generated_backend_writefont.initialize_fields(invocation);
            __tekai_init_generated_backend_writeimg.initialize_fields(invocation);
            __tekai_init_generated_backend_writejbig2.initialize_fields(invocation);
            __tekai_init_generated_backend_writepng.initialize_fields(invocation);
            __tekai_init_generated_backend_writet1.initialize_fields(invocation);
            __tekai_init_generated_backend_writet3.initialize_fields(invocation);
            __tekai_init_generated_backend_writettf.initialize_fields(invocation);
            __tekai_init_generated_pdftex0.initialize_fields(invocation);
            __tekai_init_generated_pdftex_pool.initialize_fields(invocation);
            __tekai_init_generated_pdftexextra.initialize_fields(invocation);
            __tekai_init_kpathsea.initialize_fields(invocation);
            __tekai_init_openclose.initialize_fields(invocation);
            __tekai_init_support.initialize_fields(invocation);
            __tekai_init_utils.initialize_fields(invocation);
            __tekai_init_generated_backend_avl.initialize_fixups(invocation);
            __tekai_init_generated_backend_avlstuff.initialize_fixups(invocation);
            __tekai_init_generated_backend_mapfile.initialize_fixups(invocation);
            __tekai_init_generated_backend_pkin.initialize_fixups(invocation);
            __tekai_init_generated_backend_subfont.initialize_fixups(invocation);
            __tekai_init_generated_backend_tounicode.initialize_fixups(invocation);
            __tekai_init_generated_backend_vfpacket.initialize_fixups(invocation);
            __tekai_init_generated_backend_writeenc.initialize_fixups(invocation);
            __tekai_init_generated_backend_writefont.initialize_fixups(invocation);
            __tekai_init_generated_backend_writeimg.initialize_fixups(invocation);
            __tekai_init_generated_backend_writejbig2.initialize_fixups(invocation);
            __tekai_init_generated_backend_writepng.initialize_fixups(invocation);
            __tekai_init_generated_backend_writet1.initialize_fixups(invocation);
            __tekai_init_generated_backend_writet3.initialize_fixups(invocation);
            __tekai_init_generated_backend_writettf.initialize_fixups(invocation);
            __tekai_init_generated_pdftex0.initialize_fixups(invocation);
            __tekai_init_generated_pdftex_pool.initialize_fixups(invocation);
            __tekai_init_generated_pdftexextra.initialize_fixups(invocation);
            __tekai_init_kpathsea.initialize_fixups(invocation);
            __tekai_init_openclose.initialize_fixups(invocation);
            __tekai_init_support.initialize_fixups(invocation);
            __tekai_init_utils.initialize_fixups(invocation);
            core::ptr::addr_of_mut!((*invocation)._thread_confined).write(PhantomData);
            core::ptr::addr_of_mut!((*invocation)._pinned).write(PhantomPinned);
            __tekai_init_generated_backend_avl.disarm();
            __tekai_init_generated_backend_avlstuff.disarm();
            __tekai_init_generated_backend_mapfile.disarm();
            __tekai_init_generated_backend_pkin.disarm();
            __tekai_init_generated_backend_subfont.disarm();
            __tekai_init_generated_backend_tounicode.disarm();
            __tekai_init_generated_backend_vfpacket.disarm();
            __tekai_init_generated_backend_writeenc.disarm();
            __tekai_init_generated_backend_writefont.disarm();
            __tekai_init_generated_backend_writeimg.disarm();
            __tekai_init_generated_backend_writejbig2.disarm();
            __tekai_init_generated_backend_writepng.disarm();
            __tekai_init_generated_backend_writet1.disarm();
            __tekai_init_generated_backend_writet3.disarm();
            __tekai_init_generated_backend_writettf.disarm();
            __tekai_init_generated_pdftex0.disarm();
            __tekai_init_generated_pdftex_pool.disarm();
            __tekai_init_generated_pdftexextra.disarm();
            __tekai_init_kpathsea.disarm();
            __tekai_init_openclose.disarm();
            __tekai_init_support.disarm();
            __tekai_init_utils.disarm();
            let owner = Self {
                storage: NonNull::new_unchecked(invocation),
                _thread_confined: PhantomData,
                _pinned: PhantomPinned,
            };
            allocation.armed = false;
            Box::pin(owner)
        }
    }

    pub(crate) fn scope<T>(
        self: Pin<&mut Self>,
        operation: impl FnOnce() -> T,
    ) -> Result<T, NestedState> {
        ACTIVE.with(|active| {
            if !active.get().is_null() {
                return Err(NestedState);
            }
            // The exclusive borrow covers only the small owning handle. The
            // independent raw allocation is not reborrowed as &mut Storage.
            active.set(self.as_ref().get_ref().storage.as_ptr().cast_const());
            Ok(())
        })?;
        let _guard = ScopeGuard;
        Ok(operation())
    }
}

/// Caller must remain inside this thread's owner scope and retain no pointers
/// or borrowed field references past its teardown. There is no fallback owner.
pub(crate) fn is_active() -> bool {
    ACTIVE.with(|active| !active.get().is_null())
}

pub(crate) unsafe fn current() -> *const Storage {
    ACTIVE.with(|active| {
        let pointer = active.get();
        assert!(
            !pointer.is_null(),
            "engine state access outside its invocation"
        );
        pointer
    })
}

/// Give a stateful unit test fresh core storage and raw-buffer ownership.
/// This helper never supplies an owner to production calls or missing scopes.
#[cfg(test)]
pub(crate) fn with_fixture<T>(operation: impl FnOnce() -> T) -> T {
    // Declaration order also gives this teardown order during unwinding.
    let mut arena = crate::allocation::AllocationArena::new();
    let mut core = InvocationState::new();
    let result = arena.as_mut().scope(|| {
        core.as_mut()
            .scope(operation)
            .expect("stateful unit-test fixtures cannot nest")
    });
    drop(core);
    drop(arena);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn explicit_fixtures_release_state_and_buffers_after_unwind() {
        assert!(catch_unwind(|| with_fixture(|| unsafe {
            let state = crate::generated::pdftexextra::state();
            (*state).history = 2;
            let buffer = crate::allocation::malloc(32);
            assert!(!buffer.is_null());
            // The fixture arena owns the intentionally unfinished buffer.
            panic!("unfinished stateful unit test");
        }))
        .is_err());
        assert!(catch_unwind(|| unsafe { current() }).is_err());
        with_fixture(|| unsafe {
            assert_eq!((*crate::generated::pdftexextra::state()).history, 0);
        });
    }

    #[test]
    fn actual_core_constructs_fieldwise_on_a_small_stack() {
        std::thread::Builder::new()
            .stack_size(128 * 1024)
            .spawn(|| {
                let mut owner = InvocationState::new();
                let expected = owner.storage.as_ptr().cast_const();
                owner
                    .as_mut()
                    .scope(|| unsafe {
                        assert_eq!(current(), expected);
                        crate::generated::backend::avl::check_state_construction(0);
                        crate::generated::backend::avlstuff::check_state_construction(0);
                        crate::generated::backend::mapfile::check_state_construction(0);
                        crate::generated::backend::pkin::check_state_construction(0);
                        crate::generated::backend::subfont::check_state_construction(0);
                        crate::generated::backend::tounicode::check_state_construction(0);
                        crate::generated::backend::vfpacket::check_state_construction(0);
                        crate::generated::backend::writeenc::check_state_construction(0);
                        crate::generated::backend::writefont::check_state_construction(0);
                        crate::generated::backend::writeimg::check_state_construction(0);
                        crate::generated::backend::writejbig2::check_state_construction(0);
                        crate::generated::backend::writepng::check_state_construction(0);
                        crate::generated::backend::writet1::check_state_construction(0);
                        crate::generated::backend::writet3::check_state_construction(0);
                        crate::generated::backend::writettf::check_state_construction(0);
                        crate::generated::pdftex0::check_state_construction(0);
                        crate::generated::pdftex_pool::check_state_construction(0);
                        crate::generated::pdftexextra::check_state_construction(0);
                        crate::kpathsea::check_state_construction(0);
                        crate::openclose::check_state_construction(0);
                        crate::support::check_state_construction(0);
                        crate::utils::check_state_construction(0);
                    })
                    .unwrap();
                // Move the owning handle between registrations. Internal raw
                // fixups and the mutated target must retain their provenance.
                let mut moved = owner;
                moved
                    .as_mut()
                    .scope(|| unsafe {
                        assert_eq!(current(), expected);
                        crate::generated::backend::avl::check_state_construction(1);
                        crate::generated::backend::avlstuff::check_state_construction(1);
                        crate::generated::backend::mapfile::check_state_construction(1);
                        crate::generated::backend::pkin::check_state_construction(1);
                        crate::generated::backend::subfont::check_state_construction(1);
                        crate::generated::backend::tounicode::check_state_construction(1);
                        crate::generated::backend::vfpacket::check_state_construction(1);
                        crate::generated::backend::writeenc::check_state_construction(1);
                        crate::generated::backend::writefont::check_state_construction(1);
                        crate::generated::backend::writeimg::check_state_construction(1);
                        crate::generated::backend::writejbig2::check_state_construction(1);
                        crate::generated::backend::writepng::check_state_construction(1);
                        crate::generated::backend::writet1::check_state_construction(1);
                        crate::generated::backend::writet3::check_state_construction(1);
                        crate::generated::backend::writettf::check_state_construction(1);
                        crate::generated::pdftex0::check_state_construction(1);
                        crate::generated::pdftex_pool::check_state_construction(1);
                        crate::generated::pdftexextra::check_state_construction(1);
                        crate::kpathsea::check_state_construction(1);
                        crate::openclose::check_state_construction(1);
                        crate::support::check_state_construction(1);
                        crate::utils::check_state_construction(1);
                    })
                    .unwrap();
                ACTIVE.with(|active| assert!(active.get().is_null()));
            })
            .unwrap()
            .join()
            .unwrap();
    }

    #[test]
    fn private_scope_rejects_nesting_and_clears_on_unwind() {
        let mut first = InvocationState::new();
        let mut second = InvocationState::new();
        assert!(catch_unwind(AssertUnwindSafe(|| {
            first
                .as_mut()
                .scope(|| {
                    assert_eq!(
                        second.as_mut().scope(|| -> () { unreachable!() }),
                        Err(NestedState)
                    );
                    panic!("exercise engine state unwind");
                })
                .unwrap();
        }))
        .is_err());
        ACTIVE.with(|active| assert!(active.get().is_null()));
        second
            .as_mut()
            .scope(|| assert!(!unsafe { current() }.is_null()))
            .unwrap();
    }

    #[test]
    fn thread_confined_scopes_route_to_independent_owned_storage() {
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
        let threads = std::array::from_fn::<_, 2, _>(|_| {
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                let mut owner = InvocationState::new();
                let expected = owner.storage.as_ptr().cast_const();
                owner
                    .as_mut()
                    .scope(|| {
                        barrier.wait();
                        assert_eq!(unsafe { current() }, expected);
                        barrier.wait();
                        assert_eq!(unsafe { current() }, expected);
                    })
                    .unwrap();
                ACTIVE.with(|active| assert!(active.get().is_null()));
            })
        });
        for thread in threads {
            thread.join().unwrap();
        }
    }
}
