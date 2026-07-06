#[repr(C)]
pub struct __sFILEX {
    _unused: [u8; 0],
}

extern "C" {
    fn atan(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn feof(_: *mut FILE) -> ::core::ffi::c_int;
    fn fgetc(_: *mut FILE) -> ::core::ffi::c_int;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn ungetc(_: ::core::ffi::c_int, _: *mut FILE) -> ::core::ffi::c_int;
    fn free(_: *mut ::core::ffi::c_void);
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcat(
        __s1: *mut ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strcpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strncpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strstr(
        __big: *const ::core::ffi::c_char,
        __little: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn xstrdup(s: const_string) -> string;
    fn xfopen(filename: const_string, mode: const_string) -> *mut FILE;
    fn xfclose(fp: *mut FILE, filename: const_string);
    fn xmalloc(size: size_t) -> address;
    fn xrealloc(old_address: address, new_size: size_t) -> address;
    fn open_input(_: *mut *mut FILE, _: ::core::ffi::c_int, fopen_mode: const_string) -> boolean;
    fn recorder_record_input(_: const_string);
    fn zround(_: ::core::ffi::c_double) -> integer;
    static mut _DefaultRuneLocale: _RuneLocale;
    static mut nameoffile: *mut ASCIIcode;
    fn zpackfilename(n: strnumber, a: strnumber, e: strnumber);
    fn getnullstr() -> strnumber;
    fn avl_create(
        _: Option<avl_comparison_func>,
        _: *mut ::core::ffi::c_void,
        _: *mut libavl_allocator,
    ) -> *mut avl_table;
    fn avl_destroy(_: *mut avl_table, _: Option<avl_item_func>);
    fn avl_probe(_: *mut avl_table, _: *mut ::core::ffi::c_void) -> *mut *mut ::core::ffi::c_void;
    fn avl_find(_: *const avl_table, _: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn avl_t_init(_: *mut avl_traverser, _: *mut avl_table);
    fn avl_t_first(_: *mut avl_traverser, _: *mut avl_table) -> *mut ::core::ffi::c_void;
    fn avl_t_next(_: *mut avl_traverser) -> *mut ::core::ffi::c_void;
    static mut avl_xallocator: libavl_allocator;
    static mut cur_file_name: *mut ::core::ffi::c_char;
    static mut last_ptr_index: size_t;
    static mut fb_array: *mut ::core::ffi::c_char;
    static mut notdef: [::core::ffi::c_char; 0];
    fn pdftex_fail(_: *const ::core::ffi::c_char, ...) -> !;
    fn pdftex_warn(_: *const ::core::ffi::c_char, ...);
    fn tex_printf(_: *const ::core::ffi::c_char, ...);
    fn check_ff_exist(_: *mut ::core::ffi::c_char, _: boolean) -> *mut ff_entry;
    fn maketexstring(_: *const ::core::ffi::c_char) -> strnumber;
    fn fb_offset() -> integer;
    fn fb_putchar(b: eightbits);
    fn make_subset_tag(_: *mut fd_entry);
}
pub type __builtin_va_list = *mut ::core::ffi::c_char;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __darwin_ct_rune_t = ::core::ffi::c_int;
pub type __darwin_size_t = usize;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_wchar_t = ::libc::wchar_t;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_off_t = __int64_t;
pub type va_list = __darwin_va_list;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const kpse_last_format: C2RustUnnamed = 59;
pub const kpse_bltxml_format: C2RustUnnamed = 58;
pub const kpse_ris_format: C2RustUnnamed = 57;
pub const kpse_clua_format: C2RustUnnamed = 56;
pub const kpse_mlbst_format: C2RustUnnamed = 55;
pub const kpse_mlbib_format: C2RustUnnamed = 54;
pub const kpse_cid_format: C2RustUnnamed = 53;
pub const kpse_fea_format: C2RustUnnamed = 52;
pub const kpse_lua_format: C2RustUnnamed = 51;
pub const kpse_texmfscripts_format: C2RustUnnamed = 50;
pub const kpse_lig_format: C2RustUnnamed = 49;
pub const kpse_pdftex_config_format: C2RustUnnamed = 48;
pub const kpse_opentype_format: C2RustUnnamed = 47;
pub const kpse_sfd_format: C2RustUnnamed = 46;
pub const kpse_cmap_format: C2RustUnnamed = 45;
pub const kpse_enc_format: C2RustUnnamed = 44;
pub const kpse_cweb_format: C2RustUnnamed = 43;
pub const kpse_web_format: C2RustUnnamed = 42;
pub const kpse_miscfonts_format: C2RustUnnamed = 41;
pub const kpse_program_binary_format: C2RustUnnamed = 40;
pub const kpse_program_text_format: C2RustUnnamed = 39;
pub const kpse_web2c_format: C2RustUnnamed = 38;
pub const kpse_type42_format: C2RustUnnamed = 37;
pub const kpse_truetype_format: C2RustUnnamed = 36;
pub const kpse_ist_format: C2RustUnnamed = 35;
pub const kpse_dvips_config_format: C2RustUnnamed = 34;
pub const kpse_vf_format: C2RustUnnamed = 33;
pub const kpse_type1_format: C2RustUnnamed = 32;
pub const kpse_troff_font_format: C2RustUnnamed = 31;
pub const kpse_tex_ps_header_format: C2RustUnnamed = 30;
pub const kpse_texsource_format: C2RustUnnamed = 29;
pub const kpse_texpool_format: C2RustUnnamed = 28;
pub const kpse_texdoc_format: C2RustUnnamed = 27;
pub const kpse_tex_format: C2RustUnnamed = 26;
pub const kpse_pict_format: C2RustUnnamed = 25;
pub const kpse_ovp_format: C2RustUnnamed = 24;
pub const kpse_ovf_format: C2RustUnnamed = 23;
pub const kpse_otp_format: C2RustUnnamed = 22;
pub const kpse_opl_format: C2RustUnnamed = 21;
pub const kpse_ofm_format: C2RustUnnamed = 20;
pub const kpse_ocp_format: C2RustUnnamed = 19;
pub const kpse_mpsupport_format: C2RustUnnamed = 18;
pub const kpse_mppool_format: C2RustUnnamed = 17;
pub const kpse_mp_format: C2RustUnnamed = 16;
pub const kpse_mft_format: C2RustUnnamed = 15;
pub const kpse_mfpool_format: C2RustUnnamed = 14;
pub const kpse_mf_format: C2RustUnnamed = 13;
pub const kpse_mem_format: C2RustUnnamed = 12;
pub const kpse_fontmap_format: C2RustUnnamed = 11;
pub const kpse_fmt_format: C2RustUnnamed = 10;
pub const kpse_db_format: C2RustUnnamed = 9;
pub const kpse_cnf_format: C2RustUnnamed = 8;
pub const kpse_bst_format: C2RustUnnamed = 7;
pub const kpse_bib_format: C2RustUnnamed = 6;
pub const kpse_base_format: C2RustUnnamed = 5;
pub const kpse_afm_format: C2RustUnnamed = 4;
pub const kpse_tfm_format: C2RustUnnamed = 3;
pub const kpse_any_glyph_format: C2RustUnnamed = 2;
pub const kpse_pk_format: C2RustUnnamed = 1;
pub const kpse_gf_format: C2RustUnnamed = 0;
pub type integer = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: ::core::ffi::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [::core::ffi::c_char; 14],
    pub __mask: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [::core::ffi::c_char; 8],
    pub __encoding: [::core::ffi::c_char; 32],
    pub __sgetrune: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            __darwin_size_t,
            *mut *const ::core::ffi::c_char,
        ) -> __darwin_rune_t,
    >,
    pub __sputrune: Option<
        unsafe extern "C" fn(
            __darwin_rune_t,
            *mut ::core::ffi::c_char,
            __darwin_size_t,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub __invalid_rune: __darwin_rune_t,
    pub __runetype: [__uint32_t; 256],
    pub __maplower: [__darwin_rune_t; 256],
    pub __mapupper: [__darwin_rune_t; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut ::core::ffi::c_void,
    pub __variable_len: ::core::ffi::c_int,
    pub __ncharclasses: ::core::ffi::c_int,
    pub __charclasses: *mut _RuneCharClass,
}
pub type ASCIIcode = ::core::ffi::c_uchar;
pub type eightbits = ::core::ffi::c_uchar;
pub type strnumber = integer;
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
pub struct avl_traverser {
    pub avl_table: *mut avl_table,
    pub avl_node: *mut avl_node,
    pub avl_stack: [*mut avl_node; 32],
    pub avl_height: size_t,
    pub avl_generation: ::core::ffi::c_ulong,
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
pub struct intparm {
    pub val: ::core::ffi::c_int,
    pub set: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_entry_ {
    pub fd_objnum: integer,
    pub fontname: *mut ::core::ffi::c_char,
    pub subset_tag: *mut ::core::ffi::c_char,
    pub ff_found: boolean,
    pub ff_objnum: integer,
    pub fn_objnum: integer,
    pub all_glyphs: boolean,
    pub write_ttf_glyph_names: boolean,
    pub font_dim: [intparm; 11],
    pub fe: *mut fe_entry,
    pub builtin_glyph_names: *mut *mut ::core::ffi::c_char,
    pub fm: *mut fm_entry,
    pub tx_tree: *mut avl_table,
    pub gl_tree: *mut avl_table,
}
pub type fd_entry = fd_entry_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ff_entry {
    pub ff_name: *mut ::core::ffi::c_char,
    pub ff_path: *mut ::core::ffi::c_char,
}
pub type t1_line_entry = ::core::ffi::c_char;
pub type byte = ::core::ffi::c_uchar;
pub type t1_buf_entry = ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cs_entry {
    pub name: *mut ::core::ffi::c_char,
    pub data: *mut byte,
    pub len: ::core::ffi::c_ushort,
    pub cslen: ::core::ffi::c_ushort,
    pub used: boolean,
    pub valid: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cc_entry {
    pub nargs: byte,
    pub bottom: boolean,
    pub clear: boolean,
    pub valid: boolean,
}
pub const ENC_STANDARD: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const ENC_BUILTIN: C2RustUnnamed_0 = 1;
pub const M_PI: ::core::ffi::c_double = 3.14159265358979323846264338327950288f64;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const FOPEN_RBIN_MODE: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"rb\0") };
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ITALIC_ANGLE_CODE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const STEMV_CODE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FONTBBOX1_CODE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const FONTNAME_CODE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const FONT_KEYS_NUM: ::core::ffi::c_int = FONTNAME_CODE + 1 as ::core::ffi::c_int;
pub const F_SUBSETTED: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const F_TRUETYPE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const _CACHED_RUNES: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const _CTYPE_D: ::core::ffi::c_long = 0x400 as ::core::ffi::c_long;
unsafe fn __isctype(
    mut _c: __darwin_ct_rune_t,
    mut _f: ::core::ffi::c_ulong,
) -> __darwin_ct_rune_t {
    return if _c < 0 as ::core::ffi::c_int || _c >= _CACHED_RUNES {
        0 as __darwin_ct_rune_t
    } else {
        (_DefaultRuneLocale.__runetype[_c as usize] as ::core::ffi::c_ulong & _f != 0)
            as ::core::ffi::c_int
    };
}
unsafe fn isdigit(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return __isctype(_c as __darwin_ct_rune_t, _CTYPE_D as ::core::ffi::c_ulong)
        as ::core::ffi::c_int;
}
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
pub const fixedcontent: ::core::ffi::c_int = false_0;
static mut standard_glyph_names: [*const ::core::ffi::c_char; 256] = unsafe {
    [
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        b"space\0" as *const u8 as *const ::core::ffi::c_char,
        b"exclam\0" as *const u8 as *const ::core::ffi::c_char,
        b"quotedbl\0" as *const u8 as *const ::core::ffi::c_char,
        b"numbersign\0" as *const u8 as *const ::core::ffi::c_char,
        b"dollar\0" as *const u8 as *const ::core::ffi::c_char,
        b"percent\0" as *const u8 as *const ::core::ffi::c_char,
        b"ampersand\0" as *const u8 as *const ::core::ffi::c_char,
        b"quoteright\0" as *const u8 as *const ::core::ffi::c_char,
        b"parenleft\0" as *const u8 as *const ::core::ffi::c_char,
        b"parenright\0" as *const u8 as *const ::core::ffi::c_char,
        b"asterisk\0" as *const u8 as *const ::core::ffi::c_char,
        b"plus\0" as *const u8 as *const ::core::ffi::c_char,
        b"comma\0" as *const u8 as *const ::core::ffi::c_char,
        b"hyphen\0" as *const u8 as *const ::core::ffi::c_char,
        b"period\0" as *const u8 as *const ::core::ffi::c_char,
        b"slash\0" as *const u8 as *const ::core::ffi::c_char,
        b"zero\0" as *const u8 as *const ::core::ffi::c_char,
        b"one\0" as *const u8 as *const ::core::ffi::c_char,
        b"two\0" as *const u8 as *const ::core::ffi::c_char,
        b"three\0" as *const u8 as *const ::core::ffi::c_char,
        b"four\0" as *const u8 as *const ::core::ffi::c_char,
        b"five\0" as *const u8 as *const ::core::ffi::c_char,
        b"six\0" as *const u8 as *const ::core::ffi::c_char,
        b"seven\0" as *const u8 as *const ::core::ffi::c_char,
        b"eight\0" as *const u8 as *const ::core::ffi::c_char,
        b"nine\0" as *const u8 as *const ::core::ffi::c_char,
        b"colon\0" as *const u8 as *const ::core::ffi::c_char,
        b"semicolon\0" as *const u8 as *const ::core::ffi::c_char,
        b"less\0" as *const u8 as *const ::core::ffi::c_char,
        b"equal\0" as *const u8 as *const ::core::ffi::c_char,
        b"greater\0" as *const u8 as *const ::core::ffi::c_char,
        b"question\0" as *const u8 as *const ::core::ffi::c_char,
        b"at\0" as *const u8 as *const ::core::ffi::c_char,
        b"A\0" as *const u8 as *const ::core::ffi::c_char,
        b"B\0" as *const u8 as *const ::core::ffi::c_char,
        b"C\0" as *const u8 as *const ::core::ffi::c_char,
        b"D\0" as *const u8 as *const ::core::ffi::c_char,
        b"E\0" as *const u8 as *const ::core::ffi::c_char,
        b"F\0" as *const u8 as *const ::core::ffi::c_char,
        b"G\0" as *const u8 as *const ::core::ffi::c_char,
        b"H\0" as *const u8 as *const ::core::ffi::c_char,
        b"I\0" as *const u8 as *const ::core::ffi::c_char,
        b"J\0" as *const u8 as *const ::core::ffi::c_char,
        b"K\0" as *const u8 as *const ::core::ffi::c_char,
        b"L\0" as *const u8 as *const ::core::ffi::c_char,
        b"M\0" as *const u8 as *const ::core::ffi::c_char,
        b"N\0" as *const u8 as *const ::core::ffi::c_char,
        b"O\0" as *const u8 as *const ::core::ffi::c_char,
        b"P\0" as *const u8 as *const ::core::ffi::c_char,
        b"Q\0" as *const u8 as *const ::core::ffi::c_char,
        b"R\0" as *const u8 as *const ::core::ffi::c_char,
        b"S\0" as *const u8 as *const ::core::ffi::c_char,
        b"T\0" as *const u8 as *const ::core::ffi::c_char,
        b"U\0" as *const u8 as *const ::core::ffi::c_char,
        b"V\0" as *const u8 as *const ::core::ffi::c_char,
        b"W\0" as *const u8 as *const ::core::ffi::c_char,
        b"X\0" as *const u8 as *const ::core::ffi::c_char,
        b"Y\0" as *const u8 as *const ::core::ffi::c_char,
        b"Z\0" as *const u8 as *const ::core::ffi::c_char,
        b"bracketleft\0" as *const u8 as *const ::core::ffi::c_char,
        b"backslash\0" as *const u8 as *const ::core::ffi::c_char,
        b"bracketright\0" as *const u8 as *const ::core::ffi::c_char,
        b"asciicircum\0" as *const u8 as *const ::core::ffi::c_char,
        b"underscore\0" as *const u8 as *const ::core::ffi::c_char,
        b"quoteleft\0" as *const u8 as *const ::core::ffi::c_char,
        b"a\0" as *const u8 as *const ::core::ffi::c_char,
        b"b\0" as *const u8 as *const ::core::ffi::c_char,
        b"c\0" as *const u8 as *const ::core::ffi::c_char,
        b"d\0" as *const u8 as *const ::core::ffi::c_char,
        b"e\0" as *const u8 as *const ::core::ffi::c_char,
        b"f\0" as *const u8 as *const ::core::ffi::c_char,
        b"g\0" as *const u8 as *const ::core::ffi::c_char,
        b"h\0" as *const u8 as *const ::core::ffi::c_char,
        b"i\0" as *const u8 as *const ::core::ffi::c_char,
        b"j\0" as *const u8 as *const ::core::ffi::c_char,
        b"k\0" as *const u8 as *const ::core::ffi::c_char,
        b"l\0" as *const u8 as *const ::core::ffi::c_char,
        b"m\0" as *const u8 as *const ::core::ffi::c_char,
        b"n\0" as *const u8 as *const ::core::ffi::c_char,
        b"o\0" as *const u8 as *const ::core::ffi::c_char,
        b"p\0" as *const u8 as *const ::core::ffi::c_char,
        b"q\0" as *const u8 as *const ::core::ffi::c_char,
        b"r\0" as *const u8 as *const ::core::ffi::c_char,
        b"s\0" as *const u8 as *const ::core::ffi::c_char,
        b"t\0" as *const u8 as *const ::core::ffi::c_char,
        b"u\0" as *const u8 as *const ::core::ffi::c_char,
        b"v\0" as *const u8 as *const ::core::ffi::c_char,
        b"w\0" as *const u8 as *const ::core::ffi::c_char,
        b"x\0" as *const u8 as *const ::core::ffi::c_char,
        b"y\0" as *const u8 as *const ::core::ffi::c_char,
        b"z\0" as *const u8 as *const ::core::ffi::c_char,
        b"braceleft\0" as *const u8 as *const ::core::ffi::c_char,
        b"bar\0" as *const u8 as *const ::core::ffi::c_char,
        b"braceright\0" as *const u8 as *const ::core::ffi::c_char,
        b"asciitilde\0" as *const u8 as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        b"exclamdown\0" as *const u8 as *const ::core::ffi::c_char,
        b"cent\0" as *const u8 as *const ::core::ffi::c_char,
        b"sterling\0" as *const u8 as *const ::core::ffi::c_char,
        b"fraction\0" as *const u8 as *const ::core::ffi::c_char,
        b"yen\0" as *const u8 as *const ::core::ffi::c_char,
        b"florin\0" as *const u8 as *const ::core::ffi::c_char,
        b"section\0" as *const u8 as *const ::core::ffi::c_char,
        b"currency\0" as *const u8 as *const ::core::ffi::c_char,
        b"quotesingle\0" as *const u8 as *const ::core::ffi::c_char,
        b"quotedblleft\0" as *const u8 as *const ::core::ffi::c_char,
        b"guillemotleft\0" as *const u8 as *const ::core::ffi::c_char,
        b"guilsinglleft\0" as *const u8 as *const ::core::ffi::c_char,
        b"guilsinglright\0" as *const u8 as *const ::core::ffi::c_char,
        b"fi\0" as *const u8 as *const ::core::ffi::c_char,
        b"fl\0" as *const u8 as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        b"endash\0" as *const u8 as *const ::core::ffi::c_char,
        b"dagger\0" as *const u8 as *const ::core::ffi::c_char,
        b"daggerdbl\0" as *const u8 as *const ::core::ffi::c_char,
        b"periodcentered\0" as *const u8 as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        b"paragraph\0" as *const u8 as *const ::core::ffi::c_char,
        b"bullet\0" as *const u8 as *const ::core::ffi::c_char,
        b"quotesinglbase\0" as *const u8 as *const ::core::ffi::c_char,
        b"quotedblbase\0" as *const u8 as *const ::core::ffi::c_char,
        b"quotedblright\0" as *const u8 as *const ::core::ffi::c_char,
        b"guillemotright\0" as *const u8 as *const ::core::ffi::c_char,
        b"ellipsis\0" as *const u8 as *const ::core::ffi::c_char,
        b"perthousand\0" as *const u8 as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        b"questiondown\0" as *const u8 as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        b"grave\0" as *const u8 as *const ::core::ffi::c_char,
        b"acute\0" as *const u8 as *const ::core::ffi::c_char,
        b"circumflex\0" as *const u8 as *const ::core::ffi::c_char,
        b"tilde\0" as *const u8 as *const ::core::ffi::c_char,
        b"macron\0" as *const u8 as *const ::core::ffi::c_char,
        b"breve\0" as *const u8 as *const ::core::ffi::c_char,
        b"dotaccent\0" as *const u8 as *const ::core::ffi::c_char,
        b"dieresis\0" as *const u8 as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        b"ring\0" as *const u8 as *const ::core::ffi::c_char,
        b"cedilla\0" as *const u8 as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        b"hungarumlaut\0" as *const u8 as *const ::core::ffi::c_char,
        b"ogonek\0" as *const u8 as *const ::core::ffi::c_char,
        b"caron\0" as *const u8 as *const ::core::ffi::c_char,
        b"emdash\0" as *const u8 as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        b"AE\0" as *const u8 as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        b"ordfeminine\0" as *const u8 as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        b"Lslash\0" as *const u8 as *const ::core::ffi::c_char,
        b"Oslash\0" as *const u8 as *const ::core::ffi::c_char,
        b"OE\0" as *const u8 as *const ::core::ffi::c_char,
        b"ordmasculine\0" as *const u8 as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        b"ae\0" as *const u8 as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        b"dotlessi\0" as *const u8 as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        b"lslash\0" as *const u8 as *const ::core::ffi::c_char,
        b"oslash\0" as *const u8 as *const ::core::ffi::c_char,
        b"oe\0" as *const u8 as *const ::core::ffi::c_char,
        b"germandbls\0" as *const u8 as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
    ]
};
#[no_mangle]
pub static mut t1_length1: integer = 0;
#[no_mangle]
pub static mut t1_length3: integer = 0;
#[no_mangle]
pub static mut t1_length2: integer = 0;
static mut t1_save_offset: integer = 0;
static mut t1_fontname_offset: integer = 0;
static mut fd_cur: *mut fd_entry = ::core::ptr::null::<fd_entry>() as *mut fd_entry;
static mut charstringname: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"/CharStrings\0") };
#[no_mangle]
pub static mut t1_encoding: C2RustUnnamed_0 = ENC_STANDARD;
pub const CS_CALLSUBR: ::core::ffi::c_long = 10 as ::core::ffi::c_long;
pub const CS_RETURN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const CS_ESCAPE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const CS_HVCURVETO: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const CS_1BYTE_MAX: ::core::ffi::c_int = CS_HVCURVETO + 1 as ::core::ffi::c_int;
pub const CS_2BYTE_MAX: ::core::ffi::c_int =
    CS_1BYTE_MAX + 33 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
pub const CS_MAX: ::core::ffi::c_int = CS_2BYTE_MAX;
static mut t1_dr: ::core::ffi::c_ushort = 0;
static mut t1_er: ::core::ffi::c_ushort = 0;
static mut t1_c2: ::core::ffi::c_ushort = 22719 as ::core::ffi::c_ushort;
static mut t1_c1: ::core::ffi::c_ushort = 52845 as ::core::ffi::c_ushort;
static mut t1_cslen: ::core::ffi::c_ushort = 0;
static mut t1_lenIV: ::core::ffi::c_short = 0;
static mut enc_line: [::core::ffi::c_char; 4096] = [0; 4096];
#[no_mangle]
pub static mut t1_line_ptr: *mut t1_line_entry =
    ::core::ptr::null::<t1_line_entry>() as *mut t1_line_entry;
#[no_mangle]
pub static mut t1_line_array: *mut t1_line_entry =
    ::core::ptr::null::<t1_line_entry>() as *mut t1_line_entry;
#[no_mangle]
pub static mut t1_line_limit: size_t = 0;
#[no_mangle]
pub static mut t1_buf_ptr: *mut t1_buf_entry =
    ::core::ptr::null::<t1_buf_entry>() as *mut t1_buf_entry;
#[no_mangle]
pub static mut t1_buf_array: *mut t1_buf_entry =
    ::core::ptr::null::<t1_buf_entry>() as *mut t1_buf_entry;
#[no_mangle]
pub static mut t1_buf_limit: size_t = 0;
static mut cs_start: ::core::ffi::c_int = 0;
static mut cs_tab: *mut cs_entry = ::core::ptr::null::<cs_entry>() as *mut cs_entry;
static mut cs_ptr: *mut cs_entry = ::core::ptr::null::<cs_entry>() as *mut cs_entry;
static mut cs_notdef: *mut cs_entry = ::core::ptr::null::<cs_entry>() as *mut cs_entry;
static mut cs_dict_end: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut cs_dict_start: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut cs_count: ::core::ffi::c_int = 0;
static mut cs_size_pos: ::core::ffi::c_int = 0;
static mut cs_size: ::core::ffi::c_int = 0;
static mut subr_tab: *mut cs_entry = ::core::ptr::null::<cs_entry>() as *mut cs_entry;
static mut subr_array_start: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut subr_array_end: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut subr_max: ::core::ffi::c_int = 0;
static mut subr_size: ::core::ffi::c_int = 0;
static mut subr_size_pos: ::core::ffi::c_int = 0;
static mut cs_token_pairs_list: [[*const ::core::ffi::c_char; 2]; 5] = [
    [
        b" RD\0" as *const u8 as *const ::core::ffi::c_char,
        b"NP\0" as *const u8 as *const ::core::ffi::c_char,
    ],
    [
        b" -|\0" as *const u8 as *const ::core::ffi::c_char,
        b"|\0" as *const u8 as *const ::core::ffi::c_char,
    ],
    [
        b" RD\0" as *const u8 as *const ::core::ffi::c_char,
        b"noaccess put\0" as *const u8 as *const ::core::ffi::c_char,
    ],
    [
        b" -|\0" as *const u8 as *const ::core::ffi::c_char,
        b"noaccess put\0" as *const u8 as *const ::core::ffi::c_char,
    ],
    [
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ],
];
static mut cs_token_pair: *mut *const ::core::ffi::c_char =
    ::core::ptr::null::<*const ::core::ffi::c_char>() as *mut *const ::core::ffi::c_char;
static mut t1_eexec_encrypt: boolean = 0;
static mut t1_scan: boolean = 0;
static mut t1_synthetic: boolean = 0;
static mut t1_pfa: boolean = 0;
static mut t1_cs: boolean = 0;
static mut t1_in_eexec: ::core::ffi::c_int = 0;
static mut t1_block_length: ::core::ffi::c_long = 0;
static mut last_hexbyte: ::core::ffi::c_int = 0;
static mut t1_file: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
static mut enc_file: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
unsafe extern "C" fn enc_getline() {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut c: ::core::ffi::c_int = 0;
    loop {
        if feof(enc_file) != 0 {
            crate::utils::pdftex_fail_args(
                b"unexpected end of file\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        p = &raw mut enc_line as *mut ::core::ffi::c_char;
        loop {
            c = fgetc(enc_file);
            if c == 9 as ::core::ffi::c_int {
                c = 32 as ::core::ffi::c_int;
            }
            if c == 13 as ::core::ffi::c_int || c == EOF {
                c = 10 as ::core::ffi::c_int;
            }
            if c != ' ' as i32
                || p > &raw mut enc_line as *mut ::core::ffi::c_char
                    && *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                        != 32 as ::core::ffi::c_int
            {
                if (p.offset_from(&raw mut enc_line as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                    > 0x1000 as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    crate::utils::pdftex_fail_args(
                        b"buffer overflow at file %s, line %d\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[
                            crate::utils::PrintfArg::from(
                                b"pdftex-rust/generated/backend/writet1.rs\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            ),
                            crate::utils::PrintfArg::from(223 as ::core::ffi::c_int),
                        ],
                    );
                }
                let fresh4 = p;
                p = p.offset(1);
                *fresh4 = c as ::core::ffi::c_char;
            }
            if !(c != 10 as ::core::ffi::c_int) {
                break;
            }
        }
        if (p.offset_from(&raw mut enc_line as *mut ::core::ffi::c_char) as ::core::ffi::c_long
            + 2 as ::core::ffi::c_long) as ::core::ffi::c_uint
            > 0x1000 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::utils::pdftex_fail_args(
                b"buffer overflow at file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(
                        b"pdftex-rust/generated/backend/writet1.rs\0" as *const u8
                            as *const ::core::ffi::c_char,
                    ),
                    crate::utils::PrintfArg::from(225 as ::core::ffi::c_int),
                ],
            );
        }
        if p.offset_from(&raw mut enc_line as *mut ::core::ffi::c_char) as ::core::ffi::c_long
            > 1 as ::core::ffi::c_long
            && *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                != 10 as ::core::ffi::c_int
        {
            let fresh5 = p;
            p = p.offset(1);
            *fresh5 = 10 as ::core::ffi::c_char;
        }
        if p.offset_from(&raw mut enc_line as *mut ::core::ffi::c_char) as ::core::ffi::c_long
            > 2 as ::core::ffi::c_long
            && *p.offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == 32 as ::core::ffi::c_int
        {
            *p.offset(-(2 as ::core::ffi::c_int) as isize) = 10 as ::core::ffi::c_char;
            p = p.offset(-1);
        }
        *p = 0 as ::core::ffi::c_char;
        if !((p.offset_from(&raw mut enc_line as *mut ::core::ffi::c_char) as ::core::ffi::c_long)
            < 2 as ::core::ffi::c_long
            || *(&raw mut enc_line as *mut ::core::ffi::c_char) as ::core::ffi::c_int == '%' as i32)
        {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn load_enc_file(
    mut enc_name: *mut ::core::ffi::c_char,
) -> *mut *mut ::core::ffi::c_char {
    let mut buf: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut names_count: ::core::ffi::c_int = 0;
    let mut glyph_names: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    cur_file_name = enc_name;
    zpackfilename(maketexstring(cur_file_name), getnullstr(), getnullstr());
    if open_input(
        &raw mut enc_file,
        kpse_enc_format as ::core::ffi::c_int,
        FOPEN_RBIN_MODE.as_ptr(),
    ) == 0
    {
        crate::utils::pdftex_fail_args(
            b"cannot open encoding file for reading\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
    }
    glyph_names = xmalloc(
        (256 as size_t).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
    ) as *mut *mut ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        let ref mut fresh0 = *glyph_names.offset(i as isize);
        *fresh0 = &raw mut notdef as *mut ::core::ffi::c_char;
        i += 1;
    }
    crate::utils::tex_printf_args(
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(
            b"{\0" as *const u8 as *const ::core::ffi::c_char,
        )],
    );
    cur_file_name =
        (nameoffile as *mut ::core::ffi::c_char).offset(1 as ::core::ffi::c_int as isize);
    crate::utils::tex_printf_args(
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(cur_file_name)],
    );
    enc_getline();
    if *(&raw mut enc_line as *mut ::core::ffi::c_char) as ::core::ffi::c_int != '/' as i32 || {
        r = strchr(&raw mut enc_line as *mut ::core::ffi::c_char, '[' as i32);
        r.is_null()
    } {
        r = strchr(
            &raw mut enc_line as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        )
        .offset(-(1 as ::core::ffi::c_int as isize));
        if *r as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
            *r = 0 as ::core::ffi::c_char;
        }
        crate::utils::pdftex_fail_args(
            b"invalid encoding vector (a name or `[' missing): `%s'\0" as *const u8
                as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                &raw mut enc_line as *mut ::core::ffi::c_char,
            )],
        );
    }
    names_count = 0 as ::core::ffi::c_int;
    r = r.offset(1);
    if *r as ::core::ffi::c_int == ' ' as i32 {
        r = r.offset(1);
    }
    loop {
        while *r as ::core::ffi::c_int == '/' as i32 {
            p = &raw mut buf as *mut ::core::ffi::c_char;
            r = r.offset(1);
            while *r as ::core::ffi::c_int != ' ' as i32
                && *r as ::core::ffi::c_int != 10 as ::core::ffi::c_int
                && *r as ::core::ffi::c_int != ']' as i32
                && *r as ::core::ffi::c_int != '/' as i32
            {
                let fresh1 = r;
                r = r.offset(1);
                let fresh2 = p;
                p = p.offset(1);
                *fresh2 = *fresh1;
            }
            *p = 0 as ::core::ffi::c_char;
            if *r as ::core::ffi::c_int == ' ' as i32 {
                r = r.offset(1);
            }
            if names_count > 255 as ::core::ffi::c_int {
                crate::utils::pdftex_fail_args(
                    b"encoding vector contains more than 256 names\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if strcmp(
                &raw mut buf as *mut ::core::ffi::c_char,
                &raw mut notdef as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            {
                let ref mut fresh3 = *glyph_names.offset(names_count as isize);
                *fresh3 = xstrdup(&raw mut buf as *mut ::core::ffi::c_char as const_string)
                    as *mut ::core::ffi::c_char;
            }
            names_count += 1;
        }
        if *r as ::core::ffi::c_int != 10 as ::core::ffi::c_int
            && *r as ::core::ffi::c_int != '%' as i32
        {
            if strncmp(
                r,
                b"] def\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"] def\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0 as ::core::ffi::c_int
            {
                break;
            }
            r = strchr(
                &raw mut enc_line as *mut ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            )
            .offset(-(1 as ::core::ffi::c_int as isize));
            if *r as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
                *r = 0 as ::core::ffi::c_char;
            }
            crate::utils::pdftex_fail_args(
                b"invalid encoding vector: a name or `] def' expected: `%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(
                    &raw mut enc_line as *mut ::core::ffi::c_char,
                )],
            );
        } else {
            enc_getline();
            r = &raw mut enc_line as *mut ::core::ffi::c_char;
        }
    }
    xfclose(enc_file, cur_file_name as const_string);
    crate::utils::tex_printf_args(
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(
            b"}\0" as *const u8 as *const ::core::ffi::c_char,
        )],
    );
    cur_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return glyph_names;
}
unsafe extern "C" fn t1_check_pfa() {
    let c: ::core::ffi::c_int = fgetc(t1_file) as ::core::ffi::c_int;
    t1_pfa = (if c != 128 as ::core::ffi::c_int {
        true_0
    } else {
        false_0
    }) as boolean;
    ungetc(c, t1_file);
}
unsafe extern "C" fn t1_getbyte() -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = fgetc(t1_file);
    if t1_pfa != 0 {
        return c;
    }
    if t1_block_length == 0 as ::core::ffi::c_long {
        if c != 128 as ::core::ffi::c_int {
            crate::utils::pdftex_fail_args(
                b"invalid marker\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        c = fgetc(t1_file);
        if c == 3 as ::core::ffi::c_int {
            while feof(t1_file) == 0 {
                fgetc(t1_file);
            }
            return EOF;
        }
        t1_block_length = (fgetc(t1_file) & 0xff as ::core::ffi::c_int) as ::core::ffi::c_long;
        t1_block_length |= ((fgetc(t1_file) & 0xff as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int) as ::core::ffi::c_long;
        t1_block_length |= ((fgetc(t1_file) & 0xff as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int) as ::core::ffi::c_long;
        t1_block_length |= ((fgetc(t1_file) & 0xff as ::core::ffi::c_int)
            << 24 as ::core::ffi::c_int) as ::core::ffi::c_long;
        c = fgetc(t1_file);
    }
    t1_block_length -= 1;
    return c;
}
unsafe extern "C" fn hexval(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if c >= 'A' as i32 && c <= 'F' as i32 {
        return c - 'A' as i32 + 10 as ::core::ffi::c_int;
    } else if c >= 'a' as i32 && c <= 'f' as i32 {
        return c - 'a' as i32 + 10 as ::core::ffi::c_int;
    } else if c >= '0' as i32 && c <= '9' as i32 {
        return c - '0' as i32;
    } else {
        return -(1 as ::core::ffi::c_int);
    };
}
unsafe extern "C" fn edecrypt(mut cipher: byte) -> byte {
    let mut plain: byte = 0;
    if t1_pfa != 0 {
        while cipher as ::core::ffi::c_int == 10 as ::core::ffi::c_int
            || cipher as ::core::ffi::c_int == 13 as ::core::ffi::c_int
        {
            cipher = t1_getbyte() as byte;
        }
        cipher = ((hexval(cipher as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            + hexval(t1_getbyte())) as byte;
        last_hexbyte = cipher as ::core::ffi::c_int;
    }
    plain = (cipher as ::core::ffi::c_int ^ t1_dr as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
        as byte;
    t1_dr = ((cipher as ::core::ffi::c_int + t1_dr as ::core::ffi::c_int)
        * t1_c1 as ::core::ffi::c_int
        + t1_c2 as ::core::ffi::c_int) as ::core::ffi::c_ushort;
    return plain;
}
unsafe extern "C" fn cdecrypt(mut cipher: byte, mut cr: *mut ::core::ffi::c_ushort) -> byte {
    let plain: byte = (cipher as ::core::ffi::c_int
        ^ *cr as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as byte;
    *cr = ((cipher as ::core::ffi::c_int + *cr as ::core::ffi::c_int) * t1_c1 as ::core::ffi::c_int
        + t1_c2 as ::core::ffi::c_int) as ::core::ffi::c_ushort;
    return plain;
}
unsafe extern "C" fn eencrypt(mut plain: byte) -> byte {
    let cipher: byte = (plain as ::core::ffi::c_int
        ^ t1_er as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as byte;
    t1_er = ((cipher as ::core::ffi::c_int + t1_er as ::core::ffi::c_int)
        * t1_c1 as ::core::ffi::c_int
        + t1_c2 as ::core::ffi::c_int) as ::core::ffi::c_ushort;
    return cipher;
}
unsafe extern "C" fn cencrypt(mut plain: byte, mut cr: *mut ::core::ffi::c_ushort) -> byte {
    let cipher: byte = (plain as ::core::ffi::c_int
        ^ *cr as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as byte;
    *cr = ((cipher as ::core::ffi::c_int + *cr as ::core::ffi::c_int) * t1_c1 as ::core::ffi::c_int
        + t1_c2 as ::core::ffi::c_int) as ::core::ffi::c_ushort;
    return cipher;
}
unsafe extern "C" fn eol(mut s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = strchr(s, 0 as ::core::ffi::c_int);
    if p.offset_from(s) as ::core::ffi::c_long > 1 as ::core::ffi::c_long
        && *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            != 10 as ::core::ffi::c_int
    {
        let fresh16 = p;
        p = p.offset(1);
        *fresh16 = 10 as ::core::ffi::c_char;
        *p = 0 as ::core::ffi::c_char;
    }
    return p;
}
unsafe extern "C" fn t1_scan_num(
    mut p: *mut ::core::ffi::c_char,
    mut r: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_float {
    let mut f: ::core::ffi::c_float = 0.;
    if *p as ::core::ffi::c_int == ' ' as i32 {
        p = p.offset(1);
    }
    if sscanf(
        p,
        b"%g\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut f,
    ) != 1 as ::core::ffi::c_int
    {
        p = strchr(t1_line_array, 0 as ::core::ffi::c_int)
            .offset(-(1 as ::core::ffi::c_int as isize));
        if *p as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
            *p = 0 as ::core::ffi::c_char;
        }
        crate::utils::pdftex_fail_args(
            b"a number expected: `%s'\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(t1_line_array)],
        );
    }
    if !r.is_null() {
        while isdigit(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0
            || *p as ::core::ffi::c_int == '.' as i32
            || *p as ::core::ffi::c_int == 'e' as i32
            || *p as ::core::ffi::c_int == 'E' as i32
            || *p as ::core::ffi::c_int == '+' as i32
            || *p as ::core::ffi::c_int == '-' as i32
        {
            p = p.offset(1);
        }
        *r = p;
    }
    return f;
}
unsafe extern "C" fn str_suffix(
    mut begin_buf: *const ::core::ffi::c_char,
    mut end_buf: *const ::core::ffi::c_char,
    mut s: *const ::core::ffi::c_char,
) -> boolean {
    let mut s1: *const ::core::ffi::c_char = end_buf.offset(-(1 as ::core::ffi::c_int as isize));
    let mut s2: *const ::core::ffi::c_char =
        strchr(s, 0 as ::core::ffi::c_int).offset(-(1 as ::core::ffi::c_int as isize));
    if *s1 as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
        s1 = s1.offset(-1);
    }
    while s1 >= begin_buf && s2 >= s {
        let fresh12 = s1;
        s1 = s1.offset(-1);
        let fresh13 = s2;
        s2 = s2.offset(-1);
        if *fresh12 as ::core::ffi::c_int != *fresh13 as ::core::ffi::c_int {
            return false_0;
        }
    }
    return (s2 < s) as ::core::ffi::c_int;
}
unsafe extern "C" fn t1_getline() {
    let mut c: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut eexec_scan: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    static mut eexec_str: [::core::ffi::c_char; 18] = unsafe {
        ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"currentfile eexec\0")
    };
    static mut eexec_len: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
    loop {
        if feof(t1_file) != 0 {
            crate::utils::pdftex_fail_args(
                b"unexpected end of file\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        t1_line_ptr = t1_line_array;
        if t1_line_array.is_null() {
            t1_line_limit = 0x10 as size_t;
            if 1 as ::core::ffi::c_int as ::core::ffi::c_uint > t1_line_limit as ::core::ffi::c_uint
            {
                t1_line_limit = 1 as size_t;
            }
            t1_line_array = xmalloc(
                t1_line_limit.wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
            ) as *mut t1_line_entry;
            t1_line_ptr = t1_line_array;
        } else if (t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
            > t1_line_limit as ::core::ffi::c_uint
        {
            last_ptr_index =
                t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long as size_t;
            t1_line_limit = t1_line_limit.wrapping_mul(2 as size_t);
            if (t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                > t1_line_limit as ::core::ffi::c_uint
            {
                t1_line_limit = (t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as size_t;
            }
            if t1_line_limit as ::core::ffi::c_uint > INT_MAX as ::core::ffi::c_uint {
                crate::utils::pdftex_fail_args(
                    b"t1_line_array exceeds size limit\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                );
            }
            t1_line_array = xrealloc(
                t1_line_array as address,
                t1_line_limit.wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
            ) as *mut t1_line_entry;
            t1_line_ptr = t1_line_array.offset(last_ptr_index as isize);
        }
        t1_cslen = 0 as ::core::ffi::c_ushort;
        eexec_scan = 0 as ::core::ffi::c_int;
        c = t1_getbyte();
        if c == EOF {
            break;
        }
        while feof(t1_file) == 0 {
            if t1_in_eexec == 1 as ::core::ffi::c_int {
                c = edecrypt(c as byte) as ::core::ffi::c_int;
            }
            if t1_line_array.is_null() {
                t1_line_limit = 0x10 as size_t;
                if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                    > t1_line_limit as ::core::ffi::c_uint
                {
                    t1_line_limit = 1 as size_t;
                }
                t1_line_array = xmalloc(
                    t1_line_limit.wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
                ) as *mut t1_line_entry;
                t1_line_ptr = t1_line_array;
            } else if (t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                > t1_line_limit as ::core::ffi::c_uint
            {
                last_ptr_index =
                    t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long as size_t;
                t1_line_limit = t1_line_limit.wrapping_mul(2 as size_t);
                if (t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                    > t1_line_limit as ::core::ffi::c_uint
                {
                    t1_line_limit = (t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as size_t;
                }
                if t1_line_limit as ::core::ffi::c_uint > INT_MAX as ::core::ffi::c_uint {
                    crate::utils::pdftex_fail_args(
                        b"t1_line_array exceeds size limit\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                }
                t1_line_array = xrealloc(
                    t1_line_array as address,
                    t1_line_limit.wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
                ) as *mut t1_line_entry;
                t1_line_ptr = t1_line_array.offset(last_ptr_index as isize);
            }
            if c == 9 as ::core::ffi::c_int {
                c = 32 as ::core::ffi::c_int;
            }
            if c == 13 as ::core::ffi::c_int || c == EOF {
                c = 10 as ::core::ffi::c_int;
            }
            if c != ' ' as i32
                || t1_line_ptr > t1_line_array
                    && *t1_line_ptr.offset(-(1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        != 32 as ::core::ffi::c_int
            {
                if (t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                    > t1_line_limit as ::core::ffi::c_uint
                {
                    crate::utils::pdftex_fail_args(
                        b"buffer overflow at file %s, line %d\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[
                            crate::utils::PrintfArg::from(
                                b"pdftex-rust/generated/backend/writet1.rs\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            ),
                            crate::utils::PrintfArg::from(422 as ::core::ffi::c_int),
                        ],
                    );
                }
                let fresh8 = t1_line_ptr;
                t1_line_ptr = t1_line_ptr.offset(1);
                *fresh8 = c as t1_line_entry;
            }
            if t1_in_eexec == 0 as ::core::ffi::c_int
                && eexec_scan >= 0 as ::core::ffi::c_int
                && eexec_scan < eexec_len
            {
                if *t1_line_array.offset(eexec_scan as isize) as ::core::ffi::c_int
                    == eexec_str[eexec_scan as usize] as ::core::ffi::c_int
                {
                    eexec_scan += 1;
                } else {
                    eexec_scan = -(1 as ::core::ffi::c_int);
                }
            }
            if c == 10 as ::core::ffi::c_int
                || t1_pfa != 0 && eexec_scan == eexec_len && c == 32 as ::core::ffi::c_int
            {
                break;
            }
            if t1_cs != 0
                && t1_cslen as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long
                    > 4 as ::core::ffi::c_long
                && (str_suffix(
                    t1_line_array,
                    t1_line_ptr,
                    b" RD \0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                    || str_suffix(
                        t1_line_array,
                        t1_line_ptr,
                        b" -| \0" as *const u8 as *const ::core::ffi::c_char,
                    ) != 0)
            {
                p = t1_line_ptr.offset(-(5 as ::core::ffi::c_int as isize))
                    as *mut ::core::ffi::c_char;
                while *p as ::core::ffi::c_int != ' ' as i32 {
                    p = p.offset(-1);
                }
                l = t1_scan_num(
                    p.offset(1 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                ) as ::core::ffi::c_int;
                t1_cslen = l as ::core::ffi::c_ushort;
                cs_start = t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
                if t1_line_array.is_null() {
                    t1_line_limit = 0x10 as size_t;
                    if l as ::core::ffi::c_uint > t1_line_limit as ::core::ffi::c_uint {
                        t1_line_limit = l as size_t;
                    }
                    t1_line_array = xmalloc(
                        t1_line_limit
                            .wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
                    ) as *mut t1_line_entry;
                    t1_line_ptr = t1_line_array;
                } else if (t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long
                    + l as ::core::ffi::c_long) as ::core::ffi::c_uint
                    > t1_line_limit as ::core::ffi::c_uint
                {
                    last_ptr_index =
                        t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long as size_t;
                    t1_line_limit = t1_line_limit.wrapping_mul(2 as size_t);
                    if (t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long
                        + l as ::core::ffi::c_long) as ::core::ffi::c_uint
                        > t1_line_limit as ::core::ffi::c_uint
                    {
                        t1_line_limit =
                            (t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long
                                + l as ::core::ffi::c_long) as size_t;
                    }
                    if t1_line_limit as ::core::ffi::c_uint > INT_MAX as ::core::ffi::c_uint {
                        crate::utils::pdftex_fail_args(
                            b"t1_line_array exceeds size limit\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    }
                    t1_line_array = xrealloc(
                        t1_line_array as address,
                        t1_line_limit
                            .wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
                    ) as *mut t1_line_entry;
                    t1_line_ptr = t1_line_array.offset(last_ptr_index as isize);
                }
                loop {
                    let fresh9 = l;
                    l = l - 1;
                    if !(fresh9 > 0 as ::core::ffi::c_int) {
                        break;
                    }
                    let fresh10 = t1_line_ptr;
                    t1_line_ptr = t1_line_ptr.offset(1);
                    *fresh10 = edecrypt(t1_getbyte() as byte) as t1_line_entry;
                }
            }
            c = t1_getbyte();
        }
        if t1_line_array.is_null() {
            t1_line_limit = 0x10 as size_t;
            if 2 as ::core::ffi::c_int as ::core::ffi::c_uint > t1_line_limit as ::core::ffi::c_uint
            {
                t1_line_limit = 2 as size_t;
            }
            t1_line_array = xmalloc(
                t1_line_limit.wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
            ) as *mut t1_line_entry;
            t1_line_ptr = t1_line_array;
        } else if (t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long
            + 2 as ::core::ffi::c_long) as ::core::ffi::c_uint
            > t1_line_limit as ::core::ffi::c_uint
        {
            last_ptr_index =
                t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long as size_t;
            t1_line_limit = t1_line_limit.wrapping_mul(2 as size_t);
            if (t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long
                + 2 as ::core::ffi::c_long) as ::core::ffi::c_uint
                > t1_line_limit as ::core::ffi::c_uint
            {
                t1_line_limit = (t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long
                    + 2 as ::core::ffi::c_long) as size_t;
            }
            if t1_line_limit as ::core::ffi::c_uint > INT_MAX as ::core::ffi::c_uint {
                crate::utils::pdftex_fail_args(
                    b"t1_line_array exceeds size limit\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                );
            }
            t1_line_array = xrealloc(
                t1_line_array as address,
                t1_line_limit.wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
            ) as *mut t1_line_entry;
            t1_line_ptr = t1_line_array.offset(last_ptr_index as isize);
        }
        if (t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long
            + 2 as ::core::ffi::c_long) as ::core::ffi::c_uint
            > t1_line_limit as ::core::ffi::c_uint
        {
            crate::utils::pdftex_fail_args(
                b"buffer overflow at file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(
                        b"pdftex-rust/generated/backend/writet1.rs\0" as *const u8
                            as *const ::core::ffi::c_char,
                    ),
                    crate::utils::PrintfArg::from(445 as ::core::ffi::c_int),
                ],
            );
        }
        if t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long > 1 as ::core::ffi::c_long
            && *t1_line_ptr.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                != 10 as ::core::ffi::c_int
        {
            let fresh11 = t1_line_ptr;
            t1_line_ptr = t1_line_ptr.offset(1);
            *fresh11 = 10 as t1_line_entry;
        }
        if t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long > 2 as ::core::ffi::c_long
            && *t1_line_ptr.offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == 32 as ::core::ffi::c_int
        {
            *t1_line_ptr.offset(-(2 as ::core::ffi::c_int) as isize) = 10 as t1_line_entry;
            t1_line_ptr = t1_line_ptr.offset(-1);
        }
        *t1_line_ptr = 0 as t1_line_entry;
        if (t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long)
            < 2 as ::core::ffi::c_long
        {
            continue;
        }
        if eexec_scan == eexec_len {
            t1_in_eexec = 1 as ::core::ffi::c_int;
        }
        break;
    }
    t1_buf_ptr = t1_buf_array;
    if t1_buf_array.is_null() {
        t1_buf_limit = t1_line_limit;
        if t1_line_limit as ::core::ffi::c_uint > t1_buf_limit as ::core::ffi::c_uint {
            t1_buf_limit = t1_line_limit;
        }
        t1_buf_array =
            xmalloc(t1_buf_limit.wrapping_mul(::core::mem::size_of::<t1_buf_entry>() as size_t))
                as *mut t1_buf_entry;
        t1_buf_ptr = t1_buf_array;
    } else if (t1_buf_ptr.offset_from(t1_buf_array) as ::core::ffi::c_long as size_t)
        .wrapping_add(t1_line_limit) as ::core::ffi::c_uint
        > t1_buf_limit as ::core::ffi::c_uint
    {
        last_ptr_index = t1_buf_ptr.offset_from(t1_buf_array) as ::core::ffi::c_long as size_t;
        t1_buf_limit = t1_buf_limit.wrapping_mul(2 as size_t);
        if (t1_buf_ptr.offset_from(t1_buf_array) as ::core::ffi::c_long as size_t)
            .wrapping_add(t1_line_limit) as ::core::ffi::c_uint
            > t1_buf_limit as ::core::ffi::c_uint
        {
            t1_buf_limit = (t1_buf_ptr.offset_from(t1_buf_array) as ::core::ffi::c_long as size_t)
                .wrapping_add(t1_line_limit);
        }
        if t1_buf_limit as ::core::ffi::c_uint > INT_MAX as ::core::ffi::c_uint {
            crate::utils::pdftex_fail_args(
                b"t1_buf_array exceeds size limit\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        t1_buf_array = xrealloc(
            t1_buf_array as address,
            t1_buf_limit.wrapping_mul(::core::mem::size_of::<t1_buf_entry>() as size_t),
        ) as *mut t1_buf_entry;
        t1_buf_ptr = t1_buf_array.offset(last_ptr_index as isize);
    }
}
unsafe extern "C" fn t1_putline() {
    let mut p: *mut ::core::ffi::c_char = t1_line_array as *mut ::core::ffi::c_char;
    if t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long <= 1 as ::core::ffi::c_long {
        return;
    }
    if t1_eexec_encrypt != 0 {
        while p < t1_line_ptr {
            let fresh6 = p;
            p = p.offset(1);
            fb_putchar(eencrypt(*fresh6 as byte) as eightbits);
        }
    } else {
        while p < t1_line_ptr {
            let fresh7 = p;
            p = p.offset(1);
            fb_putchar(*fresh7 as eightbits);
        }
    };
}
unsafe extern "C" fn t1_puts(mut s: *const ::core::ffi::c_char) {
    if s != t1_line_array as *const ::core::ffi::c_char {
        strcpy(t1_line_array as *mut ::core::ffi::c_char, s);
    }
    t1_line_ptr = strchr(t1_line_array, 0 as ::core::ffi::c_int) as *mut t1_line_entry;
    t1_putline();
}
unsafe extern "C" fn t1_printf(
    mut fmt: *const ::core::ffi::c_char,
    mut code: ::core::ffi::c_int,
    mut glyph: *mut ::core::ffi::c_char,
) {
    sprintf(t1_line_array as *mut ::core::ffi::c_char, fmt, code, glyph);
    t1_puts(t1_line_array);
}
unsafe extern "C" fn t1_init_params(mut open_name_prefix: *const ::core::ffi::c_char) {
    crate::utils::tex_printf_args(
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(open_name_prefix)],
    );
    crate::utils::tex_printf_args(
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(cur_file_name)],
    );
    t1_lenIV = 4 as ::core::ffi::c_short;
    t1_dr = 55665 as ::core::ffi::c_ushort;
    t1_er = 55665 as ::core::ffi::c_ushort;
    t1_in_eexec = 0 as ::core::ffi::c_int;
    t1_cs = false_0 as boolean;
    t1_scan = true_0 as boolean;
    t1_synthetic = false_0 as boolean;
    t1_eexec_encrypt = false_0 as boolean;
    t1_block_length = 0 as ::core::ffi::c_long;
    t1_check_pfa();
}
unsafe extern "C" fn t1_close_font_file(mut close_name_suffix: *const ::core::ffi::c_char) {
    crate::utils::tex_printf_args(
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(close_name_suffix)],
    );
    xfclose(t1_file, cur_file_name as const_string);
    cur_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn t1_check_block_len(mut decrypt: boolean) {
    let mut l: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    if t1_block_length == 0 as ::core::ffi::c_long {
        return;
    }
    c = t1_getbyte();
    if decrypt != 0 {
        c = edecrypt(c as byte) as ::core::ffi::c_int;
    }
    l = t1_block_length as ::core::ffi::c_int;
    if !(l == 0 as ::core::ffi::c_int
        && (c == 10 as ::core::ffi::c_int || c == 13 as ::core::ffi::c_int))
    {
        crate::utils::pdftex_fail_args(
            b"%i bytes more than expected\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(l + 1 as ::core::ffi::c_int)],
        );
    }
}
unsafe extern "C" fn t1_start_eexec() {
    let mut i: ::core::ffi::c_int = 0;
    if !((*(*fd_cur).fm).type_0 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"t1_start_eexec\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            527 as ::core::ffi::c_int,
            b"is_included(fd_cur->fm)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    t1_length1 = fb_offset() - t1_save_offset;
    t1_save_offset = fb_offset();
    if t1_pfa == 0 {
        t1_check_block_len(false_0);
    }
    t1_line_ptr = t1_line_array;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        edecrypt(t1_getbyte() as byte);
        let fresh42 = t1_line_ptr;
        t1_line_ptr = t1_line_ptr.offset(1);
        *fresh42 = 0 as t1_line_entry;
        i += 1;
    }
    t1_eexec_encrypt = true_0 as boolean;
    t1_putline();
}
unsafe extern "C" fn t1_stop_eexec() {
    let mut c: ::core::ffi::c_int = 0;
    if !((*(*fd_cur).fm).type_0 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"t1_stop_eexec\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            543 as ::core::ffi::c_int,
            b"is_included(fd_cur->fm)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    t1_length2 = fb_offset() - t1_save_offset;
    t1_save_offset = fb_offset();
    t1_eexec_encrypt = false_0 as boolean;
    if t1_pfa == 0 {
        t1_check_block_len(true_0);
    } else {
        c = edecrypt(t1_getbyte() as byte) as ::core::ffi::c_int;
        if !(c == 10 as ::core::ffi::c_int || c == 13 as ::core::ffi::c_int) {
            if last_hexbyte == 0 as ::core::ffi::c_int {
                t1_puts(b"00\0" as *const u8 as *const ::core::ffi::c_char);
            } else {
                crate::utils::pdftex_fail_args(
                    b"unexpected data after eexec\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
        }
    }
    t1_cs = false_0 as boolean;
    t1_in_eexec = 2 as ::core::ffi::c_int;
}
unsafe extern "C" fn t1_modify_fm() {
    let mut a: [::core::ffi::c_float; 6] = [0.; 6];
    let mut i: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = strchr(t1_line_array, '[' as i32);
    if p.is_null() {
        p = strchr(t1_line_array, '{' as i32);
        if p.is_null() {
            p = strchr(t1_line_array, 0 as ::core::ffi::c_int)
                .offset(-(1 as ::core::ffi::c_int as isize));
            if *p as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
                *p = 0 as ::core::ffi::c_char;
            }
            crate::utils::pdftex_fail_args(
                b"FontMatrix: an array expected: `%s'\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(t1_line_array)],
            );
        }
    }
    let fresh41 = p;
    p = p.offset(1);
    c = *fresh41 as ::core::ffi::c_int;
    strncpy(
        t1_buf_array as *mut ::core::ffi::c_char,
        t1_line_array,
        p.offset_from(t1_line_array) as ::core::ffi::c_long as size_t,
    );
    r = t1_buf_array.offset(p.offset_from(t1_line_array) as ::core::ffi::c_long as isize)
        as *mut ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i < 6 as ::core::ffi::c_int {
        a[i as usize] = t1_scan_num(p, &raw mut q);
        p = q;
        i += 1;
    }
    if (*(*fd_cur).fm).slant != 0 as ::core::ffi::c_int {
        a[0 as ::core::ffi::c_int as usize] = (a[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_double
            + a[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                * ((*(*fd_cur).fm).slant as ::core::ffi::c_double * 1E-3f64))
            as ::core::ffi::c_float;
        a[2 as ::core::ffi::c_int as usize] = (a[2 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_double
            + a[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                * ((*(*fd_cur).fm).slant as ::core::ffi::c_double * 1E-3f64))
            as ::core::ffi::c_float;
        a[4 as ::core::ffi::c_int as usize] = (a[4 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_double
            + a[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                * ((*(*fd_cur).fm).slant as ::core::ffi::c_double * 1E-3f64))
            as ::core::ffi::c_float;
    }
    if (*(*fd_cur).fm).extend != 0 as ::core::ffi::c_int {
        a[0 as ::core::ffi::c_int as usize] = (a[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_double
            * ((*(*fd_cur).fm).extend as ::core::ffi::c_double * 1E-3f64))
            as ::core::ffi::c_float;
        a[2 as ::core::ffi::c_int as usize] = (a[2 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_double
            * ((*(*fd_cur).fm).extend as ::core::ffi::c_double * 1E-3f64))
            as ::core::ffi::c_float;
        a[4 as ::core::ffi::c_int as usize] = (a[4 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_double
            * ((*(*fd_cur).fm).extend as ::core::ffi::c_double * 1E-3f64))
            as ::core::ffi::c_float;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 6 as ::core::ffi::c_int {
        sprintf(
            r,
            b"%g \0" as *const u8 as *const ::core::ffi::c_char,
            a[i as usize] as ::core::ffi::c_double,
        );
        r = strchr(r, 0 as ::core::ffi::c_int);
        i += 1;
    }
    if c == '[' as i32 {
        while *p as ::core::ffi::c_int != ']' as i32
            && *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
    } else {
        while *p as ::core::ffi::c_int != '}' as i32
            && *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
    }
    if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        p = strchr(t1_line_array, 0 as ::core::ffi::c_int)
            .offset(-(1 as ::core::ffi::c_int as isize));
        if *p as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
            *p = 0 as ::core::ffi::c_char;
        }
        crate::utils::pdftex_fail_args(
            b"FontMatrix: cannot find the corresponding character to '%c': `%s'\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(c),
                crate::utils::PrintfArg::from(t1_line_array),
            ],
        );
    }
    strcpy(r, p);
    strcpy(t1_line_array as *mut ::core::ffi::c_char, t1_buf_array);
    t1_line_ptr = eol(t1_line_array as *mut ::core::ffi::c_char) as *mut t1_line_entry;
}
unsafe extern "C" fn t1_modify_italic() {
    let mut a: ::core::ffi::c_float = 0.;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*(*fd_cur).fm).slant == 0 as ::core::ffi::c_int {
        return;
    }
    p = strchr(t1_line_array, ' ' as i32);
    strncpy(
        t1_buf_array as *mut ::core::ffi::c_char,
        t1_line_array,
        (p.offset_from(t1_line_array) as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as size_t,
    );
    a = t1_scan_num(p.offset(1 as ::core::ffi::c_int as isize), &raw mut r);
    a = (a as ::core::ffi::c_double
        - atan((*(*fd_cur).fm).slant as ::core::ffi::c_double * 1E-3f64)
            * (180 as ::core::ffi::c_int as ::core::ffi::c_double / M_PI))
        as ::core::ffi::c_float;
    sprintf(
        t1_buf_array.offset(
            (p.offset_from(t1_line_array) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                as isize,
        ),
        b"%g\0" as *const u8 as *const ::core::ffi::c_char,
        a as ::core::ffi::c_double,
    );
    strcpy(strchr(t1_buf_array, 0 as ::core::ffi::c_int), r);
    strcpy(t1_line_array as *mut ::core::ffi::c_char, t1_buf_array);
    t1_line_ptr = eol(t1_line_array as *mut ::core::ffi::c_char) as *mut t1_line_entry;
    (*fd_cur).font_dim[ITALIC_ANGLE_CODE as usize].val =
        zround(a as ::core::ffi::c_double) as ::core::ffi::c_int;
    (*fd_cur).font_dim[ITALIC_ANGLE_CODE as usize].set = true_0 as boolean;
}
unsafe extern "C" fn t1_scan_keys() {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut key: *const key_entry = ::core::ptr::null::<key_entry>();
    if (*(*fd_cur).fm).extend != 0 as ::core::ffi::c_int
        || (*(*fd_cur).fm).slant != 0 as ::core::ffi::c_int
    {
        if strncmp(
            t1_line_array,
            b"/FontMatrix\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"/FontMatrix\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int
        {
            t1_modify_fm();
            return;
        }
        if strncmp(
            t1_line_array,
            b"/ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"/ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int
        {
            t1_modify_italic();
            return;
        }
    }
    if strncmp(
        t1_line_array,
        b"/FontType\0" as *const u8 as *const ::core::ffi::c_char,
        strlen(b"/FontType\0" as *const u8 as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int
    {
        p = t1_line_array
            .offset(strlen(b"FontType\0" as *const u8 as *const ::core::ffi::c_char) as isize)
            .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char;
        i = t1_scan_num(p, ::core::ptr::null_mut::<*mut ::core::ffi::c_char>())
            as ::core::ffi::c_int;
        if i != 1 as ::core::ffi::c_int {
            crate::utils::pdftex_fail_args(
                b"Type%d fonts unsupported by pdfTeX\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(i)],
            );
        }
        return;
    }
    key = &raw const font_key as *const key_entry;
    while (key.offset_from(&raw const font_key as *const key_entry) as ::core::ffi::c_long)
        < FONT_KEYS_NUM as ::core::ffi::c_long
    {
        if *(*key).t1name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '\0' as i32
            && strncmp(
                t1_line_array.offset(1 as ::core::ffi::c_int as isize),
                (*key).t1name,
                strlen((*key).t1name),
            ) == 0 as ::core::ffi::c_int
        {
            break;
        }
        key = key.offset(1);
    }
    if key.offset_from(&raw const font_key as *const key_entry) as ::core::ffi::c_long
        == FONT_KEYS_NUM as ::core::ffi::c_long
    {
        return;
    }
    p = t1_line_array
        .offset(strlen((*key).t1name) as isize)
        .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char;
    if *p as ::core::ffi::c_int == ' ' as i32 {
        p = p.offset(1);
    }
    k = key.offset_from(&raw const font_key as *const key_entry) as ::core::ffi::c_long
        as ::core::ffi::c_int;
    if k == FONTNAME_CODE {
        if *p as ::core::ffi::c_int != '/' as i32 {
            p = strchr(t1_line_array, 0 as ::core::ffi::c_int)
                .offset(-(1 as ::core::ffi::c_int as isize));
            if *p as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
                *p = 0 as ::core::ffi::c_char;
            }
            crate::utils::pdftex_fail_args(
                b"a name expected: `%s'\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(t1_line_array)],
            );
        }
        p = p.offset(1);
        r = p;
        q = t1_buf_array as *mut ::core::ffi::c_char;
        while *p as ::core::ffi::c_int != ' ' as i32
            && *p as ::core::ffi::c_int != 10 as ::core::ffi::c_int
        {
            let fresh39 = p;
            p = p.offset(1);
            let fresh40 = q;
            q = q.offset(1);
            *fresh40 = *fresh39;
        }
        *q = 0 as ::core::ffi::c_char;
        if (*(*fd_cur).fm).slant != 0 as ::core::ffi::c_int {
            sprintf(
                q,
                b"-Slant_%i\0" as *const u8 as *const ::core::ffi::c_char,
                (*(*fd_cur).fm).slant,
            );
            q = strchr(q, 0 as ::core::ffi::c_int);
        }
        if (*(*fd_cur).fm).extend != 0 as ::core::ffi::c_int {
            sprintf(
                q,
                b"-Extend_%i\0" as *const u8 as *const ::core::ffi::c_char,
                (*(*fd_cur).fm).extend,
            );
        }
        if !(*fd_cur).fontname.is_null() {
            free((*fd_cur).fontname as *mut ::core::ffi::c_void);
        }
        (*fd_cur).fontname = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*fd_cur).fontname = xstrdup(t1_buf_array as const_string) as *mut ::core::ffi::c_char;
        if (*(*fd_cur).fm).type_0 as ::core::ffi::c_int & F_SUBSETTED != 0 as ::core::ffi::c_int {
            if !((*(*fd_cur).fm).type_0 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_long
                != 0
            {
                __assert_rtn(
                    b"t1_scan_keys\0" as *const u8 as *const ::core::ffi::c_char,
                    b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
                    715 as ::core::ffi::c_int,
                    b"is_included(fd_cur->fm)\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
            t1_fontname_offset = (fb_offset() as ::core::ffi::c_long
                + r.offset_from(t1_line_array) as ::core::ffi::c_long)
                as integer;
            strcpy(t1_buf_array as *mut ::core::ffi::c_char, p);
            sprintf(
                r,
                b"ABCDEF+%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                (*fd_cur).fontname,
                t1_buf_array,
            );
            t1_line_ptr = eol(r) as *mut t1_line_entry;
        }
        return;
    }
    if (k == STEMV_CODE || k == FONTBBOX1_CODE)
        && (*p as ::core::ffi::c_int == '[' as i32 || *p as ::core::ffi::c_int == '{' as i32)
    {
        p = p.offset(1);
    }
    if k == FONTBBOX1_CODE {
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            (*fd_cur).font_dim[k as usize].val = t1_scan_num(p, &raw mut r) as ::core::ffi::c_int;
            (*fd_cur).font_dim[k as usize].set = true_0 as boolean;
            p = r;
            i += 1;
            k += 1;
        }
        return;
    }
    (*fd_cur).font_dim[k as usize].val =
        t1_scan_num(p, ::core::ptr::null_mut::<*mut ::core::ffi::c_char>()) as ::core::ffi::c_int;
    (*fd_cur).font_dim[k as usize].set = true_0 as boolean;
}
unsafe extern "C" fn t1_scan_param() {
    static mut lenIV: *const ::core::ffi::c_char =
        b"/lenIV\0" as *const u8 as *const ::core::ffi::c_char;
    if t1_scan == 0 || *t1_line_array as ::core::ffi::c_int != '/' as i32 {
        return;
    }
    if strncmp(t1_line_array, lenIV, strlen(lenIV)) == 0 as ::core::ffi::c_int {
        t1_lenIV = t1_scan_num(
            t1_line_array.offset(strlen(lenIV) as isize),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) as ::core::ffi::c_short;
        if (t1_lenIV as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            crate::utils::pdftex_fail_args(
                b"negative value of lenIV is not supported\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
        }
        return;
    }
    t1_scan_keys();
}
unsafe extern "C" fn copy_glyph_names(
    mut glyph_names: *mut *mut ::core::ffi::c_char,
    mut a: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
) {
    if *glyph_names.offset(b as isize) != &raw mut notdef as *mut ::core::ffi::c_char {
        if !(*glyph_names.offset(b as isize)).is_null() {
            free(*glyph_names.offset(b as isize) as *mut ::core::ffi::c_void);
        }
        let ref mut fresh49 = *glyph_names.offset(b as isize);
        *fresh49 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let ref mut fresh50 = *glyph_names.offset(b as isize);
        *fresh50 = &raw mut notdef as *mut ::core::ffi::c_char;
    }
    if *glyph_names.offset(a as isize) != &raw mut notdef as *mut ::core::ffi::c_char {
        let ref mut fresh51 = *glyph_names.offset(b as isize);
        *fresh51 =
            xstrdup(*glyph_names.offset(a as isize) as const_string) as *mut ::core::ffi::c_char;
    }
}
unsafe extern "C" fn t1_builtin_enc() -> *mut *mut ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0;
    let mut a: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut counter: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut glyph_names: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    glyph_names = xmalloc(
        (256 as size_t).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
    ) as *mut *mut ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        let ref mut fresh43 = *glyph_names.offset(i as isize);
        *fresh43 = &raw mut notdef as *mut ::core::ffi::c_char;
        i += 1;
    }
    if str_suffix(
        t1_line_array,
        t1_line_ptr,
        b"def\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        if sscanf(
            t1_line_array
                .offset(strlen(b"/Encoding\0" as *const u8 as *const ::core::ffi::c_char) as isize),
            b"%255s\0" as *const u8 as *const ::core::ffi::c_char,
            t1_buf_array,
        ) == 1 as ::core::ffi::c_int
            && strcmp(
                t1_buf_array,
                b"StandardEncoding\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            t1_encoding = ENC_STANDARD;
            i = 0 as ::core::ffi::c_int;
            while i < 256 as ::core::ffi::c_int {
                if standard_glyph_names[i as usize]
                    != &raw mut notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char
                {
                    let ref mut fresh44 = *glyph_names.offset(i as isize);
                    *fresh44 =
                        xstrdup(standard_glyph_names[i as usize]) as *mut ::core::ffi::c_char;
                }
                i += 1;
            }
            return glyph_names;
        }
        crate::utils::pdftex_fail_args(
            b"cannot subset font (unknown predefined encoding `%s')\0" as *const u8
                as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(t1_buf_array)],
        );
    }
    t1_encoding = ENC_BUILTIN;
    if strncmp(
        t1_line_array,
        b"/Encoding [\0" as *const u8 as *const ::core::ffi::c_char,
        strlen(b"/Encoding [\0" as *const u8 as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int
        || strncmp(
            t1_line_array,
            b"/Encoding[\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"/Encoding[\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int
    {
        r = strchr(t1_line_array, '[' as i32).offset(1 as ::core::ffi::c_int as isize);
        if *r as ::core::ffi::c_int == ' ' as i32 {
            r = r.offset(1);
        }
        loop {
            while *r as ::core::ffi::c_int == '/' as i32 {
                p = t1_buf_array as *mut ::core::ffi::c_char;
                r = r.offset(1);
                while *r as ::core::ffi::c_int != 32 as ::core::ffi::c_int
                    && *r as ::core::ffi::c_int != 10 as ::core::ffi::c_int
                    && *r as ::core::ffi::c_int != ']' as i32
                    && *r as ::core::ffi::c_int != '/' as i32
                {
                    let fresh45 = r;
                    r = r.offset(1);
                    let fresh46 = p;
                    p = p.offset(1);
                    *fresh46 = *fresh45;
                }
                *p = 0 as ::core::ffi::c_char;
                if *r as ::core::ffi::c_int == ' ' as i32 {
                    r = r.offset(1);
                }
                if counter > 255 as ::core::ffi::c_int {
                    crate::utils::pdftex_fail_args(
                        b"encoding vector contains more than 256 names\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                }
                if strcmp(t1_buf_array, &raw mut notdef as *mut ::core::ffi::c_char)
                    != 0 as ::core::ffi::c_int
                {
                    let ref mut fresh47 = *glyph_names.offset(counter as isize);
                    *fresh47 = xstrdup(t1_buf_array as const_string) as *mut ::core::ffi::c_char;
                }
                counter += 1;
            }
            if *r as ::core::ffi::c_int != 10 as ::core::ffi::c_int
                && *r as ::core::ffi::c_int != '%' as i32
            {
                if strncmp(
                    r,
                    b"] def\0" as *const u8 as *const ::core::ffi::c_char,
                    strlen(b"] def\0" as *const u8 as *const ::core::ffi::c_char),
                ) == 0 as ::core::ffi::c_int
                    || strncmp(
                        r,
                        b"] readonly def\0" as *const u8 as *const ::core::ffi::c_char,
                        strlen(b"] readonly def\0" as *const u8 as *const ::core::ffi::c_char),
                    ) == 0 as ::core::ffi::c_int
                {
                    break;
                }
                r = strchr(t1_line_array, 0 as ::core::ffi::c_int)
                    .offset(-(1 as ::core::ffi::c_int as isize));
                if *r as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
                    *r = 0 as ::core::ffi::c_char;
                }
                crate::utils::pdftex_fail_args(
                    b"a name or `] def' or `] readonly def' expected: `%s'\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(t1_line_array)],
                );
            } else {
                t1_getline();
                r = t1_line_array as *mut ::core::ffi::c_char;
            }
        }
    } else {
        p = strchr(t1_line_array, 10 as ::core::ffi::c_int);
        loop {
            if *p as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
                t1_getline();
                p = t1_line_array as *mut ::core::ffi::c_char;
            }
            if sscanf(
                p,
                b"dup %i%255s put\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut i,
                t1_buf_array,
            ) == 2 as ::core::ffi::c_int
                && *t1_buf_array as ::core::ffi::c_int == '/' as i32
                && (i >= 0 as ::core::ffi::c_int && i < 256 as ::core::ffi::c_int)
            {
                if strcmp(
                    t1_buf_array.offset(1 as ::core::ffi::c_int as isize),
                    &raw mut notdef as *mut ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                {
                    let ref mut fresh48 = *glyph_names.offset(i as isize);
                    *fresh48 = xstrdup(
                        t1_buf_array.offset(1 as ::core::ffi::c_int as isize) as const_string
                    ) as *mut ::core::ffi::c_char;
                }
                p = strstr(p, b" put\0" as *const u8 as *const ::core::ffi::c_char);
                if p.is_null() {
                    crate::utils::pdftex_fail_args(
                        b"invalid pfb, no put found in dup\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                }
                p = p.offset(strlen(b" put\0" as *const u8 as *const ::core::ffi::c_char) as isize);
                if *p as ::core::ffi::c_int == ' ' as i32 {
                    p = p.offset(1);
                }
            } else if sscanf(
                p,
                b"dup dup %i exch %i get put\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut b,
                &raw mut a,
            ) == 2 as ::core::ffi::c_int
                && (a >= 0 as ::core::ffi::c_int && a < 256 as ::core::ffi::c_int)
                && (b >= 0 as ::core::ffi::c_int && b < 256 as ::core::ffi::c_int)
            {
                copy_glyph_names(glyph_names, a, b);
                p = strstr(p, b" get put\0" as *const u8 as *const ::core::ffi::c_char);
                if p.is_null() {
                    crate::utils::pdftex_fail_args(
                        b"invalid pfb, no get put found in dup dup\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                }
                p = p.offset(
                    strlen(b" get put\0" as *const u8 as *const ::core::ffi::c_char) as isize,
                );
                if *p as ::core::ffi::c_int == ' ' as i32 {
                    p = p.offset(1);
                }
            } else if sscanf(
                p,
                b"dup dup %i %i getinterval %i exch putinterval\0" as *const u8
                    as *const ::core::ffi::c_char,
                &raw mut a,
                &raw mut c,
                &raw mut b,
            ) == 3 as ::core::ffi::c_int
                && (a >= 0 as ::core::ffi::c_int && a < 256 as ::core::ffi::c_int)
                && (b >= 0 as ::core::ffi::c_int && b < 256 as ::core::ffi::c_int)
                && (c >= 0 as ::core::ffi::c_int && c < 256 as ::core::ffi::c_int)
            {
                i = 0 as ::core::ffi::c_int;
                while i < c {
                    copy_glyph_names(glyph_names, a + i, b + i);
                    i += 1;
                }
                p = strstr(
                    p,
                    b" putinterval\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if p.is_null() {
                    crate::utils::pdftex_fail_args(
                        b"invalid pfb, no putinterval found in dup dup\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                }
                p = p.offset(
                    strlen(b" putinterval\0" as *const u8 as *const ::core::ffi::c_char) as isize,
                );
                if *p as ::core::ffi::c_int == ' ' as i32 {
                    p = p.offset(1);
                }
            } else if (p == t1_line_array
                || p > t1_line_array
                    && *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                        == ' ' as i32)
                && strcmp(p, b"def\n\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
            {
                return glyph_names;
            } else {
                while *p as ::core::ffi::c_int != ' ' as i32
                    && *p as ::core::ffi::c_int != 10 as ::core::ffi::c_int
                {
                    p = p.offset(1);
                }
                if *p as ::core::ffi::c_int == ' ' as i32 {
                    p = p.offset(1);
                }
            }
        }
    }
    return glyph_names;
}
unsafe extern "C" fn t1_check_end() {
    if feof(t1_file) != 0 {
        return;
    }
    t1_getline();
    if strncmp(
        t1_line_array,
        b"{restore}\0" as *const u8 as *const ::core::ffi::c_char,
        strlen(b"{restore}\0" as *const u8 as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int
    {
        t1_putline();
    }
}
unsafe extern "C" fn t1_open_fontfile(mut open_name_prefix: *const ::core::ffi::c_char) -> boolean {
    let mut ff: *mut ff_entry = ::core::ptr::null_mut::<ff_entry>();
    ff = check_ff_exist(
        (*(*fd_cur).fm).ff_name,
        ((*(*fd_cur).fm).type_0 as ::core::ffi::c_int & F_TRUETYPE != 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int,
    );
    if !(*ff).ff_path.is_null() {
        cur_file_name = (*ff).ff_path;
        t1_file = xfopen(cur_file_name as const_string, FOPEN_RBIN_MODE.as_ptr());
        recorder_record_input((*ff).ff_path as const_string);
    } else {
        cur_file_name = (*(*fd_cur).fm).ff_name;
        zpackfilename(maketexstring(cur_file_name), getnullstr(), getnullstr());
        crate::utils::pdftex_fail_args(
            b"cannot open Type 1 font file for reading\0" as *const u8
                as *const ::core::ffi::c_char,
            &[],
        );
    }
    t1_init_params(open_name_prefix);
    return true_0;
}
unsafe extern "C" fn t1_include() {
    loop {
        t1_getline();
        t1_scan_param();
        t1_putline();
        if !(t1_in_eexec == 0 as ::core::ffi::c_int) {
            break;
        }
    }
    t1_start_eexec();
    loop {
        t1_getline();
        t1_scan_param();
        t1_putline();
        if !strstr(
            t1_line_array,
            &raw mut charstringname as *mut ::core::ffi::c_char,
        )
        .is_null()
            || strncmp(
                t1_line_array,
                b"/Subrs\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"/Subrs\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0 as ::core::ffi::c_int
        {
            break;
        }
    }
    t1_cs = true_0 as boolean;
    loop {
        t1_getline();
        t1_putline();
        if !(str_suffix(
            t1_line_array,
            t1_line_ptr,
            b"mark currentfile closefile\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0)
        {
            break;
        }
    }
    t1_stop_eexec();
    t1_length3 = (if fixedcontent != 0 {
        fb_offset() as ::core::ffi::c_int - t1_save_offset as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as integer;
}
unsafe extern "C" fn check_cs_token_pair() -> *mut *const ::core::ffi::c_char {
    let mut p: *mut *const ::core::ffi::c_char = &raw mut cs_token_pairs_list
        as *mut [*const ::core::ffi::c_char; 2]
        as *mut *const ::core::ffi::c_char;
    while !(*p.offset(0 as ::core::ffi::c_int as isize)).is_null() {
        if strncmp(
            t1_buf_array,
            *p.offset(0 as ::core::ffi::c_int as isize),
            strlen(*p.offset(0 as ::core::ffi::c_int as isize)),
        ) == 0 as ::core::ffi::c_int
            && str_suffix(
                t1_buf_array,
                t1_buf_ptr,
                *p.offset(1 as ::core::ffi::c_int as isize),
            ) != 0
        {
            return p;
        }
        p = p.offset(1);
    }
    return ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
}
unsafe extern "C" fn cs_store(mut is_subr: boolean) {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ptr: *mut cs_entry = ::core::ptr::null_mut::<cs_entry>();
    let mut subr: ::core::ffi::c_int = 0;
    p = t1_line_array as *mut ::core::ffi::c_char;
    t1_buf_ptr = t1_buf_array;
    while *p as ::core::ffi::c_int != ' ' as i32 {
        let fresh33 = p;
        p = p.offset(1);
        let fresh34 = t1_buf_ptr;
        t1_buf_ptr = t1_buf_ptr.offset(1);
        *fresh34 = *fresh33 as t1_buf_entry;
    }
    *t1_buf_ptr = 0 as t1_buf_entry;
    if is_subr != 0 {
        subr = t1_scan_num(
            p.offset(1 as ::core::ffi::c_int as isize),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) as ::core::ffi::c_int;
        if subr >= subr_size || subr < 0 as ::core::ffi::c_int {
            crate::utils::pdftex_fail_args(
                b"Subrs array: entry index out of range (%i)\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(subr)],
            );
        }
        ptr = subr_tab.offset(subr as isize);
    } else {
        let fresh35 = cs_ptr;
        cs_ptr = cs_ptr.offset(1);
        ptr = fresh35;
        if cs_ptr.offset_from(cs_tab) as ::core::ffi::c_long > cs_size as ::core::ffi::c_long {
            crate::utils::pdftex_fail_args(
                b"CharStrings dict: more entries than dict size (%i)\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(cs_size)],
            );
        }
        if strcmp(
            t1_buf_array.offset(1 as ::core::ffi::c_int as isize),
            &raw mut notdef as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*ptr).name = &raw mut notdef as *mut ::core::ffi::c_char;
        } else {
            (*ptr).name =
                xstrdup(t1_buf_array.offset(1 as ::core::ffi::c_int as isize) as const_string)
                    as *mut ::core::ffi::c_char;
        }
    }
    memcpy(
        t1_buf_array as *mut ::core::ffi::c_void,
        t1_line_array
            .offset(cs_start as isize)
            .offset(-(4 as ::core::ffi::c_int as isize)) as *const ::core::ffi::c_void,
        (t1_cslen as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as ::core::ffi::c_uint as size_t,
    );
    p = t1_line_array
        .offset(cs_start as isize)
        .offset(t1_cslen as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char;
    t1_buf_ptr = t1_buf_array
        .offset(t1_cslen as ::core::ffi::c_int as isize)
        .offset(4 as ::core::ffi::c_int as isize);
    while *p as ::core::ffi::c_int != 10 as ::core::ffi::c_int {
        let fresh36 = p;
        p = p.offset(1);
        let fresh37 = t1_buf_ptr;
        t1_buf_ptr = t1_buf_ptr.offset(1);
        *fresh37 = *fresh36 as t1_buf_entry;
    }
    let fresh38 = t1_buf_ptr;
    t1_buf_ptr = t1_buf_ptr.offset(1);
    *fresh38 = 10 as t1_buf_entry;
    if is_subr != 0 && cs_token_pair.is_null() {
        cs_token_pair = check_cs_token_pair();
    }
    (*ptr).len =
        t1_buf_ptr.offset_from(t1_buf_array) as ::core::ffi::c_long as ::core::ffi::c_ushort;
    (*ptr).cslen = t1_cslen;
    (*ptr).data =
        xmalloc(((*ptr).len as size_t).wrapping_mul(::core::mem::size_of::<byte>() as size_t))
            as *mut byte;
    memcpy(
        (*ptr).data as *mut ::core::ffi::c_void,
        t1_buf_array as *const ::core::ffi::c_void,
        (*ptr).len as size_t,
    );
    (*ptr).valid = true_0 as boolean;
}
static mut stack_ptr: *mut integer = unsafe { &raw const cc_stack as *mut integer };
static mut cc_stack: [integer; 24] = [0; 24];
static mut cc_tab: [cc_entry; 66] = [cc_entry {
    nargs: 0,
    bottom: 0,
    clear: 0,
    valid: 0,
}; 66];
static mut is_cc_init: boolean = false_0;
unsafe extern "C" fn cc_init() {
    let mut i: ::core::ffi::c_int = 0;
    if is_cc_init != 0 {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < CS_MAX {
        cc_tab[i as usize].valid = false_0 as boolean;
        i += 1;
    }
    cc_tab[1 as ::core::ffi::c_int as usize].nargs = 2 as byte;
    cc_tab[1 as ::core::ffi::c_int as usize].bottom = 1 as ::core::ffi::c_int as boolean;
    cc_tab[1 as ::core::ffi::c_int as usize].clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab[1 as ::core::ffi::c_int as usize].valid = true_0 as boolean;
    cc_tab[3 as ::core::ffi::c_int as usize].nargs = 2 as byte;
    cc_tab[3 as ::core::ffi::c_int as usize].bottom = 1 as ::core::ffi::c_int as boolean;
    cc_tab[3 as ::core::ffi::c_int as usize].clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab[3 as ::core::ffi::c_int as usize].valid = true_0 as boolean;
    cc_tab[4 as ::core::ffi::c_int as usize].nargs = 1 as byte;
    cc_tab[4 as ::core::ffi::c_int as usize].bottom = 1 as ::core::ffi::c_int as boolean;
    cc_tab[4 as ::core::ffi::c_int as usize].clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab[4 as ::core::ffi::c_int as usize].valid = true_0 as boolean;
    cc_tab[5 as ::core::ffi::c_int as usize].nargs = 2 as byte;
    cc_tab[5 as ::core::ffi::c_int as usize].bottom = 1 as ::core::ffi::c_int as boolean;
    cc_tab[5 as ::core::ffi::c_int as usize].clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab[5 as ::core::ffi::c_int as usize].valid = true_0 as boolean;
    cc_tab[6 as ::core::ffi::c_int as usize].nargs = 1 as byte;
    cc_tab[6 as ::core::ffi::c_int as usize].bottom = 1 as ::core::ffi::c_int as boolean;
    cc_tab[6 as ::core::ffi::c_int as usize].clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab[6 as ::core::ffi::c_int as usize].valid = true_0 as boolean;
    cc_tab[7 as ::core::ffi::c_int as usize].nargs = 1 as byte;
    cc_tab[7 as ::core::ffi::c_int as usize].bottom = 1 as ::core::ffi::c_int as boolean;
    cc_tab[7 as ::core::ffi::c_int as usize].clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab[7 as ::core::ffi::c_int as usize].valid = true_0 as boolean;
    cc_tab[8 as ::core::ffi::c_int as usize].nargs = 6 as byte;
    cc_tab[8 as ::core::ffi::c_int as usize].bottom = 1 as ::core::ffi::c_int as boolean;
    cc_tab[8 as ::core::ffi::c_int as usize].clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab[8 as ::core::ffi::c_int as usize].valid = true_0 as boolean;
    cc_tab[9 as ::core::ffi::c_int as usize].nargs = 0 as byte;
    cc_tab[9 as ::core::ffi::c_int as usize].bottom = 0 as ::core::ffi::c_int as boolean;
    cc_tab[9 as ::core::ffi::c_int as usize].clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab[9 as ::core::ffi::c_int as usize].valid = true_0 as boolean;
    cc_tab[10 as ::core::ffi::c_int as usize].nargs = 1 as byte;
    cc_tab[10 as ::core::ffi::c_int as usize].bottom = 0 as ::core::ffi::c_int as boolean;
    cc_tab[10 as ::core::ffi::c_int as usize].clear = 0 as ::core::ffi::c_int as boolean;
    cc_tab[10 as ::core::ffi::c_int as usize].valid = true_0 as boolean;
    cc_tab[11 as ::core::ffi::c_int as usize].nargs = 0 as byte;
    cc_tab[11 as ::core::ffi::c_int as usize].bottom = 0 as ::core::ffi::c_int as boolean;
    cc_tab[11 as ::core::ffi::c_int as usize].clear = 0 as ::core::ffi::c_int as boolean;
    cc_tab[11 as ::core::ffi::c_int as usize].valid = true_0 as boolean;
    cc_tab[13 as ::core::ffi::c_int as usize].nargs = 2 as byte;
    cc_tab[13 as ::core::ffi::c_int as usize].bottom = 1 as ::core::ffi::c_int as boolean;
    cc_tab[13 as ::core::ffi::c_int as usize].clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab[13 as ::core::ffi::c_int as usize].valid = true_0 as boolean;
    cc_tab[14 as ::core::ffi::c_int as usize].nargs = 0 as byte;
    cc_tab[14 as ::core::ffi::c_int as usize].bottom = 0 as ::core::ffi::c_int as boolean;
    cc_tab[14 as ::core::ffi::c_int as usize].clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab[14 as ::core::ffi::c_int as usize].valid = true_0 as boolean;
    cc_tab[21 as ::core::ffi::c_int as usize].nargs = 2 as byte;
    cc_tab[21 as ::core::ffi::c_int as usize].bottom = 1 as ::core::ffi::c_int as boolean;
    cc_tab[21 as ::core::ffi::c_int as usize].clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab[21 as ::core::ffi::c_int as usize].valid = true_0 as boolean;
    cc_tab[22 as ::core::ffi::c_int as usize].nargs = 1 as byte;
    cc_tab[22 as ::core::ffi::c_int as usize].bottom = 1 as ::core::ffi::c_int as boolean;
    cc_tab[22 as ::core::ffi::c_int as usize].clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab[22 as ::core::ffi::c_int as usize].valid = true_0 as boolean;
    cc_tab[30 as ::core::ffi::c_int as usize].nargs = 4 as byte;
    cc_tab[30 as ::core::ffi::c_int as usize].bottom = 1 as ::core::ffi::c_int as boolean;
    cc_tab[30 as ::core::ffi::c_int as usize].clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab[30 as ::core::ffi::c_int as usize].valid = true_0 as boolean;
    cc_tab[31 as ::core::ffi::c_int as usize].nargs = 4 as byte;
    cc_tab[31 as ::core::ffi::c_int as usize].bottom = 1 as ::core::ffi::c_int as boolean;
    cc_tab[31 as ::core::ffi::c_int as usize].clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab[31 as ::core::ffi::c_int as usize].valid = true_0 as boolean;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
        .nargs = 0 as byte;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
        .bottom = 0 as ::core::ffi::c_int as boolean;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
        .clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
        .valid = true_0 as boolean;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
        .nargs = 6 as byte;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
        .bottom = 1 as ::core::ffi::c_int as boolean;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
        .clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
        .valid = true_0 as boolean;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
        .nargs = 6 as byte;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
        .bottom = 1 as ::core::ffi::c_int as boolean;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
        .clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
        .valid = true_0 as boolean;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as usize]
        .nargs = 5 as byte;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as usize]
        .bottom = 1 as ::core::ffi::c_int as boolean;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as usize]
        .clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as usize]
        .valid = true_0 as boolean;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as usize]
        .nargs = 4 as byte;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as usize]
        .bottom = 1 as ::core::ffi::c_int as boolean;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as usize]
        .clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as usize]
        .valid = true_0 as boolean;
    cc_tab[(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
        as usize]
        .nargs = 2 as byte;
    cc_tab[(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
        as usize]
        .bottom = 0 as ::core::ffi::c_int as boolean;
    cc_tab[(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
        as usize]
        .clear = 0 as ::core::ffi::c_int as boolean;
    cc_tab[(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
        as usize]
        .valid = true_0 as boolean;
    cc_tab[(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 16 as ::core::ffi::c_int)
        as usize]
        .nargs = 0 as byte;
    cc_tab[(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 16 as ::core::ffi::c_int)
        as usize]
        .bottom = 0 as ::core::ffi::c_int as boolean;
    cc_tab[(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 16 as ::core::ffi::c_int)
        as usize]
        .clear = 0 as ::core::ffi::c_int as boolean;
    cc_tab[(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 16 as ::core::ffi::c_int)
        as usize]
        .valid = true_0 as boolean;
    cc_tab[(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 17 as ::core::ffi::c_int)
        as usize]
        .nargs = 0 as byte;
    cc_tab[(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 17 as ::core::ffi::c_int)
        as usize]
        .bottom = 0 as ::core::ffi::c_int as boolean;
    cc_tab[(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 17 as ::core::ffi::c_int)
        as usize]
        .clear = 0 as ::core::ffi::c_int as boolean;
    cc_tab[(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 17 as ::core::ffi::c_int)
        as usize]
        .valid = true_0 as boolean;
    cc_tab[(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 33 as ::core::ffi::c_int)
        as usize]
        .nargs = 2 as byte;
    cc_tab[(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 33 as ::core::ffi::c_int)
        as usize]
        .bottom = 1 as ::core::ffi::c_int as boolean;
    cc_tab[(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 33 as ::core::ffi::c_int)
        as usize]
        .clear = 1 as ::core::ffi::c_int as boolean;
    cc_tab[(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 33 as ::core::ffi::c_int)
        as usize]
        .valid = true_0 as boolean;
    is_cc_init = true_0 as boolean;
}
unsafe extern "C" fn cs_fail(
    mut cs_name: *const ::core::ffi::c_char,
    mut subr: ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut arg1: ::core::ffi::c_int,
    mut arg2: ::core::ffi::c_int,
) {
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    sprintf(&raw mut buf as *mut ::core::ffi::c_char, fmt, arg1, arg2);
    if cs_name.is_null() {
        crate::utils::pdftex_fail_args(
            b"Subr (%i): %s\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(subr),
                crate::utils::PrintfArg::from(&raw mut buf as *mut ::core::ffi::c_char),
            ],
        );
    } else {
        crate::utils::pdftex_fail_args(
            b"CharString (/%s): %s\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(cs_name),
                crate::utils::PrintfArg::from(&raw mut buf as *mut ::core::ffi::c_char),
            ],
        );
    };
}
unsafe extern "C" fn append_cs_return(mut ptr: *mut cs_entry) {
    let mut cr: ::core::ffi::c_ushort = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut p: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut q: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut data: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut new_data: *mut byte = ::core::ptr::null_mut::<byte>();
    if !(!ptr.is_null() && (*ptr).valid != 0 && (*ptr).used != 0) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"append_cs_return\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            1108 as ::core::ffi::c_int,
            b"ptr != NULL && ptr->valid && ptr->used\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    p = t1_buf_array as *mut byte;
    data = (*ptr).data.offset(4 as ::core::ffi::c_int as isize);
    cr = 4330 as ::core::ffi::c_ushort;
    i = 0 as ::core::ffi::c_int;
    while i < (*ptr).cslen as ::core::ffi::c_int {
        let fresh29 = data;
        data = data.offset(1);
        let fresh30 = p;
        p = p.offset(1);
        *fresh30 = cdecrypt(*fresh29, &raw mut cr);
        i += 1;
    }
    *p = CS_RETURN as byte;
    new_data = xmalloc(
        (((*ptr).len as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<byte>() as size_t),
    ) as *mut byte;
    memcpy(
        new_data as *mut ::core::ffi::c_void,
        (*ptr).data as *const ::core::ffi::c_void,
        4 as size_t,
    );
    p = new_data.offset(4 as ::core::ffi::c_int as isize);
    q = t1_buf_array as *mut byte;
    cr = 4330 as ::core::ffi::c_ushort;
    i = 0 as ::core::ffi::c_int;
    while i < (*ptr).cslen as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
        let fresh31 = q;
        q = q.offset(1);
        let fresh32 = p;
        p = p.offset(1);
        *fresh32 = cencrypt(*fresh31, &raw mut cr);
        i += 1;
    }
    memcpy(
        p as *mut ::core::ffi::c_void,
        (*ptr)
            .data
            .offset(4 as ::core::ffi::c_int as isize)
            .offset((*ptr).cslen as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        ((*ptr).len as ::core::ffi::c_int
            - (*ptr).cslen as ::core::ffi::c_int
            - 4 as ::core::ffi::c_int) as size_t,
    );
    if !(*ptr).data.is_null() {
        free((*ptr).data as *mut ::core::ffi::c_void);
    }
    (*ptr).data = ::core::ptr::null_mut::<byte>();
    (*ptr).data = new_data;
    (*ptr).len = (*ptr).len.wrapping_add(1);
    (*ptr).cslen = (*ptr).cslen.wrapping_add(1);
}
unsafe extern "C" fn cs_mark(
    mut cs_name: *const ::core::ffi::c_char,
    mut subr: ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut data: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut i: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut cs_len: ::core::ffi::c_int = 0;
    let mut last_cmd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut a: integer = 0;
    let mut a1: integer = 0;
    let mut a2: integer = 0;
    let mut cr: ::core::ffi::c_ushort = 0;
    static mut lastargOtherSubr3: integer = 3 as integer;
    let mut ptr: *mut cs_entry = ::core::ptr::null_mut::<cs_entry>();
    let mut cc: *mut cc_entry = ::core::ptr::null_mut::<cc_entry>();
    if cs_name.is_null() {
        if subr >= subr_size || subr < 0 as ::core::ffi::c_int {
            crate::utils::pdftex_fail_args(
                b"Subrs array: entry index out of range (%i)\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(subr)],
            );
        }
        ptr = subr_tab.offset(subr as isize);
        if (*ptr).valid == 0 {
            return;
        }
    } else if !cs_notdef.is_null()
        && (cs_name == &raw mut notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char
            || strcmp(cs_name, &raw mut notdef as *mut ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int)
    {
        ptr = cs_notdef;
    } else {
        ptr = cs_tab;
        while ptr < cs_ptr {
            if strcmp((*ptr).name, cs_name) == 0 as ::core::ffi::c_int {
                break;
            }
            ptr = ptr.offset(1);
        }
        if ptr == cs_ptr {
            crate::utils::pdftex_warn_args(
                b"glyph `%s' undefined\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(cs_name)],
            );
            return;
        }
        if (*ptr).name == &raw mut notdef as *mut ::core::ffi::c_char {
            cs_notdef = ptr;
        }
    }
    if (*ptr).valid == 0 || (*ptr).used != 0 && !cs_name.is_null() {
        return;
    }
    (*ptr).used = true_0 as boolean;
    cr = 4330 as ::core::ffi::c_ushort;
    cs_len = (*ptr).cslen as ::core::ffi::c_int;
    data = (*ptr).data.offset(4 as ::core::ffi::c_int as isize);
    i = 0 as ::core::ffi::c_int;
    while i < t1_lenIV as ::core::ffi::c_int {
        let fresh17 = data;
        data = data.offset(1);
        cdecrypt(*fresh17, &raw mut cr);
        i += 1;
        cs_len -= 1;
    }
    loop {
        if !(cs_len > 0 as ::core::ffi::c_int) {
            current_block = 1852451392920375136;
            break;
        }
        cs_len -= 1;
        let fresh18 = data;
        data = data.offset(1);
        b = cdecrypt(*fresh18, &raw mut cr) as ::core::ffi::c_int;
        if b >= 32 as ::core::ffi::c_int {
            if b <= 246 as ::core::ffi::c_int {
                a = (b - 139 as ::core::ffi::c_int) as integer;
            } else if b <= 250 as ::core::ffi::c_int {
                cs_len -= 1;
                let fresh19 = data;
                data = data.offset(1);
                a = (((b - 247 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    + 108 as ::core::ffi::c_int
                    + cdecrypt(*fresh19, &raw mut cr) as ::core::ffi::c_int)
                    as integer;
            } else if b <= 254 as ::core::ffi::c_int {
                cs_len -= 1;
                let fresh20 = data;
                data = data.offset(1);
                a = (-((b - 251 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    - 108 as ::core::ffi::c_int
                    - cdecrypt(*fresh20, &raw mut cr) as ::core::ffi::c_int)
                    as integer;
            } else {
                cs_len -= 4 as ::core::ffi::c_int;
                let fresh21 = data;
                data = data.offset(1);
                a = ((cdecrypt(*fresh21, &raw mut cr) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int)
                    << 24 as ::core::ffi::c_int) as integer;
                let fresh22 = data;
                data = data.offset(1);
                a |= (cdecrypt(*fresh22, &raw mut cr) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int)
                    << 16 as ::core::ffi::c_int;
                let fresh23 = data;
                data = data.offset(1);
                a |= (cdecrypt(*fresh23, &raw mut cr) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int;
                let fresh24 = data;
                data = data.offset(1);
                a |= (cdecrypt(*fresh24, &raw mut cr) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int;
                if ::core::mem::size_of::<integer>() as usize > 4 as usize
                    && a as ::core::ffi::c_uint & 0x80000000 as ::core::ffi::c_uint != 0
                {
                    a |= !(0x7fffffff as ::core::ffi::c_int);
                }
            }
            let fresh25 = stack_ptr;
            stack_ptr = stack_ptr.offset(1);
            *fresh25 = a;
        } else {
            if b == CS_ESCAPE {
                let fresh26 = data;
                data = data.offset(1);
                b = cdecrypt(*fresh26, &raw mut cr) as ::core::ffi::c_int + CS_1BYTE_MAX;
                cs_len -= 1;
            }
            if b >= CS_MAX {
                cs_fail(
                    cs_name,
                    subr,
                    b"command value out of range: %i\0" as *const u8 as *const ::core::ffi::c_char,
                    b,
                    0 as ::core::ffi::c_int,
                );
                current_block = 6766438951925407704;
                break;
            } else {
                cc = (&raw mut cc_tab as *mut cc_entry).offset(b as isize);
                if (*cc).valid == 0 {
                    cs_fail(
                        cs_name,
                        subr,
                        b"command not valid: %i\0" as *const u8 as *const ::core::ffi::c_char,
                        b,
                        0 as ::core::ffi::c_int,
                    );
                    current_block = 6766438951925407704;
                    break;
                } else {
                    if (*cc).bottom != 0 {
                        if (stack_ptr.offset_from(&raw mut cc_stack as *mut integer)
                            as ::core::ffi::c_long)
                            < (*cc).nargs as ::core::ffi::c_long
                        {
                            cs_fail(
                                cs_name,
                                subr,
                                b"less arguments on stack (%i) than required (%i)\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                stack_ptr.offset_from(&raw mut cc_stack as *mut integer)
                                    as ::core::ffi::c_long
                                    as ::core::ffi::c_int,
                                (*cc).nargs as ::core::ffi::c_int,
                            );
                        } else if stack_ptr.offset_from(&raw mut cc_stack as *mut integer)
                            as ::core::ffi::c_long
                            > (*cc).nargs as ::core::ffi::c_long
                        {
                            cs_fail(
                                cs_name,
                                subr,
                                b"more arguments on stack (%i) than required (%i)\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                stack_ptr.offset_from(&raw mut cc_stack as *mut integer)
                                    as ::core::ffi::c_long
                                    as ::core::ffi::c_int,
                                (*cc).nargs as ::core::ffi::c_int,
                            );
                        }
                    }
                    last_cmd = b;
                    match cc.offset_from(&raw mut cc_tab as *mut cc_entry) as ::core::ffi::c_long {
                        10 => {
                            a1 = if -(1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                                *stack_ptr.offset(-(1 as ::core::ffi::c_int) as isize)
                            } else {
                                *(&raw mut cc_stack as *mut integer)
                                    .offset(-(1 as ::core::ffi::c_int) as isize)
                            };
                            if (stack_ptr.offset_from(&raw mut cc_stack as *mut integer)
                                as ::core::ffi::c_long)
                                < 1 as ::core::ffi::c_long
                            {
                                crate::utils::pdftex_fail_args(
                                    b"CharString: invalid access (%i) to stack (%i entries)\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                    &[
                                        crate::utils::PrintfArg::from(1 as ::core::ffi::c_int),
                                        crate::utils::PrintfArg::from(
                                            stack_ptr.offset_from(&raw mut cc_stack as *mut integer)
                                                as ::core::ffi::c_long
                                                as ::core::ffi::c_int,
                                        ),
                                    ],
                                );
                            } else {
                                stack_ptr = stack_ptr.offset(-(1 as ::core::ffi::c_int as isize));
                                cs_mark(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    a1 as ::core::ffi::c_int,
                                );
                                if !((*subr_tab.offset(a1 as isize)).valid == 0) {
                                    continue;
                                }
                                cs_fail(
                                    cs_name,
                                    subr,
                                    b"cannot call subr (%i)\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    a1,
                                    0 as ::core::ffi::c_int,
                                );
                                current_block = 6766438951925407704;
                                break;
                            }
                        }
                        44 => {
                            if (stack_ptr.offset_from(&raw mut cc_stack as *mut integer)
                                as ::core::ffi::c_long)
                                < 2 as ::core::ffi::c_long
                            {
                                crate::utils::pdftex_fail_args(
                                    b"CharString: invalid access (%i) to stack (%i entries)\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                    &[
                                        crate::utils::PrintfArg::from(2 as ::core::ffi::c_int),
                                        crate::utils::PrintfArg::from(
                                            stack_ptr.offset_from(&raw mut cc_stack as *mut integer)
                                                as ::core::ffi::c_long
                                                as ::core::ffi::c_int,
                                        ),
                                    ],
                                );
                            } else {
                                stack_ptr = stack_ptr.offset(-(2 as ::core::ffi::c_int as isize));
                                let fresh27 = stack_ptr;
                                stack_ptr = stack_ptr.offset(1);
                                *fresh27 = 0 as ::core::ffi::c_int as integer;
                            }
                        }
                        48 => {
                            if (if -(1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                                *stack_ptr.offset(-(1 as ::core::ffi::c_int) as isize)
                            } else {
                                *(&raw mut cc_stack as *mut integer)
                                    .offset(-(1 as ::core::ffi::c_int) as isize)
                            }) == 3 as ::core::ffi::c_int
                            {
                                lastargOtherSubr3 =
                                    if -(3 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                                        *stack_ptr.offset(-(3 as ::core::ffi::c_int) as isize)
                                    } else {
                                        *(&raw mut cc_stack as *mut integer)
                                            .offset(-(3 as ::core::ffi::c_int) as isize)
                                    };
                            }
                            a1 = ((if -(2 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                                *stack_ptr.offset(-(2 as ::core::ffi::c_int) as isize)
                            } else {
                                *(&raw mut cc_stack as *mut integer)
                                    .offset(-(2 as ::core::ffi::c_int) as isize)
                            }) + 2 as ::core::ffi::c_int)
                                as integer;
                            if (stack_ptr.offset_from(&raw mut cc_stack as *mut integer)
                                as ::core::ffi::c_long)
                                < a1 as ::core::ffi::c_long
                            {
                                crate::utils::pdftex_fail_args(
                                    b"CharString: invalid access (%i) to stack (%i entries)\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                    &[
                                        crate::utils::PrintfArg::from(a1),
                                        crate::utils::PrintfArg::from(
                                            stack_ptr.offset_from(&raw mut cc_stack as *mut integer)
                                                as ::core::ffi::c_long
                                                as ::core::ffi::c_int,
                                        ),
                                    ],
                                );
                            } else {
                                stack_ptr = stack_ptr.offset(-(a1 as isize));
                            }
                        }
                        49 => {
                            let fresh28 = stack_ptr;
                            stack_ptr = stack_ptr.offset(1);
                            *fresh28 = lastargOtherSubr3;
                        }
                        38 => {
                            a1 = if (3 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                                *stack_ptr.offset(3 as ::core::ffi::c_int as isize)
                            } else {
                                *(&raw mut cc_stack as *mut integer)
                                    .offset(3 as ::core::ffi::c_int as isize)
                            };
                            a2 = if (4 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                                *stack_ptr.offset(4 as ::core::ffi::c_int as isize)
                            } else {
                                *(&raw mut cc_stack as *mut integer)
                                    .offset(4 as ::core::ffi::c_int as isize)
                            };
                            stack_ptr = &raw mut cc_stack as *mut integer;
                            cs_mark(standard_glyph_names[a1 as usize], 0 as ::core::ffi::c_int);
                            cs_mark(standard_glyph_names[a2 as usize], 0 as ::core::ffi::c_int);
                            if !(*fd_cur).gl_tree.is_null() {
                                avl_probe(
                                    (*fd_cur).gl_tree,
                                    standard_glyph_names[a1 as usize] as *mut ::core::ffi::c_void,
                                );
                                avl_probe(
                                    (*fd_cur).gl_tree,
                                    standard_glyph_names[a2 as usize] as *mut ::core::ffi::c_void,
                                );
                            }
                        }
                        _ => {
                            if (*cc).clear != 0 {
                                stack_ptr = &raw mut cc_stack as *mut integer;
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        6766438951925407704 => {
            stack_ptr = &raw mut cc_stack as *mut integer;
            (*ptr).valid = false_0 as boolean;
            (*ptr).used = false_0 as boolean;
            return;
        }
        _ => {
            if cs_name.is_null() && last_cmd != CS_RETURN {
                crate::utils::pdftex_warn_args(b"last command in subr `%i' is not a RETURN; I will add it now but please consider fixing the font\0"
                        as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(subr)]);
                append_cs_return(ptr);
            }
            return;
        }
    };
}
unsafe extern "C" fn comp_t1_glyphs(
    mut pa: *const ::core::ffi::c_void,
    mut pb: *const ::core::ffi::c_void,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        *(pa as *const *const ::core::ffi::c_char),
        *(pb as *const *const ::core::ffi::c_char),
    );
}
unsafe extern "C" fn create_t1_glyph_tree(
    mut glyph_names: *mut *mut ::core::ffi::c_char,
) -> *mut avl_table {
    let mut i: ::core::ffi::c_int = 0;
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    static mut gl_tree: *mut avl_table = ::core::ptr::null::<avl_table>() as *mut avl_table;
    gl_tree = avl_create(
        Some(
            comp_t1_glyphs
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
            b"create_t1_glyph_tree\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            1299 as ::core::ffi::c_int,
            b"gl_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        if *glyph_names.offset(i as isize) != &raw mut notdef as *mut ::core::ffi::c_char
            && (avl_find(
                gl_tree,
                glyph_names.offset(i as isize) as *mut *mut ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
            ) as *mut *mut ::core::ffi::c_char)
                .is_null()
        {
            aa = avl_probe(
                gl_tree,
                glyph_names.offset(i as isize) as *mut *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
            );
            if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                __assert_rtn(
                    b"create_t1_glyph_tree\0" as *const u8 as *const ::core::ffi::c_char,
                    b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1305 as ::core::ffi::c_int,
                    b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
        }
        i += 1;
    }
    return gl_tree;
}
unsafe extern "C" fn destroy_t1_glyph_tree(mut gl_tree: *mut avl_table) {
    if gl_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"destroy_t1_glyph_tree\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            1313 as ::core::ffi::c_int,
            b"gl_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    avl_destroy(gl_tree, None);
}
unsafe extern "C" fn t1_subset_ascii_part() {
    let mut j: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut glyph: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut gg: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut glyph_names: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut gl_tree: *mut avl_table = ::core::ptr::null_mut::<avl_table>();
    let mut t: avl_traverser = avl_traverser {
        avl_table: ::core::ptr::null_mut::<avl_table>(),
        avl_node: ::core::ptr::null_mut::<avl_node>(),
        avl_stack: [::core::ptr::null_mut::<avl_node>(); 32],
        avl_height: 0,
        avl_generation: 0,
    };
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    if fd_cur.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"t1_subset_ascii_part\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            1326 as ::core::ffi::c_int,
            b"fd_cur != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if (*fd_cur).gl_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"t1_subset_ascii_part\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            1327 as ::core::ffi::c_int,
            b"fd_cur->gl_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    t1_getline();
    while !(strncmp(
        t1_line_array,
        b"/Encoding\0" as *const u8 as *const ::core::ffi::c_char,
        strlen(b"/Encoding\0" as *const u8 as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int)
    {
        t1_scan_param();
        if !(strncmp(
            t1_line_array,
            b"/UniqueID\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"/UniqueID\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int
            && strncmp(
                t1_line_array
                    .offset(strlen(t1_line_array) as isize)
                    .offset(-(4 as ::core::ffi::c_int as isize)),
                b"def\0" as *const u8 as *const ::core::ffi::c_char,
                3 as size_t,
            ) == 0)
        {
            t1_putline();
        }
        t1_getline();
    }
    glyph_names = t1_builtin_enc();
    (*fd_cur).builtin_glyph_names = glyph_names;
    if (*(*fd_cur).fm).type_0 as ::core::ffi::c_int & F_SUBSETTED != 0 as ::core::ffi::c_int {
        if !((*(*fd_cur).fm).type_0 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            __assert_rtn(
                b"t1_subset_ascii_part\0" as *const u8 as *const ::core::ffi::c_char,
                b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
                1339 as ::core::ffi::c_int,
                b"is_included(fd_cur->fm)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        if !(*fd_cur).tx_tree.is_null() {
            avl_t_init(&raw mut t, (*fd_cur).tx_tree);
            p = avl_t_first(&raw mut t, (*fd_cur).tx_tree) as *mut ::core::ffi::c_int;
            while !p.is_null() {
                if (avl_find(
                    (*fd_cur).gl_tree,
                    *glyph_names.offset(*p as isize) as *const ::core::ffi::c_void,
                ) as *mut ::core::ffi::c_char)
                    .is_null()
                {
                    glyph = xstrdup(*glyph_names.offset(*p as isize) as const_string)
                        as *mut ::core::ffi::c_char;
                    aa = avl_probe((*fd_cur).gl_tree, glyph as *mut ::core::ffi::c_void);
                    if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                        __assert_rtn(
                            b"t1_subset_ascii_part\0" as *const u8 as *const ::core::ffi::c_char,
                            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
                            1348 as ::core::ffi::c_int,
                            b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                    };
                }
                p = avl_t_next(&raw mut t) as *mut ::core::ffi::c_int;
            }
        }
        make_subset_tag(fd_cur);
        if !(t1_fontname_offset != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
        {
            __assert_rtn(
                b"t1_subset_ascii_part\0" as *const u8 as *const ::core::ffi::c_char,
                b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
                1353 as ::core::ffi::c_int,
                b"t1_fontname_offset != 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        strncpy(
            fb_array.offset(t1_fontname_offset as isize),
            (*fd_cur).subset_tag,
            6 as size_t,
        );
    }
    if t1_encoding as ::core::ffi::c_uint
        == ENC_STANDARD as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        t1_puts(b"/Encoding StandardEncoding def\n\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        t1_puts(
            b"/Encoding 256 array\n0 1 255 {1 index exch /.notdef put} for\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        gl_tree = create_t1_glyph_tree(glyph_names);
        avl_t_init(&raw mut t, (*fd_cur).gl_tree);
        j = 0 as ::core::ffi::c_int;
        glyph = avl_t_first(&raw mut t, (*fd_cur).gl_tree) as *mut ::core::ffi::c_char;
        while !glyph.is_null() {
            gg = avl_find(gl_tree, &raw mut glyph as *const ::core::ffi::c_void)
                as *mut *mut ::core::ffi::c_char;
            if !gg.is_null() {
                t1_printf(
                    b"dup %i /%s put\n\0" as *const u8 as *const ::core::ffi::c_char,
                    gg.offset_from(glyph_names) as ::core::ffi::c_long as ::core::ffi::c_int,
                    *gg,
                );
                j += 1;
            }
            glyph = avl_t_next(&raw mut t) as *mut ::core::ffi::c_char;
        }
        destroy_t1_glyph_tree(gl_tree);
        if j == 0 as ::core::ffi::c_int {
            t1_puts(b"dup 0 /.notdef put\n\0" as *const u8 as *const ::core::ffi::c_char);
        }
        t1_puts(b"readonly def\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    loop {
        t1_getline();
        t1_scan_param();
        if !(strncmp(
            t1_line_array,
            b"/UniqueID\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"/UniqueID\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int)
        {
            t1_putline();
        }
        if !(t1_in_eexec == 0 as ::core::ffi::c_int) {
            break;
        }
    }
}
unsafe extern "C" fn cs_init() {
    cs_tab = ::core::ptr::null_mut::<cs_entry>();
    cs_ptr = cs_tab;
    cs_dict_end = ::core::ptr::null_mut::<::core::ffi::c_char>();
    cs_dict_start = cs_dict_end;
    cs_size_pos = 0 as ::core::ffi::c_int;
    cs_size = cs_size_pos;
    cs_count = cs_size;
    cs_token_pair = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    subr_tab = ::core::ptr::null_mut::<cs_entry>();
    subr_array_end = ::core::ptr::null_mut::<::core::ffi::c_char>();
    subr_array_start = subr_array_end;
    subr_size_pos = 0 as ::core::ffi::c_int;
    subr_size = subr_size_pos;
    subr_max = subr_size;
}
unsafe extern "C" fn init_cs_entry(mut cs: *mut cs_entry) {
    (*cs).data = ::core::ptr::null_mut::<byte>();
    (*cs).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*cs).len = 0 as ::core::ffi::c_ushort;
    (*cs).cslen = 0 as ::core::ffi::c_ushort;
    (*cs).used = false_0 as boolean;
    (*cs).valid = false_0 as boolean;
}
unsafe extern "C" fn t1_read_subrs() {
    let mut i: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0;
    let mut ptr: *mut cs_entry = ::core::ptr::null_mut::<cs_entry>();
    t1_getline();
    while !(!strstr(
        t1_line_array,
        &raw mut charstringname as *mut ::core::ffi::c_char,
    )
    .is_null()
        || strncmp(
            t1_line_array,
            b"/Subrs\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"/Subrs\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int)
    {
        t1_scan_param();
        if !(strncmp(
            t1_line_array,
            b"/UniqueID\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"/UniqueID\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int)
        {
            t1_putline();
        }
        t1_getline();
    }
    loop {
        t1_cs = true_0 as boolean;
        t1_scan = false_0 as boolean;
        if !(strncmp(
            t1_line_array,
            b"/Subrs\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"/Subrs\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int)
        {
            return;
        }
        subr_size_pos = strlen(b"/Subrs\0" as *const u8 as *const ::core::ffi::c_char)
            .wrapping_add(1 as size_t) as ::core::ffi::c_int;
        subr_size = t1_scan_num(
            t1_line_array.offset(subr_size_pos as isize),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) as ::core::ffi::c_int;
        if subr_size == 0 as ::core::ffi::c_int {
            while strstr(
                t1_line_array,
                &raw mut charstringname as *mut ::core::ffi::c_char,
            )
            .is_null()
            {
                t1_getline();
            }
            return;
        }
        subr_tab = xmalloc(
            (subr_size as size_t).wrapping_mul(::core::mem::size_of::<cs_entry>() as size_t),
        ) as *mut cs_entry;
        ptr = subr_tab;
        while (ptr.offset_from(subr_tab) as ::core::ffi::c_long) < subr_size as ::core::ffi::c_long
        {
            init_cs_entry(ptr);
            ptr = ptr.offset(1);
        }
        subr_array_start = xstrdup(t1_line_array as const_string) as *mut ::core::ffi::c_char;
        t1_getline();
        while t1_cslen != 0 {
            cs_store(true_0);
            t1_getline();
        }
        i = 0 as ::core::ffi::c_int;
        while i < subr_size && i < 4 as ::core::ffi::c_int {
            (*subr_tab.offset(i as isize)).used = true_0 as boolean;
            i += 1;
        }
        s = 0 as ::core::ffi::c_int;
        *t1_buf_array = 0 as t1_buf_entry;
        i = 0 as ::core::ffi::c_int;
        while i < POST_SUBRS_SCAN {
            if !strstr(
                t1_line_array,
                &raw mut charstringname as *mut ::core::ffi::c_char,
            )
            .is_null()
            {
                break;
            }
            s = (s as ::core::ffi::c_long
                + t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long)
                as ::core::ffi::c_int;
            if t1_buf_array.is_null() {
                t1_buf_limit = 0x10 as size_t;
                if s as ::core::ffi::c_uint > t1_buf_limit as ::core::ffi::c_uint {
                    t1_buf_limit = s as size_t;
                }
                t1_buf_array = xmalloc(
                    t1_buf_limit.wrapping_mul(::core::mem::size_of::<t1_buf_entry>() as size_t),
                ) as *mut t1_buf_entry;
                t1_buf_ptr = t1_buf_array;
            } else if (t1_buf_ptr.offset_from(t1_buf_array) as ::core::ffi::c_long
                + s as ::core::ffi::c_long) as ::core::ffi::c_uint
                > t1_buf_limit as ::core::ffi::c_uint
            {
                last_ptr_index =
                    t1_buf_ptr.offset_from(t1_buf_array) as ::core::ffi::c_long as size_t;
                t1_buf_limit = t1_buf_limit.wrapping_mul(2 as size_t);
                if (t1_buf_ptr.offset_from(t1_buf_array) as ::core::ffi::c_long
                    + s as ::core::ffi::c_long) as ::core::ffi::c_uint
                    > t1_buf_limit as ::core::ffi::c_uint
                {
                    t1_buf_limit = (t1_buf_ptr.offset_from(t1_buf_array) as ::core::ffi::c_long
                        + s as ::core::ffi::c_long) as size_t;
                }
                if t1_buf_limit as ::core::ffi::c_uint > INT_MAX as ::core::ffi::c_uint {
                    crate::utils::pdftex_fail_args(
                        b"t1_buf_array exceeds size limit\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                }
                t1_buf_array = xrealloc(
                    t1_buf_array as address,
                    t1_buf_limit.wrapping_mul(::core::mem::size_of::<t1_buf_entry>() as size_t),
                ) as *mut t1_buf_entry;
                t1_buf_ptr = t1_buf_array.offset(last_ptr_index as isize);
            }
            strcat(t1_buf_array as *mut ::core::ffi::c_char, t1_line_array);
            t1_getline();
            i += 1;
        }
        subr_array_end = xstrdup(t1_buf_array as const_string) as *mut ::core::ffi::c_char;
        if !(i == POST_SUBRS_SCAN) {
            break;
        }
        ptr = subr_tab;
        while (ptr.offset_from(subr_tab) as ::core::ffi::c_long) < subr_size as ::core::ffi::c_long
        {
            if (*ptr).valid != 0 {
                if !(*ptr).data.is_null() {
                    free((*ptr).data as *mut ::core::ffi::c_void);
                }
                (*ptr).data = ::core::ptr::null_mut::<byte>();
            }
            ptr = ptr.offset(1);
        }
        if !subr_tab.is_null() {
            free(subr_tab as *mut ::core::ffi::c_void);
        }
        subr_tab = ::core::ptr::null_mut::<cs_entry>();
        if !subr_array_start.is_null() {
            free(subr_array_start as *mut ::core::ffi::c_void);
        }
        subr_array_start = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if !subr_array_end.is_null() {
            free(subr_array_end as *mut ::core::ffi::c_void);
        }
        subr_array_end = ::core::ptr::null_mut::<::core::ffi::c_char>();
        cs_init();
        t1_cs = false_0 as boolean;
        t1_synthetic = true_0 as boolean;
        while !(!strstr(
            t1_line_array,
            &raw mut charstringname as *mut ::core::ffi::c_char,
        )
        .is_null()
            || strncmp(
                t1_line_array,
                b"/Subrs\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"/Subrs\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0 as ::core::ffi::c_int)
        {
            t1_getline();
        }
    }
}
pub const POST_SUBRS_SCAN: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
unsafe extern "C" fn t1_flush_cs(mut is_subr: boolean) {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut return_cs: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut tab: *mut cs_entry = ::core::ptr::null_mut::<cs_entry>();
    let mut end_tab: *mut cs_entry = ::core::ptr::null_mut::<cs_entry>();
    let mut ptr: *mut cs_entry = ::core::ptr::null_mut::<cs_entry>();
    let mut start_line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut line_end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut count: ::core::ffi::c_int = 0;
    let mut size_pos: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_ushort = 0;
    let mut cs_len: ::core::ffi::c_ushort = 0 as ::core::ffi::c_ushort;
    if is_subr != 0 {
        start_line = subr_array_start;
        line_end = subr_array_end;
        size_pos = subr_size_pos;
        tab = subr_tab;
        count = subr_max + 1 as ::core::ffi::c_int;
        end_tab = subr_tab.offset(count as isize);
    } else {
        start_line = cs_dict_start;
        line_end = cs_dict_end;
        size_pos = cs_size_pos;
        tab = cs_tab;
        end_tab = cs_ptr;
        count = cs_count;
    }
    t1_line_ptr = t1_line_array;
    p = start_line;
    while (p.offset_from(start_line) as ::core::ffi::c_long) < size_pos as ::core::ffi::c_long {
        let fresh14 = p;
        p = p.offset(1);
        let fresh15 = t1_line_ptr;
        t1_line_ptr = t1_line_ptr.offset(1);
        *fresh15 = *fresh14 as t1_line_entry;
    }
    while isdigit(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        p = p.offset(1);
    }
    sprintf(
        t1_line_ptr as *mut ::core::ffi::c_char,
        b"%u\0" as *const u8 as *const ::core::ffi::c_char,
        count,
    );
    strcat(t1_line_ptr as *mut ::core::ffi::c_char, p);
    t1_line_ptr = eol(t1_line_array as *mut ::core::ffi::c_char) as *mut t1_line_entry;
    t1_putline();
    if is_subr != 0 {
        cr = 4330 as ::core::ffi::c_ushort;
        cs_len = 0 as ::core::ffi::c_ushort;
        return_cs = xmalloc(
            ((t1_lenIV as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<byte>() as size_t),
        ) as *mut byte;
        cs_len = 0 as ::core::ffi::c_ushort;
        r = return_cs;
        while (cs_len as ::core::ffi::c_int) < t1_lenIV as ::core::ffi::c_int {
            *r = cencrypt(0 as byte, &raw mut cr);
            cs_len = cs_len.wrapping_add(1);
            r = r.offset(1);
        }
        *r = cencrypt(CS_RETURN as byte, &raw mut cr);
        cs_len = cs_len.wrapping_add(1);
    }
    ptr = tab;
    while ptr < end_tab {
        if (*ptr).used != 0 {
            if is_subr != 0 {
                sprintf(
                    t1_line_array as *mut ::core::ffi::c_char,
                    b"dup %lu %u\0" as *const u8 as *const ::core::ffi::c_char,
                    ptr.offset_from(tab) as ::core::ffi::c_long as ::core::ffi::c_ulong,
                    (*ptr).cslen as ::core::ffi::c_int,
                );
            } else {
                sprintf(
                    t1_line_array as *mut ::core::ffi::c_char,
                    b"/%s %u\0" as *const u8 as *const ::core::ffi::c_char,
                    (*ptr).name,
                    (*ptr).cslen as ::core::ffi::c_int,
                );
            }
            p = strchr(t1_line_array, 0 as ::core::ffi::c_int);
            memcpy(
                p as *mut ::core::ffi::c_void,
                (*ptr).data as *const ::core::ffi::c_void,
                (*ptr).len as size_t,
            );
            t1_line_ptr = p.offset((*ptr).len as ::core::ffi::c_int as isize) as *mut t1_line_entry;
            t1_putline();
        } else if is_subr != 0 {
            sprintf(
                t1_line_array as *mut ::core::ffi::c_char,
                b"dup %lu %u%s \0" as *const u8 as *const ::core::ffi::c_char,
                ptr.offset_from(tab) as ::core::ffi::c_long as ::core::ffi::c_ulong,
                cs_len as ::core::ffi::c_int,
                *cs_token_pair.offset(0 as ::core::ffi::c_int as isize),
            );
            p = strchr(t1_line_array, 0 as ::core::ffi::c_int);
            memcpy(
                p as *mut ::core::ffi::c_void,
                return_cs as *const ::core::ffi::c_void,
                cs_len as size_t,
            );
            t1_line_ptr = p.offset(cs_len as ::core::ffi::c_int as isize) as *mut t1_line_entry;
            t1_putline();
            sprintf(
                t1_line_array as *mut ::core::ffi::c_char,
                b" %s\0" as *const u8 as *const ::core::ffi::c_char,
                *cs_token_pair.offset(1 as ::core::ffi::c_int as isize),
            );
            t1_line_ptr = eol(t1_line_array as *mut ::core::ffi::c_char) as *mut t1_line_entry;
            t1_putline();
        }
        if !(*ptr).data.is_null() {
            free((*ptr).data as *mut ::core::ffi::c_void);
        }
        (*ptr).data = ::core::ptr::null_mut::<byte>();
        if (*ptr).name != &raw mut notdef as *mut ::core::ffi::c_char {
            if !(*ptr).name.is_null() {
                free((*ptr).name as *mut ::core::ffi::c_void);
            }
            (*ptr).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        ptr = ptr.offset(1);
    }
    sprintf(
        t1_line_array as *mut ::core::ffi::c_char,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        line_end,
    );
    t1_line_ptr = eol(t1_line_array as *mut ::core::ffi::c_char) as *mut t1_line_entry;
    t1_putline();
    if is_subr != 0 {
        if !return_cs.is_null() {
            free(return_cs as *mut ::core::ffi::c_void);
        }
        return_cs = ::core::ptr::null_mut::<byte>();
    }
    if !tab.is_null() {
        free(tab as *mut ::core::ffi::c_void);
    }
    tab = ::core::ptr::null_mut::<cs_entry>();
    if !start_line.is_null() {
        free(start_line as *mut ::core::ffi::c_void);
    }
    start_line = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !line_end.is_null() {
        free(line_end as *mut ::core::ffi::c_void);
    }
    line_end = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn t1_mark_glyphs() {
    let mut glyph: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut t: avl_traverser = avl_traverser {
        avl_table: ::core::ptr::null_mut::<avl_table>(),
        avl_node: ::core::ptr::null_mut::<avl_node>(),
        avl_stack: [::core::ptr::null_mut::<avl_node>(); 32],
        avl_height: 0,
        avl_generation: 0,
    };
    let mut ptr: *mut cs_entry = ::core::ptr::null_mut::<cs_entry>();
    if t1_synthetic != 0 || (*fd_cur).all_glyphs != 0 {
        if !cs_tab.is_null() {
            ptr = cs_tab;
            while ptr < cs_ptr {
                if (*ptr).valid != 0 {
                    (*ptr).used = true_0 as boolean;
                }
                ptr = ptr.offset(1);
            }
        }
        if !subr_tab.is_null() {
            ptr = subr_tab;
            while (ptr.offset_from(subr_tab) as ::core::ffi::c_long)
                < subr_size as ::core::ffi::c_long
            {
                if (*ptr).valid != 0 {
                    (*ptr).used = true_0 as boolean;
                }
                ptr = ptr.offset(1);
            }
            subr_max = subr_size - 1 as ::core::ffi::c_int;
        }
        return;
    }
    cs_mark(
        &raw mut notdef as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    avl_t_init(&raw mut t, (*fd_cur).gl_tree);
    glyph = avl_t_first(&raw mut t, (*fd_cur).gl_tree) as *mut ::core::ffi::c_char;
    while !glyph.is_null() {
        cs_mark(glyph, 0 as ::core::ffi::c_int);
        glyph = avl_t_next(&raw mut t) as *mut ::core::ffi::c_char;
    }
    if !subr_tab.is_null() {
        subr_max = -(1 as ::core::ffi::c_int);
        ptr = subr_tab;
        while (ptr.offset_from(subr_tab) as ::core::ffi::c_long) < subr_size as ::core::ffi::c_long
        {
            if (*ptr).used != 0
                && ptr.offset_from(subr_tab) as ::core::ffi::c_long
                    > subr_max as ::core::ffi::c_long
            {
                subr_max = ptr.offset_from(subr_tab) as ::core::ffi::c_long as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
        }
    }
}
unsafe extern "C" fn t1_check_unusual_charstring() {
    let mut p: *mut ::core::ffi::c_char = strstr(
        t1_line_array,
        &raw mut charstringname as *mut ::core::ffi::c_char,
    )
    .offset(strlen(&raw mut charstringname as *mut ::core::ffi::c_char) as isize);
    let mut i: ::core::ffi::c_int = 0;
    if sscanf(
        p,
        b"%i\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut i,
    ) != 1 as ::core::ffi::c_int
    {
        strcpy(t1_buf_array as *mut ::core::ffi::c_char, t1_line_array);
        *strchr(t1_buf_array, 0 as ::core::ffi::c_int)
            .offset(-(1 as ::core::ffi::c_int as isize)) = ' ' as i32 as ::core::ffi::c_char;
        t1_getline();
        if t1_buf_array.is_null() {
            t1_buf_limit = 0x10 as size_t;
            if strlen(t1_line_array).wrapping_add(
                (if !t1_buf_array.is_null() {
                    strlen(t1_buf_array).wrapping_add(1 as size_t)
                } else {
                    0 as size_t
                }),
            ) as ::core::ffi::c_uint
                > t1_buf_limit as ::core::ffi::c_uint
            {
                t1_buf_limit = strlen(t1_line_array).wrapping_add(
                    (if !t1_buf_array.is_null() {
                        strlen(t1_buf_array).wrapping_add(1 as size_t)
                    } else {
                        0 as size_t
                    }),
                );
            }
            t1_buf_array = xmalloc(
                t1_buf_limit.wrapping_mul(::core::mem::size_of::<t1_buf_entry>() as size_t),
            ) as *mut t1_buf_entry;
            t1_buf_ptr = t1_buf_array;
        } else if (t1_buf_ptr.offset_from(t1_buf_array) as ::core::ffi::c_long as size_t)
            .wrapping_add(strlen(t1_line_array).wrapping_add(
                (if !t1_buf_array.is_null() {
                    strlen(t1_buf_array).wrapping_add(1 as size_t)
                } else {
                    0 as size_t
                }),
            )) as ::core::ffi::c_uint
            > t1_buf_limit as ::core::ffi::c_uint
        {
            last_ptr_index = t1_buf_ptr.offset_from(t1_buf_array) as ::core::ffi::c_long as size_t;
            t1_buf_limit = t1_buf_limit.wrapping_mul(2 as size_t);
            if (t1_buf_ptr.offset_from(t1_buf_array) as ::core::ffi::c_long as size_t).wrapping_add(
                strlen(t1_line_array).wrapping_add(
                    (if !t1_buf_array.is_null() {
                        strlen(t1_buf_array).wrapping_add(1 as size_t)
                    } else {
                        0 as size_t
                    }),
                ),
            ) as ::core::ffi::c_uint
                > t1_buf_limit as ::core::ffi::c_uint
            {
                t1_buf_limit = (t1_buf_ptr.offset_from(t1_buf_array) as ::core::ffi::c_long
                    as size_t)
                    .wrapping_add(strlen(t1_line_array).wrapping_add(
                        (if !t1_buf_array.is_null() {
                            strlen(t1_buf_array).wrapping_add(1 as size_t)
                        } else {
                            0 as size_t
                        }),
                    ));
            }
            if t1_buf_limit as ::core::ffi::c_uint > INT_MAX as ::core::ffi::c_uint {
                crate::utils::pdftex_fail_args(
                    b"t1_buf_array exceeds size limit\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            t1_buf_array = xrealloc(
                t1_buf_array as address,
                t1_buf_limit.wrapping_mul(::core::mem::size_of::<t1_buf_entry>() as size_t),
            ) as *mut t1_buf_entry;
            t1_buf_ptr = t1_buf_array.offset(last_ptr_index as isize);
        }
        strcat(t1_buf_array as *mut ::core::ffi::c_char, t1_line_array);
        if t1_line_array.is_null() {
            t1_line_limit = 0x10 as size_t;
            if strlen(t1_buf_array).wrapping_add(1 as size_t) as ::core::ffi::c_uint
                > t1_line_limit as ::core::ffi::c_uint
            {
                t1_line_limit = strlen(t1_buf_array).wrapping_add(1 as size_t);
            }
            t1_line_array = xmalloc(
                t1_line_limit.wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
            ) as *mut t1_line_entry;
            t1_line_ptr = t1_line_array;
        } else if (t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long as size_t)
            .wrapping_add(strlen(t1_buf_array).wrapping_add(1 as size_t))
            as ::core::ffi::c_uint
            > t1_line_limit as ::core::ffi::c_uint
        {
            last_ptr_index =
                t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long as size_t;
            t1_line_limit = t1_line_limit.wrapping_mul(2 as size_t);
            if (t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long as size_t)
                .wrapping_add(strlen(t1_buf_array).wrapping_add(1 as size_t))
                as ::core::ffi::c_uint
                > t1_line_limit as ::core::ffi::c_uint
            {
                t1_line_limit = (t1_line_ptr.offset_from(t1_line_array) as ::core::ffi::c_long
                    as size_t)
                    .wrapping_add(strlen(t1_buf_array).wrapping_add(1 as size_t));
            }
            if t1_line_limit as ::core::ffi::c_uint > INT_MAX as ::core::ffi::c_uint {
                crate::utils::pdftex_fail_args(
                    b"t1_line_array exceeds size limit\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                );
            }
            t1_line_array = xrealloc(
                t1_line_array as address,
                t1_line_limit.wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
            ) as *mut t1_line_entry;
            t1_line_ptr = t1_line_array.offset(last_ptr_index as isize);
        }
        strcpy(t1_line_array as *mut ::core::ffi::c_char, t1_buf_array);
        t1_line_ptr = eol(t1_line_array as *mut ::core::ffi::c_char) as *mut t1_line_entry;
    }
}
unsafe extern "C" fn t1_subset_charstrings() {
    let mut ptr: *mut cs_entry = ::core::ptr::null_mut::<cs_entry>();
    t1_check_unusual_charstring();
    cs_size_pos = (strstr(
        t1_line_array,
        &raw mut charstringname as *mut ::core::ffi::c_char,
    )
    .offset(strlen(&raw mut charstringname as *mut ::core::ffi::c_char) as isize)
    .offset_from(t1_line_array) as ::core::ffi::c_long
        + 1 as ::core::ffi::c_long) as ::core::ffi::c_int;
    cs_size = t1_scan_num(
        t1_line_array.offset(cs_size_pos as isize),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) as ::core::ffi::c_int;
    cs_tab = xmalloc((cs_size as size_t).wrapping_mul(::core::mem::size_of::<cs_entry>() as size_t))
        as *mut cs_entry;
    cs_ptr = cs_tab;
    ptr = cs_tab;
    while (ptr.offset_from(cs_tab) as ::core::ffi::c_long) < cs_size as ::core::ffi::c_long {
        init_cs_entry(ptr);
        ptr = ptr.offset(1);
    }
    cs_notdef = ::core::ptr::null_mut::<cs_entry>();
    cs_dict_start = xstrdup(t1_line_array as const_string) as *mut ::core::ffi::c_char;
    t1_getline();
    while t1_cslen != 0 {
        cs_store(false_0);
        t1_getline();
    }
    cs_dict_end = xstrdup(t1_line_array as const_string) as *mut ::core::ffi::c_char;
    t1_mark_glyphs();
    if !subr_tab.is_null() {
        if cs_token_pair.is_null() {
            crate::utils::pdftex_fail_args(
                b"This Type 1 font uses mismatched subroutine begin/end token pairs.\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
        }
        t1_flush_cs(true_0);
    }
    cs_count = 0 as ::core::ffi::c_int;
    ptr = cs_tab;
    while ptr < cs_ptr {
        if (*ptr).used != 0 {
            cs_count += 1;
        }
        ptr = ptr.offset(1);
    }
    t1_flush_cs(false_0);
}
unsafe extern "C" fn t1_subset_end() {
    if t1_synthetic != 0 {
        while strstr(
            t1_line_array,
            b"definefont\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            t1_getline();
            t1_putline();
        }
        while str_suffix(
            t1_line_array,
            t1_line_ptr,
            b"mark currentfile closefile\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        {
            t1_getline();
        }
        t1_putline();
    } else {
        while str_suffix(
            t1_line_array,
            t1_line_ptr,
            b"mark currentfile closefile\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        {
            t1_getline();
            t1_putline();
        }
    }
    t1_stop_eexec();
    t1_length3 = (if fixedcontent != 0 {
        fb_offset() as ::core::ffi::c_int - t1_save_offset as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as integer;
}
#[no_mangle]
pub unsafe extern "C" fn writet1(mut fd: *mut fd_entry) {
    fd_cur = fd;
    if (*fd_cur).fm.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"writet1\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            1698 as ::core::ffi::c_int,
            b"fd_cur->fm != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !((*(*fd).fm).type_0 as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"writet1\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            1699 as ::core::ffi::c_int,
            b"is_type1(fd->fm)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !((*(*fd).fm).type_0 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"writet1\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            1700 as ::core::ffi::c_int,
            b"is_included(fd->fm)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    t1_save_offset = 0 as ::core::ffi::c_int as integer;
    if !((*(*fd_cur).fm).type_0 as ::core::ffi::c_int & F_SUBSETTED != 0 as ::core::ffi::c_int) {
        (*fd).ff_found = t1_open_fontfile(b"<<\0" as *const u8 as *const ::core::ffi::c_char);
        if (*fd).ff_found == 0 {
            return;
        }
        t1_include();
        t1_close_font_file(b">>\0" as *const u8 as *const ::core::ffi::c_char);
        return;
    }
    (*fd).ff_found = t1_open_fontfile(b"<\0" as *const u8 as *const ::core::ffi::c_char);
    if (*fd).ff_found == 0 {
        return;
    }
    t1_subset_ascii_part();
    t1_start_eexec();
    cc_init();
    cs_init();
    t1_read_subrs();
    t1_subset_charstrings();
    t1_subset_end();
    t1_close_font_file(b">\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn t1_free() {
    if !t1_line_array.is_null() {
        free(t1_line_array as *mut ::core::ffi::c_void);
    }
    t1_line_array = ::core::ptr::null_mut::<t1_line_entry>();
    if !t1_buf_array.is_null() {
        free(t1_buf_array as *mut ::core::ffi::c_void);
    }
    t1_buf_array = ::core::ptr::null_mut::<t1_buf_entry>();
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
