extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
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
    fn xmalloc(size: size_t) -> address;
    fn zpdfbegindict(i: integer, pdfoslevel: integer);
    fn pdfenddict();
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
    static mut notdef: [::core::ffi::c_char; 0];
    fn pdf_printf(_: *const ::core::ffi::c_char, ...);
    fn pdf_puts(_: *const ::core::ffi::c_char);
    fn load_enc_file(_: *mut ::core::ffi::c_char) -> *mut *mut ::core::ffi::c_char;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type address = *mut ::core::ffi::c_void;
pub type integer = ::core::ffi::c_int;
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
pub struct fe_entry {
    pub fe_objnum: integer,
    pub name: *mut ::core::ffi::c_char,
    pub glyph_names: *mut *mut ::core::ffi::c_char,
    pub tx_tree: *mut avl_table,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut fe_tree: *mut avl_table = ::core::ptr::null::<avl_table>() as *mut avl_table;
unsafe extern "C" fn comp_fe_entry(
    mut pa: *const ::core::ffi::c_void,
    mut pb: *const ::core::ffi::c_void,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        (*(pa as *const fe_entry)).name,
        (*(pb as *const fe_entry)).name,
    );
}
unsafe extern "C" fn new_fe_entry() -> *mut fe_entry {
    let mut fe: *mut fe_entry = ::core::ptr::null_mut::<fe_entry>();
    fe = xmalloc((1 as size_t).wrapping_mul(::core::mem::size_of::<fe_entry>() as size_t))
        as *mut fe_entry;
    (*fe).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*fe).fe_objnum = 0 as ::core::ffi::c_int as integer;
    (*fe).glyph_names = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*fe).tx_tree = ::core::ptr::null_mut::<avl_table>();
    return fe;
}
unsafe extern "C" fn lookup_fe_entry(mut s: *mut ::core::ffi::c_char) -> *mut fe_entry {
    let mut fe: fe_entry = fe_entry {
        fe_objnum: 0,
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        glyph_names: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        tx_tree: ::core::ptr::null_mut::<avl_table>(),
    };
    if s.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"lookup_fe_entry\0" as *const u8 as *const ::core::ffi::c_char,
            b"writeenc.c\0" as *const u8 as *const ::core::ffi::c_char,
            48 as ::core::ffi::c_int,
            b"s != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    fe.name = s;
    if fe_tree.is_null() {
        fe_tree = avl_create(
            Some(
                comp_fe_entry
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
            &raw mut avl_xallocator,
        );
        if fe_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"lookup_fe_entry\0" as *const u8 as *const ::core::ffi::c_char,
                b"writeenc.c\0" as *const u8 as *const ::core::ffi::c_char,
                52 as ::core::ffi::c_int,
                b"fe_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    return avl_find(fe_tree, &raw mut fe as *const ::core::ffi::c_void) as *mut fe_entry;
}
unsafe extern "C" fn register_fe_entry(mut fe: *mut fe_entry) {
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    if fe_tree.is_null() {
        fe_tree = avl_create(
            Some(
                comp_fe_entry
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
            &raw mut avl_xallocator,
        );
        if fe_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"register_fe_entry\0" as *const u8 as *const ::core::ffi::c_char,
                b"writeenc.c\0" as *const u8 as *const ::core::ffi::c_char,
                62 as ::core::ffi::c_int,
                b"fe_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    if fe.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"register_fe_entry\0" as *const u8 as *const ::core::ffi::c_char,
            b"writeenc.c\0" as *const u8 as *const ::core::ffi::c_char,
            64 as ::core::ffi::c_int,
            b"fe != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if (*fe).name.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"register_fe_entry\0" as *const u8 as *const ::core::ffi::c_char,
            b"writeenc.c\0" as *const u8 as *const ::core::ffi::c_char,
            65 as ::core::ffi::c_int,
            b"fe->name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !lookup_fe_entry((*fe).name).is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"register_fe_entry\0" as *const u8 as *const ::core::ffi::c_char,
            b"writeenc.c\0" as *const u8 as *const ::core::ffi::c_char,
            66 as ::core::ffi::c_int,
            b"lookup_fe_entry(fe->name) == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    aa = avl_probe(fe_tree, fe as *mut ::core::ffi::c_void);
    if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"register_fe_entry\0" as *const u8 as *const ::core::ffi::c_char,
            b"writeenc.c\0" as *const u8 as *const ::core::ffi::c_char,
            68 as ::core::ffi::c_int,
            b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_fe_entry(mut s: *mut ::core::ffi::c_char) -> *mut fe_entry {
    let mut fe: *mut fe_entry = ::core::ptr::null_mut::<fe_entry>();
    let mut gl: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    fe = lookup_fe_entry(s);
    if fe.is_null() && {
        gl = load_enc_file(s);
        !gl.is_null()
    } {
        fe = new_fe_entry();
        (*fe).name = s;
        (*fe).glyph_names = gl;
        register_fe_entry(fe);
    }
    return fe;
}
#[no_mangle]
pub unsafe extern "C" fn epdf_write_enc(
    mut glyph_names: *mut *mut ::core::ffi::c_char,
    mut fe_objnum: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut i_old: ::core::ffi::c_int = 0;
    if glyph_names.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"epdf_write_enc\0" as *const u8 as *const ::core::ffi::c_char,
            b"writeenc.c\0" as *const u8 as *const ::core::ffi::c_char,
            89 as ::core::ffi::c_int,
            b"glyph_names != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !(fe_objnum != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"epdf_write_enc\0" as *const u8 as *const ::core::ffi::c_char,
            b"writeenc.c\0" as *const u8 as *const ::core::ffi::c_char,
            90 as ::core::ffi::c_int,
            b"fe_objnum != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    zpdfbegindict(fe_objnum, 1 as ::core::ffi::c_int);
    pdf_puts(b"/Type /Encoding\n\0" as *const u8 as *const ::core::ffi::c_char);
    pdf_puts(b"/Differences [\0" as *const u8 as *const ::core::ffi::c_char);
    i = 0 as ::core::ffi::c_int;
    i_old = -(2 as ::core::ffi::c_int);
    while i < 256 as ::core::ffi::c_int {
        if *glyph_names.offset(i as isize) != &raw mut notdef as *mut ::core::ffi::c_char {
            if i == i_old + 1 as ::core::ffi::c_int {
                crate::utils::pdf_printf_args(
                    b"/%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(
                        *glyph_names.offset(i as isize),
                    )],
                );
            } else if i_old == -(2 as ::core::ffi::c_int) {
                crate::utils::pdf_printf_args(
                    b"%i/%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(i),
                        crate::utils::PrintfArg::from(*glyph_names.offset(i as isize)),
                    ],
                );
            } else {
                crate::utils::pdf_printf_args(
                    b" %i/%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(i),
                        crate::utils::PrintfArg::from(*glyph_names.offset(i as isize)),
                    ],
                );
            }
            i_old = i;
        }
        i += 1;
    }
    pdf_puts(b"]\n\0" as *const u8 as *const ::core::ffi::c_char);
    pdfenddict();
}
unsafe extern "C" fn write_enc(
    mut glyph_names: *mut *mut ::core::ffi::c_char,
    mut tx_tree: *mut avl_table,
    mut fe_objnum: ::core::ffi::c_int,
) {
    let mut i_old: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut t: avl_traverser = avl_traverser {
        avl_table: ::core::ptr::null_mut::<avl_table>(),
        avl_node: ::core::ptr::null_mut::<avl_node>(),
        avl_stack: [::core::ptr::null_mut::<avl_node>(); 32],
        avl_height: 0,
        avl_generation: 0,
    };
    if glyph_names.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"write_enc\0" as *const u8 as *const ::core::ffi::c_char,
            b"writeenc.c\0" as *const u8 as *const ::core::ffi::c_char,
            114 as ::core::ffi::c_int,
            b"glyph_names != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if tx_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"write_enc\0" as *const u8 as *const ::core::ffi::c_char,
            b"writeenc.c\0" as *const u8 as *const ::core::ffi::c_char,
            115 as ::core::ffi::c_int,
            b"tx_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !(fe_objnum != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"write_enc\0" as *const u8 as *const ::core::ffi::c_char,
            b"writeenc.c\0" as *const u8 as *const ::core::ffi::c_char,
            116 as ::core::ffi::c_int,
            b"fe_objnum != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    zpdfbegindict(fe_objnum, 1 as ::core::ffi::c_int);
    pdf_puts(b"/Type /Encoding\n\0" as *const u8 as *const ::core::ffi::c_char);
    pdf_puts(b"/Differences [\0" as *const u8 as *const ::core::ffi::c_char);
    avl_t_init(&raw mut t, tx_tree);
    i_old = -(2 as ::core::ffi::c_int);
    p = avl_t_first(&raw mut t, tx_tree) as *mut ::core::ffi::c_int;
    while !p.is_null() {
        if *p == i_old + 1 as ::core::ffi::c_int {
            crate::utils::pdf_printf_args(
                b"/%s\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(
                    *glyph_names.offset(*p as isize),
                )],
            );
        } else if i_old == -(2 as ::core::ffi::c_int) {
            crate::utils::pdf_printf_args(
                b"%i/%s\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(*p),
                    crate::utils::PrintfArg::from(*glyph_names.offset(*p as isize)),
                ],
            );
        } else {
            crate::utils::pdf_printf_args(
                b" %i/%s\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(*p),
                    crate::utils::PrintfArg::from(*glyph_names.offset(*p as isize)),
                ],
            );
        }
        i_old = *p;
        p = avl_t_next(&raw mut t) as *mut ::core::ffi::c_int;
    }
    pdf_puts(b"]\n\0" as *const u8 as *const ::core::ffi::c_char);
    pdfenddict();
}
unsafe extern "C" fn write_fontencoding(mut fe: *mut fe_entry) {
    if fe.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"write_fontencoding\0" as *const u8 as *const ::core::ffi::c_char,
            b"writeenc.c\0" as *const u8 as *const ::core::ffi::c_char,
            139 as ::core::ffi::c_int,
            b"fe != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    write_enc(
        (*fe).glyph_names,
        (*fe).tx_tree,
        (*fe).fe_objnum as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn write_fontencodings() {
    let mut fe: *mut fe_entry = ::core::ptr::null_mut::<fe_entry>();
    let mut t: avl_traverser = avl_traverser {
        avl_table: ::core::ptr::null_mut::<avl_table>(),
        avl_node: ::core::ptr::null_mut::<avl_node>(),
        avl_stack: [::core::ptr::null_mut::<avl_node>(); 32],
        avl_height: 0,
        avl_generation: 0,
    };
    if fe_tree.is_null() {
        return;
    }
    avl_t_init(&raw mut t, fe_tree);
    fe = avl_t_first(&raw mut t, fe_tree) as *mut fe_entry;
    while !fe.is_null() {
        if (*fe).fe_objnum != 0 as ::core::ffi::c_int {
            write_fontencoding(fe);
        }
        fe = avl_t_next(&raw mut t) as *mut fe_entry;
    }
}
unsafe extern "C" fn destroy_fe_entry(
    mut pa: *mut ::core::ffi::c_void,
    mut pb: *mut ::core::ffi::c_void,
) {
    let mut p: *mut fe_entry = ::core::ptr::null_mut::<fe_entry>();
    let mut i: ::core::ffi::c_int = 0;
    p = pa as *mut fe_entry;
    if !(*p).name.is_null() {
        free((*p).name as *mut ::core::ffi::c_void);
    }
    (*p).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*p).glyph_names.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < 256 as ::core::ffi::c_int {
            if *(*p).glyph_names.offset(i as isize) != &raw mut notdef as *mut ::core::ffi::c_char {
                if !(*(*p).glyph_names.offset(i as isize)).is_null() {
                    free(*(*p).glyph_names.offset(i as isize) as *mut ::core::ffi::c_void);
                }
                let ref mut fresh0 = *(*p).glyph_names.offset(i as isize);
                *fresh0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            i += 1;
        }
    }
    if !(*p).glyph_names.is_null() {
        free((*p).glyph_names as *mut ::core::ffi::c_void);
    }
    (*p).glyph_names = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    if !p.is_null() {
        free(p as *mut ::core::ffi::c_void);
    }
    p = ::core::ptr::null_mut::<fe_entry>();
}
#[no_mangle]
pub unsafe extern "C" fn enc_free() {
    if !fe_tree.is_null() {
        avl_destroy(
            fe_tree,
            Some(
                destroy_fe_entry
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
        );
    }
    fe_tree = ::core::ptr::null_mut::<avl_table>();
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
