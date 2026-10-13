// Rust translation and subsequent modifications for Tekai.
// Upstream TeX Live source revision 52dac44dde13a8310bbb8231e81c58be28cebe20.
// Origin texk/web2c/pdftexdir/writeenc.c.
// GPL-2.0-or-later.
// Modified for Tekai. Licensing and provenance notice restored 2026-10-13.
// Original copyrights and license terms are retained in the crate's
// NOTICE.md and licenses/upstream-notices.txt.

pub use crate::generated::backend::writet1::load_enc_file;
pub use crate::generated::pdftex0::{pdfenddict, zpdfbegindict};
pub use crate::utils::pdf_puts;

pub use crate::generated::backend::avl::{
    avl_create, avl_destroy, avl_find, avl_node, avl_probe, avl_t_first, avl_t_init, avl_t_next,
    avl_table, avl_traverser,
};

use crate::byte_string::strcmp;

use crate::allocation::free;
use crate::openclose::xmalloc;

use crate::termination::__assert_rtn;

pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type address = *mut ::core::ffi::c_void;
pub type integer = ::core::ffi::c_int;
pub use crate::backend_types::fe_entry;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

unsafe fn comp_fe_entry(
    mut pa: *const ::core::ffi::c_void,
    mut pb: *const ::core::ffi::c_void,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        (*(pa as *const fe_entry)).name,
        (*(pb as *const fe_entry)).name,
    );
}
unsafe fn new_fe_entry() -> *mut fe_entry {
    let mut fe: *mut fe_entry = ::core::ptr::null_mut::<fe_entry>();
    fe = xmalloc((1 as size_t).wrapping_mul(::core::mem::size_of::<fe_entry>() as size_t))
        as *mut fe_entry;
    (*fe).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*fe).fe_objnum = 0 as ::core::ffi::c_int as integer;
    (*fe).glyph_names = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*fe).tx_tree = ::core::ptr::null_mut::<avl_table>();
    return fe;
}
unsafe fn lookup_fe_entry(mut s: *mut ::core::ffi::c_char) -> *mut fe_entry {
    let __tekai_state_generated_backend_avlstuff: *mut crate::generated::backend::avlstuff::State =
        unsafe { crate::generated::backend::avlstuff::state() };
    let __tekai_state_generated_backend_writeenc: *mut crate::generated::backend::writeenc::State =
        unsafe { crate::generated::backend::writeenc::state() };

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
    if (*__tekai_state_generated_backend_writeenc)
        .fe_tree
        .is_null()
    {
        (*__tekai_state_generated_backend_writeenc).fe_tree = avl_create(
            Some(
                comp_fe_entry
                    as unsafe fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
            &raw mut (*__tekai_state_generated_backend_avlstuff).avl_xallocator,
        );
        if (*__tekai_state_generated_backend_writeenc)
            .fe_tree
            .is_null() as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            __assert_rtn(
                b"lookup_fe_entry\0" as *const u8 as *const ::core::ffi::c_char,
                b"writeenc.c\0" as *const u8 as *const ::core::ffi::c_char,
                52 as ::core::ffi::c_int,
                b"fe_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    return avl_find(
        (*__tekai_state_generated_backend_writeenc).fe_tree,
        &raw mut fe as *const ::core::ffi::c_void,
    ) as *mut fe_entry;
}
unsafe fn register_fe_entry(mut fe: *mut fe_entry) {
    let __tekai_state_generated_backend_avlstuff: *mut crate::generated::backend::avlstuff::State =
        unsafe { crate::generated::backend::avlstuff::state() };
    let __tekai_state_generated_backend_writeenc: *mut crate::generated::backend::writeenc::State =
        unsafe { crate::generated::backend::writeenc::state() };

    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    if (*__tekai_state_generated_backend_writeenc)
        .fe_tree
        .is_null()
    {
        (*__tekai_state_generated_backend_writeenc).fe_tree = avl_create(
            Some(
                comp_fe_entry
                    as unsafe fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
            &raw mut (*__tekai_state_generated_backend_avlstuff).avl_xallocator,
        );
        if (*__tekai_state_generated_backend_writeenc)
            .fe_tree
            .is_null() as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
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
    aa = avl_probe(
        (*__tekai_state_generated_backend_writeenc).fe_tree,
        fe as *mut ::core::ffi::c_void,
    );
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
pub unsafe fn get_fe_entry(mut s: *mut ::core::ffi::c_char) -> *mut fe_entry {
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
pub unsafe fn epdf_write_enc(
    mut glyph_names: *mut *mut ::core::ffi::c_char,
    mut fe_objnum: ::core::ffi::c_int,
) {
    let __tekai_state_generated_backend_writettf: *mut crate::generated::backend::writettf::State =
        unsafe { crate::generated::backend::writettf::state() };

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
        if *glyph_names.offset(i as isize)
            != &raw mut (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char
        {
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
unsafe fn write_enc(
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
unsafe fn write_fontencoding(mut fe: *mut fe_entry) {
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
pub unsafe fn write_fontencodings() {
    let __tekai_state_generated_backend_writeenc: *mut crate::generated::backend::writeenc::State =
        unsafe { crate::generated::backend::writeenc::state() };

    let mut fe: *mut fe_entry = ::core::ptr::null_mut::<fe_entry>();
    let mut t: avl_traverser = avl_traverser {
        avl_table: ::core::ptr::null_mut::<avl_table>(),
        avl_node: ::core::ptr::null_mut::<avl_node>(),
        avl_stack: [::core::ptr::null_mut::<avl_node>(); 32],
        avl_height: 0,
        avl_generation: 0,
    };
    if (*__tekai_state_generated_backend_writeenc)
        .fe_tree
        .is_null()
    {
        return;
    }
    avl_t_init(
        &raw mut t,
        (*__tekai_state_generated_backend_writeenc).fe_tree,
    );
    fe = avl_t_first(
        &raw mut t,
        (*__tekai_state_generated_backend_writeenc).fe_tree,
    ) as *mut fe_entry;
    while !fe.is_null() {
        if (*fe).fe_objnum != 0 as ::core::ffi::c_int {
            write_fontencoding(fe);
        }
        fe = avl_t_next(&raw mut t) as *mut fe_entry;
    }
}
unsafe fn destroy_fe_entry(mut pa: *mut ::core::ffi::c_void, mut pb: *mut ::core::ffi::c_void) {
    let __tekai_state_generated_backend_writettf: *mut crate::generated::backend::writettf::State =
        unsafe { crate::generated::backend::writettf::state() };

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
            if *(*p).glyph_names.offset(i as isize)
                != &raw mut (*__tekai_state_generated_backend_writettf).notdef
                    as *mut ::core::ffi::c_char
            {
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
pub unsafe fn enc_free() {
    let __tekai_state_generated_backend_writeenc: *mut crate::generated::backend::writeenc::State =
        unsafe { crate::generated::backend::writeenc::state() };

    if !(*__tekai_state_generated_backend_writeenc)
        .fe_tree
        .is_null()
    {
        avl_destroy(
            (*__tekai_state_generated_backend_writeenc).fe_tree,
            Some(
                destroy_fe_entry
                    as unsafe fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
            ),
        );
    }
    (*__tekai_state_generated_backend_writeenc).fe_tree = ::core::ptr::null_mut::<avl_table>();
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;

pub(crate) struct State {
    fe_tree: *mut avl_table,
}

#[inline(always)]
fn __tekai_require_copy<T: Copy>() {}

pub(crate) struct StateInitialization {
    destination: *mut State,
    initialized: [bool; 1],
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
            if self.initialized[0] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fe_tree));
            }
        }
    }
}
impl StateInitialization {
    pub(crate) unsafe fn begin(destination: *mut State) -> Self {
        Self {
            destination,
            initialized: [false; 1],
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
            assert!(core::mem::size_of::<*mut avl_table>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fe_tree)
            .write(::core::ptr::null::<avl_table>() as *mut avl_table);
        self.initialized[0] = true;
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
        (*crate::invocation_state::current()).generated_backend_writeenc
    ))
}
#[cfg(test)]
pub(crate) unsafe fn check_state_construction(phase: u8) {
    let invocation = crate::invocation_state::current();
    let destination = state();
    assert!(!destination.is_null());
    let _ = (invocation, phase);
}
