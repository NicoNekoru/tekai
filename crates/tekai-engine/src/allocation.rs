//! Rust allocation for the remaining translated raw-pointer buffers.
//!
//! Every buffer consumed here must also be produced here. A header stores its
//! layout beside the buffer, so releasing a buffer needs no pointer registry or
//! lock. The payload starts at a 16-byte boundary, matching the fundamental
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
use std::ffi::c_void;
use std::mem::{align_of, size_of};
use std::ptr;

#[repr(C, align(16))]
struct Header {
    payload_size: usize,
}

const ALIGNMENT: usize = align_of::<Header>();
const HEADER_SIZE: usize = size_of::<Header>();

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
pub(crate) unsafe fn malloc(size: usize) -> *mut c_void {
    let Some(layout) = allocation_layout(size) else {
        return ptr::null_mut();
    };
    let base = unsafe { alloc(layout) };
    if base.is_null() {
        return ptr::null_mut();
    }
    unsafe {
        base.cast::<Header>().write(Header {
            payload_size: size.max(1),
        });
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
        let old_size = (*base.cast::<Header>()).payload_size;
        // The header was written only after this exact layout was validated.
        let old_layout = allocation_layout(old_size).unwrap_unchecked();
        let new_base = realloc_block(base, old_layout, new_layout.size());
        if new_base.is_null() {
            return ptr::null_mut();
        }
        new_base.cast::<Header>().write(Header {
            payload_size: size.max(1),
        });
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

    proptest! {
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
