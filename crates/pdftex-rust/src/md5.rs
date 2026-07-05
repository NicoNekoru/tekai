//! Rust implementation of the small `libmd5` ABI used by pdfTeX support code.

use core::ffi::c_int;
use core::{ptr, slice};

#[repr(C)]
pub struct Md5State {
    count: [u32; 2],
    abcd: [u32; 4],
    buf: [u8; 64],
}

const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
    9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6,
    10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

const K: [u32; 64] = [
    0xd76a_a478,
    0xe8c7_b756,
    0x2420_70db,
    0xc1bd_ceee,
    0xf57c_0faf,
    0x4787_c62a,
    0xa830_4613,
    0xfd46_9501,
    0x6980_98d8,
    0x8b44_f7af,
    0xffff_5bb1,
    0x895c_d7be,
    0x6b90_1122,
    0xfd98_7193,
    0xa679_438e,
    0x49b4_0821,
    0xf61e_2562,
    0xc040_b340,
    0x265e_5a51,
    0xe9b6_c7aa,
    0xd62f_105d,
    0x0244_1453,
    0xd8a1_e681,
    0xe7d3_fbc8,
    0x21e1_cde6,
    0xc337_07d6,
    0xf4d5_0d87,
    0x455a_14ed,
    0xa9e3_e905,
    0xfcef_a3f8,
    0x676f_02d9,
    0x8d2a_4c8a,
    0xfffa_3942,
    0x8771_f681,
    0x6d9d_6122,
    0xfde5_380c,
    0xa4be_ea44,
    0x4bde_cfa9,
    0xf6bb_4b60,
    0xbebf_bc70,
    0x289b_7ec6,
    0xeaa1_27fa,
    0xd4ef_3085,
    0x0488_1d05,
    0xd9d4_d039,
    0xe6db_99e5,
    0x1fa2_7cf8,
    0xc4ac_5665,
    0xf429_2244,
    0x432a_ff97,
    0xab94_23a7,
    0xfc93_a039,
    0x655b_59c3,
    0x8f0c_cc92,
    0xffef_f47d,
    0x8584_5dd1,
    0x6fa8_7e4f,
    0xfe2c_e6e0,
    0xa301_4314,
    0x4e08_11a1,
    0xf753_7e82,
    0xbd3a_f235,
    0x2ad7_d2bb,
    0xeb86_d391,
];

#[no_mangle]
pub unsafe extern "C" fn md5_init(state: *mut Md5State) {
    let state = unsafe { &mut *state };
    state.count = [0, 0];
    state.abcd = [0x6745_2301, 0xefcd_ab89, 0x98ba_dcfe, 0x1032_5476];
    state.buf = [0; 64];
}

#[no_mangle]
pub unsafe extern "C" fn md5_append(state: *mut Md5State, data: *const u8, nbytes: c_int) {
    if nbytes <= 0 {
        return;
    }
    let input = unsafe { slice::from_raw_parts(data, nbytes as usize) };
    let state = unsafe { &mut *state };
    append(state, input);
}

#[no_mangle]
pub unsafe extern "C" fn md5_finish(state: *mut Md5State, digest: *mut u8) {
    let state = unsafe { &mut *state };
    let mut length = [0_u8; 8];
    for (i, byte) in length.iter_mut().enumerate() {
        *byte = (state.count[i >> 2] >> ((i & 3) << 3)) as u8;
    }

    let pad_len = (((55_u32.wrapping_sub(state.count[0] >> 3)) & 63) + 1) as usize;
    append(state, &PADDING[..pad_len]);
    append(state, &length);

    for i in 0..16 {
        unsafe {
            *digest.add(i) = (state.abcd[i >> 2] >> ((i & 3) << 3)) as u8;
        }
    }
}

const PADDING: [u8; 64] = {
    let mut pad = [0_u8; 64];
    pad[0] = 0x80;
    pad
};

fn append(state: &mut Md5State, mut input: &[u8]) {
    let offset = ((state.count[0] >> 3) & 63) as usize;
    let nbytes = input.len() as u32;
    let nbits = nbytes << 3;

    state.count[1] = state.count[1].wrapping_add(nbytes >> 29);
    state.count[0] = state.count[0].wrapping_add(nbits);
    if state.count[0] < nbits {
        state.count[1] = state.count[1].wrapping_add(1);
    }

    if offset != 0 {
        let copy = input.len().min(64 - offset);
        state.buf[offset..offset + copy].copy_from_slice(&input[..copy]);
        if offset + copy < 64 {
            return;
        }
        let block = state.buf;
        process(state, &block);
        input = &input[copy..];
    }

    while input.len() >= 64 {
        process(state, (&input[..64]).try_into().expect("slice length is fixed"));
        input = &input[64..];
    }

    if !input.is_empty() {
        unsafe {
            ptr::copy_nonoverlapping(input.as_ptr(), state.buf.as_mut_ptr(), input.len());
        }
    }
}

fn process(state: &mut Md5State, block: &[u8; 64]) {
    let mut x = [0_u32; 16];
    for (slot, chunk) in x.iter_mut().zip(block.chunks_exact(4)) {
        *slot = u32::from_le_bytes(chunk.try_into().expect("chunk length is fixed"));
    }

    let mut a = state.abcd[0];
    let mut b = state.abcd[1];
    let mut c = state.abcd[2];
    let mut d = state.abcd[3];

    for i in 0..64 {
        let (f, g) = match i {
            0..=15 => (((b & c) | ((!b) & d)), i),
            16..=31 => (((b & d) | (c & (!d))), (5 * i + 1) & 15),
            32..=47 => ((b ^ c ^ d), (3 * i + 5) & 15),
            _ => ((c ^ (b | (!d))), (7 * i) & 15),
        };

        let next = a
            .wrapping_add(f)
            .wrapping_add(K[i])
            .wrapping_add(x[g])
            .rotate_left(S[i])
            .wrapping_add(b);
        a = d;
        d = c;
        c = b;
        b = next;
    }

    state.abcd[0] = state.abcd[0].wrapping_add(a);
    state.abcd[1] = state.abcd[1].wrapping_add(b);
    state.abcd[2] = state.abcd[2].wrapping_add(c);
    state.abcd[3] = state.abcd[3].wrapping_add(d);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest_hex(input: &[u8]) -> String {
        let mut state = Md5State {
            count: [0; 2],
            abcd: [0; 4],
            buf: [0; 64],
        };
        let mut digest = [0_u8; 16];
        unsafe {
            md5_init(&mut state);
            md5_append(&mut state, input.as_ptr(), input.len() as c_int);
            md5_finish(&mut state, digest.as_mut_ptr());
        }
        digest.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    #[test]
    fn md5_matches_rfc_vectors() {
        let cases = [
            (b"".as_slice(), "d41d8cd98f00b204e9800998ecf8427e"),
            (b"a".as_slice(), "0cc175b9c0f1b6a831c399e269772661"),
            (b"abc".as_slice(), "900150983cd24fb0d6963f7d28e17f72"),
            (
                b"message digest".as_slice(),
                "f96b697d7cb7938d525a2f31aaf161d0",
            ),
            (
                b"abcdefghijklmnopqrstuvwxyz".as_slice(),
                "c3fcd3d76192e4007dfb496cca67e13b",
            ),
        ];

        for (input, expected) in cases {
            assert_eq!(digest_hex(input), expected);
        }
    }

    #[test]
    fn md5_append_supports_fragmented_input() {
        let input = b"abcdefghijklmnopqrstuvwxyz";
        let mut state = Md5State {
            count: [0; 2],
            abcd: [0; 4],
            buf: [0; 64],
        };
        let mut digest = [0_u8; 16];
        unsafe {
            md5_init(&mut state);
            md5_append(&mut state, input[..3].as_ptr(), 3);
            md5_append(&mut state, input[3..9].as_ptr(), 6);
            md5_append(&mut state, input[9..].as_ptr(), (input.len() - 9) as c_int);
            md5_finish(&mut state, digest.as_mut_ptr());
        }
        let hex: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
        assert_eq!(hex, "c3fcd3d76192e4007dfb496cca67e13b");
    }
}
