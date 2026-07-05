extern "C" {
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn xstrdup(s: const_string) -> string;
    fn pdfnewobjnum() -> integer;
    fn avl_create(
        _: Option<avl_comparison_func>,
        _: *mut ::core::ffi::c_void,
        _: *mut libavl_allocator,
    ) -> *mut avl_table;
    fn avl_probe(_: *mut avl_table, _: *mut ::core::ffi::c_void) -> *mut *mut ::core::ffi::c_void;
    fn avl_find(_: *const avl_table, _: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    static mut avl_xallocator: libavl_allocator;
    fn epdf_check_mem();
    fn lookup_fd_entry(_: *mut ::core::ffi::c_char, _: integer, _: integer) -> *mut fd_entry;
    fn new_fd_entry() -> *mut fd_entry;
    fn register_fd_entry(_: *mut fd_entry);
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
pub type integer = ::core::ffi::c_int;
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
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const STEMV_CODE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const F_SUBSETTED: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn is_subsetable(mut fm: *mut fm_entry) -> ::core::ffi::c_int {
    if !((*fm).type_0 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"is_subsetable\0" as *const u8 as *const ::core::ffi::c_char,
            b"epdf.c\0" as *const u8 as *const ::core::ffi::c_char,
            26 as ::core::ffi::c_int,
            b"is_included(fm)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    return ((*fm).type_0 as ::core::ffi::c_int & F_SUBSETTED != 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn epdf_create_fontdescriptor(
    mut fm: *mut fm_entry,
    mut stemV: ::core::ffi::c_int,
) -> *mut fd_entry {
    let mut fd: *mut fd_entry = ::core::ptr::null_mut::<fd_entry>();
    fd = lookup_fd_entry((*fm).ff_name, (*fm).slant, (*fm).extend);
    if fd.is_null() {
        (*fm).in_use = true_0 as boolean;
        fd = new_fd_entry();
        (*fd).fm = fm;
        register_fd_entry(fd);
        (*fd).fd_objnum = pdfnewobjnum();
        if (*fm).ps_name.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"epdf_create_fontdescriptor\0" as *const u8 as *const ::core::ffi::c_char,
                b"epdf.c\0" as *const u8 as *const ::core::ffi::c_char,
                39 as ::core::ffi::c_int,
                b"fm->ps_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        (*fd).fontname = xstrdup((*fm).ps_name as const_string) as *mut ::core::ffi::c_char;
        (*fd).font_dim[STEMV_CODE as usize].val = stemV;
        (*fd).font_dim[STEMV_CODE as usize].set = true_0 as boolean;
        (*fd).gl_tree = avl_create(
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
        if (*fd).gl_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"epdf_create_fontdescriptor\0" as *const u8 as *const ::core::ffi::c_char,
                b"epdf.c\0" as *const u8 as *const ::core::ffi::c_char,
                45 as ::core::ffi::c_int,
                b"fd->gl_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn get_fd_objnum(mut fd: *mut fd_entry) -> ::core::ffi::c_int {
    if !((*fd).fd_objnum != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"get_fd_objnum\0" as *const u8 as *const ::core::ffi::c_char,
            b"epdf.c\0" as *const u8 as *const ::core::ffi::c_char,
            52 as ::core::ffi::c_int,
            b"fd->fd_objnum != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    return (*fd).fd_objnum as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_fn_objnum(mut fd: *mut fd_entry) -> ::core::ffi::c_int {
    if (*fd).fn_objnum == 0 as ::core::ffi::c_int {
        (*fd).fn_objnum = pdfnewobjnum();
    }
    return (*fd).fn_objnum as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn epdf_mark_glyphs(
    mut fd: *mut fd_entry,
    mut charset: *mut ::core::ffi::c_char,
) {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut glyph: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    if charset.is_null() {
        return;
    }
    if fd.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"epdf_mark_glyphs\0" as *const u8 as *const ::core::ffi::c_char,
            b"epdf.c\0" as *const u8 as *const ::core::ffi::c_char,
            83 as ::core::ffi::c_int,
            b"fd != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    while *charset as ::core::ffi::c_int == ' ' as i32
        || *charset as ::core::ffi::c_int == '\t' as i32
        || *charset as ::core::ffi::c_int == '\n' as i32
        || *charset as ::core::ffi::c_int == '\r' as i32
        || *charset as ::core::ffi::c_int == '\u{c}' as i32
    {
        charset = charset.offset(1);
    }
    s = charset.offset(1 as ::core::ffi::c_int as isize);
    q = charset.offset(strlen(charset) as isize);
    while s < q {
        p = s;
        while *p as ::core::ffi::c_int != '\0' as i32
            && *p as ::core::ffi::c_int != '/' as i32
            && !(*p as ::core::ffi::c_int == ' ' as i32
                || *p as ::core::ffi::c_int == '\t' as i32
                || *p as ::core::ffi::c_int == '\n' as i32
                || *p as ::core::ffi::c_int == '\r' as i32
                || *p as ::core::ffi::c_int == '\u{c}' as i32)
        {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int == ' ' as i32
            || *p as ::core::ffi::c_int == '\t' as i32
            || *p as ::core::ffi::c_int == '\n' as i32
            || *p as ::core::ffi::c_int == '\r' as i32
            || *p as ::core::ffi::c_int == '\u{c}' as i32
        {
            *p = '\0' as i32 as ::core::ffi::c_char;
            p = p.offset(1);
            while *p as ::core::ffi::c_int == ' ' as i32
                || *p as ::core::ffi::c_int == '\t' as i32
                || *p as ::core::ffi::c_int == '\n' as i32
                || *p as ::core::ffi::c_int == '\r' as i32
                || *p as ::core::ffi::c_int == '\u{c}' as i32
            {
                p = p.offset(1);
            }
        }
        *p = '\0' as i32 as ::core::ffi::c_char;
        if (avl_find((*fd).gl_tree, s as *const ::core::ffi::c_void) as *mut ::core::ffi::c_char)
            .is_null()
        {
            glyph = xstrdup(s as const_string) as *mut ::core::ffi::c_char;
            aa = avl_probe((*fd).gl_tree, glyph as *mut ::core::ffi::c_void);
            if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                __assert_rtn(
                    b"epdf_mark_glyphs\0" as *const u8 as *const ::core::ffi::c_char,
                    b"epdf.c\0" as *const u8 as *const ::core::ffi::c_char,
                    98 as ::core::ffi::c_int,
                    b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
        }
        s = p.offset(1 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn embed_whole_font(mut fd: *mut fd_entry) {
    (*fd).all_glyphs = true_0 as boolean;
}
#[no_mangle]
pub unsafe extern "C" fn epdf_free() {
    epdf_check_mem();
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
