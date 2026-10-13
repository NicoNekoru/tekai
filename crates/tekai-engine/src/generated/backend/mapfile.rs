// Rust translation and subsequent modifications for Tekai.
// Upstream TeX Live source revision 52dac44dde13a8310bbb8231e81c58be28cebe20.
// Origin texk/web2c/pdftexdir/mapfile.c.
// GPL-2.0-or-later.
// Modified for Tekai. Licensing and provenance notice restored 2026-10-13.
// Original copyrights and license terms are retained in the crate's
// NOTICE.md and licenses/upstream-notices.txt.

pub use crate::generated::backend::avlstuff::comp_string_entry;
pub use crate::generated::backend::subfont::handle_subfont_fm;
pub use crate::generated::backend::writeenc::get_fe_entry;
pub use crate::generated::pdftex0::{
    getnullstr, getpdfsuppresswarningdupmap, zflushstr, zpackfilename, ztokenstostring,
};
pub use crate::generated::pdftexextra::makecstring;
pub(crate) use crate::kpathsea::kpse_find_file;
pub use crate::utils::maketexstring;

pub use crate::generated::backend::avl::{
    avl_create, avl_delete, avl_destroy, avl_find, avl_node, avl_probe, avl_t_find, avl_t_next,
    avl_t_prev, avl_table, avl_traverser,
};

use crate::byte_string::isdigit;
use crate::byte_string::{strcasecmp, strchr, strcmp, strlen, strncmp, strstr};

use crate::backend_grammar::platform_encoding;
use crate::backend_parse::FieldError;
use crate::float_parse::{map_operand, scaled_map_operand, FloatError};
use crate::integer_parse::{decimal_i32_legacy, signed_prefix, IntegerBase};
use crate::openclose::open_input;
use crate::stdio::feof;
use crate::support::xfclose;
use crate::utils::xgetc;
use std::ffi::CStr;

use crate::allocation::free;
use crate::openclose::{xmalloc, xstrdup};

use crate::termination::__assert_rtn;

pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __darwin_size_t = usize;
pub type __darwin_wchar_t = ::libc::wchar_t;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
pub use crate::stdio::Stream as FILE;
pub type boolean = ::core::ffi::c_int;
pub type string = *mut ::core::ffi::c_char;
pub type const_string = *const ::core::ffi::c_char;
pub type address = *mut ::core::ffi::c_void;
pub type kpse_file_format_type = ::core::ffi::c_uint;
pub const kpse_last_format: kpse_file_format_type = 59;
pub const kpse_bltxml_format: kpse_file_format_type = 58;
pub const kpse_ris_format: kpse_file_format_type = 57;
pub const kpse_clua_format: kpse_file_format_type = 56;
pub const kpse_mlbst_format: kpse_file_format_type = 55;
pub const kpse_mlbib_format: kpse_file_format_type = 54;
pub const kpse_cid_format: kpse_file_format_type = 53;
pub const kpse_fea_format: kpse_file_format_type = 52;
pub const kpse_lua_format: kpse_file_format_type = 51;
pub const kpse_texmfscripts_format: kpse_file_format_type = 50;
pub const kpse_lig_format: kpse_file_format_type = 49;
pub const kpse_pdftex_config_format: kpse_file_format_type = 48;
pub const kpse_opentype_format: kpse_file_format_type = 47;
pub const kpse_sfd_format: kpse_file_format_type = 46;
pub const kpse_cmap_format: kpse_file_format_type = 45;
pub const kpse_enc_format: kpse_file_format_type = 44;
pub const kpse_cweb_format: kpse_file_format_type = 43;
pub const kpse_web_format: kpse_file_format_type = 42;
pub const kpse_miscfonts_format: kpse_file_format_type = 41;
pub const kpse_program_binary_format: kpse_file_format_type = 40;
pub const kpse_program_text_format: kpse_file_format_type = 39;
pub const kpse_web2c_format: kpse_file_format_type = 38;
pub const kpse_type42_format: kpse_file_format_type = 37;
pub const kpse_truetype_format: kpse_file_format_type = 36;
pub const kpse_ist_format: kpse_file_format_type = 35;
pub const kpse_dvips_config_format: kpse_file_format_type = 34;
pub const kpse_vf_format: kpse_file_format_type = 33;
pub const kpse_type1_format: kpse_file_format_type = 32;
pub const kpse_troff_font_format: kpse_file_format_type = 31;
pub const kpse_tex_ps_header_format: kpse_file_format_type = 30;
pub const kpse_texsource_format: kpse_file_format_type = 29;
pub const kpse_texpool_format: kpse_file_format_type = 28;
pub const kpse_texdoc_format: kpse_file_format_type = 27;
pub const kpse_tex_format: kpse_file_format_type = 26;
pub const kpse_pict_format: kpse_file_format_type = 25;
pub const kpse_ovp_format: kpse_file_format_type = 24;
pub const kpse_ovf_format: kpse_file_format_type = 23;
pub const kpse_otp_format: kpse_file_format_type = 22;
pub const kpse_opl_format: kpse_file_format_type = 21;
pub const kpse_ofm_format: kpse_file_format_type = 20;
pub const kpse_ocp_format: kpse_file_format_type = 19;
pub const kpse_mpsupport_format: kpse_file_format_type = 18;
pub const kpse_mppool_format: kpse_file_format_type = 17;
pub const kpse_mp_format: kpse_file_format_type = 16;
pub const kpse_mft_format: kpse_file_format_type = 15;
pub const kpse_mfpool_format: kpse_file_format_type = 14;
pub const kpse_mf_format: kpse_file_format_type = 13;
pub const kpse_mem_format: kpse_file_format_type = 12;
pub const kpse_fontmap_format: kpse_file_format_type = 11;
pub const kpse_fmt_format: kpse_file_format_type = 10;
pub const kpse_db_format: kpse_file_format_type = 9;
pub const kpse_cnf_format: kpse_file_format_type = 8;
pub const kpse_bst_format: kpse_file_format_type = 7;
pub const kpse_bib_format: kpse_file_format_type = 6;
pub const kpse_base_format: kpse_file_format_type = 5;
pub const kpse_afm_format: kpse_file_format_type = 4;
pub const kpse_tfm_format: kpse_file_format_type = 3;
pub const kpse_any_glyph_format: kpse_file_format_type = 2;
pub const kpse_pk_format: kpse_file_format_type = 1;
pub const kpse_gf_format: kpse_file_format_type = 0;
pub type integer = ::core::ffi::c_int;
pub type ASCIIcode = ::core::ffi::c_uchar;
pub type strnumber = integer;
pub type halfword = integer;
pub type internalfontnumber = integer;
pub type fmentryptr = *mut integer;
pub use crate::backend_types::_subfont_entry;
pub type subfont_entry = _subfont_entry;
pub use crate::backend_types::fe_entry;
pub use crate::backend_types::ff_entry;
pub use crate::backend_types::fm_entry;
pub type mapitem = mitem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mitem {
    pub mode: updatemode,
    pub type_0: maptype,
    pub line: *mut ::core::ffi::c_char,
    pub lineno: ::core::ffi::c_int,
}
pub type maptype = ::core::ffi::c_uint;
pub const MAPLINE: maptype = 1;
pub const MAPFILE: maptype = 0;
pub type updatemode = ::core::ffi::c_uint;
pub const FM_DELETE: updatemode = 2;
pub const FM_REPLACE: updatemode = 1;
pub const FM_DUPIGNORE: updatemode = 0;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const FOPEN_RBIN_MODE: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"rb\0") };
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const F_INCLUDED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const F_SUBSETTED: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const F_STDT1FONT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const F_TYPE1: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const F_TRUETYPE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const F_OTF: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const F_PK: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const LINK_TFM: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const LINK_PS: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const FD_FLAGS_NOT_SET_IN_MAPLINE: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);

pub unsafe fn new_fm_entry() -> *mut fm_entry {
    let mut fm: *mut fm_entry = ::core::ptr::null_mut::<fm_entry>();
    fm = xmalloc((1 as size_t).wrapping_mul(::core::mem::size_of::<fm_entry>() as size_t))
        as *mut fm_entry;
    (*fm).tfm_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*fm).sfd_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*fm).ps_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*fm).fd_flags = FD_FLAGS_NOT_SET_IN_MAPLINE as integer;
    (*fm).ff_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*fm).encname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*fm).type_0 = 0 as ::core::ffi::c_ushort;
    (*fm).slant = 0 as ::core::ffi::c_int as integer;
    (*fm).extend = 0 as ::core::ffi::c_int as integer;
    (*fm).links = 0 as ::core::ffi::c_ushort;
    (*fm).pid = -(1 as ::core::ffi::c_int) as ::core::ffi::c_short;
    (*fm).eid = -(1 as ::core::ffi::c_int) as ::core::ffi::c_short;
    (*fm).subfont = ::core::ptr::null_mut::<subfont_entry>();
    (*fm).in_use = false_0 as boolean;
    return fm;
}
pub unsafe fn delete_fm_entry(mut fm: *mut fm_entry) {
    if !(*fm).tfm_name.is_null() {
        free((*fm).tfm_name as *mut ::core::ffi::c_void);
    }
    (*fm).tfm_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*fm).sfd_name.is_null() {
        free((*fm).sfd_name as *mut ::core::ffi::c_void);
    }
    (*fm).sfd_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*fm).ps_name.is_null() {
        free((*fm).ps_name as *mut ::core::ffi::c_void);
    }
    (*fm).ps_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*fm).ff_name.is_null() {
        free((*fm).ff_name as *mut ::core::ffi::c_void);
    }
    (*fm).ff_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !fm.is_null() {
        free(fm as *mut ::core::ffi::c_void);
    }
    fm = ::core::ptr::null_mut::<fm_entry>();
}
unsafe fn new_ff_entry() -> *mut ff_entry {
    let mut ff: *mut ff_entry = ::core::ptr::null_mut::<ff_entry>();
    ff = xmalloc((1 as size_t).wrapping_mul(::core::mem::size_of::<ff_entry>() as size_t))
        as *mut ff_entry;
    (*ff).ff_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*ff).ff_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return ff;
}
unsafe fn delete_ff_entry(mut ff: *mut ff_entry) {
    if !(*ff).ff_name.is_null() {
        free((*ff).ff_name as *mut ::core::ffi::c_void);
    }
    (*ff).ff_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*ff).ff_path.is_null() {
        free((*ff).ff_path as *mut ::core::ffi::c_void);
    }
    (*ff).ff_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !ff.is_null() {
        free(ff as *mut ::core::ffi::c_void);
    }
    ff = ::core::ptr::null_mut::<ff_entry>();
}
unsafe fn dummy_fm_entry() -> *mut fm_entry {
    let __tekai_state_generated_backend_mapfile: *mut crate::generated::backend::mapfile::State =
        unsafe { crate::generated::backend::mapfile::state() };

    return &raw mut (*__tekai_state_generated_backend_mapfile)
        .local_dummy_fm_entry_228_const_fm_entry;
}

unsafe fn comp_fm_entry_tfm(
    mut pa: *const ::core::ffi::c_void,
    mut pb: *const ::core::ffi::c_void,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        (*(pa as *const fm_entry)).tfm_name,
        (*(pb as *const fm_entry)).tfm_name,
    );
}
unsafe fn comp_fm_entry_ps(
    mut pa: *const ::core::ffi::c_void,
    mut pb: *const ::core::ffi::c_void,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut p1: *const fm_entry = pa as *const fm_entry;
    let mut p2: *const fm_entry = pb as *const fm_entry;
    if !(!(*p1).ps_name.is_null() && !(*p2).ps_name.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"comp_fm_entry_ps\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            140 as ::core::ffi::c_int,
            b"p1->ps_name != NULL && p2->ps_name != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
    };
    i = strcmp((*p1).ps_name, (*p2).ps_name);
    if i != 0 as ::core::ffi::c_int {
        return i;
    }
    if (*p1).slant > (*p2).slant {
        return 1 as ::core::ffi::c_int;
    }
    if (*p1).slant < (*p2).slant {
        return -(1 as ::core::ffi::c_int);
    }
    if (*p1).extend > (*p2).extend {
        return 1 as ::core::ffi::c_int;
    }
    if (*p1).extend < (*p2).extend {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe fn comp_ff_entry(
    mut pa: *const ::core::ffi::c_void,
    mut pb: *const ::core::ffi::c_void,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        (*(pa as *const ff_entry)).ff_name,
        (*(pb as *const ff_entry)).ff_name,
    );
}
unsafe fn create_avl_trees() {
    let __tekai_state_generated_backend_avlstuff: *mut crate::generated::backend::avlstuff::State =
        unsafe { crate::generated::backend::avlstuff::state() };
    let __tekai_state_generated_backend_mapfile: *mut crate::generated::backend::mapfile::State =
        unsafe { crate::generated::backend::mapfile::state() };

    if !(*__tekai_state_generated_backend_mapfile)
        .tfm_tree
        .is_null() as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"create_avl_trees\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            158 as ::core::ffi::c_int,
            b"tfm_tree == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*__tekai_state_generated_backend_mapfile).tfm_tree = avl_create(
        Some(
            comp_fm_entry_tfm
                as unsafe fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        NULL,
        &raw mut (*__tekai_state_generated_backend_avlstuff).avl_xallocator,
    );
    if (*__tekai_state_generated_backend_mapfile)
        .tfm_tree
        .is_null() as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"create_avl_trees\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            160 as ::core::ffi::c_int,
            b"tfm_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !(*__tekai_state_generated_backend_mapfile).ps_tree.is_null() as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"create_avl_trees\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            161 as ::core::ffi::c_int,
            b"ps_tree == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*__tekai_state_generated_backend_mapfile).ps_tree = avl_create(
        Some(
            comp_fm_entry_ps
                as unsafe fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        NULL,
        &raw mut (*__tekai_state_generated_backend_avlstuff).avl_xallocator,
    );
    if (*__tekai_state_generated_backend_mapfile).ps_tree.is_null() as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"create_avl_trees\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            163 as ::core::ffi::c_int,
            b"ps_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !(*__tekai_state_generated_backend_mapfile).ff_tree.is_null() as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"create_avl_trees\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            164 as ::core::ffi::c_int,
            b"ff_tree == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*__tekai_state_generated_backend_mapfile).ff_tree = avl_create(
        Some(
            comp_ff_entry
                as unsafe fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        NULL,
        &raw mut (*__tekai_state_generated_backend_avlstuff).avl_xallocator,
    );
    if (*__tekai_state_generated_backend_mapfile).ff_tree.is_null() as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"create_avl_trees\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            166 as ::core::ffi::c_int,
            b"ff_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !(*__tekai_state_generated_backend_mapfile)
        .encname_tree
        .is_null() as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"create_avl_trees\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            167 as ::core::ffi::c_int,
            b"encname_tree == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*__tekai_state_generated_backend_mapfile).encname_tree = avl_create(
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
    if (*__tekai_state_generated_backend_mapfile)
        .encname_tree
        .is_null() as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"create_avl_trees\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            169 as ::core::ffi::c_int,
            b"encname_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
}
pub unsafe fn avl_do_entry(
    mut fm: *mut fm_entry,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __tekai_state_generated_backend_mapfile: *mut crate::generated::backend::mapfile::State =
        unsafe { crate::generated::backend::mapfile::state() };

    let mut current_block: u64;
    let mut p: *mut fm_entry = ::core::ptr::null_mut::<fm_entry>();
    let mut a: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut suppress_warn: boolean =
        (getpdfsuppresswarningdupmap() > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if strcmp(
        (*fm).tfm_name,
        &raw const (*__tekai_state_generated_backend_mapfile).nontfm as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        p = avl_find(
            (*__tekai_state_generated_backend_mapfile).tfm_tree,
            fm as *const ::core::ffi::c_void,
        ) as *mut fm_entry;
        if !p.is_null() {
            match mode {
                0 => {
                    if suppress_warn == 0 {
                        crate::utils::pdftex_warn_args(
                            b"fontmap entry for `%s' already exists, duplicates ignored\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            &[crate::utils::PrintfArg::from((*fm).tfm_name)],
                        );
                    }
                    current_block = 12876239492772886194;
                }
                1 | 2 => {
                    if (*p).in_use != 0 {
                        crate::utils::pdftex_warn_args(
                            b"fontmap entry for `%s' has been used, replace/delete not allowed\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            &[crate::utils::PrintfArg::from((*fm).tfm_name)],
                        );
                        current_block = 12876239492772886194;
                    } else {
                        a = avl_delete(
                            (*__tekai_state_generated_backend_mapfile).tfm_tree,
                            p as *const ::core::ffi::c_void,
                        );
                        if a.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                            __assert_rtn(
                                b"avl_do_entry\0" as *const u8 as *const ::core::ffi::c_char,
                                b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                                209 as ::core::ffi::c_int,
                                b"a != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                        } else {
                        };
                        (*p).links =
                            ((*p).links as ::core::ffi::c_int & !LINK_TFM) as ::core::ffi::c_ushort;
                        if (*p).links as ::core::ffi::c_int & LINK_PS == 0 {
                            delete_fm_entry(p);
                        }
                        current_block = 13056961889198038528;
                    }
                }
                _ => {
                    if (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as ::core::ffi::c_long
                        != 0
                    {
                        __assert_rtn(
                            b"avl_do_entry\0" as *const u8 as *const ::core::ffi::c_char,
                            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                            215 as ::core::ffi::c_int,
                            b"0\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                    };
                    current_block = 13056961889198038528;
                }
            }
        } else {
            current_block = 13056961889198038528;
        }
        match current_block {
            12876239492772886194 => {}
            _ => {
                if mode != FM_DELETE as ::core::ffi::c_int {
                    aa = avl_probe(
                        (*__tekai_state_generated_backend_mapfile).tfm_tree,
                        fm as *mut ::core::ffi::c_void,
                    );
                    if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                        __assert_rtn(
                            b"avl_do_entry\0" as *const u8 as *const ::core::ffi::c_char,
                            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                            220 as ::core::ffi::c_int,
                            b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                    };
                    (*fm).links =
                        ((*fm).links as ::core::ffi::c_int | LINK_TFM) as ::core::ffi::c_ushort;
                }
                current_block = 4808432441040389987;
            }
        }
    } else {
        current_block = 4808432441040389987;
    }
    match current_block {
        4808432441040389987 => {
            if !(*fm).ps_name.is_null() {
                p = avl_find(
                    (*__tekai_state_generated_backend_mapfile).ps_tree,
                    fm as *const ::core::ffi::c_void,
                ) as *mut fm_entry;
                if !p.is_null() {
                    match mode {
                        0 => {
                            current_block = 12876239492772886194;
                        }
                        1 | 2 => {
                            current_block = 14402169362640155747;
                            match current_block {
                                12127208494873144039 => {
                                    if (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                                        as ::core::ffi::c_long
                                        != 0
                                    {
                                        __assert_rtn(
                                            b"avl_do_entry\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            b"mapfile.c\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            245 as ::core::ffi::c_int,
                                            b"0\0" as *const u8 as *const ::core::ffi::c_char,
                                        );
                                    } else {
                                    };
                                    current_block = 5494826135382683477;
                                }
                                _ => {
                                    if (*p).in_use != 0 {
                                        current_block = 12876239492772886194;
                                    } else {
                                        a = avl_delete(
                                            (*__tekai_state_generated_backend_mapfile).ps_tree,
                                            p as *const ::core::ffi::c_void,
                                        );
                                        if a.is_null() as ::core::ffi::c_int as ::core::ffi::c_long
                                            != 0
                                        {
                                            __assert_rtn(
                                                b"avl_do_entry\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                b"mapfile.c\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                239 as ::core::ffi::c_int,
                                                b"a != NULL\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            );
                                        } else {
                                        };
                                        (*p).links = ((*p).links as ::core::ffi::c_int & !LINK_PS)
                                            as ::core::ffi::c_ushort;
                                        if (*p).links as ::core::ffi::c_int & LINK_TFM == 0 {
                                            delete_fm_entry(p);
                                        }
                                        current_block = 5494826135382683477;
                                    }
                                }
                            }
                        }
                        _ => {
                            current_block = 12127208494873144039;
                            match current_block {
                                12127208494873144039 => {
                                    if (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                                        as ::core::ffi::c_long
                                        != 0
                                    {
                                        __assert_rtn(
                                            b"avl_do_entry\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            b"mapfile.c\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            245 as ::core::ffi::c_int,
                                            b"0\0" as *const u8 as *const ::core::ffi::c_char,
                                        );
                                    } else {
                                    };
                                    current_block = 5494826135382683477;
                                }
                                _ => {
                                    if (*p).in_use != 0 {
                                        current_block = 12876239492772886194;
                                    } else {
                                        a = avl_delete(
                                            (*__tekai_state_generated_backend_mapfile).ps_tree,
                                            p as *const ::core::ffi::c_void,
                                        );
                                        if a.is_null() as ::core::ffi::c_int as ::core::ffi::c_long
                                            != 0
                                        {
                                            __assert_rtn(
                                                b"avl_do_entry\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                b"mapfile.c\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                239 as ::core::ffi::c_int,
                                                b"a != NULL\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            );
                                        } else {
                                        };
                                        (*p).links = ((*p).links as ::core::ffi::c_int & !LINK_PS)
                                            as ::core::ffi::c_ushort;
                                        if (*p).links as ::core::ffi::c_int & LINK_TFM == 0 {
                                            delete_fm_entry(p);
                                        }
                                        current_block = 5494826135382683477;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    current_block = 5494826135382683477;
                }
                match current_block {
                    12876239492772886194 => {}
                    _ => {
                        if mode != FM_DELETE as ::core::ffi::c_int
                            && (!(*fm).ff_name.is_null()
                                && (*fm).type_0 as ::core::ffi::c_int & F_TYPE1
                                    != 0 as ::core::ffi::c_int)
                            && (*fm).type_0 as ::core::ffi::c_int & F_INCLUDED
                                != 0 as ::core::ffi::c_int
                        {
                            aa = avl_probe(
                                (*__tekai_state_generated_backend_mapfile).ps_tree,
                                fm as *mut ::core::ffi::c_void,
                            );
                            if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                                __assert_rtn(
                                    b"avl_do_entry\0" as *const u8 as *const ::core::ffi::c_char,
                                    b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                                    250 as ::core::ffi::c_int,
                                    b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                                );
                            } else {
                            };
                            (*fm).links = ((*fm).links as ::core::ffi::c_int | LINK_PS)
                                as ::core::ffi::c_ushort;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if (*fm).links as ::core::ffi::c_int & LINK_TFM == 0
        && (*fm).links as ::core::ffi::c_int & LINK_PS == 0
    {
        return 1 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
unsafe fn add_encname(mut s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let __tekai_state_generated_backend_mapfile: *mut crate::generated::backend::mapfile::State =
        unsafe { crate::generated::backend::mapfile::state() };

    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    if s.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"add_encname\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            267 as ::core::ffi::c_int,
            b"s != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if (*__tekai_state_generated_backend_mapfile)
        .encname_tree
        .is_null() as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"add_encname\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            268 as ::core::ffi::c_int,
            b"encname_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    p = avl_find(
        (*__tekai_state_generated_backend_mapfile).encname_tree,
        s as *const ::core::ffi::c_void,
    ) as *mut ::core::ffi::c_char;
    if p.is_null() {
        p = xstrdup(s as const_string) as *mut ::core::ffi::c_char;
        aa = avl_probe(
            (*__tekai_state_generated_backend_mapfile).encname_tree,
            p as *mut ::core::ffi::c_void,
        );
        if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"add_encname\0" as *const u8 as *const ::core::ffi::c_char,
                b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                272 as ::core::ffi::c_int,
                b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    return p;
}
unsafe fn check_fm_entry(mut fm: *mut fm_entry, mut warn: boolean) -> ::core::ffi::c_int {
    let mut a: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if fm.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"check_fm_entry\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            285 as ::core::ffi::c_int,
            b"fm != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if (*fm).tfm_name.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"check_fm_entry\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            286 as ::core::ffi::c_int,
            b"fm->tfm_name\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !(*fm).ff_name.is_null()
        && !((*fm).type_0 as ::core::ffi::c_int & F_INCLUDED != 0 as ::core::ffi::c_int)
    {
        if warn != 0 {
            crate::utils::pdftex_warn_args(b"ambiguous entry for `%s': font file present but not included, will be treated as font file not present\0"
                    as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from((*fm).tfm_name)]);
        }
        if !(*fm).ff_name.is_null() {
            free((*fm).ff_name as *mut ::core::ffi::c_void);
        }
        (*fm).ff_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if *(*fm).tfm_name as ::core::ffi::c_int == '\0' as i32 {
        if warn != 0 {
            crate::utils::pdftex_warn_args(
                b"invalid map entry: tfm missing\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        a += 1 as ::core::ffi::c_int;
    }
    if (*fm).type_0 as ::core::ffi::c_int & F_TRUETYPE != 0 as ::core::ffi::c_int
        && !(*fm).encname.is_null()
        && !((*fm).type_0 as ::core::ffi::c_int & F_SUBSETTED != 0 as ::core::ffi::c_int)
    {
        if warn != 0 {
            crate::utils::pdftex_warn_args(
                b"invalid entry for `%s': only subsetted TrueType fonts can be reencoded\0"
                    as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from((*fm).tfm_name)],
            );
        }
        a += 2 as ::core::ffi::c_int;
    }
    if ((*fm).slant != 0 as ::core::ffi::c_int || (*fm).extend != 0 as ::core::ffi::c_int)
        && (strlen((*fm).tfm_name) == 0 as size_t
            || !(!(*fm).ff_name.is_null()
                && (*fm).type_0 as ::core::ffi::c_int & F_TYPE1 != 0 as ::core::ffi::c_int
                && (*fm).type_0 as ::core::ffi::c_int & F_INCLUDED != 0 as ::core::ffi::c_int))
    {
        if warn != 0 {
            crate::utils::pdftex_warn_args(b"invalid entry for `%s': SlantFont/ExtendFont can be used only with embedded Type1 fonts\0"
                    as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from((*fm).tfm_name)]);
        }
        a += 4 as ::core::ffi::c_int;
    }
    if (if (*fm).slant >= 0 as ::core::ffi::c_int {
        (*fm).slant
    } else {
        -(*fm).slant
    }) > 1000 as ::core::ffi::c_int
    {
        if warn != 0 {
            crate::utils::pdftex_warn_args(
                b"invalid entry for `%s': SlantFont value too big: %g\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from((*fm).tfm_name),
                    crate::utils::PrintfArg::from((*fm).slant as ::core::ffi::c_double / 1000.0f64),
                ],
            );
        }
        a += 8 as ::core::ffi::c_int;
    }
    if (if (*fm).extend >= 0 as ::core::ffi::c_int {
        (*fm).extend
    } else {
        -(*fm).extend
    }) > 2000 as ::core::ffi::c_int
    {
        if warn != 0 {
            crate::utils::pdftex_warn_args(
                b"invalid entry for `%s': ExtendFont value too big: %g\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from((*fm).tfm_name),
                    crate::utils::PrintfArg::from(
                        (*fm).extend as ::core::ffi::c_double / 1000.0f64,
                    ),
                ],
            );
        }
        a += 16 as ::core::ffi::c_int;
    }
    if (*fm).pid as ::core::ffi::c_int != -(1 as ::core::ffi::c_int)
        && !((*fm).type_0 as ::core::ffi::c_int & F_TRUETYPE != 0 as ::core::ffi::c_int
            && (*fm).type_0 as ::core::ffi::c_int & F_SUBSETTED != 0 as ::core::ffi::c_int
            && (*fm).encname.is_null())
    {
        if warn != 0 {
            crate::utils::pdftex_warn_args(b"invalid entry for `%s': PidEid can be used only with subsetted non-reencoded TrueType fonts\0"
                    as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from((*fm).tfm_name)]);
        }
        a += 32 as ::core::ffi::c_int;
    }
    if !(*fm).ff_name.is_null()
        && (*fm).type_0 as ::core::ffi::c_int & F_PK != 0 as ::core::ffi::c_int
    {
        if warn != 0 {
            crate::utils::pdftex_warn_args(
                b"invalid entry for `%s': FontFile cannot be specified for bitmap PK font: %s\0"
                    as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from((*fm).tfm_name),
                    crate::utils::PrintfArg::from((*fm).ff_name),
                ],
            );
        }
        a += 64 as ::core::ffi::c_int;
    }
    if !(*fm).ps_name.is_null()
        && (*fm).type_0 as ::core::ffi::c_int & F_PK != 0 as ::core::ffi::c_int
    {
        if warn != 0 {
            crate::utils::pdftex_warn_args(
                b"invalid entry for `%s': PsName cannot be specified for bitmap PK font: %s\0"
                    as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from((*fm).tfm_name),
                    crate::utils::PrintfArg::from((*fm).ps_name),
                ],
            );
        }
        a += 128 as ::core::ffi::c_int;
    }
    return a;
}
pub unsafe fn check_std_t1font(mut s: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let __tekai_state_generated_backend_mapfile: *mut crate::generated::backend::mapfile::State =
        unsafe { crate::generated::backend::mapfile::state() };

    let mut n: size_t = 0;
    let mut k: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if s.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"check_std_t1font\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            401 as ::core::ffi::c_int,
            b"s != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    n = strlen(s);
    if n > 21 as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    if n == 12 as size_t {
        match *s as ::core::ffi::c_int {
            67 => {
                k = 1 as ::core::ffi::c_int;
            }
            84 => {
                k = 11 as ::core::ffi::c_int;
            }
            90 => {
                k = 13 as ::core::ffi::c_int;
            }
            _ => return -(1 as ::core::ffi::c_int),
        }
    } else {
        k = (*__tekai_state_generated_backend_mapfile).local_check_std_t1font_882_index[n as usize];
    }
    if k > -(1 as ::core::ffi::c_int)
        && strcmp(
            (*__tekai_state_generated_backend_mapfile).local_check_std_t1font_866_std_t1font_names
                [k as usize],
            s,
        ) == 0 as ::core::ffi::c_int
    {
        return k;
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe fn fm_scan_line() {
    let __tekai_state_generated_backend_mapfile: *mut crate::generated::backend::mapfile::State =
        unsafe { crate::generated::backend::mapfile::state() };

    let mut current_block: u64;
    let mut a: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut u: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut d: ::core::ffi::c_float = 0.;
    let mut fm: *mut fm_entry = ::core::ptr::null_mut::<fm_entry>();
    let mut fm_line: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut initialized_line_end: Option<*mut ::core::ffi::c_char> = None;
    match (*(*__tekai_state_generated_backend_mapfile).mitem).type_0 as ::core::ffi::c_uint {
        0 => {
            p = &raw mut fm_line as *mut ::core::ffi::c_char;
            loop {
                c = xgetc((*__tekai_state_generated_backend_mapfile).fm_file);
                if c == 9 as ::core::ffi::c_int {
                    c = 32 as ::core::ffi::c_int;
                }
                if c == 13 as ::core::ffi::c_int || c == EOF {
                    c = 10 as ::core::ffi::c_int;
                }
                if c != ' ' as i32
                    || p > &raw mut fm_line as *mut ::core::ffi::c_char
                        && *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                            != 32 as ::core::ffi::c_int
                {
                    if (p.offset_from(&raw mut fm_line as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                        > 1024 as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        crate::utils::pdftex_fail_args(
                            b"buffer overflow at file %s, line %d\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[
                                crate::utils::PrintfArg::from(
                                    b"tekai-engine/generated/backend/mapfile.rs\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                ),
                                crate::utils::PrintfArg::from(440 as ::core::ffi::c_int),
                            ],
                        );
                    }
                    let fresh0 = p;
                    p = p.offset(1);
                    *fresh0 = c as ::core::ffi::c_char;
                }
                if !(c != 10 as ::core::ffi::c_int) {
                    break;
                }
            }
            p = p.offset(-1);
            *p = '\0' as i32 as ::core::ffi::c_char;
            r = &raw mut fm_line as *mut ::core::ffi::c_char;
        }
        1 => {
            r = (*(*__tekai_state_generated_backend_mapfile).mitem).line;
        }
        _ => {
            if (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                __assert_rtn(
                    b"fm_scan_line\0" as *const u8 as *const ::core::ffi::c_char,
                    b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                    450 as ::core::ffi::c_int,
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
        }
    }
    if *r as ::core::ffi::c_int == '\0' as i32
        || (*r as ::core::ffi::c_int == 10 as ::core::ffi::c_int
            || *r as ::core::ffi::c_int == '*' as i32
            || *r as ::core::ffi::c_int == '#' as i32
            || *r as ::core::ffi::c_int == ';' as i32
            || *r as ::core::ffi::c_int == '%' as i32)
    {
        return;
    }
    fm = new_fm_entry();
    q = &raw mut buf as *mut ::core::ffi::c_char;
    while *r as ::core::ffi::c_int != ' ' as i32
        && *r as ::core::ffi::c_int != '<' as i32
        && *r as ::core::ffi::c_int != '"' as i32
        && *r as ::core::ffi::c_int != '\0' as i32
    {
        if (q.offset_from(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
            > 1024 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::utils::pdftex_fail_args(
                b"buffer overflow at file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(
                        b"tekai-engine/generated/backend/mapfile.rs\0" as *const u8
                            as *const ::core::ffi::c_char,
                    ),
                    crate::utils::PrintfArg::from(455 as ::core::ffi::c_int),
                ],
            );
        }
        let fresh1 = r;
        r = r.offset(1);
        let fresh2 = q;
        q = q.offset(1);
        *fresh2 = *fresh1;
    }
    *q = '\0' as i32 as ::core::ffi::c_char;
    if *r as ::core::ffi::c_int == ' ' as i32 {
        r = r.offset(1);
    }
    if q > &raw mut buf as *mut ::core::ffi::c_char {
        (*fm).tfm_name = xstrdup(&raw mut buf as *mut ::core::ffi::c_char as const_string)
            as *mut ::core::ffi::c_char;
    }
    if *r as ::core::ffi::c_int == '\0' as i32 {
        current_block = 7829721040285479420;
    } else {
        if (*fm).tfm_name.is_null() {
            (*fm).tfm_name =
                xstrdup(b"\0" as *const u8 as const_string) as *mut ::core::ffi::c_char;
        }
        if isdigit(*r as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0 {
            q = &raw mut buf as *mut ::core::ffi::c_char;
            while *r as ::core::ffi::c_int != ' ' as i32
                && *r as ::core::ffi::c_int != '<' as i32
                && *r as ::core::ffi::c_int != '"' as i32
                && *r as ::core::ffi::c_int != '\0' as i32
            {
                if (q.offset_from(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                    > 1024 as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    crate::utils::pdftex_fail_args(
                        b"buffer overflow at file %s, line %d\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[
                            crate::utils::PrintfArg::from(
                                b"tekai-engine/generated/backend/mapfile.rs\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            ),
                            crate::utils::PrintfArg::from(460 as ::core::ffi::c_int),
                        ],
                    );
                }
                let fresh3 = r;
                r = r.offset(1);
                let fresh4 = q;
                q = q.offset(1);
                *fresh4 = *fresh3;
            }
            *q = '\0' as i32 as ::core::ffi::c_char;
            if *r as ::core::ffi::c_int == ' ' as i32 {
                r = r.offset(1);
            }
            if q > &raw mut buf as *mut ::core::ffi::c_char {
                (*fm).ps_name = xstrdup(&raw mut buf as *mut ::core::ffi::c_char as const_string)
                    as *mut ::core::ffi::c_char;
            }
            if *r as ::core::ffi::c_int == '\0' as i32 {
                current_block = 7829721040285479420;
            } else {
                current_block = 5141539773904409130;
            }
        } else {
            current_block = 5141539773904409130;
        }
        match current_block {
            7829721040285479420 => {}
            _ => {
                if isdigit(*r as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
                    s = r;
                    while isdigit(*s as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
                        s = s.offset(1);
                    }
                    if *s as ::core::ffi::c_int == ' ' as i32
                        || *s as ::core::ffi::c_int == '"' as i32
                        || *s as ::core::ffi::c_int == '<' as i32
                        || *s as ::core::ffi::c_int == '\0' as i32
                    {
                        (*fm).fd_flags =
                            decimal_i32_legacy(CStr::from_ptr(r).to_bytes()).value as integer;
                        while isdigit(*r as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
                            r = r.offset(1);
                        }
                    }
                }
                loop {
                    if *r as ::core::ffi::c_int == ' ' as i32 {
                        r = r.offset(1);
                    }
                    match *r as ::core::ffi::c_int {
                        0 => {
                            current_block = 7829721040285479420;
                            break;
                        }
                        34 => {
                            r = r.offset(1);
                            v = 0 as ::core::ffi::c_int;
                            u = v;
                            loop {
                                if *r as ::core::ffi::c_int == ' ' as i32 {
                                    r = r.offset(1);
                                }
                                let end = initialized_line_end.get_or_insert_with(|| {
                                    r.add(CStr::from_ptr(r).to_bytes().len())
                                });
                                let remaining =
                                    end.addr().checked_sub(r.addr()).unwrap_or_else(|| {
                                        crate::utils::pdftex_fail_args(
                                            c"invalid map line cursor".as_ptr(),
                                            &[],
                                        )
                                    });
                                let field = std::slice::from_raw_parts(r.cast::<u8>(), remaining);
                                let operand = map_operand(field);
                                if matches!(
                                    operand,
                                    Err(FloatError::NonFinite | FloatError::Overflow)
                                ) {
                                    crate::utils::pdftex_warn_args(
                                        c"invalid entry for `%s': floating field out of range"
                                            .as_ptr(),
                                        &[crate::utils::PrintfArg::from((*fm).tfm_name)],
                                    );
                                    delete_fm_entry(fm);
                                    return;
                                }
                                if let Ok(operand) = operand {
                                    d = operand.value;
                                    s = r.add(operand.consumed);
                                    if *s.offset(-(1 as ::core::ffi::c_int as isize))
                                        as ::core::ffi::c_int
                                        == 'E' as i32
                                        || *s.offset(-(1 as ::core::ffi::c_int as isize))
                                            as ::core::ffi::c_int
                                            == 'e' as i32
                                    {
                                        s = s.offset(-1);
                                    }
                                    if strncmp(
                                        s,
                                        b"SlantFont\0" as *const u8 as *const ::core::ffi::c_char,
                                        9 as size_t,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        let Ok(slant) = scaled_map_operand(d) else {
                                            crate::utils::pdftex_warn_args(
                                                c"invalid entry for `%s': SlantFont value out of range".as_ptr(),
                                                &[crate::utils::PrintfArg::from((*fm).tfm_name)],
                                            );
                                            delete_fm_entry(fm);
                                            return;
                                        };
                                        (*fm).slant = slant;
                                        r = s.offset(9 as isize);
                                    } else if strncmp(
                                        s,
                                        b"ExtendFont\0" as *const u8 as *const ::core::ffi::c_char,
                                        10 as size_t,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        let Ok(extend) = scaled_map_operand(d) else {
                                            crate::utils::pdftex_warn_args(
                                                c"invalid entry for `%s': ExtendFont value out of range".as_ptr(),
                                                &[crate::utils::PrintfArg::from((*fm).tfm_name)],
                                            );
                                            delete_fm_entry(fm);
                                            return;
                                        };
                                        (*fm).extend = extend;
                                        if (*fm).extend == 1000 as ::core::ffi::c_int {
                                            (*fm).extend = 0 as ::core::ffi::c_int as integer;
                                        }
                                        r = s.offset(10 as isize);
                                    } else {
                                        r = s;
                                        while *r as ::core::ffi::c_int != ' ' as i32
                                            && *r as ::core::ffi::c_int != '"' as i32
                                            && *r as ::core::ffi::c_int != '\0' as i32
                                        {
                                            r = r.offset(1);
                                        }
                                        c = *r as ::core::ffi::c_int;
                                        *r = '\0' as i32 as ::core::ffi::c_char;
                                        crate::utils::pdftex_warn_args(
                                            b"invalid entry for `%s': unknown name `%s' ignored\0"
                                                as *const u8
                                                as *const ::core::ffi::c_char,
                                            &[
                                                crate::utils::PrintfArg::from((*fm).tfm_name),
                                                crate::utils::PrintfArg::from(s),
                                            ],
                                        );
                                        *r = c as ::core::ffi::c_char;
                                    }
                                } else {
                                    while *r as ::core::ffi::c_int != ' ' as i32
                                        && *r as ::core::ffi::c_int != '"' as i32
                                        && *r as ::core::ffi::c_int != '\0' as i32
                                    {
                                        r = r.offset(1);
                                    }
                                }
                                if !(*r as ::core::ffi::c_int == ' ' as i32) {
                                    break;
                                }
                            }
                            if *r as ::core::ffi::c_int == '"' as i32 {
                                r = r.offset(1);
                                continue;
                            } else {
                                crate::utils::pdftex_warn_args(
                                    b"invalid entry for `%s': closing quote missing\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    &[crate::utils::PrintfArg::from((*fm).tfm_name)],
                                );
                                current_block = 17162251379607524254;
                                break;
                            }
                        }
                        80 => {
                            // The input line stays fixed for this scan. Find its
                            // terminator once, even for repeated PidEid fields.
                            let end = initialized_line_end
                                .get_or_insert_with(|| r.add(CStr::from_ptr(r).to_bytes().len()));
                            let remaining = end.addr().checked_sub(r.addr()).unwrap_or_else(|| {
                                crate::utils::pdftex_fail_args(
                                    c"invalid map line cursor".as_ptr(),
                                    &[],
                                )
                            });
                            let bytes = std::slice::from_raw_parts(r.cast::<u8>(), remaining);
                            match platform_encoding(bytes) {
                                Ok(ids) => {
                                    (*fm).pid = ids.platform;
                                    (*fm).eid = ids.encoding;
                                    r = r.add(ids.consumed);
                                    continue;
                                }
                                Err(FieldError::Overflow) => {
                                    crate::utils::pdftex_warn_args(
                                        b"invalid entry for `%s': PidEid value out of range\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                        &[crate::utils::PrintfArg::from((*fm).tfm_name)],
                                    );
                                    current_block = 17162251379607524254;
                                    break;
                                }
                                Err(FieldError::NoMatch) => {}
                            }
                        }
                        _ => {}
                    }
                    b = 0 as ::core::ffi::c_int;
                    a = b;
                    if *r as ::core::ffi::c_int == '<' as i32 {
                        let fresh5 = r;
                        r = r.offset(1);
                        a = *fresh5 as ::core::ffi::c_int;
                        if *r as ::core::ffi::c_int == '<' as i32
                            || *r as ::core::ffi::c_int == '[' as i32
                        {
                            let fresh6 = r;
                            r = r.offset(1);
                            b = *fresh6 as ::core::ffi::c_int;
                        }
                    }
                    q = &raw mut buf as *mut ::core::ffi::c_char;
                    while *r as ::core::ffi::c_int != ' ' as i32
                        && *r as ::core::ffi::c_int != '<' as i32
                        && *r as ::core::ffi::c_int != '"' as i32
                        && *r as ::core::ffi::c_int != '\0' as i32
                    {
                        if (q.offset_from(&raw mut buf as *mut ::core::ffi::c_char)
                            as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long)
                            as ::core::ffi::c_uint
                            > 1024 as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            crate::utils::pdftex_fail_args(
                                b"buffer overflow at file %s, line %d\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[
                                    crate::utils::PrintfArg::from(
                                        b"tekai-engine/generated/backend/mapfile.rs\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    ),
                                    crate::utils::PrintfArg::from(531 as ::core::ffi::c_int),
                                ],
                            );
                        }
                        let fresh7 = r;
                        r = r.offset(1);
                        let fresh8 = q;
                        q = q.offset(1);
                        *fresh8 = *fresh7;
                    }
                    *q = '\0' as i32 as ::core::ffi::c_char;
                    if *r as ::core::ffi::c_int == ' ' as i32 {
                        r = r.offset(1);
                    }
                    let buf_len = q.offset_from(&raw mut buf as *mut ::core::ffi::c_char) as size_t;
                    if buf_len > 4 as size_t
                        && strcasecmp(
                            (&raw mut buf as *mut ::core::ffi::c_char)
                                .offset(buf_len as isize - 4 as isize),
                            b".enc\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        (*fm).encname = add_encname(&raw mut buf as *mut ::core::ffi::c_char);
                        v = 0 as ::core::ffi::c_int;
                        u = v;
                    } else if buf_len > 0 as size_t {
                        if a == '<' as i32 || u == '<' as i32 {
                            (*fm).type_0 = ((*fm).type_0 as ::core::ffi::c_int | F_INCLUDED)
                                as ::core::ffi::c_ushort;
                            if a == '<' as i32 && b == 0 as ::core::ffi::c_int
                                || a == 0 as ::core::ffi::c_int && v == 0 as ::core::ffi::c_int
                            {
                                (*fm).type_0 = ((*fm).type_0 as ::core::ffi::c_int | F_SUBSETTED)
                                    as ::core::ffi::c_ushort;
                            }
                        }
                        if q > &raw mut buf as *mut ::core::ffi::c_char {
                            (*fm).ff_name =
                                xstrdup(&raw mut buf as *mut ::core::ffi::c_char as const_string)
                                    as *mut ::core::ffi::c_char;
                        }
                        if *r as ::core::ffi::c_int == '\0' as i32 {
                            current_block = 7829721040285479420;
                            break;
                        }
                        v = 0 as ::core::ffi::c_int;
                        u = v;
                    } else {
                        u = a;
                        v = b;
                    }
                }
            }
        }
    }
    match current_block {
        7829721040285479420 => {
            if !(*fm).ps_name.is_null()
                && check_std_t1font((*fm).ps_name) >= 0 as ::core::ffi::c_int
            {
                (*fm).type_0 =
                    ((*fm).type_0 as ::core::ffi::c_int | F_STDT1FONT) as ::core::ffi::c_ushort;
            }
            if !(*fm).ff_name.is_null() {
                let ff_name_len = strlen((*fm).ff_name);
                if ff_name_len > 3 as size_t {
                    let ff_suffix = (*fm).ff_name.offset(ff_name_len as isize - 4 as isize);
                    if strcasecmp(
                        ff_suffix,
                        b".ttf\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*fm).type_0 = ((*fm).type_0 as ::core::ffi::c_int | F_TRUETYPE)
                            as ::core::ffi::c_ushort;
                    } else if strcasecmp(
                        ff_suffix,
                        b".ttc\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*fm).type_0 = ((*fm).type_0 as ::core::ffi::c_int | F_TRUETYPE)
                            as ::core::ffi::c_ushort;
                    } else if strcasecmp(
                        ff_suffix,
                        b".otf\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*fm).type_0 =
                            ((*fm).type_0 as ::core::ffi::c_int | F_OTF) as ::core::ffi::c_ushort;
                    } else {
                        (*fm).type_0 =
                            ((*fm).type_0 as ::core::ffi::c_int | F_TYPE1) as ::core::ffi::c_ushort;
                    }
                } else if (*fm).ps_name.is_null() {
                    (*fm).type_0 =
                        ((*fm).type_0 as ::core::ffi::c_int | F_PK) as ::core::ffi::c_ushort;
                } else {
                    (*fm).type_0 =
                        ((*fm).type_0 as ::core::ffi::c_int | F_TYPE1) as ::core::ffi::c_ushort;
                }
            } else if (*fm).ps_name.is_null() {
                (*fm).type_0 = ((*fm).type_0 as ::core::ffi::c_int | F_PK) as ::core::ffi::c_ushort;
            } else {
                (*fm).type_0 =
                    ((*fm).type_0 as ::core::ffi::c_int | F_TYPE1) as ::core::ffi::c_ushort;
            }
            if !(check_fm_entry(fm, true_0) != 0 as ::core::ffi::c_int) {
                if handle_subfont_fm(
                    fm,
                    (*(*__tekai_state_generated_backend_mapfile).mitem).mode as ::core::ffi::c_int,
                ) != 0
                {
                    return;
                }
                if avl_do_entry(
                    fm,
                    (*(*__tekai_state_generated_backend_mapfile).mitem).mode as ::core::ffi::c_int,
                ) == 0 as ::core::ffi::c_int
                {
                    return;
                }
            }
        }
        _ => {}
    }
    delete_fm_entry(fm);
}
pub unsafe fn fm_read_info() {
    let __tekai_state_generated_backend_mapfile: *mut crate::generated::backend::mapfile::State =
        unsafe { crate::generated::backend::mapfile::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    if (*__tekai_state_generated_backend_mapfile)
        .tfm_tree
        .is_null()
    {
        create_avl_trees();
    }
    if (*(*__tekai_state_generated_backend_mapfile).mitem)
        .line
        .is_null()
    {
        return;
    }
    (*(*__tekai_state_generated_backend_mapfile).mitem).lineno = 1 as ::core::ffi::c_int;
    match (*(*__tekai_state_generated_backend_mapfile).mitem).type_0 as ::core::ffi::c_uint {
        0 => {
            (*__tekai_state_utils).cur_file_name =
                (*(*__tekai_state_generated_backend_mapfile).mitem).line;
            zpackfilename(
                maketexstring((*__tekai_state_utils).cur_file_name),
                getnullstr(),
                getnullstr(),
            );
            if open_input(
                &raw mut (*__tekai_state_generated_backend_mapfile).fm_file,
                kpse_fontmap_format as ::core::ffi::c_int,
                FOPEN_RBIN_MODE.as_ptr(),
            ) == 0
            {
                crate::utils::pdftex_warn_args(
                    b"cannot open font map file\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            } else {
                (*__tekai_state_utils).cur_file_name =
                    ((*__tekai_state_generated_pdftexextra).nameoffile as *mut ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize);
                crate::utils::tex_printf_args(
                    b"{%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(
                        (*__tekai_state_utils).cur_file_name,
                    )],
                );
                while feof((*__tekai_state_generated_backend_mapfile).fm_file) == 0 {
                    fm_scan_line();
                    (*(*__tekai_state_generated_backend_mapfile).mitem).lineno += 1;
                }
                xfclose(
                    (*__tekai_state_generated_backend_mapfile).fm_file,
                    (*__tekai_state_utils).cur_file_name as const_string,
                );
                crate::utils::tex_printf_args(
                    b"}\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
                (*__tekai_state_generated_backend_mapfile).fm_file =
                    ::core::ptr::null_mut::<FILE>();
            }
        }
        1 => {
            (*__tekai_state_utils).cur_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
            fm_scan_line();
        }
        _ => {
            if (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                __assert_rtn(
                    b"fm_read_info\0" as *const u8 as *const ::core::ffi::c_char,
                    b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                    620 as ::core::ffi::c_int,
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
        }
    }
    (*(*__tekai_state_generated_backend_mapfile).mitem).line =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*__tekai_state_utils).cur_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe fn fmlookup(mut f: internalfontnumber) -> fmentryptr {
    let __tekai_state_generated_backend_mapfile: *mut crate::generated::backend::mapfile::State =
        unsafe { crate::generated::backend::mapfile::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut tfm: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fm: *mut fm_entry = ::core::ptr::null_mut::<fm_entry>();
    let mut tmp: fm_entry = fm_entry {
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
    if (*__tekai_state_generated_backend_mapfile)
        .tfm_tree
        .is_null()
    {
        fm_read_info();
    }
    tfm = makecstring(
        *(*__tekai_state_generated_pdftexextra)
            .fontname
            .offset(f as isize) as integer,
    );
    if !(strcmp(
        tfm,
        &raw const (*__tekai_state_generated_backend_mapfile).nontfm as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"fmlookup\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            637 as ::core::ffi::c_int,
            b"strcmp(tfm, nontfm) != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    tmp.tfm_name = tfm;
    fm = avl_find(
        (*__tekai_state_generated_backend_mapfile).tfm_tree,
        &raw mut tmp as *const ::core::ffi::c_void,
    ) as *mut fm_entry;
    if !fm.is_null() {
        (*fm).in_use = true_0 as boolean;
        return fm as fmentryptr;
    }
    return dummy_fm_entry() as fmentryptr;
}
pub unsafe fn hasfmentry(mut f: internalfontnumber) -> boolean {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    if (*(*__tekai_state_generated_pdftexextra)
        .pdffontmap
        .offset(f as isize))
    .is_null()
    {
        let ref mut fresh9 = *(*__tekai_state_generated_pdftexextra)
            .pdffontmap
            .offset(f as isize);
        *fresh9 = fmlookup(f);
    }
    if (*(*__tekai_state_generated_pdftexextra)
        .pdffontmap
        .offset(f as isize))
    .is_null() as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"hasfmentry\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            653 as ::core::ffi::c_int,
            b"pdffontmap[f] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    return (*(*__tekai_state_generated_pdftexextra)
        .pdffontmap
        .offset(f as isize)
        != dummy_fm_entry() as fmentryptr) as ::core::ffi::c_int;
}
pub unsafe fn isscalable(mut f: internalfontnumber) -> boolean {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    return (hasfmentry(f) != 0
        && !((*(*(*__tekai_state_generated_pdftexextra)
            .pdffontmap
            .offset(f as isize) as *mut fm_entry))
            .type_0 as ::core::ffi::c_int
            & F_PK
            != 0 as ::core::ffi::c_int)) as ::core::ffi::c_int;
}
pub unsafe fn hasspacechar(mut f: internalfontnumber) -> boolean {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut fm: *mut fm_entry = ::core::ptr::null_mut::<fm_entry>();
    let mut fe: *mut fe_entry = ::core::ptr::null_mut::<fe_entry>();
    if isscalable(f) == 0 {
        return false_0;
    }
    fm = *(*__tekai_state_generated_pdftexextra)
        .pdffontmap
        .offset(f as isize) as *mut fm_entry;
    if !(*fm).encname.is_null() && {
        fe = get_fe_entry((*fm).encname);
        !fe.is_null()
    } {
        let mut s: *mut ::core::ffi::c_char =
            *(*fe).glyph_names.offset(32 as ::core::ffi::c_int as isize);
        if s.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"hasspacechar\0" as *const u8 as *const ::core::ffi::c_char,
                b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                674 as ::core::ffi::c_int,
                b"s != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        if strcmp(s, b"space\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            return true_0;
        }
    }
    return false_0;
}
unsafe fn fm_valid_for_font_replacement(mut fm: *mut fm_entry) -> boolean {
    let mut ff: *mut ff_entry = ::core::ptr::null_mut::<ff_entry>();
    if fm.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"fm_valid_for_font_replacement\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            687 as ::core::ffi::c_int,
            b"fm != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if (*fm).ff_name.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"fm_valid_for_font_replacement\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            688 as ::core::ffi::c_int,
            b"is_fontfile(fm)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !((*fm).type_0 as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"fm_valid_for_font_replacement\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            689 as ::core::ffi::c_int,
            b"is_type1(fm)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    ff = check_ff_exist((*fm).ff_name, false_0);
    if ff.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"fm_valid_for_font_replacement\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            692 as ::core::ffi::c_int,
            b"ff != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if (*ff).ff_path.is_null() {
        return false_0;
    }
    return true_0;
}
pub unsafe fn lookup_fontmap(mut ps_name: *mut ::core::ffi::c_char) -> *mut fm_entry {
    let __tekai_state_generated_backend_mapfile: *mut crate::generated::backend::mapfile::State =
        unsafe { crate::generated::backend::mapfile::state() };

    let mut fm: *mut fm_entry = ::core::ptr::null_mut::<fm_entry>();
    let mut fm2: *mut fm_entry = ::core::ptr::null_mut::<fm_entry>();
    let mut tmp: fm_entry = fm_entry {
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
    let mut a: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut b: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut c: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut d: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut e: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut sl: ::core::ffi::c_int = 0;
    let mut ex: ::core::ffi::c_int = 0;
    let mut t: avl_traverser = avl_traverser {
        avl_table: ::core::ptr::null_mut::<avl_table>(),
        avl_node: ::core::ptr::null_mut::<avl_node>(),
        avl_stack: [::core::ptr::null_mut::<avl_node>(); 32],
        avl_height: 0,
        avl_generation: 0,
    };
    let mut t2: avl_traverser = avl_traverser {
        avl_table: ::core::ptr::null_mut::<avl_table>(),
        avl_node: ::core::ptr::null_mut::<avl_node>(),
        avl_stack: [::core::ptr::null_mut::<avl_node>(); 32],
        avl_height: 0,
        avl_generation: 0,
    };
    if (*__tekai_state_generated_backend_mapfile)
        .tfm_tree
        .is_null()
    {
        fm_read_info();
    }
    if ps_name.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"lookup_fontmap\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            712 as ::core::ffi::c_int,
            b"ps_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    s = ps_name;
    if strlen(ps_name) > 7 as size_t {
        i = 0 as ::core::ffi::c_int;
        while i < 6 as ::core::ffi::c_int {
            if (*s as ::core::ffi::c_int) < 'A' as i32 || *s as ::core::ffi::c_int > 'Z' as i32 {
                break;
            }
            i += 1;
            s = s.offset(1);
        }
        if i == 6 as ::core::ffi::c_int && *s as ::core::ffi::c_int == '+' as i32 {
            s = s.offset(1);
        } else {
            s = ps_name;
        }
    }
    tmp.slant = 0 as ::core::ffi::c_int as integer;
    tmp.extend = 0 as ::core::ffi::c_int as integer;
    a = strstr(s, b"-Slant_\0" as *const u8 as *const ::core::ffi::c_char);
    if !a.is_null() {
        b = a.offset(strlen(b"-Slant_\0" as *const u8 as *const ::core::ffi::c_char) as isize);
        let parsed = signed_prefix(CStr::from_ptr(b).to_bytes(), IntegerBase::Decimal);
        sl = parsed.value as ::core::ffi::c_int;
        e = b.add(parsed.consumed);
        if e != b && e == strchr(b, 0 as ::core::ffi::c_int) {
            tmp.slant = sl as integer;
            *a = '\0' as i32 as ::core::ffi::c_char;
        } else if e != b {
            c = strstr(e, b"-Extend_\0" as *const u8 as *const ::core::ffi::c_char);
            if !c.is_null() {
                d = c.offset(
                    strlen(b"-Extend_\0" as *const u8 as *const ::core::ffi::c_char) as isize,
                );
                let parsed = signed_prefix(CStr::from_ptr(d).to_bytes(), IntegerBase::Decimal);
                ex = parsed.value as ::core::ffi::c_int;
                e = d.add(parsed.consumed);
                if e != d && e == strchr(d, 0 as ::core::ffi::c_int) {
                    tmp.slant = sl as integer;
                    tmp.extend = ex as integer;
                    *a = '\0' as i32 as ::core::ffi::c_char;
                }
            }
        }
    } else {
        a = strstr(s, b"-Extend_\0" as *const u8 as *const ::core::ffi::c_char);
        if !a.is_null() {
            b = a.offset(strlen(b"-Extend_\0" as *const u8 as *const ::core::ffi::c_char) as isize);
            let parsed = signed_prefix(CStr::from_ptr(b).to_bytes(), IntegerBase::Decimal);
            ex = parsed.value as ::core::ffi::c_int;
            e = b.add(parsed.consumed);
            if e != b && e == strchr(b, 0 as ::core::ffi::c_int) {
                tmp.extend = ex as integer;
                *a = '\0' as i32 as ::core::ffi::c_char;
            }
        }
    }
    tmp.ps_name = s;
    fm = avl_t_find(
        &raw mut t,
        (*__tekai_state_generated_backend_mapfile).ps_tree,
        &raw mut tmp as *mut ::core::ffi::c_void,
    ) as *mut fm_entry;
    if fm.is_null() {
        return ::core::ptr::null_mut::<fm_entry>();
    }
    t2 = t;
    fm2 = avl_t_prev(&raw mut t2) as *mut fm_entry;
    loop {
        if fm_valid_for_font_replacement(fm) != 0 {
            return fm;
        }
        fm = avl_t_next(&raw mut t) as *mut fm_entry;
        if !(!fm.is_null()
            && comp_fm_entry_ps(
                fm as *const ::core::ffi::c_void,
                &raw mut tmp as *const ::core::ffi::c_void,
                NULL,
            ) == 0 as ::core::ffi::c_int)
        {
            break;
        }
    }
    while !fm2.is_null()
        && comp_fm_entry_ps(
            fm2 as *const ::core::ffi::c_void,
            &raw mut tmp as *const ::core::ffi::c_void,
            NULL,
        ) == 0 as ::core::ffi::c_int
    {
        if fm_valid_for_font_replacement(fm2) != 0 {
            return fm2;
        }
        fm2 = avl_t_prev(&raw mut t2) as *mut fm_entry;
    }
    return ::core::ptr::null_mut::<fm_entry>();
}
unsafe fn process_map_item(mut s: *mut ::core::ffi::c_char, mut type_0: ::core::ffi::c_int) {
    let __tekai_state_generated_backend_mapfile: *mut crate::generated::backend::mapfile::State =
        unsafe { crate::generated::backend::mapfile::state() };

    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut mode: ::core::ffi::c_int = 0;
    if *s as ::core::ffi::c_int == ' ' as i32 {
        s = s.offset(1);
    }
    match *s as ::core::ffi::c_int {
        43 => {
            mode = FM_DUPIGNORE as ::core::ffi::c_int;
            s = s.offset(1);
        }
        61 => {
            mode = FM_REPLACE as ::core::ffi::c_int;
            s = s.offset(1);
        }
        45 => {
            mode = FM_DELETE as ::core::ffi::c_int;
            s = s.offset(1);
        }
        _ => {
            mode = FM_DUPIGNORE as ::core::ffi::c_int;
            (*(*__tekai_state_generated_backend_mapfile).mitem).line =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    if *s as ::core::ffi::c_int == ' ' as i32 {
        s = s.offset(1);
    }
    p = s;
    match type_0 {
        0 => {
            while *p as ::core::ffi::c_int != '\0' as i32 && *p as ::core::ffi::c_int != ' ' as i32
            {
                p = p.offset(1);
            }
            *p = '\0' as i32 as ::core::ffi::c_char;
        }
        1 => {}
        _ => {
            if (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                __assert_rtn(
                    b"process_map_item\0" as *const u8 as *const ::core::ffi::c_char,
                    b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                    837 as ::core::ffi::c_int,
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
        }
    }
    if !(*(*__tekai_state_generated_backend_mapfile).mitem)
        .line
        .is_null()
    {
        fm_read_info();
    }
    if *s as ::core::ffi::c_int != '\0' as i32 {
        (*(*__tekai_state_generated_backend_mapfile).mitem).mode = mode as updatemode;
        (*(*__tekai_state_generated_backend_mapfile).mitem).type_0 = type_0 as maptype;
        (*(*__tekai_state_generated_backend_mapfile).mitem).line = s;
        fm_read_info();
    }
}
pub unsafe fn pdfmapfile(mut t: integer) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    process_map_item(
        makecstring(ztokenstostring(t) as integer),
        MAPFILE as ::core::ffi::c_int,
    );
    zflushstr((*__tekai_state_generated_pdftexextra).lasttokensstring);
}
pub unsafe fn pdfmapline(mut t: integer) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    process_map_item(
        makecstring(ztokenstostring(t) as integer),
        MAPLINE as ::core::ffi::c_int,
    );
    zflushstr((*__tekai_state_generated_pdftexextra).lasttokensstring);
}
pub unsafe fn pdfmaplinesp() {
    process_map_item(
        b"=pdftexspace PdfTeX-Space <pdftexspace.pfb\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        MAPLINE as ::core::ffi::c_int,
    );
}
pub unsafe fn pdfinitmapfile(mut map_name: const_string) {
    let __tekai_state_generated_backend_mapfile: *mut crate::generated::backend::mapfile::State =
        unsafe { crate::generated::backend::mapfile::state() };

    if !(*__tekai_state_generated_backend_mapfile).mitem.is_null() as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"pdfinitmapfile\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            868 as ::core::ffi::c_int,
            b"mitem == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*__tekai_state_generated_backend_mapfile).mitem =
        xmalloc((1 as size_t).wrapping_mul(::core::mem::size_of::<mapitem>() as size_t))
            as *mut mapitem;
    (*(*__tekai_state_generated_backend_mapfile).mitem).mode = FM_DUPIGNORE;
    (*(*__tekai_state_generated_backend_mapfile).mitem).type_0 = MAPFILE;
    (*(*__tekai_state_generated_backend_mapfile).mitem).line =
        xstrdup(map_name) as *mut ::core::ffi::c_char;
}
pub unsafe fn check_ff_exist(
    mut ff_name: *mut ::core::ffi::c_char,
    mut is_tt: boolean,
) -> *mut ff_entry {
    let __tekai_state_generated_backend_mapfile: *mut crate::generated::backend::mapfile::State =
        unsafe { crate::generated::backend::mapfile::state() };

    let mut ff: *mut ff_entry = ::core::ptr::null_mut::<ff_entry>();
    let mut tmp: ff_entry = ff_entry {
        ff_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ff_path: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    if ff_name.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"check_ff_exist\0" as *const u8 as *const ::core::ffi::c_char,
            b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            893 as ::core::ffi::c_int,
            b"ff_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    tmp.ff_name = ff_name;
    ff = avl_find(
        (*__tekai_state_generated_backend_mapfile).ff_tree,
        &raw mut tmp as *const ::core::ffi::c_void,
    ) as *mut ff_entry;
    if ff.is_null() {
        ff = new_ff_entry();
        (*ff).ff_name = xstrdup(ff_name as const_string) as *mut ::core::ffi::c_char;
        if is_tt != 0 {
            (*ff).ff_path =
                kpse_find_file(ff_name as const_string, kpse_truetype_format, 0 as boolean)
                    as *mut ::core::ffi::c_char;
        } else {
            (*ff).ff_path = kpse_find_file(ff_name as const_string, kpse_type1_format, 0 as boolean)
                as *mut ::core::ffi::c_char;
        }
        aa = avl_probe(
            (*__tekai_state_generated_backend_mapfile).ff_tree,
            ff as *mut ::core::ffi::c_void,
        );
        if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"check_ff_exist\0" as *const u8 as *const ::core::ffi::c_char,
                b"mapfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                904 as ::core::ffi::c_int,
                b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    return ff;
}
unsafe fn destroy_fm_entry_tfm(mut pa: *mut ::core::ffi::c_void, mut pb: *mut ::core::ffi::c_void) {
    let mut fm: *mut fm_entry = ::core::ptr::null_mut::<fm_entry>();
    fm = pa as *mut fm_entry;
    if (*fm).links as ::core::ffi::c_int & LINK_PS == 0 {
        delete_fm_entry(fm);
    } else {
        (*fm).links = ((*fm).links as ::core::ffi::c_int & !LINK_TFM) as ::core::ffi::c_ushort;
    };
}
unsafe fn destroy_fm_entry_ps(mut pa: *mut ::core::ffi::c_void, mut pb: *mut ::core::ffi::c_void) {
    let mut fm: *mut fm_entry = ::core::ptr::null_mut::<fm_entry>();
    fm = pa as *mut fm_entry;
    if (*fm).links as ::core::ffi::c_int & LINK_TFM == 0 {
        delete_fm_entry(fm);
    } else {
        (*fm).links = ((*fm).links as ::core::ffi::c_int & !LINK_PS) as ::core::ffi::c_ushort;
    };
}
unsafe fn destroy_ff_entry(mut pa: *mut ::core::ffi::c_void, mut pb: *mut ::core::ffi::c_void) {
    let mut ff: *mut ff_entry = ::core::ptr::null_mut::<ff_entry>();
    ff = pa as *mut ff_entry;
    delete_ff_entry(ff);
}
pub unsafe fn fm_free() {
    let __tekai_state_generated_backend_mapfile: *mut crate::generated::backend::mapfile::State =
        unsafe { crate::generated::backend::mapfile::state() };

    if !(*__tekai_state_generated_backend_mapfile)
        .tfm_tree
        .is_null()
    {
        avl_destroy(
            (*__tekai_state_generated_backend_mapfile).tfm_tree,
            Some(
                destroy_fm_entry_tfm
                    as unsafe fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
            ),
        );
        (*__tekai_state_generated_backend_mapfile).tfm_tree = ::core::ptr::null_mut::<avl_table>();
    }
    if !(*__tekai_state_generated_backend_mapfile).ps_tree.is_null() {
        avl_destroy(
            (*__tekai_state_generated_backend_mapfile).ps_tree,
            Some(
                destroy_fm_entry_ps
                    as unsafe fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
            ),
        );
        (*__tekai_state_generated_backend_mapfile).ps_tree = ::core::ptr::null_mut::<avl_table>();
    }
    if !(*__tekai_state_generated_backend_mapfile).ff_tree.is_null() {
        avl_destroy(
            (*__tekai_state_generated_backend_mapfile).ff_tree,
            Some(
                destroy_ff_entry
                    as unsafe fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
            ),
        );
        (*__tekai_state_generated_backend_mapfile).ff_tree = ::core::ptr::null_mut::<avl_table>();
    }
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;

pub(crate) struct State {
    fm_file: *mut FILE,
    mitem: *mut mapitem,
    nontfm: [::core::ffi::c_char; 9],
    local_dummy_fm_entry_228_const_fm_entry: fm_entry,
    tfm_tree: *mut avl_table,
    ps_tree: *mut avl_table,
    ff_tree: *mut avl_table,
    encname_tree: *mut avl_table,
    local_check_std_t1font_866_std_t1font_names: [*const ::core::ffi::c_char; 14],
    local_check_std_t1font_882_index: [::core::ffi::c_int; 22],
}

#[inline(always)]
fn __tekai_require_copy<T: Copy>() {}

pub(crate) struct StateInitialization {
    destination: *mut State,
    initialized: [bool; 10],
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
            if self.initialized[9] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).local_check_std_t1font_882_index
                ));
            }
            if self.initialized[8] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).local_check_std_t1font_866_std_t1font_names
                ));
            }
            if self.initialized[7] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).encname_tree));
            }
            if self.initialized[6] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).ff_tree));
            }
            if self.initialized[5] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).ps_tree));
            }
            if self.initialized[4] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).tfm_tree));
            }
            if self.initialized[3] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).local_dummy_fm_entry_228_const_fm_entry
                ));
            }
            if self.initialized[2] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).nontfm));
            }
            if self.initialized[1] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mitem));
            }
            if self.initialized[0] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fm_file));
            }
        }
    }
}
impl StateInitialization {
    pub(crate) unsafe fn begin(destination: *mut State) -> Self {
        Self {
            destination,
            initialized: [false; 10],
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
            assert!(core::mem::size_of::<*mut FILE>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fm_file)
            .write(::core::ptr::null::<FILE>() as *mut FILE);
        self.initialized[0] = true;
        const {
            assert!(core::mem::size_of::<*mut mapitem>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mitem)
            .write(::core::ptr::null::<mapitem>() as *mut mapitem);
        self.initialized[1] = true;
        const {
            assert!(core::mem::size_of::<[::core::ffi::c_char; 9]>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).nontfm).write(unsafe {
            ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"<nontfm>\0")
        });
        self.initialized[2] = true;
        const {
            assert!(core::mem::size_of::<fm_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_dummy_fm_entry_228_const_fm_entry).write(
            fm_entry {
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
            },
        );
        self.initialized[3] = true;
        const {
            assert!(core::mem::size_of::<*mut avl_table>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).tfm_tree)
            .write(::core::ptr::null::<avl_table>() as *mut avl_table);
        self.initialized[4] = true;
        const {
            assert!(core::mem::size_of::<*mut avl_table>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).ps_tree)
            .write(::core::ptr::null::<avl_table>() as *mut avl_table);
        self.initialized[5] = true;
        const {
            assert!(core::mem::size_of::<*mut avl_table>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).ff_tree)
            .write(::core::ptr::null::<avl_table>() as *mut avl_table);
        self.initialized[6] = true;
        const {
            assert!(core::mem::size_of::<*mut avl_table>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).encname_tree)
            .write(::core::ptr::null::<avl_table>() as *mut avl_table);
        self.initialized[7] = true;
        const {
            assert!((14) == 14);
        }
        __tekai_require_copy::<*const ::core::ffi::c_char>();
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_866_std_t1font_names[0])
            .write(b"Courier\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_866_std_t1font_names[1])
            .write(b"Courier-Bold\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_866_std_t1font_names[2])
            .write(b"Courier-Oblique\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_866_std_t1font_names[3])
            .write(b"Courier-BoldOblique\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_866_std_t1font_names[4])
            .write(b"Helvetica\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_866_std_t1font_names[5])
            .write(b"Helvetica-Bold\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_866_std_t1font_names[6])
            .write(b"Helvetica-Oblique\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_866_std_t1font_names[7])
            .write(b"Helvetica-BoldOblique\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_866_std_t1font_names[8])
            .write(b"Symbol\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_866_std_t1font_names[9])
            .write(b"Times-Roman\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_866_std_t1font_names[10])
            .write(b"Times-Bold\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_866_std_t1font_names[11])
            .write(b"Times-Italic\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_866_std_t1font_names[12])
            .write(b"Times-BoldItalic\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_866_std_t1font_names[13])
            .write(b"ZapfDingbats\0" as *const u8 as *const ::core::ffi::c_char);
        self.initialized[8] = true;
        const {
            assert!((22) == 22);
        }
        __tekai_require_copy::<::core::ffi::c_int>();
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[0])
            .write(-(1 as ::core::ffi::c_int));
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[1])
            .write(-(1 as ::core::ffi::c_int));
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[2])
            .write(-(1 as ::core::ffi::c_int));
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[3])
            .write(-(1 as ::core::ffi::c_int));
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[4])
            .write(-(1 as ::core::ffi::c_int));
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[5])
            .write(-(1 as ::core::ffi::c_int));
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[6])
            .write(8 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[7])
            .write(0 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[8])
            .write(-(1 as ::core::ffi::c_int));
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[9])
            .write(4 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[10])
            .write(10 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[11])
            .write(9 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[12])
            .write(-(1 as ::core::ffi::c_int));
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[13])
            .write(-(1 as ::core::ffi::c_int));
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[14])
            .write(5 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[15])
            .write(2 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[16])
            .write(12 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[17])
            .write(6 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[18])
            .write(-(1 as ::core::ffi::c_int));
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[19])
            .write(3 as ::core::ffi::c_int);
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[20])
            .write(-(1 as ::core::ffi::c_int));
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_check_std_t1font_882_index[21])
            .write(7 as ::core::ffi::c_int);
        self.initialized[9] = true;
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
        (*crate::invocation_state::current()).generated_backend_mapfile
    ))
}
#[cfg(test)]
pub(crate) unsafe fn check_state_construction(phase: u8) {
    let invocation = crate::invocation_state::current();
    let destination = state();
    assert!(!destination.is_null());
    let _ = (invocation, phase);
}
