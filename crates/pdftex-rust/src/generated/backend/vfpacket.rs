extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn xmalloc(size: size_t) -> address;
    fn xrealloc(old_address: address, new_size: size_t) -> address;
    static mut strpool: *mut packedASCIIcode;
    static mut strstart: *mut poolpointer;
    static mut fontbc: *mut eightbits;
    static mut fontec: *mut eightbits;
    static mut vfpacketbase: *mut integer;
    static mut vfpacketlength: integer;
    static mut last_ptr_index: size_t;
    fn pdftex_fail(_: *const ::core::ffi::c_char, ...) -> !;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type address = *mut ::core::ffi::c_void;
pub type integer = ::core::ffi::c_int;
pub type eightbits = ::core::ffi::c_uchar;
pub type poolpointer = integer;
pub type strnumber = integer;
pub type packedASCIIcode = ::core::ffi::c_uchar;
pub type internalfontnumber = integer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vf_entry {
    pub data: *mut *mut ::core::ffi::c_char,
    pub len: *mut ::core::ffi::c_int,
    pub char_count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct packet_entry {
    pub dataptr: *mut ::core::ffi::c_char,
    pub len: integer,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
#[no_mangle]
pub static mut packet_limit: size_t = 0;
#[no_mangle]
pub static mut packet_array: *mut packet_entry =
    ::core::ptr::null::<packet_entry>() as *mut packet_entry;
#[no_mangle]
pub static mut packet_ptr: *mut packet_entry =
    ::core::ptr::null::<packet_entry>() as *mut packet_entry;
#[no_mangle]
pub static mut vf_array: *mut vf_entry = ::core::ptr::null::<vf_entry>() as *mut vf_entry;
#[no_mangle]
pub static mut vf_limit: size_t = 0;
#[no_mangle]
pub static mut vf_ptr: *mut vf_entry = ::core::ptr::null::<vf_entry>() as *mut vf_entry;
static mut packet_data_ptr: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub unsafe extern "C" fn newvfpacket(mut f: internalfontnumber) -> integer {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = *fontec.offset(f as isize) as ::core::ffi::c_int
        - *fontbc.offset(f as isize) as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int;
    if vf_array.is_null() {
        vf_limit = 256 as size_t;
        if 1 as ::core::ffi::c_int as ::core::ffi::c_uint > vf_limit as ::core::ffi::c_uint {
            vf_limit = 1 as size_t;
        }
        vf_array = xmalloc(vf_limit.wrapping_mul(::core::mem::size_of::<vf_entry>() as size_t))
            as *mut vf_entry;
        vf_ptr = vf_array;
    } else if (vf_ptr.offset_from(vf_array) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
        as ::core::ffi::c_uint
        > vf_limit as ::core::ffi::c_uint
    {
        last_ptr_index = vf_ptr.offset_from(vf_array) as ::core::ffi::c_long as size_t;
        vf_limit = vf_limit.wrapping_mul(2 as size_t);
        if (vf_ptr.offset_from(vf_array) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
            as ::core::ffi::c_uint
            > vf_limit as ::core::ffi::c_uint
        {
            vf_limit = (vf_ptr.offset_from(vf_array) as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as size_t;
        }
        if vf_limit as ::core::ffi::c_uint > INT_MAX as ::core::ffi::c_uint {
            pdftex_fail(
                b"vf_array exceeds size limit\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        vf_array = xrealloc(
            vf_array as address,
            vf_limit.wrapping_mul(::core::mem::size_of::<vf_entry>() as size_t),
        ) as *mut vf_entry;
        vf_ptr = vf_array.offset(last_ptr_index as isize);
    }
    (*vf_ptr).len =
        xmalloc((n as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t))
            as *mut ::core::ffi::c_int;
    (*vf_ptr).data = xmalloc(
        (n as size_t).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
    ) as *mut *mut ::core::ffi::c_char;
    (*vf_ptr).char_count = n;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        let ref mut fresh0 = *(*vf_ptr).data.offset(i as isize);
        *fresh0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        *(*vf_ptr).len.offset(i as isize) = 0 as ::core::ffi::c_int;
        i += 1;
    }
    let fresh1 = vf_ptr;
    vf_ptr = vf_ptr.offset(1);
    return fresh1.offset_from(vf_array) as ::core::ffi::c_long as integer;
}
#[no_mangle]
pub unsafe extern "C" fn storepacket(mut f: internalfontnumber, mut c: integer, mut s: strnumber) {
    let mut l: ::core::ffi::c_int = *strstart
        .offset((s as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int
        - *strstart.offset(s as isize) as ::core::ffi::c_int;
    *(*vf_array.offset(*vfpacketbase.offset(f as isize) as isize))
        .len
        .offset(
            (c as ::core::ffi::c_int - *fontbc.offset(f as isize) as ::core::ffi::c_int) as isize,
        ) = l;
    let ref mut fresh2 = *(*vf_array.offset(*vfpacketbase.offset(f as isize) as isize))
        .data
        .offset(
            (c as ::core::ffi::c_int - *fontbc.offset(f as isize) as ::core::ffi::c_int) as isize,
        );
    *fresh2 = xmalloc(
        (l as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    ) as *mut ::core::ffi::c_char;
    memcpy(
        *(*vf_array.offset(*vfpacketbase.offset(f as isize) as isize))
            .data
            .offset(
                (c as ::core::ffi::c_int - *fontbc.offset(f as isize) as ::core::ffi::c_int)
                    as isize,
            ) as *mut ::core::ffi::c_void,
        strpool.offset(*strstart.offset(s as isize) as isize) as *mut ::core::ffi::c_void,
        l as ::core::ffi::c_uint as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn startpacket(mut f: internalfontnumber, mut c: eightbits) {
    packet_data_ptr = *(*vf_array.offset(*vfpacketbase.offset(f as isize) as isize))
        .data
        .offset(
            (c as ::core::ffi::c_int - *fontbc.offset(f as isize) as ::core::ffi::c_int) as isize,
        );
    vfpacketlength = *(*vf_array.offset(*vfpacketbase.offset(f as isize) as isize))
        .len
        .offset(
            (c as ::core::ffi::c_int - *fontbc.offset(f as isize) as ::core::ffi::c_int) as isize,
        ) as integer;
}
#[no_mangle]
pub unsafe extern "C" fn packetbyte() -> eightbits {
    vfpacketlength -= 1;
    let fresh3 = packet_data_ptr;
    packet_data_ptr = packet_data_ptr.offset(1);
    return *fresh3 as eightbits;
}
#[no_mangle]
pub unsafe extern "C" fn pushpacketstate() {
    if packet_array.is_null() {
        packet_limit = 256 as size_t;
        if 1 as ::core::ffi::c_int as ::core::ffi::c_uint > packet_limit as ::core::ffi::c_uint {
            packet_limit = 1 as size_t;
        }
        packet_array =
            xmalloc(packet_limit.wrapping_mul(::core::mem::size_of::<packet_entry>() as size_t))
                as *mut packet_entry;
        packet_ptr = packet_array;
    } else if (packet_ptr.offset_from(packet_array) as ::core::ffi::c_long
        + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
        > packet_limit as ::core::ffi::c_uint
    {
        last_ptr_index = packet_ptr.offset_from(packet_array) as ::core::ffi::c_long as size_t;
        packet_limit = packet_limit.wrapping_mul(2 as size_t);
        if (packet_ptr.offset_from(packet_array) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
            as ::core::ffi::c_uint
            > packet_limit as ::core::ffi::c_uint
        {
            packet_limit = (packet_ptr.offset_from(packet_array) as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as size_t;
        }
        if packet_limit as ::core::ffi::c_uint > INT_MAX as ::core::ffi::c_uint {
            pdftex_fail(
                b"packet_array exceeds size limit\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        packet_array = xrealloc(
            packet_array as address,
            packet_limit.wrapping_mul(::core::mem::size_of::<packet_entry>() as size_t),
        ) as *mut packet_entry;
        packet_ptr = packet_array.offset(last_ptr_index as isize);
    }
    (*packet_ptr).dataptr = packet_data_ptr;
    (*packet_ptr).len = vfpacketlength;
    packet_ptr = packet_ptr.offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn poppacketstate() {
    if packet_ptr == packet_array {
        pdftex_fail(
            b"packet stack empty, impossible to pop\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    packet_ptr = packet_ptr.offset(-1);
    packet_data_ptr = (*packet_ptr).dataptr;
    vfpacketlength = (*packet_ptr).len;
}
#[no_mangle]
pub unsafe extern "C" fn vf_free() {
    let mut v: *mut vf_entry = ::core::ptr::null_mut::<vf_entry>();
    let mut p: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    if !vf_array.is_null() {
        v = vf_array;
        while v < vf_ptr {
            if !(*v).len.is_null() {
                free((*v).len as *mut ::core::ffi::c_void);
            }
            (*v).len = ::core::ptr::null_mut::<::core::ffi::c_int>();
            p = (*v).data;
            while (p.offset_from((*v).data) as ::core::ffi::c_long)
                < (*v).char_count as ::core::ffi::c_long
            {
                if !(*p).is_null() {
                    free(*p as *mut ::core::ffi::c_void);
                }
                *p = ::core::ptr::null_mut::<::core::ffi::c_char>();
                p = p.offset(1);
            }
            if !(*v).data.is_null() {
                free((*v).data as *mut ::core::ffi::c_void);
            }
            (*v).data = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
            v = v.offset(1);
        }
        if !vf_array.is_null() {
            free(vf_array as *mut ::core::ffi::c_void);
        }
        vf_array = ::core::ptr::null_mut::<vf_entry>();
    }
    if !packet_array.is_null() {
        free(packet_array as *mut ::core::ffi::c_void);
    }
    packet_array = ::core::ptr::null_mut::<packet_entry>();
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
