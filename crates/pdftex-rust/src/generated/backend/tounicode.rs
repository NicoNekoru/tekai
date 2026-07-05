extern "C" {
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
    fn snprintf(
        __str: *mut ::core::ffi::c_char,
        __size: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn free(_: *mut ::core::ffi::c_void);
    fn strtol(
        __str: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
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
    fn strncat(
        __s1: *mut ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn xstrdup(s: const_string) -> string;
    fn xmalloc(size: size_t) -> address;
    static mut _DefaultRuneLocale: _RuneLocale;
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
    static mut fixedgentounicode: integer;
    static mut fmtfile: wordfile;
    fn pdfnewobjnum() -> integer;
    fn zpdfbegindict(i: integer, pdfoslevel: integer);
    fn pdfbeginstream();
    fn pdfendstream();
    fn avl_create(
        _: Option<avl_comparison_func>,
        _: *mut ::core::ffi::c_void,
        _: *mut libavl_allocator,
    ) -> *mut avl_table;
    fn avl_destroy(_: *mut avl_table, _: Option<avl_item_func>);
    fn avl_probe(_: *mut avl_table, _: *mut ::core::ffi::c_void) -> *mut *mut ::core::ffi::c_void;
    fn avl_find(_: *const avl_table, _: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn avl_t_init(_: *mut avl_traverser, _: *mut avl_table);
    fn avl_t_next(_: *mut avl_traverser) -> *mut ::core::ffi::c_void;
    static mut avl_xallocator: libavl_allocator;
    static mut notdef: [::core::ffi::c_char; 0];
    fn pdf_printf(_: *const ::core::ffi::c_char, ...);
    fn pdftex_fail(_: *const ::core::ffi::c_char, ...) -> !;
    fn pdftex_warn(_: *const ::core::ffi::c_char, ...);
}
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __darwin_ct_rune_t = ::core::ffi::c_int;
pub type __darwin_size_t = usize;
pub type __darwin_wchar_t = ::libc::wchar_t;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type off_t = __darwin_off_t;
pub type boolean = ::core::ffi::c_int;
pub type string = *mut ::core::ffi::c_char;
pub type const_string = *const ::core::ffi::c_char;
pub type address = *mut ::core::ffi::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: ::core::ffi::c_uint,
    pub next: *mut ::core::ffi::c_uchar,
    pub pos: off_t,
}
pub type gzFile = *mut gzFile_s;
pub type strnumber = integer;
pub type wordfile = gzFile;
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
pub struct glyph_unicode_entry {
    pub name: *mut ::core::ffi::c_char,
    pub code: integer,
    pub unicode_seq: *mut ::core::ffi::c_char,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SMALL_BUF_SIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
pub const UNI_UNDEF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const UNI_STRING: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const UNI_EXTRA_STRING: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
static mut glyph_unicode_tree: *mut avl_table = ::core::ptr::null::<avl_table>() as *mut avl_table;
unsafe extern "C" fn comp_glyph_unicode_entry(
    mut pa: *const ::core::ffi::c_void,
    mut pb: *const ::core::ffi::c_void,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        (*(pa as *const glyph_unicode_entry)).name,
        (*(pb as *const glyph_unicode_entry)).name,
    );
}
unsafe extern "C" fn new_glyph_unicode_entry() -> *mut glyph_unicode_entry {
    let mut e: *mut glyph_unicode_entry = ::core::ptr::null_mut::<glyph_unicode_entry>();
    e = xmalloc((1 as size_t).wrapping_mul(::core::mem::size_of::<glyph_unicode_entry>() as size_t))
        as *mut glyph_unicode_entry;
    (*e).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*e).code = UNI_UNDEF as integer;
    (*e).unicode_seq = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return e;
}
unsafe extern "C" fn destroy_glyph_unicode_entry(
    mut pa: *mut ::core::ffi::c_void,
    mut pb: *mut ::core::ffi::c_void,
) {
    let mut e: *mut glyph_unicode_entry = pa as *mut glyph_unicode_entry;
    if !(*e).name.is_null() {
        free((*e).name as *mut ::core::ffi::c_void);
    }
    (*e).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*e).code == UNI_STRING {
        if (*e).unicode_seq.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"destroy_glyph_unicode_entry\0" as *const u8 as *const ::core::ffi::c_char,
                b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
                50 as ::core::ffi::c_int,
                b"e->unicode_seq != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        if !(*e).unicode_seq.is_null() {
            free((*e).unicode_seq as *mut ::core::ffi::c_void);
        }
        (*e).unicode_seq = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn glyph_unicode_free() {
    if !glyph_unicode_tree.is_null() {
        avl_destroy(
            glyph_unicode_tree,
            Some(
                destroy_glyph_unicode_entry
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn deftounicode(mut glyph: strnumber, mut unistr: strnumber) {
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut buf2: [::core::ffi::c_char; 256] = [0; 256];
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut valid_unistr: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut gu: *mut glyph_unicode_entry = ::core::ptr::null_mut::<glyph_unicode_entry>();
    let mut t: glyph_unicode_entry = glyph_unicode_entry {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        code: 0,
        unicode_seq: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut sscan_result: ::core::ffi::c_ulong = 0;
    p = makecstring(glyph as integer);
    if !(strlen(p) < 256 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"deftounicode\0" as *const u8 as *const ::core::ffi::c_char,
            b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
            72 as ::core::ffi::c_int,
            b"strlen(p) < SMALL_BUF_SIZE\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    strcpy(&raw mut buf as *mut ::core::ffi::c_char, p);
    p = makecstring(unistr as integer);
    while *p as ::core::ffi::c_int == ' ' as i32 {
        p = p.offset(1);
    }
    l = strlen(p) as ::core::ffi::c_int;
    while l > 0 as ::core::ffi::c_int
        && *p.offset((l - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int == ' ' as i32
    {
        l -= 1;
    }
    valid_unistr = 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < l {
        if *p.offset(i as isize) as ::core::ffi::c_int == ' ' as i32 {
            valid_unistr = 2 as ::core::ffi::c_int;
        } else if !(isdigit(*p.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int)
            != 0
            || 'A' as i32 <= *p.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int
                && *p.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int
                    <= 'F' as i32)
        {
            valid_unistr = 0 as ::core::ffi::c_int;
            break;
        }
        i += 1;
    }
    if l == 0 as ::core::ffi::c_int
        || valid_unistr == 0 as ::core::ffi::c_int
        || strlen(&raw mut buf as *mut ::core::ffi::c_char) == 0 as size_t
        || strcmp(
            &raw mut buf as *mut ::core::ffi::c_char,
            &raw mut notdef as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        crate::utils::pdftex_warn_args(b"ToUnicode: invalid parameter(s): `%s' => `%s'\0" as *const u8
                as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(&raw mut buf as *mut ::core::ffi::c_char), crate::utils::PrintfArg::from(p)]);
        return;
    }
    if glyph_unicode_tree.is_null() {
        glyph_unicode_tree = avl_create(
            Some(
                comp_glyph_unicode_entry
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
            &raw mut avl_xallocator,
        );
        if glyph_unicode_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"deftounicode\0" as *const u8 as *const ::core::ffi::c_char,
                b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
                97 as ::core::ffi::c_int,
                b"glyph_unicode_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    t.name = &raw mut buf as *mut ::core::ffi::c_char;
    gu = avl_find(glyph_unicode_tree, &raw mut t as *const ::core::ffi::c_void)
        as *mut glyph_unicode_entry;
    if !gu.is_null() {
        if (*gu).code == UNI_STRING {
            if (*gu).unicode_seq.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                __assert_rtn(
                    b"deftounicode\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
                    103 as ::core::ffi::c_int,
                    b"gu->unicode_seq != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
            if !(*gu).unicode_seq.is_null() {
                free((*gu).unicode_seq as *mut ::core::ffi::c_void);
            }
            (*gu).unicode_seq = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    } else {
        gu = new_glyph_unicode_entry();
        (*gu).name = xstrdup(&raw mut buf as *mut ::core::ffi::c_char as const_string)
            as *mut ::core::ffi::c_char;
    }
    if valid_unistr == 2 as ::core::ffi::c_int {
        q = &raw mut buf2 as *mut ::core::ffi::c_char;
        while *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if *p as ::core::ffi::c_int != ' ' as i32 {
                let fresh0 = q;
                q = q.offset(1);
                *fresh0 = *p;
            }
            p = p.offset(1);
        }
        *q = 0 as ::core::ffi::c_char;
        (*gu).code = UNI_STRING as integer;
        (*gu).unicode_seq = xstrdup(&raw mut buf2 as *mut ::core::ffi::c_char as const_string)
            as *mut ::core::ffi::c_char;
    } else {
        i = sscanf(
            p,
            b"%lX\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut sscan_result,
        );
        if !(i == 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"deftounicode\0" as *const u8 as *const ::core::ffi::c_char,
                b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
                120 as ::core::ffi::c_int,
                b"i == 1\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        if sscan_result > 0x10ffff as ::core::ffi::c_ulong {
            crate::utils::pdftex_warn_args(b"ToUnicode: value out of range [0,10FFFF]: %lX\0" as *const u8
                    as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(sscan_result)]);
            (*gu).code = UNI_UNDEF as integer;
        } else {
            (*gu).code = sscan_result as integer;
        }
    }
    aa = avl_probe(glyph_unicode_tree, gu as *mut ::core::ffi::c_void);
    if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"deftounicode\0" as *const u8 as *const ::core::ffi::c_char,
            b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
            130 as ::core::ffi::c_int,
            b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
}
unsafe extern "C" fn check_unicode_value(
    mut s: *const ::core::ffi::c_char,
    mut multiple_value: boolean,
) -> ::core::ffi::c_long {
    let mut l: ::core::ffi::c_int = strlen(s) as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut code: ::core::ffi::c_long = 0;
    if l == 0 as ::core::ffi::c_int {
        return UNI_UNDEF as ::core::ffi::c_long;
    }
    if multiple_value != 0 && l % 4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        return UNI_UNDEF as ::core::ffi::c_long;
    }
    if multiple_value == 0 && !(4 as ::core::ffi::c_int <= l && l <= 6 as ::core::ffi::c_int) {
        return UNI_UNDEF as ::core::ffi::c_long;
    }
    i = 0 as ::core::ffi::c_int;
    while i < l {
        if !(isdigit(*s.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0
            || 'A' as i32 <= *s.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int
                && *s.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int
                    <= 'F' as i32)
        {
            return UNI_UNDEF as ::core::ffi::c_long;
        }
        if multiple_value != 0 {
            if i % 4 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
                if sscanf(
                    s.offset(i as isize)
                        .offset(-(3 as ::core::ffi::c_int as isize)),
                    b"%4lX\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut code,
                ) != 1 as ::core::ffi::c_int
                {
                    return UNI_UNDEF as ::core::ffi::c_long;
                }
                if !(0 as ::core::ffi::c_long <= code && code <= 0xd7ff as ::core::ffi::c_long
                    || 0xe000 as ::core::ffi::c_long <= code
                        && code <= 0xffff as ::core::ffi::c_long)
                {
                    return UNI_UNDEF as ::core::ffi::c_long;
                }
            }
        } else if i == l - 1 as ::core::ffi::c_int {
            if sscanf(
                s,
                b"%lX\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut code,
            ) != 1 as ::core::ffi::c_int
            {
                return UNI_UNDEF as ::core::ffi::c_long;
            }
            if !(0 as ::core::ffi::c_long <= code && code <= 0xd7ff as ::core::ffi::c_long
                || 0xe000 as ::core::ffi::c_long <= code && code <= 0x10ffff as ::core::ffi::c_long)
            {
                return UNI_UNDEF as ::core::ffi::c_long;
            }
        }
        i += 1;
    }
    return code;
}
unsafe extern "C" fn utf16be_str(mut code: ::core::ffi::c_long) -> *mut ::core::ffi::c_char {
    static mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut v: ::core::ffi::c_long = 0;
    let mut vh: ::core::ffi::c_uint = 0;
    let mut vl: ::core::ffi::c_uint = 0;
    if !(code >= 0 as ::core::ffi::c_long) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"utf16be_str\0" as *const u8 as *const ::core::ffi::c_char,
            b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
            177 as ::core::ffi::c_int,
            b"code >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if code <= 0xffff as ::core::ffi::c_long {
        sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"%04lX\0" as *const u8 as *const ::core::ffi::c_char,
            code,
        );
    } else {
        v = code - 0x10000 as ::core::ffi::c_long;
        vh = (v / 0x400 as ::core::ffi::c_long + 0xd800 as ::core::ffi::c_long)
            as ::core::ffi::c_uint;
        vl = (v % 0x400 as ::core::ffi::c_long + 0xdc00 as ::core::ffi::c_long)
            as ::core::ffi::c_uint;
        sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"%04X%04X\0" as *const u8 as *const ::core::ffi::c_char,
            vh,
            vl,
        );
    }
    return &raw mut buf as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn set_glyph_unicode(
    mut s: *const ::core::ffi::c_char,
    mut tfmname: *const ::core::ffi::c_char,
    mut gp: *mut glyph_unicode_entry,
) {
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut buf2: [::core::ffi::c_char; 256] = [0; 256];
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p2: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut code: ::core::ffi::c_long = 0;
    let mut last_component: boolean = 0;
    let mut tmp: glyph_unicode_entry = glyph_unicode_entry {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        code: 0,
        unicode_seq: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut ptmp: *mut glyph_unicode_entry = ::core::ptr::null_mut::<glyph_unicode_entry>();
    if s.is_null() || s == &raw mut notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char
    {
        return;
    }
    p = strchr(s, '.' as i32);
    if !p.is_null() {
        *(&raw mut buf as *mut ::core::ffi::c_char) = 0 as ::core::ffi::c_char;
        strncat(
            &raw mut buf as *mut ::core::ffi::c_char,
            s,
            p.offset_from(s) as ::core::ffi::c_long as size_t,
        );
        s = &raw mut buf as *mut ::core::ffi::c_char;
    }
    if strlen(s) == 0 as size_t {
        return;
    }
    p = strchr(s, '_' as i32);
    if !p.is_null() {
        if !(strlen(s) < ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as usize)
            as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            __assert_rtn(
                b"set_glyph_unicode\0" as *const u8 as *const ::core::ffi::c_char,
                b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
                222 as ::core::ffi::c_int,
                b"strlen(s) < sizeof(buf)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        if s != &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_char {
            strcpy(&raw mut buf as *mut ::core::ffi::c_char, s);
            p = strchr(&raw mut buf as *mut ::core::ffi::c_char, '_' as i32);
            s = &raw mut buf as *mut ::core::ffi::c_char;
        }
        *(&raw mut buf2 as *mut ::core::ffi::c_char) = 0 as ::core::ffi::c_char;
        last_component = false_0 as boolean;
        loop {
            *p = 0 as ::core::ffi::c_char;
            tmp.code = UNI_UNDEF as integer;
            set_glyph_unicode(s, tfmname, &raw mut tmp);
            match tmp.code {
                UNI_UNDEF => {}
                UNI_STRING => {
                    if tmp.unicode_seq.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                        __assert_rtn(
                            b"set_glyph_unicode\0" as *const u8 as *const ::core::ffi::c_char,
                            b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
                            238 as ::core::ffi::c_int,
                            b"tmp.unicode_seq != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                    };
                    if !(strlen(&raw mut buf2 as *mut ::core::ffi::c_char)
                        .wrapping_add(strlen(tmp.unicode_seq))
                        < ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as usize)
                        as ::core::ffi::c_int as ::core::ffi::c_long
                        != 0
                    {
                        __assert_rtn(
                            b"set_glyph_unicode\0" as *const u8 as *const ::core::ffi::c_char,
                            b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
                            239 as ::core::ffi::c_int,
                            b"strlen(buf2) + strlen(tmp.unicode_seq) < sizeof(buf2)\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    } else {
                    };
                    strcat(&raw mut buf2 as *mut ::core::ffi::c_char, tmp.unicode_seq);
                }
                UNI_EXTRA_STRING => {
                    if !(strlen(&raw mut buf2 as *mut ::core::ffi::c_char)
                        .wrapping_add(strlen(tmp.unicode_seq))
                        < ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as usize)
                        as ::core::ffi::c_int as ::core::ffi::c_long
                        != 0
                    {
                        __assert_rtn(
                            b"set_glyph_unicode\0" as *const u8 as *const ::core::ffi::c_char,
                            b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
                            243 as ::core::ffi::c_int,
                            b"strlen(buf2) + strlen(tmp.unicode_seq) < sizeof(buf2)\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    } else {
                    };
                    strcat(&raw mut buf2 as *mut ::core::ffi::c_char, tmp.unicode_seq);
                    if !tmp.unicode_seq.is_null() {
                        free(tmp.unicode_seq as *mut ::core::ffi::c_void);
                    }
                    tmp.unicode_seq = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                _ => {
                    if !(tmp.code >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as ::core::ffi::c_long
                        != 0
                    {
                        __assert_rtn(
                            b"set_glyph_unicode\0" as *const u8 as *const ::core::ffi::c_char,
                            b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
                            249 as ::core::ffi::c_int,
                            b"tmp.code >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                    };
                    strcat(
                        &raw mut buf2 as *mut ::core::ffi::c_char,
                        utf16be_str(tmp.code as ::core::ffi::c_long),
                    );
                }
            }
            if last_component != 0 {
                break;
            }
            s = p.offset(1 as ::core::ffi::c_int as isize);
            p = strchr(s, '_' as i32);
            if p.is_null() {
                p = strchr(s, 0 as ::core::ffi::c_int);
                last_component = true_0 as boolean;
            }
        }
        (*gp).code = UNI_EXTRA_STRING as integer;
        (*gp).unicode_seq = xstrdup(&raw mut buf2 as *mut ::core::ffi::c_char as const_string)
            as *mut ::core::ffi::c_char;
        return;
    }
    snprintf(
        &raw mut buf2 as *mut ::core::ffi::c_char,
        SMALL_BUF_SIZE as size_t,
        b"tfm:%s/%s\0" as *const u8 as *const ::core::ffi::c_char,
        tfmname,
        s,
    );
    tmp.name = &raw mut buf2 as *mut ::core::ffi::c_char;
    tmp.code = UNI_UNDEF as integer;
    ptmp = avl_find(
        glyph_unicode_tree,
        &raw mut tmp as *const ::core::ffi::c_void,
    ) as *mut glyph_unicode_entry;
    if !ptmp.is_null() {
        (*gp).code = (*ptmp).code;
        (*gp).unicode_seq = (*ptmp).unicode_seq;
        return;
    }
    snprintf(
        &raw mut buf2 as *mut ::core::ffi::c_char,
        SMALL_BUF_SIZE as size_t,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        s,
    );
    tmp.name = &raw mut buf2 as *mut ::core::ffi::c_char;
    tmp.code = UNI_UNDEF as integer;
    ptmp = avl_find(
        glyph_unicode_tree,
        &raw mut tmp as *const ::core::ffi::c_void,
    ) as *mut glyph_unicode_entry;
    if !ptmp.is_null() {
        (*gp).code = (*ptmp).code;
        (*gp).unicode_seq = (*ptmp).unicode_seq;
        return;
    }
    if strncmp(
        s,
        b"uni\0" as *const u8 as *const ::core::ffi::c_char,
        strlen(b"uni\0" as *const u8 as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int
    {
        p2 = s.offset(strlen(b"uni\0" as *const u8 as *const ::core::ffi::c_char) as isize);
        code = check_unicode_value(p2, true_0);
        if code != UNI_UNDEF as ::core::ffi::c_long {
            if strlen(p2) == 4 as size_t {
                (*gp).code = code as integer;
            } else {
                (*gp).code = UNI_EXTRA_STRING as integer;
                (*gp).unicode_seq = xstrdup(p2 as const_string) as *mut ::core::ffi::c_char;
            }
        }
        return;
    }
    if strncmp(
        s,
        b"u\0" as *const u8 as *const ::core::ffi::c_char,
        strlen(b"u\0" as *const u8 as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int
    {
        p2 = s.offset(strlen(b"u\0" as *const u8 as *const ::core::ffi::c_char) as isize);
        code = check_unicode_value(p2, false_0);
        if code != UNI_UNDEF as ::core::ffi::c_long {
            if !(code >= 0 as ::core::ffi::c_long) as ::core::ffi::c_int as ::core::ffi::c_long != 0
            {
                __assert_rtn(
                    b"set_glyph_unicode\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
                    314 as ::core::ffi::c_int,
                    b"code >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
            (*gp).code = code as integer;
        }
    }
}
unsafe extern "C" fn is_last_byte_valid(
    mut srcCode1: ::core::ffi::c_int,
    mut srcCode2: ::core::ffi::c_int,
    mut code: ::core::ffi::c_long,
) -> boolean {
    let mut s: *mut ::core::ffi::c_char = strchr(utf16be_str(code), 0 as ::core::ffi::c_int)
        .offset(-(2 as ::core::ffi::c_int as isize));
    let mut l: ::core::ffi::c_long = strtol(
        s,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        16 as ::core::ffi::c_int,
    );
    return (l < (255 as ::core::ffi::c_int - (srcCode2 - srcCode1)) as ::core::ffi::c_long)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn write_tounicode(
    mut glyph_names: *mut *mut ::core::ffi::c_char,
    mut tfmname: *const ::core::ffi::c_char,
    mut encname: *const ::core::ffi::c_char,
) -> integer {
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    static mut builtin_suffix: [::core::ffi::c_char; 8] =
        unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"builtin\0") };
    let mut range_size: [::core::ffi::c_short; 257] = [0; 257];
    let mut gtab: [glyph_unicode_entry; 257] = [glyph_unicode_entry {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        code: 0,
        unicode_seq: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    }; 257];
    let mut objnum: integer = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut bfchar_count: ::core::ffi::c_int = 0;
    let mut bfrange_count: ::core::ffi::c_int = 0;
    let mut subrange_count: ::core::ffi::c_int = 0;
    if glyph_unicode_tree.is_null() {
        crate::utils::pdftex_warn_args(b"no GlyphToUnicode entry has been inserted yet!\0" as *const u8
                as *const ::core::ffi::c_char, &[]);
        fixedgentounicode = 0 as ::core::ffi::c_int as integer;
        return 0 as integer;
    }
    strcpy(&raw mut buf as *mut ::core::ffi::c_char, tfmname);
    strcat(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"-\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !encname.is_null() {
        if !(strlen(tfmname)
            .wrapping_add(strlen(encname))
            .wrapping_add(1 as size_t)
            < 256 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            __assert_rtn(
                b"write_tounicode\0" as *const u8 as *const ::core::ffi::c_char,
                b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
                357 as ::core::ffi::c_int,
                b"strlen(tfmname) + strlen(encname) + 1 < SMALL_BUF_SIZE\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        } else {
        };
        strcat(&raw mut buf as *mut ::core::ffi::c_char, encname);
        p = strrchr(&raw mut buf as *mut ::core::ffi::c_char, '.' as i32);
        if !p.is_null()
            && strcmp(p, b".enc\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            *p = 0 as ::core::ffi::c_char;
        } else {
            crate::utils::pdftex_warn_args(b"Dubious encoding file name: `%s'\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(encname)]);
        }
    } else {
        if !(strlen(tfmname)
            .wrapping_add(strlen(&raw mut builtin_suffix as *mut ::core::ffi::c_char))
            .wrapping_add(1 as size_t)
            < 256 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            __assert_rtn(
                b"write_tounicode\0" as *const u8 as *const ::core::ffi::c_char,
                b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
                364 as ::core::ffi::c_int,
                b"strlen(tfmname) + strlen(builtin_suffix) + 1 < SMALL_BUF_SIZE\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        } else {
        };
        strcat(
            &raw mut buf as *mut ::core::ffi::c_char,
            &raw mut builtin_suffix as *mut ::core::ffi::c_char,
        );
    }
    objnum = pdfnewobjnum();
    zpdfbegindict(objnum, 0 as ::core::ffi::c_int);
    pdfbeginstream();
    crate::utils::pdf_printf_args(b"%%!PS-Adobe-3.0 Resource-CMap\n%%%%DocumentNeededResources: ProcSet (CIDInit)\n%%%%IncludeResource: ProcSet (CIDInit)\n%%%%BeginResource: CMap (TeX-%s-0)\n%%%%Title: (TeX-%s-0 TeX %s 0)\n%%%%Version: 1.000\n%%%%EndComments\n/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CIDSystemInfo\n<< /Registry (TeX)\n/Ordering (%s)\n/Supplement 0\n>> def\n/CMapName /TeX-%s-0 def\n/CMapType 2 def\n1 begincodespacerange\n<00> <FF>\nendcodespacerange\n\0"
            as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(&raw mut buf as *mut ::core::ffi::c_char), crate::utils::PrintfArg::from(&raw mut buf as *mut ::core::ffi::c_char), crate::utils::PrintfArg::from(&raw mut buf as *mut ::core::ffi::c_char), crate::utils::PrintfArg::from(&raw mut buf as *mut ::core::ffi::c_char), crate::utils::PrintfArg::from(&raw mut buf as *mut ::core::ffi::c_char)]);
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        gtab[i as usize].code = UNI_UNDEF as integer;
        set_glyph_unicode(
            *glyph_names.offset(i as isize),
            tfmname,
            (&raw mut gtab as *mut glyph_unicode_entry).offset(i as isize)
                as *mut glyph_unicode_entry,
        );
        i += 1;
    }
    gtab[256 as ::core::ffi::c_int as usize].code = UNI_UNDEF as integer;
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        if gtab[i as usize].code == UNI_STRING || gtab[i as usize].code == UNI_EXTRA_STRING {
            range_size[i as usize] = 1 as ::core::ffi::c_short;
            i += 1;
        } else if gtab[i as usize].code == UNI_UNDEF {
            range_size[i as usize] = 0 as ::core::ffi::c_short;
            i += 1;
        } else {
            j = i;
            while i < 256 as ::core::ffi::c_int
                && gtab[(i + 1 as ::core::ffi::c_int) as usize].code >= 0 as ::core::ffi::c_int
                && gtab[i as usize].code as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    == gtab[(i + 1 as ::core::ffi::c_int) as usize].code
                && is_last_byte_valid(j, i, gtab[i as usize].code as ::core::ffi::c_long) != 0
            {
                i += 1;
            }
            i += 1;
            range_size[j as usize] = (i - j) as ::core::ffi::c_short;
        }
    }
    bfrange_count = 0 as ::core::ffi::c_int;
    bfchar_count = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        if range_size[i as usize] as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            bfchar_count += 1;
            i += 1;
        } else if range_size[i as usize] as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            bfrange_count += 1;
            i += range_size[i as usize] as ::core::ffi::c_int;
        } else {
            i += 1;
        }
    }
    i = 0 as ::core::ffi::c_int;
    loop {
        if bfrange_count > 100 as ::core::ffi::c_int {
            subrange_count = 100 as ::core::ffi::c_int;
        } else {
            subrange_count = bfrange_count;
        }
        bfrange_count -= subrange_count;
        crate::utils::pdf_printf_args(b"%i beginbfrange\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(subrange_count)]);
        j = 0 as ::core::ffi::c_int;
        while j < subrange_count {
            while range_size[i as usize] as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
                && i < 256 as ::core::ffi::c_int
            {
                i += 1;
            }
            if !(i < 256 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                __assert_rtn(
                    b"write_tounicode\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
                    445 as ::core::ffi::c_int,
                    b"i < 256\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
            crate::utils::pdf_printf_args(b"<%02X> <%02X> <%s>\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(i), crate::utils::PrintfArg::from(i + range_size[i as usize] as ::core::ffi::c_int - 1 as ::core::ffi::c_int), crate::utils::PrintfArg::from(utf16be_str(gtab[i as usize].code as ::core::ffi::c_long))]);
            i += range_size[i as usize] as ::core::ffi::c_int;
            j += 1;
        }
        crate::utils::pdf_printf_args(b"endbfrange\n\0" as *const u8 as *const ::core::ffi::c_char, &[]);
        if !(bfrange_count > 0 as ::core::ffi::c_int) {
            break;
        }
    }
    i = 0 as ::core::ffi::c_int;
    loop {
        if bfchar_count > 100 as ::core::ffi::c_int {
            subrange_count = 100 as ::core::ffi::c_int;
        } else {
            subrange_count = bfchar_count;
        }
        bfchar_count -= subrange_count;
        crate::utils::pdf_printf_args(b"%i beginbfchar\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(subrange_count)]);
        j = 0 as ::core::ffi::c_int;
        while j < subrange_count {
            while i < 256 as ::core::ffi::c_int {
                if range_size[i as usize] as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    i += range_size[i as usize] as ::core::ffi::c_int;
                } else {
                    if !(range_size[i as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
                        break;
                    }
                    i += 1;
                }
            }
            if !(i < 256 as ::core::ffi::c_int
                && gtab[i as usize].code != -(1 as ::core::ffi::c_int))
                as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                __assert_rtn(
                    b"write_tounicode\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
                    472 as ::core::ffi::c_int,
                    b"i < 256 && gtab[i].code != UNI_UNDEF\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            } else {
            };
            if gtab[i as usize].code == UNI_STRING || gtab[i as usize].code == UNI_EXTRA_STRING {
                if gtab[i as usize].unicode_seq.is_null() as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0
                {
                    __assert_rtn(
                        b"write_tounicode\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
                        474 as ::core::ffi::c_int,
                        b"gtab[i].unicode_seq != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                };
                crate::utils::pdf_printf_args(b"<%02X> <%s>\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(i), crate::utils::PrintfArg::from(gtab[i as usize].unicode_seq)]);
            } else {
                crate::utils::pdf_printf_args(b"<%02X> <%s>\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(i), crate::utils::PrintfArg::from(utf16be_str(gtab[i as usize].code as ::core::ffi::c_long))]);
            }
            i += 1;
            j += 1;
        }
        crate::utils::pdf_printf_args(b"endbfchar\n\0" as *const u8 as *const ::core::ffi::c_char, &[]);
        if !(bfchar_count > 0 as ::core::ffi::c_int) {
            break;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        if gtab[i as usize].code == UNI_EXTRA_STRING {
            if !gtab[i as usize].unicode_seq.is_null() {
                free(gtab[i as usize].unicode_seq as *mut ::core::ffi::c_void);
            }
            gtab[i as usize].unicode_seq = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        i += 1;
    }
    crate::utils::pdf_printf_args(b"endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n%%%%EndResource\n%%%%EOF\n\0"
            as *const u8 as *const ::core::ffi::c_char, &[]);
    pdfendstream();
    return objnum;
}
#[no_mangle]
pub unsafe extern "C" fn dumptounicode() {
    let mut traverse: avl_traverser = avl_traverser {
        avl_table: ::core::ptr::null_mut::<avl_table>(),
        avl_node: ::core::ptr::null_mut::<avl_node>(),
        avl_stack: [::core::ptr::null_mut::<avl_node>(); 32],
        avl_height: 0,
        avl_generation: 0,
    };
    let mut count: integer = 0;
    let mut gu: *mut glyph_unicode_entry = ::core::ptr::null_mut::<glyph_unicode_entry>();
    if glyph_unicode_tree.is_null() {
        count = 0 as ::core::ffi::c_int as integer;
        do_dump(
            &raw mut count as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        return;
    }
    count = (*glyph_unicode_tree).avl_count as integer;
    do_dump(
        &raw mut count as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    avl_t_init(&raw mut traverse, glyph_unicode_tree);
    loop {
        gu = avl_t_next(&raw mut traverse) as *mut glyph_unicode_entry;
        if gu.is_null() {
            break;
        }
        let mut x: integer = 0;
        if !(*gu).name.is_null() {
            x = strlen((*gu).name).wrapping_add(1 as size_t) as integer;
            do_dump(
                &raw mut x as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                fmtfile as gzFile,
            );
            do_dump(
                (*gu).name,
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
            &raw mut (*gu).code as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        if (*gu).code == UNI_STRING {
            let mut x_0: integer = 0;
            if !(*gu).unicode_seq.is_null() {
                x_0 = strlen((*gu).unicode_seq).wrapping_add(1 as size_t) as integer;
                do_dump(
                    &raw mut x_0 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    fmtfile as gzFile,
                );
                do_dump(
                    (*gu).unicode_seq,
                    ::core::mem::size_of::<::core::ffi::c_char>() as ::core::ffi::c_int,
                    x_0,
                    fmtfile as gzFile,
                );
            } else {
                x_0 = 0 as ::core::ffi::c_int as integer;
                do_dump(
                    &raw mut x_0 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    fmtfile as gzFile,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn undumptounicode() {
    let mut tmp: *mut glyph_unicode_entry = ::core::ptr::null_mut::<glyph_unicode_entry>();
    let mut remaining: integer = 0;
    do_undump(
        &raw mut remaining as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    if remaining == 0 as ::core::ffi::c_int {
        return;
    }
    if !glyph_unicode_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"undumptounicode\0" as *const u8 as *const ::core::ffi::c_char,
            b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
            532 as ::core::ffi::c_int,
            b"glyph_unicode_tree == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    glyph_unicode_tree = avl_create(
        Some(
            comp_glyph_unicode_entry
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        NULL,
        &raw mut avl_xallocator,
    );
    if glyph_unicode_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"undumptounicode\0" as *const u8 as *const ::core::ffi::c_char,
            b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
            535 as ::core::ffi::c_int,
            b"glyph_unicode_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    loop {
        let fresh1 = remaining;
        remaining = remaining - 1;
        if !(fresh1 != 0) {
            break;
        }
        let mut result: *mut *mut ::core::ffi::c_void =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
        let mut gu: *mut glyph_unicode_entry = new_glyph_unicode_entry();
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
            (*gu).name = a;
        } else {
            (*gu).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if (*gu).name.is_null() {
            crate::utils::pdftex_fail_args(b"undumpcharptr(gu->name) got NULL\0" as *const u8 as *const ::core::ffi::c_char, &[]);
        }
        do_undump(
            &raw mut (*gu).code as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        if (*gu).code == UNI_STRING {
            let mut x_0: integer = 0;
            let mut a_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            do_undump(
                &raw mut x_0 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                fmtfile as gzFile,
            );
            if x_0 > 0 as ::core::ffi::c_int {
                a_0 = xmalloc(x_0 as size_t) as *mut ::core::ffi::c_char;
                do_undump(
                    a_0,
                    ::core::mem::size_of::<::core::ffi::c_char>() as ::core::ffi::c_int,
                    x_0,
                    fmtfile as gzFile,
                );
                (*gu).unicode_seq = a_0;
            } else {
                (*gu).unicode_seq = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            if (*gu).unicode_seq.is_null() {
                crate::utils::pdftex_fail_args(b"undumpcharptr(gu->unicode_seq) got NULL\0" as *const u8
                        as *const ::core::ffi::c_char, &[]);
            }
        }
        result = avl_probe(glyph_unicode_tree, gu as *mut ::core::ffi::c_void);
        if !(*result == gu as *mut ::core::ffi::c_void) as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            __assert_rtn(
                b"undumptounicode\0" as *const u8 as *const ::core::ffi::c_char,
                b"tounicode.c\0" as *const u8 as *const ::core::ffi::c_char,
                554 as ::core::ffi::c_int,
                b"*result == gu\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
