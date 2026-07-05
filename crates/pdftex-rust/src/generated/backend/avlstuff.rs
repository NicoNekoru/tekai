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
    static mut strpool: *mut packedASCIIcode;
    static mut strstart: *mut poolpointer;
    static mut objtab: *mut objentry;
    fn avl_create(
        _: Option<avl_comparison_func>,
        _: *mut ::core::ffi::c_void,
        _: *mut libavl_allocator,
    ) -> *mut avl_table;
    fn avl_probe(_: *mut avl_table, _: *mut ::core::ffi::c_void) -> *mut *mut ::core::ffi::c_void;
    fn avl_find(_: *const avl_table, _: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn pdftex_fail(_: *const ::core::ffi::c_char, ...) -> !;
}
pub type __int64_t = i64;
pub type __darwin_size_t = usize;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type off_t = __darwin_off_t;
pub type address = *mut ::core::ffi::c_void;
pub type integer = ::core::ffi::c_int;
pub type longinteger = off_t;
pub type poolpointer = integer;
pub type packedASCIIcode = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct objentry {
    pub int0: integer,
    pub int1: integer,
    pub int2: longinteger,
    pub int3: integer,
    pub int4: integer,
}
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
pub type oentry = oentry_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oentry_ {
    pub int0: integer,
    pub objptr: integer,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut PdfObjTree: [*mut avl_table; 11] = [
    ::core::ptr::null::<avl_table>() as *mut avl_table,
    ::core::ptr::null::<avl_table>() as *mut avl_table,
    ::core::ptr::null::<avl_table>() as *mut avl_table,
    ::core::ptr::null::<avl_table>() as *mut avl_table,
    ::core::ptr::null::<avl_table>() as *mut avl_table,
    ::core::ptr::null::<avl_table>() as *mut avl_table,
    ::core::ptr::null::<avl_table>() as *mut avl_table,
    ::core::ptr::null::<avl_table>() as *mut avl_table,
    ::core::ptr::null::<avl_table>() as *mut avl_table,
    ::core::ptr::null::<avl_table>() as *mut avl_table,
    ::core::ptr::null::<avl_table>() as *mut avl_table,
];
unsafe extern "C" fn avl_xmalloc(
    mut allocator: *mut libavl_allocator,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    if !(!allocator.is_null() && size > 0 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"avl_xmalloc\0" as *const u8 as *const ::core::ffi::c_char,
            b"avlstuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            31 as ::core::ffi::c_int,
            b"allocator != NULL && size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    return xmalloc(size) as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn avl_xfree(
    mut allocator: *mut libavl_allocator,
    mut block: *mut ::core::ffi::c_void,
) {
    if !(!allocator.is_null() && !block.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0
    {
        __assert_rtn(
            b"avl_xfree\0" as *const u8 as *const ::core::ffi::c_char,
            b"avlstuff.c\0" as *const u8 as *const ::core::ffi::c_char,
            37 as ::core::ffi::c_int,
            b"allocator != NULL && block != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !block.is_null() {
        free(block);
    }
    block = NULL;
}
#[no_mangle]
pub static mut avl_xallocator: libavl_allocator = unsafe {
    libavl_allocator {
        libavl_malloc: Some(
            avl_xmalloc
                as unsafe extern "C" fn(*mut libavl_allocator, size_t) -> *mut ::core::ffi::c_void,
        ),
        libavl_free: Some(
            avl_xfree
                as unsafe extern "C" fn(*mut libavl_allocator, *mut ::core::ffi::c_void) -> (),
        ),
    }
};
#[no_mangle]
pub unsafe extern "C" fn comp_int_entry(
    mut pa: *const ::core::ffi::c_void,
    mut pb: *const ::core::ffi::c_void,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if *(pa as *const ::core::ffi::c_int) > *(pb as *const ::core::ffi::c_int) {
        return 1 as ::core::ffi::c_int;
    }
    if *(pa as *const ::core::ffi::c_int) < *(pb as *const ::core::ffi::c_int) {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn comp_string_entry(
    mut pa: *const ::core::ffi::c_void,
    mut pb: *const ::core::ffi::c_void,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        pa as *const ::core::ffi::c_char,
        pb as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn compare_info(
    mut pa: *const ::core::ffi::c_void,
    mut pb: *const ::core::ffi::c_void,
    mut param: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut a: integer = 0;
    let mut b: integer = 0;
    let mut as_0: ::core::ffi::c_int = 0;
    let mut ae: ::core::ffi::c_int = 0;
    let mut bs: ::core::ffi::c_int = 0;
    let mut be: ::core::ffi::c_int = 0;
    let mut al: ::core::ffi::c_int = 0;
    let mut bl: ::core::ffi::c_int = 0;
    a = (*(pa as *const oentry)).int0;
    b = (*(pb as *const oentry)).int0;
    if a < 0 as ::core::ffi::c_int && b < 0 as ::core::ffi::c_int {
        as_0 = *strstart.offset(-a as isize) as ::core::ffi::c_int;
        ae = *strstart.offset((-(a as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int;
        bs = *strstart.offset(-b as isize) as ::core::ffi::c_int;
        be = *strstart.offset((-(b as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int;
        al = ae - as_0;
        bl = be - bs;
        if al < bl {
            return -(1 as ::core::ffi::c_int);
        }
        if al > bl {
            return 1 as ::core::ffi::c_int;
        }
        while as_0 < ae {
            if (*strpool.offset(as_0 as isize) as ::core::ffi::c_int)
                < *strpool.offset(bs as isize) as ::core::ffi::c_int
            {
                return -(1 as ::core::ffi::c_int);
            }
            if *strpool.offset(as_0 as isize) as ::core::ffi::c_int
                > *strpool.offset(bs as isize) as ::core::ffi::c_int
            {
                return 1 as ::core::ffi::c_int;
            }
            as_0 += 1;
            bs += 1;
        }
    } else {
        if a < b {
            return -(1 as ::core::ffi::c_int);
        }
        if a > b {
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avlputobj(mut objptr: integer, mut t: integer) {
    static mut pp: *mut *mut ::core::ffi::c_void =
        ::core::ptr::null::<*mut ::core::ffi::c_void>() as *mut *mut ::core::ffi::c_void;
    static mut oe: *mut oentry = ::core::ptr::null::<oentry>() as *mut oentry;
    if PdfObjTree[t as usize].is_null() {
        PdfObjTree[t as usize] = avl_create(
            Some(
                compare_info
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
            &raw mut avl_xallocator,
        );
        if PdfObjTree[t as usize].is_null() {
            crate::utils::pdftex_fail_args(b"avlstuff.c: avl_create() PdfObjTree failed\0" as *const u8
                    as *const ::core::ffi::c_char, &[]);
        }
    }
    oe = xmalloc((1 as size_t).wrapping_mul(::core::mem::size_of::<oentry>() as size_t))
        as *mut oentry;
    (*oe).int0 = (*objtab.offset(objptr as isize)).int0;
    (*oe).objptr = objptr;
    pp = avl_probe(PdfObjTree[t as usize], oe as *mut ::core::ffi::c_void);
    if pp.is_null() {
        crate::utils::pdftex_fail_args(b"avlstuff.c: avl_probe() out of memory in insertion\0" as *const u8
                as *const ::core::ffi::c_char, &[]);
    }
}
#[no_mangle]
pub unsafe extern "C" fn avlfindobj(
    mut t: integer,
    mut i: integer,
    mut byname: integer,
) -> integer {
    static mut p: *mut oentry = ::core::ptr::null::<oentry>() as *mut oentry;
    static mut tmp: oentry = oentry_ { int0: 0, objptr: 0 };
    if byname > 0 as ::core::ffi::c_int {
        tmp.int0 = -i;
    } else {
        tmp.int0 = i;
    }
    if PdfObjTree[t as usize].is_null() {
        return 0 as integer;
    }
    p = avl_find(
        PdfObjTree[t as usize],
        &raw mut tmp as *const ::core::ffi::c_void,
    ) as *mut oentry;
    if p.is_null() {
        return 0 as integer;
    }
    return (*p).objptr;
}
#[no_mangle]
pub static mut mf_tree: *mut avl_table = ::core::ptr::null::<avl_table>() as *mut avl_table;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
