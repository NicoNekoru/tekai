extern "C" {
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type avl_comparison_func = unsafe extern "C" fn(
    *const ::core::ffi::c_void,
    *const ::core::ffi::c_void,
    *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int;
pub type avl_item_func =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ();
pub type avl_copy_func = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void;
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
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
#[no_mangle]
pub unsafe extern "C" fn avl_create(
    mut compare: Option<avl_comparison_func>,
    mut param: *mut ::core::ffi::c_void,
    mut allocator: *mut libavl_allocator,
) -> *mut avl_table {
    let mut tree: *mut avl_table = ::core::ptr::null_mut::<avl_table>();
    if compare.is_none() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"avl_create\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            41 as ::core::ffi::c_int,
            b"compare != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if allocator.is_null() {
        allocator = &raw mut avl_allocator_default;
    }
    tree = (*allocator)
        .libavl_malloc
        .expect("non-null function pointer")(
        allocator,
        ::core::mem::size_of::<avl_table>() as size_t,
    ) as *mut avl_table;
    if tree.is_null() {
        return ::core::ptr::null_mut::<avl_table>();
    }
    (*tree).avl_root = ::core::ptr::null_mut::<avl_node>();
    (*tree).avl_compare = compare;
    (*tree).avl_param = param;
    (*tree).avl_alloc = allocator;
    (*tree).avl_count = 0 as size_t;
    (*tree).avl_generation = 0 as ::core::ffi::c_ulong;
    return tree;
}
#[no_mangle]
pub unsafe extern "C" fn avl_find(
    mut tree: *const avl_table,
    mut item: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut p: *const avl_node = ::core::ptr::null::<avl_node>();
    if !(!tree.is_null() && !item.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"avl_find\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            66 as ::core::ffi::c_int,
            b"tree != NULL && item != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    p = (*tree).avl_root;
    while !p.is_null() {
        let mut cmp: ::core::ffi::c_int = (*tree).avl_compare.expect("non-null function pointer")(
            item,
            (*p).avl_data,
            (*tree).avl_param,
        );
        if cmp < 0 as ::core::ffi::c_int {
            p = (*p).avl_link[0 as ::core::ffi::c_int as usize];
        } else if cmp > 0 as ::core::ffi::c_int {
            p = (*p).avl_link[1 as ::core::ffi::c_int as usize];
        } else {
            return (*p).avl_data;
        }
    }
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn avl_probe(
    mut tree: *mut avl_table,
    mut item: *mut ::core::ffi::c_void,
) -> *mut *mut ::core::ffi::c_void {
    let mut y: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
    let mut z: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
    let mut p: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
    let mut q: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
    let mut n: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
    let mut w: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
    let mut dir: ::core::ffi::c_int = 0;
    let mut da: [::core::ffi::c_uchar; 32] = [0; 32];
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(!tree.is_null() && !item.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"avl_probe\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            96 as ::core::ffi::c_int,
            b"tree != NULL && item != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    z = &raw mut (*tree).avl_root as *mut avl_node;
    y = (*tree).avl_root as *mut avl_node;
    dir = 0 as ::core::ffi::c_int;
    q = z;
    p = y;
    while !p.is_null() {
        let mut cmp: ::core::ffi::c_int = (*tree).avl_compare.expect("non-null function pointer")(
            item,
            (*p).avl_data,
            (*tree).avl_param,
        );
        if cmp == 0 as ::core::ffi::c_int {
            return &raw mut (*p).avl_data;
        }
        if (*p).avl_balance as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            z = q;
            y = p;
            k = 0 as ::core::ffi::c_int;
        }
        dir = (cmp > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        let fresh3 = k;
        k = k + 1;
        da[fresh3 as usize] = dir as ::core::ffi::c_uchar;
        q = p;
        p = (*p).avl_link[dir as usize];
    }
    (*q).avl_link[dir as usize] = (*(*tree).avl_alloc)
        .libavl_malloc
        .expect("non-null function pointer")(
        (*tree).avl_alloc,
        ::core::mem::size_of::<avl_node>() as size_t,
    ) as *mut avl_node;
    n = (*q).avl_link[dir as usize];
    if n.is_null() {
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    }
    (*tree).avl_count = (*tree).avl_count.wrapping_add(1);
    (*n).avl_data = item;
    (*n).avl_link[1 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<avl_node>();
    (*n).avl_link[0 as ::core::ffi::c_int as usize] =
        (*n).avl_link[1 as ::core::ffi::c_int as usize];
    (*n).avl_balance = 0 as ::core::ffi::c_schar;
    if y.is_null() {
        return &raw mut (*n).avl_data;
    }
    p = y;
    k = 0 as ::core::ffi::c_int;
    while p != n {
        if da[k as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (*p).avl_balance -= 1;
        } else {
            (*p).avl_balance += 1;
        }
        p = (*p).avl_link[da[k as usize] as usize];
        k += 1;
    }
    if (*y).avl_balance as ::core::ffi::c_int == -(2 as ::core::ffi::c_int) {
        let mut x: *mut avl_node = (*y).avl_link[0 as ::core::ffi::c_int as usize];
        if (*x).avl_balance as ::core::ffi::c_int == -(1 as ::core::ffi::c_int) {
            w = x;
            (*y).avl_link[0 as ::core::ffi::c_int as usize] =
                (*x).avl_link[1 as ::core::ffi::c_int as usize];
            (*x).avl_link[1 as ::core::ffi::c_int as usize] = y;
            (*y).avl_balance = 0 as ::core::ffi::c_schar;
            (*x).avl_balance = (*y).avl_balance;
        } else {
            if !((*x).avl_balance as ::core::ffi::c_int == 1 as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                __assert_rtn(
                    b"avl_probe\0" as *const u8 as *const ::core::ffi::c_char,
                    b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
                    137 as ::core::ffi::c_int,
                    b"x->avl_balance == +1\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
            w = (*x).avl_link[1 as ::core::ffi::c_int as usize];
            (*x).avl_link[1 as ::core::ffi::c_int as usize] =
                (*w).avl_link[0 as ::core::ffi::c_int as usize];
            (*w).avl_link[0 as ::core::ffi::c_int as usize] = x;
            (*y).avl_link[0 as ::core::ffi::c_int as usize] =
                (*w).avl_link[1 as ::core::ffi::c_int as usize];
            (*w).avl_link[1 as ::core::ffi::c_int as usize] = y;
            if (*w).avl_balance as ::core::ffi::c_int == -(1 as ::core::ffi::c_int) {
                (*x).avl_balance = 0 as ::core::ffi::c_schar;
                (*y).avl_balance = 1 as ::core::ffi::c_int as ::core::ffi::c_schar;
            } else if (*w).avl_balance as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*y).avl_balance = 0 as ::core::ffi::c_schar;
                (*x).avl_balance = (*y).avl_balance;
            } else {
                (*x).avl_balance = -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar;
                (*y).avl_balance = 0 as ::core::ffi::c_schar;
            }
            (*w).avl_balance = 0 as ::core::ffi::c_schar;
        }
    } else if (*y).avl_balance as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        let mut x_0: *mut avl_node = (*y).avl_link[1 as ::core::ffi::c_int as usize];
        if (*x_0).avl_balance as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            w = x_0;
            (*y).avl_link[1 as ::core::ffi::c_int as usize] =
                (*x_0).avl_link[0 as ::core::ffi::c_int as usize];
            (*x_0).avl_link[0 as ::core::ffi::c_int as usize] = y;
            (*y).avl_balance = 0 as ::core::ffi::c_schar;
            (*x_0).avl_balance = (*y).avl_balance;
        } else {
            if !((*x_0).avl_balance as ::core::ffi::c_int == -(1 as ::core::ffi::c_int))
                as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                __assert_rtn(
                    b"avl_probe\0" as *const u8 as *const ::core::ffi::c_char,
                    b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
                    159 as ::core::ffi::c_int,
                    b"x->avl_balance == -1\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
            w = (*x_0).avl_link[0 as ::core::ffi::c_int as usize];
            (*x_0).avl_link[0 as ::core::ffi::c_int as usize] =
                (*w).avl_link[1 as ::core::ffi::c_int as usize];
            (*w).avl_link[1 as ::core::ffi::c_int as usize] = x_0;
            (*y).avl_link[1 as ::core::ffi::c_int as usize] =
                (*w).avl_link[0 as ::core::ffi::c_int as usize];
            (*w).avl_link[0 as ::core::ffi::c_int as usize] = y;
            if (*w).avl_balance as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                (*x_0).avl_balance = 0 as ::core::ffi::c_schar;
                (*y).avl_balance = -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar;
            } else if (*w).avl_balance as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*y).avl_balance = 0 as ::core::ffi::c_schar;
                (*x_0).avl_balance = (*y).avl_balance;
            } else {
                (*x_0).avl_balance = 1 as ::core::ffi::c_int as ::core::ffi::c_schar;
                (*y).avl_balance = 0 as ::core::ffi::c_schar;
            }
            (*w).avl_balance = 0 as ::core::ffi::c_schar;
        }
    } else {
        return &raw mut (*n).avl_data;
    }
    (*z).avl_link
        [(y != (*z).avl_link[0 as ::core::ffi::c_int as usize]) as ::core::ffi::c_int as usize] = w;
    (*tree).avl_generation = (*tree).avl_generation.wrapping_add(1);
    return &raw mut (*n).avl_data;
}
#[no_mangle]
pub unsafe extern "C" fn avl_insert(
    mut table: *mut avl_table,
    mut item: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut p: *mut *mut ::core::ffi::c_void = avl_probe(table, item);
    return if p.is_null() || *p == item { NULL } else { *p };
}
#[no_mangle]
pub unsafe extern "C" fn avl_replace(
    mut table: *mut avl_table,
    mut item: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut p: *mut *mut ::core::ffi::c_void = avl_probe(table, item);
    if p.is_null() || *p == item {
        return NULL;
    } else {
        let mut r: *mut ::core::ffi::c_void = *p;
        *p = item;
        return r;
    };
}
#[no_mangle]
pub unsafe extern "C" fn avl_delete(
    mut tree: *mut avl_table,
    mut item: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut pa: [*mut avl_node; 32] = [::core::ptr::null_mut::<avl_node>(); 32];
    let mut da: [::core::ffi::c_uchar; 32] = [0; 32];
    let mut k: ::core::ffi::c_int = 0;
    let mut p: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
    let mut cmp: ::core::ffi::c_int = 0;
    let mut res: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if !(!tree.is_null() && !item.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"avl_delete\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            221 as ::core::ffi::c_int,
            b"tree != NULL && item != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    k = 0 as ::core::ffi::c_int;
    p = &raw mut (*tree).avl_root as *mut avl_node;
    cmp = -(1 as ::core::ffi::c_int);
    while cmp != 0 as ::core::ffi::c_int {
        let mut dir: ::core::ffi::c_int = (cmp > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        pa[k as usize] = p;
        let fresh4 = k;
        k = k + 1;
        da[fresh4 as usize] = dir as ::core::ffi::c_uchar;
        p = (*p).avl_link[dir as usize];
        if p.is_null() {
            return NULL;
        }
        cmp = (*tree).avl_compare.expect("non-null function pointer")(
            item,
            (*p).avl_data,
            (*tree).avl_param,
        );
    }
    res = (*p).avl_data;
    if (*p).avl_link[1 as ::core::ffi::c_int as usize].is_null() {
        (*pa[(k - 1 as ::core::ffi::c_int) as usize]).avl_link
            [da[(k - 1 as ::core::ffi::c_int) as usize] as usize] =
            (*p).avl_link[0 as ::core::ffi::c_int as usize];
    } else {
        let mut r: *mut avl_node = (*p).avl_link[1 as ::core::ffi::c_int as usize];
        if (*r).avl_link[0 as ::core::ffi::c_int as usize].is_null() {
            (*r).avl_link[0 as ::core::ffi::c_int as usize] =
                (*p).avl_link[0 as ::core::ffi::c_int as usize];
            (*r).avl_balance = (*p).avl_balance;
            (*pa[(k - 1 as ::core::ffi::c_int) as usize]).avl_link
                [da[(k - 1 as ::core::ffi::c_int) as usize] as usize] = r;
            da[k as usize] = 1 as ::core::ffi::c_uchar;
            let fresh5 = k;
            k = k + 1;
            pa[fresh5 as usize] = r;
        } else {
            let mut s: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
            let fresh6 = k;
            k = k + 1;
            let mut j: ::core::ffi::c_int = fresh6;
            loop {
                da[k as usize] = 0 as ::core::ffi::c_uchar;
                let fresh7 = k;
                k = k + 1;
                pa[fresh7 as usize] = r;
                s = (*r).avl_link[0 as ::core::ffi::c_int as usize];
                if (*s).avl_link[0 as ::core::ffi::c_int as usize].is_null() {
                    break;
                }
                r = s;
            }
            (*s).avl_link[0 as ::core::ffi::c_int as usize] =
                (*p).avl_link[0 as ::core::ffi::c_int as usize];
            (*r).avl_link[0 as ::core::ffi::c_int as usize] =
                (*s).avl_link[1 as ::core::ffi::c_int as usize];
            (*s).avl_link[1 as ::core::ffi::c_int as usize] =
                (*p).avl_link[1 as ::core::ffi::c_int as usize];
            (*s).avl_balance = (*p).avl_balance;
            (*pa[(j - 1 as ::core::ffi::c_int) as usize]).avl_link
                [da[(j - 1 as ::core::ffi::c_int) as usize] as usize] = s;
            da[j as usize] = 1 as ::core::ffi::c_uchar;
            pa[j as usize] = s;
        }
    }
    (*(*tree).avl_alloc)
        .libavl_free
        .expect("non-null function pointer")((*tree).avl_alloc, p as *mut ::core::ffi::c_void);
    if !(k > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"avl_delete\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            275 as ::core::ffi::c_int,
            b"k > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    loop {
        k -= 1;
        if !(k > 0 as ::core::ffi::c_int) {
            break;
        }
        let mut y: *mut avl_node = pa[k as usize];
        if da[k as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (*y).avl_balance += 1;
            if (*y).avl_balance as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                break;
            }
            if !((*y).avl_balance as ::core::ffi::c_int == 2 as ::core::ffi::c_int) {
                continue;
            }
            let mut x: *mut avl_node = (*y).avl_link[1 as ::core::ffi::c_int as usize];
            if (*x).avl_balance as ::core::ffi::c_int == -(1 as ::core::ffi::c_int) {
                let mut w: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
                if !((*x).avl_balance as ::core::ffi::c_int == -(1 as ::core::ffi::c_int))
                    as ::core::ffi::c_int as ::core::ffi::c_long
                    != 0
                {
                    __assert_rtn(
                        b"avl_delete\0" as *const u8 as *const ::core::ffi::c_char,
                        b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
                        287 as ::core::ffi::c_int,
                        b"x->avl_balance == -1\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                };
                w = (*x).avl_link[0 as ::core::ffi::c_int as usize];
                (*x).avl_link[0 as ::core::ffi::c_int as usize] =
                    (*w).avl_link[1 as ::core::ffi::c_int as usize];
                (*w).avl_link[1 as ::core::ffi::c_int as usize] = x;
                (*y).avl_link[1 as ::core::ffi::c_int as usize] =
                    (*w).avl_link[0 as ::core::ffi::c_int as usize];
                (*w).avl_link[0 as ::core::ffi::c_int as usize] = y;
                if (*w).avl_balance as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    (*x).avl_balance = 0 as ::core::ffi::c_schar;
                    (*y).avl_balance = -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar;
                } else if (*w).avl_balance as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    (*y).avl_balance = 0 as ::core::ffi::c_schar;
                    (*x).avl_balance = (*y).avl_balance;
                } else {
                    (*x).avl_balance = 1 as ::core::ffi::c_int as ::core::ffi::c_schar;
                    (*y).avl_balance = 0 as ::core::ffi::c_schar;
                }
                (*w).avl_balance = 0 as ::core::ffi::c_schar;
                (*pa[(k - 1 as ::core::ffi::c_int) as usize]).avl_link
                    [da[(k - 1 as ::core::ffi::c_int) as usize] as usize] = w;
            } else {
                (*y).avl_link[1 as ::core::ffi::c_int as usize] =
                    (*x).avl_link[0 as ::core::ffi::c_int as usize];
                (*x).avl_link[0 as ::core::ffi::c_int as usize] = y;
                (*pa[(k - 1 as ::core::ffi::c_int) as usize]).avl_link
                    [da[(k - 1 as ::core::ffi::c_int) as usize] as usize] = x;
                if (*x).avl_balance as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    (*x).avl_balance = -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar;
                    (*y).avl_balance = 1 as ::core::ffi::c_int as ::core::ffi::c_schar;
                    break;
                } else {
                    (*y).avl_balance = 0 as ::core::ffi::c_schar;
                    (*x).avl_balance = (*y).avl_balance;
                }
            }
        } else {
            (*y).avl_balance -= 1;
            if (*y).avl_balance as ::core::ffi::c_int == -(1 as ::core::ffi::c_int) {
                break;
            }
            if !((*y).avl_balance as ::core::ffi::c_int == -(2 as ::core::ffi::c_int)) {
                continue;
            }
            let mut x_0: *mut avl_node = (*y).avl_link[0 as ::core::ffi::c_int as usize];
            if (*x_0).avl_balance as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                let mut w_0: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
                if !((*x_0).avl_balance as ::core::ffi::c_int == 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_int as ::core::ffi::c_long
                    != 0
                {
                    __assert_rtn(
                        b"avl_delete\0" as *const u8 as *const ::core::ffi::c_char,
                        b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
                        321 as ::core::ffi::c_int,
                        b"x->avl_balance == +1\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                };
                w_0 = (*x_0).avl_link[1 as ::core::ffi::c_int as usize];
                (*x_0).avl_link[1 as ::core::ffi::c_int as usize] =
                    (*w_0).avl_link[0 as ::core::ffi::c_int as usize];
                (*w_0).avl_link[0 as ::core::ffi::c_int as usize] = x_0;
                (*y).avl_link[0 as ::core::ffi::c_int as usize] =
                    (*w_0).avl_link[1 as ::core::ffi::c_int as usize];
                (*w_0).avl_link[1 as ::core::ffi::c_int as usize] = y;
                if (*w_0).avl_balance as ::core::ffi::c_int == -(1 as ::core::ffi::c_int) {
                    (*x_0).avl_balance = 0 as ::core::ffi::c_schar;
                    (*y).avl_balance = 1 as ::core::ffi::c_int as ::core::ffi::c_schar;
                } else if (*w_0).avl_balance as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    (*y).avl_balance = 0 as ::core::ffi::c_schar;
                    (*x_0).avl_balance = (*y).avl_balance;
                } else {
                    (*x_0).avl_balance = -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar;
                    (*y).avl_balance = 0 as ::core::ffi::c_schar;
                }
                (*w_0).avl_balance = 0 as ::core::ffi::c_schar;
                (*pa[(k - 1 as ::core::ffi::c_int) as usize]).avl_link
                    [da[(k - 1 as ::core::ffi::c_int) as usize] as usize] = w_0;
            } else {
                (*y).avl_link[0 as ::core::ffi::c_int as usize] =
                    (*x_0).avl_link[1 as ::core::ffi::c_int as usize];
                (*x_0).avl_link[1 as ::core::ffi::c_int as usize] = y;
                (*pa[(k - 1 as ::core::ffi::c_int) as usize]).avl_link
                    [da[(k - 1 as ::core::ffi::c_int) as usize] as usize] = x_0;
                if (*x_0).avl_balance as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    (*x_0).avl_balance = 1 as ::core::ffi::c_int as ::core::ffi::c_schar;
                    (*y).avl_balance = -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar;
                    break;
                } else {
                    (*y).avl_balance = 0 as ::core::ffi::c_schar;
                    (*x_0).avl_balance = (*y).avl_balance;
                }
            }
        }
    }
    (*tree).avl_count = (*tree).avl_count.wrapping_sub(1);
    (*tree).avl_generation = (*tree).avl_generation.wrapping_add(1);
    return res;
}
unsafe extern "C" fn trav_refresh(mut trav: *mut avl_traverser) {
    if trav.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"trav_refresh\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            359 as ::core::ffi::c_int,
            b"trav != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*trav).avl_generation = (*(*trav).avl_table).avl_generation;
    if !(*trav).avl_node.is_null() {
        let mut cmp: Option<avl_comparison_func> = (*(*trav).avl_table).avl_compare;
        let mut param: *mut ::core::ffi::c_void = (*(*trav).avl_table).avl_param;
        let mut node: *mut avl_node = (*trav).avl_node;
        let mut i: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
        (*trav).avl_height = 0 as size_t;
        i = (*(*trav).avl_table).avl_root as *mut avl_node;
        while i != node {
            if !((*trav).avl_height < 32 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                __assert_rtn(
                    b"trav_refresh\0" as *const u8 as *const ::core::ffi::c_char,
                    b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
                    371 as ::core::ffi::c_int,
                    b"trav->avl_height < AVL_MAX_HEIGHT\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            } else {
            };
            if i.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                __assert_rtn(
                    b"trav_refresh\0" as *const u8 as *const ::core::ffi::c_char,
                    b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
                    372 as ::core::ffi::c_int,
                    b"i != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
            let fresh13 = (*trav).avl_height;
            (*trav).avl_height = (*trav).avl_height.wrapping_add(1);
            (*trav).avl_stack[fresh13 as usize] = i;
            i = (*i).avl_link[(cmp.expect("non-null function pointer")(
                (*node).avl_data,
                (*i).avl_data,
                param,
            ) > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as usize];
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn avl_t_init(mut trav: *mut avl_traverser, mut tree: *mut avl_table) {
    (*trav).avl_table = tree;
    (*trav).avl_node = ::core::ptr::null_mut::<avl_node>();
    (*trav).avl_height = 0 as size_t;
    (*trav).avl_generation = (*tree).avl_generation;
}
#[no_mangle]
pub unsafe extern "C" fn avl_t_first(
    mut trav: *mut avl_traverser,
    mut tree: *mut avl_table,
) -> *mut ::core::ffi::c_void {
    let mut x: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
    if !(!tree.is_null() && !trav.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"avl_t_first\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            397 as ::core::ffi::c_int,
            b"tree != NULL && trav != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*trav).avl_table = tree;
    (*trav).avl_height = 0 as size_t;
    (*trav).avl_generation = (*tree).avl_generation;
    x = (*tree).avl_root as *mut avl_node;
    if !x.is_null() {
        while !(*x).avl_link[0 as ::core::ffi::c_int as usize].is_null() {
            if !((*trav).avl_height < 32 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                __assert_rtn(
                    b"avl_t_first\0" as *const u8 as *const ::core::ffi::c_char,
                    b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
                    406 as ::core::ffi::c_int,
                    b"trav->avl_height < AVL_MAX_HEIGHT\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            } else {
            };
            let fresh8 = (*trav).avl_height;
            (*trav).avl_height = (*trav).avl_height.wrapping_add(1);
            (*trav).avl_stack[fresh8 as usize] = x;
            x = (*x).avl_link[0 as ::core::ffi::c_int as usize];
        }
    }
    (*trav).avl_node = x;
    return if !x.is_null() { (*x).avl_data } else { NULL };
}
#[no_mangle]
pub unsafe extern "C" fn avl_t_last(
    mut trav: *mut avl_traverser,
    mut tree: *mut avl_table,
) -> *mut ::core::ffi::c_void {
    let mut x: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
    if !(!tree.is_null() && !trav.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"avl_t_last\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            422 as ::core::ffi::c_int,
            b"tree != NULL && trav != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*trav).avl_table = tree;
    (*trav).avl_height = 0 as size_t;
    (*trav).avl_generation = (*tree).avl_generation;
    x = (*tree).avl_root as *mut avl_node;
    if !x.is_null() {
        while !(*x).avl_link[1 as ::core::ffi::c_int as usize].is_null() {
            if !((*trav).avl_height < 32 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                __assert_rtn(
                    b"avl_t_last\0" as *const u8 as *const ::core::ffi::c_char,
                    b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
                    431 as ::core::ffi::c_int,
                    b"trav->avl_height < AVL_MAX_HEIGHT\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            } else {
            };
            let fresh9 = (*trav).avl_height;
            (*trav).avl_height = (*trav).avl_height.wrapping_add(1);
            (*trav).avl_stack[fresh9 as usize] = x;
            x = (*x).avl_link[1 as ::core::ffi::c_int as usize];
        }
    }
    (*trav).avl_node = x;
    return if !x.is_null() { (*x).avl_data } else { NULL };
}
#[no_mangle]
pub unsafe extern "C" fn avl_t_find(
    mut trav: *mut avl_traverser,
    mut tree: *mut avl_table,
    mut item: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut p: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
    let mut q: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
    if !(!trav.is_null() && !tree.is_null() && !item.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"avl_t_find\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            449 as ::core::ffi::c_int,
            b"trav != NULL && tree != NULL && item != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*trav).avl_table = tree;
    (*trav).avl_height = 0 as size_t;
    (*trav).avl_generation = (*tree).avl_generation;
    p = (*tree).avl_root as *mut avl_node;
    while !p.is_null() {
        let mut cmp: ::core::ffi::c_int = (*tree).avl_compare.expect("non-null function pointer")(
            item,
            (*p).avl_data,
            (*tree).avl_param,
        );
        if cmp < 0 as ::core::ffi::c_int {
            q = (*p).avl_link[0 as ::core::ffi::c_int as usize];
        } else if cmp > 0 as ::core::ffi::c_int {
            q = (*p).avl_link[1 as ::core::ffi::c_int as usize];
        } else {
            (*trav).avl_node = p;
            return (*p).avl_data;
        }
        if !((*trav).avl_height < 32 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"avl_t_find\0" as *const u8 as *const ::core::ffi::c_char,
                b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
                466 as ::core::ffi::c_int,
                b"trav->avl_height < AVL_MAX_HEIGHT\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        let fresh10 = (*trav).avl_height;
        (*trav).avl_height = (*trav).avl_height.wrapping_add(1);
        (*trav).avl_stack[fresh10 as usize] = p;
        p = q;
    }
    (*trav).avl_height = 0 as size_t;
    (*trav).avl_node = ::core::ptr::null_mut::<avl_node>();
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn avl_t_insert(
    mut trav: *mut avl_traverser,
    mut tree: *mut avl_table,
    mut item: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut p: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    if !(!trav.is_null() && !tree.is_null() && !item.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"avl_t_insert\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            487 as ::core::ffi::c_int,
            b"trav != NULL && tree != NULL && item != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
    };
    p = avl_probe(tree, item);
    if !p.is_null() {
        (*trav).avl_table = tree;
        (*trav).avl_node = (p as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize))
            as *mut avl_node;
        (*trav).avl_generation = (*tree)
            .avl_generation
            .wrapping_sub(1 as ::core::ffi::c_ulong);
        return *p;
    } else {
        avl_t_init(trav, tree);
        return NULL;
    };
}
#[no_mangle]
pub unsafe extern "C" fn avl_t_copy(
    mut trav: *mut avl_traverser,
    mut src: *const avl_traverser,
) -> *mut ::core::ffi::c_void {
    if !(!trav.is_null() && !src.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"avl_t_copy\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            505 as ::core::ffi::c_int,
            b"trav != NULL && src != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if trav != src as *mut avl_traverser {
        (*trav).avl_table = (*src).avl_table;
        (*trav).avl_node = (*src).avl_node;
        (*trav).avl_generation = (*src).avl_generation;
        if (*trav).avl_generation == (*(*trav).avl_table).avl_generation {
            (*trav).avl_height = (*src).avl_height;
            memcpy(
                &raw mut (*trav).avl_stack as *mut *mut avl_node as *mut ::core::ffi::c_void,
                &raw const (*src).avl_stack as *const *mut avl_node as *const ::core::ffi::c_void,
                (::core::mem::size_of::<*mut avl_node>() as size_t)
                    .wrapping_mul((*trav).avl_height),
            );
        }
    }
    return if !(*trav).avl_node.is_null() {
        (*(*trav).avl_node).avl_data
    } else {
        NULL
    };
}
#[no_mangle]
pub unsafe extern "C" fn avl_t_next(mut trav: *mut avl_traverser) -> *mut ::core::ffi::c_void {
    let mut x: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
    if trav.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"avl_t_next\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            528 as ::core::ffi::c_int,
            b"trav != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if (*trav).avl_generation != (*(*trav).avl_table).avl_generation {
        trav_refresh(trav);
    }
    x = (*trav).avl_node;
    if x.is_null() {
        return avl_t_first(trav, (*trav).avl_table);
    } else if !(*x).avl_link[1 as ::core::ffi::c_int as usize].is_null() {
        if !((*trav).avl_height < 32 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"avl_t_next\0" as *const u8 as *const ::core::ffi::c_char,
                b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
                537 as ::core::ffi::c_int,
                b"trav->avl_height < AVL_MAX_HEIGHT\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        let fresh11 = (*trav).avl_height;
        (*trav).avl_height = (*trav).avl_height.wrapping_add(1);
        (*trav).avl_stack[fresh11 as usize] = x;
        x = (*x).avl_link[1 as ::core::ffi::c_int as usize];
        while !(*x).avl_link[0 as ::core::ffi::c_int as usize].is_null() {
            if !((*trav).avl_height < 32 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                __assert_rtn(
                    b"avl_t_next\0" as *const u8 as *const ::core::ffi::c_char,
                    b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
                    542 as ::core::ffi::c_int,
                    b"trav->avl_height < AVL_MAX_HEIGHT\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            } else {
            };
            let fresh12 = (*trav).avl_height;
            (*trav).avl_height = (*trav).avl_height.wrapping_add(1);
            (*trav).avl_stack[fresh12 as usize] = x;
            x = (*x).avl_link[0 as ::core::ffi::c_int as usize];
        }
    } else {
        let mut y: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
        loop {
            if (*trav).avl_height == 0 as size_t {
                (*trav).avl_node = ::core::ptr::null_mut::<avl_node>();
                return NULL;
            }
            y = x;
            (*trav).avl_height = (*trav).avl_height.wrapping_sub(1);
            x = (*trav).avl_stack[(*trav).avl_height as usize];
            if !(y == (*x).avl_link[1 as ::core::ffi::c_int as usize]) {
                break;
            }
        }
    }
    (*trav).avl_node = x;
    return (*x).avl_data;
}
#[no_mangle]
pub unsafe extern "C" fn avl_t_prev(mut trav: *mut avl_traverser) -> *mut ::core::ffi::c_void {
    let mut x: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
    if trav.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"avl_t_prev\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            572 as ::core::ffi::c_int,
            b"trav != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if (*trav).avl_generation != (*(*trav).avl_table).avl_generation {
        trav_refresh(trav);
    }
    x = (*trav).avl_node;
    if x.is_null() {
        return avl_t_last(trav, (*trav).avl_table);
    } else if !(*x).avl_link[0 as ::core::ffi::c_int as usize].is_null() {
        if !((*trav).avl_height < 32 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"avl_t_prev\0" as *const u8 as *const ::core::ffi::c_char,
                b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
                581 as ::core::ffi::c_int,
                b"trav->avl_height < AVL_MAX_HEIGHT\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        let fresh14 = (*trav).avl_height;
        (*trav).avl_height = (*trav).avl_height.wrapping_add(1);
        (*trav).avl_stack[fresh14 as usize] = x;
        x = (*x).avl_link[0 as ::core::ffi::c_int as usize];
        while !(*x).avl_link[1 as ::core::ffi::c_int as usize].is_null() {
            if !((*trav).avl_height < 32 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                __assert_rtn(
                    b"avl_t_prev\0" as *const u8 as *const ::core::ffi::c_char,
                    b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
                    586 as ::core::ffi::c_int,
                    b"trav->avl_height < AVL_MAX_HEIGHT\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            } else {
            };
            let fresh15 = (*trav).avl_height;
            (*trav).avl_height = (*trav).avl_height.wrapping_add(1);
            (*trav).avl_stack[fresh15 as usize] = x;
            x = (*x).avl_link[1 as ::core::ffi::c_int as usize];
        }
    } else {
        let mut y: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
        loop {
            if (*trav).avl_height == 0 as size_t {
                (*trav).avl_node = ::core::ptr::null_mut::<avl_node>();
                return NULL;
            }
            y = x;
            (*trav).avl_height = (*trav).avl_height.wrapping_sub(1);
            x = (*trav).avl_stack[(*trav).avl_height as usize];
            if !(y == (*x).avl_link[0 as ::core::ffi::c_int as usize]) {
                break;
            }
        }
    }
    (*trav).avl_node = x;
    return (*x).avl_data;
}
#[no_mangle]
pub unsafe extern "C" fn avl_t_cur(mut trav: *mut avl_traverser) -> *mut ::core::ffi::c_void {
    if trav.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"avl_t_cur\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            612 as ::core::ffi::c_int,
            b"trav != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    return if !(*trav).avl_node.is_null() {
        (*(*trav).avl_node).avl_data
    } else {
        NULL
    };
}
#[no_mangle]
pub unsafe extern "C" fn avl_t_replace(
    mut trav: *mut avl_traverser,
    mut new: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut old: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if !(!trav.is_null() && !(*trav).avl_node.is_null() && !new.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"avl_t_replace\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            624 as ::core::ffi::c_int,
            b"trav != NULL && trav->avl_node != NULL && new != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
    };
    old = (*(*trav).avl_node).avl_data;
    (*(*trav).avl_node).avl_data = new;
    return old;
}
unsafe extern "C" fn copy_error_recovery(
    mut stack: *mut *mut avl_node,
    mut height: ::core::ffi::c_int,
    mut new: *mut avl_table,
    mut destroy: Option<avl_item_func>,
) {
    if !(!stack.is_null() && height >= 0 as ::core::ffi::c_int && !new.is_null())
        as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"copy_error_recovery\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            637 as ::core::ffi::c_int,
            b"stack != NULL && height >= 0 && new != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
    };
    while height > 2 as ::core::ffi::c_int {
        let ref mut fresh2 = (**stack.offset((height - 1 as ::core::ffi::c_int) as isize)).avl_link
            [1 as ::core::ffi::c_int as usize];
        *fresh2 = ::core::ptr::null_mut::<avl_node>();
        height -= 2 as ::core::ffi::c_int;
    }
    avl_destroy(new, destroy);
}
#[no_mangle]
pub unsafe extern "C" fn avl_copy(
    mut org: *const avl_table,
    mut copy: Option<avl_copy_func>,
    mut destroy: Option<avl_item_func>,
    mut allocator: *mut libavl_allocator,
) -> *mut avl_table {
    let mut stack: [*mut avl_node; 66] = [::core::ptr::null_mut::<avl_node>(); 66];
    let mut height: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut new: *mut avl_table = ::core::ptr::null_mut::<avl_table>();
    let mut org_head: avl_node = avl_node {
        avl_link: [::core::ptr::null_mut::<avl_node>(); 2],
        avl_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        avl_balance: 0,
    };
    let mut x: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
    let mut y: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
    if org.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"avl_copy\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            663 as ::core::ffi::c_int,
            b"org != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    new = avl_create(
        (*org).avl_compare,
        (*org).avl_param,
        if !allocator.is_null() {
            allocator
        } else {
            (*org).avl_alloc
        },
    );
    if new.is_null() {
        return ::core::ptr::null_mut::<avl_table>();
    }
    (*new).avl_count = (*org).avl_count;
    if (*new).avl_count == 0 as size_t {
        return new;
    }
    org_head.avl_link[0 as ::core::ffi::c_int as usize] = (*org).avl_root as *mut avl_node;
    x = &raw mut org_head;
    y = &raw mut (*new).avl_root as *mut avl_node;
    loop {
        while !(*x).avl_link[0 as ::core::ffi::c_int as usize].is_null() {
            if !(height
                < 2 as ::core::ffi::c_int * (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                __assert_rtn(
                    b"avl_copy\0" as *const u8 as *const ::core::ffi::c_char,
                    b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
                    677 as ::core::ffi::c_int,
                    b"height < 2 * (AVL_MAX_HEIGHT + 1)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            } else {
            };
            (*y).avl_link[0 as ::core::ffi::c_int as usize] = (*(*new).avl_alloc)
                .libavl_malloc
                .expect("non-null function pointer")(
                (*new).avl_alloc,
                ::core::mem::size_of::<avl_node>() as size_t,
            ) as *mut avl_node;
            if (*y).avl_link[0 as ::core::ffi::c_int as usize].is_null() {
                if y != &raw mut (*new).avl_root as *mut avl_node {
                    (*y).avl_data = NULL;
                    (*y).avl_link[1 as ::core::ffi::c_int as usize] =
                        ::core::ptr::null_mut::<avl_node>();
                }
                copy_error_recovery(&raw mut stack as *mut *mut avl_node, height, new, destroy);
                return ::core::ptr::null_mut::<avl_table>();
            }
            let fresh0 = height;
            height = height + 1;
            stack[fresh0 as usize] = x;
            let fresh1 = height;
            height = height + 1;
            stack[fresh1 as usize] = y;
            x = (*x).avl_link[0 as ::core::ffi::c_int as usize];
            y = (*y).avl_link[0 as ::core::ffi::c_int as usize];
        }
        (*y).avl_link[0 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<avl_node>();
        loop {
            (*y).avl_balance = (*x).avl_balance;
            if copy.is_none() {
                (*y).avl_data = (*x).avl_data;
            } else {
                (*y).avl_data =
                    copy.expect("non-null function pointer")((*x).avl_data, (*org).avl_param);
                if (*y).avl_data.is_null() {
                    (*y).avl_link[1 as ::core::ffi::c_int as usize] =
                        ::core::ptr::null_mut::<avl_node>();
                    copy_error_recovery(&raw mut stack as *mut *mut avl_node, height, new, destroy);
                    return ::core::ptr::null_mut::<avl_table>();
                }
            }
            if !(*x).avl_link[1 as ::core::ffi::c_int as usize].is_null() {
                (*y).avl_link[1 as ::core::ffi::c_int as usize] = (*(*new).avl_alloc)
                    .libavl_malloc
                    .expect("non-null function pointer")(
                    (*new).avl_alloc,
                    ::core::mem::size_of::<avl_node>() as size_t,
                )
                    as *mut avl_node;
                if (*y).avl_link[1 as ::core::ffi::c_int as usize].is_null() {
                    copy_error_recovery(&raw mut stack as *mut *mut avl_node, height, new, destroy);
                    return ::core::ptr::null_mut::<avl_table>();
                }
                x = (*x).avl_link[1 as ::core::ffi::c_int as usize];
                y = (*y).avl_link[1 as ::core::ffi::c_int as usize];
                break;
            } else {
                (*y).avl_link[1 as ::core::ffi::c_int as usize] =
                    ::core::ptr::null_mut::<avl_node>();
                if height <= 2 as ::core::ffi::c_int {
                    return new;
                }
                height -= 1;
                y = stack[height as usize];
                height -= 1;
                x = stack[height as usize];
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn avl_destroy(mut tree: *mut avl_table, mut destroy: Option<avl_item_func>) {
    let mut p: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
    let mut q: *mut avl_node = ::core::ptr::null_mut::<avl_node>();
    if tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"avl_destroy\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            742 as ::core::ffi::c_int,
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    p = (*tree).avl_root as *mut avl_node;
    while !p.is_null() {
        if (*p).avl_link[0 as ::core::ffi::c_int as usize].is_null() {
            q = (*p).avl_link[1 as ::core::ffi::c_int as usize];
            if destroy.is_some() && !(*p).avl_data.is_null() {
                destroy.expect("non-null function pointer")((*p).avl_data, (*tree).avl_param);
            }
            (*(*tree).avl_alloc)
                .libavl_free
                .expect("non-null function pointer")(
                (*tree).avl_alloc,
                p as *mut ::core::ffi::c_void,
            );
        } else {
            q = (*p).avl_link[0 as ::core::ffi::c_int as usize];
            (*p).avl_link[0 as ::core::ffi::c_int as usize] =
                (*q).avl_link[1 as ::core::ffi::c_int as usize];
            (*q).avl_link[1 as ::core::ffi::c_int as usize] = p;
        }
        p = q;
    }
    (*(*tree).avl_alloc)
        .libavl_free
        .expect("non-null function pointer")(
        (*tree).avl_alloc, tree as *mut ::core::ffi::c_void
    );
}
#[no_mangle]
pub unsafe extern "C" fn avl_malloc(
    mut allocator: *mut libavl_allocator,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    if !(!allocator.is_null() && size > 0 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"avl_malloc\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            763 as ::core::ffi::c_int,
            b"allocator != NULL && size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    return malloc(size);
}
#[no_mangle]
pub unsafe extern "C" fn avl_free(
    mut allocator: *mut libavl_allocator,
    mut block: *mut ::core::ffi::c_void,
) {
    if !(!allocator.is_null() && !block.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0
    {
        __assert_rtn(
            b"avl_free\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            770 as ::core::ffi::c_int,
            b"allocator != NULL && block != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    free(block);
}
#[no_mangle]
pub static mut avl_allocator_default: libavl_allocator = unsafe {
    libavl_allocator {
        libavl_malloc: Some(
            avl_malloc
                as unsafe extern "C" fn(*mut libavl_allocator, size_t) -> *mut ::core::ffi::c_void,
        ),
        libavl_free: Some(
            avl_free as unsafe extern "C" fn(*mut libavl_allocator, *mut ::core::ffi::c_void) -> (),
        ),
    }
};
#[no_mangle]
pub unsafe extern "C" fn avl_assert_insert(
    mut table: *mut avl_table,
    mut item: *mut ::core::ffi::c_void,
) {
    let mut p: *mut *mut ::core::ffi::c_void = avl_probe(table, item);
    if !(!p.is_null() && *p == item) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"avl_assert_insert\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            787 as ::core::ffi::c_int,
            b"p != NULL && *p == item\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
}
#[no_mangle]
pub unsafe extern "C" fn avl_assert_delete(
    mut table: *mut avl_table,
    mut item: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut p: *mut ::core::ffi::c_void = avl_delete(table, item);
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"avl_assert_delete\0" as *const u8 as *const ::core::ffi::c_char,
            b"avl.c\0" as *const u8 as *const ::core::ffi::c_char,
            794 as ::core::ffi::c_int,
            b"p != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    return p;
}
