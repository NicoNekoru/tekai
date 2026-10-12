use crate::byte_string::memcpy;

use crate::allocation::free;
use crate::openclose::{xmalloc, xrealloc};

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

pub unsafe fn newvfpacket(mut f: internalfontnumber) -> integer {
    let __tekai_state_generated_backend_vfpacket: *mut crate::generated::backend::vfpacket::State =
        unsafe { crate::generated::backend::vfpacket::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = *(*__tekai_state_generated_pdftexextra)
        .fontec
        .offset(f as isize) as ::core::ffi::c_int
        - *(*__tekai_state_generated_pdftexextra)
            .fontbc
            .offset(f as isize) as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int;
    if (*__tekai_state_generated_backend_vfpacket)
        .vf_array
        .is_null()
    {
        (*__tekai_state_generated_backend_vfpacket).vf_limit = 256 as size_t;
        if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
            > (*__tekai_state_generated_backend_vfpacket).vf_limit as ::core::ffi::c_uint
        {
            (*__tekai_state_generated_backend_vfpacket).vf_limit = 1 as size_t;
        }
        (*__tekai_state_generated_backend_vfpacket).vf_array = xmalloc(
            (*__tekai_state_generated_backend_vfpacket)
                .vf_limit
                .wrapping_mul(::core::mem::size_of::<vf_entry>() as size_t),
        ) as *mut vf_entry;
        (*__tekai_state_generated_backend_vfpacket).vf_ptr =
            (*__tekai_state_generated_backend_vfpacket).vf_array;
    } else if ((*__tekai_state_generated_backend_vfpacket)
        .vf_ptr
        .offset_from((*__tekai_state_generated_backend_vfpacket).vf_array)
        as ::core::ffi::c_long
        + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
        > (*__tekai_state_generated_backend_vfpacket).vf_limit as ::core::ffi::c_uint
    {
        (*__tekai_state_utils).last_ptr_index = (*__tekai_state_generated_backend_vfpacket)
            .vf_ptr
            .offset_from((*__tekai_state_generated_backend_vfpacket).vf_array)
            as ::core::ffi::c_long as size_t;
        (*__tekai_state_generated_backend_vfpacket).vf_limit =
            (*__tekai_state_generated_backend_vfpacket)
                .vf_limit
                .wrapping_mul(2 as size_t);
        if ((*__tekai_state_generated_backend_vfpacket)
            .vf_ptr
            .offset_from((*__tekai_state_generated_backend_vfpacket).vf_array)
            as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
            > (*__tekai_state_generated_backend_vfpacket).vf_limit as ::core::ffi::c_uint
        {
            (*__tekai_state_generated_backend_vfpacket).vf_limit =
                ((*__tekai_state_generated_backend_vfpacket)
                    .vf_ptr
                    .offset_from((*__tekai_state_generated_backend_vfpacket).vf_array)
                    as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as size_t;
        }
        if (*__tekai_state_generated_backend_vfpacket).vf_limit as ::core::ffi::c_uint
            > INT_MAX as ::core::ffi::c_uint
        {
            crate::utils::pdftex_fail_args(
                b"vf_array exceeds size limit\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        (*__tekai_state_generated_backend_vfpacket).vf_array = xrealloc(
            (*__tekai_state_generated_backend_vfpacket).vf_array as address,
            (*__tekai_state_generated_backend_vfpacket)
                .vf_limit
                .wrapping_mul(::core::mem::size_of::<vf_entry>() as size_t),
        ) as *mut vf_entry;
        (*__tekai_state_generated_backend_vfpacket).vf_ptr =
            (*__tekai_state_generated_backend_vfpacket)
                .vf_array
                .offset((*__tekai_state_utils).last_ptr_index as isize);
    }
    (*(*__tekai_state_generated_backend_vfpacket).vf_ptr).len =
        xmalloc((n as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t))
            as *mut ::core::ffi::c_int;
    (*(*__tekai_state_generated_backend_vfpacket).vf_ptr).data = xmalloc(
        (n as size_t).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
    )
        as *mut *mut ::core::ffi::c_char;
    (*(*__tekai_state_generated_backend_vfpacket).vf_ptr).char_count = n;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        let ref mut fresh0 = *(*(*__tekai_state_generated_backend_vfpacket).vf_ptr)
            .data
            .offset(i as isize);
        *fresh0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        *(*(*__tekai_state_generated_backend_vfpacket).vf_ptr)
            .len
            .offset(i as isize) = 0 as ::core::ffi::c_int;
        i += 1;
    }
    let fresh1 = (*__tekai_state_generated_backend_vfpacket).vf_ptr;
    (*__tekai_state_generated_backend_vfpacket).vf_ptr =
        (*__tekai_state_generated_backend_vfpacket).vf_ptr.offset(1);
    return fresh1.offset_from((*__tekai_state_generated_backend_vfpacket).vf_array)
        as ::core::ffi::c_long as integer;
}
pub unsafe fn storepacket(mut f: internalfontnumber, mut c: integer, mut s: strnumber) {
    let __tekai_state_generated_backend_vfpacket: *mut crate::generated::backend::vfpacket::State =
        unsafe { crate::generated::backend::vfpacket::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut l: ::core::ffi::c_int = *(*__tekai_state_generated_pdftexextra)
        .strstart
        .offset((s as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int
        - *(*__tekai_state_generated_pdftexextra)
            .strstart
            .offset(s as isize) as ::core::ffi::c_int;
    *(*(*__tekai_state_generated_backend_vfpacket).vf_array.offset(
        *(*__tekai_state_generated_pdftexextra)
            .vfpacketbase
            .offset(f as isize) as isize,
    ))
    .len
    .offset(
        (c as ::core::ffi::c_int
            - *(*__tekai_state_generated_pdftexextra)
                .fontbc
                .offset(f as isize) as ::core::ffi::c_int) as isize,
    ) = l;
    let ref mut fresh2 = *(*(*__tekai_state_generated_backend_vfpacket).vf_array.offset(
        *(*__tekai_state_generated_pdftexextra)
            .vfpacketbase
            .offset(f as isize) as isize,
    ))
    .data
    .offset(
        (c as ::core::ffi::c_int
            - *(*__tekai_state_generated_pdftexextra)
                .fontbc
                .offset(f as isize) as ::core::ffi::c_int) as isize,
    );
    *fresh2 = xmalloc(
        (l as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    ) as *mut ::core::ffi::c_char;
    memcpy(
        *(*(*__tekai_state_generated_backend_vfpacket).vf_array.offset(
            *(*__tekai_state_generated_pdftexextra)
                .vfpacketbase
                .offset(f as isize) as isize,
        ))
        .data
        .offset(
            (c as ::core::ffi::c_int
                - *(*__tekai_state_generated_pdftexextra)
                    .fontbc
                    .offset(f as isize) as ::core::ffi::c_int) as isize,
        ) as *mut ::core::ffi::c_void,
        (*__tekai_state_generated_pdftexextra).strpool.offset(
            *(*__tekai_state_generated_pdftexextra)
                .strstart
                .offset(s as isize) as isize,
        ) as *mut ::core::ffi::c_void,
        l as ::core::ffi::c_uint as size_t,
    );
}
pub unsafe fn startpacket(mut f: internalfontnumber, mut c: eightbits) {
    let __tekai_state_generated_backend_vfpacket: *mut crate::generated::backend::vfpacket::State =
        unsafe { crate::generated::backend::vfpacket::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    (*__tekai_state_generated_backend_vfpacket).packet_data_ptr =
        *(*(*__tekai_state_generated_backend_vfpacket).vf_array.offset(
            *(*__tekai_state_generated_pdftexextra)
                .vfpacketbase
                .offset(f as isize) as isize,
        ))
        .data
        .offset(
            (c as ::core::ffi::c_int
                - *(*__tekai_state_generated_pdftexextra)
                    .fontbc
                    .offset(f as isize) as ::core::ffi::c_int) as isize,
        );
    (*__tekai_state_generated_pdftexextra).vfpacketlength =
        *(*(*__tekai_state_generated_backend_vfpacket).vf_array.offset(
            *(*__tekai_state_generated_pdftexextra)
                .vfpacketbase
                .offset(f as isize) as isize,
        ))
        .len
        .offset(
            (c as ::core::ffi::c_int
                - *(*__tekai_state_generated_pdftexextra)
                    .fontbc
                    .offset(f as isize) as ::core::ffi::c_int) as isize,
        ) as integer;
}
pub unsafe fn packetbyte() -> eightbits {
    let __tekai_state_generated_backend_vfpacket: *mut crate::generated::backend::vfpacket::State =
        unsafe { crate::generated::backend::vfpacket::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    (*__tekai_state_generated_pdftexextra).vfpacketlength -= 1;
    let fresh3 = (*__tekai_state_generated_backend_vfpacket).packet_data_ptr;
    (*__tekai_state_generated_backend_vfpacket).packet_data_ptr =
        (*__tekai_state_generated_backend_vfpacket)
            .packet_data_ptr
            .offset(1);
    return *fresh3 as eightbits;
}
pub unsafe fn pushpacketstate() {
    let __tekai_state_generated_backend_vfpacket: *mut crate::generated::backend::vfpacket::State =
        unsafe { crate::generated::backend::vfpacket::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    if (*__tekai_state_generated_backend_vfpacket)
        .packet_array
        .is_null()
    {
        (*__tekai_state_generated_backend_vfpacket).packet_limit = 256 as size_t;
        if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
            > (*__tekai_state_generated_backend_vfpacket).packet_limit as ::core::ffi::c_uint
        {
            (*__tekai_state_generated_backend_vfpacket).packet_limit = 1 as size_t;
        }
        (*__tekai_state_generated_backend_vfpacket).packet_array = xmalloc(
            (*__tekai_state_generated_backend_vfpacket)
                .packet_limit
                .wrapping_mul(::core::mem::size_of::<packet_entry>() as size_t),
        ) as *mut packet_entry;
        (*__tekai_state_generated_backend_vfpacket).packet_ptr =
            (*__tekai_state_generated_backend_vfpacket).packet_array;
    } else if ((*__tekai_state_generated_backend_vfpacket)
        .packet_ptr
        .offset_from((*__tekai_state_generated_backend_vfpacket).packet_array)
        as ::core::ffi::c_long
        + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
        > (*__tekai_state_generated_backend_vfpacket).packet_limit as ::core::ffi::c_uint
    {
        (*__tekai_state_utils).last_ptr_index = (*__tekai_state_generated_backend_vfpacket)
            .packet_ptr
            .offset_from((*__tekai_state_generated_backend_vfpacket).packet_array)
            as ::core::ffi::c_long as size_t;
        (*__tekai_state_generated_backend_vfpacket).packet_limit =
            (*__tekai_state_generated_backend_vfpacket)
                .packet_limit
                .wrapping_mul(2 as size_t);
        if ((*__tekai_state_generated_backend_vfpacket)
            .packet_ptr
            .offset_from((*__tekai_state_generated_backend_vfpacket).packet_array)
            as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
            > (*__tekai_state_generated_backend_vfpacket).packet_limit as ::core::ffi::c_uint
        {
            (*__tekai_state_generated_backend_vfpacket).packet_limit =
                ((*__tekai_state_generated_backend_vfpacket)
                    .packet_ptr
                    .offset_from((*__tekai_state_generated_backend_vfpacket).packet_array)
                    as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as size_t;
        }
        if (*__tekai_state_generated_backend_vfpacket).packet_limit as ::core::ffi::c_uint
            > INT_MAX as ::core::ffi::c_uint
        {
            crate::utils::pdftex_fail_args(
                b"packet_array exceeds size limit\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        (*__tekai_state_generated_backend_vfpacket).packet_array = xrealloc(
            (*__tekai_state_generated_backend_vfpacket).packet_array as address,
            (*__tekai_state_generated_backend_vfpacket)
                .packet_limit
                .wrapping_mul(::core::mem::size_of::<packet_entry>() as size_t),
        ) as *mut packet_entry;
        (*__tekai_state_generated_backend_vfpacket).packet_ptr =
            (*__tekai_state_generated_backend_vfpacket)
                .packet_array
                .offset((*__tekai_state_utils).last_ptr_index as isize);
    }
    (*(*__tekai_state_generated_backend_vfpacket).packet_ptr).dataptr =
        (*__tekai_state_generated_backend_vfpacket).packet_data_ptr;
    (*(*__tekai_state_generated_backend_vfpacket).packet_ptr).len =
        (*__tekai_state_generated_pdftexextra).vfpacketlength;
    (*__tekai_state_generated_backend_vfpacket).packet_ptr =
        (*__tekai_state_generated_backend_vfpacket)
            .packet_ptr
            .offset(1);
}
pub unsafe fn poppacketstate() {
    let __tekai_state_generated_backend_vfpacket: *mut crate::generated::backend::vfpacket::State =
        unsafe { crate::generated::backend::vfpacket::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    if (*__tekai_state_generated_backend_vfpacket).packet_ptr
        == (*__tekai_state_generated_backend_vfpacket).packet_array
    {
        crate::utils::pdftex_fail_args(
            b"packet stack empty, impossible to pop\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
    }
    (*__tekai_state_generated_backend_vfpacket).packet_ptr =
        (*__tekai_state_generated_backend_vfpacket)
            .packet_ptr
            .offset(-1);
    (*__tekai_state_generated_backend_vfpacket).packet_data_ptr =
        (*(*__tekai_state_generated_backend_vfpacket).packet_ptr).dataptr;
    (*__tekai_state_generated_pdftexextra).vfpacketlength =
        (*(*__tekai_state_generated_backend_vfpacket).packet_ptr).len;
}
pub unsafe fn vf_free() {
    let __tekai_state_generated_backend_vfpacket: *mut crate::generated::backend::vfpacket::State =
        unsafe { crate::generated::backend::vfpacket::state() };

    let mut v: *mut vf_entry = ::core::ptr::null_mut::<vf_entry>();
    let mut p: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    if !(*__tekai_state_generated_backend_vfpacket)
        .vf_array
        .is_null()
    {
        v = (*__tekai_state_generated_backend_vfpacket).vf_array;
        while v < (*__tekai_state_generated_backend_vfpacket).vf_ptr {
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
        if !(*__tekai_state_generated_backend_vfpacket)
            .vf_array
            .is_null()
        {
            free((*__tekai_state_generated_backend_vfpacket).vf_array as *mut ::core::ffi::c_void);
        }
        (*__tekai_state_generated_backend_vfpacket).vf_array = ::core::ptr::null_mut::<vf_entry>();
    }
    if !(*__tekai_state_generated_backend_vfpacket)
        .packet_array
        .is_null()
    {
        free((*__tekai_state_generated_backend_vfpacket).packet_array as *mut ::core::ffi::c_void);
    }
    (*__tekai_state_generated_backend_vfpacket).packet_array =
        ::core::ptr::null_mut::<packet_entry>();
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;

pub(crate) struct State {
    packet_limit: size_t,
    packet_array: *mut packet_entry,
    packet_ptr: *mut packet_entry,
    vf_array: *mut vf_entry,
    vf_limit: size_t,
    vf_ptr: *mut vf_entry,
    packet_data_ptr: *mut ::core::ffi::c_char,
}

#[inline(always)]
fn __tekai_require_copy<T: Copy>() {}

pub(crate) struct StateInitialization {
    destination: *mut State,
    initialized: [bool; 7],
}
#[inline]
unsafe fn drop_initialized<T>(pointer: *mut T) {
    if core::mem::needs_drop::<T>() {
        unsafe { core::ptr::drop_in_place(pointer) };
    }
}

impl Drop for StateInitialization {
    fn drop(&mut self) {
        unsafe {
            if self.initialized[6] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).packet_data_ptr));
            }
            if self.initialized[5] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).vf_ptr));
            }
            if self.initialized[4] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).vf_limit));
            }
            if self.initialized[3] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).vf_array));
            }
            if self.initialized[2] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).packet_ptr));
            }
            if self.initialized[1] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).packet_array));
            }
            if self.initialized[0] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).packet_limit));
            }
        }
    }
}
impl StateInitialization {
    pub(crate) unsafe fn begin(destination: *mut State) -> Self {
        Self {
            destination,
            initialized: [false; 7],
        }
    }
    pub(crate) fn disarm(&mut self) {
        self.initialized.fill(false);
    }
    pub(crate) unsafe fn initialize_fields(
        &mut self,
        invocation: *const crate::invocation_state::Storage,
    ) {
        let destination = self.destination;
        let _ = invocation;
        const {
            assert!(core::mem::size_of::<size_t>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).packet_limit).write(0);
        self.initialized[0] = true;
        const {
            assert!(core::mem::size_of::<*mut packet_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).packet_array)
            .write(::core::ptr::null::<packet_entry>() as *mut packet_entry);
        self.initialized[1] = true;
        const {
            assert!(core::mem::size_of::<*mut packet_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).packet_ptr)
            .write(::core::ptr::null::<packet_entry>() as *mut packet_entry);
        self.initialized[2] = true;
        const {
            assert!(core::mem::size_of::<*mut vf_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).vf_array)
            .write(::core::ptr::null::<vf_entry>() as *mut vf_entry);
        self.initialized[3] = true;
        const {
            assert!(core::mem::size_of::<size_t>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).vf_limit).write(0);
        self.initialized[4] = true;
        const {
            assert!(core::mem::size_of::<*mut vf_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).vf_ptr)
            .write(::core::ptr::null::<vf_entry>() as *mut vf_entry);
        self.initialized[5] = true;
        const {
            assert!(core::mem::size_of::<*mut ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).packet_data_ptr)
            .write(::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char);
        self.initialized[6] = true;
    }
    pub(crate) unsafe fn initialize_fixups(
        &mut self,
        invocation: *const crate::invocation_state::Storage,
    ) {
        let destination = self.destination;
        let _ = invocation;
    }
}
pub(crate) unsafe fn state() -> *mut State {
    core::cell::UnsafeCell::raw_get(core::ptr::addr_of!(
        (*crate::invocation_state::current()).generated_backend_vfpacket
    ))
}
#[cfg(test)]
pub(crate) unsafe fn check_state_construction(phase: u8) {
    let invocation = crate::invocation_state::current();
    let destination = state();
    assert!(!destination.is_null());
    let _ = (invocation, phase);
}
