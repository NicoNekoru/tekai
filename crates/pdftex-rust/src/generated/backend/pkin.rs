#[repr(C)]
pub struct __sFILEX {
    _unused: [u8; 0],
}

extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
    fn xmalloc(size: size_t) -> address;
    static mut t3_file: *mut FILE;
    fn pdftex_fail(_: *const ::core::ffi::c_char, ...) -> !;
    fn xgetc(_: *mut FILE) -> ::core::ffi::c_int;
}
pub type __int64_t = i64;
pub type __darwin_size_t = usize;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut ::core::ffi::c_uchar,
    pub _size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut ::core::ffi::c_uchar,
    pub _r: ::core::ffi::c_int,
    pub _w: ::core::ffi::c_int,
    pub _flags: ::core::ffi::c_short,
    pub _file: ::core::ffi::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::core::ffi::c_int,
    pub _cookie: *mut ::core::ffi::c_void,
    pub _close: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub _read: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _seek: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, fpos_t, ::core::ffi::c_int) -> fpos_t,
    >,
    pub _write: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::core::ffi::c_int,
    pub _ubuf: [::core::ffi::c_uchar; 3],
    pub _nbuf: [::core::ffi::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: ::core::ffi::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
pub type boolean = ::core::ffi::c_int;
pub type address = *mut ::core::ffi::c_void;
pub type integer = ::core::ffi::c_int;
pub type halfword = integer;
pub type shalfword = ::core::ffi::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chardesc {
    pub charcode: integer,
    pub cwidth: integer,
    pub cheight: integer,
    pub xoff: integer,
    pub yoff: integer,
    pub xescape: integer,
    pub rastersize: integer,
    pub raster: *mut halfword,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
unsafe extern "C" fn pkbyte() -> shalfword {
    let mut i: shalfword = 0;
    i = xgetc(t3_file) as shalfword;
    if i as ::core::ffi::c_int == EOF {
        crate::utils::pdftex_fail_args(
            b"unexpected eof in pk file\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
    }
    return i;
}
unsafe extern "C" fn pkduo() -> integer {
    let mut i: integer = 0;
    i = pkbyte() as integer;
    if i > 127 as ::core::ffi::c_int {
        i -= 256 as ::core::ffi::c_int;
    }
    i = (i as ::core::ffi::c_int * 256 as ::core::ffi::c_int + pkbyte() as ::core::ffi::c_int)
        as integer;
    return i;
}
unsafe extern "C" fn pktrio() -> integer {
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
unsafe extern "C" fn pkquad() -> integer {
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
static mut inputbyte: halfword = 0;
static mut flagbyte: halfword = 0;
static mut bitweight: halfword = 0;
static mut dynf: halfword = 0;
static mut repeatcount: halfword = 0;
unsafe extern "C" fn getnyb() -> shalfword {
    let mut temp: halfword = 0;
    if bitweight == 0 as ::core::ffi::c_int {
        bitweight = 16 as ::core::ffi::c_int as halfword;
        inputbyte = pkbyte() as halfword;
        temp = inputbyte >> 4 as ::core::ffi::c_int;
    } else {
        bitweight = 0 as ::core::ffi::c_int as halfword;
        temp = (inputbyte as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as halfword;
    }
    return temp as shalfword;
}
unsafe extern "C" fn getbit() -> boolean {
    bitweight >>= 1 as ::core::ffi::c_int;
    if bitweight == 0 as ::core::ffi::c_int {
        inputbyte = pkbyte() as halfword;
        bitweight = 128 as ::core::ffi::c_int as halfword;
    }
    return inputbyte as boolean & bitweight as boolean;
}
static mut realfunc: Option<unsafe extern "C" fn() -> halfword> = None;
#[no_mangle]
pub static mut pk_remainder: ::core::ffi::c_long = 0;
unsafe extern "C" fn pkpackednum() -> halfword {
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
            return j - 15 as halfword + (13 as halfword - dynf) * 16 as halfword + dynf;
        }
    } else if i <= dynf {
        return i;
    } else if i < 14 as ::core::ffi::c_int {
        return (i - dynf - 1 as halfword) * 16 as halfword
            + getnyb() as halfword
            + dynf
            + 1 as halfword;
    } else {
        if i == 14 as ::core::ffi::c_int {
            repeatcount = pkpackednum();
        } else {
            repeatcount = 1 as ::core::ffi::c_int as halfword;
        }
        return Some(realfunc.expect("non-null function pointer"))
            .expect("non-null function pointer")();
    };
}
unsafe extern "C" fn rest() -> halfword {
    let mut i: halfword = 0;
    if pk_remainder < 0 as ::core::ffi::c_long {
        pk_remainder = -pk_remainder;
        return 0 as halfword;
    } else if pk_remainder > 0 as ::core::ffi::c_long {
        if pk_remainder > 4000 as ::core::ffi::c_long {
            pk_remainder = 4000 as ::core::ffi::c_long - pk_remainder;
            return 4000 as halfword;
        } else {
            i = pk_remainder as halfword;
            pk_remainder = 0 as ::core::ffi::c_long;
            realfunc = Some(pkpackednum as unsafe extern "C" fn() -> halfword)
                as Option<unsafe extern "C" fn() -> halfword>;
            return i;
        }
    } else {
        crate::utils::pdftex_fail_args(
            b"shouldn't happen\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
    };
}
unsafe extern "C" fn handlehuge(mut i: halfword, mut k: halfword) -> halfword {
    let mut j: ::core::ffi::c_long = k as ::core::ffi::c_long;
    while i != 0 {
        j = (j << 4 as ::core::ffi::c_long) + getnyb() as ::core::ffi::c_long;
        i -= 1;
    }
    pk_remainder = j - 15 as ::core::ffi::c_long
        + ((13 as ::core::ffi::c_int - dynf as ::core::ffi::c_int) * 16 as ::core::ffi::c_int)
            as ::core::ffi::c_long
        + dynf as ::core::ffi::c_long;
    realfunc = Some(rest as unsafe extern "C" fn() -> halfword)
        as Option<unsafe extern "C" fn() -> halfword>;
    return rest();
}
static mut gpower: [halfword; 17] = [
    0 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    15 as ::core::ffi::c_int,
    31 as ::core::ffi::c_int,
    63 as ::core::ffi::c_int,
    127 as ::core::ffi::c_int,
    255 as ::core::ffi::c_int,
    511 as ::core::ffi::c_int,
    1023 as ::core::ffi::c_int,
    2047 as ::core::ffi::c_int,
    4095 as ::core::ffi::c_int,
    8191 as ::core::ffi::c_int,
    16383 as ::core::ffi::c_int,
    32767 as ::core::ffi::c_int,
    65535 as ::core::ffi::c_int,
];
unsafe extern "C" fn unpack(mut cd: *mut chardesc) {
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
    realfunc = Some(pkpackednum as unsafe extern "C" fn() -> halfword)
        as Option<unsafe extern "C" fn() -> halfword>;
    dynf = (flagbyte as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as halfword;
    turnon = (flagbyte as ::core::ffi::c_int & 8 as ::core::ffi::c_int) as boolean;
    if dynf == 14 as ::core::ffi::c_int {
        bitweight = 0 as ::core::ffi::c_int as halfword;
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
        repeatcount = 0 as ::core::ffi::c_int as halfword;
        wordweight = 16 as ::core::ffi::c_int as halfword;
        word = 0 as ::core::ffi::c_int as halfword;
        bitweight = 0 as ::core::ffi::c_int as halfword;
        while rowsleft as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            count = Some(realfunc.expect("non-null function pointer"))
                .expect("non-null function pointer")();
            while count != 0 as ::core::ffi::c_int {
                if count < wordweight && count < hbit as ::core::ffi::c_int {
                    if turnon != 0 {
                        word += gpower[wordweight as usize] - gpower[(wordweight - count) as usize];
                    }
                    hbit = (hbit as ::core::ffi::c_int - count as ::core::ffi::c_int) as shalfword;
                    wordweight -= count;
                    count = 0 as ::core::ffi::c_int as halfword;
                } else if count >= hbit as ::core::ffi::c_int
                    && hbit as ::core::ffi::c_int <= wordweight
                {
                    if turnon != 0 {
                        word += gpower[wordweight as usize]
                            - gpower[(wordweight as ::core::ffi::c_int - hbit as ::core::ffi::c_int)
                                as usize];
                    }
                    let fresh3 = raster;
                    raster = raster.offset(1);
                    *fresh3 = word;
                    i = 1 as ::core::ffi::c_int as integer;
                    while i <= repeatcount {
                        j = 1 as ::core::ffi::c_int as integer;
                        while j <= wordwidth as ::core::ffi::c_int {
                            *raster = *raster.offset(-(wordwidth as ::core::ffi::c_int as isize));
                            raster = raster.offset(1);
                            j += 1;
                        }
                        i += 1;
                    }
                    rowsleft = (rowsleft as ::core::ffi::c_int
                        - (repeatcount as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                        as shalfword;
                    repeatcount = 0 as ::core::ffi::c_int as halfword;
                    word = 0 as ::core::ffi::c_int as halfword;
                    wordweight = 16 as ::core::ffi::c_int as halfword;
                    count -= hbit as ::core::ffi::c_int;
                    hbit = (*cd).cwidth as shalfword;
                } else {
                    if turnon != 0 {
                        word += gpower[wordweight as usize];
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
#[no_mangle]
pub unsafe extern "C" fn readchar(
    mut check_preamble: boolean,
    mut cd: *mut chardesc,
) -> ::core::ffi::c_int {
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
        flagbyte = pkbyte() as halfword;
        if !(flagbyte != 245 as ::core::ffi::c_int) {
            break;
        }
        if flagbyte < 240 as ::core::ffi::c_int {
            match flagbyte as ::core::ffi::c_int & 7 as ::core::ffi::c_int {
                0 | 1 | 2 | 3 => {
                    length = ((flagbyte as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
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
                    length = ((flagbyte as ::core::ffi::c_int & 3 as ::core::ffi::c_int)
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
            match flagbyte {
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
                        &[crate::utils::PrintfArg::from(flagbyte)],
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
