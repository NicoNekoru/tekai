// Rust translation and subsequent modifications for Tekai.
// Upstream TeX Live source revision 52dac44dde13a8310bbb8231e81c58be28cebe20.
// Origin texk/web2c/pdftexdir/epdf.c.
// GPL-2.0-or-later.
// Modified for Tekai. Licensing and provenance notice restored 2026-10-13.
// Original copyrights and license terms are retained in the crate's
// NOTICE.md and licenses/upstream-notices.txt.

pub use crate::generated::backend::avlstuff::comp_string_entry;
pub use crate::generated::backend::writefont::{lookup_fd_entry, new_fd_entry, register_fd_entry};
pub use crate::generated::pdftex0::pdfnewobjnum;
pub use crate::pdftoepdf::epdf_check_mem;

pub use crate::generated::backend::avl::{avl_create, avl_find, avl_probe};

use crate::byte_string::strlen;

use crate::openclose::xstrdup;

use crate::termination::__assert_rtn;

pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type boolean = ::core::ffi::c_int;
pub type string = *mut ::core::ffi::c_char;
pub type const_string = *const ::core::ffi::c_char;
pub type integer = ::core::ffi::c_int;
pub use crate::backend_types::_subfont_entry;
pub type subfont_entry = _subfont_entry;
pub use crate::backend_types::fd_entry_;
#[cfg(test)]
pub use crate::backend_types::fe_entry;
pub use crate::backend_types::fm_entry;
#[cfg(test)]
pub use crate::backend_types::intparm;
pub type fd_entry = fd_entry_;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const STEMV_CODE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const F_SUBSETTED: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub unsafe fn is_subsetable(mut fm: *mut fm_entry) -> ::core::ffi::c_int {
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
pub unsafe fn epdf_create_fontdescriptor(
    mut fm: *mut fm_entry,
    mut stemV: ::core::ffi::c_int,
) -> *mut fd_entry {
    let __tekai_state_generated_backend_avlstuff: *mut crate::generated::backend::avlstuff::State =
        unsafe { crate::generated::backend::avlstuff::state() };

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
                    as unsafe fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
            &raw mut (*__tekai_state_generated_backend_avlstuff).avl_xallocator,
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
pub unsafe fn get_fd_objnum(mut fd: *mut fd_entry) -> ::core::ffi::c_int {
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
pub unsafe fn get_fn_objnum(mut fd: *mut fd_entry) -> ::core::ffi::c_int {
    if (*fd).fn_objnum == 0 as ::core::ffi::c_int {
        (*fd).fn_objnum = pdfnewobjnum();
    }
    return (*fd).fn_objnum as ::core::ffi::c_int;
}
pub unsafe fn epdf_mark_glyphs(mut fd: *mut fd_entry, mut charset: *mut ::core::ffi::c_char) {
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
pub unsafe fn embed_whole_font(mut fd: *mut fd_entry) {
    (*fd).all_glyphs = true_0 as boolean;
}
pub unsafe fn epdf_free() {
    epdf_check_mem();
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
