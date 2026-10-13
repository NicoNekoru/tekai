//! Allocation-free RGBA8 row packing into separate RGB and alpha buffers.

#![warn(clippy::all)]

use std::mem::MaybeUninit;

#[derive(Clone, Copy)]
enum Kernel {
    Portable,
    #[cfg(target_arch = "x86_64")]
    Ssse3,
}

/// A state-free row packer whose private selection retains CPU availability.
/// No caller can construct an unchecked SIMD selection through the safe API.
pub(crate) struct Rgba8RowPacker {
    kernel: Kernel,
}

impl Rgba8RowPacker {
    /// Select once per image, before any row borrows or writes.
    pub(crate) fn new() -> Self {
        #[cfg(target_arch = "x86_64")]
        if let Some(packer) = Self::ssse3_if_available() {
            return packer;
        }
        Self {
            kernel: Kernel::Portable,
        }
    }

    #[cfg(target_arch = "x86_64")]
    fn ssse3_if_available() -> Option<Self> {
        if ::std::is_x86_feature_detected!("ssse3") {
            Some(Self {
                kernel: Kernel::Ssse3,
            })
        } else {
            None
        }
    }

    /// Write every logical RGB and alpha byte without reading either output.
    /// Invalid shapes panic before any write. Successful return proves both
    /// output slices initialized. The slice borrows require disjoint buffers.
    pub(crate) fn pack(
        &self,
        source: &[u8],
        rgb: &mut [MaybeUninit<u8>],
        alpha: &mut [MaybeUninit<u8>],
    ) {
        assert_eq!(source.len(), alpha.len().checked_mul(4).unwrap());
        assert_eq!(rgb.len(), alpha.len().checked_mul(3).unwrap());
        match self.kernel {
            Kernel::Portable => pack_portable(source, rgb, alpha),
            #[cfg(target_arch = "x86_64")]
            Kernel::Ssse3 => {
                // SAFETY The private selection is created only after runtime
                // SSSE3 detection. The exact shapes were checked before writes.
                unsafe { pack_ssse3(source, rgb, alpha) };
            }
        }
    }
}

fn pack_portable(source: &[u8], rgb: &mut [MaybeUninit<u8>], alpha: &mut [MaybeUninit<u8>]) {
    for ((rgba, rgb), alpha) in source
        .as_chunks::<4>()
        .0
        .iter()
        .zip(rgb.as_chunks_mut::<3>().0.iter_mut())
        .zip(alpha.iter_mut())
    {
        for (destination, &byte) in rgb.iter_mut().zip(&rgba[..3]) {
            destination.write(byte);
        }
        alpha.write(rgba[3]);
    }
}

/// # Safety
/// The CPU must support SSSE3. Source length must be four times alpha length,
/// RGB length three times alpha length, and all three slices must be disjoint.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "ssse3")]
unsafe fn pack_ssse3(source: &[u8], rgb: &mut [MaybeUninit<u8>], alpha: &mut [MaybeUninit<u8>]) {
    use std::arch::x86_64::{
        __m128i, _mm_cvtsi128_si32, _mm_loadu_si128, _mm_setr_epi8, _mm_shuffle_epi8,
        _mm_srli_si128, _mm_storel_epi64,
    };

    let rgb_indices = _mm_setr_epi8(
        0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14, -128, -128, -128, -128,
    );
    let alpha_indices = _mm_setr_epi8(
        3, 7, 11, 15, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128, -128,
    );
    let groups = alpha.len() / 4;
    for group in 0..groups {
        // SAFETY Each group has four complete pixels, so its source range is
        // [16g, 16g+16), RGB ranges [12g, 12g+8) and [12g+8, 12g+12), and alpha
        // range [4g, 4g+4). All lie inside the exact, disjoint slices. These
        // unaligned accesses never read outputs or store padding bytes.
        unsafe {
            let rgba = _mm_loadu_si128(source.as_ptr().add(group * 16).cast::<__m128i>());
            let rgb_bytes = _mm_shuffle_epi8(rgba, rgb_indices);
            let alpha_bytes = _mm_shuffle_epi8(rgba, alpha_indices);
            let rgb_group = rgb.as_mut_ptr().add(group * 12);
            _mm_storel_epi64(rgb_group.cast::<__m128i>(), rgb_bytes);
            let final_rgb = _mm_cvtsi128_si32(_mm_srli_si128::<8>(rgb_bytes)) as u32;
            rgb_group.add(8).cast::<u32>().write_unaligned(final_rgb);
            let alpha_word = _mm_cvtsi128_si32(alpha_bytes) as u32;
            alpha
                .as_mut_ptr()
                .add(group * 4)
                .cast::<u32>()
                .write_unaligned(alpha_word);
        }
    }
    let pixels = groups * 4;
    pack_portable(
        &source[pixels * 4..],
        &mut rgb[pixels * 3..],
        &mut alpha[pixels..],
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    const GUARD: usize = 32;

    fn packers() -> Vec<Rgba8RowPacker> {
        let selected = Rgba8RowPacker::new();
        match selected.kernel {
            Kernel::Portable => vec![selected],
            #[cfg(target_arch = "x86_64")]
            Kernel::Ssse3 => vec![
                Rgba8RowPacker {
                    kernel: Kernel::Portable,
                },
                selected,
            ],
        }
    }

    fn uninitialized_output(len: usize, offset: usize, sentinel: u8) -> Vec<MaybeUninit<u8>> {
        let start = GUARD + offset;
        let mut output = Vec::with_capacity(start + len + GUARD);
        output.resize_with(start + len + GUARD, MaybeUninit::uninit);
        for byte in &mut output[..start] {
            byte.write(sentinel);
        }
        for byte in &mut output[start + len..] {
            byte.write(sentinel);
        }
        output
    }

    fn check_guards(output: &[MaybeUninit<u8>], start: usize, len: usize, sentinel: u8) {
        for byte in output[..start].iter().chain(&output[start + len..]) {
            // SAFETY The constructor initialized every guard byte. Packers
            // receive only the logical payload and write initialized values.
            assert_eq!(unsafe { byte.assume_init() }, sentinel);
        }
    }

    proptest! {
        #[test]
        fn rgba_rows_match_bytewise_model_with_uninitialized_outputs_and_guards(
            pixels in proptest::collection::vec(any::<[u8; 4]>(), 0..=1024),
            short_pixel in any::<[u8; 4]>(),
            source_offset in 0_usize..32,
            rgb_offset in 0_usize..32,
            alpha_offset in 0_usize..32,
        ) {
            // Every case covers empty/short rows and all four SIMD tails, in
            // addition to its arbitrary-width row with arbitrary pixel bytes.
            for width in (0..8).chain(std::iter::once(pixels.len())) {
                let source_start = GUARD + source_offset;
                let source_len = width * 4;
                let rgb_len = width * 3;
                let alpha_len = width;
                let mut source = vec![0xA6; source_start + source_len + GUARD];
                let mut expected_rgb = Vec::with_capacity(rgb_len);
                let mut expected_alpha = Vec::with_capacity(alpha_len);
                for index in 0..width {
                    let pixel = pixels.get(index).copied().unwrap_or(short_pixel);
                    for channel in 0..4 {
                        source[source_start + index * 4 + channel] = pixel[channel];
                        if channel == 3 {
                            expected_alpha.push(pixel[channel]);
                        } else {
                            expected_rgb.push(pixel[channel]);
                        }
                    }
                }
                let original_source = source.clone();
                for packer in packers() {
                    let rgb_start = GUARD + rgb_offset;
                    let alpha_start = GUARD + alpha_offset;
                    let mut rgb = uninitialized_output(rgb_len, rgb_offset, 0xD3);
                    let mut alpha = uninitialized_output(alpha_len, alpha_offset, 0x7B);
                    packer.pack(
                        &source[source_start..source_start + source_len],
                        &mut rgb[rgb_start..rgb_start + rgb_len],
                        &mut alpha[alpha_start..alpha_start + alpha_len],
                    );
                    // SAFETY Successful pack initializes every logical output byte.
                    // No initialized payload is inspected before that return.
                    let actual_rgb: Vec<_> = rgb[rgb_start..rgb_start + rgb_len]
                        .iter().map(|byte| unsafe { byte.assume_init() }).collect();
                    let actual_alpha: Vec<_> = alpha[alpha_start..alpha_start + alpha_len]
                        .iter().map(|byte| unsafe { byte.assume_init() }).collect();
                    prop_assert_eq!(&actual_rgb, &expected_rgb);
                    prop_assert_eq!(&actual_alpha, &expected_alpha);
                    prop_assert_eq!(&source, &original_source);
                    check_guards(&rgb, rgb_start, rgb_len, 0xD3);
                    check_guards(&alpha, alpha_start, alpha_len, 0x7B);
                }
            }
        }
    }

    #[test]
    fn invalid_shapes_fail_before_writing_either_output() {
        for packer in packers() {
            for (source_len, rgb_len, alpha_len) in
                [(3, 3, 1), (5, 3, 1), (4, 2, 1), (4, 4, 1), (4, 3, 0)]
            {
                let source = vec![0x91; source_len];
                let mut rgb = vec![MaybeUninit::new(0xD3); rgb_len + 2];
                let mut alpha = vec![MaybeUninit::new(0x7B); alpha_len + 2];
                let failure = catch_unwind(AssertUnwindSafe(|| {
                    packer.pack(
                        &source,
                        &mut rgb[1..rgb_len + 1],
                        &mut alpha[1..alpha_len + 1],
                    );
                }));
                assert!(failure.is_err());
                // SAFETY This failure test initialized all bytes before calling
                // pack, so unchanged logical bytes and guards can be inspected.
                assert!(rgb.iter().all(|byte| unsafe { byte.assume_init() } == 0xD3));
                assert!(alpha
                    .iter()
                    .all(|byte| unsafe { byte.assume_init() } == 0x7B));
            }
        }
    }
}
