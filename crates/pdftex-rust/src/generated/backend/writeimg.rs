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
    fn feof(_: *mut FILE) -> ::core::ffi::c_int;
    fn free(_: *mut ::core::ffi::c_void);
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strcasecmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn xstrdup(s: const_string) -> string;
    fn xfopen(filename: const_string, mode: const_string) -> *mut FILE;
    fn xfclose(fp: *mut FILE, filename: const_string);
    fn xmalloc(size: size_t) -> address;
    fn xrealloc(old_address: address, new_size: size_t) -> address;
    fn kpse_find_file(
        name: const_string,
        format: kpse_file_format_type,
        must_exist: boolean,
    ) -> string;
    fn recorder_record_input(_: const_string);
    fn zround(_: ::core::ffi::c_double) -> integer;
    fn find_input_file(s: integer) -> string;
    fn makecstring(s: integer) -> *mut ::core::ffi::c_char;
    fn do_dump(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: gzFile,
    );
    fn do_undump(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: gzFile,
    );
    static mut iniversion: boolean;
    static mut onehundredbp: scaled;
    static mut pdfimageprocset: integer;
    static mut fmtfile: wordfile;
    static mut cur_file_name: *mut ::core::ffi::c_char;
    static mut last_ptr_index: size_t;
    fn epdf_delete();
    fn read_pdf_info(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn pdftex_fail(_: *const ::core::ffi::c_char, ...) -> !;
    fn tex_printf(_: *const ::core::ffi::c_char, ...);
    fn write_epdf();
    fn xgetc(_: *mut FILE) -> ::core::ffi::c_int;
    fn png_destroy_read_struct(
        png_ptr_ptr: png_structpp,
        info_ptr_ptr: png_infopp,
        end_info_ptr_ptr: png_infopp,
    );
    fn png_get_io_ptr(png_ptr: png_const_structrp) -> png_voidp;
    fn png_get_bit_depth(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> png_byte;
    fn read_png_info(_: integer);
    fn write_png(_: integer);
    fn read_jpg_info(_: integer);
    fn write_jpg(_: integer);
    fn read_jbig2_info(_: integer);
    fn write_jbig2(_: integer);
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
pub type string = *mut ::core::ffi::c_char;
pub type const_string = *const ::core::ffi::c_char;
pub type address = *mut ::core::ffi::c_void;
pub type kpse_file_format_type = ::core::ffi::c_uint;
pub const kpse_last_format: kpse_file_format_type = 59;
pub const kpse_bltxml_format: kpse_file_format_type = 58;
pub const kpse_ris_format: kpse_file_format_type = 57;
pub const kpse_clua_format: kpse_file_format_type = 56;
pub const kpse_mlbst_format: kpse_file_format_type = 55;
pub const kpse_mlbib_format: kpse_file_format_type = 54;
pub const kpse_cid_format: kpse_file_format_type = 53;
pub const kpse_fea_format: kpse_file_format_type = 52;
pub const kpse_lua_format: kpse_file_format_type = 51;
pub const kpse_texmfscripts_format: kpse_file_format_type = 50;
pub const kpse_lig_format: kpse_file_format_type = 49;
pub const kpse_pdftex_config_format: kpse_file_format_type = 48;
pub const kpse_opentype_format: kpse_file_format_type = 47;
pub const kpse_sfd_format: kpse_file_format_type = 46;
pub const kpse_cmap_format: kpse_file_format_type = 45;
pub const kpse_enc_format: kpse_file_format_type = 44;
pub const kpse_cweb_format: kpse_file_format_type = 43;
pub const kpse_web_format: kpse_file_format_type = 42;
pub const kpse_miscfonts_format: kpse_file_format_type = 41;
pub const kpse_program_binary_format: kpse_file_format_type = 40;
pub const kpse_program_text_format: kpse_file_format_type = 39;
pub const kpse_web2c_format: kpse_file_format_type = 38;
pub const kpse_type42_format: kpse_file_format_type = 37;
pub const kpse_truetype_format: kpse_file_format_type = 36;
pub const kpse_ist_format: kpse_file_format_type = 35;
pub const kpse_dvips_config_format: kpse_file_format_type = 34;
pub const kpse_vf_format: kpse_file_format_type = 33;
pub const kpse_type1_format: kpse_file_format_type = 32;
pub const kpse_troff_font_format: kpse_file_format_type = 31;
pub const kpse_tex_ps_header_format: kpse_file_format_type = 30;
pub const kpse_texsource_format: kpse_file_format_type = 29;
pub const kpse_texpool_format: kpse_file_format_type = 28;
pub const kpse_texdoc_format: kpse_file_format_type = 27;
pub const kpse_tex_format: kpse_file_format_type = 26;
pub const kpse_pict_format: kpse_file_format_type = 25;
pub const kpse_ovp_format: kpse_file_format_type = 24;
pub const kpse_ovf_format: kpse_file_format_type = 23;
pub const kpse_otp_format: kpse_file_format_type = 22;
pub const kpse_opl_format: kpse_file_format_type = 21;
pub const kpse_ofm_format: kpse_file_format_type = 20;
pub const kpse_ocp_format: kpse_file_format_type = 19;
pub const kpse_mpsupport_format: kpse_file_format_type = 18;
pub const kpse_mppool_format: kpse_file_format_type = 17;
pub const kpse_mp_format: kpse_file_format_type = 16;
pub const kpse_mft_format: kpse_file_format_type = 15;
pub const kpse_mfpool_format: kpse_file_format_type = 14;
pub const kpse_mf_format: kpse_file_format_type = 13;
pub const kpse_mem_format: kpse_file_format_type = 12;
pub const kpse_fontmap_format: kpse_file_format_type = 11;
pub const kpse_fmt_format: kpse_file_format_type = 10;
pub const kpse_db_format: kpse_file_format_type = 9;
pub const kpse_cnf_format: kpse_file_format_type = 8;
pub const kpse_bst_format: kpse_file_format_type = 7;
pub const kpse_bib_format: kpse_file_format_type = 6;
pub const kpse_base_format: kpse_file_format_type = 5;
pub const kpse_afm_format: kpse_file_format_type = 4;
pub const kpse_tfm_format: kpse_file_format_type = 3;
pub const kpse_any_glyph_format: kpse_file_format_type = 2;
pub const kpse_pk_format: kpse_file_format_type = 1;
pub const kpse_gf_format: kpse_file_format_type = 0;
pub type integer = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: ::core::ffi::c_uint,
    pub next: *mut ::core::ffi::c_uchar,
    pub pos: off_t,
}
pub type gzFile = *mut gzFile_s;
pub type strnumber = integer;
pub type scaled = integer;
pub type wordfile = gzFile;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JBIG2_IMAGE_INFO {
    pub selected_page: integer,
    pub file: *mut FILE,
}
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
pub type png_infop = *mut png_info;
pub type png_info = png_info_def;
pub type png_structp = *mut png_struct;
pub type png_struct = png_struct_def;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_image_struct {
    pub orig_x: integer,
    pub orig_y: integer,
    pub selected_page: integer,
    pub page_box: integer,
    pub doc: *mut ::core::ffi::c_void,
}
pub type png_byte = ::core::ffi::c_uchar;
pub type png_const_inforp = *const png_info;
pub type png_const_structrp = *const png_struct;
pub type png_infopp = *mut *mut png_info;
pub type png_structpp = *mut *mut png_struct;
pub type png_voidp = *mut ::core::ffi::c_void;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const FOPEN_RBIN_MODE: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"rb\0") };
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IMAGE_TYPE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const IMAGE_TYPE_PDF: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IMAGE_TYPE_PNG: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const IMAGE_TYPE_JPG: ::core::ffi::c_int = 3;
pub const IMAGE_TYPE_JBIG2: ::core::ffi::c_int = 5;
pub const IMAGE_COLOR_B: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IMAGE_COLOR_C: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const IMAGE_COLOR_I: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[no_mangle]
pub static mut image_ptr: *mut image_entry = ::core::ptr::null::<image_entry>() as *mut image_entry;
#[no_mangle]
pub static mut image_array: *mut image_entry =
    ::core::ptr::null::<image_entry>() as *mut image_entry;
#[no_mangle]
pub static mut image_limit: integer = 0;
#[no_mangle]
pub static mut epdf_width: ::core::ffi::c_float = 0.;
#[no_mangle]
pub static mut epdf_height: ::core::ffi::c_float = 0.;
#[no_mangle]
pub static mut epdf_orig_x: ::core::ffi::c_float = 0.;
#[no_mangle]
pub static mut epdf_orig_y: ::core::ffi::c_float = 0.;
#[no_mangle]
pub static mut epdf_rotate: ::core::ffi::c_float = 0.;
#[no_mangle]
pub static mut epdf_selected_page: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut epdf_num_pages: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut epdf_page_box: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut epdf_doc: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[no_mangle]
pub static mut epdf_has_page_group: ::core::ffi::c_int = 0;
unsafe extern "C" fn new_image_entry() -> integer {
    if image_array.is_null() {
        image_limit = 256 as ::core::ffi::c_int as integer;
        if 1 as ::core::ffi::c_int as ::core::ffi::c_uint > image_limit as ::core::ffi::c_uint {
            image_limit = 1 as ::core::ffi::c_int as integer;
        }
        image_array = xmalloc(
            (image_limit as size_t).wrapping_mul(::core::mem::size_of::<image_entry>() as size_t),
        ) as *mut image_entry;
        image_ptr = image_array;
    } else if (image_ptr.offset_from(image_array) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
        as ::core::ffi::c_uint
        > image_limit as ::core::ffi::c_uint
    {
        last_ptr_index = image_ptr.offset_from(image_array) as ::core::ffi::c_long as size_t;
        image_limit *= 2 as ::core::ffi::c_int;
        if (image_ptr.offset_from(image_array) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
            as ::core::ffi::c_uint
            > image_limit as ::core::ffi::c_uint
        {
            image_limit = (image_ptr.offset_from(image_array) as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as integer;
        }
        if image_limit as ::core::ffi::c_uint > INT_MAX as ::core::ffi::c_uint {
            pdftex_fail(
                b"image_array exceeds size limit\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        image_array = xrealloc(
            image_array as address,
            (image_limit as size_t).wrapping_mul(::core::mem::size_of::<image_entry>() as size_t),
        ) as *mut image_entry;
        image_ptr = image_array.offset(last_ptr_index as isize);
    }
    (*image_ptr).image_type = IMAGE_TYPE_NONE;
    (*image_ptr).color_type = 0 as ::core::ffi::c_int;
    (*image_ptr).num_pages = 0 as ::core::ffi::c_int as integer;
    (*image_ptr).x_res = 0 as ::core::ffi::c_int as integer;
    (*image_ptr).y_res = 0 as ::core::ffi::c_int as integer;
    (*image_ptr).width = 0 as ::core::ffi::c_int as integer;
    (*image_ptr).height = 0 as ::core::ffi::c_int as integer;
    (*image_ptr).rotate = 0 as ::core::ffi::c_int as integer;
    (*image_ptr).colorspace_ref = 0 as ::core::ffi::c_int as integer;
    (*image_ptr).group_ref = 0 as ::core::ffi::c_int as integer;
    let fresh5 = image_ptr;
    image_ptr = image_ptr.offset(1);
    return fresh5.offset_from(image_array) as ::core::ffi::c_long as integer;
}
#[no_mangle]
pub unsafe extern "C" fn imagecolor(mut img: integer) -> integer {
    return (*image_array.offset(img as isize)).color_type as integer;
}
#[no_mangle]
pub unsafe extern "C" fn imagewidth(mut img: integer) -> integer {
    return (*image_array.offset(img as isize)).width;
}
#[no_mangle]
pub unsafe extern "C" fn imageheight(mut img: integer) -> integer {
    return (*image_array.offset(img as isize)).height;
}
#[no_mangle]
pub unsafe extern "C" fn imagerotate(mut img: integer) -> integer {
    return (*image_array.offset(img as isize)).rotate;
}
#[no_mangle]
pub unsafe extern "C" fn imagexres(mut img: integer) -> integer {
    return (*image_array.offset(img as isize)).x_res;
}
#[no_mangle]
pub unsafe extern "C" fn imageyres(mut img: integer) -> integer {
    return (*image_array.offset(img as isize)).y_res;
}
#[no_mangle]
pub unsafe extern "C" fn ispdfimage(mut img: integer) -> boolean {
    return ((*image_array.offset(img as isize)).image_type == IMAGE_TYPE_PDF)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ispngimage(mut img: integer) -> boolean {
    return ((*image_array.offset(img as isize)).image_type == IMAGE_TYPE_PNG)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn checkimageb(mut procset: integer) -> boolean {
    return procset as boolean & IMAGE_COLOR_B;
}
#[no_mangle]
pub unsafe extern "C" fn checkimagec(mut procset: integer) -> boolean {
    return procset as boolean & IMAGE_COLOR_C;
}
#[no_mangle]
pub unsafe extern "C" fn checkimagei(mut procset: integer) -> boolean {
    return procset as boolean & IMAGE_COLOR_I;
}
#[no_mangle]
pub unsafe extern "C" fn updateimageprocset(mut img: integer) {
    pdfimageprocset |= (*image_array.offset(img as isize)).color_type;
}
#[no_mangle]
pub unsafe extern "C" fn epdforigx(mut img: integer) -> integer {
    return (*(*image_array.offset(img as isize)).image_struct.pdf).orig_x;
}
#[no_mangle]
pub unsafe extern "C" fn epdforigy(mut img: integer) -> integer {
    return (*(*image_array.offset(img as isize)).image_struct.pdf).orig_y;
}
#[no_mangle]
pub unsafe extern "C" fn imagepages(mut img: integer) -> integer {
    return (*image_array.offset(img as isize)).num_pages;
}
#[no_mangle]
pub unsafe extern "C" fn imagecolordepth(mut img: integer) -> integer {
    match (*image_array.offset(img as isize)).image_type {
        IMAGE_TYPE_PNG => {
            return png_get_bit_depth(
                (*image_array.offset(img as isize)).image_struct.png.png_ptr as png_const_structrp,
                (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
            ) as integer;
        }
        IMAGE_TYPE_JPG => {
            return (*(*image_array.offset(img as isize)).image_struct.jpg).bits_per_component
                as integer;
        }
        IMAGE_TYPE_JBIG2 => return 0 as integer,
        IMAGE_TYPE_PDF => return 0 as integer,
        _ => {
            pdftex_fail(b"unknown type of image\0" as *const u8 as *const ::core::ffi::c_char);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn getimagegroupref(mut img: integer) -> integer {
    return (*image_array.offset(img as isize)).group_ref;
}
#[no_mangle]
pub unsafe extern "C" fn setimagegroupref(mut img: integer, mut value: integer) {
    (*image_array.offset(img as isize)).group_ref = value;
}
pub const HEADER_JPG: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"\xFF\xD8\0") };
pub const HEADER_PNG: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"\x89PNG\r\n\x1A\n\0") };
pub const HEADER_JBIG2: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"\x97JB2\r\n\x1A\n\0") };
pub const HEADER_PDF: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"%PDF-1.\0") };
pub const MAX_HEADER: usize =
    (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as usize).wrapping_sub(1 as usize);
unsafe extern "C" fn checktypebyheader(mut img: integer) {
    let mut i: ::core::ffi::c_int = 0;
    let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut header: [::core::ffi::c_char; 8] = [0; 8];
    if (*image_array.offset(img as isize)).image_type != IMAGE_TYPE_NONE {
        return;
    }
    file = xfopen(
        (*image_array.offset(img as isize)).image_name as const_string,
        FOPEN_RBIN_MODE.as_ptr(),
    );
    i = 0 as ::core::ffi::c_int;
    while (i as ::core::ffi::c_uint as usize) < MAX_HEADER {
        header[i as usize] = xgetc(file) as ::core::ffi::c_char;
        if feof(file) != 0 {
            pdftex_fail(b"reading image file failed\0" as *const u8 as *const ::core::ffi::c_char);
        }
        i += 1;
    }
    xfclose(
        file,
        (*image_array.offset(img as isize)).image_name as const_string,
    );
    if strncmp(
        &raw mut header as *mut ::core::ffi::c_char,
        HEADER_JPG.as_ptr(),
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1 as size_t),
    ) == 0 as ::core::ffi::c_int
    {
        (*image_array.offset(img as isize)).image_type = IMAGE_TYPE_JPG;
    } else if strncmp(
        &raw mut header as *mut ::core::ffi::c_char,
        HEADER_PNG.as_ptr(),
        (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t).wrapping_sub(1 as size_t),
    ) == 0 as ::core::ffi::c_int
    {
        (*image_array.offset(img as isize)).image_type = IMAGE_TYPE_PNG;
    } else if strncmp(
        &raw mut header as *mut ::core::ffi::c_char,
        HEADER_JBIG2.as_ptr(),
        (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t).wrapping_sub(1 as size_t),
    ) == 0 as ::core::ffi::c_int
    {
        (*image_array.offset(img as isize)).image_type = IMAGE_TYPE_JBIG2;
    } else if strncmp(
        &raw mut header as *mut ::core::ffi::c_char,
        HEADER_PDF.as_ptr(),
        (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as size_t).wrapping_sub(1 as size_t),
    ) == 0 as ::core::ffi::c_int
    {
        (*image_array.offset(img as isize)).image_type = IMAGE_TYPE_PDF;
    }
}
unsafe extern "C" fn checktypebyextension(mut img: integer) {
    let mut image_suffix: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*image_array.offset(img as isize)).image_type != IMAGE_TYPE_NONE {
        return;
    }
    image_suffix = strrchr(cur_file_name, '.' as i32);
    if image_suffix.is_null() {
        (*image_array.offset(img as isize)).image_type = IMAGE_TYPE_NONE;
    } else if strcasecmp(
        image_suffix,
        b".png\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*image_array.offset(img as isize)).image_type = IMAGE_TYPE_PNG;
    } else if strcasecmp(
        image_suffix,
        b".jpg\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcasecmp(
            image_suffix,
            b".jpeg\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        (*image_array.offset(img as isize)).image_type = IMAGE_TYPE_JPG;
    } else if strcasecmp(
        image_suffix,
        b".jbig2\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcasecmp(
            image_suffix,
            b".jb2\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        (*image_array.offset(img as isize)).image_type = IMAGE_TYPE_JBIG2;
    } else if strcasecmp(
        image_suffix,
        b".pdf\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*image_array.offset(img as isize)).image_type = IMAGE_TYPE_PDF;
    }
}
#[no_mangle]
pub unsafe extern "C" fn readimage(
    mut s: strnumber,
    mut page_num: integer,
    mut page_name: strnumber,
    mut colorspace: integer,
    mut pagebox: integer,
    mut pdfmajorversion: integer,
    mut pdfminorversion: integer,
    mut pdfinclusionerrorlevel: integer,
) -> integer {
    let mut dest: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut img: integer = new_image_entry();
    (*image_array.offset(img as isize)).colorspace_ref = colorspace;
    if page_name != 0 as ::core::ffi::c_int {
        dest =
            xstrdup(makecstring(page_name as integer) as const_string) as *mut ::core::ffi::c_char;
    }
    cur_file_name = find_input_file(s as integer) as *mut ::core::ffi::c_char;
    let ref mut fresh0 = (*image_array.offset(img as isize)).image_name;
    *fresh0 = cur_file_name;
    if (*image_array.offset(img as isize)).image_name.is_null() {
        pdftex_fail(
            b"cannot find image file %s\0" as *const u8 as *const ::core::ffi::c_char,
            makecstring(s as integer),
        );
    }
    recorder_record_input(cur_file_name as const_string);
    checktypebyheader(img);
    checktypebyextension(img);
    match (*image_array.offset(img as isize)).image_type {
        IMAGE_TYPE_PDF => {
            let ref mut fresh1 = (*image_array.offset(img as isize)).image_struct.pdf;
            *fresh1 = xmalloc(
                (1 as size_t).wrapping_mul(::core::mem::size_of::<pdf_image_struct>() as size_t),
            ) as *mut pdf_image_struct;
            (*(*image_array.offset(img as isize)).image_struct.pdf).page_box = pagebox;
            page_num = read_pdf_info(
                (*image_array.offset(img as isize)).image_name,
                dest,
                page_num as ::core::ffi::c_int,
                pagebox as ::core::ffi::c_int,
                pdfmajorversion as ::core::ffi::c_int,
                pdfminorversion as ::core::ffi::c_int,
                pdfinclusionerrorlevel as ::core::ffi::c_int,
            ) as integer;
            (*image_array.offset(img as isize)).width = zround(
                epdf_width as ::core::ffi::c_double
                    * (onehundredbp as ::core::ffi::c_double / 100.0f64),
            );
            (*image_array.offset(img as isize)).height = zround(
                epdf_height as ::core::ffi::c_double
                    * (onehundredbp as ::core::ffi::c_double / 100.0f64),
            );
            (*image_array.offset(img as isize)).rotate = epdf_rotate as integer;
            (*image_array.offset(img as isize)).num_pages = epdf_num_pages as integer;
            (*(*image_array.offset(img as isize)).image_struct.pdf).orig_x = zround(
                epdf_orig_x as ::core::ffi::c_double
                    * (onehundredbp as ::core::ffi::c_double / 100.0f64),
            );
            (*(*image_array.offset(img as isize)).image_struct.pdf).orig_y = zround(
                epdf_orig_y as ::core::ffi::c_double
                    * (onehundredbp as ::core::ffi::c_double / 100.0f64),
            );
            (*(*image_array.offset(img as isize)).image_struct.pdf).selected_page = page_num;
            let ref mut fresh2 = (*(*image_array.offset(img as isize)).image_struct.pdf).doc;
            *fresh2 = epdf_doc;
            if epdf_has_page_group == 1 as ::core::ffi::c_int {
                (*image_array.offset(img as isize)).group_ref =
                    -(1 as ::core::ffi::c_int) as integer;
            } else {
                (*image_array.offset(img as isize)).group_ref = 0 as ::core::ffi::c_int as integer;
            }
        }
        IMAGE_TYPE_PNG => {
            (*image_array.offset(img as isize)).num_pages = 1 as ::core::ffi::c_int as integer;
            read_png_info(img);
        }
        IMAGE_TYPE_JPG => {
            let ref mut fresh3 = (*image_array.offset(img as isize)).image_struct.jpg;
            *fresh3 = xmalloc(
                (1 as size_t).wrapping_mul(::core::mem::size_of::<JPG_IMAGE_INFO>() as size_t),
            ) as *mut JPG_IMAGE_INFO;
            (*image_array.offset(img as isize)).num_pages = 1 as ::core::ffi::c_int as integer;
            read_jpg_info(img);
        }
        IMAGE_TYPE_JBIG2 => {
            if pdfmajorversion == 1 as ::core::ffi::c_int
                && pdfminorversion < 4 as ::core::ffi::c_int
            {
                pdftex_fail(
                    b"JBIG2 images only possible with at least PDF 1.4; you are generating PDF 1.%i\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    pdfminorversion,
                );
            }
            let ref mut fresh4 = (*image_array.offset(img as isize)).image_struct.jbig2;
            *fresh4 = xmalloc(
                (1 as size_t).wrapping_mul(::core::mem::size_of::<JBIG2_IMAGE_INFO>() as size_t),
            ) as *mut JBIG2_IMAGE_INFO;
            (*image_array.offset(img as isize)).image_type = IMAGE_TYPE_JBIG2;
            (*(*image_array.offset(img as isize)).image_struct.jbig2).selected_page = page_num;
            read_jbig2_info(img);
        }
        _ => {
            pdftex_fail(b"unknown type of image\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
    if !dest.is_null() {
        free(dest as *mut ::core::ffi::c_void);
    }
    dest = ::core::ptr::null_mut::<::core::ffi::c_char>();
    cur_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return img;
}
#[no_mangle]
pub unsafe extern "C" fn writeimage(mut img: integer) {
    cur_file_name = (*image_array.offset(img as isize)).image_name;
    tex_printf(
        b" <%s\0" as *const u8 as *const ::core::ffi::c_char,
        (*image_array.offset(img as isize)).image_name,
    );
    match (*image_array.offset(img as isize)).image_type {
        IMAGE_TYPE_PNG => {
            write_png(img);
        }
        IMAGE_TYPE_JPG => {
            write_jpg(img);
        }
        IMAGE_TYPE_JBIG2 => {
            write_jbig2(img);
        }
        IMAGE_TYPE_PDF => {
            epdf_doc = (*(*image_array.offset(img as isize)).image_struct.pdf).doc;
            epdf_selected_page = (*(*image_array.offset(img as isize)).image_struct.pdf)
                .selected_page as ::core::ffi::c_int;
            epdf_page_box = (*(*image_array.offset(img as isize)).image_struct.pdf).page_box
                as ::core::ffi::c_int;
            write_epdf();
        }
        _ => {
            pdftex_fail(b"unknown type of image\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
    tex_printf(b">\0" as *const u8 as *const ::core::ffi::c_char);
    cur_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn deleteimage(mut img: integer) {
    if iniversion != 0 {
        return;
    }
    match (*image_array.offset(img as isize)).image_type {
        IMAGE_TYPE_PDF => {
            epdf_doc = (*(*image_array.offset(img as isize)).image_struct.pdf).doc;
            epdf_delete();
        }
        IMAGE_TYPE_PNG => {
            xfclose(
                png_get_io_ptr(
                    (*image_array.offset(img as isize)).image_struct.png.png_ptr
                        as png_const_structrp,
                ) as *mut FILE,
                cur_file_name as const_string,
            );
            png_destroy_read_struct(
                &raw mut (*image_array.offset(img as isize)).image_struct.png.png_ptr,
                &raw mut (*image_array.offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr,
                ::core::ptr::null_mut::<*mut png_info>(),
            );
        }
        IMAGE_TYPE_JPG => {
            xfclose(
                (*(*image_array.offset(img as isize)).image_struct.jpg).file,
                cur_file_name as const_string,
            );
        }
        IMAGE_TYPE_JBIG2 => {}
        _ => {
            pdftex_fail(b"unknown type of image\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
    if !(*image_array.offset(img as isize)).image_name.is_null() {
        free((*image_array.offset(img as isize)).image_name as *mut ::core::ffi::c_void);
    }
    let ref mut fresh6 = (*image_array.offset(img as isize)).image_name;
    *fresh6 = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn img_free() {
    if !image_array.is_null() {
        free(image_array as *mut ::core::ffi::c_void);
    }
    image_array = ::core::ptr::null_mut::<image_entry>();
}
#[no_mangle]
pub unsafe extern "C" fn dumpimagemeta() {
    let mut cur_image: ::core::ffi::c_int = 0;
    let mut img: ::core::ffi::c_int = 0;
    do_dump(
        &raw mut image_limit as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    cur_image = image_ptr.offset_from(image_array) as ::core::ffi::c_long as ::core::ffi::c_int;
    do_dump(
        &raw mut cur_image as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    img = 0 as ::core::ffi::c_int;
    while img < cur_image {
        let mut x: integer = 0;
        if !(*image_array.offset(img as isize)).image_name.is_null() {
            x = strlen((*image_array.offset(img as isize)).image_name).wrapping_add(1 as size_t)
                as integer;
            do_dump(
                &raw mut x as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                fmtfile as gzFile,
            );
            do_dump(
                (*image_array.offset(img as isize)).image_name,
                ::core::mem::size_of::<::core::ffi::c_char>() as ::core::ffi::c_int,
                x,
                fmtfile as gzFile,
            );
        } else {
            x = 0 as ::core::ffi::c_int as integer;
            do_dump(
                &raw mut x as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                fmtfile as gzFile,
            );
        }
        do_dump(
            &raw mut (*image_array.offset(img as isize)).image_type as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_dump(
            &raw mut (*image_array.offset(img as isize)).color_type as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_dump(
            &raw mut (*image_array.offset(img as isize)).width as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_dump(
            &raw mut (*image_array.offset(img as isize)).height as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_dump(
            &raw mut (*image_array.offset(img as isize)).x_res as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_dump(
            &raw mut (*image_array.offset(img as isize)).y_res as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_dump(
            &raw mut (*image_array.offset(img as isize)).num_pages as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_dump(
            &raw mut (*image_array.offset(img as isize)).colorspace_ref as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_dump(
            &raw mut (*image_array.offset(img as isize)).group_ref as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        if (*image_array.offset(img as isize)).image_type == IMAGE_TYPE_PDF {
            do_dump(
                &raw mut (*(*image_array.offset(img as isize)).image_struct.pdf).page_box
                    as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                fmtfile as gzFile,
            );
            do_dump(
                &raw mut (*(*image_array.offset(img as isize)).image_struct.pdf).selected_page
                    as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                fmtfile as gzFile,
            );
        } else if (*image_array.offset(img as isize)).image_type == IMAGE_TYPE_JBIG2 {
            do_dump(
                &raw mut (*(*image_array.offset(img as isize)).image_struct.jbig2).selected_page
                    as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                fmtfile as gzFile,
            );
        }
        img += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn undumpimagemeta(
    mut pdfmajorversion: integer,
    mut pdfminorversion: integer,
    mut pdfinclusionerrorlevel: integer,
) {
    let mut cur_image: ::core::ffi::c_int = 0;
    let mut img: ::core::ffi::c_int = 0;
    do_undump(
        &raw mut image_limit as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    image_array = xmalloc(
        (image_limit as size_t).wrapping_mul(::core::mem::size_of::<image_entry>() as size_t),
    ) as *mut image_entry;
    do_undump(
        &raw mut cur_image as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    image_ptr = image_array.offset(cur_image as isize);
    img = 0 as ::core::ffi::c_int;
    while img < cur_image {
        let mut x: integer = 0;
        let mut a: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        do_undump(
            &raw mut x as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        if x > 0 as ::core::ffi::c_int {
            a = xmalloc(x as size_t) as *mut ::core::ffi::c_char;
            do_undump(
                a,
                ::core::mem::size_of::<::core::ffi::c_char>() as ::core::ffi::c_int,
                x,
                fmtfile as gzFile,
            );
            let ref mut fresh7 = (*image_array.offset(img as isize)).image_name;
            *fresh7 = a;
        } else {
            let ref mut fresh8 = (*image_array.offset(img as isize)).image_name;
            *fresh8 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        do_undump(
            &raw mut (*image_array.offset(img as isize)).image_type as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_undump(
            &raw mut (*image_array.offset(img as isize)).color_type as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_undump(
            &raw mut (*image_array.offset(img as isize)).width as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_undump(
            &raw mut (*image_array.offset(img as isize)).height as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_undump(
            &raw mut (*image_array.offset(img as isize)).x_res as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_undump(
            &raw mut (*image_array.offset(img as isize)).y_res as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_undump(
            &raw mut (*image_array.offset(img as isize)).num_pages as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_undump(
            &raw mut (*image_array.offset(img as isize)).colorspace_ref as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_undump(
            &raw mut (*image_array.offset(img as isize)).group_ref as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        if kpse_find_file(
            (*image_array.offset(img as isize)).image_name as const_string,
            kpse_tex_format,
            true_0,
        )
        .is_null()
        {
            pdftex_fail(
                b"cannot find image file %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*image_array.offset(img as isize)).image_name,
            );
        }
        match (*image_array.offset(img as isize)).image_type {
            IMAGE_TYPE_PDF => {
                let ref mut fresh9 = (*image_array.offset(img as isize)).image_struct.pdf;
                *fresh9 = xmalloc(
                    (1 as size_t)
                        .wrapping_mul(::core::mem::size_of::<pdf_image_struct>() as size_t),
                ) as *mut pdf_image_struct;
                do_undump(
                    &raw mut (*(*image_array.offset(img as isize)).image_struct.pdf).page_box
                        as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    fmtfile as gzFile,
                );
                do_undump(
                    &raw mut (*(*image_array.offset(img as isize)).image_struct.pdf).selected_page
                        as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    fmtfile as gzFile,
                );
                read_pdf_info(
                    (*image_array.offset(img as isize)).image_name,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    (*(*image_array.offset(img as isize)).image_struct.pdf).selected_page
                        as ::core::ffi::c_int,
                    (*(*image_array.offset(img as isize)).image_struct.pdf).page_box
                        as ::core::ffi::c_int,
                    pdfmajorversion as ::core::ffi::c_int,
                    pdfminorversion as ::core::ffi::c_int,
                    pdfinclusionerrorlevel as ::core::ffi::c_int,
                );
                (*image_array.offset(img as isize)).width = zround(
                    epdf_width as ::core::ffi::c_double
                        * (onehundredbp as ::core::ffi::c_double / 100.0f64),
                );
                (*image_array.offset(img as isize)).height = zround(
                    epdf_height as ::core::ffi::c_double
                        * (onehundredbp as ::core::ffi::c_double / 100.0f64),
                );
                (*image_array.offset(img as isize)).num_pages = epdf_num_pages as integer;
                (*(*image_array.offset(img as isize)).image_struct.pdf).orig_x = zround(
                    epdf_orig_x as ::core::ffi::c_double
                        * (onehundredbp as ::core::ffi::c_double / 100.0f64),
                );
                (*(*image_array.offset(img as isize)).image_struct.pdf).orig_y = zround(
                    epdf_orig_y as ::core::ffi::c_double
                        * (onehundredbp as ::core::ffi::c_double / 100.0f64),
                );
                let ref mut fresh10 = (*(*image_array.offset(img as isize)).image_struct.pdf).doc;
                *fresh10 = epdf_doc;
            }
            IMAGE_TYPE_PNG => {
                (*image_array.offset(img as isize)).num_pages = 1 as ::core::ffi::c_int as integer;
                read_png_info(img as integer);
            }
            IMAGE_TYPE_JPG => {
                let ref mut fresh11 = (*image_array.offset(img as isize)).image_struct.jpg;
                *fresh11 = xmalloc(
                    (1 as size_t).wrapping_mul(::core::mem::size_of::<JPG_IMAGE_INFO>() as size_t),
                ) as *mut JPG_IMAGE_INFO;
                (*image_array.offset(img as isize)).num_pages = 1 as ::core::ffi::c_int as integer;
                read_jpg_info(img as integer);
            }
            IMAGE_TYPE_JBIG2 => {
                if pdfmajorversion == 1 as ::core::ffi::c_int
                    && pdfminorversion < 4 as ::core::ffi::c_int
                {
                    pdftex_fail(
                        b"JBIG2 images only possible with at least PDF 1.4; you are generating PDF 1.%i\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        pdfminorversion,
                    );
                }
                let ref mut fresh12 = (*image_array.offset(img as isize)).image_struct.jbig2;
                *fresh12 = xmalloc(
                    (1 as size_t)
                        .wrapping_mul(::core::mem::size_of::<JBIG2_IMAGE_INFO>() as size_t),
                ) as *mut JBIG2_IMAGE_INFO;
                (*image_array.offset(img as isize)).image_type = IMAGE_TYPE_JBIG2;
                do_undump(
                    &raw mut (*(*image_array.offset(img as isize)).image_struct.jbig2).selected_page
                        as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    fmtfile as gzFile,
                );
                read_jbig2_info(img as integer);
            }
            _ => {
                pdftex_fail(b"unknown type of image\0" as *const u8 as *const ::core::ffi::c_char);
            }
        }
        img += 1;
    }
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
