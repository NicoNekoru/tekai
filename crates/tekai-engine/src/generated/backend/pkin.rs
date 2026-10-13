// Rust translation and subsequent modifications for Tekai.
// Upstream TeX Live source revision 52dac44dde13a8310bbb8231e81c58be28cebe20.
// Origin texk/web2c/pdftexdir/pkin.c.
// GPL-2.0-or-later.
// Modified for Tekai. Licensing and provenance notice restored 2026-10-13.
// Original copyrights and license terms are retained in the crate's
// NOTICE.md and licenses/upstream-notices.txt.

use crate::allocation::free;
use crate::openclose::xmalloc;
use crate::utils::xgetc;

pub type __int64_t = i64;
pub type __darwin_size_t = usize;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
pub type boolean = ::core::ffi::c_int;
pub type address = *mut ::core::ffi::c_void;
pub type integer = ::core::ffi::c_int;
pub type halfword = integer;
pub type shalfword = ::core::ffi::c_short;
pub use crate::backend_types::chardesc;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
unsafe fn pkbyte() -> shalfword {
    let __tekai_state_generated_backend_writet3: *mut crate::generated::backend::writet3::State =
        unsafe { crate::generated::backend::writet3::state() };

    let mut i: shalfword = 0;
    i = xgetc((*__tekai_state_generated_backend_writet3).t3_file) as shalfword;
    if i as ::core::ffi::c_int == EOF {
        crate::utils::pdftex_fail_args(
            b"unexpected eof in pk file\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
    }
    return i;
}
unsafe fn pkduo() -> integer {
    let mut i: integer = 0;
    i = pkbyte() as integer;
    if i > 127 as ::core::ffi::c_int {
        i -= 256 as ::core::ffi::c_int;
    }
    i = (i as ::core::ffi::c_int * 256 as ::core::ffi::c_int + pkbyte() as ::core::ffi::c_int)
        as integer;
    return i;
}
unsafe fn pktrio() -> integer {
    let mut i: integer = 0;
    i = pkbyte() as integer;
    if i > 127 as ::core::ffi::c_int {
        i -= 256 as ::core::ffi::c_int;
    }
    i = (i as ::core::ffi::c_int * 256 as ::core::ffi::c_int + pkbyte() as ::core::ffi::c_int)
        as integer;
    i = (i as ::core::ffi::c_int * 256 as ::core::ffi::c_int + pkbyte() as ::core::ffi::c_int)
        as integer;
    return i;
}
unsafe fn pkquad() -> integer {
    let mut i: integer = 0;
    i = pkbyte() as integer;
    if i > 127 as ::core::ffi::c_int {
        i -= 256 as ::core::ffi::c_int;
    }
    i = (i as ::core::ffi::c_int * 256 as ::core::ffi::c_int + pkbyte() as ::core::ffi::c_int)
        as integer;
    i = (i as ::core::ffi::c_int * 256 as ::core::ffi::c_int + pkbyte() as ::core::ffi::c_int)
        as integer;
    i = (i as ::core::ffi::c_int * 256 as ::core::ffi::c_int + pkbyte() as ::core::ffi::c_int)
        as integer;
    return i;
}

unsafe fn getnyb() -> shalfword {
    let __tekai_state_generated_backend_pkin: *mut crate::generated::backend::pkin::State =
        unsafe { crate::generated::backend::pkin::state() };

    let mut temp: halfword = 0;
    if (*__tekai_state_generated_backend_pkin).bitweight == 0 as ::core::ffi::c_int {
        (*__tekai_state_generated_backend_pkin).bitweight = 16 as ::core::ffi::c_int as halfword;
        (*__tekai_state_generated_backend_pkin).inputbyte = pkbyte() as halfword;
        temp = (*__tekai_state_generated_backend_pkin).inputbyte >> 4 as ::core::ffi::c_int;
    } else {
        (*__tekai_state_generated_backend_pkin).bitweight = 0 as ::core::ffi::c_int as halfword;
        temp = ((*__tekai_state_generated_backend_pkin).inputbyte as ::core::ffi::c_int
            & 15 as ::core::ffi::c_int) as halfword;
    }
    return temp as shalfword;
}
unsafe fn getbit() -> boolean {
    let __tekai_state_generated_backend_pkin: *mut crate::generated::backend::pkin::State =
        unsafe { crate::generated::backend::pkin::state() };

    (*__tekai_state_generated_backend_pkin).bitweight >>= 1 as ::core::ffi::c_int;
    if (*__tekai_state_generated_backend_pkin).bitweight == 0 as ::core::ffi::c_int {
        (*__tekai_state_generated_backend_pkin).inputbyte = pkbyte() as halfword;
        (*__tekai_state_generated_backend_pkin).bitweight = 128 as ::core::ffi::c_int as halfword;
    }
    return (*__tekai_state_generated_backend_pkin).inputbyte as boolean
        & (*__tekai_state_generated_backend_pkin).bitweight as boolean;
}

unsafe fn pkpackednum() -> halfword {
    let __tekai_state_generated_backend_pkin: *mut crate::generated::backend::pkin::State =
        unsafe { crate::generated::backend::pkin::state() };

    let mut i: halfword = 0;
    let mut j: halfword = 0;
    i = getnyb() as halfword;
    if i == 0 as ::core::ffi::c_int {
        loop {
            j = getnyb() as halfword;
            i += 1;
            if j != 0 as ::core::ffi::c_int {
                break;
            }
        }
        if i > 3 as ::core::ffi::c_int {
            return handlehuge(i, j);
        } else {
            while i > 0 as ::core::ffi::c_int {
                j = (j as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + getnyb() as ::core::ffi::c_int) as halfword;
                i -= 1;
            }
            return j - 15 as halfword
                + (13 as halfword - (*__tekai_state_generated_backend_pkin).dynf) * 16 as halfword
                + (*__tekai_state_generated_backend_pkin).dynf;
        }
    } else if i <= (*__tekai_state_generated_backend_pkin).dynf {
        return i;
    } else if i < 14 as ::core::ffi::c_int {
        return (i - (*__tekai_state_generated_backend_pkin).dynf - 1 as halfword) * 16 as halfword
            + getnyb() as halfword
            + (*__tekai_state_generated_backend_pkin).dynf
            + 1 as halfword;
    } else {
        if i == 14 as ::core::ffi::c_int {
            (*__tekai_state_generated_backend_pkin).repeatcount = pkpackednum();
        } else {
            (*__tekai_state_generated_backend_pkin).repeatcount =
                1 as ::core::ffi::c_int as halfword;
        }
        return Some(
            (*__tekai_state_generated_backend_pkin)
                .realfunc
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")();
    };
}
unsafe fn rest() -> halfword {
    let __tekai_state_generated_backend_pkin: *mut crate::generated::backend::pkin::State =
        unsafe { crate::generated::backend::pkin::state() };

    let mut i: halfword = 0;
    if (*__tekai_state_generated_backend_pkin).pk_remainder < 0 as ::core::ffi::c_long {
        (*__tekai_state_generated_backend_pkin).pk_remainder =
            -(*__tekai_state_generated_backend_pkin).pk_remainder;
        return 0 as halfword;
    } else if (*__tekai_state_generated_backend_pkin).pk_remainder > 0 as ::core::ffi::c_long {
        if (*__tekai_state_generated_backend_pkin).pk_remainder > 4000 as ::core::ffi::c_long {
            (*__tekai_state_generated_backend_pkin).pk_remainder =
                4000 as ::core::ffi::c_long - (*__tekai_state_generated_backend_pkin).pk_remainder;
            return 4000 as halfword;
        } else {
            i = (*__tekai_state_generated_backend_pkin).pk_remainder as halfword;
            (*__tekai_state_generated_backend_pkin).pk_remainder = 0 as ::core::ffi::c_long;
            (*__tekai_state_generated_backend_pkin).realfunc =
                Some(pkpackednum as unsafe fn() -> halfword) as Option<unsafe fn() -> halfword>;
            return i;
        }
    } else {
        crate::utils::pdftex_fail_args(
            b"shouldn't happen\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
    };
}
unsafe fn handlehuge(mut i: halfword, mut k: halfword) -> halfword {
    let __tekai_state_generated_backend_pkin: *mut crate::generated::backend::pkin::State =
        unsafe { crate::generated::backend::pkin::state() };

    let mut j: ::core::ffi::c_long = k as ::core::ffi::c_long;
    while i != 0 {
        j = (j << 4 as ::core::ffi::c_long) + getnyb() as ::core::ffi::c_long;
        i -= 1;
    }
    (*__tekai_state_generated_backend_pkin).pk_remainder = j - 15 as ::core::ffi::c_long
        + ((13 as ::core::ffi::c_int
            - (*__tekai_state_generated_backend_pkin).dynf as ::core::ffi::c_int)
            * 16 as ::core::ffi::c_int) as ::core::ffi::c_long
        + (*__tekai_state_generated_backend_pkin).dynf as ::core::ffi::c_long;
    (*__tekai_state_generated_backend_pkin).realfunc =
        Some(rest as unsafe fn() -> halfword) as Option<unsafe fn() -> halfword>;
    return rest();
}

unsafe fn unpack(mut cd: *mut chardesc) {
    let __tekai_state_generated_backend_pkin: *mut crate::generated::backend::pkin::State =
        unsafe { crate::generated::backend::pkin::state() };

    let mut i: integer = 0;
    let mut j: integer = 0;
    let mut word: halfword = 0;
    let mut wordweight: halfword = 0;
    let mut raster: *mut halfword = ::core::ptr::null_mut::<halfword>();
    let mut rowsleft: shalfword = 0;
    let mut turnon: boolean = 0;
    let mut hbit: shalfword = 0;
    let mut count: halfword = 0;
    let mut wordwidth: shalfword = 0;
    wordwidth = (((*cd).cwidth as ::core::ffi::c_int + 15 as ::core::ffi::c_int)
        / 16 as ::core::ffi::c_int) as shalfword;
    i = ((2 as integer * (*cd).cheight) as ::core::ffi::c_long * wordwidth as ::core::ffi::c_long)
        as integer;
    if i <= 0 as ::core::ffi::c_int {
        i = 2 as ::core::ffi::c_int as integer;
    }
    if i > (*cd).rastersize {
        if !(*cd).raster.is_null() {
            free((*cd).raster as *mut ::core::ffi::c_void);
        }
        (*cd).raster = ::core::ptr::null_mut::<halfword>();
        (*cd).rastersize = i;
        (*cd).raster = xmalloc(
            ((*cd).rastersize as size_t).wrapping_mul(::core::mem::size_of::<halfword>() as size_t),
        ) as *mut halfword;
    }
    raster = (*cd).raster;
    (*__tekai_state_generated_backend_pkin).realfunc =
        Some(pkpackednum as unsafe fn() -> halfword) as Option<unsafe fn() -> halfword>;
    (*__tekai_state_generated_backend_pkin).dynf = ((*__tekai_state_generated_backend_pkin).flagbyte
        as ::core::ffi::c_int
        / 16 as ::core::ffi::c_int) as halfword;
    turnon = ((*__tekai_state_generated_backend_pkin).flagbyte as ::core::ffi::c_int
        & 8 as ::core::ffi::c_int) as boolean;
    if (*__tekai_state_generated_backend_pkin).dynf == 14 as ::core::ffi::c_int {
        (*__tekai_state_generated_backend_pkin).bitweight = 0 as ::core::ffi::c_int as halfword;
        i = 1 as ::core::ffi::c_int as integer;
        while i <= (*cd).cheight {
            word = 0 as ::core::ffi::c_int as halfword;
            wordweight = 32768 as ::core::ffi::c_int as halfword;
            j = 1 as ::core::ffi::c_int as integer;
            while j <= (*cd).cwidth {
                if getbit() != 0 {
                    word += wordweight;
                }
                wordweight >>= 1 as ::core::ffi::c_int;
                if wordweight == 0 as ::core::ffi::c_int {
                    let fresh1 = raster;
                    raster = raster.offset(1);
                    *fresh1 = word;
                    word = 0 as ::core::ffi::c_int as halfword;
                    wordweight = 32768 as ::core::ffi::c_int as halfword;
                }
                j += 1;
            }
            if wordweight != 32768 as ::core::ffi::c_int {
                let fresh2 = raster;
                raster = raster.offset(1);
                *fresh2 = word;
            }
            i += 1;
        }
    } else {
        rowsleft = (*cd).cheight as shalfword;
        hbit = (*cd).cwidth as shalfword;
        (*__tekai_state_generated_backend_pkin).repeatcount = 0 as ::core::ffi::c_int as halfword;
        wordweight = 16 as ::core::ffi::c_int as halfword;
        word = 0 as ::core::ffi::c_int as halfword;
        (*__tekai_state_generated_backend_pkin).bitweight = 0 as ::core::ffi::c_int as halfword;
        while rowsleft as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            count = Some(
                (*__tekai_state_generated_backend_pkin)
                    .realfunc
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")();
            while count != 0 as ::core::ffi::c_int {
                if count < wordweight && count < hbit as ::core::ffi::c_int {
                    if turnon != 0 {
                        word += (*__tekai_state_generated_backend_pkin).gpower[wordweight as usize]
                            - (*__tekai_state_generated_backend_pkin).gpower
                                [(wordweight - count) as usize];
                    }
                    hbit = (hbit as ::core::ffi::c_int - count as ::core::ffi::c_int) as shalfword;
                    wordweight -= count;
                    count = 0 as ::core::ffi::c_int as halfword;
                } else if count >= hbit as ::core::ffi::c_int
                    && hbit as ::core::ffi::c_int <= wordweight
                {
                    if turnon != 0 {
                        word += (*__tekai_state_generated_backend_pkin).gpower[wordweight as usize]
                            - (*__tekai_state_generated_backend_pkin).gpower[(wordweight
                                as ::core::ffi::c_int
                                - hbit as ::core::ffi::c_int)
                                as usize];
                    }
                    let fresh3 = raster;
                    raster = raster.offset(1);
                    *fresh3 = word;
                    i = 1 as ::core::ffi::c_int as integer;
                    while i <= (*__tekai_state_generated_backend_pkin).repeatcount {
                        j = 1 as ::core::ffi::c_int as integer;
                        while j <= wordwidth as ::core::ffi::c_int {
                            *raster = *raster.offset(-(wordwidth as ::core::ffi::c_int as isize));
                            raster = raster.offset(1);
                            j += 1;
                        }
                        i += 1;
                    }
                    rowsleft = (rowsleft as ::core::ffi::c_int
                        - ((*__tekai_state_generated_backend_pkin).repeatcount
                            as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int)) as shalfword;
                    (*__tekai_state_generated_backend_pkin).repeatcount =
                        0 as ::core::ffi::c_int as halfword;
                    word = 0 as ::core::ffi::c_int as halfword;
                    wordweight = 16 as ::core::ffi::c_int as halfword;
                    count -= hbit as ::core::ffi::c_int;
                    hbit = (*cd).cwidth as shalfword;
                } else {
                    if turnon != 0 {
                        word += (*__tekai_state_generated_backend_pkin).gpower[wordweight as usize];
                    }
                    let fresh4 = raster;
                    raster = raster.offset(1);
                    *fresh4 = word;
                    word = 0 as ::core::ffi::c_int as halfword;
                    count -= wordweight;
                    hbit = (hbit as ::core::ffi::c_int - wordweight as ::core::ffi::c_int)
                        as shalfword;
                    wordweight = 16 as ::core::ffi::c_int as halfword;
                }
            }
            turnon = (turnon == 0) as ::core::ffi::c_int as boolean;
        }
        if rowsleft as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            || hbit as integer != (*cd).cwidth
        {
            crate::utils::pdftex_fail_args(
                b"error while unpacking; more bits than required\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
        }
    };
}
pub unsafe fn readchar(mut check_preamble: boolean, mut cd: *mut chardesc) -> ::core::ffi::c_int {
    let __tekai_state_generated_backend_pkin: *mut crate::generated::backend::pkin::State =
        unsafe { crate::generated::backend::pkin::state() };

    let mut i: shalfword = 0;
    let mut k: integer = 0;
    let mut length: integer = 0 as integer;
    if check_preamble != 0 {
        if pkbyte() as ::core::ffi::c_int != 247 as ::core::ffi::c_int {
            crate::utils::pdftex_fail_args(
                b"bad pk file, expected pre\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        if pkbyte() as ::core::ffi::c_int != 89 as ::core::ffi::c_int {
            crate::utils::pdftex_fail_args(
                b"bad version of pk file\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        i = pkbyte();
        while i as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            pkbyte();
            i -= 1;
        }
        pkquad();
        k = pkquad();
        k = pkquad();
        k = pkquad();
    }
    loop {
        (*__tekai_state_generated_backend_pkin).flagbyte = pkbyte() as halfword;
        if !((*__tekai_state_generated_backend_pkin).flagbyte != 245 as ::core::ffi::c_int) {
            break;
        }
        if (*__tekai_state_generated_backend_pkin).flagbyte < 240 as ::core::ffi::c_int {
            match (*__tekai_state_generated_backend_pkin).flagbyte as ::core::ffi::c_int
                & 7 as ::core::ffi::c_int
            {
                0 | 1 | 2 | 3 => {
                    length = (((*__tekai_state_generated_backend_pkin).flagbyte
                        as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int)
                        * 256 as ::core::ffi::c_int
                        + pkbyte() as ::core::ffi::c_int
                        - 3 as ::core::ffi::c_int) as integer;
                    (*cd).charcode = pkbyte() as integer;
                    pktrio();
                    (*cd).xescape = pkbyte() as integer;
                    (*cd).cwidth = pkbyte() as integer;
                    (*cd).cheight = pkbyte() as integer;
                    (*cd).xoff = pkbyte() as integer;
                    (*cd).yoff = pkbyte() as integer;
                    if (*cd).xoff > 127 as ::core::ffi::c_int {
                        (*cd).xoff -= 256 as ::core::ffi::c_int;
                    }
                    if (*cd).yoff > 127 as ::core::ffi::c_int {
                        (*cd).yoff -= 256 as ::core::ffi::c_int;
                    }
                }
                4 | 5 | 6 => {
                    length = (((*__tekai_state_generated_backend_pkin).flagbyte
                        as ::core::ffi::c_int
                        & 3 as ::core::ffi::c_int)
                        as ::core::ffi::c_long
                        * 65536 as ::core::ffi::c_long
                        + pkbyte() as ::core::ffi::c_long * 256 as ::core::ffi::c_long)
                        as integer;
                    length = ((length as ::core::ffi::c_int + pkbyte() as ::core::ffi::c_int)
                        as ::core::ffi::c_long
                        - 4 as ::core::ffi::c_long) as integer;
                    (*cd).charcode = pkbyte() as integer;
                    pktrio();
                    (*cd).xescape = pkduo();
                    (*cd).cwidth = pkduo();
                    (*cd).cheight = pkduo();
                    (*cd).xoff = pkduo();
                    (*cd).yoff = pkduo();
                }
                7 => {
                    length =
                        (pkquad() as ::core::ffi::c_long - 9 as ::core::ffi::c_long) as integer;
                    (*cd).charcode = pkquad();
                    pkquad();
                    (*cd).xescape = pkquad();
                    k = pkquad();
                    (*cd).cwidth = pkquad();
                    (*cd).cheight = pkquad();
                    (*cd).xoff = pkquad();
                    (*cd).yoff = pkquad();
                }
                _ => {}
            }
            if length <= 0 as ::core::ffi::c_int {
                crate::utils::pdftex_fail_args(
                    b"packet length (%i) too small\0" as *const u8 as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(length)],
                );
            }
            unpack(cd);
            return 1 as ::core::ffi::c_int;
        } else {
            k = 0 as ::core::ffi::c_int as integer;
            let mut current_block_59: u64;
            match (*__tekai_state_generated_backend_pkin).flagbyte {
                243 => {
                    k = pkbyte() as integer;
                    if k > 127 as ::core::ffi::c_int {
                        k -= 256 as ::core::ffi::c_int;
                    }
                    current_block_59 = 6818318202340592218;
                }
                242 => {
                    current_block_59 = 6818318202340592218;
                }
                241 => {
                    current_block_59 = 7413763809724399682;
                }
                240 => {
                    current_block_59 = 9945727987654589790;
                }
                244 => {
                    k = pkquad();
                    current_block_59 = 11777552016271000781;
                }
                246 => {
                    current_block_59 = 11777552016271000781;
                }
                _ => {
                    crate::utils::pdftex_fail_args(
                        b"unexpected command (%i)\0" as *const u8 as *const ::core::ffi::c_char,
                        &[crate::utils::PrintfArg::from(
                            (*__tekai_state_generated_backend_pkin).flagbyte,
                        )],
                    );
                }
            }
            match current_block_59 {
                6818318202340592218 => {
                    k = (k as ::core::ffi::c_int * 256 as ::core::ffi::c_int
                        + pkbyte() as ::core::ffi::c_int) as integer;
                    current_block_59 = 7413763809724399682;
                }
                _ => {}
            }
            match current_block_59 {
                7413763809724399682 => {
                    k = (k as ::core::ffi::c_int * 256 as ::core::ffi::c_int
                        + pkbyte() as ::core::ffi::c_int) as integer;
                    current_block_59 = 9945727987654589790;
                }
                _ => {}
            }
            match current_block_59 {
                9945727987654589790 => {
                    k = (k as ::core::ffi::c_int * 256 as ::core::ffi::c_int
                        + pkbyte() as ::core::ffi::c_int) as integer;
                    loop {
                        let fresh0 = k;
                        k = k - 1;
                        if !(fresh0 > 0 as ::core::ffi::c_int) {
                            break;
                        }
                        i = pkbyte();
                    }
                }
                _ => {}
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;

pub(crate) struct State {
    inputbyte: halfword,
    flagbyte: halfword,
    bitweight: halfword,
    dynf: halfword,
    repeatcount: halfword,
    realfunc: Option<unsafe fn() -> halfword>,
    pk_remainder: ::core::ffi::c_long,
    gpower: [halfword; 17],
}

#[inline(always)]
fn __tekai_require_copy<T: Copy>() {}

pub(crate) struct StateInitialization {
    destination: *mut State,
    initialized: [bool; 8],
}
#[inline]
unsafe fn drop_initialized<T>(pointer: *mut T) {
    if core::mem::needs_drop::<T>() {
        unsafe { core::ptr::drop_in_place(pointer) };
    }
}

impl Drop for StateInitialization {
    fn drop(&mut self) {
        unsafe {
            if self.initialized[7] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).gpower));
            }
            if self.initialized[6] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pk_remainder));
            }
            if self.initialized[5] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).realfunc));
            }
            if self.initialized[4] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).repeatcount));
            }
            if self.initialized[3] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).dynf));
            }
            if self.initialized[2] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).bitweight));
            }
            if self.initialized[1] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).flagbyte));
            }
            if self.initialized[0] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).inputbyte));
            }
        }
    }
}
impl StateInitialization {
    pub(crate) unsafe fn begin(destination: *mut State) -> Self {
        Self {
            destination,
            initialized: [false; 8],
        }
    }
    pub(crate) fn disarm(&mut self) {
        self.initialized.fill(false);
    }
    pub(crate) unsafe fn initialize_fields(
        &mut self,
        invocation: *const crate::invocation_state::Storage,
    ) {
        let destination = self.destination;
        let _ = invocation;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).inputbyte).write(0);
        self.initialized[0] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).flagbyte).write(0);
        self.initialized[1] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).bitweight).write(0);
        self.initialized[2] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).dynf).write(0);
        self.initialized[3] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).repeatcount).write(0);
        self.initialized[4] = true;
        const {
            assert!(core::mem::size_of::<Option<unsafe fn() -> halfword>>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).realfunc).write(None);
        self.initialized[5] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_long>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pk_remainder).write(0);
        self.initialized[6] = true;
        const {
            assert!((17) == 17);
        }
        __tekai_require_copy::<halfword>();
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).gpower[0]).write(0 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).gpower[1]).write(1 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).gpower[2]).write(3 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).gpower[3]).write(7 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).gpower[4]).write(15 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).gpower[5]).write(31 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).gpower[6]).write(63 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).gpower[7]).write(127 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).gpower[8]).write(255 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).gpower[9]).write(511 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).gpower[10]).write(1023 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).gpower[11]).write(2047 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).gpower[12]).write(4095 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).gpower[13]).write(8191 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).gpower[14]).write(16383 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).gpower[15]).write(32767 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).gpower[16]).write(65535 as ::core::ffi::c_int);
        self.initialized[7] = true;
    }
    pub(crate) unsafe fn initialize_fixups(
        &mut self,
        invocation: *const crate::invocation_state::Storage,
    ) {
        let destination = self.destination;
        let _ = invocation;
    }
}
pub(crate) unsafe fn state() -> *mut State {
    core::cell::UnsafeCell::raw_get(core::ptr::addr_of!(
        (*crate::invocation_state::current()).generated_backend_pkin
    ))
}
#[cfg(test)]
pub(crate) unsafe fn check_state_construction(phase: u8) {
    let invocation = crate::invocation_state::current();
    let destination = state();
    assert!(!destination.is_null());
    let _ = (invocation, phase);
}
