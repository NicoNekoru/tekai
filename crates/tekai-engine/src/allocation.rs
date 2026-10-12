//! Rust allocation for the remaining translated raw-pointer buffers.
//!
//! Every buffer consumed here must also be produced here. A header stores its
//! layout and invocation-owner links beside the buffer, so releasing a buffer
//! needs no pointer registry or lock. The payload starts at a 16-byte boundary,
//! matching the fundamental
//! alignment required by the translated engine's types. This header is private
//! allocation metadata and never enters a TeX format, image or PDF.
//!
//! These adapters are a migration step. They replace owned libc allocation and
//! its internal C linkage, but raw engine buffers still need typed owners. Rust
//! `Box`, `Vec`, borrowed strings and OS-owned pointers keep their own contracts
//! and must never be passed to `free` or `realloc`.
//!
//! The translated core and AVL default allocator allocate here. Font maps,
//! encodings, PK/Type1/TrueType data, VF packets, image rows and compression
//! staging buffers allocate through `openclose::xmalloc` and `xrealloc`.
//! `xstrdup`, kpathsea lookup results, support string helpers, `read_line` and
//! `xgetcwd` return copied payloads from the same allocator. PDF document names
//! and utility scratch arrays also use it. Their existing destruction paths
//! all import these Rust functions. PNG decoder state and palettes, xpdf
//! objects, PDF document records, SyncTeX and codec-internal storage remain
//! owned Rust values and never use these raw destruction paths.

#![warn(clippy::all)]

use std::alloc::{alloc, dealloc, realloc as realloc_block, Layout};
use std::cell::Cell;
use std::ffi::c_void;
use std::marker::{PhantomData, PhantomPinned};
use std::mem::{align_of, size_of};
use std::pin::Pin;
use std::ptr::{self, NonNull};
use std::rc::Rc;

#[repr(C, align(16))]
struct Header {
    payload_size: usize,
    owner: *mut ArenaState,
    previous: *mut Header,
    next: *mut Header,
}

const ALIGNMENT: usize = align_of::<Header>();
const HEADER_SIZE: usize = size_of::<Header>();

thread_local! {
    // A scoped routing pointer, not an owner. Engine state must discard every
    // alias before dropping the arena. Each thread can route to its own arena.
    static ACTIVE_ARENA: Cell<*mut ArenaState> = const { Cell::new(ptr::null_mut()) };
}

#[derive(Default)]
struct ArenaState {
    head: *mut Header,
    allocations: usize,
    bytes: usize,
}

/// Invocation-owned storage for translated buffers without individual owners.
///
/// The intrusive list gives allocation, resize and explicit release constant
/// bookkeeping work. Dropping an arena releases every remaining payload, even
/// after a Rust unwind. This does not drop objects stored in the raw payloads.
/// Such objects must keep their own typed owners and never allocate here.
///
/// The pinned handle owns a separate raw allocation. Headers retain that raw
/// state's provenance, which handle moves and reborrows cannot invalidate.
/// No `Box<ArenaState>` or whole-state reference exists while buffers are live.
/// The handle is thread-confined and cannot be shared across threads.
pub(crate) struct AllocationArena {
    state: NonNull<ArenaState>,
    _thread_confined: PhantomData<Rc<()>>,
    _pinned: PhantomPinned,
}

impl AllocationArena {
    pub(crate) fn new() -> Pin<Box<Self>> {
        Box::pin(Self {
            // Relinquish Box's unique payload borrow before the first header
            // or routing pointer is formed. Only Drop reconstructs the Box.
            state: NonNull::new(Box::into_raw(Box::new(ArenaState::default())))
                .expect("Box allocation is non-null"),
            _thread_confined: PhantomData,
            _pinned: PhantomPinned,
        })
    }

    /// Route newly allocated buffers to this owner for the closure's duration.
    /// Nested scopes are rejected before replacing the active routing pointer.
    /// Buffers allocated earlier retain their original owner when resized.
    pub(crate) fn scope<T>(self: Pin<&mut Self>, operation: impl FnOnce() -> T) -> T {
        struct RoutingGuard;
        impl Drop for RoutingGuard {
            fn drop(&mut self) {
                ACTIVE_ARENA.with(|active| active.set(ptr::null_mut()));
            }
        }

        ACTIVE_ARENA.with(|active| {
            assert!(active.get().is_null(), "nested allocation arena scope");
            active.set(self.as_ref().get_ref().state.as_ptr());
        });
        let _routing = RoutingGuard;
        operation()
    }

    pub(crate) fn live_allocations(&self) -> usize {
        // Reading the handle does not reborrow the independent raw state.
        // No operation mutates its list concurrently, since it is !Send/!Sync.
        unsafe { (*self.state.as_ptr()).allocations }
    }

    pub(crate) fn live_bytes(&self) -> usize {
        unsafe { (*self.state.as_ptr()).bytes }
    }
}

impl Drop for AllocationArena {
    fn drop(&mut self) {
        let state = self.state.as_ptr();
        while !unsafe { (*state).head }.is_null() {
            // SAFETY: the list contains only live allocations created here.
            // An explicit free unlinks first. A successful realloc replaces
            // its entry before returning. Payloads contain no Rust drop glue.
            unsafe {
                let header = (*state).head;
                (*state).head = (*header).next;
                let layout = allocation_layout((*header).payload_size).unwrap_unchecked();
                dealloc(header.cast(), layout);
            }
        }
        unsafe {
            (*state).allocations = 0;
            (*state).bytes = 0;
            // The private scope has unregistered routing and every payload
            // alias has ended. Reconstruct raw state ownership exactly once.
            drop(Box::from_raw(state));
        }
    }
}

unsafe fn register(header: *mut Header, owner: *mut ArenaState) {
    if owner.is_null() {
        return;
    }
    unsafe {
        (*header).next = (*owner).head;
        if !(*owner).head.is_null() {
            (*(*owner).head).previous = header;
        }
        (*owner).head = header;
        (*owner).allocations += 1;
        (*owner).bytes += (*header).payload_size;
    }
}

unsafe fn unlink(header: *mut Header) {
    unsafe {
        let owner = (*header).owner;
        if owner.is_null() {
            return;
        }
        if (*header).previous.is_null() {
            (*owner).head = (*header).next;
        } else {
            (*(*header).previous).next = (*header).next;
        }
        if !(*header).next.is_null() {
            (*(*header).next).previous = (*header).previous;
        }
        (*owner).allocations -= 1;
        (*owner).bytes -= (*header).payload_size;
    }
}

fn allocation_layout(size: usize) -> Option<Layout> {
    let total = size.max(1).checked_add(HEADER_SIZE)?;
    Layout::from_size_align(total, ALIGNMENT).ok()
}

/// Allocate uninitialized payload bytes, or return null on overflow or failure.
/// Zero bytes still produce a unique allocation with one usable byte.
///
/// # Safety
/// The caller must initialize bytes before reading them and release the returned
/// pointer through this module. The payload supports alignments up to 16 bytes.
/// When allocated in an arena scope, all accesses must end before that arena is
/// dropped. An explicit release must happen before dropping the arena, or the
/// arena will release it instead.
pub(crate) unsafe fn malloc(size: usize) -> *mut c_void {
    let Some(layout) = allocation_layout(size) else {
        return ptr::null_mut();
    };
    let owner = ACTIVE_ARENA.with(Cell::get);
    if !owner.is_null()
        && (unsafe { (*owner).allocations }.checked_add(1).is_none()
            || unsafe { (*owner).bytes }.checked_add(size.max(1)).is_none())
    {
        return ptr::null_mut();
    }
    let base = unsafe { alloc(layout) };
    if base.is_null() {
        return ptr::null_mut();
    }
    unsafe {
        base.cast::<Header>().write(Header {
            payload_size: size.max(1),
            owner,
            previous: ptr::null_mut(),
            next: ptr::null_mut(),
        });
        register(base.cast(), owner);
        base.add(HEADER_SIZE).cast()
    }
}

/// Resize a buffer, preserving the prefix shared by the old and new sizes.
/// Null allocates a new buffer. Zero retains one usable byte. Failure leaves the
/// original buffer live and unchanged.
///
/// # Safety
/// A non-null pointer must be the exact live payload pointer returned by this
/// module, with no other references used during the resize. It cannot be an
/// interior pointer or storage owned by `Box`, `Vec`, libc or the OS.
pub(crate) unsafe fn realloc(payload: *mut c_void, size: usize) -> *mut c_void {
    if payload.is_null() {
        return unsafe { malloc(size) };
    }
    let Some(new_layout) = allocation_layout(size) else {
        return ptr::null_mut();
    };
    unsafe {
        let base = payload.cast::<u8>().sub(HEADER_SIZE);
        let header = base.cast::<Header>();
        let old_size = (*header).payload_size;
        let owner = (*header).owner;
        let previous = (*header).previous;
        let next = (*header).next;
        if !owner.is_null() {
            let bytes = (*owner).bytes;
            if (bytes - old_size).checked_add(size.max(1)).is_none() {
                return ptr::null_mut();
            }
        }
        // The header was written only after this exact layout was validated.
        let old_layout = allocation_layout(old_size).unwrap_unchecked();
        let new_base = realloc_block(base, old_layout, new_layout.size());
        if new_base.is_null() {
            return ptr::null_mut();
        }
        let new_header = new_base.cast::<Header>();
        new_header.write(Header {
            payload_size: size.max(1),
            owner,
            previous,
            next,
        });
        if !owner.is_null() {
            if previous.is_null() {
                (*owner).head = new_header;
            } else {
                (*previous).next = new_header;
            }
            if !next.is_null() {
                (*next).previous = new_header;
            }
            (*owner).bytes = (*owner).bytes - old_size + size.max(1);
        }
        new_base.add(HEADER_SIZE).cast()
    }
}

/// Release a buffer. Null does nothing.
///
/// # Safety
/// A non-null pointer must be the exact live payload pointer returned by this
/// module. It cannot be an interior pointer, already freed storage, or storage
/// owned by `Box`, `Vec`, libc or the OS. No references may outlive this call.
pub(crate) unsafe fn free(payload: *mut c_void) {
    if payload.is_null() {
        return;
    }
    unsafe {
        let base = payload.cast::<u8>().sub(HEADER_SIZE);
        let size = (*base.cast::<Header>()).payload_size;
        let layout = allocation_layout(size).unwrap_unchecked();
        unlink(base.cast());
        dealloc(base, layout);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    struct Buffer(*mut c_void);

    impl Drop for Buffer {
        fn drop(&mut self) {
            unsafe { free(self.0) };
        }
    }

    #[test]
    fn null_and_zero_sizes_have_defined_behavior() {
        unsafe { free(ptr::null_mut()) };
        let mut buffer = Buffer(unsafe { realloc(ptr::null_mut(), 0) });
        assert!(!buffer.0.is_null());
        unsafe { buffer.0.cast::<u8>().write(197) };
        let resized = unsafe { realloc(buffer.0, 0) };
        assert!(!resized.is_null());
        buffer.0 = resized;
        assert_eq!(unsafe { buffer.0.cast::<u8>().read() }, 197);
    }

    #[test]
    fn oversized_resize_preserves_the_allocation() {
        let buffer = Buffer(unsafe { malloc(32) });
        assert!(!buffer.0.is_null());
        unsafe { ptr::write_bytes(buffer.0.cast::<u8>(), 0x6a, 32) };
        for size in [usize::MAX, isize::MAX as usize] {
            assert!(allocation_layout(size).is_none());
            assert!(unsafe { malloc(size) }.is_null());
            assert!(unsafe { realloc(buffer.0, size) }.is_null());
            assert_eq!(
                unsafe { std::slice::from_raw_parts(buffer.0.cast::<u8>(), 32) },
                &[0x6a; 32]
            );
        }
    }

    #[test]
    fn alignment_covers_engine_records_and_scalars() {
        use crate::generated::pdftexini::{destnameentry, fourquarters, memoryword, objentry};
        use crate::utils::{FdEntry, FmEntry};

        for alignment in [
            align_of::<u128>(),
            align_of::<f64>(),
            align_of::<*mut c_void>(),
            align_of::<memoryword>(),
            align_of::<fourquarters>(),
            align_of::<objentry>(),
            align_of::<destnameentry>(),
            align_of::<FdEntry>(),
            align_of::<FmEntry>(),
        ] {
            assert!(ALIGNMENT >= alignment);
        }
        for size in [0, 1, 3, 15, 16, 17, 31, 512, 4096] {
            let buffer = Buffer(unsafe { malloc(size) });
            assert!(!buffer.0.is_null());
            assert_eq!(buffer.0 as usize % ALIGNMENT, 0);
        }
    }

    #[test]
    fn arena_owns_unreleased_buffers_and_resizes_outside_its_scope() {
        let mut arena = AllocationArena::new();
        let (first, middle, last) = arena
            .as_mut()
            .scope(|| unsafe { (malloc(17), malloc(29), malloc(41)) });
        assert_eq!(arena.live_allocations(), 3);
        assert_eq!(arena.live_bytes(), 87);
        unsafe { free(middle) };
        assert_eq!(arena.live_allocations(), 2);
        assert_eq!(arena.live_bytes(), 58);
        let first = unsafe { realloc(first, 8192) };
        assert!(!first.is_null());
        assert_eq!(arena.live_bytes(), 8233);
        unsafe { free(last) };
        assert_eq!(arena.live_allocations(), 1);
        assert_eq!(arena.live_bytes(), 8192);
        assert!(unsafe { realloc(first, usize::MAX) }.is_null());
        assert_eq!(arena.live_bytes(), 8192);
        // This intentionally leaves one buffer for the arena to release.
        drop(arena);
        assert!(ACTIVE_ARENA.with(Cell::get).is_null());
    }

    #[test]
    fn retained_owner_survives_handle_moves_reborrows_and_unscoped_resize() {
        let mut arena = AllocationArena::new();
        let state = arena.state.as_ptr();
        let handle = arena.as_ref().get_ref() as *const AllocationArena;
        assert_ne!(state.cast::<u8>(), handle.cast_mut().cast::<u8>());
        let mut payload = arena.as_mut().scope(|| unsafe { malloc(17) });
        assert!(!payload.is_null());
        unsafe { ptr::write_bytes(payload.cast::<u8>(), 0x69, 17) };
        let retained_owner =
            unsafe { (*payload.cast::<u8>().sub(HEADER_SIZE).cast::<Header>()).owner };
        assert_eq!(retained_owner, state);

        // Moving the pinning pointer and borrowing the small handle must not
        // create a new exclusive borrow of the retained raw ArenaState.
        let mut moved = arena;
        for size in [32, 8192, 9, 0, 65] {
            assert_eq!(moved.live_allocations(), 1);
            moved
                .as_mut()
                .scope(|| assert_eq!(ACTIVE_ARENA.with(Cell::get), state));
            assert!(ACTIVE_ARENA.with(Cell::get).is_null());
            let old_size = moved.live_bytes();
            let resized = unsafe { realloc(payload, size) };
            assert!(!resized.is_null());
            payload = resized;
            assert_eq!(payload as usize % ALIGNMENT, 0);
            assert_eq!(
                unsafe { (*payload.cast::<u8>().sub(HEADER_SIZE).cast::<Header>()).owner },
                state
            );
            let preserved = old_size.min(size.max(1));
            assert!(
                unsafe { std::slice::from_raw_parts(payload.cast::<u8>(), preserved) }
                    .iter()
                    .all(|&byte| byte == 0x69)
            );
            unsafe { ptr::write_bytes(payload.cast::<u8>(), 0x69, size.max(1)) };
            assert_eq!(moved.live_bytes(), size.max(1));
        }
        let again = moved;
        assert_eq!(again.state.as_ptr(), retained_owner);
        unsafe { free(payload) };
        assert_eq!(again.live_allocations(), 0);
        assert_eq!(again.live_bytes(), 0);
    }

    #[test]
    fn unscoped_release_and_resize_use_the_retained_raw_owner_not_tls() {
        let mut first = AllocationArena::new();
        let mut second = AllocationArena::new();
        let first_state = first.state.as_ptr();
        let (head, tail) = first.as_mut().scope(|| unsafe { (malloc(8), malloc(16)) });
        assert!(!head.is_null() && !tail.is_null());
        let head = unsafe { realloc(head, 33) };
        assert!(!head.is_null());
        assert_eq!(first.live_allocations(), 2);
        assert_eq!(first.live_bytes(), 49);
        assert!(ACTIVE_ARENA.with(Cell::get).is_null());
        second.as_mut().scope(|| unsafe {
            assert_ne!(ACTIVE_ARENA.with(Cell::get), first_state);
            free(tail);
        });
        assert_eq!(first.live_allocations(), 1);
        assert_eq!(first.live_bytes(), 33);
        assert_eq!(second.live_allocations(), 0);
        assert_eq!(second.live_bytes(), 0);
        unsafe { free(head) };
        assert_eq!(first.live_allocations(), 0);
        assert_eq!(first.live_bytes(), 0);
        assert!(ACTIVE_ARENA.with(Cell::get).is_null());
    }

    #[test]
    fn resize_keeps_original_owner_in_a_different_scope() {
        let mut first = AllocationArena::new();
        let mut second = AllocationArena::new();
        let buffer = first.as_mut().scope(|| unsafe { malloc(8) });
        let resized = second.as_mut().scope(|| unsafe { realloc(buffer, 32) });
        assert!(!resized.is_null());
        assert_eq!(first.live_allocations(), 1);
        assert_eq!(first.live_bytes(), 32);
        assert_eq!(second.live_allocations(), 0);
        unsafe { free(resized) };
        assert_eq!(first.live_allocations(), 0);
        assert_eq!(first.live_bytes(), 0);
    }

    #[test]
    fn unwinding_restores_routing_and_preserves_existing_owner() {
        use std::panic::{catch_unwind, AssertUnwindSafe};

        let mut first = AllocationArena::new();
        let mut second = AllocationArena::new();
        let result = catch_unwind(AssertUnwindSafe(|| {
            first.as_mut().scope(|| {
                assert!(!unsafe { malloc(64) }.is_null());
                let rejected = catch_unwind(AssertUnwindSafe(|| {
                    second.as_mut().scope(|| unreachable!());
                }));
                assert!(rejected.is_err());
                assert!(!unsafe { malloc(32) }.is_null());
                panic!("test scope unwinding");
            });
        }));
        assert!(result.is_err());
        assert!(ACTIVE_ARENA.with(Cell::get).is_null());
        assert_eq!(first.live_allocations(), 2);
        assert_eq!(first.live_bytes(), 96);
        assert_eq!(second.live_allocations(), 0);
        second.as_mut().scope(|| {
            let buffer = unsafe { malloc(8) };
            assert!(!buffer.is_null());
            unsafe { free(buffer) };
        });
        assert_eq!(second.live_allocations(), 0);
    }

    #[test]
    fn different_threads_have_independent_routing() {
        let start = std::sync::Arc::new(std::sync::Barrier::new(8));
        let threads = (0..8)
            .map(|index| {
                let start = start.clone();
                std::thread::spawn(move || {
                    let mut arena = AllocationArena::new();
                    arena.as_mut().scope(|| {
                        start.wait();
                        assert!(!unsafe { malloc(16 + index) }.is_null());
                    });
                    assert_eq!(arena.live_allocations(), 1);
                    assert_eq!(arena.live_bytes(), 16 + index);
                })
            })
            .collect::<Vec<_>>();
        for thread in threads {
            thread.join().unwrap();
        }
    }

    proptest! {
        #[test]
        fn arena_tracks_random_release_and_resize_orders(
            operations in prop::collection::vec((0usize..16, 0usize..8193, any::<bool>()), 1..128),
        ) {
            let mut arena = AllocationArena::new();
            let mut slots = [(ptr::null_mut(), 0usize); 16];
            let mut expected_count = 0;
            let mut expected_bytes = 0;
            for (index, size, release) in operations {
                let (buffer, old_size) = slots[index];
                if release {
                    unsafe { free(buffer) };
                    if !buffer.is_null() {
                        expected_count -= 1;
                        expected_bytes -= old_size;
                    }
                    slots[index] = (ptr::null_mut(), 0);
                } else {
                    let resized = arena.as_mut().scope(|| unsafe { realloc(buffer, size) });
                    prop_assert!(!resized.is_null());
                    if buffer.is_null() {
                        expected_count += 1;
                    }
                    expected_bytes = expected_bytes - old_size + size.max(1);
                    slots[index] = (resized, size.max(1));
                }
                prop_assert_eq!(arena.live_allocations(), expected_count);
                prop_assert_eq!(arena.live_bytes(), expected_bytes);
            }
            // Free in slot order, not allocation order, exercising both links.
            for (buffer, _) in slots {
                unsafe { free(buffer) };
            }
            prop_assert_eq!(arena.live_allocations(), 0);
            prop_assert_eq!(arena.live_bytes(), 0);
        }

        #[test]
        fn resize_sequences_preserve_initialized_prefixes(
            operations in prop::collection::vec((0usize..8193, any::<u8>(), any::<bool>()), 1..64),
        ) {
            let mut buffer = Buffer(ptr::null_mut());
            let mut expected = Vec::<u8>::new();
            for (requested, fill, release) in operations {
                if release {
                    unsafe { free(buffer.0) };
                    buffer.0 = ptr::null_mut();
                    expected.clear();
                }
                let resized = unsafe { realloc(buffer.0, requested) };
                prop_assert!(!resized.is_null());
                buffer.0 = resized;
                prop_assert_eq!(buffer.0 as usize % ALIGNMENT, 0);
                let actual = requested.max(1);
                let preserved = expected.len().min(actual);
                let prefix = unsafe { std::slice::from_raw_parts(buffer.0.cast::<u8>(), preserved) };
                prop_assert_eq!(prefix, &expected[..preserved]);
                unsafe { ptr::write_bytes(buffer.0.cast::<u8>(), fill, actual) };
                expected.resize(actual, fill);
                expected.fill(fill);
            }
        }

        #[test]
        fn layouts_reject_overflow_without_wrapping(size in any::<usize>()) {
            let layout = allocation_layout(size);
            let expected = size.max(1).checked_add(HEADER_SIZE)
                .and_then(|total| Layout::from_size_align(total, ALIGNMENT).ok());
            prop_assert_eq!(layout, expected);
            if let Some(layout) = layout {
                prop_assert!(layout.size() >= HEADER_SIZE);
                prop_assert_eq!(layout.align(), ALIGNMENT);
                prop_assert!(layout.size() - HEADER_SIZE >= size);
            }
        }
    }
}
