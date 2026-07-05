extern "C" {
    fn atan(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn xstrdup(s: const_string) -> string;
    fn xmalloc(size: size_t) -> address;
    static mut fontbc: *mut eightbits;
    static mut fontec: *mut eightbits;
    static mut fixedgentounicode: integer;
    static mut pdffontattr: *mut strnumber;
    static mut pdffontnobuiltintounicode: *mut boolean;
    static mut pdfcharused: *mut charusedarray;
    static mut pdffontsize: *mut scaled;
    static mut pdffontmap: *mut fmentryptr;
    fn zpdfprint(s: strnumber);
    fn zdividescaled(s: scaled, m: scaled, dd: integer) -> scaled;
    fn pdfnewobjnum() -> integer;
    fn zpdfbeginobj(i: integer, pdfoslevel: integer);
    fn pdfendobj();
    fn zpdfbegindict(i: integer, pdfoslevel: integer);
    fn pdfenddict();
    fn getpdfomitcharset() -> integer;
    fn zgetxheight(f: internalfontnumber) -> scaled;
    fn zgetcharwidth(f: internalfontnumber, c: eightbits) -> scaled;
    fn zgetcharheight(f: internalfontnumber, c: eightbits) -> scaled;
    fn zgetchardepth(f: internalfontnumber, c: eightbits) -> scaled;
    fn zgetquad(f: internalfontnumber) -> scaled;
    fn zgetslant(f: internalfontnumber) -> scaled;
    fn pdfbeginstream();
    fn pdfendstream();
    fn getnullstr() -> strnumber;
    fn avl_create(
        _: Option<avl_comparison_func>,
        _: *mut ::core::ffi::c_void,
        _: *mut libavl_allocator,
    ) -> *mut avl_table;
    fn avl_probe(_: *mut avl_table, _: *mut ::core::ffi::c_void) -> *mut *mut ::core::ffi::c_void;
    fn avl_find(_: *const avl_table, _: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn avl_t_init(_: *mut avl_traverser, _: *mut avl_table);
    fn avl_t_first(_: *mut avl_traverser, _: *mut avl_table) -> *mut ::core::ffi::c_void;
    fn avl_t_next(_: *mut avl_traverser) -> *mut ::core::ffi::c_void;
    static mut avl_xallocator: libavl_allocator;
    static mut t1_length1: integer;
    static mut t1_length3: integer;
    static mut t1_length2: integer;
    static mut ttf_length: integer;
    static mut notdef: [::core::ffi::c_char; 0];
    fn pdf_printf(_: *const ::core::ffi::c_char, ...);
    fn pdf_puts(_: *const ::core::ffi::c_char);
    fn pdftex_fail(_: *const ::core::ffi::c_char, ...) -> !;
    fn pdftex_warn(_: *const ::core::ffi::c_char, ...);
    fn hasfmentry(_: internalfontnumber) -> boolean;
    fn check_std_t1font(s: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn write_tounicode(
        _: *mut *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> integer;
    fn fb_flush();
    fn get_fe_entry(_: *mut ::core::ffi::c_char) -> *mut fe_entry;
    fn write_fontencodings();
    fn writet1(_: *mut fd_entry);
    fn writet3(_: *mut fm_entry, _: ::core::ffi::c_int, _: internalfontnumber);
    fn writettf(_: *mut fd_entry);
    fn writeotf(_: *mut fd_entry);
    fn comp_int_entry(
        _: *const ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn comp_string_entry(
        _: *const ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type boolean = ::core::ffi::c_int;
pub type string = *mut ::core::ffi::c_char;
pub type const_string = *const ::core::ffi::c_char;
pub type address = *mut ::core::ffi::c_void;
pub type integer = ::core::ffi::c_int;
pub type eightbits = ::core::ffi::c_uchar;
pub type strnumber = integer;
pub type scaled = integer;
pub type internalfontnumber = integer;
pub type charusedarray = [eightbits; 32];
pub type fmentryptr = *mut integer;
pub type avl_comparison_func = unsafe extern "C" fn(
    *const ::core::ffi::c_void,
    *const ::core::ffi::c_void,
    *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int;
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
pub struct cw_entry_ {
    pub cw_objnum: integer,
    pub width: *mut integer,
}
pub type cw_entry = cw_entry_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fo_entry_ {
    pub fo_objnum: integer,
    pub tex_font: internalfontnumber,
    pub fm: *mut fm_entry,
    pub fd: *mut fd_entry,
    pub fe: *mut fe_entry,
    pub cw: *mut cw_entry,
    pub first_char: integer,
    pub last_char: integer,
    pub tounicode_objnum: integer,
}
pub type fo_entry = fo_entry_;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ASCENT_CODE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CAPHEIGHT_CODE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DESCENT_CODE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ITALIC_ANGLE_CODE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const STEMV_CODE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const XHEIGHT_CODE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FONTBBOX1_CODE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const FONTBBOX2_CODE: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const FONTBBOX3_CODE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const FONTBBOX4_CODE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const FONTNAME_CODE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const GEN_KEY_NUM: ::core::ffi::c_int = XHEIGHT_CODE + 1 as ::core::ffi::c_int;
pub const INT_KEYS_NUM: ::core::ffi::c_int = FONTBBOX4_CODE + 1 as ::core::ffi::c_int;
pub const FONT_KEYS_NUM: ::core::ffi::c_int = FONTNAME_CODE + 1 as ::core::ffi::c_int;
pub const F_SUBSETTED: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const F_STDT1FONT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const F_TYPE1: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const F_TRUETYPE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const F_OTF: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const F_PK: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const FD_FLAGS_NOT_SET_IN_MAPLINE: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const FD_FLAGS_DEFAULT_EMBED: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FD_FLAGS_DEFAULT_NON_EMBED: ::core::ffi::c_int = 0x22 as ::core::ffi::c_int;
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
pub static mut fo_tree: *mut avl_table = ::core::ptr::null::<avl_table>() as *mut avl_table;
#[no_mangle]
pub static mut fd_tree: *mut avl_table = ::core::ptr::null::<avl_table>() as *mut avl_table;
unsafe extern "C" fn comp_fo_entry(
    mut pa: *const ::core::ffi::c_void,
    mut pb: *const ::core::ffi::c_void,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        (*(*(pa as *const fo_entry)).fm).tfm_name,
        (*(*(pb as *const fo_entry)).fm).tfm_name,
    );
}
unsafe extern "C" fn comp_fd_entry(
    mut pa: *const ::core::ffi::c_void,
    mut pb: *const ::core::ffi::c_void,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut p1: *const fd_entry = pa as *const fd_entry;
    let mut p2: *const fd_entry = pb as *const fd_entry;
    if !(!(*p1).fm.is_null()
        && !(*(*p1).fm).ff_name.is_null()
        && !(*p2).fm.is_null()
        && !(*(*p2).fm).ff_name.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"comp_fd_entry\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            38 as ::core::ffi::c_int,
            b"p1->fm != NULL && is_fontfile(p1->fm) && p2->fm != NULL && is_fontfile(p2->fm)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    i = strcmp((*(*p1).fm).ff_name, (*(*p2).fm).ff_name);
    if i != 0 as ::core::ffi::c_int {
        return i;
    }
    if (*(*p1).fm).slant > (*(*p2).fm).slant {
        return 1 as ::core::ffi::c_int;
    }
    if (*(*p1).fm).slant < (*(*p2).fm).slant {
        return -(1 as ::core::ffi::c_int);
    }
    if (*(*p1).fm).extend > (*(*p2).fm).extend {
        return 1 as ::core::ffi::c_int;
    }
    if (*(*p1).fm).extend < (*(*p2).fm).extend {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn new_fo_entry() -> *mut fo_entry {
    let mut fo: *mut fo_entry = ::core::ptr::null_mut::<fo_entry>();
    fo = xmalloc((1 as size_t).wrapping_mul(::core::mem::size_of::<fo_entry>() as size_t))
        as *mut fo_entry;
    (*fo).fo_objnum = 0 as ::core::ffi::c_int as integer;
    (*fo).tex_font = 0 as ::core::ffi::c_int as internalfontnumber;
    (*fo).fm = ::core::ptr::null_mut::<fm_entry>();
    (*fo).fd = ::core::ptr::null_mut::<fd_entry>();
    (*fo).fe = ::core::ptr::null_mut::<fe_entry>();
    (*fo).cw = ::core::ptr::null_mut::<cw_entry>();
    (*fo).first_char = 1 as ::core::ffi::c_int as integer;
    (*fo).last_char = 0 as ::core::ffi::c_int as integer;
    (*fo).tounicode_objnum = 0 as ::core::ffi::c_int as integer;
    return fo;
}
#[no_mangle]
pub unsafe extern "C" fn new_fd_entry() -> *mut fd_entry {
    let mut fd: *mut fd_entry = ::core::ptr::null_mut::<fd_entry>();
    let mut i: ::core::ffi::c_int = 0;
    fd = xmalloc((1 as size_t).wrapping_mul(::core::mem::size_of::<fd_entry>() as size_t))
        as *mut fd_entry;
    (*fd).fd_objnum = 0 as ::core::ffi::c_int as integer;
    (*fd).fontname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*fd).subset_tag = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*fd).ff_found = false_0 as boolean;
    (*fd).ff_objnum = 0 as ::core::ffi::c_int as integer;
    (*fd).fn_objnum = 0 as ::core::ffi::c_int as integer;
    (*fd).all_glyphs = false_0 as boolean;
    (*fd).write_ttf_glyph_names = false_0 as boolean;
    i = 0 as ::core::ffi::c_int;
    while i < FONT_KEYS_NUM {
        (*fd).font_dim[i as usize].val = 0 as ::core::ffi::c_int;
        (*fd).font_dim[i as usize].set = false_0 as boolean;
        i += 1;
    }
    (*fd).fe = ::core::ptr::null_mut::<fe_entry>();
    (*fd).builtin_glyph_names = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*fd).fm = ::core::ptr::null_mut::<fm_entry>();
    (*fd).tx_tree = ::core::ptr::null_mut::<avl_table>();
    (*fd).gl_tree = ::core::ptr::null_mut::<avl_table>();
    return fd;
}
unsafe extern "C" fn new_cw_entry() -> *mut cw_entry {
    let mut cw: *mut cw_entry = ::core::ptr::null_mut::<cw_entry>();
    cw = xmalloc((1 as size_t).wrapping_mul(::core::mem::size_of::<cw_entry>() as size_t))
        as *mut cw_entry;
    (*cw).cw_objnum = 0 as ::core::ffi::c_int as integer;
    (*cw).width = ::core::ptr::null_mut::<integer>();
    return cw;
}
unsafe extern "C" fn preset_fontmetrics(mut fd: *mut fd_entry, mut f: internalfontnumber) {
    let mut i: ::core::ffi::c_int = 0;
    (*fd).font_dim[ITALIC_ANGLE_CODE as usize].val = zdividescaled(
        (-atan(zgetslant(f) as ::core::ffi::c_double / 65536.0f64)
            * (180 as ::core::ffi::c_int as ::core::ffi::c_double
                / 3.14159265358979323846264338327950288f64)) as scaled,
        *pdffontsize.offset(f as isize),
        3 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    (*fd).font_dim[ASCENT_CODE as usize].val = zdividescaled(
        zgetcharheight(f, 'h' as i32 as eightbits),
        *pdffontsize.offset(f as isize),
        3 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    (*fd).font_dim[CAPHEIGHT_CODE as usize].val = zdividescaled(
        zgetcharheight(f, 'H' as i32 as eightbits),
        *pdffontsize.offset(f as isize),
        3 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    i = -zdividescaled(
        zgetchardepth(f, 'y' as i32 as eightbits),
        *pdffontsize.offset(f as isize),
        3 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    (*fd).font_dim[DESCENT_CODE as usize].val = if i < 0 as ::core::ffi::c_int {
        i
    } else {
        0 as ::core::ffi::c_int
    };
    (*fd).font_dim[STEMV_CODE as usize].val = zdividescaled(
        zgetcharwidth(f, '.' as i32 as eightbits) as ::core::ffi::c_int / 3 as ::core::ffi::c_int,
        *pdffontsize.offset(f as isize),
        3 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    (*fd).font_dim[XHEIGHT_CODE as usize].val = zdividescaled(
        zgetxheight(f),
        *pdffontsize.offset(f as isize),
        3 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    (*fd).font_dim[FONTBBOX1_CODE as usize].val = 0 as ::core::ffi::c_int;
    (*fd).font_dim[FONTBBOX2_CODE as usize].val = (*fd).font_dim[DESCENT_CODE as usize].val;
    (*fd).font_dim[FONTBBOX3_CODE as usize].val = zdividescaled(
        zgetquad(f),
        *pdffontsize.offset(f as isize),
        3 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    (*fd).font_dim[FONTBBOX4_CODE as usize].val =
        if (*fd).font_dim[CAPHEIGHT_CODE as usize].val > (*fd).font_dim[ASCENT_CODE as usize].val {
            (*fd).font_dim[CAPHEIGHT_CODE as usize].val
        } else {
            (*fd).font_dim[ASCENT_CODE as usize].val
        };
    i = 0 as ::core::ffi::c_int;
    while i < INT_KEYS_NUM {
        (*fd).font_dim[i as usize].set = true_0 as boolean;
        i += 1;
    }
}
unsafe extern "C" fn fix_fontmetrics(mut fd: *mut fd_entry) {
    let mut p: *mut intparm = &raw mut (*fd).font_dim as *mut intparm;
    if (*p.offset(FONTBBOX1_CODE as isize)).set == 0
        || (*p.offset(FONTBBOX2_CODE as isize)).set == 0
        || (*p.offset(FONTBBOX3_CODE as isize)).set == 0
        || (*p.offset(FONTBBOX4_CODE as isize)).set == 0
    {
        crate::utils::pdftex_warn_args(b"font `%s' doesn't have a BoundingBox\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from((*(*fd).fm).ff_name)]);
        return;
    }
    if (*p.offset(ASCENT_CODE as isize)).set == 0 {
        (*p.offset(ASCENT_CODE as isize)).val = (*p.offset(FONTBBOX4_CODE as isize)).val;
        (*p.offset(ASCENT_CODE as isize)).set = true_0 as boolean;
    }
    if (*p.offset(DESCENT_CODE as isize)).set == 0 {
        (*p.offset(DESCENT_CODE as isize)).val = (*p.offset(FONTBBOX2_CODE as isize)).val;
        (*p.offset(DESCENT_CODE as isize)).set = true_0 as boolean;
    }
    if (*p.offset(CAPHEIGHT_CODE as isize)).set == 0 {
        (*p.offset(CAPHEIGHT_CODE as isize)).val = (*p.offset(FONTBBOX4_CODE as isize)).val;
        (*p.offset(CAPHEIGHT_CODE as isize)).set = true_0 as boolean;
    }
}
unsafe extern "C" fn write_fontmetrics(mut fd: *mut fd_entry) {
    let mut i: ::core::ffi::c_int = 0;
    fix_fontmetrics(fd);
    if (*fd).font_dim[FONTBBOX1_CODE as usize].set != 0
        && (*fd).font_dim[FONTBBOX2_CODE as usize].set != 0
        && (*fd).font_dim[FONTBBOX3_CODE as usize].set != 0
        && (*fd).font_dim[FONTBBOX4_CODE as usize].set != 0
    {
        crate::utils::pdf_printf_args(b"/%s [%i %i %i %i]\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(font_key[FONTBBOX1_CODE as usize].pdfname), crate::utils::PrintfArg::from((*fd).font_dim[FONTBBOX1_CODE as usize].val), crate::utils::PrintfArg::from((*fd).font_dim[FONTBBOX2_CODE as usize].val), crate::utils::PrintfArg::from((*fd).font_dim[FONTBBOX3_CODE as usize].val), crate::utils::PrintfArg::from((*fd).font_dim[FONTBBOX4_CODE as usize].val)]);
    }
    i = 0 as ::core::ffi::c_int;
    while i < GEN_KEY_NUM {
        if (*fd).font_dim[i as usize].set != 0 {
            crate::utils::pdf_printf_args(b"/%s %i\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(font_key[i as usize].pdfname), crate::utils::PrintfArg::from((*fd).font_dim[i as usize].val)]);
        }
        i += 1;
    }
}
unsafe extern "C" fn preset_fontname(mut fo: *mut fo_entry) {
    if !(*(*fo).fm).ps_name.is_null() {
        (*(*fo).fd).fontname =
            xstrdup((*(*fo).fm).ps_name as const_string) as *mut ::core::ffi::c_char;
    } else {
        (*(*fo).fd).fontname =
            xstrdup((*(*fo).fm).tfm_name as const_string) as *mut ::core::ffi::c_char;
    };
}
unsafe extern "C" fn write_fontname(mut fd: *mut fd_entry, mut key: *const ::core::ffi::c_char) {
    if (*fd).fontname.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"write_fontname\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            187 as ::core::ffi::c_int,
            b"fd->fontname != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    pdf_puts(b"/\0" as *const u8 as *const ::core::ffi::c_char);
    if !key.is_null() {
        crate::utils::pdf_printf_args(b"%s /\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(key)]);
    }
    if !(*fd).subset_tag.is_null() {
        crate::utils::pdf_printf_args(b"%s+\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from((*fd).subset_tag)]);
    }
    crate::utils::pdf_printf_args(b"%s\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from((*fd).fontname)]);
}
unsafe extern "C" fn write_fontname_object(mut fd: *mut fd_entry) {
    if !((*fd).fn_objnum != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"write_fontname_object\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            198 as ::core::ffi::c_int,
            b"fd->fn_objnum != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    zpdfbeginobj((*fd).fn_objnum, 1 as ::core::ffi::c_int);
    write_fontname(fd, ::core::ptr::null::<::core::ffi::c_char>());
    pdfendobj();
}
#[no_mangle]
pub unsafe extern "C" fn lookup_fd_entry(
    mut s: *mut ::core::ffi::c_char,
    mut slant: integer,
    mut extend: integer,
) -> *mut fd_entry {
    let mut fd: fd_entry = fd_entry_ {
        fd_objnum: 0,
        fontname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        subset_tag: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ff_found: 0,
        ff_objnum: 0,
        fn_objnum: 0,
        all_glyphs: 0,
        write_ttf_glyph_names: 0,
        font_dim: [intparm { val: 0, set: 0 }; 11],
        fe: ::core::ptr::null_mut::<fe_entry>(),
        builtin_glyph_names: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        fm: ::core::ptr::null_mut::<fm_entry>(),
        tx_tree: ::core::ptr::null_mut::<avl_table>(),
        gl_tree: ::core::ptr::null_mut::<avl_table>(),
    };
    let mut fm: fm_entry = fm_entry {
        tfm_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        sfd_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ps_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        fd_flags: 0,
        slant: 0,
        extend: 0,
        encname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ff_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        type_0: 0,
        pid: 0,
        eid: 0,
        subfont: ::core::ptr::null_mut::<subfont_entry>(),
        links: 0,
        in_use: 0,
    };
    if s.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"lookup_fd_entry\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            210 as ::core::ffi::c_int,
            b"s != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    fm.ff_name = s;
    fm.slant = slant;
    fm.extend = extend;
    fd.fm = &raw mut fm;
    if fd_tree.is_null() {
        fd_tree = avl_create(
            Some(
                comp_fd_entry
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
            &raw mut avl_xallocator,
        );
        if fd_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"lookup_fd_entry\0" as *const u8 as *const ::core::ffi::c_char,
                b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
                217 as ::core::ffi::c_int,
                b"fd_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    return avl_find(fd_tree, &raw mut fd as *const ::core::ffi::c_void) as *mut fd_entry;
}
unsafe extern "C" fn lookup_fontdescriptor(mut fo: *mut fo_entry) -> *mut fd_entry {
    if fo.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"lookup_fontdescriptor\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            224 as ::core::ffi::c_int,
            b"fo != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if (*fo).fm.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"lookup_fontdescriptor\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            225 as ::core::ffi::c_int,
            b"fo->fm != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if (*(*fo).fm).ff_name.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"lookup_fontdescriptor\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            226 as ::core::ffi::c_int,
            b"is_fontfile(fo->fm)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    return lookup_fd_entry((*(*fo).fm).ff_name, (*(*fo).fm).slant, (*(*fo).fm).extend);
}
#[no_mangle]
pub unsafe extern "C" fn register_fd_entry(mut fd: *mut fd_entry) {
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    if fd_tree.is_null() {
        fd_tree = avl_create(
            Some(
                comp_fd_entry
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
            &raw mut avl_xallocator,
        );
        if fd_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"register_fd_entry\0" as *const u8 as *const ::core::ffi::c_char,
                b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
                235 as ::core::ffi::c_int,
                b"fd_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    if !(!fd.is_null() && !(*fd).fm.is_null() && !(*(*fd).fm).ff_name.is_null())
        as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"register_fd_entry\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            237 as ::core::ffi::c_int,
            b"fd != NULL && fd->fm != NULL && is_fontfile(fd->fm)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !lookup_fd_entry((*(*fd).fm).ff_name, (*(*fd).fm).slant, (*(*fd).fm).extend).is_null()
        as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"register_fd_entry\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            238 as ::core::ffi::c_int,
            b"lookup_fd_entry(fd->fm->ff_name, fd->fm->slant, fd->fm->extend) == NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    aa = avl_probe(fd_tree, fd as *mut ::core::ffi::c_void);
    if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"register_fd_entry\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            240 as ::core::ffi::c_int,
            b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
}
unsafe extern "C" fn create_fontdescriptor(mut fo: *mut fo_entry, mut f: internalfontnumber) {
    if fo.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"create_fontdescriptor\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            245 as ::core::ffi::c_int,
            b"fo != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if (*fo).fm.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"create_fontdescriptor\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            246 as ::core::ffi::c_int,
            b"fo->fm != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !(*fo).fd.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"create_fontdescriptor\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            247 as ::core::ffi::c_int,
            b"fo->fd == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*fo).fd = new_fd_entry();
    preset_fontname(fo);
    preset_fontmetrics((*fo).fd, f);
    (*(*fo).fd).fe = (*fo).fe;
    (*(*fo).fd).fm = (*fo).fm;
    (*(*fo).fd).gl_tree = avl_create(
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
    if (*(*fo).fd).gl_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"create_fontdescriptor\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            254 as ::core::ffi::c_int,
            b"fo->fd->gl_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
}
unsafe extern "C" fn mark_reenc_glyphs(mut fo: *mut fo_entry, mut f: internalfontnumber) {
    let mut i: ::core::ffi::c_int = 0;
    let mut g: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    if (*fo).fe.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"mark_reenc_glyphs\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            269 as ::core::ffi::c_int,
            b"fo->fe != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if (*(*fo).fm).type_0 as ::core::ffi::c_int & F_SUBSETTED != 0 as ::core::ffi::c_int {
        if !((*(*fo).fm).type_0 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            __assert_rtn(
                b"mark_reenc_glyphs\0" as *const u8 as *const ::core::ffi::c_char,
                b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
                271 as ::core::ffi::c_int,
                b"is_included(fo->fm)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        g = (*(*fo).fe).glyph_names;
        i = (*fo).first_char as ::core::ffi::c_int;
        while i <= (*fo).last_char {
            if (*pdfcharused.offset(f as isize))[(i / 8 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << i % 8 as ::core::ffi::c_int
                != 0
                && *g.offset(i as isize) != &raw mut notdef as *mut ::core::ffi::c_char
                && (avl_find(
                    (*(*fo).fd).gl_tree,
                    *g.offset(i as isize) as *const ::core::ffi::c_void,
                ) as *mut ::core::ffi::c_char)
                    .is_null()
            {
                aa = avl_probe(
                    (*(*fo).fd).gl_tree,
                    xstrdup(*g.offset(i as isize) as const_string) as *mut ::core::ffi::c_void,
                );
                if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                    __assert_rtn(
                        b"mark_reenc_glyphs\0" as *const u8 as *const ::core::ffi::c_char,
                        b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
                        278 as ::core::ffi::c_int,
                        b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                };
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn mark_chars(
    mut fo: *mut fo_entry,
    mut tx_tree: *mut avl_table,
    mut f: internalfontnumber,
) -> *mut avl_table {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    if tx_tree.is_null() {
        tx_tree = avl_create(
            Some(
                comp_int_entry
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
            &raw mut avl_xallocator,
        );
        if tx_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"mark_chars\0" as *const u8 as *const ::core::ffi::c_char,
                b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
                297 as ::core::ffi::c_int,
                b"tx_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    i = (*fo).first_char as ::core::ffi::c_int;
    while i <= (*fo).last_char {
        if (*pdfcharused.offset(f as isize))[(i / 8 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << i % 8 as ::core::ffi::c_int
            != 0
            && (avl_find(tx_tree, &raw mut i as *const ::core::ffi::c_void)
                as *mut ::core::ffi::c_int)
                .is_null()
        {
            j = xmalloc(
                (1 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
            ) as *mut ::core::ffi::c_int;
            *j = i;
            aa = avl_probe(tx_tree, j as *mut ::core::ffi::c_void);
            if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                __assert_rtn(
                    b"mark_chars\0" as *const u8 as *const ::core::ffi::c_char,
                    b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
                    304 as ::core::ffi::c_int,
                    b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
        }
        i += 1;
    }
    return tx_tree;
}
unsafe extern "C" fn get_char_range(mut fo: *mut fo_entry, mut f: internalfontnumber) {
    let mut i: ::core::ffi::c_int = 0;
    if fo.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"get_char_range\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            315 as ::core::ffi::c_int,
            b"fo != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
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
    (*fo).first_char = i as integer;
    i = *fontec.offset(f as isize) as ::core::ffi::c_int;
    while i >= *fontbc.offset(f as isize) as ::core::ffi::c_int {
        if (*pdfcharused.offset(f as isize))[(i / 8 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << i % 8 as ::core::ffi::c_int
            != 0
        {
            break;
        }
        i -= 1;
    }
    (*fo).last_char = i as integer;
    if (*fo).first_char > (*fo).last_char
        || (*pdfcharused.offset(f as isize))
            [((*fo).first_char as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << (*fo).first_char as ::core::ffi::c_int % 8 as ::core::ffi::c_int
            == 0
    {
        (*fo).last_char = 0 as ::core::ffi::c_int as integer;
        (*fo).first_char =
            ((*fo).last_char as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as integer;
    }
}
unsafe extern "C" fn create_charwidth_array(mut fo: *mut fo_entry, mut f: internalfontnumber) {
    let mut i: ::core::ffi::c_int = 0;
    if fo.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"create_charwidth_array\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            334 as ::core::ffi::c_int,
            b"fo != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !(*fo).cw.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"create_charwidth_array\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            335 as ::core::ffi::c_int,
            b"fo->cw == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*fo).cw = new_cw_entry();
    (*(*fo).cw).width =
        xmalloc((256 as size_t).wrapping_mul(::core::mem::size_of::<integer>() as size_t))
            as *mut integer;
    i = 0 as ::core::ffi::c_int;
    while i < (*fo).first_char {
        *(*(*fo).cw).width.offset(i as isize) = 0 as ::core::ffi::c_int as integer;
        i += 1;
    }
    i = (*fo).first_char as ::core::ffi::c_int;
    while i <= (*fo).last_char {
        *(*(*fo).cw).width.offset(i as isize) = zdividescaled(
            zgetcharwidth(f, i as eightbits),
            *pdffontsize.offset(f as isize),
            4 as ::core::ffi::c_int,
        ) as integer;
        i += 1;
    }
    i = (*fo).last_char as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        *(*(*fo).cw).width.offset(i as isize) = 0 as ::core::ffi::c_int as integer;
        i += 1;
    }
}
unsafe extern "C" fn write_charwidth_array(mut fo: *mut fo_entry) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    if (*fo).cw.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"write_charwidth_array\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            349 as ::core::ffi::c_int,
            b"fo->cw != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !((*(*fo).cw).cw_objnum == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"write_charwidth_array\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            350 as ::core::ffi::c_int,
            b"fo->cw->cw_objnum == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*(*fo).cw).cw_objnum = pdfnewobjnum();
    zpdfbeginobj((*(*fo).cw).cw_objnum, 1 as ::core::ffi::c_int);
    pdf_puts(b"[\0" as *const u8 as *const ::core::ffi::c_char);
    i = (*fo).first_char as ::core::ffi::c_int;
    while i <= (*fo).last_char {
        crate::utils::pdf_printf_args(b"%i\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(*(*(*fo).cw).width.offset(i as isize) / 10 as ::core::ffi::c_int)]);
        j = *(*(*fo).cw).width.offset(i as isize) as ::core::ffi::c_int % 10 as ::core::ffi::c_int;
        if j != 0 as ::core::ffi::c_int {
            crate::utils::pdf_printf_args(b".%i\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(j)]);
        }
        if i != (*fo).last_char {
            pdf_puts(b" \0" as *const u8 as *const ::core::ffi::c_char);
        }
        i += 1;
    }
    pdf_puts(b"]\n\0" as *const u8 as *const ::core::ffi::c_char);
    pdfendobj();
}
unsafe extern "C" fn lookup_fo_entry(mut s: *mut ::core::ffi::c_char) -> *mut fo_entry {
    let mut fo: fo_entry = fo_entry_ {
        fo_objnum: 0,
        tex_font: 0,
        fm: ::core::ptr::null_mut::<fm_entry>(),
        fd: ::core::ptr::null_mut::<fd_entry>(),
        fe: ::core::ptr::null_mut::<fe_entry>(),
        cw: ::core::ptr::null_mut::<cw_entry>(),
        first_char: 0,
        last_char: 0,
        tounicode_objnum: 0,
    };
    let mut fm: fm_entry = fm_entry {
        tfm_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        sfd_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ps_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        fd_flags: 0,
        slant: 0,
        extend: 0,
        encname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ff_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        type_0: 0,
        pid: 0,
        eid: 0,
        subfont: ::core::ptr::null_mut::<subfont_entry>(),
        links: 0,
        in_use: 0,
    };
    if s.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"lookup_fo_entry\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            375 as ::core::ffi::c_int,
            b"s != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    fm.tfm_name = s;
    fo.fm = &raw mut fm;
    if fo_tree.is_null() {
        fo_tree = avl_create(
            Some(
                comp_fo_entry
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
            &raw mut avl_xallocator,
        );
        if fo_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"lookup_fo_entry\0" as *const u8 as *const ::core::ffi::c_char,
                b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
                380 as ::core::ffi::c_int,
                b"fo_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    return avl_find(fo_tree, &raw mut fo as *const ::core::ffi::c_void) as *mut fo_entry;
}
unsafe extern "C" fn register_fo_entry(mut fo: *mut fo_entry) {
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    if fo_tree.is_null() {
        fo_tree = avl_create(
            Some(
                comp_fo_entry
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
            &raw mut avl_xallocator,
        );
        if fo_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"register_fo_entry\0" as *const u8 as *const ::core::ffi::c_char,
                b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
                390 as ::core::ffi::c_int,
                b"fo_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    if fo.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"register_fo_entry\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            392 as ::core::ffi::c_int,
            b"fo != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if (*fo).fm.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"register_fo_entry\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            393 as ::core::ffi::c_int,
            b"fo->fm != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if (*(*fo).fm).tfm_name.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"register_fo_entry\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            394 as ::core::ffi::c_int,
            b"fo->fm->tfm_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !lookup_fo_entry((*(*fo).fm).tfm_name).is_null() as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"register_fo_entry\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            395 as ::core::ffi::c_int,
            b"lookup_fo_entry(fo->fm->tfm_name) == NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
    };
    aa = avl_probe(fo_tree, fo as *mut ::core::ffi::c_void);
    if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"register_fo_entry\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            397 as ::core::ffi::c_int,
            b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
}
unsafe extern "C" fn write_fontfile(mut fd: *mut fd_entry) {
    if !((*(*fd).fm).type_0 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"write_fontfile\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            404 as ::core::ffi::c_int,
            b"is_included(fd->fm)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if (*(*fd).fm).type_0 as ::core::ffi::c_int & F_TYPE1 != 0 as ::core::ffi::c_int {
        writet1(fd);
    } else if (*(*fd).fm).type_0 as ::core::ffi::c_int & F_TRUETYPE != 0 as ::core::ffi::c_int {
        writettf(fd);
    } else if (*(*fd).fm).type_0 as ::core::ffi::c_int & F_OTF != 0 as ::core::ffi::c_int {
        writeotf(fd);
    } else {
        if (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"write_fontfile\0" as *const u8 as *const ::core::ffi::c_char,
                b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
                412 as ::core::ffi::c_int,
                b"0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    if (*fd).ff_found == 0 {
        return;
    }
    if !((*fd).ff_objnum == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"write_fontfile\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            415 as ::core::ffi::c_int,
            b"fd->ff_objnum == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*fd).ff_objnum = pdfnewobjnum();
    zpdfbegindict((*fd).ff_objnum, 0 as ::core::ffi::c_int);
    if (*(*fd).fm).type_0 as ::core::ffi::c_int & F_TYPE1 != 0 as ::core::ffi::c_int {
        crate::utils::pdf_printf_args(b"/Length1 %i\n/Length2 %i\n/Length3 %i\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(t1_length1), crate::utils::PrintfArg::from(t1_length2), crate::utils::PrintfArg::from(t1_length3)]);
    } else if (*(*fd).fm).type_0 as ::core::ffi::c_int & F_TRUETYPE != 0 as ::core::ffi::c_int {
        crate::utils::pdf_printf_args(b"/Length1 %i\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(ttf_length)]);
    } else if (*(*fd).fm).type_0 as ::core::ffi::c_int & F_OTF != 0 as ::core::ffi::c_int {
        pdf_puts(b"/Subtype /Type1C\n\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        if (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"write_fontfile\0" as *const u8 as *const ::core::ffi::c_char,
                b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
                426 as ::core::ffi::c_int,
                b"0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    pdfbeginstream();
    fb_flush();
    pdfendstream();
}
unsafe extern "C" fn write_fontdescriptor(mut fd: *mut fd_entry) {
    static mut std_flags: [::core::ffi::c_int; 14] = [
        1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int),
        1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int),
        1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int),
        1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int),
        (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int,
        ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int),
        ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int),
        ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int),
        4 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int + ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int),
        2 as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int),
        2 as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int),
        2 as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int),
        4 as ::core::ffi::c_int,
    ];
    let mut glyph: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut t: avl_traverser = avl_traverser {
        avl_table: ::core::ptr::null_mut::<avl_table>(),
        avl_node: ::core::ptr::null_mut::<avl_node>(),
        avl_stack: [::core::ptr::null_mut::<avl_node>(); 32],
        avl_height: 0,
        avl_generation: 0,
    };
    let mut fd_flags: ::core::ffi::c_int = 0;
    if !(!fd.is_null() && !(*fd).fm.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"write_fontdescriptor\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            459 as ::core::ffi::c_int,
            b"fd != NULL && fd->fm != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !(*(*fd).fm).ff_name.is_null() {
        write_fontfile(fd);
    }
    if (*fd).fn_objnum != 0 as ::core::ffi::c_int {
        write_fontname_object(fd);
    }
    if (*fd).fd_objnum == 0 as ::core::ffi::c_int {
        (*fd).fd_objnum = pdfnewobjnum();
    }
    zpdfbegindict((*fd).fd_objnum, 1 as ::core::ffi::c_int);
    pdf_puts(b"/Type /FontDescriptor\n\0" as *const u8 as *const ::core::ffi::c_char);
    write_fontname(fd, b"FontName\0" as *const u8 as *const ::core::ffi::c_char);
    if (*(*fd).fm).fd_flags != FD_FLAGS_NOT_SET_IN_MAPLINE {
        fd_flags = (*(*fd).fm).fd_flags;
    } else if (*fd).ff_found != 0 {
        fd_flags = FD_FLAGS_DEFAULT_EMBED;
    } else {
        fd_flags =
            if (*(*fd).fm).type_0 as ::core::ffi::c_int & F_STDT1FONT != 0 as ::core::ffi::c_int {
                std_flags[check_std_t1font((*(*fd).fm).ps_name) as usize]
            } else {
                FD_FLAGS_DEFAULT_NON_EMBED
            };
        crate::utils::pdftex_warn_args(b"No flags specified for non-embedded font `%s' (%s) (I'm using %i): fix your map entry.\0"
                as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(if !(*(*fd).fm).ps_name.is_null() {
                (*(*fd).fm).ps_name as *const ::core::ffi::c_char
            } else {
                b"No name given\0" as *const u8 as *const ::core::ffi::c_char
            }), crate::utils::PrintfArg::from((*(*fd).fm).tfm_name), crate::utils::PrintfArg::from(fd_flags)]);
    }
    crate::utils::pdf_printf_args(b"/Flags %i\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(fd_flags)]);
    write_fontmetrics(fd);
    if (*fd).ff_found != 0 {
        if getpdfomitcharset() == 0 as ::core::ffi::c_int
            && (*(*fd).fm).type_0 as ::core::ffi::c_int & F_SUBSETTED != 0 as ::core::ffi::c_int
            && (*(*fd).fm).type_0 as ::core::ffi::c_int & F_TYPE1 != 0 as ::core::ffi::c_int
        {
            if (*fd).gl_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                __assert_rtn(
                    b"write_fontdescriptor\0" as *const u8 as *const ::core::ffi::c_char,
                    b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
                    503 as ::core::ffi::c_int,
                    b"fd->gl_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
            avl_t_init(&raw mut t, (*fd).gl_tree);
            pdf_puts(b"/CharSet (\0" as *const u8 as *const ::core::ffi::c_char);
            glyph = avl_t_first(&raw mut t, (*fd).gl_tree) as *mut ::core::ffi::c_char;
            while !glyph.is_null() {
                crate::utils::pdf_printf_args(b"/%s\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(glyph)]);
                glyph = avl_t_next(&raw mut t) as *mut ::core::ffi::c_char;
            }
            pdf_puts(b")\n\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if (*(*fd).fm).type_0 as ::core::ffi::c_int & F_TYPE1 != 0 as ::core::ffi::c_int {
            crate::utils::pdf_printf_args(b"/FontFile %i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from((*fd).ff_objnum)]);
        } else if (*(*fd).fm).type_0 as ::core::ffi::c_int & F_TRUETYPE != 0 as ::core::ffi::c_int {
            crate::utils::pdf_printf_args(b"/FontFile2 %i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from((*fd).ff_objnum)]);
        } else if (*(*fd).fm).type_0 as ::core::ffi::c_int & F_OTF != 0 as ::core::ffi::c_int {
            crate::utils::pdf_printf_args(b"/FontFile3 %i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from((*fd).ff_objnum)]);
        } else {
            if (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                __assert_rtn(
                    b"write_fontdescriptor\0" as *const u8 as *const ::core::ffi::c_char,
                    b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
                    520 as ::core::ffi::c_int,
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
        }
    }
    pdfenddict();
}
unsafe extern "C" fn write_fontdescriptors() {
    let mut fd: *mut fd_entry = ::core::ptr::null_mut::<fd_entry>();
    let mut t: avl_traverser = avl_traverser {
        avl_table: ::core::ptr::null_mut::<avl_table>(),
        avl_node: ::core::ptr::null_mut::<avl_node>(),
        avl_stack: [::core::ptr::null_mut::<avl_node>(); 32],
        avl_height: 0,
        avl_generation: 0,
    };
    if fd_tree.is_null() {
        return;
    }
    avl_t_init(&raw mut t, fd_tree);
    fd = avl_t_first(&raw mut t, fd_tree) as *mut fd_entry;
    while !fd.is_null() {
        write_fontdescriptor(fd);
        fd = avl_t_next(&raw mut t) as *mut fd_entry;
    }
}
unsafe extern "C" fn write_fontdictionary(mut fo: *mut fo_entry) {
    if fo.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"write_fontdictionary\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            541 as ::core::ffi::c_int,
            b"fo != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if (*fo).fm.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"write_fontdictionary\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            542 as ::core::ffi::c_int,
            b"fo->fm != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !((*fo).fo_objnum != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"write_fontdictionary\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            543 as ::core::ffi::c_int,
            b"fo->fo_objnum != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !((*fo).tex_font != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"write_fontdictionary\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            544 as ::core::ffi::c_int,
            b"fo->tex_font != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if fixedgentounicode > 0 as ::core::ffi::c_int
        && !(*fo).fd.is_null()
        && *pdffontnobuiltintounicode.offset((*fo).tex_font as isize) == 0
        || (!(*fo).fd.is_null()
            && !(*(*fo).fm).tfm_name.is_null()
            && strcmp(
                (*(*fo).fm).tfm_name,
                b"dummy-space\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        if !(*fo).fe.is_null() {
            (*fo).tounicode_objnum = write_tounicode(
                (*(*fo).fe).glyph_names,
                (*(*fo).fm).tfm_name,
                (*(*fo).fe).name,
            );
        } else if (*(*fo).fm).type_0 as ::core::ffi::c_int & F_TYPE1 != 0 as ::core::ffi::c_int {
            if (*(*fo).fd).builtin_glyph_names.is_null() {
                crate::utils::pdftex_fail_args(b"builtin glyph names is empty\0" as *const u8 as *const ::core::ffi::c_char, &[]);
            }
            (*fo).tounicode_objnum = write_tounicode(
                (*(*fo).fd).builtin_glyph_names,
                (*(*fo).fm).tfm_name,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    zpdfbegindict((*fo).fo_objnum, 1 as ::core::ffi::c_int);
    pdf_puts(b"/Type /Font\n\0" as *const u8 as *const ::core::ffi::c_char);
    pdf_puts(b"/Subtype /\0" as *const u8 as *const ::core::ffi::c_char);
    if (*(*fo).fm).type_0 as ::core::ffi::c_int & F_TYPE1 != 0 as ::core::ffi::c_int {
        crate::utils::pdf_printf_args(b"%s\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(b"Type1\0" as *const u8 as *const ::core::ffi::c_char)]);
    } else if (*(*fo).fm).type_0 as ::core::ffi::c_int & F_TRUETYPE != 0 as ::core::ffi::c_int {
        crate::utils::pdf_printf_args(b"%s\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(b"TrueType\0" as *const u8 as *const ::core::ffi::c_char)]);
    } else if (*(*fo).fm).type_0 as ::core::ffi::c_int & F_OTF != 0 as ::core::ffi::c_int {
        crate::utils::pdf_printf_args(b"%s\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(b"Type1\0" as *const u8 as *const ::core::ffi::c_char)]);
    } else {
        if (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"write_fontdictionary\0" as *const u8 as *const ::core::ffi::c_char,
                b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
                573 as ::core::ffi::c_int,
                b"0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    if !(!(*fo).fd.is_null() && (*(*fo).fd).fd_objnum != 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"write_fontdictionary\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            574 as ::core::ffi::c_int,
            b"fo->fd != NULL && fo->fd->fd_objnum != 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
    };
    write_fontname(
        (*fo).fd,
        b"BaseFont\0" as *const u8 as *const ::core::ffi::c_char,
    );
    crate::utils::pdf_printf_args(b"/FontDescriptor %i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from((*(*fo).fd).fd_objnum)]);
    if (*fo).cw.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"write_fontdictionary\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            577 as ::core::ffi::c_int,
            b"fo->cw != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    crate::utils::pdf_printf_args(b"/FirstChar %i\n/LastChar %i\n/Widths %i 0 R\n\0" as *const u8
            as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from((*fo).first_char), crate::utils::PrintfArg::from((*fo).last_char), crate::utils::PrintfArg::from((*(*fo).cw).cw_objnum)]);
    if ((*(*fo).fm).type_0 as ::core::ffi::c_int & F_TYPE1 != 0 as ::core::ffi::c_int
        || (*(*fo).fm).type_0 as ::core::ffi::c_int & F_OTF != 0 as ::core::ffi::c_int)
        && !(*fo).fe.is_null()
        && (*(*fo).fe).fe_objnum != 0 as ::core::ffi::c_int
    {
        crate::utils::pdf_printf_args(b"/Encoding %i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from((*(*fo).fe).fe_objnum)]);
    }
    if (*fo).tounicode_objnum != 0 as ::core::ffi::c_int {
        crate::utils::pdf_printf_args(b"/ToUnicode %i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from((*fo).tounicode_objnum)]);
    }
    if *pdffontattr.offset((*fo).tex_font as isize) != getnullstr() {
        zpdfprint(*pdffontattr.offset((*fo).tex_font as isize));
        pdf_puts(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    pdfenddict();
}
unsafe extern "C" fn write_fontdictionaries() {
    let mut fo: *mut fo_entry = ::core::ptr::null_mut::<fo_entry>();
    let mut t: avl_traverser = avl_traverser {
        avl_table: ::core::ptr::null_mut::<avl_table>(),
        avl_node: ::core::ptr::null_mut::<avl_node>(),
        avl_stack: [::core::ptr::null_mut::<avl_node>(); 32],
        avl_height: 0,
        avl_generation: 0,
    };
    if fo_tree.is_null() {
        return;
    }
    avl_t_init(&raw mut t, fo_tree);
    fo = avl_t_first(&raw mut t, fo_tree) as *mut fo_entry;
    while !fo.is_null() {
        write_fontdictionary(fo);
        fo = avl_t_next(&raw mut t) as *mut fo_entry;
    }
}
#[no_mangle]
pub unsafe extern "C" fn writefontstuff() {
    write_fontdescriptors();
    write_fontencodings();
    write_fontdictionaries();
}
unsafe extern "C" fn create_fontdictionary(
    mut fm: *mut fm_entry,
    mut font_objnum: integer,
    mut f: internalfontnumber,
) {
    let mut fo: *mut fo_entry = new_fo_entry();
    get_char_range(fo, f);
    if !((*fo).last_char >= (*fo).first_char) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"create_fontdictionary\0" as *const u8 as *const ::core::ffi::c_char,
            b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
            625 as ::core::ffi::c_int,
            b"fo->last_char >= fo->first_char\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*fo).fm = fm;
    (*fo).fo_objnum = font_objnum;
    (*fo).tex_font = f;
    if !(*(*fo).fm).encname.is_null() {
        (*fo).fe = get_fe_entry((*(*fo).fm).encname);
        if !(*fo).fe.is_null()
            && ((*(*fo).fm).type_0 as ::core::ffi::c_int & F_TYPE1 != 0 as ::core::ffi::c_int
                || (*(*fo).fm).type_0 as ::core::ffi::c_int & F_OTF != 0 as ::core::ffi::c_int)
        {
            if (*(*fo).fe).fe_objnum == 0 as ::core::ffi::c_int {
                (*(*fo).fe).fe_objnum = pdfnewobjnum();
            }
            (*(*fo).fe).tx_tree = mark_chars(fo, (*(*fo).fe).tx_tree, f);
        }
    }
    if !(*(*fo).fm).ff_name.is_null() {
        if (*(*fo).fm).type_0 as ::core::ffi::c_int & F_TYPE1 != 0 as ::core::ffi::c_int {
            (*fo).fd = lookup_fontdescriptor(fo);
            if (*fo).fd.is_null() {
                create_fontdescriptor(fo, f);
                register_fd_entry((*fo).fd);
            }
        } else {
            create_fontdescriptor(fo, f);
        }
        create_charwidth_array(fo, f);
        write_charwidth_array(fo);
        if !(*fo).fe.is_null() {
            mark_reenc_glyphs(fo, f);
            if !((*(*fo).fm).type_0 as ::core::ffi::c_int & F_TYPE1 != 0 as ::core::ffi::c_int) {
                if !(*(*fo).fd).tx_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0
                {
                    __assert_rtn(
                        b"create_fontdictionary\0" as *const u8 as *const ::core::ffi::c_char,
                        b"writefont.c\0" as *const u8 as *const ::core::ffi::c_char,
                        652 as ::core::ffi::c_int,
                        b"fo->fd->tx_tree == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                };
                (*(*fo).fd).tx_tree = mark_chars(fo, (*(*fo).fd).tx_tree, f);
                if (*(*fo).fm).type_0 as ::core::ffi::c_int & F_TRUETYPE != 0 as ::core::ffi::c_int
                {
                    (*(*fo).fd).write_ttf_glyph_names = true_0 as boolean;
                }
            }
        } else {
            (*(*fo).fd).tx_tree = mark_chars(fo, (*(*fo).fd).tx_tree, f);
        }
        if !((*(*fo).fm).type_0 as ::core::ffi::c_int & F_TYPE1 != 0 as ::core::ffi::c_int) {
            write_fontdescriptor((*fo).fd);
        }
    } else {
        create_charwidth_array(fo, f);
        write_charwidth_array(fo);
        create_fontdescriptor(fo, f);
        write_fontdescriptor((*fo).fd);
        if !((*(*fo).fm).type_0 as ::core::ffi::c_int & F_STDT1FONT != 0 as ::core::ffi::c_int) {
            crate::utils::pdftex_warn_args(b"font `%s' is not a standard font; I suppose it is available to your PDF viewer then\0"
                    as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from((*(*fo).fm).ps_name)]);
        }
    }
    if (*(*fo).fm).type_0 as ::core::ffi::c_int & F_TYPE1 != 0 as ::core::ffi::c_int {
        register_fo_entry(fo);
    } else {
        write_fontdictionary(fo);
    };
}
unsafe extern "C" fn font_is_used(mut f: internalfontnumber) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0;
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
    s = i;
    i = *fontec.offset(f as isize) as ::core::ffi::c_int;
    while i >= *fontbc.offset(f as isize) as ::core::ffi::c_int {
        if (*pdfcharused.offset(f as isize))[(i / 8 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << i % 8 as ::core::ffi::c_int
            != 0
        {
            break;
        }
        i -= 1;
    }
    if s > i {
        return 0 as ::core::ffi::c_int;
    } else {
        return 1 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn dopdffont(mut font_objnum: integer, mut f: internalfontnumber) {
    let mut fm: *mut fm_entry = ::core::ptr::null_mut::<fm_entry>();
    if font_is_used(f) == 0 {
        return;
    }
    fm = if hasfmentry(f) != 0 {
        *pdffontmap.offset(f as isize) as *mut fm_entry
    } else {
        ::core::ptr::null_mut::<fm_entry>()
    };
    if fm.is_null() || (*fm).type_0 as ::core::ffi::c_int & F_PK != 0 as ::core::ffi::c_int {
        writet3(fm, font_objnum as ::core::ffi::c_int, f);
    } else {
        create_fontdictionary(fm, font_objnum, f);
    };
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
