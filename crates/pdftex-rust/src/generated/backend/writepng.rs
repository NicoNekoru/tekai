#[repr(C)]
pub struct __sFILEX {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct png_struct_def {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct png_info_def {
    _unused: [u8; 0],
}

extern "C" {
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __nitems: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fseek(_: *mut FILE, _: ::core::ffi::c_long, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn free(_: *mut ::core::ffi::c_void);
    fn getenv(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn xfopen(filename: const_string, mode: const_string) -> *mut FILE;
    fn xmalloc(size: size_t) -> address;
    fn zround(_: ::core::ffi::c_double) -> integer;
    static mut pdfbuf: *mut eightbits;
    static mut pdfbufsize: integer;
    static mut pdfptr: integer;
    static mut pdfosmode: boolean;
    static mut fixedpdfmajorversion: integer;
    static mut fixedpdfminorversion: integer;
    static mut fixedgamma: integer;
    static mut fixedimagegamma: integer;
    static mut fixedimagehicolor: boolean;
    static mut fixedimageapplygamma: integer;
    static mut pdfpagegroupval: integer;
    static mut objptr: integer;
    fn zpdfosgetosbuf(s: integer);
    fn zpdfcreateobj(t: integer, i: integer);
    fn pdfnewobjnum() -> integer;
    fn zpdfbeginobj(i: integer, pdfoslevel: integer);
    fn pdfendobj();
    fn zpdfbegindict(i: integer, pdfoslevel: integer);
    fn getpdfcompresslevel() -> integer;
    fn pdfflush();
    fn pdfbeginstream();
    fn pdfendstream();
    fn pdf_printf(_: *const ::core::ffi::c_char, ...);
    fn pdf_puts(_: *const ::core::ffi::c_char);
    fn pdftex_fail(_: *const ::core::ffi::c_char, ...) -> !;
    fn pdftex_warn(_: *const ::core::ffi::c_char, ...);
    fn tex_printf(_: *const ::core::ffi::c_char, ...);
    fn setjmp(_: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn longjmp(_: *mut ::core::ffi::c_int, _: ::core::ffi::c_int) -> !;
    fn png_create_read_struct(
        user_png_ver: png_const_charp,
        error_ptr: png_voidp,
        error_fn: png_error_ptr,
        warn_fn: png_error_ptr,
    ) -> png_structp;
    fn png_set_longjmp_fn(
        png_ptr: png_structrp,
        longjmp_fn: png_longjmp_ptr,
        jmp_buf_size: size_t,
    ) -> *mut jmp_buf;
    fn png_create_info_struct(png_ptr: png_const_structrp) -> png_infop;
    fn png_read_info(png_ptr: png_structrp, info_ptr: png_inforp);
    fn png_set_tRNS_to_alpha(png_ptr: png_structrp);
    fn png_set_strip_alpha(png_ptr: png_structrp);
    fn png_set_interlace_handling(png_ptr: png_structrp) -> ::core::ffi::c_int;
    fn png_set_strip_16(png_ptr: png_structrp);
    fn png_set_gamma(
        png_ptr: png_structrp,
        screen_gamma: ::core::ffi::c_double,
        override_file_gamma: ::core::ffi::c_double,
    );
    fn png_read_update_info(png_ptr: png_structrp, info_ptr: png_inforp);
    fn png_read_row(png_ptr: png_structrp, row: png_bytep, display_row: png_bytep);
    fn png_read_image(png_ptr: png_structrp, image: png_bytepp);
    fn png_init_io(png_ptr: png_structrp, fp: *mut FILE);
    fn png_get_io_ptr(png_ptr: png_const_structrp) -> png_voidp;
    fn png_get_valid(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        flag: png_uint_32,
    ) -> png_uint_32;
    fn png_get_rowbytes(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> size_t;
    fn png_get_image_width(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> png_uint_32;
    fn png_get_image_height(png_ptr: png_const_structrp, info_ptr: png_const_inforp)
        -> png_uint_32;
    fn png_get_bit_depth(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> png_byte;
    fn png_get_color_type(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> png_byte;
    fn png_get_interlace_type(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> png_byte;
    fn png_get_x_pixels_per_meter(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
    ) -> png_uint_32;
    fn png_get_y_pixels_per_meter(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
    ) -> png_uint_32;
    fn png_get_gAMA(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        file_gamma: *mut ::core::ffi::c_double,
    ) -> png_uint_32;
    fn png_get_gAMA_fixed(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        int_file_gamma: *mut png_fixed_point,
    ) -> png_uint_32;
    fn png_get_PLTE(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        palette: *mut png_colorp,
        num_palette: *mut ::core::ffi::c_int,
    ) -> png_uint_32;
    fn png_set_option(
        png_ptr: png_structrp,
        option: ::core::ffi::c_int,
        onoff: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut image_array: *mut image_entry;
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
pub type const_string = *const ::core::ffi::c_char;
pub type address = *mut ::core::ffi::c_void;
pub type integer = ::core::ffi::c_int;
pub type eightbits = ::core::ffi::c_uchar;
pub type jmp_buf = [::core::ffi::c_int; 48];
pub type png_byte = ::core::ffi::c_uchar;
pub type png_int_32 = ::core::ffi::c_int;
pub type png_uint_32 = ::core::ffi::c_uint;
pub type png_fixed_point = png_int_32;
pub type png_voidp = *mut ::core::ffi::c_void;
pub type png_bytep = *mut png_byte;
pub type png_const_charp = *const ::core::ffi::c_char;
pub type png_bytepp = *mut *mut png_byte;
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_structrp = *mut png_struct;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
pub type png_const_inforp = *const png_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
}
pub type png_color = png_color_struct;
pub type png_colorp = *mut png_color;
pub type png_error_ptr = Option<unsafe extern "C" fn(png_structp, png_const_charp) -> ()>;
pub type png_longjmp_ptr =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_int, ::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JPG_IMAGE_INFO {
    pub color_space: ::core::ffi::c_int,
    pub bits_per_component: ::core::ffi::c_uchar,
    pub length: ::core::ffi::c_ulong,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_image_struct {
    pub png_ptr: png_structp,
    pub info_ptr: png_infop,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_image_struct {
    pub orig_x: integer,
    pub orig_y: integer,
    pub selected_page: integer,
    pub page_box: integer,
    pub doc: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JBIG2_IMAGE_INFO {
    pub selected_page: integer,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct image_entry {
    pub image_name: *mut ::core::ffi::c_char,
    pub image_type: ::core::ffi::c_int,
    pub color_type: ::core::ffi::c_int,
    pub width: integer,
    pub height: integer,
    pub rotate: integer,
    pub x_res: integer,
    pub y_res: integer,
    pub num_pages: integer,
    pub colorspace_ref: integer,
    pub group_ref: integer,
    pub image_struct: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub pdf: *mut pdf_image_struct,
    pub png: png_image_struct,
    pub jpg: *mut JPG_IMAGE_INFO,
    pub jbig2: *mut JBIG2_IMAGE_INFO,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FOPEN_RBIN_MODE: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"rb\0") };
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PNG_LIBPNG_VER_STRING: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"1.6.58\0") };
pub const PNG_FP_1: ::core::ffi::c_int = 100000 as ::core::ffi::c_int;
pub const PNG_COLOR_MASK_PALETTE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PNG_COLOR_MASK_COLOR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PNG_COLOR_MASK_ALPHA: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PNG_COLOR_TYPE_GRAY: ::core::ffi::c_int = 0;
pub const PNG_COLOR_TYPE_PALETTE: ::core::ffi::c_int = 3;
pub const PNG_COLOR_TYPE_RGB: ::core::ffi::c_int = 2;
pub const PNG_COLOR_TYPE_RGB_ALPHA: ::core::ffi::c_int =
    PNG_COLOR_MASK_COLOR | PNG_COLOR_MASK_ALPHA;
pub const PNG_COLOR_TYPE_GRAY_ALPHA: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PNG_INTERLACE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PNG_INFO_gAMA: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const PNG_INFO_sBIT: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const PNG_INFO_cHRM: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const PNG_INFO_tRNS: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const PNG_INFO_bKGD: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const PNG_INFO_hIST: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const PNG_INFO_pHYs: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const PNG_INFO_sRGB: ::core::ffi::c_uint = 0x800 as ::core::ffi::c_uint;
pub const PNG_INFO_iCCP: ::core::ffi::c_uint = 0x1000 as ::core::ffi::c_uint;
pub const PNG_INFO_sPLT: ::core::ffi::c_uint = 0x2000 as ::core::ffi::c_uint;
pub const PNG_MAXIMUM_INFLATE_WINDOW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PNG_OPTION_ON: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const IMAGE_COLOR_B: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IMAGE_COLOR_C: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const IMAGE_COLOR_I: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
static mut transparent_page_group: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn warn(mut png_ptr: png_structp, mut msg: png_const_charp) {}
#[no_mangle]
pub unsafe extern "C" fn read_png_info(mut img: integer) {
    let mut png_file: *mut FILE = xfopen(
        (*image_array.offset(img as isize)).image_name as const_string,
        FOPEN_RBIN_MODE.as_ptr(),
    );
    let ref mut fresh0 = (*image_array.offset(img as isize)).image_struct.png.png_ptr;
    *fresh0 = png_create_read_struct(
        PNG_LIBPNG_VER_STRING.as_ptr(),
        NULL,
        None,
        Some(warn as unsafe extern "C" fn(png_structp, png_const_charp) -> ()),
    );
    if (*fresh0).is_null() {
        crate::utils::pdftex_fail_args(
            b"libpng: png_create_read_struct() failed\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
    }
    let ref mut fresh1 = (*image_array.offset(img as isize))
        .image_struct
        .png
        .info_ptr;
    *fresh1 = png_create_info_struct(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
    );
    if (*fresh1).is_null() {
        crate::utils::pdftex_fail_args(
            b"libpng: png_create_info_struct() failed\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
    }
    if setjmp(&raw mut *(png_set_longjmp_fn
        as unsafe extern "C" fn(png_structrp, png_longjmp_ptr, size_t) -> *mut jmp_buf)(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_int, ::core::ffi::c_int) -> !>,
            png_longjmp_ptr,
        >(Some(
            longjmp as unsafe extern "C" fn(*mut ::core::ffi::c_int, ::core::ffi::c_int) -> !,
        )),
        ::core::mem::size_of::<jmp_buf>() as size_t,
    ) as *mut ::core::ffi::c_int)
        != 0
    {
        crate::utils::pdftex_fail_args(
            b"libpng: internal error\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
    }
    png_set_option(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
        PNG_MAXIMUM_INFLATE_WINDOW,
        PNG_OPTION_ON,
    );
    png_init_io(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
        png_file,
    );
    png_read_info(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_inforp,
    );
    (*image_array.offset(img as isize)).width = png_get_image_width(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
    ) as integer;
    (*image_array.offset(img as isize)).height = png_get_image_height(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
    ) as integer;
    if png_get_valid(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        PNG_INFO_pHYs,
    ) != 0
    {
        (*image_array.offset(img as isize)).x_res = zround(
            0.0254f64
                * png_get_x_pixels_per_meter(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_double,
        );
        (*image_array.offset(img as isize)).y_res = zround(
            0.0254f64
                * png_get_y_pixels_per_meter(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_double,
        );
    }
    match png_get_color_type(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
    ) as ::core::ffi::c_int
    {
        PNG_COLOR_TYPE_PALETTE => {
            (*image_array.offset(img as isize)).color_type = IMAGE_COLOR_C | IMAGE_COLOR_I;
        }
        PNG_COLOR_TYPE_GRAY | PNG_COLOR_TYPE_GRAY_ALPHA => {
            (*image_array.offset(img as isize)).color_type = IMAGE_COLOR_B;
        }
        PNG_COLOR_TYPE_RGB | PNG_COLOR_TYPE_RGB_ALPHA => {
            (*image_array.offset(img as isize)).color_type = IMAGE_COLOR_C;
        }
        _ => {
            crate::utils::pdftex_fail_args(
                b"unsupported type of color_type <%i>\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(png_get_color_type(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                )
                    as ::core::ffi::c_int)],
            );
        }
    }
    if (fixedpdfmajorversion > 1 as ::core::ffi::c_int
        || fixedpdfminorversion >= 4 as ::core::ffi::c_int)
        && (png_get_color_type(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
            == PNG_COLOR_TYPE_GRAY_ALPHA
            || png_get_color_type(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
                == PNG_COLOR_TYPE_RGB_ALPHA)
    {
        if transparent_page_group == 0 as ::core::ffi::c_int {
            transparent_page_group = pdfnewobjnum() as ::core::ffi::c_int;
        }
        if pdfpagegroupval == 0 as ::core::ffi::c_int {
            pdfpagegroupval = transparent_page_group as integer;
        }
        (*image_array.offset(img as isize)).group_ref = pdfpagegroupval;
    }
}
unsafe extern "C" fn write_png_palette(mut img: integer) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut row: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut r: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut rows: *mut png_bytep = ::core::ptr::null_mut::<png_bytep>();
    let mut palette_objnum: integer = 0 as integer;
    let mut palette: png_colorp = ::core::ptr::null_mut::<png_color>();
    let mut num_palette: ::core::ffi::c_int = 0;
    png_get_PLTE(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_inforp,
        &raw mut palette,
        &raw mut num_palette,
    );
    zpdfcreateobj(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    palette_objnum = objptr;
    if (*image_array.offset(img as isize)).colorspace_ref != 0 as ::core::ffi::c_int {
        crate::utils::pdf_printf_args(
            b"%i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                (*image_array.offset(img as isize)).colorspace_ref,
            )],
        );
    } else {
        crate::utils::pdf_printf_args(
            b"[/Indexed /DeviceRGB %i %i 0 R]\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(num_palette - 1 as ::core::ffi::c_int),
                crate::utils::PrintfArg::from(palette_objnum),
            ],
        );
    }
    pdfbeginstream();
    if png_get_interlace_type(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
    ) as ::core::ffi::c_int
        == PNG_INTERLACE_NONE
    {
        row = xmalloc(
            png_get_rowbytes(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            )
            .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
        ) as *mut png_byte as png_bytep;
        i = 0 as ::core::ffi::c_int;
        while i < png_get_image_height(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
        {
            png_read_row(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
                row,
                ::core::ptr::null_mut::<png_byte>(),
            );
            r = row;
            k = png_get_rowbytes(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int;
            while k > 0 as ::core::ffi::c_int {
                l = if k > pdfbufsize {
                    pdfbufsize as ::core::ffi::c_int
                } else {
                    k
                };
                if (l as integer + pdfptr) as ::core::ffi::c_uint
                    > pdfbufsize as ::core::ffi::c_uint
                {
                    if pdfosmode != 0 {
                        zpdfosgetosbuf(l);
                    } else if l as ::core::ffi::c_uint > pdfbufsize as ::core::ffi::c_uint {
                        crate::utils::pdftex_fail_args(
                            b"PDF output buffer overflowed\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    } else {
                        pdfflush();
                    }
                }
                j = 0 as ::core::ffi::c_int;
                while j < l {
                    let fresh57 = r;
                    r = r.offset(1);
                    let fresh58 = pdfptr;
                    pdfptr = pdfptr + 1;
                    *pdfbuf.offset(fresh58 as isize) = *fresh57 as eightbits;
                    j += 1;
                }
                k -= l;
            }
            i += 1;
        }
        if !row.is_null() {
            free(row as *mut ::core::ffi::c_void);
        }
        row = ::core::ptr::null_mut::<png_byte>();
    } else {
        if (png_get_image_height(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as size_t)
            .wrapping_mul(png_get_rowbytes(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ))
            >= 10240000 as ::core::ffi::c_long as size_t
        {
            crate::utils::pdftex_warn_args(b"large interlaced PNG might cause out of memory (use non-interlaced PNG to fix this)\0"
                    as *const u8 as *const ::core::ffi::c_char, &[]);
        }
        rows = xmalloc(
            (png_get_image_height(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as size_t)
                .wrapping_mul(::core::mem::size_of::<png_bytep>() as size_t),
        ) as *mut png_bytep;
        i = 0 as ::core::ffi::c_int;
        while (i as ::core::ffi::c_uint)
            < png_get_image_height(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            )
        {
            let ref mut fresh59 = *rows.offset(i as isize);
            *fresh59 = xmalloc(
                png_get_rowbytes(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                )
                .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
            ) as *mut png_byte as png_bytep;
            i += 1;
        }
        png_read_image(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
            rows as png_bytepp,
        );
        i = 0 as ::core::ffi::c_int;
        while i < png_get_image_height(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
        {
            row = *rows.offset(i as isize);
            k = png_get_rowbytes(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int;
            while k > 0 as ::core::ffi::c_int {
                l = if k > pdfbufsize {
                    pdfbufsize as ::core::ffi::c_int
                } else {
                    k
                };
                if (l as integer + pdfptr) as ::core::ffi::c_uint
                    > pdfbufsize as ::core::ffi::c_uint
                {
                    if pdfosmode != 0 {
                        zpdfosgetosbuf(l);
                    } else if l as ::core::ffi::c_uint > pdfbufsize as ::core::ffi::c_uint {
                        crate::utils::pdftex_fail_args(
                            b"PDF output buffer overflowed\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    } else {
                        pdfflush();
                    }
                }
                j = 0 as ::core::ffi::c_int;
                while j < l {
                    let fresh60 = row;
                    row = row.offset(1);
                    let fresh61 = pdfptr;
                    pdfptr = pdfptr + 1;
                    *pdfbuf.offset(fresh61 as isize) = *fresh60 as eightbits;
                    j += 1;
                }
                k -= l;
            }
            if !(*rows.offset(i as isize)).is_null() {
                free(*rows.offset(i as isize) as *mut ::core::ffi::c_void);
            }
            let ref mut fresh62 = *rows.offset(i as isize);
            *fresh62 = ::core::ptr::null_mut::<png_byte>();
            i += 1;
        }
        if !rows.is_null() {
            free(rows as *mut ::core::ffi::c_void);
        }
        rows = ::core::ptr::null_mut::<png_bytep>();
    }
    pdfendstream();
    if palette_objnum > 0 as ::core::ffi::c_int {
        zpdfbegindict(palette_objnum, 0 as ::core::ffi::c_int);
        pdfbeginstream();
        i = 0 as ::core::ffi::c_int;
        while (i as ::core::ffi::c_uint) < num_palette as ::core::ffi::c_uint {
            if (3 as integer + pdfptr) as ::core::ffi::c_uint > pdfbufsize as ::core::ffi::c_uint {
                if pdfosmode != 0 {
                    zpdfosgetosbuf(3 as ::core::ffi::c_int);
                } else if 3 as ::core::ffi::c_int as ::core::ffi::c_uint
                    > pdfbufsize as ::core::ffi::c_uint
                {
                    crate::utils::pdftex_fail_args(
                        b"PDF output buffer overflowed\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                } else {
                    pdfflush();
                }
            }
            let fresh63 = pdfptr;
            pdfptr = pdfptr + 1;
            *pdfbuf.offset(fresh63 as isize) = (*palette.offset(i as isize)).red as eightbits;
            let fresh64 = pdfptr;
            pdfptr = pdfptr + 1;
            *pdfbuf.offset(fresh64 as isize) = (*palette.offset(i as isize)).green as eightbits;
            let fresh65 = pdfptr;
            pdfptr = pdfptr + 1;
            *pdfbuf.offset(fresh65 as isize) = (*palette.offset(i as isize)).blue as eightbits;
            i += 1;
        }
        pdfendstream();
    }
}
unsafe extern "C" fn write_png_gray(mut img: integer) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut row: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut r: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut rows: *mut png_bytep = ::core::ptr::null_mut::<png_bytep>();
    if (*image_array.offset(img as isize)).colorspace_ref != 0 as ::core::ffi::c_int {
        crate::utils::pdf_printf_args(
            b"%i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                (*image_array.offset(img as isize)).colorspace_ref,
            )],
        );
    } else {
        pdf_puts(b"/DeviceGray\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    pdfbeginstream();
    if png_get_interlace_type(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
    ) as ::core::ffi::c_int
        == PNG_INTERLACE_NONE
    {
        row = xmalloc(
            png_get_rowbytes(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            )
            .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
        ) as *mut png_byte as png_bytep;
        i = 0 as ::core::ffi::c_int;
        while i < png_get_image_height(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
        {
            png_read_row(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
                row,
                ::core::ptr::null_mut::<png_byte>(),
            );
            r = row;
            k = png_get_rowbytes(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int;
            while k > 0 as ::core::ffi::c_int {
                l = if k > pdfbufsize {
                    pdfbufsize as ::core::ffi::c_int
                } else {
                    k
                };
                if (l as integer + pdfptr) as ::core::ffi::c_uint
                    > pdfbufsize as ::core::ffi::c_uint
                {
                    if pdfosmode != 0 {
                        zpdfosgetosbuf(l);
                    } else if l as ::core::ffi::c_uint > pdfbufsize as ::core::ffi::c_uint {
                        crate::utils::pdftex_fail_args(
                            b"PDF output buffer overflowed\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    } else {
                        pdfflush();
                    }
                }
                j = 0 as ::core::ffi::c_int;
                while j < l {
                    let fresh31 = r;
                    r = r.offset(1);
                    let fresh32 = pdfptr;
                    pdfptr = pdfptr + 1;
                    *pdfbuf.offset(fresh32 as isize) = *fresh31 as eightbits;
                    j += 1;
                }
                k -= l;
            }
            i += 1;
        }
        if !row.is_null() {
            free(row as *mut ::core::ffi::c_void);
        }
        row = ::core::ptr::null_mut::<png_byte>();
    } else {
        if (png_get_image_height(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as size_t)
            .wrapping_mul(png_get_rowbytes(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ))
            >= 10240000 as ::core::ffi::c_long as size_t
        {
            crate::utils::pdftex_warn_args(b"large interlaced PNG might cause out of memory (use non-interlaced PNG to fix this)\0"
                    as *const u8 as *const ::core::ffi::c_char, &[]);
        }
        rows = xmalloc(
            (png_get_image_height(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as size_t)
                .wrapping_mul(::core::mem::size_of::<png_bytep>() as size_t),
        ) as *mut png_bytep;
        i = 0 as ::core::ffi::c_int;
        while (i as ::core::ffi::c_uint)
            < png_get_image_height(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            )
        {
            let ref mut fresh33 = *rows.offset(i as isize);
            *fresh33 = xmalloc(
                png_get_rowbytes(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                )
                .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
            ) as *mut png_byte as png_bytep;
            i += 1;
        }
        png_read_image(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
            rows as png_bytepp,
        );
        i = 0 as ::core::ffi::c_int;
        while i < png_get_image_height(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
        {
            row = *rows.offset(i as isize);
            k = png_get_rowbytes(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int;
            while k > 0 as ::core::ffi::c_int {
                l = if k > pdfbufsize {
                    pdfbufsize as ::core::ffi::c_int
                } else {
                    k
                };
                if (l as integer + pdfptr) as ::core::ffi::c_uint
                    > pdfbufsize as ::core::ffi::c_uint
                {
                    if pdfosmode != 0 {
                        zpdfosgetosbuf(l);
                    } else if l as ::core::ffi::c_uint > pdfbufsize as ::core::ffi::c_uint {
                        crate::utils::pdftex_fail_args(
                            b"PDF output buffer overflowed\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    } else {
                        pdfflush();
                    }
                }
                j = 0 as ::core::ffi::c_int;
                while j < l {
                    let fresh34 = row;
                    row = row.offset(1);
                    let fresh35 = pdfptr;
                    pdfptr = pdfptr + 1;
                    *pdfbuf.offset(fresh35 as isize) = *fresh34 as eightbits;
                    j += 1;
                }
                k -= l;
            }
            if !(*rows.offset(i as isize)).is_null() {
                free(*rows.offset(i as isize) as *mut ::core::ffi::c_void);
            }
            let ref mut fresh36 = *rows.offset(i as isize);
            *fresh36 = ::core::ptr::null_mut::<png_byte>();
            i += 1;
        }
        if !rows.is_null() {
            free(rows as *mut ::core::ffi::c_void);
        }
        rows = ::core::ptr::null_mut::<png_bytep>();
    }
    pdfendstream();
}
unsafe extern "C" fn write_png_gray_alpha(mut img: integer) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut row: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut r: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut rows: *mut png_bytep = ::core::ptr::null_mut::<png_bytep>();
    let mut smask_objnum: integer = 0 as integer;
    let mut smask: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut smask_ptr: integer = 0 as integer;
    let mut smask_size: integer = 0 as integer;
    let mut bitdepth: ::core::ffi::c_int = 0;
    if (*image_array.offset(img as isize)).colorspace_ref != 0 as ::core::ffi::c_int {
        crate::utils::pdf_printf_args(
            b"%i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                (*image_array.offset(img as isize)).colorspace_ref,
            )],
        );
    } else {
        pdf_puts(b"/DeviceGray\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    zpdfcreateobj(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    smask_objnum = objptr;
    crate::utils::pdf_printf_args(
        b"/SMask %i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(smask_objnum)],
    );
    smask_size = png_get_rowbytes(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
    )
    .wrapping_div(2 as size_t)
    .wrapping_mul(png_get_image_height(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
    ) as size_t) as integer;
    smask =
        xmalloc((smask_size as size_t).wrapping_mul(::core::mem::size_of::<png_byte>() as size_t))
            as *mut png_byte as png_bytep;
    pdfbeginstream();
    if png_get_interlace_type(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
    ) as ::core::ffi::c_int
        == PNG_INTERLACE_NONE
    {
        row = xmalloc(
            png_get_rowbytes(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            )
            .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
        ) as *mut png_byte as png_bytep;
        if png_get_bit_depth(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
            == 16 as ::core::ffi::c_int
            && fixedimagehicolor != 0
        {
            i = 0 as ::core::ffi::c_int;
            while i < png_get_image_height(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
            {
                png_read_row(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
                    row,
                    ::core::ptr::null_mut::<png_byte>(),
                );
                r = row;
                k = png_get_rowbytes(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int;
                while k > 0 as ::core::ffi::c_int {
                    l = if k > pdfbufsize {
                        pdfbufsize as ::core::ffi::c_int
                    } else {
                        k
                    };
                    if (l as integer + pdfptr) as ::core::ffi::c_uint
                        > pdfbufsize as ::core::ffi::c_uint
                    {
                        if pdfosmode != 0 {
                            zpdfosgetosbuf(l);
                        } else if l as ::core::ffi::c_uint > pdfbufsize as ::core::ffi::c_uint {
                            crate::utils::pdftex_fail_args(
                                b"PDF output buffer overflowed\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[],
                            );
                        } else {
                            pdfflush();
                        }
                    }
                    j = 0 as ::core::ffi::c_int;
                    while j < l {
                        if j % 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            || j % 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                        {
                            let fresh37 = r;
                            r = r.offset(1);
                            let fresh38 = pdfptr;
                            pdfptr = pdfptr + 1;
                            *pdfbuf.offset(fresh38 as isize) = *fresh37 as eightbits;
                        } else {
                            let fresh39 = r;
                            r = r.offset(1);
                            let fresh40 = smask_ptr;
                            smask_ptr = smask_ptr + 1;
                            *smask.offset(fresh40 as isize) = *fresh39;
                        }
                        j += 1;
                    }
                    k -= l;
                }
                i += 1;
            }
        } else {
            i = 0 as ::core::ffi::c_int;
            while i < png_get_image_height(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
            {
                png_read_row(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
                    row,
                    ::core::ptr::null_mut::<png_byte>(),
                );
                r = row;
                k = png_get_rowbytes(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int;
                while k > 0 as ::core::ffi::c_int {
                    l = if k > pdfbufsize {
                        pdfbufsize as ::core::ffi::c_int
                    } else {
                        k
                    };
                    if (l as integer + pdfptr) as ::core::ffi::c_uint
                        > pdfbufsize as ::core::ffi::c_uint
                    {
                        if pdfosmode != 0 {
                            zpdfosgetosbuf(l);
                        } else if l as ::core::ffi::c_uint > pdfbufsize as ::core::ffi::c_uint {
                            crate::utils::pdftex_fail_args(
                                b"PDF output buffer overflowed\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[],
                            );
                        } else {
                            pdfflush();
                        }
                    }
                    j = 0 as ::core::ffi::c_int;
                    while j < l {
                        if j % 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            let fresh41 = r;
                            r = r.offset(1);
                            let fresh42 = pdfptr;
                            pdfptr = pdfptr + 1;
                            *pdfbuf.offset(fresh42 as isize) = *fresh41 as eightbits;
                        } else {
                            let fresh43 = r;
                            r = r.offset(1);
                            let fresh44 = smask_ptr;
                            smask_ptr = smask_ptr + 1;
                            *smask.offset(fresh44 as isize) = *fresh43;
                        }
                        j += 1;
                    }
                    k -= l;
                }
                i += 1;
            }
        }
        if !row.is_null() {
            free(row as *mut ::core::ffi::c_void);
        }
        row = ::core::ptr::null_mut::<png_byte>();
    } else {
        if (png_get_image_height(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as size_t)
            .wrapping_mul(png_get_rowbytes(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ))
            >= 10240000 as ::core::ffi::c_long as size_t
        {
            crate::utils::pdftex_warn_args(b"large interlaced PNG might cause out of memory (use non-interlaced PNG to fix this)\0"
                    as *const u8 as *const ::core::ffi::c_char, &[]);
        }
        rows = xmalloc(
            (png_get_image_height(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as size_t)
                .wrapping_mul(::core::mem::size_of::<png_bytep>() as size_t),
        ) as *mut png_bytep;
        i = 0 as ::core::ffi::c_int;
        while (i as ::core::ffi::c_uint)
            < png_get_image_height(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            )
        {
            let ref mut fresh45 = *rows.offset(i as isize);
            *fresh45 = xmalloc(
                png_get_rowbytes(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                )
                .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
            ) as *mut png_byte as png_bytep;
            i += 1;
        }
        png_read_image(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
            rows as png_bytepp,
        );
        if png_get_bit_depth(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
            == 16 as ::core::ffi::c_int
            && fixedimagehicolor != 0
        {
            i = 0 as ::core::ffi::c_int;
            while i < png_get_image_height(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
            {
                row = *rows.offset(i as isize);
                k = png_get_rowbytes(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int;
                while k > 0 as ::core::ffi::c_int {
                    l = if k > pdfbufsize {
                        pdfbufsize as ::core::ffi::c_int
                    } else {
                        k
                    };
                    if (l as integer + pdfptr) as ::core::ffi::c_uint
                        > pdfbufsize as ::core::ffi::c_uint
                    {
                        if pdfosmode != 0 {
                            zpdfosgetosbuf(l);
                        } else if l as ::core::ffi::c_uint > pdfbufsize as ::core::ffi::c_uint {
                            crate::utils::pdftex_fail_args(
                                b"PDF output buffer overflowed\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[],
                            );
                        } else {
                            pdfflush();
                        }
                    }
                    j = 0 as ::core::ffi::c_int;
                    while j < l {
                        if j % 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            || j % 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                        {
                            let fresh46 = row;
                            row = row.offset(1);
                            let fresh47 = pdfptr;
                            pdfptr = pdfptr + 1;
                            *pdfbuf.offset(fresh47 as isize) = *fresh46 as eightbits;
                        } else {
                            let fresh48 = row;
                            row = row.offset(1);
                            let fresh49 = smask_ptr;
                            smask_ptr = smask_ptr + 1;
                            *smask.offset(fresh49 as isize) = *fresh48;
                        }
                        j += 1;
                    }
                    k -= l;
                }
                if !(*rows.offset(i as isize)).is_null() {
                    free(*rows.offset(i as isize) as *mut ::core::ffi::c_void);
                }
                let ref mut fresh50 = *rows.offset(i as isize);
                *fresh50 = ::core::ptr::null_mut::<png_byte>();
                i += 1;
            }
        } else {
            i = 0 as ::core::ffi::c_int;
            while i < png_get_image_height(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
            {
                row = *rows.offset(i as isize);
                k = png_get_rowbytes(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int;
                while k > 0 as ::core::ffi::c_int {
                    l = if k > pdfbufsize {
                        pdfbufsize as ::core::ffi::c_int
                    } else {
                        k
                    };
                    if (l as integer + pdfptr) as ::core::ffi::c_uint
                        > pdfbufsize as ::core::ffi::c_uint
                    {
                        if pdfosmode != 0 {
                            zpdfosgetosbuf(l);
                        } else if l as ::core::ffi::c_uint > pdfbufsize as ::core::ffi::c_uint {
                            crate::utils::pdftex_fail_args(
                                b"PDF output buffer overflowed\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[],
                            );
                        } else {
                            pdfflush();
                        }
                    }
                    j = 0 as ::core::ffi::c_int;
                    while j < l {
                        if j % 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            let fresh51 = row;
                            row = row.offset(1);
                            let fresh52 = pdfptr;
                            pdfptr = pdfptr + 1;
                            *pdfbuf.offset(fresh52 as isize) = *fresh51 as eightbits;
                        } else {
                            let fresh53 = row;
                            row = row.offset(1);
                            let fresh54 = smask_ptr;
                            smask_ptr = smask_ptr + 1;
                            *smask.offset(fresh54 as isize) = *fresh53;
                        }
                        j += 1;
                    }
                    k -= l;
                }
                if !(*rows.offset(i as isize)).is_null() {
                    free(*rows.offset(i as isize) as *mut ::core::ffi::c_void);
                }
                let ref mut fresh55 = *rows.offset(i as isize);
                *fresh55 = ::core::ptr::null_mut::<png_byte>();
                i += 1;
            }
        }
        if !rows.is_null() {
            free(rows as *mut ::core::ffi::c_void);
        }
        rows = ::core::ptr::null_mut::<png_bytep>();
    }
    pdfendstream();
    pdfflush();
    if smask_objnum > 0 as ::core::ffi::c_int {
        bitdepth = png_get_bit_depth(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int;
        zpdfbegindict(smask_objnum, 0 as ::core::ffi::c_int);
        pdf_puts(b"/Type /XObject\n/Subtype /Image\n\0" as *const u8 as *const ::core::ffi::c_char);
        crate::utils::pdf_printf_args(
            b"/Width %i\n/Height %i\n/BitsPerComponent %i\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(png_get_image_width(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int),
                crate::utils::PrintfArg::from(png_get_image_height(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int),
                crate::utils::PrintfArg::from(if bitdepth == 16 as ::core::ffi::c_int {
                    8 as ::core::ffi::c_int
                } else {
                    bitdepth
                }),
            ],
        );
        pdf_puts(b"/ColorSpace /DeviceGray\n\0" as *const u8 as *const ::core::ffi::c_char);
        pdfbeginstream();
        if bitdepth == 8 as ::core::ffi::c_int {
            crate::utils::pdf_write_bytes(smask, smask_size as usize);
        } else {
            i = 0 as ::core::ffi::c_int;
            while i < smask_size {
                if i % 8 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    if (8 as integer + pdfptr) as ::core::ffi::c_uint
                        > pdfbufsize as ::core::ffi::c_uint
                    {
                        if pdfosmode != 0 {
                            zpdfosgetosbuf(8 as ::core::ffi::c_int);
                        } else if 8 as ::core::ffi::c_int as ::core::ffi::c_uint
                            > pdfbufsize as ::core::ffi::c_uint
                        {
                            crate::utils::pdftex_fail_args(
                                b"PDF output buffer overflowed\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[],
                            );
                        } else {
                            pdfflush();
                        }
                    }
                }
                let fresh56 = pdfptr;
                pdfptr = pdfptr + 1;
                *pdfbuf.offset(fresh56 as isize) = *smask.offset(i as isize) as eightbits;
                if bitdepth == 16 as ::core::ffi::c_int {
                    i += 1;
                }
                i += 1;
            }
        }
        if !smask.is_null() {
            free(smask as *mut ::core::ffi::c_void);
        }
        smask = ::core::ptr::null_mut::<png_byte>();
        pdfendstream();
    }
}
unsafe extern "C" fn write_png_rgb(mut img: integer) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut row: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut r: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut rows: *mut png_bytep = ::core::ptr::null_mut::<png_bytep>();
    if (*image_array.offset(img as isize)).colorspace_ref != 0 as ::core::ffi::c_int {
        crate::utils::pdf_printf_args(
            b"%i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                (*image_array.offset(img as isize)).colorspace_ref,
            )],
        );
    } else {
        pdf_puts(b"/DeviceRGB\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    pdfbeginstream();
    if png_get_interlace_type(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
    ) as ::core::ffi::c_int
        == PNG_INTERLACE_NONE
    {
        row = xmalloc(
            png_get_rowbytes(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            )
            .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
        ) as *mut png_byte as png_bytep;
        i = 0 as ::core::ffi::c_int;
        while i < png_get_image_height(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
        {
            png_read_row(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
                row,
                ::core::ptr::null_mut::<png_byte>(),
            );
            r = row;
            k = png_get_rowbytes(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int;
            while k > 0 as ::core::ffi::c_int {
                l = if k > pdfbufsize {
                    pdfbufsize as ::core::ffi::c_int
                } else {
                    k
                };
                if (l as integer + pdfptr) as ::core::ffi::c_uint
                    > pdfbufsize as ::core::ffi::c_uint
                {
                    if pdfosmode != 0 {
                        zpdfosgetosbuf(l);
                    } else if l as ::core::ffi::c_uint > pdfbufsize as ::core::ffi::c_uint {
                        crate::utils::pdftex_fail_args(
                            b"PDF output buffer overflowed\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    } else {
                        pdfflush();
                    }
                }
                j = 0 as ::core::ffi::c_int;
                while j < l {
                    let fresh5 = r;
                    r = r.offset(1);
                    let fresh6 = pdfptr;
                    pdfptr = pdfptr + 1;
                    *pdfbuf.offset(fresh6 as isize) = *fresh5 as eightbits;
                    j += 1;
                }
                k -= l;
            }
            i += 1;
        }
        if !row.is_null() {
            free(row as *mut ::core::ffi::c_void);
        }
        row = ::core::ptr::null_mut::<png_byte>();
    } else {
        if (png_get_image_height(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as size_t)
            .wrapping_mul(png_get_rowbytes(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ))
            >= 10240000 as ::core::ffi::c_long as size_t
        {
            crate::utils::pdftex_warn_args(b"large interlaced PNG might cause out of memory (use non-interlaced PNG to fix this)\0"
                    as *const u8 as *const ::core::ffi::c_char, &[]);
        }
        rows = xmalloc(
            (png_get_image_height(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as size_t)
                .wrapping_mul(::core::mem::size_of::<png_bytep>() as size_t),
        ) as *mut png_bytep;
        i = 0 as ::core::ffi::c_int;
        while (i as ::core::ffi::c_uint)
            < png_get_image_height(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            )
        {
            let ref mut fresh7 = *rows.offset(i as isize);
            *fresh7 = xmalloc(
                png_get_rowbytes(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                )
                .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
            ) as *mut png_byte as png_bytep;
            i += 1;
        }
        png_read_image(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
            rows as png_bytepp,
        );
        i = 0 as ::core::ffi::c_int;
        while i < png_get_image_height(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
        {
            row = *rows.offset(i as isize);
            k = png_get_rowbytes(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int;
            while k > 0 as ::core::ffi::c_int {
                l = if k > pdfbufsize {
                    pdfbufsize as ::core::ffi::c_int
                } else {
                    k
                };
                if (l as integer + pdfptr) as ::core::ffi::c_uint
                    > pdfbufsize as ::core::ffi::c_uint
                {
                    if pdfosmode != 0 {
                        zpdfosgetosbuf(l);
                    } else if l as ::core::ffi::c_uint > pdfbufsize as ::core::ffi::c_uint {
                        crate::utils::pdftex_fail_args(
                            b"PDF output buffer overflowed\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    } else {
                        pdfflush();
                    }
                }
                j = 0 as ::core::ffi::c_int;
                while j < l {
                    let fresh8 = row;
                    row = row.offset(1);
                    let fresh9 = pdfptr;
                    pdfptr = pdfptr + 1;
                    *pdfbuf.offset(fresh9 as isize) = *fresh8 as eightbits;
                    j += 1;
                }
                k -= l;
            }
            if !(*rows.offset(i as isize)).is_null() {
                free(*rows.offset(i as isize) as *mut ::core::ffi::c_void);
            }
            let ref mut fresh10 = *rows.offset(i as isize);
            *fresh10 = ::core::ptr::null_mut::<png_byte>();
            i += 1;
        }
        if !rows.is_null() {
            free(rows as *mut ::core::ffi::c_void);
        }
        rows = ::core::ptr::null_mut::<png_bytep>();
    }
    pdfendstream();
}
unsafe fn try_write_png_rgb_alpha_8_fast(
    mut img: integer,
    mut smask: png_bytep,
    mut smask_ptr: *mut integer,
) -> boolean {
    if smask.is_null() || smask_ptr.is_null() {
        return false_0 as boolean;
    }
    let width = png_get_image_width(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
    ) as usize;
    let height = png_get_image_height(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
    ) as usize;
    let Some(source_rowbytes) = width.checked_mul(4) else {
        return false_0 as boolean;
    };
    let Some(rgb_rowbytes) = width.checked_mul(3) else {
        return false_0 as boolean;
    };
    let Some(expected_len) = source_rowbytes.checked_mul(height) else {
        return false_0 as boolean;
    };

    let mut decoded_rowbytes: size_t = 0;
    let mut decoded_len: size_t = 0;
    let decoded = crate::pngshim::png_decoded_data(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as *mut ::core::ffi::c_void,
        &mut decoded_rowbytes,
        &mut decoded_len,
    );
    if decoded.is_null() || decoded_rowbytes != source_rowbytes || decoded_len < expected_len {
        return false_0 as boolean;
    }

    let rgb_row = xmalloc(rgb_rowbytes as size_t) as *mut u8;
    if rgb_row.is_null() {
        return false_0 as boolean;
    }

    let mut mask_offset = *smask_ptr as usize;
    for y in 0..height {
        let source = decoded.add(y * decoded_rowbytes);
        let mut source_offset = 0usize;
        let mut rgb_offset = 0usize;
        for _ in 0..width {
            *rgb_row.add(rgb_offset) = *source.add(source_offset);
            *rgb_row.add(rgb_offset + 1) = *source.add(source_offset + 1);
            *rgb_row.add(rgb_offset + 2) = *source.add(source_offset + 2);
            *smask.add(mask_offset) = *source.add(source_offset + 3);
            source_offset += 4;
            rgb_offset += 3;
            mask_offset += 1;
        }
        crate::utils::pdf_write_bytes(rgb_row, rgb_rowbytes);
    }

    free(rgb_row as *mut ::core::ffi::c_void);
    *smask_ptr = mask_offset as integer;
    true_0 as boolean
}
unsafe fn try_write_opaque_png_rgb_alpha_8_fast(mut img: integer) -> boolean {
    if png_get_bit_depth(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
    ) as ::core::ffi::c_int
        != 8 as ::core::ffi::c_int
        || png_get_interlace_type(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
            != PNG_INTERLACE_NONE
    {
        return false_0 as boolean;
    }

    let width = png_get_image_width(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
    ) as usize;
    let height = png_get_image_height(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
    ) as usize;
    let Some(source_rowbytes) = width.checked_mul(4) else {
        return false_0 as boolean;
    };
    let Some(rgb_rowbytes) = width.checked_mul(3) else {
        return false_0 as boolean;
    };
    let Some(expected_len) = source_rowbytes.checked_mul(height) else {
        return false_0 as boolean;
    };

    let mut decoded_rowbytes: size_t = 0;
    let mut decoded_len: size_t = 0;
    let decoded = crate::pngshim::png_decoded_data(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as *mut ::core::ffi::c_void,
        &mut decoded_rowbytes,
        &mut decoded_len,
    );
    if decoded.is_null() || decoded_rowbytes != source_rowbytes || decoded_len < expected_len {
        return false_0 as boolean;
    }

    for y in 0..height {
        let source = decoded.add(y * decoded_rowbytes);
        let mut alpha_offset = 3usize;
        while alpha_offset < source_rowbytes {
            if *source.add(alpha_offset) != 255 {
                return false_0 as boolean;
            }
            alpha_offset += 4;
        }
    }

    let rgb_row = xmalloc(rgb_rowbytes as size_t) as *mut u8;
    if rgb_row.is_null() {
        return false_0 as boolean;
    }

    pdfbeginstream();
    for y in 0..height {
        let source = decoded.add(y * decoded_rowbytes);
        let mut source_offset = 0usize;
        let mut rgb_offset = 0usize;
        for _ in 0..width {
            *rgb_row.add(rgb_offset) = *source.add(source_offset);
            *rgb_row.add(rgb_offset + 1) = *source.add(source_offset + 1);
            *rgb_row.add(rgb_offset + 2) = *source.add(source_offset + 2);
            source_offset += 4;
            rgb_offset += 3;
        }
        crate::utils::pdf_write_bytes(rgb_row, rgb_rowbytes);
    }
    pdfendstream();

    free(rgb_row as *mut ::core::ffi::c_void);
    true_0 as boolean
}
unsafe fn write_opaque_smask_bytes(mut len: usize) {
    let buf = [255u8; 8192];
    while len > 0 {
        let chunk = if len > buf.len() { buf.len() } else { len };
        crate::utils::pdf_write_bytes(buf.as_ptr(), chunk);
        len -= chunk;
    }
}

unsafe extern "C" fn write_png_rgb_alpha(mut img: integer) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut row: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut r: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut rows: *mut png_bytep = ::core::ptr::null_mut::<png_bytep>();
    let mut smask_objnum: integer = 0 as integer;
    let mut smask: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut smask_ptr: integer = 0 as integer;
    let mut smask_size: integer = 0 as integer;
    let mut bitdepth: ::core::ffi::c_int = 0;
    let mut opaque_rgb_alpha_fast: boolean = false_0 as boolean;
    if (*image_array.offset(img as isize)).colorspace_ref != 0 as ::core::ffi::c_int {
        crate::utils::pdf_printf_args(
            b"%i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                (*image_array.offset(img as isize)).colorspace_ref,
            )],
        );
    } else {
        pdf_puts(b"/DeviceRGB\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    zpdfcreateobj(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    smask_objnum = objptr;
    crate::utils::pdf_printf_args(
        b"/SMask %i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(smask_objnum)],
    );
    smask_size = png_get_rowbytes(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
    )
    .wrapping_div(4 as size_t)
    .wrapping_mul(png_get_image_height(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
    ) as size_t) as integer;
    opaque_rgb_alpha_fast = try_write_opaque_png_rgb_alpha_8_fast(img);
    if opaque_rgb_alpha_fast == 0 {
        smask = xmalloc(
            (smask_size as size_t).wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
        ) as *mut png_byte as png_bytep;
        pdfbeginstream();
        let fast_rgb_alpha = png_get_bit_depth(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
            == 8 as ::core::ffi::c_int
            && png_get_interlace_type(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
                == PNG_INTERLACE_NONE
            && try_write_png_rgb_alpha_8_fast(img, smask, &mut smask_ptr) != 0;
        if fast_rgb_alpha {
        } else if png_get_interlace_type(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
    ) as ::core::ffi::c_int
        == PNG_INTERLACE_NONE
    {
        row = xmalloc(
            png_get_rowbytes(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            )
            .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
        ) as *mut png_byte as png_bytep;
        if png_get_bit_depth(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
            == 16 as ::core::ffi::c_int
            && fixedimagehicolor != 0
        {
            i = 0 as ::core::ffi::c_int;
            while i < png_get_image_height(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
            {
                png_read_row(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
                    row,
                    ::core::ptr::null_mut::<png_byte>(),
                );
                r = row;
                k = png_get_rowbytes(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int;
                while k > 0 as ::core::ffi::c_int {
                    l = if k > pdfbufsize {
                        pdfbufsize as ::core::ffi::c_int
                    } else {
                        k
                    };
                    if (l as integer + pdfptr) as ::core::ffi::c_uint
                        > pdfbufsize as ::core::ffi::c_uint
                    {
                        if pdfosmode != 0 {
                            zpdfosgetosbuf(l);
                        } else if l as ::core::ffi::c_uint > pdfbufsize as ::core::ffi::c_uint {
                            crate::utils::pdftex_fail_args(
                                b"PDF output buffer overflowed\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[],
                            );
                        } else {
                            pdfflush();
                        }
                    }
                    j = 0 as ::core::ffi::c_int;
                    while j < l {
                        if !(j % 8 as ::core::ffi::c_int == 6 as ::core::ffi::c_int
                            || j % 8 as ::core::ffi::c_int == 7 as ::core::ffi::c_int)
                        {
                            let fresh11 = r;
                            r = r.offset(1);
                            let fresh12 = pdfptr;
                            pdfptr = pdfptr + 1;
                            *pdfbuf.offset(fresh12 as isize) = *fresh11 as eightbits;
                        } else {
                            let fresh13 = r;
                            r = r.offset(1);
                            let fresh14 = smask_ptr;
                            smask_ptr = smask_ptr + 1;
                            *smask.offset(fresh14 as isize) = *fresh13;
                        }
                        j += 1;
                    }
                    k -= l;
                }
                i += 1;
            }
        } else {
            i = 0 as ::core::ffi::c_int;
            while i < png_get_image_height(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
            {
                png_read_row(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
                    row,
                    ::core::ptr::null_mut::<png_byte>(),
                );
                r = row;
                k = png_get_rowbytes(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int;
                while k > 0 as ::core::ffi::c_int {
                    l = if k > pdfbufsize {
                        pdfbufsize as ::core::ffi::c_int
                    } else {
                        k
                    };
                    if (l as integer + pdfptr) as ::core::ffi::c_uint
                        > pdfbufsize as ::core::ffi::c_uint
                    {
                        if pdfosmode != 0 {
                            zpdfosgetosbuf(l);
                        } else if l as ::core::ffi::c_uint > pdfbufsize as ::core::ffi::c_uint {
                            crate::utils::pdftex_fail_args(
                                b"PDF output buffer overflowed\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[],
                            );
                        } else {
                            pdfflush();
                        }
                    }
                    j = 0 as ::core::ffi::c_int;
                    while j < l {
                        if j % 4 as ::core::ffi::c_int != 3 as ::core::ffi::c_int {
                            let fresh15 = r;
                            r = r.offset(1);
                            let fresh16 = pdfptr;
                            pdfptr = pdfptr + 1;
                            *pdfbuf.offset(fresh16 as isize) = *fresh15 as eightbits;
                        } else {
                            let fresh17 = r;
                            r = r.offset(1);
                            let fresh18 = smask_ptr;
                            smask_ptr = smask_ptr + 1;
                            *smask.offset(fresh18 as isize) = *fresh17;
                        }
                        j += 1;
                    }
                    k -= l;
                }
                i += 1;
            }
        }
        if !row.is_null() {
            free(row as *mut ::core::ffi::c_void);
        }
        row = ::core::ptr::null_mut::<png_byte>();
    } else {
        if (png_get_image_height(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as size_t)
            .wrapping_mul(png_get_rowbytes(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ))
            >= 10240000 as ::core::ffi::c_long as size_t
        {
            crate::utils::pdftex_warn_args(b"large interlaced PNG might cause out of memory (use non-interlaced PNG to fix this)\0"
                    as *const u8 as *const ::core::ffi::c_char, &[]);
        }
        rows = xmalloc(
            (png_get_image_height(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as size_t)
                .wrapping_mul(::core::mem::size_of::<png_bytep>() as size_t),
        ) as *mut png_bytep;
        i = 0 as ::core::ffi::c_int;
        while (i as ::core::ffi::c_uint)
            < png_get_image_height(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            )
        {
            let ref mut fresh19 = *rows.offset(i as isize);
            *fresh19 = xmalloc(
                png_get_rowbytes(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                )
                .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
            ) as *mut png_byte as png_bytep;
            i += 1;
        }
        png_read_image(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
            rows as png_bytepp,
        );
        if png_get_bit_depth(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
            == 16 as ::core::ffi::c_int
            && fixedimagehicolor != 0
        {
            i = 0 as ::core::ffi::c_int;
            while i < png_get_image_height(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
            {
                row = *rows.offset(i as isize);
                k = png_get_rowbytes(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int;
                while k > 0 as ::core::ffi::c_int {
                    l = if k > pdfbufsize {
                        pdfbufsize as ::core::ffi::c_int
                    } else {
                        k
                    };
                    if (l as integer + pdfptr) as ::core::ffi::c_uint
                        > pdfbufsize as ::core::ffi::c_uint
                    {
                        if pdfosmode != 0 {
                            zpdfosgetosbuf(l);
                        } else if l as ::core::ffi::c_uint > pdfbufsize as ::core::ffi::c_uint {
                            crate::utils::pdftex_fail_args(
                                b"PDF output buffer overflowed\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[],
                            );
                        } else {
                            pdfflush();
                        }
                    }
                    j = 0 as ::core::ffi::c_int;
                    while j < l {
                        if !(j % 8 as ::core::ffi::c_int == 6 as ::core::ffi::c_int
                            || j % 8 as ::core::ffi::c_int == 7 as ::core::ffi::c_int)
                        {
                            let fresh20 = row;
                            row = row.offset(1);
                            let fresh21 = pdfptr;
                            pdfptr = pdfptr + 1;
                            *pdfbuf.offset(fresh21 as isize) = *fresh20 as eightbits;
                        } else {
                            let fresh22 = row;
                            row = row.offset(1);
                            let fresh23 = smask_ptr;
                            smask_ptr = smask_ptr + 1;
                            *smask.offset(fresh23 as isize) = *fresh22;
                        }
                        j += 1;
                    }
                    k -= l;
                }
                if !(*rows.offset(i as isize)).is_null() {
                    free(*rows.offset(i as isize) as *mut ::core::ffi::c_void);
                }
                let ref mut fresh24 = *rows.offset(i as isize);
                *fresh24 = ::core::ptr::null_mut::<png_byte>();
                i += 1;
            }
        } else {
            i = 0 as ::core::ffi::c_int;
            while i < png_get_image_height(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
            {
                row = *rows.offset(i as isize);
                k = png_get_rowbytes(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int;
                while k > 0 as ::core::ffi::c_int {
                    l = if k > pdfbufsize {
                        pdfbufsize as ::core::ffi::c_int
                    } else {
                        k
                    };
                    if (l as integer + pdfptr) as ::core::ffi::c_uint
                        > pdfbufsize as ::core::ffi::c_uint
                    {
                        if pdfosmode != 0 {
                            zpdfosgetosbuf(l);
                        } else if l as ::core::ffi::c_uint > pdfbufsize as ::core::ffi::c_uint {
                            crate::utils::pdftex_fail_args(
                                b"PDF output buffer overflowed\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[],
                            );
                        } else {
                            pdfflush();
                        }
                    }
                    j = 0 as ::core::ffi::c_int;
                    while j < l {
                        if j % 4 as ::core::ffi::c_int != 3 as ::core::ffi::c_int {
                            let fresh25 = row;
                            row = row.offset(1);
                            let fresh26 = pdfptr;
                            pdfptr = pdfptr + 1;
                            *pdfbuf.offset(fresh26 as isize) = *fresh25 as eightbits;
                        } else {
                            let fresh27 = row;
                            row = row.offset(1);
                            let fresh28 = smask_ptr;
                            smask_ptr = smask_ptr + 1;
                            *smask.offset(fresh28 as isize) = *fresh27;
                        }
                        j += 1;
                    }
                    k -= l;
                }
                if !(*rows.offset(i as isize)).is_null() {
                    free(*rows.offset(i as isize) as *mut ::core::ffi::c_void);
                }
                let ref mut fresh29 = *rows.offset(i as isize);
                *fresh29 = ::core::ptr::null_mut::<png_byte>();
                i += 1;
            }
        }
        if !rows.is_null() {
            free(rows as *mut ::core::ffi::c_void);
        }
        rows = ::core::ptr::null_mut::<png_bytep>();
        }
        pdfendstream();
    }
    pdfflush();
    if smask_objnum > 0 as ::core::ffi::c_int {
        bitdepth = png_get_bit_depth(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int;
        zpdfbegindict(smask_objnum, 0 as ::core::ffi::c_int);
        pdf_puts(b"/Type /XObject\n/Subtype /Image\n\0" as *const u8 as *const ::core::ffi::c_char);
        crate::utils::pdf_printf_args(
            b"/Width %i\n/Height %i\n/BitsPerComponent %i\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(png_get_image_width(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int),
                crate::utils::PrintfArg::from(png_get_image_height(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int),
                crate::utils::PrintfArg::from(if bitdepth == 16 as ::core::ffi::c_int {
                    8 as ::core::ffi::c_int
                } else {
                    bitdepth
                }),
            ],
        );
        pdf_puts(b"/ColorSpace /DeviceGray\n\0" as *const u8 as *const ::core::ffi::c_char);
        pdfbeginstream();
        if bitdepth == 8 as ::core::ffi::c_int {
            if opaque_rgb_alpha_fast != 0 {
                write_opaque_smask_bytes(smask_size as usize);
            } else {
                crate::utils::pdf_write_bytes(smask, smask_size as usize);
            }
        } else {
            i = 0 as ::core::ffi::c_int;
            while i < smask_size {
                if i % 8 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    if (8 as integer + pdfptr) as ::core::ffi::c_uint
                        > pdfbufsize as ::core::ffi::c_uint
                    {
                        if pdfosmode != 0 {
                            zpdfosgetosbuf(8 as ::core::ffi::c_int);
                        } else if 8 as ::core::ffi::c_int as ::core::ffi::c_uint
                            > pdfbufsize as ::core::ffi::c_uint
                        {
                            crate::utils::pdftex_fail_args(
                                b"PDF output buffer overflowed\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[],
                            );
                        } else {
                            pdfflush();
                        }
                    }
                }
                let fresh30 = pdfptr;
                pdfptr = pdfptr + 1;
                *pdfbuf.offset(fresh30 as isize) = *smask.offset(i as isize) as eightbits;
                if bitdepth == 16 as ::core::ffi::c_int {
                    i += 1;
                }
                i += 1;
            }
        }
        if !smask.is_null() {
            free(smask as *mut ::core::ffi::c_void);
        }
        smask = ::core::ptr::null_mut::<png_byte>();
        pdfendstream();
    }
}
unsafe extern "C" fn spng_getint(mut fp: *mut FILE) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_uchar; 4] = [0; 4];
    if fread(
        &raw mut buf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        1 as size_t,
        4 as size_t,
        fp,
    ) != 4 as ::core::ffi::c_ulong
    {
        crate::utils::pdftex_fail_args(
            b"writepng: reading chunk type failed\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
    }
    return ((((((buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int)
        + buf[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int)
        + buf[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int)
        + buf[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
}
pub const SPNG_CHUNK_IDAT: ::core::ffi::c_int = 1229209940;
pub const SPNG_CHUNK_IEND: ::core::ffi::c_int = 1229278788;
unsafe extern "C" fn copy_png(mut img: integer) {
    let mut fp: *mut FILE = png_get_io_ptr(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
    ) as *mut FILE;
    let mut i: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut type_0: ::core::ffi::c_int = 0;
    let mut streamlength: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut endflag: boolean = false_0;
    let mut idat: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if fseek(fp, 8 as ::core::ffi::c_long, SEEK_SET) != 0 as ::core::ffi::c_int {
        crate::utils::pdftex_fail_args(
            b"writepng: fseek in PNG file failed\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
    }
    loop {
        len = spng_getint(fp);
        type_0 = spng_getint(fp);
        let mut current_block_7: u64;
        match type_0 {
            SPNG_CHUNK_IEND => {
                endflag = true_0 as boolean;
                current_block_7 = 1917311967535052937;
            }
            SPNG_CHUNK_IDAT => {
                streamlength += len;
                current_block_7 = 3343632758709677197;
            }
            _ => {
                current_block_7 = 3343632758709677197;
            }
        }
        match current_block_7 {
            3343632758709677197 => {
                if fseek(
                    fp,
                    (len + 4 as ::core::ffi::c_int) as ::core::ffi::c_long,
                    SEEK_CUR,
                ) != 0 as ::core::ffi::c_int
                {
                    crate::utils::pdftex_fail_args(
                        b"writepng: fseek in PNG file failed\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                }
            }
            _ => {}
        }
        if !(endflag == false_0) {
            break;
        }
    }
    crate::utils::pdf_printf_args(b"/Length %d\n/Filter/FlateDecode\n/DecodeParms<</Colors %d/Columns %d/BitsPerComponent %i/Predictor 10>>\n>>\nstream\n\0"
            as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(streamlength), crate::utils::PrintfArg::from(if png_get_color_type(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr
                as png_const_structrp,
            (*image_array.offset(img as isize)).image_struct.png.info_ptr
                as png_const_inforp,
        ) as ::core::ffi::c_int == 2 as ::core::ffi::c_int
        {
            3 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }), crate::utils::PrintfArg::from(png_get_image_width(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr
                as png_const_structrp,
            (*image_array.offset(img as isize)).image_struct.png.info_ptr
                as png_const_inforp,
        ) as ::core::ffi::c_int), crate::utils::PrintfArg::from(png_get_bit_depth(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr
                as png_const_structrp,
            (*image_array.offset(img as isize)).image_struct.png.info_ptr
                as png_const_inforp,
        ) as ::core::ffi::c_int)]);
    endflag = false_0 as boolean;
    if fseek(fp, 8 as ::core::ffi::c_long, SEEK_SET) != 0 as ::core::ffi::c_int {
        crate::utils::pdftex_fail_args(
            b"writepng: fseek in PNG file failed\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
    }
    loop {
        len = spng_getint(fp);
        type_0 = spng_getint(fp);
        match type_0 {
            SPNG_CHUNK_IDAT => {
                if idat == 2 as ::core::ffi::c_int {
                    crate::utils::pdftex_fail_args(
                        b"writepng: IDAT chunk sequence broken\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                }
                idat = 1 as ::core::ffi::c_int;
                while len > 0 as ::core::ffi::c_int {
                    i = if len > pdfbufsize {
                        pdfbufsize as ::core::ffi::c_int
                    } else {
                        len
                    };
                    if (i as integer + pdfptr) as ::core::ffi::c_uint
                        > pdfbufsize as ::core::ffi::c_uint
                    {
                        if pdfosmode != 0 {
                            zpdfosgetosbuf(i);
                        } else if i as ::core::ffi::c_uint > pdfbufsize as ::core::ffi::c_uint {
                            crate::utils::pdftex_fail_args(
                                b"PDF output buffer overflowed\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[],
                            );
                        } else {
                            pdfflush();
                        }
                    }
                    fread(
                        pdfbuf.offset(pdfptr as isize) as *mut eightbits
                            as *mut ::core::ffi::c_void,
                        1 as size_t,
                        i as size_t,
                        fp,
                    );
                    pdfptr += i;
                    len -= i;
                }
                if fseek(fp, 4 as ::core::ffi::c_long, SEEK_CUR) != 0 as ::core::ffi::c_int {
                    crate::utils::pdftex_fail_args(
                        b"writepng: fseek in PNG file failed\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                }
            }
            SPNG_CHUNK_IEND => {
                pdfendstream();
                endflag = true_0 as boolean;
            }
            _ => {
                if idat == 1 as ::core::ffi::c_int {
                    idat = 2 as ::core::ffi::c_int;
                }
                if fseek(
                    fp,
                    (len + 4 as ::core::ffi::c_int) as ::core::ffi::c_long,
                    SEEK_CUR,
                ) != 0 as ::core::ffi::c_int
                {
                    crate::utils::pdftex_fail_args(
                        b"writepng: fseek in PNG file failed\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                }
            }
        }
        if !(endflag == false_0) {
            break;
        }
    }
}
static mut last_png_needs_page_group: boolean = 0;
static mut transparent_page_group_was_written: boolean = false_0;
unsafe extern "C" fn write_additional_png_objects() {
    if last_png_needs_page_group != 0 {
        if transparent_page_group_was_written == 0
            && transparent_page_group > 0 as ::core::ffi::c_int
        {
            transparent_page_group_was_written = true_0 as boolean;
            zpdfbeginobj(transparent_page_group, 2 as ::core::ffi::c_int);
            if getpdfcompresslevel() == 0 as ::core::ffi::c_int {
                pdf_puts(
                    b"%PTEX Group needed for transparent pngs\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            pdf_puts(
                b"<</Type/Group /S/Transparency /CS/DeviceRGB /I true>>\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            pdfendobj();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn write_png(mut img: integer) {
    let mut png_copy: boolean = true_0;
    let mut gamma: ::core::ffi::c_double = 0.0f64;
    let mut int_file_gamma: png_fixed_point = 0 as png_fixed_point;
    let mut i: ::core::ffi::c_int = 0;
    let mut palette_objnum: integer = 0 as integer;
    let mut palette: png_colorp = ::core::ptr::null_mut::<png_color>();
    let mut num_palette: ::core::ffi::c_int = 0;
    last_png_needs_page_group = false_0 as boolean;
    png_get_PLTE(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_inforp,
        &raw mut palette,
        &raw mut num_palette,
    );
    if fixedpdfmajorversion == 1 as ::core::ffi::c_int
        && fixedpdfminorversion < 5 as ::core::ffi::c_int
    {
        fixedimagehicolor = 0 as ::core::ffi::c_int as boolean;
    }
    pdf_puts(b"/Type /XObject\n/Subtype /Image\n\0" as *const u8 as *const ::core::ffi::c_char);
    if png_get_valid(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        PNG_INFO_tRNS,
    ) != 0
    {
        png_set_tRNS_to_alpha(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
        );
        png_copy = false_0 as boolean;
    }
    if fixedpdfmajorversion == 1 as ::core::ffi::c_int
        && fixedpdfminorversion < 4 as ::core::ffi::c_int
        && png_get_color_type(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
            & PNG_COLOR_MASK_ALPHA
            != 0
    {
        png_set_strip_alpha(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
        );
        png_copy = false_0 as boolean;
    }
    if fixedpdfmajorversion == 1 as ::core::ffi::c_int
        && fixedpdfminorversion < 5 as ::core::ffi::c_int
    {
        fixedimagehicolor = 0 as ::core::ffi::c_int as boolean;
    }
    if png_get_bit_depth(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
    ) as ::core::ffi::c_int
        == 16 as ::core::ffi::c_int
        && fixedimagehicolor == 0 as ::core::ffi::c_int
    {
        png_set_strip_16(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
        );
        png_copy = false_0 as boolean;
    }
    if png_get_valid(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        PNG_INFO_gAMA,
    ) != 0
    {
        png_get_gAMA(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            &raw mut gamma,
        );
        png_get_gAMA_fixed(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            &raw mut int_file_gamma,
        );
    }
    if fixedimageapplygamma != 0 {
        if png_get_valid(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            PNG_INFO_gAMA,
        ) != 0
        {
            png_set_gamma(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
                fixedgamma as ::core::ffi::c_double / 1000.0f64,
                gamma,
            );
        } else {
            png_set_gamma(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
                fixedgamma as ::core::ffi::c_double / 1000.0f64,
                1000.0f64 / fixedimagegamma as ::core::ffi::c_double,
            );
        }
        png_copy = false_0 as boolean;
    }
    png_set_interlace_handling(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
    );
    png_read_update_info(
        (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_structrp,
        (*image_array.offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_inforp,
    );
    crate::utils::pdf_printf_args(
        b"/Width %i\n/Height %i\n/BitsPerComponent %i\n\0" as *const u8
            as *const ::core::ffi::c_char,
        &[
            crate::utils::PrintfArg::from(png_get_image_width(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int),
            crate::utils::PrintfArg::from(png_get_image_height(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int),
            crate::utils::PrintfArg::from(png_get_bit_depth(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int),
        ],
    );
    pdf_puts(b"/ColorSpace \0" as *const u8 as *const ::core::ffi::c_char);
    if png_copy != 0
        && (fixedpdfmajorversion > 1 as ::core::ffi::c_int
            || fixedpdfminorversion > 1 as ::core::ffi::c_int)
        && png_get_interlace_type(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
            == PNG_INTERLACE_NONE
        && (png_get_color_type(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
            == PNG_COLOR_TYPE_GRAY
            || png_get_color_type(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
                == PNG_COLOR_TYPE_RGB)
        && fixedimageapplygamma == 0
        && (png_get_valid(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            PNG_INFO_gAMA,
        ) == 0
            || int_file_gamma == PNG_FP_1)
        && png_get_valid(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            PNG_INFO_cHRM
                | PNG_INFO_iCCP
                | PNG_INFO_sBIT
                | PNG_INFO_sRGB
                | PNG_INFO_bKGD
                | PNG_INFO_hIST
                | PNG_INFO_tRNS
                | PNG_INFO_sPLT,
        ) == 0
    {
        if (*image_array.offset(img as isize)).colorspace_ref != 0 as ::core::ffi::c_int {
            crate::utils::pdf_printf_args(
                b"%i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(
                    (*image_array.offset(img as isize)).colorspace_ref,
                )],
            );
        } else {
            match png_get_color_type(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
            {
                PNG_COLOR_TYPE_PALETTE => {
                    zpdfcreateobj(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
                    palette_objnum = objptr;
                    crate::utils::pdf_printf_args(
                        b"[/Indexed /DeviceRGB %i %i 0 R]\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[
                            crate::utils::PrintfArg::from(num_palette - 1 as ::core::ffi::c_int),
                            crate::utils::PrintfArg::from(palette_objnum),
                        ],
                    );
                }
                PNG_COLOR_TYPE_GRAY => {
                    pdf_puts(b"/DeviceGray\n\0" as *const u8 as *const ::core::ffi::c_char);
                }
                _ => {
                    pdf_puts(b"/DeviceRGB\n\0" as *const u8 as *const ::core::ffi::c_char);
                }
            }
        }
        crate::utils::tex_printf_args(
            b" (PNG copy)\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
        copy_png(img);
        if palette_objnum > 0 as ::core::ffi::c_int {
            zpdfbegindict(palette_objnum, 0 as ::core::ffi::c_int);
            pdfbeginstream();
            i = 0 as ::core::ffi::c_int;
            while i < num_palette {
                if (3 as integer + pdfptr) as ::core::ffi::c_uint
                    > pdfbufsize as ::core::ffi::c_uint
                {
                    if pdfosmode != 0 {
                        zpdfosgetosbuf(3 as ::core::ffi::c_int);
                    } else if 3 as ::core::ffi::c_int as ::core::ffi::c_uint
                        > pdfbufsize as ::core::ffi::c_uint
                    {
                        crate::utils::pdftex_fail_args(
                            b"PDF output buffer overflowed\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    } else {
                        pdfflush();
                    }
                }
                let fresh2 = pdfptr;
                pdfptr = pdfptr + 1;
                *pdfbuf.offset(fresh2 as isize) = (*palette.offset(i as isize)).red as eightbits;
                let fresh3 = pdfptr;
                pdfptr = pdfptr + 1;
                *pdfbuf.offset(fresh3 as isize) = (*palette.offset(i as isize)).green as eightbits;
                let fresh4 = pdfptr;
                pdfptr = pdfptr + 1;
                *pdfbuf.offset(fresh4 as isize) = (*palette.offset(i as isize)).blue as eightbits;
                i += 1;
            }
            pdfendstream();
        }
    } else {
        if !getenv(b"TEXMF_DEBUG_PNG_COPY\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                getenv(b"TEXMF_DEBUG_PNG_COPY\0" as *const u8 as *const ::core::ffi::c_char),
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            crate::utils::tex_printf_args(
                b" *** PNG copy skipped because:\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
            if png_copy == 0 {
                crate::utils::tex_printf_args(
                    b" !png_copy\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if fixedpdfmajorversion == 1 as ::core::ffi::c_int
                && fixedpdfminorversion <= 1 as ::core::ffi::c_int
            {
                crate::utils::tex_printf_args(
                    b" minorversion=%d (and majorversion=1)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(fixedpdfminorversion)],
                );
            }
            if png_get_interlace_type(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
                != PNG_INTERLACE_NONE
            {
                crate::utils::tex_printf_args(
                    b" interlaced\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if !(png_get_color_type(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
                == PNG_COLOR_TYPE_GRAY
                || png_get_color_type(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                    (*image_array.offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int
                    == PNG_COLOR_TYPE_RGB)
            {
                crate::utils::tex_printf_args(
                    b" colortype\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if fixedimageapplygamma != 0 {
                crate::utils::tex_printf_args(
                    b" apply gamma\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if !(png_get_valid(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                PNG_INFO_gAMA,
            ) == 0
                || int_file_gamma == PNG_FP_1)
            {
                crate::utils::tex_printf_args(
                    b" gamma\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if png_get_valid(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                PNG_INFO_cHRM,
            ) != 0
            {
                crate::utils::tex_printf_args(
                    b" cHRM\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if png_get_valid(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                PNG_INFO_iCCP,
            ) != 0
            {
                crate::utils::tex_printf_args(
                    b" iCCP\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if png_get_valid(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                PNG_INFO_sBIT,
            ) != 0
            {
                crate::utils::tex_printf_args(
                    b" sBIT\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if png_get_valid(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                PNG_INFO_sRGB,
            ) != 0
            {
                crate::utils::tex_printf_args(
                    b" sRGB\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if png_get_valid(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                PNG_INFO_bKGD,
            ) != 0
            {
                crate::utils::tex_printf_args(
                    b" bKGD\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if png_get_valid(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                PNG_INFO_hIST,
            ) != 0
            {
                crate::utils::tex_printf_args(
                    b" hIST\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if png_get_valid(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                PNG_INFO_tRNS,
            ) != 0
            {
                crate::utils::tex_printf_args(
                    b" tRNS\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if png_get_valid(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                PNG_INFO_sPLT,
            ) != 0
            {
                crate::utils::tex_printf_args(
                    b" sPLT\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
        }
        match png_get_color_type(
            (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
            (*image_array.offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
        {
            PNG_COLOR_TYPE_PALETTE => {
                write_png_palette(img);
            }
            PNG_COLOR_TYPE_GRAY => {
                write_png_gray(img);
            }
            PNG_COLOR_TYPE_GRAY_ALPHA => {
                if fixedpdfminorversion >= 4 as ::core::ffi::c_int
                    || fixedpdfmajorversion > 1 as ::core::ffi::c_int
                {
                    write_png_gray_alpha(img);
                    last_png_needs_page_group = true_0 as boolean;
                } else {
                    write_png_gray(img);
                }
            }
            PNG_COLOR_TYPE_RGB => {
                write_png_rgb(img);
            }
            PNG_COLOR_TYPE_RGB_ALPHA => {
                if fixedpdfminorversion >= 4 as ::core::ffi::c_int
                    || fixedpdfmajorversion > 1 as ::core::ffi::c_int
                {
                    write_png_rgb_alpha(img);
                    last_png_needs_page_group = true_0 as boolean;
                } else {
                    write_png_rgb(img);
                }
            }
            _ => {
                crate::utils::pdftex_fail_args(
                    b"unsupported type of color_type <%i>\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(png_get_color_type(
                        (*image_array.offset(img as isize)).image_struct.png.png_ptr
                            as png_const_structrp,
                        (*image_array.offset(img as isize))
                            .image_struct
                            .png
                            .info_ptr as png_const_inforp,
                    )
                        as ::core::ffi::c_int)],
                );
            }
        }
    }
    pdfflush();
    write_additional_png_objects();
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
