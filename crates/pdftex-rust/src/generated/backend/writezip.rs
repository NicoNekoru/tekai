#[repr(C)]
pub struct __sFILEX {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct internal_state {
    _unused: [u8; 0],
}

extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn xmalloc(size: size_t) -> address;
    fn deflate(strm: z_streamp, flush: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn deflateEnd(strm: z_streamp) -> ::core::ffi::c_int;
    fn deflateReset(strm: z_streamp) -> ::core::ffi::c_int;
    fn deflateInit_(
        strm: z_streamp,
        level: ::core::ffi::c_int,
        version: *const ::core::ffi::c_char,
        stream_size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut pdffile: bytefile;
    static mut pdfbuf: *mut eightbits;
    static mut pdfptr: integer;
    static mut pdfgone: longinteger;
    static mut pdfstreamlength: longinteger;
    static mut pdflastbyte: eightbits;
    fn getpdfcompresslevel() -> integer;
    static mut cur_file_name: *mut ::core::ffi::c_char;
    fn pdftex_fail(_: *const ::core::ffi::c_char, ...) -> !;
    fn xfflush(_: *mut FILE) -> ::core::ffi::c_int;
    fn xfwrite(_: *mut ::core::ffi::c_void, size: size_t, nmemb: size_t, _: *mut FILE) -> size_t;
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
pub type off_t = __darwin_off_t;
pub type boolean = ::core::ffi::c_int;
pub type address = *mut ::core::ffi::c_void;
pub type integer = ::core::ffi::c_int;
pub type longinteger = off_t;
pub type text = *mut FILE;
pub type Byte = ::core::ffi::c_uchar;
pub type uInt = ::core::ffi::c_uint;
pub type uLong = ::core::ffi::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut ::core::ffi::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *const Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *const ::core::ffi::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: ::core::ffi::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
pub type eightbits = ::core::ffi::c_uchar;
pub type bytefile = text;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const Z_NO_FLUSH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Z_FINISH: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const Z_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Z_STREAM_END: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ZIP_BUF_SIZE: ::core::ffi::c_int = 32768 as ::core::ffi::c_int;
const TEXPILOT_MAX_FAST_COMPRESS_LEVEL: ::core::ffi::c_int = 1;
static mut zipbuf: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut c_stream: z_stream = z_stream_s {
    next_in: ::core::ptr::null::<Bytef>(),
    avail_in: 0,
    total_in: 0,
    next_out: ::core::ptr::null::<Bytef>() as *mut Bytef,
    avail_out: 0,
    total_out: 0,
    msg: ::core::ptr::null::<::core::ffi::c_char>(),
    state: ::core::ptr::null::<internal_state>() as *mut internal_state,
    zalloc: None,
    zfree: None,
    opaque: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    data_type: 0,
    adler: 0,
    reserved: 0,
};
#[no_mangle]
pub unsafe extern "C" fn writezip(mut finish: boolean) {
    let mut err: ::core::ffi::c_int = 0;
    static mut level_old: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut level: ::core::ffi::c_int = getpdfcompresslevel() as ::core::ffi::c_int;
    if level > TEXPILOT_MAX_FAST_COMPRESS_LEVEL {
        level = TEXPILOT_MAX_FAST_COMPRESS_LEVEL;
    }
    if !(level > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"writezip\0" as *const u8 as *const ::core::ffi::c_char,
            b"writezip.c\0" as *const u8 as *const ::core::ffi::c_char,
            38 as ::core::ffi::c_int,
            b"level > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    cur_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if pdfstreamlength == 0 as longinteger {
        if zipbuf.is_null() {
            zipbuf = xmalloc(
                (32768 as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            ) as *mut ::core::ffi::c_char;
            c_stream.zalloc = None;
            c_stream.zfree = None;
            c_stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if deflateInit_(
                &raw mut c_stream,
                level,
                b"1.3.2\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::mem::size_of::<z_stream>() as ::core::ffi::c_int,
            ) != Z_OK
            {
                crate::utils::pdftex_fail_args(
                    b"zlib: %s() failed (error code %d)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(
                            b"deflateInit\0" as *const u8 as *const ::core::ffi::c_char,
                        ),
                        crate::utils::PrintfArg::from(deflateInit_(
                            &raw mut c_stream,
                            level,
                            b"1.3.2\0" as *const u8 as *const ::core::ffi::c_char,
                            ::core::mem::size_of::<z_stream>() as ::core::ffi::c_int,
                        )),
                    ],
                );
            }
        } else if level != level_old {
            if deflateEnd(&raw mut c_stream) != Z_OK {
                crate::utils::pdftex_fail_args(
                    b"zlib: %s() failed (error code %d)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(
                            b"deflateEnd\0" as *const u8 as *const ::core::ffi::c_char,
                        ),
                        crate::utils::PrintfArg::from(deflateEnd(&raw mut c_stream)),
                    ],
                );
            }
            c_stream.zalloc = None;
            c_stream.zfree = None;
            c_stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if deflateInit_(
                &raw mut c_stream,
                level,
                b"1.3.2\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::mem::size_of::<z_stream>() as ::core::ffi::c_int,
            ) != Z_OK
            {
                crate::utils::pdftex_fail_args(
                    b"zlib: %s() failed (error code %d)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(
                            b"deflateInit\0" as *const u8 as *const ::core::ffi::c_char,
                        ),
                        crate::utils::PrintfArg::from(deflateInit_(
                            &raw mut c_stream,
                            level,
                            b"1.3.2\0" as *const u8 as *const ::core::ffi::c_char,
                            ::core::mem::size_of::<z_stream>() as ::core::ffi::c_int,
                        )),
                    ],
                );
            }
        } else if deflateReset(&raw mut c_stream) != Z_OK {
            crate::utils::pdftex_fail_args(
                b"zlib: %s() failed (error code %d)\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(
                        b"deflateReset\0" as *const u8 as *const ::core::ffi::c_char,
                    ),
                    crate::utils::PrintfArg::from(deflateReset(&raw mut c_stream)),
                ],
            );
        }
        level_old = level;
        c_stream.next_out = zipbuf as *mut Bytef;
        c_stream.avail_out = ZIP_BUF_SIZE as uInt;
    }
    if zipbuf.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"writezip\0" as *const u8 as *const ::core::ffi::c_char,
            b"writezip.c\0" as *const u8 as *const ::core::ffi::c_char,
            61 as ::core::ffi::c_int,
            b"zipbuf != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    c_stream.next_in = pdfbuf;
    c_stream.avail_in = pdfptr as uInt;
    loop {
        if c_stream.avail_out == 0 as uInt {
            pdfgone = (pdfgone as ::core::ffi::c_ulonglong).wrapping_add(xfwrite(
                zipbuf as *mut ::core::ffi::c_void,
                1 as size_t,
                ZIP_BUF_SIZE as size_t,
                pdffile as *mut FILE,
            )
                as ::core::ffi::c_ulonglong) as longinteger as longinteger;
            pdflastbyte =
                *zipbuf.offset((ZIP_BUF_SIZE - 1 as ::core::ffi::c_int) as isize) as eightbits;
            c_stream.next_out = zipbuf as *mut Bytef;
            c_stream.avail_out = ZIP_BUF_SIZE as uInt;
        }
        err = deflate(
            &raw mut c_stream,
            if finish != 0 { Z_FINISH } else { Z_NO_FLUSH },
        );
        if finish != 0 && err == Z_STREAM_END {
            break;
        }
        if err != Z_OK {
            crate::utils::pdftex_fail_args(
                b"zlib: %s() failed (error code %d)\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(
                        b"deflate\0" as *const u8 as *const ::core::ffi::c_char,
                    ),
                    crate::utils::PrintfArg::from(err),
                ],
            );
        }
        if finish == 0 && c_stream.avail_in == 0 as uInt {
            break;
        }
    }
    if finish != 0 {
        if c_stream.avail_out < ZIP_BUF_SIZE as uInt {
            pdfgone = (pdfgone as ::core::ffi::c_ulonglong).wrapping_add(xfwrite(
                zipbuf as *mut ::core::ffi::c_void,
                1 as size_t,
                (ZIP_BUF_SIZE as uInt).wrapping_sub(c_stream.avail_out) as size_t,
                pdffile as *mut FILE,
            )
                as ::core::ffi::c_ulonglong) as longinteger as longinteger;
            pdflastbyte = *zipbuf.offset(
                (ZIP_BUF_SIZE as uInt)
                    .wrapping_sub(c_stream.avail_out)
                    .wrapping_sub(1 as uInt) as isize,
            ) as eightbits;
        }
        xfflush(pdffile as *mut FILE);
    }
    pdfstreamlength = c_stream.total_out as longinteger;
}
#[no_mangle]
pub unsafe extern "C" fn zip_free() {
    if !zipbuf.is_null() {
        if deflateEnd(&raw mut c_stream) != Z_OK {
            crate::utils::pdftex_fail_args(
                b"zlib: %s() failed (error code %d)\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(
                        b"deflateEnd\0" as *const u8 as *const ::core::ffi::c_char,
                    ),
                    crate::utils::PrintfArg::from(deflateEnd(&raw mut c_stream)),
                ],
            );
        }
        free(zipbuf as *mut ::core::ffi::c_void);
    }
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
