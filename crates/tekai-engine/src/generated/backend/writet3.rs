#[repr(C)]
pub struct __sFILEX {
    _unused: [u8; 0],
}

extern "C" {
    fn feof(_: *mut FILE) -> ::core::ffi::c_int;
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn free(_: *mut ::core::ffi::c_void);
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn xfopen(filename: const_string, mode: const_string) -> *mut FILE;
    fn xfclose(fp: *mut FILE, filename: const_string);
    fn xmalloc(size: size_t) -> address;
    fn xrealloc(old_address: address, new_size: size_t) -> address;
    fn open_input(_: *mut *mut FILE, _: ::core::ffi::c_int, fopen_mode: const_string) -> boolean;
    fn recorder_record_input(_: const_string);
    fn zround(_: ::core::ffi::c_double) -> integer;
    fn makecstring(s: integer) -> *mut ::core::ffi::c_char;
    static mut nameoffile: *mut ASCIIcode;
    static mut fontdsize: *mut scaled;
    static mut fontname: *mut strnumber;
    static mut fontbc: *mut eightbits;
    static mut fontec: *mut eightbits;
    static mut pdfbuf: *mut eightbits;
    static mut pdfbufsize: integer;
    static mut pdfptr: integer;
    static mut pdfosmode: boolean;
    static mut onehundredbp: scaled;
    static mut fixedpkresolution: integer;
    static mut fixeddecimaldigits: integer;
    static mut fixedgentounicode: integer;
    static mut pkscalefactor: integer;
    static mut objptr: integer;
    static mut pdffontattr: *mut strnumber;
    static mut pdffontnobuiltintounicode: *mut boolean;
    static mut pdfcharused: *mut charusedarray;
    static mut pdffontsize: *mut scaled;
    fn zpdfosgetosbuf(s: integer);
    fn zpdfprint(s: strnumber);
    fn zdividescaled(s: scaled, m: scaled, dd: integer) -> scaled;
    fn pdfnewobjnum() -> integer;
    fn zpdfbeginobj(i: integer, pdfoslevel: integer);
    fn pdfendobj();
    fn zpdfbegindict(i: integer, pdfoslevel: integer);
    fn zpdfnewdict(t: integer, i: integer, pdfos: integer);
    fn pdfenddict();
    fn zpackfilename(n: strnumber, a: strnumber, e: strnumber);
    fn makenamestring() -> strnumber;
    fn zgetcharwidth(f: internalfontnumber, c: eightbits) -> scaled;
    fn pdfflush();
    fn pdfbeginstream();
    fn pdfendstream();
    fn zpdfprintreal(m: integer, d: integer);
    fn getnullstr() -> strnumber;
    fn avl_create(
        _: Option<avl_comparison_func>,
        _: *mut ::core::ffi::c_void,
        _: *mut libavl_allocator,
    ) -> *mut avl_table;
    fn avl_destroy(_: *mut avl_table, _: Option<avl_item_func>);
    fn avl_probe(_: *mut avl_table, _: *mut ::core::ffi::c_void) -> *mut *mut ::core::ffi::c_void;
    fn avl_find(_: *const avl_table, _: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    static mut avl_xallocator: libavl_allocator;
    static mut cur_file_name: *mut ::core::ffi::c_char;
    static mut last_ptr_index: size_t;
    static mut notdef: [::core::ffi::c_char; 0];
    fn pdf_printf(_: *const ::core::ffi::c_char, ...);
    fn pdf_puts(_: *const ::core::ffi::c_char);
    fn pdftex_fail(_: *const ::core::ffi::c_char, ...) -> !;
    fn pdftex_warn(_: *const ::core::ffi::c_char, ...);
    fn tex_printf(_: *const ::core::ffi::c_char, ...);
    fn readchar(_: boolean, _: *mut chardesc) -> ::core::ffi::c_int;
    fn write_tounicode(
        _: *mut *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> integer;
    fn xgetc(_: *mut FILE) -> ::core::ffi::c_int;
    fn maketexstring(_: *const ::core::ffi::c_char) -> strnumber;
    fn get_fe_entry(_: *mut ::core::ffi::c_char) -> *mut fe_entry;
    fn comp_string_entry(
        _: *const ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn kpse_find_glyph(
        font_name: const_string,
        dpi: ::core::ffi::c_uint,
        format: kpse_file_format_type,
        glyph_file: *mut kpse_glyph_file_type,
    ) -> string;
    fn kpse_bitmap_tolerance(dpi1: ::core::ffi::c_double, dpi2: ::core::ffi::c_double) -> boolean;
    fn kpse_magstep_fix(
        dpi: ::core::ffi::c_uint,
        bdpi: ::core::ffi::c_uint,
        m_ret: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_uint;
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
pub type ASCIIcode = ::core::ffi::c_uchar;
pub type eightbits = ::core::ffi::c_uchar;
pub type strnumber = integer;
pub type scaled = integer;
pub type halfword = integer;
pub type internalfontnumber = integer;
pub type charusedarray = [eightbits; 32];
pub type avl_comparison_func = unsafe extern "C" fn(
    *const ::core::ffi::c_void,
    *const ::core::ffi::c_void,
    *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int;
pub type avl_item_func =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libavl_allocator {
    pub libavl_malloc:
        Option<unsafe extern "C" fn(*mut libavl_allocator, size_t) -> *mut ::core::ffi::c_void>,
    pub libavl_free:
        Option<unsafe extern "C" fn(*mut libavl_allocator, *mut ::core::ffi::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avl_table {
    pub avl_root: *mut avl_node,
    pub avl_compare: Option<avl_comparison_func>,
    pub avl_param: *mut ::core::ffi::c_void,
    pub avl_alloc: *mut libavl_allocator,
    pub avl_count: size_t,
    pub avl_generation: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avl_node {
    pub avl_link: [*mut avl_node; 2],
    pub avl_data: *mut ::core::ffi::c_void,
    pub avl_balance: ::core::ffi::c_schar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_entry {
    pub pdfname: *const ::core::ffi::c_char,
    pub t1name: *const ::core::ffi::c_char,
    pub value: ::core::ffi::c_float,
    pub valid: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _subfont_entry {
    pub infix: *mut ::core::ffi::c_char,
    pub charcodes: [::core::ffi::c_long; 256],
    pub next: *mut subfont_entry,
}
pub type subfont_entry = _subfont_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fe_entry {
    pub fe_objnum: integer,
    pub name: *mut ::core::ffi::c_char,
    pub glyph_names: *mut *mut ::core::ffi::c_char,
    pub tx_tree: *mut avl_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fm_entry {
    pub tfm_name: *mut ::core::ffi::c_char,
    pub sfd_name: *mut ::core::ffi::c_char,
    pub ps_name: *mut ::core::ffi::c_char,
    pub fd_flags: integer,
    pub slant: integer,
    pub extend: integer,
    pub encname: *mut ::core::ffi::c_char,
    pub ff_name: *mut ::core::ffi::c_char,
    pub type_0: ::core::ffi::c_ushort,
    pub pid: ::core::ffi::c_short,
    pub eid: ::core::ffi::c_short,
    pub subfont: *mut subfont_entry,
    pub links: ::core::ffi::c_ushort,
    pub in_use: boolean,
}
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
pub type t3_line_entry = ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kpse_glyph_file_type {
    pub name: const_string,
    pub dpi: ::core::ffi::c_uint,
    pub format: kpse_file_format_type,
    pub source: kpse_glyph_source_type,
}
pub type kpse_glyph_source_type = ::core::ffi::c_uint;
pub const kpse_glyph_source_fallback: kpse_glyph_source_type = 4;
pub const kpse_glyph_source_fallback_res: kpse_glyph_source_type = 3;
pub const kpse_glyph_source_maketex: kpse_glyph_source_type = 2;
pub const kpse_glyph_source_alias: kpse_glyph_source_type = 1;
pub const kpse_glyph_source_normal: kpse_glyph_source_type = 0;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const FOPEN_RBIN_MODE: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"rb\0") };
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FONTBBOX1_CODE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
static mut font_key: [key_entry; 11] = [
    key_entry {
        pdfname: b"Ascent\0" as *const u8 as *const ::core::ffi::c_char,
        t1name: b"Ascender\0" as *const u8 as *const ::core::ffi::c_char,
        value: 0.,
        valid: 0,
    },
    key_entry {
        pdfname: b"CapHeight\0" as *const u8 as *const ::core::ffi::c_char,
        t1name: b"CapHeight\0" as *const u8 as *const ::core::ffi::c_char,
        value: 0.,
        valid: 0,
    },
    key_entry {
        pdfname: b"Descent\0" as *const u8 as *const ::core::ffi::c_char,
        t1name: b"Descender\0" as *const u8 as *const ::core::ffi::c_char,
        value: 0.,
        valid: 0,
    },
    key_entry {
        pdfname: b"ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char,
        t1name: b"ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char,
        value: 0.,
        valid: 0,
    },
    key_entry {
        pdfname: b"StemV\0" as *const u8 as *const ::core::ffi::c_char,
        t1name: b"StdVW\0" as *const u8 as *const ::core::ffi::c_char,
        value: 0.,
        valid: 0,
    },
    key_entry {
        pdfname: b"XHeight\0" as *const u8 as *const ::core::ffi::c_char,
        t1name: b"XHeight\0" as *const u8 as *const ::core::ffi::c_char,
        value: 0.,
        valid: 0,
    },
    key_entry {
        pdfname: b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char,
        t1name: b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char,
        value: 0.,
        valid: 0,
    },
    key_entry {
        pdfname: b"\0" as *const u8 as *const ::core::ffi::c_char,
        t1name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        value: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
        valid: 0,
    },
    key_entry {
        pdfname: b"\0" as *const u8 as *const ::core::ffi::c_char,
        t1name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        value: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
        valid: 0,
    },
    key_entry {
        pdfname: b"\0" as *const u8 as *const ::core::ffi::c_char,
        t1name: b"\0" as *const u8 as *const ::core::ffi::c_char,
        value: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
        valid: 0,
    },
    key_entry {
        pdfname: b"FontName\0" as *const u8 as *const ::core::ffi::c_char,
        t1name: b"FontName\0" as *const u8 as *const ::core::ffi::c_char,
        value: 0.,
        valid: 0,
    },
];
#[no_mangle]
pub static mut t3_line_ptr: *mut t3_line_entry =
    ::core::ptr::null::<t3_line_entry>() as *mut t3_line_entry;
#[no_mangle]
pub static mut t3_line_array: *mut t3_line_entry =
    ::core::ptr::null::<t3_line_entry>() as *mut t3_line_entry;
#[no_mangle]
pub static mut t3_line_limit: size_t = 0;
#[no_mangle]
pub static mut t3_file: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
static mut t3_image_used: boolean = 0;
static mut t3_char_procs: [integer; 256] = [0; 256];
static mut t3_char_widths: [::core::ffi::c_float; 256] = [0.; 256];
static mut t3_glyph_num: ::core::ffi::c_int = 0;
static mut t3_font_scale: ::core::ffi::c_float = 0.;
static mut t3_b3: integer = 0;
static mut t3_b2: integer = 0;
static mut t3_b0: integer = 0;
static mut t3_b1: integer = 0;
static mut is_pk_font: boolean = 0;
unsafe extern "C" fn t3_getline() {
    let mut c: ::core::ffi::c_int = 0;
    loop {
        t3_line_ptr = t3_line_array;
        c = xgetc(t3_file);
        while feof(t3_file) == 0 {
            if t3_line_array.is_null() {
                t3_line_limit = 1024 as size_t;
                if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                    > t3_line_limit as ::core::ffi::c_uint
                {
                    t3_line_limit = 1 as size_t;
                }
                t3_line_array = xmalloc(
                    t3_line_limit.wrapping_mul(::core::mem::size_of::<t3_line_entry>() as size_t),
                ) as *mut t3_line_entry;
                t3_line_ptr = t3_line_array;
            } else if (t3_line_ptr.offset_from(t3_line_array) as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                > t3_line_limit as ::core::ffi::c_uint
            {
                last_ptr_index =
                    t3_line_ptr.offset_from(t3_line_array) as ::core::ffi::c_long as size_t;
                t3_line_limit = t3_line_limit.wrapping_mul(2 as size_t);
                if (t3_line_ptr.offset_from(t3_line_array) as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                    > t3_line_limit as ::core::ffi::c_uint
                {
                    t3_line_limit = (t3_line_ptr.offset_from(t3_line_array) as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as size_t;
                }
                if t3_line_limit as ::core::ffi::c_uint > INT_MAX as ::core::ffi::c_uint {
                    crate::utils::pdftex_fail_args(
                        b"t3_line_array exceeds size limit\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                }
                t3_line_array = xrealloc(
                    t3_line_array as address,
                    t3_line_limit.wrapping_mul(::core::mem::size_of::<t3_line_entry>() as size_t),
                ) as *mut t3_line_entry;
                t3_line_ptr = t3_line_array.offset(last_ptr_index as isize);
            }
            if c == 9 as ::core::ffi::c_int {
                c = 32 as ::core::ffi::c_int;
            }
            if c == 13 as ::core::ffi::c_int || c == EOF {
                c = 10 as ::core::ffi::c_int;
            }
            if c != ' ' as i32
                || t3_line_ptr > t3_line_array
                    && *t3_line_ptr.offset(-(1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        != 32 as ::core::ffi::c_int
            {
                if (t3_line_ptr.offset_from(t3_line_array) as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                    > t3_line_limit as ::core::ffi::c_uint
                {
                    crate::utils::pdftex_fail_args(
                        b"buffer overflow at file %s, line %d\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[
                            crate::utils::PrintfArg::from(
                                b"tekai-engine/generated/backend/writet3.rs\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            ),
                            crate::utils::PrintfArg::from(60 as ::core::ffi::c_int),
                        ],
                    );
                }
                let fresh0 = t3_line_ptr;
                t3_line_ptr = t3_line_ptr.offset(1);
                *fresh0 = c as t3_line_entry;
            }
            if c == 10 as ::core::ffi::c_int {
                break;
            }
            c = xgetc(t3_file);
        }
        if t3_line_array.is_null() {
            t3_line_limit = 1024 as size_t;
            if 2 as ::core::ffi::c_int as ::core::ffi::c_uint > t3_line_limit as ::core::ffi::c_uint
            {
                t3_line_limit = 2 as size_t;
            }
            t3_line_array = xmalloc(
                t3_line_limit.wrapping_mul(::core::mem::size_of::<t3_line_entry>() as size_t),
            ) as *mut t3_line_entry;
            t3_line_ptr = t3_line_array;
        } else if (t3_line_ptr.offset_from(t3_line_array) as ::core::ffi::c_long
            + 2 as ::core::ffi::c_long) as ::core::ffi::c_uint
            > t3_line_limit as ::core::ffi::c_uint
        {
            last_ptr_index =
                t3_line_ptr.offset_from(t3_line_array) as ::core::ffi::c_long as size_t;
            t3_line_limit = t3_line_limit.wrapping_mul(2 as size_t);
            if (t3_line_ptr.offset_from(t3_line_array) as ::core::ffi::c_long
                + 2 as ::core::ffi::c_long) as ::core::ffi::c_uint
                > t3_line_limit as ::core::ffi::c_uint
            {
                t3_line_limit = (t3_line_ptr.offset_from(t3_line_array) as ::core::ffi::c_long
                    + 2 as ::core::ffi::c_long) as size_t;
            }
            if t3_line_limit as ::core::ffi::c_uint > INT_MAX as ::core::ffi::c_uint {
                crate::utils::pdftex_fail_args(
                    b"t3_line_array exceeds size limit\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                );
            }
            t3_line_array = xrealloc(
                t3_line_array as address,
                t3_line_limit.wrapping_mul(::core::mem::size_of::<t3_line_entry>() as size_t),
            ) as *mut t3_line_entry;
            t3_line_ptr = t3_line_array.offset(last_ptr_index as isize);
        }
        if (t3_line_ptr.offset_from(t3_line_array) as ::core::ffi::c_long
            + 2 as ::core::ffi::c_long) as ::core::ffi::c_uint
            > 1024 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::utils::pdftex_fail_args(
                b"buffer overflow at file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(
                        b"tekai-engine/generated/backend/writet3.rs\0" as *const u8
                            as *const ::core::ffi::c_char,
                    ),
                    crate::utils::PrintfArg::from(66 as ::core::ffi::c_int),
                ],
            );
        }
        if t3_line_ptr.offset_from(t3_line_array) as ::core::ffi::c_long > 1 as ::core::ffi::c_long
            && *t3_line_ptr.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                != 10 as ::core::ffi::c_int
        {
            let fresh1 = t3_line_ptr;
            t3_line_ptr = t3_line_ptr.offset(1);
            *fresh1 = 10 as t3_line_entry;
        }
        if t3_line_ptr.offset_from(t3_line_array) as ::core::ffi::c_long > 2 as ::core::ffi::c_long
            && *t3_line_ptr.offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == 32 as ::core::ffi::c_int
        {
            *t3_line_ptr.offset(-(2 as ::core::ffi::c_int) as isize) = 10 as t3_line_entry;
            t3_line_ptr = t3_line_ptr.offset(-1);
        }
        *t3_line_ptr = 0 as t3_line_entry;
        if !((t3_line_ptr.offset_from(t3_line_array) as ::core::ffi::c_long)
            < 2 as ::core::ffi::c_long
            || *t3_line_array as ::core::ffi::c_int == '%' as i32)
        {
            break;
        }
        if !(feof(t3_file) == 0) {
            break;
        }
    }
}
unsafe extern "C" fn t3_putline() {
    let mut p: *mut ::core::ffi::c_char = t3_line_array as *mut ::core::ffi::c_char;
    while p < t3_line_ptr {
        if (1 as integer + pdfptr) as ::core::ffi::c_uint > pdfbufsize as ::core::ffi::c_uint {
            if pdfosmode != 0 {
                zpdfosgetosbuf(1 as ::core::ffi::c_int);
            } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                > pdfbufsize as ::core::ffi::c_uint
            {
                crate::utils::pdftex_fail_args(
                    b"PDF output buffer overflowed\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            } else {
                pdfflush();
            }
        }
        let fresh2 = p;
        p = p.offset(1);
        let fresh3 = pdfptr;
        pdfptr = pdfptr + 1;
        *pdfbuf.offset(fresh3 as isize) = *fresh2 as eightbits;
    }
}
unsafe extern "C" fn update_bbox(
    mut llx: integer,
    mut lly: integer,
    mut urx: integer,
    mut ury: integer,
    mut is_first_glyph: boolean,
) {
    if is_first_glyph != 0 {
        t3_b0 = llx;
        t3_b1 = lly;
        t3_b2 = urx;
        t3_b3 = ury;
    } else {
        if llx < t3_b0 {
            t3_b0 = llx;
        }
        if lly < t3_b1 {
            t3_b1 = lly;
        }
        if urx > t3_b2 {
            t3_b2 = urx;
        }
        if ury > t3_b3 {
            t3_b3 = ury;
        }
    };
}
unsafe extern "C" fn t3_write_glyph(mut f: internalfontnumber) {
    static mut t3_begin_glyph_str: [::core::ffi::c_char; 10] =
        unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"\\pdfglyph\0") };
    static mut t3_end_glyph_str: [::core::ffi::c_char; 10] =
        unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"\\endglyph\0") };
    let mut glyph_index: ::core::ffi::c_int = 0;
    let mut width: ::core::ffi::c_int = 0;
    let mut height: ::core::ffi::c_int = 0;
    let mut depth: ::core::ffi::c_int = 0;
    let mut llx: ::core::ffi::c_int = 0;
    let mut lly: ::core::ffi::c_int = 0;
    let mut urx: ::core::ffi::c_int = 0;
    let mut ury: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    t3_getline();
    if strncmp(
        t3_line_array,
        &raw mut t3_begin_glyph_str as *mut ::core::ffi::c_char,
        strlen(&raw mut t3_begin_glyph_str as *mut ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int
    {
        if sscanf(
            t3_line_array
                .offset(strlen(&raw mut t3_begin_glyph_str as *mut ::core::ffi::c_char) as isize)
                .offset(1 as ::core::ffi::c_int as isize),
            b"%i %i %i %i %i %i %i %i =\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut glyph_index,
            &raw mut width,
            &raw mut height,
            &raw mut depth,
            &raw mut llx,
            &raw mut lly,
            &raw mut urx,
            &raw mut ury,
        ) != 8 as ::core::ffi::c_int
        {
            p = strchr(t3_line_array, 0 as ::core::ffi::c_int)
                .offset(-(1 as ::core::ffi::c_int as isize));
            if *p as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
                *p = 0 as ::core::ffi::c_char;
            }
            crate::utils::pdftex_fail_args(
                b"invalid glyph preamble: `%s'\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(t3_line_array)],
            );
        }
        if glyph_index < *fontbc.offset(f as isize) as ::core::ffi::c_int
            || glyph_index > *fontec.offset(f as isize) as ::core::ffi::c_int
        {
            return;
        }
    } else {
        return;
    }
    if (*pdfcharused.offset(f as isize))[(glyph_index / 8 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << glyph_index % 8 as ::core::ffi::c_int
        == 0
    {
        while !(strncmp(
            t3_line_array,
            &raw mut t3_end_glyph_str as *mut ::core::ffi::c_char,
            strlen(&raw mut t3_end_glyph_str as *mut ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int)
        {
            if feof(t3_file) != 0 {
                crate::utils::pdftex_fail_args(
                    b"unexpected end of file\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            t3_getline();
        }
        return;
    }
    update_bbox(
        llx as integer,
        lly as integer,
        urx as integer,
        ury as integer,
        (t3_glyph_num == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
    t3_glyph_num += 1;
    zpdfnewdict(
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    t3_char_procs[glyph_index as usize] = objptr;
    if width == 0 as ::core::ffi::c_int {
        t3_char_widths[glyph_index as usize] = zgetcharwidth(f, glyph_index as eightbits)
            as ::core::ffi::c_float
            / t3_font_scale
            / *pdffontsize.offset(f as isize) as ::core::ffi::c_float;
    } else {
        t3_char_widths[glyph_index as usize] = width as ::core::ffi::c_float;
    }
    pdfbeginstream();
    t3_getline();
    crate::utils::pdf_printf_args(
        b"%i 0 %i %i %i %i d1\nq\n\0" as *const u8 as *const ::core::ffi::c_char,
        &[
            crate::utils::PrintfArg::from(
                t3_char_widths[glyph_index as usize] as ::core::ffi::c_int,
            ),
            crate::utils::PrintfArg::from(llx),
            crate::utils::PrintfArg::from(lly),
            crate::utils::PrintfArg::from(urx),
            crate::utils::PrintfArg::from(ury),
        ],
    );
    while !(strncmp(
        t3_line_array,
        &raw mut t3_end_glyph_str as *mut ::core::ffi::c_char,
        strlen(&raw mut t3_end_glyph_str as *mut ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int)
    {
        if feof(t3_file) != 0 {
            crate::utils::pdftex_fail_args(
                b"unexpected end of file\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        if strncmp(
            t3_line_array,
            b"BI\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"BI\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int
        {
            t3_image_used = true_0 as boolean;
        }
        t3_putline();
        t3_getline();
    }
    pdf_puts(b"Q\n\0" as *const u8 as *const ::core::ffi::c_char);
    pdfendstream();
}
unsafe extern "C" fn get_pk_font_scale(mut f: internalfontnumber) -> integer {
    return zdividescaled(
        pkscalefactor,
        zdividescaled(
            *pdffontsize.offset(f as isize),
            onehundredbp,
            fixeddecimaldigits as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
        ),
        0 as ::core::ffi::c_int,
    ) as integer;
}
unsafe extern "C" fn pk_char_width(mut f: internalfontnumber, mut w: scaled) -> integer {
    return zdividescaled(
        zdividescaled(w, *pdffontsize.offset(f as isize), 7 as ::core::ffi::c_int),
        get_pk_font_scale(f),
        0 as ::core::ffi::c_int,
    ) as integer;
}
#[no_mangle]
pub unsafe extern "C" fn getpkcharwidth(mut f: internalfontnumber, mut w: scaled) -> scaled {
    return (get_pk_font_scale(f) as ::core::ffi::c_double / 100000.0f64
        * (pk_char_width(f, w) as ::core::ffi::c_double / 100.0f64)
        * *pdffontsize.offset(f as isize) as ::core::ffi::c_double) as scaled;
}
unsafe extern "C" fn writepk(mut f: internalfontnumber) -> boolean {
    let mut font_ret: kpse_glyph_file_type = kpse_glyph_file_type {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        dpi: 0,
        format: kpse_gf_format,
        source: kpse_glyph_source_normal,
    };
    let mut llx: integer = 0;
    let mut lly: integer = 0;
    let mut urx: integer = 0;
    let mut ury: integer = 0;
    let mut cw: integer = 0;
    let mut rw: integer = 0;
    let mut i: integer = 0;
    let mut j: integer = 0;
    let mut row: *mut halfword = ::core::ptr::null_mut::<halfword>();
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cd: chardesc = chardesc {
        charcode: 0,
        cwidth: 0,
        cheight: 0,
        xoff: 0,
        yoff: 0,
        xescape: 0,
        rastersize: 0,
        raster: ::core::ptr::null_mut::<halfword>(),
    };
    let mut is_null_glyph: boolean = 0;
    let mut check_preamble: boolean = 0;
    let mut dpi: integer = 0;
    dpi = kpse_magstep_fix(
        zround(
            (fixedpkresolution as ::core::ffi::c_float
                * (*pdffontsize.offset(f as isize) as ::core::ffi::c_float
                    / *fontdsize.offset(f as isize) as ::core::ffi::c_float))
                as ::core::ffi::c_double,
        ) as ::core::ffi::c_uint,
        fixedpkresolution as ::core::ffi::c_uint,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    ) as integer;
    cur_file_name = makecstring(*fontname.offset(f as isize) as integer);
    name = kpse_find_glyph(
        cur_file_name as const_string,
        dpi as ::core::ffi::c_uint,
        kpse_pk_format,
        &raw mut font_ret,
    ) as *mut ::core::ffi::c_char;
    if name.is_null()
        || !(!cur_file_name.is_null()
            && !font_ret.name.is_null()
            && strcmp(cur_file_name, font_ret.name as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int)
        || kpse_bitmap_tolerance(
            font_ret.dpi as ::core::ffi::c_float as ::core::ffi::c_double,
            dpi as ::core::ffi::c_float as ::core::ffi::c_double,
        ) == 0
    {
        crate::utils::pdftex_fail_args(
            b"Font %s at %i not found\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(cur_file_name),
                crate::utils::PrintfArg::from(dpi),
            ],
        );
    }
    t3_file = xfopen(name as const_string, FOPEN_RBIN_MODE.as_ptr());
    recorder_record_input(name as const_string);
    t3_image_used = true_0 as boolean;
    is_pk_font = true_0 as boolean;
    crate::utils::tex_printf_args(
        b" <%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(name)],
    );
    cd.rastersize = 256 as ::core::ffi::c_int as integer;
    cd.raster = xmalloc(
        (cd.rastersize as size_t).wrapping_mul(::core::mem::size_of::<halfword>() as size_t),
    ) as *mut halfword;
    check_preamble = true_0 as boolean;
    while readchar(check_preamble, &raw mut cd) != 0 as ::core::ffi::c_int {
        check_preamble = false_0 as boolean;
        if (*pdfcharused.offset(f as isize))
            [(cd.charcode as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << cd.charcode as ::core::ffi::c_int % 8 as ::core::ffi::c_int
            == 0
        {
            continue;
        }
        t3_char_widths[cd.charcode as usize] =
            pk_char_width(f, zgetcharwidth(f, cd.charcode as eightbits)) as ::core::ffi::c_float;
        if cd.cwidth < 1 as ::core::ffi::c_int || cd.cheight < 1 as ::core::ffi::c_int {
            cd.cwidth =
                zround(t3_char_widths[cd.charcode as usize] as ::core::ffi::c_double / 100.0f64);
            cd.xescape = cd.cwidth;
            cd.cheight = 1 as ::core::ffi::c_int as integer;
            cd.xoff = 0 as ::core::ffi::c_int as integer;
            cd.yoff = 0 as ::core::ffi::c_int as integer;
            is_null_glyph = true_0 as boolean;
        } else {
            is_null_glyph = false_0 as boolean;
        }
        llx = -cd.xoff;
        lly = (cd.yoff as ::core::ffi::c_int - cd.cheight as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as integer;
        urx = (cd.cwidth as ::core::ffi::c_int
            + llx as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as integer;
        ury = cd.cheight + lly;
        update_bbox(
            llx,
            lly,
            urx,
            ury,
            (t3_glyph_num == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
        t3_glyph_num += 1;
        zpdfnewdict(
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        t3_char_procs[cd.charcode as usize] = objptr;
        pdfbeginstream();
        zpdfprintreal(
            t3_char_widths[cd.charcode as usize] as integer,
            2 as ::core::ffi::c_int,
        );
        crate::utils::pdf_printf_args(
            b" 0 %i %i %i %i d1\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(llx),
                crate::utils::PrintfArg::from(lly),
                crate::utils::PrintfArg::from(urx),
                crate::utils::PrintfArg::from(ury),
            ],
        );
        if !(is_null_glyph != 0) {
            crate::utils::pdf_printf_args(
                b"q\n%i 0 0 %i %i %i cm\nBI\n\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(cd.cwidth),
                    crate::utils::PrintfArg::from(cd.cheight),
                    crate::utils::PrintfArg::from(llx),
                    crate::utils::PrintfArg::from(lly),
                ],
            );
            crate::utils::pdf_printf_args(
                b"/W %i\n/H %i\n\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(cd.cwidth),
                    crate::utils::PrintfArg::from(cd.cheight),
                ],
            );
            pdf_puts(
                b"/IM true\n/BPC 1\n/D [1 0]\nID \0" as *const u8 as *const ::core::ffi::c_char,
            );
            cw = ((cd.cwidth as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
                / 8 as ::core::ffi::c_int) as integer;
            rw = ((cd.cwidth as ::core::ffi::c_int + 15 as ::core::ffi::c_int)
                / 16 as ::core::ffi::c_int) as integer;
            row = cd.raster;
            i = 0 as ::core::ffi::c_int as integer;
            while i < cd.cheight {
                j = 0 as ::core::ffi::c_int as integer;
                while j < rw as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
                    if (1 as integer + pdfptr) as ::core::ffi::c_uint
                        > pdfbufsize as ::core::ffi::c_uint
                    {
                        if pdfosmode != 0 {
                            zpdfosgetosbuf(1 as ::core::ffi::c_int);
                        } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
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
                    let fresh4 = pdfptr;
                    pdfptr = pdfptr + 1;
                    *pdfbuf.offset(fresh4 as isize) =
                        (*row / 256 as ::core::ffi::c_int) as eightbits;
                    if (1 as integer + pdfptr) as ::core::ffi::c_uint
                        > pdfbufsize as ::core::ffi::c_uint
                    {
                        if pdfosmode != 0 {
                            zpdfosgetosbuf(1 as ::core::ffi::c_int);
                        } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
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
                    let fresh5 = pdfptr;
                    pdfptr = pdfptr + 1;
                    *pdfbuf.offset(fresh5 as isize) =
                        (*row % 256 as ::core::ffi::c_int) as eightbits;
                    row = row.offset(1);
                    j += 1;
                }
                if (1 as integer + pdfptr) as ::core::ffi::c_uint
                    > pdfbufsize as ::core::ffi::c_uint
                {
                    if pdfosmode != 0 {
                        zpdfosgetosbuf(1 as ::core::ffi::c_int);
                    } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
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
                let fresh6 = pdfptr;
                pdfptr = pdfptr + 1;
                *pdfbuf.offset(fresh6 as isize) = (*row / 256 as ::core::ffi::c_int) as eightbits;
                if 2 as integer * rw == cw {
                    if (1 as integer + pdfptr) as ::core::ffi::c_uint
                        > pdfbufsize as ::core::ffi::c_uint
                    {
                        if pdfosmode != 0 {
                            zpdfosgetosbuf(1 as ::core::ffi::c_int);
                        } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
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
                    let fresh7 = pdfptr;
                    pdfptr = pdfptr + 1;
                    *pdfbuf.offset(fresh7 as isize) =
                        (*row % 256 as ::core::ffi::c_int) as eightbits;
                }
                row = row.offset(1);
                i += 1;
            }
            pdf_puts(b"\nEI\nQ\n\0" as *const u8 as *const ::core::ffi::c_char);
        }
        pdfendstream();
    }
    if !cd.raster.is_null() {
        free(cd.raster as *mut ::core::ffi::c_void);
    }
    cd.raster = ::core::ptr::null_mut::<halfword>();
    cur_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return true_0;
}
unsafe extern "C" fn remove_duplicate_glyph_names(
    mut g: *mut *mut ::core::ffi::c_char,
    mut encname: *const ::core::ffi::c_char,
) {
    let mut gl_tree: *mut avl_table = ::core::ptr::null_mut::<avl_table>();
    let mut aa: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    gl_tree = avl_create(
        Some(
            comp_string_entry
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        NULL,
        &raw mut avl_xallocator,
    );
    if gl_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"remove_duplicate_glyph_names\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet3.c\0" as *const u8 as *const ::core::ffi::c_char,
            266 as ::core::ffi::c_int,
            b"gl_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        if !(*g.offset(i as isize) == &raw mut notdef as *mut ::core::ffi::c_char) {
            aa = avl_find(gl_tree, *g.offset(i as isize) as *const ::core::ffi::c_void)
                as *mut ::core::ffi::c_char;
            if aa.is_null() {
                aa = avl_probe(gl_tree, *g.offset(i as isize) as *mut ::core::ffi::c_void)
                    as *mut ::core::ffi::c_char;
                if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                    __assert_rtn(
                        b"remove_duplicate_glyph_names\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"writet3.c\0" as *const u8 as *const ::core::ffi::c_char,
                        273 as ::core::ffi::c_int,
                        b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                };
            } else {
                crate::utils::pdftex_warn_args(
                    b"%s: duplicate glyph name at position %d: %s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(encname),
                        crate::utils::PrintfArg::from(i),
                        crate::utils::PrintfArg::from(*g.offset(i as isize)),
                    ],
                );
                if !(*g.offset(i as isize)).is_null() {
                    free(*g.offset(i as isize) as *mut ::core::ffi::c_void);
                }
                let ref mut fresh8 = *g.offset(i as isize);
                *fresh8 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                let ref mut fresh9 = *g.offset(i as isize);
                *fresh9 = &raw mut notdef as *mut ::core::ffi::c_char;
            }
        }
        i += 1;
    }
    avl_destroy(gl_tree, None);
}
#[no_mangle]
pub unsafe extern "C" fn writet3(
    mut fm: *mut fm_entry,
    mut objnum: ::core::ffi::c_int,
    mut f: internalfontnumber,
) {
    static mut t3_font_scale_str: [::core::ffi::c_char; 14] = unsafe {
        ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"\\pdffontscale\0")
    };
    let mut i: ::core::ffi::c_int = 0;
    let mut wptr: integer = 0;
    let mut eptr: integer = 0;
    let mut cptr: integer = 0;
    let mut first_char: ::core::ffi::c_int = 0;
    let mut last_char: ::core::ffi::c_int = 0;
    let mut pk_font_scale: integer = 0;
    let mut is_notdef: boolean = 0;
    let mut fe: *mut fe_entry = ::core::ptr::null_mut::<fe_entry>();
    let mut glyph_names: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut tounicode_objnum: integer = 0;
    t3_glyph_num = 0 as ::core::ffi::c_int;
    t3_image_used = false_0 as boolean;
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        t3_char_procs[i as usize] = 0 as ::core::ffi::c_int as integer;
        t3_char_widths[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
        i += 1;
    }
    fe = if !fm.is_null() && !(*fm).encname.is_null() {
        get_fe_entry((*fm).encname)
    } else {
        ::core::ptr::null_mut::<fe_entry>()
    };
    glyph_names = if !fe.is_null() {
        (*fe).glyph_names
    } else {
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>()
    };
    if !glyph_names.is_null() {
        remove_duplicate_glyph_names(glyph_names, (*fm).encname);
    }
    zpackfilename(
        *fontname.offset(f as isize),
        getnullstr(),
        maketexstring(b".pgc\0" as *const u8 as *const ::core::ffi::c_char),
    );
    cur_file_name = makecstring(makenamestring() as integer);
    is_pk_font = false_0 as boolean;
    if open_input(
        &raw mut t3_file,
        kpse_miscfonts_format as ::core::ffi::c_int,
        FOPEN_RBIN_MODE.as_ptr(),
    ) == 0
    {
        if !(writepk(f) != 0) {
            cur_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return;
        }
    } else {
        crate::utils::tex_printf_args(
            b"<%s\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                nameoffile.offset(1 as ::core::ffi::c_int as isize),
            )],
        );
        t3_getline();
        if !(strncmp(
            t3_line_array,
            &raw mut t3_font_scale_str as *mut ::core::ffi::c_char,
            strlen(&raw mut t3_font_scale_str as *mut ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int)
            || sscanf(
                t3_line_array
                    .offset(strlen(&raw mut t3_font_scale_str as *mut ::core::ffi::c_char) as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
                b"%g\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut t3_font_scale,
            ) < 1 as ::core::ffi::c_int
            || t3_font_scale <= 0 as ::core::ffi::c_int as ::core::ffi::c_float
            || t3_font_scale > 1000 as ::core::ffi::c_int as ::core::ffi::c_float
        {
            crate::utils::pdftex_fail_args(
                b"missing or invalid font scale\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        while feof(t3_file) == 0 {
            t3_write_glyph(f);
        }
    }
    i = *fontbc.offset(f as isize) as ::core::ffi::c_int;
    while i <= *fontec.offset(f as isize) as ::core::ffi::c_int {
        if (*pdfcharused.offset(f as isize))[(i / 8 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << i % 8 as ::core::ffi::c_int
            != 0
        {
            break;
        }
        i += 1;
    }
    first_char = i;
    i = *fontec.offset(f as isize) as ::core::ffi::c_int;
    while i > first_char {
        if (*pdfcharused.offset(f as isize))[(i / 8 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << i % 8 as ::core::ffi::c_int
            != 0
        {
            break;
        }
        i -= 1;
    }
    last_char = i;
    if fixedgentounicode > 0 as ::core::ffi::c_int
        && *pdffontnobuiltintounicode.offset(f as isize) == 0
        && !fe.is_null()
    {
        tounicode_objnum = write_tounicode(glyph_names, (*fm).tfm_name, (*fe).name);
    } else {
        tounicode_objnum = 0 as ::core::ffi::c_int as integer;
    }
    zpdfbegindict(objnum, 1 as ::core::ffi::c_int);
    pdf_puts(b"/Type /Font\n/Subtype /Type3\n\0" as *const u8 as *const ::core::ffi::c_char);
    crate::utils::pdf_printf_args(
        b"/Name /F%i\n\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(f)],
    );
    if *pdffontattr.offset(f as isize) != getnullstr() {
        zpdfprint(*pdffontattr.offset(f as isize));
        pdf_puts(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if is_pk_font != 0 {
        pk_font_scale = get_pk_font_scale(f);
        pdf_puts(b"/FontMatrix [\0" as *const u8 as *const ::core::ffi::c_char);
        zpdfprintreal(pk_font_scale, 5 as ::core::ffi::c_int);
        pdf_puts(b" 0 0 \0" as *const u8 as *const ::core::ffi::c_char);
        zpdfprintreal(pk_font_scale, 5 as ::core::ffi::c_int);
        pdf_puts(b" 0 0]\n\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        crate::utils::pdf_printf_args(
            b"/FontMatrix [%g 0 0 %g 0 0]\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(t3_font_scale as ::core::ffi::c_double),
                crate::utils::PrintfArg::from(t3_font_scale as ::core::ffi::c_double),
            ],
        );
    }
    crate::utils::pdf_printf_args(
        b"/%s [ %i %i %i %i ]\n\0" as *const u8 as *const ::core::ffi::c_char,
        &[
            crate::utils::PrintfArg::from(font_key[FONTBBOX1_CODE as usize].pdfname),
            crate::utils::PrintfArg::from(t3_b0),
            crate::utils::PrintfArg::from(t3_b1),
            crate::utils::PrintfArg::from(t3_b2),
            crate::utils::PrintfArg::from(t3_b3),
        ],
    );
    crate::utils::pdf_printf_args(
        b"/Resources << /ProcSet [ /PDF %s] >>\n\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(if t3_image_used != 0 {
            b"/ImageB \0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        })],
    );
    crate::utils::pdf_printf_args(
        b"/FirstChar %i\n/LastChar %i\n\0" as *const u8 as *const ::core::ffi::c_char,
        &[
            crate::utils::PrintfArg::from(first_char),
            crate::utils::PrintfArg::from(last_char),
        ],
    );
    wptr = pdfnewobjnum();
    eptr = pdfnewobjnum();
    cptr = pdfnewobjnum();
    crate::utils::pdf_printf_args(
        b"/Widths %i 0 R\n/Encoding %i 0 R\n/CharProcs %i 0 R\n\0" as *const u8
            as *const ::core::ffi::c_char,
        &[
            crate::utils::PrintfArg::from(wptr),
            crate::utils::PrintfArg::from(eptr),
            crate::utils::PrintfArg::from(cptr),
        ],
    );
    if tounicode_objnum != 0 as ::core::ffi::c_int {
        crate::utils::pdf_printf_args(
            b"/ToUnicode %i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(tounicode_objnum)],
        );
    }
    pdfenddict();
    zpdfbeginobj(wptr, 1 as ::core::ffi::c_int);
    pdf_puts(b"[\0" as *const u8 as *const ::core::ffi::c_char);
    if is_pk_font != 0 {
        i = first_char;
        while i <= last_char {
            zpdfprintreal(
                t3_char_widths[i as usize] as integer,
                2 as ::core::ffi::c_int,
            );
            pdf_puts(b" \0" as *const u8 as *const ::core::ffi::c_char);
            i += 1;
        }
    } else {
        i = first_char;
        while i <= last_char {
            crate::utils::pdf_printf_args(
                b"%i \0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(
                    t3_char_widths[i as usize] as ::core::ffi::c_int,
                )],
            );
            i += 1;
        }
    }
    pdf_puts(b"]\n\0" as *const u8 as *const ::core::ffi::c_char);
    pdfendobj();
    zpdfbegindict(eptr, 1 as ::core::ffi::c_int);
    crate::utils::pdf_printf_args(
        b"/Type /Encoding\n/Differences [%i\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(first_char)],
    );
    if t3_char_procs[first_char as usize] == 0 as ::core::ffi::c_int {
        crate::utils::pdf_printf_args(
            b"/%s\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                &raw mut notdef as *mut ::core::ffi::c_char,
            )],
        );
        is_notdef = true_0 as boolean;
    } else {
        if !glyph_names.is_null()
            && !(*glyph_names.offset(first_char as isize)).is_null()
            && *glyph_names.offset(first_char as isize)
                != &raw mut notdef as *mut ::core::ffi::c_char
        {
            crate::utils::pdf_printf_args(
                b"/%s\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(
                    *glyph_names.offset(first_char as isize),
                )],
            );
        } else {
            crate::utils::pdf_printf_args(
                b"/a%i\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(first_char)],
            );
        }
        is_notdef = false_0 as boolean;
    }
    i = first_char + 1 as ::core::ffi::c_int;
    while i <= last_char {
        if t3_char_procs[i as usize] == 0 as ::core::ffi::c_int {
            if is_notdef == 0 {
                crate::utils::pdf_printf_args(
                    b" %i/%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(i),
                        crate::utils::PrintfArg::from(&raw mut notdef as *mut ::core::ffi::c_char),
                    ],
                );
                is_notdef = true_0 as boolean;
            }
        } else {
            if is_notdef != 0 {
                crate::utils::pdf_printf_args(
                    b" %i\0" as *const u8 as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(i)],
                );
                is_notdef = false_0 as boolean;
            }
            if !glyph_names.is_null()
                && !(*glyph_names.offset(i as isize)).is_null()
                && *glyph_names.offset(i as isize) != &raw mut notdef as *mut ::core::ffi::c_char
            {
                crate::utils::pdf_printf_args(
                    b"/%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(
                        *glyph_names.offset(i as isize),
                    )],
                );
            } else {
                crate::utils::pdf_printf_args(
                    b"/a%i\0" as *const u8 as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(i)],
                );
            }
        }
        i += 1;
    }
    pdf_puts(b"]\n\0" as *const u8 as *const ::core::ffi::c_char);
    pdfenddict();
    zpdfbegindict(cptr, 1 as ::core::ffi::c_int);
    i = first_char;
    while i <= last_char {
        if t3_char_procs[i as usize] != 0 as ::core::ffi::c_int {
            if !glyph_names.is_null()
                && !(*glyph_names.offset(i as isize)).is_null()
                && *glyph_names.offset(i as isize) != &raw mut notdef as *mut ::core::ffi::c_char
            {
                crate::utils::pdf_printf_args(
                    b"/%s %i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(*glyph_names.offset(i as isize)),
                        crate::utils::PrintfArg::from(t3_char_procs[i as usize]),
                    ],
                );
            } else {
                crate::utils::pdf_printf_args(
                    b"/a%i %i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(i),
                        crate::utils::PrintfArg::from(t3_char_procs[i as usize]),
                    ],
                );
            }
        }
        i += 1;
    }
    pdfenddict();
    xfclose(t3_file, cur_file_name as const_string);
    crate::utils::tex_printf_args(b">\0" as *const u8 as *const ::core::ffi::c_char, &[]);
    cur_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
