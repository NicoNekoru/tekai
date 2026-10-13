// Rust translation and subsequent modifications for Tekai.
// Upstream TeX Live source revision 52dac44dde13a8310bbb8231e81c58be28cebe20.
// Origin texk/web2c/pdftexdir/writet1.c.
// GPL-2.0-or-later.
// Modified for Tekai. Licensing and provenance notice restored 2026-10-13.
// Original copyrights and license terms are retained in the crate's
// NOTICE.md and licenses/upstream-notices.txt.

pub use crate::generated::backend::mapfile::check_ff_exist;
pub use crate::generated::pdftex0::{getnullstr, zpackfilename};
pub use crate::openclose::recorder_record_input;
pub use crate::utils::{fb_offset, fb_putchar, make_subset_tag, maketexstring};

pub use crate::generated::backend::avl::{
    avl_create, avl_destroy, avl_find, avl_node, avl_probe, avl_t_first, avl_t_init, avl_t_next,
    avl_table, avl_traverser,
};

use crate::byte_string::isdigit;
use crate::byte_string::{
    memcpy, strcat, strchr, strcmp, strcpy, strlen, strncmp, strncpy, strstr,
};

use crate::backend_grammar::{
    charstring_count, encoding_assignment, encoding_copy, encoding_interval, predefined_encoding,
};
use crate::backend_parse::{checked_span, FieldError};
use crate::float_parse::{type1_number, FloatError};
use crate::openclose::open_input;
use crate::stdio::{feof, ungetc};
use crate::support::{xfclose, xfopen};
use crate::web2c::zround;

use crate::allocation::free;
use crate::openclose::{xmalloc, xrealloc, xstrdup};

use crate::termination::__assert_rtn;

#[inline(always)]
unsafe fn fast_fgetc(stream: *mut FILE) -> ::core::ffi::c_int {
    unsafe { crate::stdio::fgetc(stream) }
}
pub type __builtin_va_list = *mut ::core::ffi::c_char;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __darwin_size_t = usize;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_wchar_t = ::libc::wchar_t;
pub type __darwin_off_t = __int64_t;
pub type va_list = __darwin_va_list;
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
pub use crate::stdio::Stream as FILE;
pub type boolean = ::core::ffi::c_int;
pub type string = *mut ::core::ffi::c_char;
pub type const_string = *const ::core::ffi::c_char;
pub type address = *mut ::core::ffi::c_void;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const kpse_last_format: C2RustUnnamed = 59;
pub const kpse_bltxml_format: C2RustUnnamed = 58;
pub const kpse_ris_format: C2RustUnnamed = 57;
pub const kpse_clua_format: C2RustUnnamed = 56;
pub const kpse_mlbst_format: C2RustUnnamed = 55;
pub const kpse_mlbib_format: C2RustUnnamed = 54;
pub const kpse_cid_format: C2RustUnnamed = 53;
pub const kpse_fea_format: C2RustUnnamed = 52;
pub const kpse_lua_format: C2RustUnnamed = 51;
pub const kpse_texmfscripts_format: C2RustUnnamed = 50;
pub const kpse_lig_format: C2RustUnnamed = 49;
pub const kpse_pdftex_config_format: C2RustUnnamed = 48;
pub const kpse_opentype_format: C2RustUnnamed = 47;
pub const kpse_sfd_format: C2RustUnnamed = 46;
pub const kpse_cmap_format: C2RustUnnamed = 45;
pub const kpse_enc_format: C2RustUnnamed = 44;
pub const kpse_cweb_format: C2RustUnnamed = 43;
pub const kpse_web_format: C2RustUnnamed = 42;
pub const kpse_miscfonts_format: C2RustUnnamed = 41;
pub const kpse_program_binary_format: C2RustUnnamed = 40;
pub const kpse_program_text_format: C2RustUnnamed = 39;
pub const kpse_web2c_format: C2RustUnnamed = 38;
pub const kpse_type42_format: C2RustUnnamed = 37;
pub const kpse_truetype_format: C2RustUnnamed = 36;
pub const kpse_ist_format: C2RustUnnamed = 35;
pub const kpse_dvips_config_format: C2RustUnnamed = 34;
pub const kpse_vf_format: C2RustUnnamed = 33;
pub const kpse_type1_format: C2RustUnnamed = 32;
pub const kpse_troff_font_format: C2RustUnnamed = 31;
pub const kpse_tex_ps_header_format: C2RustUnnamed = 30;
pub const kpse_texsource_format: C2RustUnnamed = 29;
pub const kpse_texpool_format: C2RustUnnamed = 28;
pub const kpse_texdoc_format: C2RustUnnamed = 27;
pub const kpse_tex_format: C2RustUnnamed = 26;
pub const kpse_pict_format: C2RustUnnamed = 25;
pub const kpse_ovp_format: C2RustUnnamed = 24;
pub const kpse_ovf_format: C2RustUnnamed = 23;
pub const kpse_otp_format: C2RustUnnamed = 22;
pub const kpse_opl_format: C2RustUnnamed = 21;
pub const kpse_ofm_format: C2RustUnnamed = 20;
pub const kpse_ocp_format: C2RustUnnamed = 19;
pub const kpse_mpsupport_format: C2RustUnnamed = 18;
pub const kpse_mppool_format: C2RustUnnamed = 17;
pub const kpse_mp_format: C2RustUnnamed = 16;
pub const kpse_mft_format: C2RustUnnamed = 15;
pub const kpse_mfpool_format: C2RustUnnamed = 14;
pub const kpse_mf_format: C2RustUnnamed = 13;
pub const kpse_mem_format: C2RustUnnamed = 12;
pub const kpse_fontmap_format: C2RustUnnamed = 11;
pub const kpse_fmt_format: C2RustUnnamed = 10;
pub const kpse_db_format: C2RustUnnamed = 9;
pub const kpse_cnf_format: C2RustUnnamed = 8;
pub const kpse_bst_format: C2RustUnnamed = 7;
pub const kpse_bib_format: C2RustUnnamed = 6;
pub const kpse_base_format: C2RustUnnamed = 5;
pub const kpse_afm_format: C2RustUnnamed = 4;
pub const kpse_tfm_format: C2RustUnnamed = 3;
pub const kpse_any_glyph_format: C2RustUnnamed = 2;
pub const kpse_pk_format: C2RustUnnamed = 1;
pub const kpse_gf_format: C2RustUnnamed = 0;
pub type integer = ::core::ffi::c_int;
pub type ASCIIcode = ::core::ffi::c_uchar;
pub type eightbits = ::core::ffi::c_uchar;
pub type strnumber = integer;
pub use crate::backend_types::_subfont_entry;
pub use crate::backend_types::key_entry;
pub type subfont_entry = _subfont_entry;
pub use crate::backend_types::fd_entry_;
#[cfg(test)]
pub use crate::backend_types::fe_entry;
#[cfg(test)]
pub use crate::backend_types::fm_entry;
#[cfg(test)]
pub use crate::backend_types::intparm;
pub type fd_entry = fd_entry_;
pub use crate::backend_types::ff_entry;
pub type t1_line_entry = ::core::ffi::c_char;
pub type byte = ::core::ffi::c_uchar;
pub type t1_buf_entry = ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cs_entry {
    pub name: *mut ::core::ffi::c_char,
    pub data: *mut byte,
    pub len: ::core::ffi::c_ushort,
    pub cslen: ::core::ffi::c_ushort,
    pub used: boolean,
    pub valid: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cc_entry {
    pub nargs: byte,
    pub bottom: boolean,
    pub clear: boolean,
    pub valid: boolean,
}
pub const ENC_STANDARD: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const ENC_BUILTIN: C2RustUnnamed_0 = 1;
pub const M_PI: ::core::ffi::c_double = 3.14159265358979323846264338327950288f64;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const FOPEN_RBIN_MODE: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"rb\0") };
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ITALIC_ANGLE_CODE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const STEMV_CODE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FONTBBOX1_CODE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const FONTNAME_CODE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const FONT_KEYS_NUM: ::core::ffi::c_int = FONTNAME_CODE + 1 as ::core::ffi::c_int;
pub const F_SUBSETTED: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const F_TRUETYPE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub const fixedcontent: ::core::ffi::c_int = false_0;

pub const CS_CALLSUBR: ::core::ffi::c_long = 10 as ::core::ffi::c_long;
pub const CS_RETURN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const CS_ESCAPE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const CS_HVCURVETO: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const CS_1BYTE_MAX: ::core::ffi::c_int = CS_HVCURVETO + 1 as ::core::ffi::c_int;
pub const CS_2BYTE_MAX: ::core::ffi::c_int =
    CS_1BYTE_MAX + 33 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
pub const CS_MAX: ::core::ffi::c_int = CS_2BYTE_MAX;

unsafe fn enc_getline() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut c: ::core::ffi::c_int = 0;
    loop {
        if feof((*__tekai_state_generated_backend_writet1).enc_file) != 0 {
            crate::utils::pdftex_fail_args(
                b"unexpected end of file\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        p = &raw mut (*__tekai_state_generated_backend_writet1).enc_line
            as *mut ::core::ffi::c_char;
        loop {
            c = fast_fgetc((*__tekai_state_generated_backend_writet1).enc_file);
            if c == 9 as ::core::ffi::c_int {
                c = 32 as ::core::ffi::c_int;
            }
            if c == 13 as ::core::ffi::c_int || c == EOF {
                c = 10 as ::core::ffi::c_int;
            }
            if c != ' ' as i32
                || p > &raw mut (*__tekai_state_generated_backend_writet1).enc_line
                    as *mut ::core::ffi::c_char
                    && *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                        != 32 as ::core::ffi::c_int
            {
                if (p.offset_from(
                    &raw mut (*__tekai_state_generated_backend_writet1).enc_line
                        as *mut ::core::ffi::c_char,
                ) as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                    > 0x1000 as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    crate::utils::pdftex_fail_args(
                        b"buffer overflow at file %s, line %d\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[
                            crate::utils::PrintfArg::from(
                                b"tekai-engine/generated/backend/writet1.rs\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            ),
                            crate::utils::PrintfArg::from(223 as ::core::ffi::c_int),
                        ],
                    );
                }
                let fresh4 = p;
                p = p.offset(1);
                *fresh4 = c as ::core::ffi::c_char;
            }
            if !(c != 10 as ::core::ffi::c_int) {
                break;
            }
        }
        if (p.offset_from(
            &raw mut (*__tekai_state_generated_backend_writet1).enc_line
                as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_long
            + 2 as ::core::ffi::c_long) as ::core::ffi::c_uint
            > 0x1000 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::utils::pdftex_fail_args(
                b"buffer overflow at file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(
                        b"tekai-engine/generated/backend/writet1.rs\0" as *const u8
                            as *const ::core::ffi::c_char,
                    ),
                    crate::utils::PrintfArg::from(225 as ::core::ffi::c_int),
                ],
            );
        }
        if p.offset_from(
            &raw mut (*__tekai_state_generated_backend_writet1).enc_line
                as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_long
            > 1 as ::core::ffi::c_long
            && *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                != 10 as ::core::ffi::c_int
        {
            let fresh5 = p;
            p = p.offset(1);
            *fresh5 = 10 as ::core::ffi::c_char;
        }
        if p.offset_from(
            &raw mut (*__tekai_state_generated_backend_writet1).enc_line
                as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_long
            > 2 as ::core::ffi::c_long
            && *p.offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == 32 as ::core::ffi::c_int
        {
            *p.offset(-(2 as ::core::ffi::c_int) as isize) = 10 as ::core::ffi::c_char;
            p = p.offset(-1);
        }
        *p = 0 as ::core::ffi::c_char;
        if !((p.offset_from(
            &raw mut (*__tekai_state_generated_backend_writet1).enc_line
                as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_long)
            < 2 as ::core::ffi::c_long
            || *(&raw mut (*__tekai_state_generated_backend_writet1).enc_line
                as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                == '%' as i32)
        {
            break;
        }
    }
}
pub unsafe fn load_enc_file(
    mut enc_name: *mut ::core::ffi::c_char,
) -> *mut *mut ::core::ffi::c_char {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };
    let __tekai_state_generated_backend_writettf: *mut crate::generated::backend::writettf::State =
        unsafe { crate::generated::backend::writettf::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    let mut buf: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut names_count: ::core::ffi::c_int = 0;
    let mut glyph_names: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*__tekai_state_utils).cur_file_name = enc_name;
    zpackfilename(
        maketexstring((*__tekai_state_utils).cur_file_name),
        getnullstr(),
        getnullstr(),
    );
    if open_input(
        &raw mut (*__tekai_state_generated_backend_writet1).enc_file,
        kpse_enc_format as ::core::ffi::c_int,
        FOPEN_RBIN_MODE.as_ptr(),
    ) == 0
    {
        crate::utils::pdftex_fail_args(
            b"cannot open encoding file for reading\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
    }
    glyph_names = xmalloc(
        (256 as size_t).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
    ) as *mut *mut ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        let ref mut fresh0 = *glyph_names.offset(i as isize);
        *fresh0 =
            &raw mut (*__tekai_state_generated_backend_writettf).notdef as *mut ::core::ffi::c_char;
        i += 1;
    }
    crate::utils::tex_printf_args(
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(
            b"{\0" as *const u8 as *const ::core::ffi::c_char,
        )],
    );
    (*__tekai_state_utils).cur_file_name = ((*__tekai_state_generated_pdftexextra).nameoffile
        as *mut ::core::ffi::c_char)
        .offset(1 as ::core::ffi::c_int as isize);
    crate::utils::tex_printf_args(
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(
            (*__tekai_state_utils).cur_file_name,
        )],
    );
    enc_getline();
    if *(&raw mut (*__tekai_state_generated_backend_writet1).enc_line as *mut ::core::ffi::c_char)
        as ::core::ffi::c_int
        != '/' as i32
        || {
            r = strchr(
                &raw mut (*__tekai_state_generated_backend_writet1).enc_line
                    as *mut ::core::ffi::c_char,
                '[' as i32,
            );
            r.is_null()
        }
    {
        r = strchr(
            &raw mut (*__tekai_state_generated_backend_writet1).enc_line
                as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        )
        .offset(-(1 as ::core::ffi::c_int as isize));
        if *r as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
            *r = 0 as ::core::ffi::c_char;
        }
        crate::utils::pdftex_fail_args(
            b"invalid encoding vector (a name or `[' missing): `%s'\0" as *const u8
                as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                &raw mut (*__tekai_state_generated_backend_writet1).enc_line
                    as *mut ::core::ffi::c_char,
            )],
        );
    }
    names_count = 0 as ::core::ffi::c_int;
    r = r.offset(1);
    if *r as ::core::ffi::c_int == ' ' as i32 {
        r = r.offset(1);
    }
    loop {
        while *r as ::core::ffi::c_int == '/' as i32 {
            p = &raw mut buf as *mut ::core::ffi::c_char;
            r = r.offset(1);
            while *r as ::core::ffi::c_int != ' ' as i32
                && *r as ::core::ffi::c_int != 10 as ::core::ffi::c_int
                && *r as ::core::ffi::c_int != ']' as i32
                && *r as ::core::ffi::c_int != '/' as i32
            {
                let fresh1 = r;
                r = r.offset(1);
                let fresh2 = p;
                p = p.offset(1);
                *fresh2 = *fresh1;
            }
            *p = 0 as ::core::ffi::c_char;
            if *r as ::core::ffi::c_int == ' ' as i32 {
                r = r.offset(1);
            }
            if names_count > 255 as ::core::ffi::c_int {
                crate::utils::pdftex_fail_args(
                    b"encoding vector contains more than 256 names\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if strcmp(
                &raw mut buf as *mut ::core::ffi::c_char,
                &raw mut (*__tekai_state_generated_backend_writettf).notdef
                    as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            {
                let ref mut fresh3 = *glyph_names.offset(names_count as isize);
                *fresh3 = xstrdup(&raw mut buf as *mut ::core::ffi::c_char as const_string)
                    as *mut ::core::ffi::c_char;
            }
            names_count += 1;
        }
        if *r as ::core::ffi::c_int != 10 as ::core::ffi::c_int
            && *r as ::core::ffi::c_int != '%' as i32
        {
            if strncmp(
                r,
                b"] def\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"] def\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0 as ::core::ffi::c_int
            {
                break;
            }
            r = strchr(
                &raw mut (*__tekai_state_generated_backend_writet1).enc_line
                    as *mut ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            )
            .offset(-(1 as ::core::ffi::c_int as isize));
            if *r as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
                *r = 0 as ::core::ffi::c_char;
            }
            crate::utils::pdftex_fail_args(
                b"invalid encoding vector: a name or `] def' expected: `%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(
                    &raw mut (*__tekai_state_generated_backend_writet1).enc_line
                        as *mut ::core::ffi::c_char,
                )],
            );
        } else {
            enc_getline();
            r = &raw mut (*__tekai_state_generated_backend_writet1).enc_line
                as *mut ::core::ffi::c_char;
        }
    }
    xfclose(
        (*__tekai_state_generated_backend_writet1).enc_file,
        (*__tekai_state_utils).cur_file_name as const_string,
    );
    crate::utils::tex_printf_args(
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(
            b"}\0" as *const u8 as *const ::core::ffi::c_char,
        )],
    );
    (*__tekai_state_utils).cur_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return glyph_names;
}
unsafe fn t1_check_pfa() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let c: ::core::ffi::c_int =
        fast_fgetc((*__tekai_state_generated_backend_writet1).t1_file) as ::core::ffi::c_int;
    (*__tekai_state_generated_backend_writet1).t1_pfa = (if c != 128 as ::core::ffi::c_int {
        true_0
    } else {
        false_0
    }) as boolean;
    ungetc(c, (*__tekai_state_generated_backend_writet1).t1_file);
}
unsafe fn t1_getbyte() -> ::core::ffi::c_int {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let mut c: ::core::ffi::c_int = fast_fgetc((*__tekai_state_generated_backend_writet1).t1_file);
    if (*__tekai_state_generated_backend_writet1).t1_pfa != 0 {
        return c;
    }
    if (*__tekai_state_generated_backend_writet1).t1_block_length == 0 as ::core::ffi::c_long {
        if c != 128 as ::core::ffi::c_int {
            crate::utils::pdftex_fail_args(
                b"invalid marker\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        c = fast_fgetc((*__tekai_state_generated_backend_writet1).t1_file);
        if c == 3 as ::core::ffi::c_int {
            while feof((*__tekai_state_generated_backend_writet1).t1_file) == 0 {
                fast_fgetc((*__tekai_state_generated_backend_writet1).t1_file);
            }
            return EOF;
        }
        (*__tekai_state_generated_backend_writet1).t1_block_length =
            (fast_fgetc((*__tekai_state_generated_backend_writet1).t1_file)
                & 0xff as ::core::ffi::c_int) as ::core::ffi::c_long;
        (*__tekai_state_generated_backend_writet1).t1_block_length |=
            ((fast_fgetc((*__tekai_state_generated_backend_writet1).t1_file)
                & 0xff as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int) as ::core::ffi::c_long;
        (*__tekai_state_generated_backend_writet1).t1_block_length |=
            ((fast_fgetc((*__tekai_state_generated_backend_writet1).t1_file)
                & 0xff as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as ::core::ffi::c_long;
        (*__tekai_state_generated_backend_writet1).t1_block_length |=
            ((fast_fgetc((*__tekai_state_generated_backend_writet1).t1_file)
                & 0xff as ::core::ffi::c_int)
                << 24 as ::core::ffi::c_int) as ::core::ffi::c_long;
        c = fast_fgetc((*__tekai_state_generated_backend_writet1).t1_file);
    }
    (*__tekai_state_generated_backend_writet1).t1_block_length -= 1;
    return c;
}
unsafe fn hexval(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if c >= 'A' as i32 && c <= 'F' as i32 {
        return c - 'A' as i32 + 10 as ::core::ffi::c_int;
    } else if c >= 'a' as i32 && c <= 'f' as i32 {
        return c - 'a' as i32 + 10 as ::core::ffi::c_int;
    } else if c >= '0' as i32 && c <= '9' as i32 {
        return c - '0' as i32;
    } else {
        return -(1 as ::core::ffi::c_int);
    };
}
unsafe fn edecrypt(mut cipher: byte) -> byte {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let mut plain: byte = 0;
    if (*__tekai_state_generated_backend_writet1).t1_pfa != 0 {
        while cipher as ::core::ffi::c_int == 10 as ::core::ffi::c_int
            || cipher as ::core::ffi::c_int == 13 as ::core::ffi::c_int
        {
            cipher = t1_getbyte() as byte;
        }
        cipher = ((hexval(cipher as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            + hexval(t1_getbyte())) as byte;
        (*__tekai_state_generated_backend_writet1).last_hexbyte = cipher as ::core::ffi::c_int;
    }
    plain = (cipher as ::core::ffi::c_int
        ^ (*__tekai_state_generated_backend_writet1).t1_dr as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int) as byte;
    (*__tekai_state_generated_backend_writet1).t1_dr = (cipher as ::core::ffi::c_int
        + (*__tekai_state_generated_backend_writet1).t1_dr as ::core::ffi::c_int)
        .wrapping_mul((*__tekai_state_generated_backend_writet1).t1_c1 as ::core::ffi::c_int)
        .wrapping_add((*__tekai_state_generated_backend_writet1).t1_c2 as ::core::ffi::c_int)
        as ::core::ffi::c_ushort;
    return plain;
}
unsafe fn cdecrypt(mut cipher: byte, mut cr: *mut ::core::ffi::c_ushort) -> byte {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let plain: byte = (cipher as ::core::ffi::c_int
        ^ *cr as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as byte;
    *cr = (cipher as ::core::ffi::c_int + *cr as ::core::ffi::c_int)
        .wrapping_mul((*__tekai_state_generated_backend_writet1).t1_c1 as ::core::ffi::c_int)
        .wrapping_add((*__tekai_state_generated_backend_writet1).t1_c2 as ::core::ffi::c_int)
        as ::core::ffi::c_ushort;
    return plain;
}
unsafe fn eencrypt(mut plain: byte) -> byte {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let cipher: byte = (plain as ::core::ffi::c_int
        ^ (*__tekai_state_generated_backend_writet1).t1_er as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int) as byte;
    (*__tekai_state_generated_backend_writet1).t1_er = (cipher as ::core::ffi::c_int
        + (*__tekai_state_generated_backend_writet1).t1_er as ::core::ffi::c_int)
        .wrapping_mul((*__tekai_state_generated_backend_writet1).t1_c1 as ::core::ffi::c_int)
        .wrapping_add((*__tekai_state_generated_backend_writet1).t1_c2 as ::core::ffi::c_int)
        as ::core::ffi::c_ushort;
    return cipher;
}
unsafe fn cencrypt(mut plain: byte, mut cr: *mut ::core::ffi::c_ushort) -> byte {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let cipher: byte = (plain as ::core::ffi::c_int
        ^ *cr as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as byte;
    *cr = (cipher as ::core::ffi::c_int + *cr as ::core::ffi::c_int)
        .wrapping_mul((*__tekai_state_generated_backend_writet1).t1_c1 as ::core::ffi::c_int)
        .wrapping_add((*__tekai_state_generated_backend_writet1).t1_c2 as ::core::ffi::c_int)
        as ::core::ffi::c_ushort;
    return cipher;
}
unsafe fn eol(mut s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = strchr(s, 0 as ::core::ffi::c_int);
    if p.offset_from(s) as ::core::ffi::c_long > 1 as ::core::ffi::c_long
        && *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            != 10 as ::core::ffi::c_int
    {
        let fresh16 = p;
        p = p.offset(1);
        *fresh16 = 10 as ::core::ffi::c_char;
        *p = 0 as ::core::ffi::c_char;
    }
    return p;
}
unsafe fn t1_scan_num(
    mut p: *mut ::core::ffi::c_char,
    mut r: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_float {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let parsed = type1_number(t1_written_suffix(p));
    if matches!(parsed, Err(FloatError::NonFinite | FloatError::Overflow)) {
        crate::utils::pdftex_fail_args(c"invalid pfb, floating field out of range".as_ptr(), &[]);
    }
    let Ok(parsed) = parsed else {
        p = strchr(
            (*__tekai_state_generated_backend_writet1).t1_line_array,
            0 as ::core::ffi::c_int,
        )
        .offset(-(1 as ::core::ffi::c_int as isize));
        if *p as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
            *p = 0 as ::core::ffi::c_char;
        }
        crate::utils::pdftex_fail_args(
            b"a number expected: `%s'\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                (*__tekai_state_generated_backend_writet1).t1_line_array,
            )],
        );
    };
    if !r.is_null() {
        *r = p.add(parsed.consumed);
    }
    return parsed.value;
}
unsafe fn str_suffix(
    mut begin_buf: *const ::core::ffi::c_char,
    mut end_buf: *const ::core::ffi::c_char,
    mut s: *const ::core::ffi::c_char,
) -> boolean {
    let mut s1: *const ::core::ffi::c_char = end_buf.offset(-(1 as ::core::ffi::c_int as isize));
    let mut s2: *const ::core::ffi::c_char =
        strchr(s, 0 as ::core::ffi::c_int).offset(-(1 as ::core::ffi::c_int as isize));
    if *s1 as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
        s1 = s1.offset(-1);
    }
    while s1 >= begin_buf && s2 >= s {
        let fresh12 = s1;
        s1 = s1.offset(-1);
        let fresh13 = s2;
        s2 = s2.offset(-1);
        if *fresh12 as ::core::ffi::c_int != *fresh13 as ::core::ffi::c_int {
            return false_0;
        }
    }
    return (s2 < s) as ::core::ffi::c_int;
}
unsafe fn t1_getline() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    let mut c: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut eexec_scan: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();

    loop {
        if feof((*__tekai_state_generated_backend_writet1).t1_file) != 0 {
            crate::utils::pdftex_fail_args(
                b"unexpected end of file\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        (*__tekai_state_generated_backend_writet1).t1_line_ptr =
            (*__tekai_state_generated_backend_writet1).t1_line_array;
        if (*__tekai_state_generated_backend_writet1)
            .t1_line_array
            .is_null()
        {
            (*__tekai_state_generated_backend_writet1).t1_line_limit = 0x10 as size_t;
            if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                > (*__tekai_state_generated_backend_writet1).t1_line_limit as ::core::ffi::c_uint
            {
                (*__tekai_state_generated_backend_writet1).t1_line_limit = 1 as size_t;
            }
            (*__tekai_state_generated_backend_writet1).t1_line_array = xmalloc(
                (*__tekai_state_generated_backend_writet1)
                    .t1_line_limit
                    .wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
            )
                as *mut t1_line_entry;
            (*__tekai_state_generated_backend_writet1).t1_line_ptr =
                (*__tekai_state_generated_backend_writet1).t1_line_array;
        } else if ((*__tekai_state_generated_backend_writet1)
            .t1_line_ptr
            .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
            as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
            > (*__tekai_state_generated_backend_writet1).t1_line_limit as ::core::ffi::c_uint
        {
            (*__tekai_state_utils).last_ptr_index = (*__tekai_state_generated_backend_writet1)
                .t1_line_ptr
                .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                as ::core::ffi::c_long
                as size_t;
            (*__tekai_state_generated_backend_writet1).t1_line_limit =
                (*__tekai_state_generated_backend_writet1)
                    .t1_line_limit
                    .wrapping_mul(2 as size_t);
            if ((*__tekai_state_generated_backend_writet1)
                .t1_line_ptr
                .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                > (*__tekai_state_generated_backend_writet1).t1_line_limit as ::core::ffi::c_uint
            {
                (*__tekai_state_generated_backend_writet1).t1_line_limit =
                    ((*__tekai_state_generated_backend_writet1)
                        .t1_line_ptr
                        .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                        as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as size_t;
            }
            if (*__tekai_state_generated_backend_writet1).t1_line_limit as ::core::ffi::c_uint
                > INT_MAX as ::core::ffi::c_uint
            {
                crate::utils::pdftex_fail_args(
                    b"t1_line_array exceeds size limit\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                );
            }
            (*__tekai_state_generated_backend_writet1).t1_line_array = xrealloc(
                (*__tekai_state_generated_backend_writet1).t1_line_array as address,
                (*__tekai_state_generated_backend_writet1)
                    .t1_line_limit
                    .wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
            )
                as *mut t1_line_entry;
            (*__tekai_state_generated_backend_writet1).t1_line_ptr =
                (*__tekai_state_generated_backend_writet1)
                    .t1_line_array
                    .offset((*__tekai_state_utils).last_ptr_index as isize);
        }
        (*__tekai_state_generated_backend_writet1).t1_cslen = 0 as ::core::ffi::c_ushort;
        eexec_scan = 0 as ::core::ffi::c_int;
        c = t1_getbyte();
        if c == EOF {
            break;
        }
        while feof((*__tekai_state_generated_backend_writet1).t1_file) == 0 {
            if (*__tekai_state_generated_backend_writet1).t1_in_eexec == 1 as ::core::ffi::c_int {
                c = edecrypt(c as byte) as ::core::ffi::c_int;
            }
            if (*__tekai_state_generated_backend_writet1)
                .t1_line_array
                .is_null()
            {
                (*__tekai_state_generated_backend_writet1).t1_line_limit = 0x10 as size_t;
                if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                    > (*__tekai_state_generated_backend_writet1).t1_line_limit
                        as ::core::ffi::c_uint
                {
                    (*__tekai_state_generated_backend_writet1).t1_line_limit = 1 as size_t;
                }
                (*__tekai_state_generated_backend_writet1).t1_line_array = xmalloc(
                    (*__tekai_state_generated_backend_writet1)
                        .t1_line_limit
                        .wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
                )
                    as *mut t1_line_entry;
                (*__tekai_state_generated_backend_writet1).t1_line_ptr =
                    (*__tekai_state_generated_backend_writet1).t1_line_array;
            } else if ((*__tekai_state_generated_backend_writet1)
                .t1_line_ptr
                .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                > (*__tekai_state_generated_backend_writet1).t1_line_limit as ::core::ffi::c_uint
            {
                (*__tekai_state_utils).last_ptr_index = (*__tekai_state_generated_backend_writet1)
                    .t1_line_ptr
                    .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                    as ::core::ffi::c_long
                    as size_t;
                (*__tekai_state_generated_backend_writet1).t1_line_limit =
                    (*__tekai_state_generated_backend_writet1)
                        .t1_line_limit
                        .wrapping_mul(2 as size_t);
                if ((*__tekai_state_generated_backend_writet1)
                    .t1_line_ptr
                    .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                    as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                    > (*__tekai_state_generated_backend_writet1).t1_line_limit
                        as ::core::ffi::c_uint
                {
                    (*__tekai_state_generated_backend_writet1).t1_line_limit =
                        ((*__tekai_state_generated_backend_writet1)
                            .t1_line_ptr
                            .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                            as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as size_t;
                }
                if (*__tekai_state_generated_backend_writet1).t1_line_limit as ::core::ffi::c_uint
                    > INT_MAX as ::core::ffi::c_uint
                {
                    crate::utils::pdftex_fail_args(
                        b"t1_line_array exceeds size limit\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                }
                (*__tekai_state_generated_backend_writet1).t1_line_array = xrealloc(
                    (*__tekai_state_generated_backend_writet1).t1_line_array as address,
                    (*__tekai_state_generated_backend_writet1)
                        .t1_line_limit
                        .wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
                )
                    as *mut t1_line_entry;
                (*__tekai_state_generated_backend_writet1).t1_line_ptr =
                    (*__tekai_state_generated_backend_writet1)
                        .t1_line_array
                        .offset((*__tekai_state_utils).last_ptr_index as isize);
            }
            if c == 9 as ::core::ffi::c_int {
                c = 32 as ::core::ffi::c_int;
            }
            if c == 13 as ::core::ffi::c_int || c == EOF {
                c = 10 as ::core::ffi::c_int;
            }
            if c != ' ' as i32
                || (*__tekai_state_generated_backend_writet1).t1_line_ptr
                    > (*__tekai_state_generated_backend_writet1).t1_line_array
                    && *(*__tekai_state_generated_backend_writet1)
                        .t1_line_ptr
                        .offset(-(1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        != 32 as ::core::ffi::c_int
            {
                if ((*__tekai_state_generated_backend_writet1)
                    .t1_line_ptr
                    .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                    as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                    > (*__tekai_state_generated_backend_writet1).t1_line_limit
                        as ::core::ffi::c_uint
                {
                    crate::utils::pdftex_fail_args(
                        b"buffer overflow at file %s, line %d\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[
                            crate::utils::PrintfArg::from(
                                b"tekai-engine/generated/backend/writet1.rs\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            ),
                            crate::utils::PrintfArg::from(422 as ::core::ffi::c_int),
                        ],
                    );
                }
                let fresh8 = (*__tekai_state_generated_backend_writet1).t1_line_ptr;
                (*__tekai_state_generated_backend_writet1).t1_line_ptr =
                    (*__tekai_state_generated_backend_writet1)
                        .t1_line_ptr
                        .offset(1);
                *fresh8 = c as t1_line_entry;
            }
            if (*__tekai_state_generated_backend_writet1).t1_in_eexec == 0 as ::core::ffi::c_int
                && eexec_scan >= 0 as ::core::ffi::c_int
                && eexec_scan
                    < (*__tekai_state_generated_backend_writet1).local_t1_getline_996_eexec_len
            {
                if *(*__tekai_state_generated_backend_writet1)
                    .t1_line_array
                    .offset(eexec_scan as isize) as ::core::ffi::c_int
                    == (*__tekai_state_generated_backend_writet1).local_t1_getline_993_eexec_str
                        [eexec_scan as usize] as ::core::ffi::c_int
                {
                    eexec_scan += 1;
                } else {
                    eexec_scan = -(1 as ::core::ffi::c_int);
                }
            }
            if c == 10 as ::core::ffi::c_int
                || (*__tekai_state_generated_backend_writet1).t1_pfa != 0
                    && eexec_scan
                        == (*__tekai_state_generated_backend_writet1).local_t1_getline_996_eexec_len
                    && c == 32 as ::core::ffi::c_int
            {
                break;
            }
            if (*__tekai_state_generated_backend_writet1).t1_cs != 0
                && (*__tekai_state_generated_backend_writet1).t1_cslen as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                && (*__tekai_state_generated_backend_writet1)
                    .t1_line_ptr
                    .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                    as ::core::ffi::c_long
                    > 4 as ::core::ffi::c_long
                && (str_suffix(
                    (*__tekai_state_generated_backend_writet1).t1_line_array,
                    (*__tekai_state_generated_backend_writet1).t1_line_ptr,
                    b" RD \0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                    || str_suffix(
                        (*__tekai_state_generated_backend_writet1).t1_line_array,
                        (*__tekai_state_generated_backend_writet1).t1_line_ptr,
                        b" -| \0" as *const u8 as *const ::core::ffi::c_char,
                    ) != 0)
            {
                p = (*__tekai_state_generated_backend_writet1)
                    .t1_line_ptr
                    .offset(-(5 as ::core::ffi::c_int as isize))
                    as *mut ::core::ffi::c_char;
                while *p as ::core::ffi::c_int != ' ' as i32 {
                    p = p.offset(-1);
                }
                l = t1_scan_num(
                    p.offset(1 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                ) as ::core::ffi::c_int;
                (*__tekai_state_generated_backend_writet1).t1_cslen = l as ::core::ffi::c_ushort;
                (*__tekai_state_generated_backend_writet1).cs_start =
                    (*__tekai_state_generated_backend_writet1)
                        .t1_line_ptr
                        .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                        as ::core::ffi::c_long as ::core::ffi::c_int;
                if (*__tekai_state_generated_backend_writet1)
                    .t1_line_array
                    .is_null()
                {
                    (*__tekai_state_generated_backend_writet1).t1_line_limit = 0x10 as size_t;
                    if l as ::core::ffi::c_uint
                        > (*__tekai_state_generated_backend_writet1).t1_line_limit
                            as ::core::ffi::c_uint
                    {
                        (*__tekai_state_generated_backend_writet1).t1_line_limit = l as size_t;
                    }
                    (*__tekai_state_generated_backend_writet1).t1_line_array = xmalloc(
                        (*__tekai_state_generated_backend_writet1)
                            .t1_line_limit
                            .wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
                    )
                        as *mut t1_line_entry;
                    (*__tekai_state_generated_backend_writet1).t1_line_ptr =
                        (*__tekai_state_generated_backend_writet1).t1_line_array;
                } else if ((*__tekai_state_generated_backend_writet1)
                    .t1_line_ptr
                    .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                    as ::core::ffi::c_long
                    + l as ::core::ffi::c_long) as ::core::ffi::c_uint
                    > (*__tekai_state_generated_backend_writet1).t1_line_limit
                        as ::core::ffi::c_uint
                {
                    (*__tekai_state_utils).last_ptr_index =
                        (*__tekai_state_generated_backend_writet1)
                            .t1_line_ptr
                            .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                            as ::core::ffi::c_long as size_t;
                    (*__tekai_state_generated_backend_writet1).t1_line_limit =
                        (*__tekai_state_generated_backend_writet1)
                            .t1_line_limit
                            .wrapping_mul(2 as size_t);
                    if ((*__tekai_state_generated_backend_writet1)
                        .t1_line_ptr
                        .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                        as ::core::ffi::c_long
                        + l as ::core::ffi::c_long) as ::core::ffi::c_uint
                        > (*__tekai_state_generated_backend_writet1).t1_line_limit
                            as ::core::ffi::c_uint
                    {
                        (*__tekai_state_generated_backend_writet1).t1_line_limit =
                            ((*__tekai_state_generated_backend_writet1)
                                .t1_line_ptr
                                .offset_from(
                                    (*__tekai_state_generated_backend_writet1).t1_line_array,
                                ) as ::core::ffi::c_long
                                + l as ::core::ffi::c_long) as size_t;
                    }
                    if (*__tekai_state_generated_backend_writet1).t1_line_limit
                        as ::core::ffi::c_uint
                        > INT_MAX as ::core::ffi::c_uint
                    {
                        crate::utils::pdftex_fail_args(
                            b"t1_line_array exceeds size limit\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    }
                    (*__tekai_state_generated_backend_writet1).t1_line_array = xrealloc(
                        (*__tekai_state_generated_backend_writet1).t1_line_array as address,
                        (*__tekai_state_generated_backend_writet1)
                            .t1_line_limit
                            .wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
                    )
                        as *mut t1_line_entry;
                    (*__tekai_state_generated_backend_writet1).t1_line_ptr =
                        (*__tekai_state_generated_backend_writet1)
                            .t1_line_array
                            .offset((*__tekai_state_utils).last_ptr_index as isize);
                }
                loop {
                    let fresh9 = l;
                    l = l - 1;
                    if !(fresh9 > 0 as ::core::ffi::c_int) {
                        break;
                    }
                    let fresh10 = (*__tekai_state_generated_backend_writet1).t1_line_ptr;
                    (*__tekai_state_generated_backend_writet1).t1_line_ptr =
                        (*__tekai_state_generated_backend_writet1)
                            .t1_line_ptr
                            .offset(1);
                    *fresh10 = edecrypt(t1_getbyte() as byte) as t1_line_entry;
                }
            }
            c = t1_getbyte();
        }
        if (*__tekai_state_generated_backend_writet1)
            .t1_line_array
            .is_null()
        {
            (*__tekai_state_generated_backend_writet1).t1_line_limit = 0x10 as size_t;
            if 2 as ::core::ffi::c_int as ::core::ffi::c_uint
                > (*__tekai_state_generated_backend_writet1).t1_line_limit as ::core::ffi::c_uint
            {
                (*__tekai_state_generated_backend_writet1).t1_line_limit = 2 as size_t;
            }
            (*__tekai_state_generated_backend_writet1).t1_line_array = xmalloc(
                (*__tekai_state_generated_backend_writet1)
                    .t1_line_limit
                    .wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
            )
                as *mut t1_line_entry;
            (*__tekai_state_generated_backend_writet1).t1_line_ptr =
                (*__tekai_state_generated_backend_writet1).t1_line_array;
        } else if ((*__tekai_state_generated_backend_writet1)
            .t1_line_ptr
            .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
            as ::core::ffi::c_long
            + 2 as ::core::ffi::c_long) as ::core::ffi::c_uint
            > (*__tekai_state_generated_backend_writet1).t1_line_limit as ::core::ffi::c_uint
        {
            (*__tekai_state_utils).last_ptr_index = (*__tekai_state_generated_backend_writet1)
                .t1_line_ptr
                .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                as ::core::ffi::c_long
                as size_t;
            (*__tekai_state_generated_backend_writet1).t1_line_limit =
                (*__tekai_state_generated_backend_writet1)
                    .t1_line_limit
                    .wrapping_mul(2 as size_t);
            if ((*__tekai_state_generated_backend_writet1)
                .t1_line_ptr
                .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                as ::core::ffi::c_long
                + 2 as ::core::ffi::c_long) as ::core::ffi::c_uint
                > (*__tekai_state_generated_backend_writet1).t1_line_limit as ::core::ffi::c_uint
            {
                (*__tekai_state_generated_backend_writet1).t1_line_limit =
                    ((*__tekai_state_generated_backend_writet1)
                        .t1_line_ptr
                        .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                        as ::core::ffi::c_long
                        + 2 as ::core::ffi::c_long) as size_t;
            }
            if (*__tekai_state_generated_backend_writet1).t1_line_limit as ::core::ffi::c_uint
                > INT_MAX as ::core::ffi::c_uint
            {
                crate::utils::pdftex_fail_args(
                    b"t1_line_array exceeds size limit\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                );
            }
            (*__tekai_state_generated_backend_writet1).t1_line_array = xrealloc(
                (*__tekai_state_generated_backend_writet1).t1_line_array as address,
                (*__tekai_state_generated_backend_writet1)
                    .t1_line_limit
                    .wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
            )
                as *mut t1_line_entry;
            (*__tekai_state_generated_backend_writet1).t1_line_ptr =
                (*__tekai_state_generated_backend_writet1)
                    .t1_line_array
                    .offset((*__tekai_state_utils).last_ptr_index as isize);
        }
        if ((*__tekai_state_generated_backend_writet1)
            .t1_line_ptr
            .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
            as ::core::ffi::c_long
            + 2 as ::core::ffi::c_long) as ::core::ffi::c_uint
            > (*__tekai_state_generated_backend_writet1).t1_line_limit as ::core::ffi::c_uint
        {
            crate::utils::pdftex_fail_args(
                b"buffer overflow at file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(
                        b"tekai-engine/generated/backend/writet1.rs\0" as *const u8
                            as *const ::core::ffi::c_char,
                    ),
                    crate::utils::PrintfArg::from(445 as ::core::ffi::c_int),
                ],
            );
        }
        if (*__tekai_state_generated_backend_writet1)
            .t1_line_ptr
            .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
            as ::core::ffi::c_long
            > 1 as ::core::ffi::c_long
            && *(*__tekai_state_generated_backend_writet1)
                .t1_line_ptr
                .offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                != 10 as ::core::ffi::c_int
        {
            let fresh11 = (*__tekai_state_generated_backend_writet1).t1_line_ptr;
            (*__tekai_state_generated_backend_writet1).t1_line_ptr =
                (*__tekai_state_generated_backend_writet1)
                    .t1_line_ptr
                    .offset(1);
            *fresh11 = 10 as t1_line_entry;
        }
        if (*__tekai_state_generated_backend_writet1)
            .t1_line_ptr
            .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
            as ::core::ffi::c_long
            > 2 as ::core::ffi::c_long
            && *(*__tekai_state_generated_backend_writet1)
                .t1_line_ptr
                .offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == 32 as ::core::ffi::c_int
        {
            *(*__tekai_state_generated_backend_writet1)
                .t1_line_ptr
                .offset(-(2 as ::core::ffi::c_int) as isize) = 10 as t1_line_entry;
            (*__tekai_state_generated_backend_writet1).t1_line_ptr =
                (*__tekai_state_generated_backend_writet1)
                    .t1_line_ptr
                    .offset(-1);
        }
        *(*__tekai_state_generated_backend_writet1).t1_line_ptr = 0 as t1_line_entry;
        if ((*__tekai_state_generated_backend_writet1)
            .t1_line_ptr
            .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
            as ::core::ffi::c_long)
            < 2 as ::core::ffi::c_long
        {
            continue;
        }
        if eexec_scan == (*__tekai_state_generated_backend_writet1).local_t1_getline_996_eexec_len {
            (*__tekai_state_generated_backend_writet1).t1_in_eexec = 1 as ::core::ffi::c_int;
        }
        break;
    }
    (*__tekai_state_generated_backend_writet1).t1_buf_ptr =
        (*__tekai_state_generated_backend_writet1).t1_buf_array;
    if (*__tekai_state_generated_backend_writet1)
        .t1_buf_array
        .is_null()
    {
        (*__tekai_state_generated_backend_writet1).t1_buf_limit =
            (*__tekai_state_generated_backend_writet1).t1_line_limit;
        if (*__tekai_state_generated_backend_writet1).t1_line_limit as ::core::ffi::c_uint
            > (*__tekai_state_generated_backend_writet1).t1_buf_limit as ::core::ffi::c_uint
        {
            (*__tekai_state_generated_backend_writet1).t1_buf_limit =
                (*__tekai_state_generated_backend_writet1).t1_line_limit;
        }
        (*__tekai_state_generated_backend_writet1).t1_buf_array = xmalloc(
            (*__tekai_state_generated_backend_writet1)
                .t1_buf_limit
                .wrapping_mul(::core::mem::size_of::<t1_buf_entry>() as size_t),
        ) as *mut t1_buf_entry;
        (*__tekai_state_generated_backend_writet1).t1_buf_ptr =
            (*__tekai_state_generated_backend_writet1).t1_buf_array;
    } else if ((*__tekai_state_generated_backend_writet1)
        .t1_buf_ptr
        .offset_from((*__tekai_state_generated_backend_writet1).t1_buf_array)
        as ::core::ffi::c_long as size_t)
        .wrapping_add((*__tekai_state_generated_backend_writet1).t1_line_limit)
        as ::core::ffi::c_uint
        > (*__tekai_state_generated_backend_writet1).t1_buf_limit as ::core::ffi::c_uint
    {
        (*__tekai_state_utils).last_ptr_index = (*__tekai_state_generated_backend_writet1)
            .t1_buf_ptr
            .offset_from((*__tekai_state_generated_backend_writet1).t1_buf_array)
            as ::core::ffi::c_long as size_t;
        (*__tekai_state_generated_backend_writet1).t1_buf_limit =
            (*__tekai_state_generated_backend_writet1)
                .t1_buf_limit
                .wrapping_mul(2 as size_t);
        if ((*__tekai_state_generated_backend_writet1)
            .t1_buf_ptr
            .offset_from((*__tekai_state_generated_backend_writet1).t1_buf_array)
            as ::core::ffi::c_long as size_t)
            .wrapping_add((*__tekai_state_generated_backend_writet1).t1_line_limit)
            as ::core::ffi::c_uint
            > (*__tekai_state_generated_backend_writet1).t1_buf_limit as ::core::ffi::c_uint
        {
            (*__tekai_state_generated_backend_writet1).t1_buf_limit =
                ((*__tekai_state_generated_backend_writet1)
                    .t1_buf_ptr
                    .offset_from((*__tekai_state_generated_backend_writet1).t1_buf_array)
                    as ::core::ffi::c_long as size_t)
                    .wrapping_add((*__tekai_state_generated_backend_writet1).t1_line_limit);
        }
        if (*__tekai_state_generated_backend_writet1).t1_buf_limit as ::core::ffi::c_uint
            > INT_MAX as ::core::ffi::c_uint
        {
            crate::utils::pdftex_fail_args(
                b"t1_buf_array exceeds size limit\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        (*__tekai_state_generated_backend_writet1).t1_buf_array = xrealloc(
            (*__tekai_state_generated_backend_writet1).t1_buf_array as address,
            (*__tekai_state_generated_backend_writet1)
                .t1_buf_limit
                .wrapping_mul(::core::mem::size_of::<t1_buf_entry>() as size_t),
        ) as *mut t1_buf_entry;
        (*__tekai_state_generated_backend_writet1).t1_buf_ptr =
            (*__tekai_state_generated_backend_writet1)
                .t1_buf_array
                .offset((*__tekai_state_utils).last_ptr_index as isize);
    }
}
unsafe fn t1_putline() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let mut p: *mut ::core::ffi::c_char =
        (*__tekai_state_generated_backend_writet1).t1_line_array as *mut ::core::ffi::c_char;
    if (*__tekai_state_generated_backend_writet1)
        .t1_line_ptr
        .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
        as ::core::ffi::c_long
        <= 1 as ::core::ffi::c_long
    {
        return;
    }
    if (*__tekai_state_generated_backend_writet1).t1_eexec_encrypt != 0 {
        while p < (*__tekai_state_generated_backend_writet1).t1_line_ptr {
            let fresh6 = p;
            p = p.offset(1);
            fb_putchar(eencrypt(*fresh6 as byte) as eightbits);
        }
    } else {
        while p < (*__tekai_state_generated_backend_writet1).t1_line_ptr {
            let fresh7 = p;
            p = p.offset(1);
            fb_putchar(*fresh7 as eightbits);
        }
    };
}
unsafe fn t1_puts(mut s: *const ::core::ffi::c_char) {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    if s != (*__tekai_state_generated_backend_writet1).t1_line_array as *const ::core::ffi::c_char {
        strcpy(
            (*__tekai_state_generated_backend_writet1).t1_line_array as *mut ::core::ffi::c_char,
            s,
        );
    }
    (*__tekai_state_generated_backend_writet1).t1_line_ptr = strchr(
        (*__tekai_state_generated_backend_writet1).t1_line_array,
        0 as ::core::ffi::c_int,
    ) as *mut t1_line_entry;
    t1_putline();
}
unsafe fn t1_printf(
    mut fmt: *const ::core::ffi::c_char,
    mut code: ::core::ffi::c_int,
    mut glyph: *mut ::core::ffi::c_char,
) {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    crate::utils::sprintf_args(
        (*__tekai_state_generated_backend_writet1).t1_line_array as *mut ::core::ffi::c_char,
        fmt,
        &[
            crate::utils::PrintfArg::from(code),
            crate::utils::PrintfArg::from(glyph),
        ],
    );
    t1_puts((*__tekai_state_generated_backend_writet1).t1_line_array);
}
unsafe fn t1_init_params(mut open_name_prefix: *const ::core::ffi::c_char) {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    crate::utils::tex_printf_args(
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(open_name_prefix)],
    );
    crate::utils::tex_printf_args(
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(
            (*__tekai_state_utils).cur_file_name,
        )],
    );
    (*__tekai_state_generated_backend_writet1).t1_lenIV = 4 as ::core::ffi::c_short;
    (*__tekai_state_generated_backend_writet1).t1_dr = 55665 as ::core::ffi::c_ushort;
    (*__tekai_state_generated_backend_writet1).t1_er = 55665 as ::core::ffi::c_ushort;
    (*__tekai_state_generated_backend_writet1).t1_in_eexec = 0 as ::core::ffi::c_int;
    (*__tekai_state_generated_backend_writet1).t1_cs = false_0 as boolean;
    (*__tekai_state_generated_backend_writet1).t1_scan = true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).t1_synthetic = false_0 as boolean;
    (*__tekai_state_generated_backend_writet1).t1_eexec_encrypt = false_0 as boolean;
    (*__tekai_state_generated_backend_writet1).t1_block_length = 0 as ::core::ffi::c_long;
    t1_check_pfa();
}
unsafe fn t1_close_font_file(mut close_name_suffix: *const ::core::ffi::c_char) {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    crate::utils::tex_printf_args(
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(close_name_suffix)],
    );
    xfclose(
        (*__tekai_state_generated_backend_writet1).t1_file,
        (*__tekai_state_utils).cur_file_name as const_string,
    );
    (*__tekai_state_utils).cur_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe fn t1_check_block_len(mut decrypt: boolean) {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let mut l: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    if (*__tekai_state_generated_backend_writet1).t1_block_length == 0 as ::core::ffi::c_long {
        return;
    }
    c = t1_getbyte();
    if decrypt != 0 {
        c = edecrypt(c as byte) as ::core::ffi::c_int;
    }
    l = (*__tekai_state_generated_backend_writet1).t1_block_length as ::core::ffi::c_int;
    if !(l == 0 as ::core::ffi::c_int
        && (c == 10 as ::core::ffi::c_int || c == 13 as ::core::ffi::c_int))
    {
        crate::utils::pdftex_fail_args(
            b"%i bytes more than expected\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(l + 1 as ::core::ffi::c_int)],
        );
    }
}
unsafe fn t1_start_eexec() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let mut i: ::core::ffi::c_int = 0;
    if !((*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).type_0 as ::core::ffi::c_int
        & 0x1 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"t1_start_eexec\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            527 as ::core::ffi::c_int,
            b"is_included(fd_cur->fm)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*__tekai_state_generated_backend_writet1).t1_length1 =
        fb_offset() - (*__tekai_state_generated_backend_writet1).t1_save_offset;
    (*__tekai_state_generated_backend_writet1).t1_save_offset = fb_offset();
    if (*__tekai_state_generated_backend_writet1).t1_pfa == 0 {
        t1_check_block_len(false_0);
    }
    (*__tekai_state_generated_backend_writet1).t1_line_ptr =
        (*__tekai_state_generated_backend_writet1).t1_line_array;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        edecrypt(t1_getbyte() as byte);
        let fresh42 = (*__tekai_state_generated_backend_writet1).t1_line_ptr;
        (*__tekai_state_generated_backend_writet1).t1_line_ptr =
            (*__tekai_state_generated_backend_writet1)
                .t1_line_ptr
                .offset(1);
        *fresh42 = 0 as t1_line_entry;
        i += 1;
    }
    (*__tekai_state_generated_backend_writet1).t1_eexec_encrypt = true_0 as boolean;
    t1_putline();
}
unsafe fn t1_stop_eexec() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let mut c: ::core::ffi::c_int = 0;
    if !((*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).type_0 as ::core::ffi::c_int
        & 0x1 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"t1_stop_eexec\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            543 as ::core::ffi::c_int,
            b"is_included(fd_cur->fm)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*__tekai_state_generated_backend_writet1).t1_length2 =
        fb_offset() - (*__tekai_state_generated_backend_writet1).t1_save_offset;
    (*__tekai_state_generated_backend_writet1).t1_save_offset = fb_offset();
    (*__tekai_state_generated_backend_writet1).t1_eexec_encrypt = false_0 as boolean;
    if (*__tekai_state_generated_backend_writet1).t1_pfa == 0 {
        t1_check_block_len(true_0);
    } else {
        c = edecrypt(t1_getbyte() as byte) as ::core::ffi::c_int;
        if !(c == 10 as ::core::ffi::c_int || c == 13 as ::core::ffi::c_int) {
            if (*__tekai_state_generated_backend_writet1).last_hexbyte == 0 as ::core::ffi::c_int {
                t1_puts(b"00\0" as *const u8 as *const ::core::ffi::c_char);
            } else {
                crate::utils::pdftex_fail_args(
                    b"unexpected data after eexec\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
        }
    }
    (*__tekai_state_generated_backend_writet1).t1_cs = false_0 as boolean;
    (*__tekai_state_generated_backend_writet1).t1_in_eexec = 2 as ::core::ffi::c_int;
}
unsafe fn t1_modify_fm() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let mut a: [::core::ffi::c_float; 6] = [0.; 6];
    let mut i: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = strchr(
        (*__tekai_state_generated_backend_writet1).t1_line_array,
        '[' as i32,
    );
    if p.is_null() {
        p = strchr(
            (*__tekai_state_generated_backend_writet1).t1_line_array,
            '{' as i32,
        );
        if p.is_null() {
            p = strchr(
                (*__tekai_state_generated_backend_writet1).t1_line_array,
                0 as ::core::ffi::c_int,
            )
            .offset(-(1 as ::core::ffi::c_int as isize));
            if *p as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
                *p = 0 as ::core::ffi::c_char;
            }
            crate::utils::pdftex_fail_args(
                b"FontMatrix: an array expected: `%s'\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(
                    (*__tekai_state_generated_backend_writet1).t1_line_array,
                )],
            );
        }
    }
    let fresh41 = p;
    p = p.offset(1);
    c = *fresh41 as ::core::ffi::c_int;
    strncpy(
        (*__tekai_state_generated_backend_writet1).t1_buf_array as *mut ::core::ffi::c_char,
        (*__tekai_state_generated_backend_writet1).t1_line_array,
        p.offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
            as ::core::ffi::c_long as size_t,
    );
    r = (*__tekai_state_generated_backend_writet1)
        .t1_buf_array
        .offset(
            p.offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                as ::core::ffi::c_long as isize,
        ) as *mut ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i < 6 as ::core::ffi::c_int {
        a[i as usize] = t1_scan_num(p, &raw mut q);
        p = q;
        i += 1;
    }
    if (*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).slant != 0 as ::core::ffi::c_int {
        a[0 as ::core::ffi::c_int as usize] =
            (a[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                + a[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    * ((*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).slant
                        as ::core::ffi::c_double
                        * 1E-3f64)) as ::core::ffi::c_float;
        a[2 as ::core::ffi::c_int as usize] =
            (a[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                + a[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    * ((*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).slant
                        as ::core::ffi::c_double
                        * 1E-3f64)) as ::core::ffi::c_float;
        a[4 as ::core::ffi::c_int as usize] =
            (a[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                + a[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    * ((*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).slant
                        as ::core::ffi::c_double
                        * 1E-3f64)) as ::core::ffi::c_float;
    }
    if (*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).extend != 0 as ::core::ffi::c_int
    {
        a[0 as ::core::ffi::c_int as usize] = (a[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_double
            * ((*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).extend
                as ::core::ffi::c_double
                * 1E-3f64)) as ::core::ffi::c_float;
        a[2 as ::core::ffi::c_int as usize] = (a[2 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_double
            * ((*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).extend
                as ::core::ffi::c_double
                * 1E-3f64)) as ::core::ffi::c_float;
        a[4 as ::core::ffi::c_int as usize] = (a[4 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_double
            * ((*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).extend
                as ::core::ffi::c_double
                * 1E-3f64)) as ::core::ffi::c_float;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 6 as ::core::ffi::c_int {
        crate::utils::sprintf_args(
            r,
            b"%g \0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                a[i as usize] as ::core::ffi::c_double,
            )],
        );
        r = strchr(r, 0 as ::core::ffi::c_int);
        i += 1;
    }
    if c == '[' as i32 {
        while *p as ::core::ffi::c_int != ']' as i32
            && *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
    } else {
        while *p as ::core::ffi::c_int != '}' as i32
            && *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
    }
    if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        p = strchr(
            (*__tekai_state_generated_backend_writet1).t1_line_array,
            0 as ::core::ffi::c_int,
        )
        .offset(-(1 as ::core::ffi::c_int as isize));
        if *p as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
            *p = 0 as ::core::ffi::c_char;
        }
        crate::utils::pdftex_fail_args(
            b"FontMatrix: cannot find the corresponding character to '%c': `%s'\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(c),
                crate::utils::PrintfArg::from(
                    (*__tekai_state_generated_backend_writet1).t1_line_array,
                ),
            ],
        );
    }
    strcpy(r, p);
    strcpy(
        (*__tekai_state_generated_backend_writet1).t1_line_array as *mut ::core::ffi::c_char,
        (*__tekai_state_generated_backend_writet1).t1_buf_array,
    );
    (*__tekai_state_generated_backend_writet1).t1_line_ptr =
        eol((*__tekai_state_generated_backend_writet1).t1_line_array as *mut ::core::ffi::c_char)
            as *mut t1_line_entry;
}
unsafe fn t1_modify_italic() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let mut a: ::core::ffi::c_float = 0.;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).slant == 0 as ::core::ffi::c_int {
        return;
    }
    p = strchr(
        (*__tekai_state_generated_backend_writet1).t1_line_array,
        ' ' as i32,
    );
    strncpy(
        (*__tekai_state_generated_backend_writet1).t1_buf_array as *mut ::core::ffi::c_char,
        (*__tekai_state_generated_backend_writet1).t1_line_array,
        (p.offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
            as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as size_t,
    );
    a = t1_scan_num(p.offset(1 as ::core::ffi::c_int as isize), &raw mut r);
    a = (a as ::core::ffi::c_double
        - libm::atan(
            (*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).slant
                as ::core::ffi::c_double
                * 1E-3f64,
        ) * (180 as ::core::ffi::c_int as ::core::ffi::c_double / M_PI))
        as ::core::ffi::c_float;
    crate::utils::sprintf_args(
        (*__tekai_state_generated_backend_writet1)
            .t1_buf_array
            .offset(
                (p.offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                    as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ),
        b"%g\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(a as ::core::ffi::c_double)],
    );
    strcpy(
        strchr(
            (*__tekai_state_generated_backend_writet1).t1_buf_array,
            0 as ::core::ffi::c_int,
        ),
        r,
    );
    strcpy(
        (*__tekai_state_generated_backend_writet1).t1_line_array as *mut ::core::ffi::c_char,
        (*__tekai_state_generated_backend_writet1).t1_buf_array,
    );
    (*__tekai_state_generated_backend_writet1).t1_line_ptr =
        eol((*__tekai_state_generated_backend_writet1).t1_line_array as *mut ::core::ffi::c_char)
            as *mut t1_line_entry;
    (*(*__tekai_state_generated_backend_writet1).fd_cur).font_dim[ITALIC_ANGLE_CODE as usize].val =
        zround(a as ::core::ffi::c_double) as ::core::ffi::c_int;
    (*(*__tekai_state_generated_backend_writet1).fd_cur).font_dim[ITALIC_ANGLE_CODE as usize].set =
        true_0 as boolean;
}
unsafe fn t1_scan_keys() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut key: *const key_entry = ::core::ptr::null::<key_entry>();
    if (*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).extend != 0 as ::core::ffi::c_int
        || (*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).slant
            != 0 as ::core::ffi::c_int
    {
        if strncmp(
            (*__tekai_state_generated_backend_writet1).t1_line_array,
            b"/FontMatrix\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"/FontMatrix\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int
        {
            t1_modify_fm();
            return;
        }
        if strncmp(
            (*__tekai_state_generated_backend_writet1).t1_line_array,
            b"/ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"/ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int
        {
            t1_modify_italic();
            return;
        }
    }
    if strncmp(
        (*__tekai_state_generated_backend_writet1).t1_line_array,
        b"/FontType\0" as *const u8 as *const ::core::ffi::c_char,
        strlen(b"/FontType\0" as *const u8 as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int
    {
        p = (*__tekai_state_generated_backend_writet1)
            .t1_line_array
            .offset(strlen(b"FontType\0" as *const u8 as *const ::core::ffi::c_char) as isize)
            .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char;
        i = t1_scan_num(p, ::core::ptr::null_mut::<*mut ::core::ffi::c_char>())
            as ::core::ffi::c_int;
        if i != 1 as ::core::ffi::c_int {
            crate::utils::pdftex_fail_args(
                b"Type%d fonts unsupported by pdfTeX\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(i)],
            );
        }
        return;
    }
    key = &raw const (*__tekai_state_generated_backend_writet1).font_key as *const key_entry;
    while (key.offset_from(
        &raw const (*__tekai_state_generated_backend_writet1).font_key as *const key_entry,
    ) as ::core::ffi::c_long)
        < FONT_KEYS_NUM as ::core::ffi::c_long
    {
        if *(*key).t1name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '\0' as i32
            && strncmp(
                (*__tekai_state_generated_backend_writet1)
                    .t1_line_array
                    .offset(1 as ::core::ffi::c_int as isize),
                (*key).t1name,
                strlen((*key).t1name),
            ) == 0 as ::core::ffi::c_int
        {
            break;
        }
        key = key.offset(1);
    }
    if key.offset_from(
        &raw const (*__tekai_state_generated_backend_writet1).font_key as *const key_entry,
    ) as ::core::ffi::c_long
        == FONT_KEYS_NUM as ::core::ffi::c_long
    {
        return;
    }
    p = (*__tekai_state_generated_backend_writet1)
        .t1_line_array
        .offset(strlen((*key).t1name) as isize)
        .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char;
    if *p as ::core::ffi::c_int == ' ' as i32 {
        p = p.offset(1);
    }
    k = key.offset_from(
        &raw const (*__tekai_state_generated_backend_writet1).font_key as *const key_entry,
    ) as ::core::ffi::c_long as ::core::ffi::c_int;
    if k == FONTNAME_CODE {
        if *p as ::core::ffi::c_int != '/' as i32 {
            p = strchr(
                (*__tekai_state_generated_backend_writet1).t1_line_array,
                0 as ::core::ffi::c_int,
            )
            .offset(-(1 as ::core::ffi::c_int as isize));
            if *p as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
                *p = 0 as ::core::ffi::c_char;
            }
            crate::utils::pdftex_fail_args(
                b"a name expected: `%s'\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(
                    (*__tekai_state_generated_backend_writet1).t1_line_array,
                )],
            );
        }
        p = p.offset(1);
        r = p;
        q = (*__tekai_state_generated_backend_writet1).t1_buf_array as *mut ::core::ffi::c_char;
        while *p as ::core::ffi::c_int != ' ' as i32
            && *p as ::core::ffi::c_int != 10 as ::core::ffi::c_int
        {
            let fresh39 = p;
            p = p.offset(1);
            let fresh40 = q;
            q = q.offset(1);
            *fresh40 = *fresh39;
        }
        *q = 0 as ::core::ffi::c_char;
        if (*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).slant
            != 0 as ::core::ffi::c_int
        {
            crate::utils::sprintf_args(
                q,
                b"-Slant_%i\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(
                    (*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).slant,
                )],
            );
            q = strchr(q, 0 as ::core::ffi::c_int);
        }
        if (*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).extend
            != 0 as ::core::ffi::c_int
        {
            crate::utils::sprintf_args(
                q,
                b"-Extend_%i\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(
                    (*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).extend,
                )],
            );
        }
        if !(*(*__tekai_state_generated_backend_writet1).fd_cur)
            .fontname
            .is_null()
        {
            free(
                (*(*__tekai_state_generated_backend_writet1).fd_cur).fontname
                    as *mut ::core::ffi::c_void,
            );
        }
        (*(*__tekai_state_generated_backend_writet1).fd_cur).fontname =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*(*__tekai_state_generated_backend_writet1).fd_cur).fontname =
            xstrdup((*__tekai_state_generated_backend_writet1).t1_buf_array as const_string)
                as *mut ::core::ffi::c_char;
        if (*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).type_0 as ::core::ffi::c_int
            & F_SUBSETTED
            != 0 as ::core::ffi::c_int
        {
            if !((*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).type_0
                as ::core::ffi::c_int
                & 0x1 as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_long
                != 0
            {
                __assert_rtn(
                    b"t1_scan_keys\0" as *const u8 as *const ::core::ffi::c_char,
                    b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
                    715 as ::core::ffi::c_int,
                    b"is_included(fd_cur->fm)\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
            (*__tekai_state_generated_backend_writet1).t1_fontname_offset =
                (fb_offset() as ::core::ffi::c_long
                    + r.offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                        as ::core::ffi::c_long) as integer;
            strcpy(
                (*__tekai_state_generated_backend_writet1).t1_buf_array as *mut ::core::ffi::c_char,
                p,
            );
            crate::utils::sprintf_args(
                r,
                b"ABCDEF+%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(
                        (*(*__tekai_state_generated_backend_writet1).fd_cur).fontname,
                    ),
                    crate::utils::PrintfArg::from(
                        (*__tekai_state_generated_backend_writet1).t1_buf_array,
                    ),
                ],
            );
            (*__tekai_state_generated_backend_writet1).t1_line_ptr = eol(r) as *mut t1_line_entry;
        }
        return;
    }
    if (k == STEMV_CODE || k == FONTBBOX1_CODE)
        && (*p as ::core::ffi::c_int == '[' as i32 || *p as ::core::ffi::c_int == '{' as i32)
    {
        p = p.offset(1);
    }
    if k == FONTBBOX1_CODE {
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            (*(*__tekai_state_generated_backend_writet1).fd_cur).font_dim[k as usize].val =
                t1_scan_num(p, &raw mut r) as ::core::ffi::c_int;
            (*(*__tekai_state_generated_backend_writet1).fd_cur).font_dim[k as usize].set =
                true_0 as boolean;
            p = r;
            i += 1;
            k += 1;
        }
        return;
    }
    (*(*__tekai_state_generated_backend_writet1).fd_cur).font_dim[k as usize].val =
        t1_scan_num(p, ::core::ptr::null_mut::<*mut ::core::ffi::c_char>()) as ::core::ffi::c_int;
    (*(*__tekai_state_generated_backend_writet1).fd_cur).font_dim[k as usize].set =
        true_0 as boolean;
}
unsafe fn t1_scan_param() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    if (*__tekai_state_generated_backend_writet1).t1_scan == 0
        || *(*__tekai_state_generated_backend_writet1).t1_line_array as ::core::ffi::c_int
            != '/' as i32
    {
        return;
    }
    if strncmp(
        (*__tekai_state_generated_backend_writet1).t1_line_array,
        (*__tekai_state_generated_backend_writet1).local_t1_scan_param_1806_lenIV,
        strlen((*__tekai_state_generated_backend_writet1).local_t1_scan_param_1806_lenIV),
    ) == 0 as ::core::ffi::c_int
    {
        (*__tekai_state_generated_backend_writet1).t1_lenIV = t1_scan_num(
            (*__tekai_state_generated_backend_writet1)
                .t1_line_array
                .offset(strlen(
                    (*__tekai_state_generated_backend_writet1).local_t1_scan_param_1806_lenIV,
                ) as isize),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) as ::core::ffi::c_short;
        if ((*__tekai_state_generated_backend_writet1).t1_lenIV as ::core::ffi::c_int)
            < 0 as ::core::ffi::c_int
        {
            crate::utils::pdftex_fail_args(
                b"negative value of lenIV is not supported\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
        }
        return;
    }
    t1_scan_keys();
}
unsafe fn copy_glyph_names(
    mut glyph_names: *mut *mut ::core::ffi::c_char,
    mut a: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
) {
    let __tekai_state_generated_backend_writettf: *mut crate::generated::backend::writettf::State =
        unsafe { crate::generated::backend::writettf::state() };

    if *glyph_names.offset(b as isize)
        != &raw mut (*__tekai_state_generated_backend_writettf).notdef as *mut ::core::ffi::c_char
    {
        if !(*glyph_names.offset(b as isize)).is_null() {
            free(*glyph_names.offset(b as isize) as *mut ::core::ffi::c_void);
        }
        let ref mut fresh49 = *glyph_names.offset(b as isize);
        *fresh49 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let ref mut fresh50 = *glyph_names.offset(b as isize);
        *fresh50 =
            &raw mut (*__tekai_state_generated_backend_writettf).notdef as *mut ::core::ffi::c_char;
    }
    if *glyph_names.offset(a as isize)
        != &raw mut (*__tekai_state_generated_backend_writettf).notdef as *mut ::core::ffi::c_char
    {
        let ref mut fresh51 = *glyph_names.offset(b as isize);
        *fresh51 =
            xstrdup(*glyph_names.offset(a as isize) as const_string) as *mut ::core::ffi::c_char;
    }
}
unsafe fn t1_written_suffix<'a>(start: *const ::core::ffi::c_char) -> &'a [u8] {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let written = (*__tekai_state_generated_backend_writet1)
        .t1_line_ptr
        .addr()
        .checked_sub(
            (*__tekai_state_generated_backend_writet1)
                .t1_line_array
                .addr(),
        );
    let offset = start.addr().checked_sub(
        (*__tekai_state_generated_backend_writet1)
            .t1_line_array
            .addr(),
    );
    let (Some(written), Some(offset)) = (written, offset) else {
        crate::utils::pdftex_fail_args(c"invalid pfb, invalid initialized line span".as_ptr(), &[]);
    };
    if (*__tekai_state_generated_backend_writet1)
        .t1_line_array
        .is_null()
        || written >= (*__tekai_state_generated_backend_writet1).t1_line_limit
        || offset > written
    {
        crate::utils::pdftex_fail_args(c"invalid pfb, invalid initialized line span".as_ptr(), &[]);
    }
    // The line reader initializes this exact span, including during assembly.
    // Never scan allocation capacity or require a terminator beyond its end.
    &std::slice::from_raw_parts(
        (*__tekai_state_generated_backend_writet1)
            .t1_line_array
            .cast::<u8>(),
        written,
    )[offset..]
}

unsafe fn t1_scan_field<T>(result: Result<T, FieldError>) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(FieldError::NoMatch) => None,
        Err(FieldError::Overflow) => {
            crate::utils::pdftex_fail_args(c"invalid pfb, integer field out of range".as_ptr(), &[])
        }
    }
}

unsafe fn t1_copy_scan_word(bytes: &[u8]) {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    if (*__tekai_state_generated_backend_writet1)
        .t1_buf_array
        .is_null()
        || bytes.len() >= (*__tekai_state_generated_backend_writet1).t1_buf_limit
    {
        crate::utils::pdftex_fail_args(c"invalid pfb, encoding word exceeds buffer".as_ptr(), &[]);
    }
    std::ptr::copy_nonoverlapping(
        bytes.as_ptr(),
        (*__tekai_state_generated_backend_writet1)
            .t1_buf_array
            .cast::<u8>(),
        bytes.len(),
    );
    *(*__tekai_state_generated_backend_writet1)
        .t1_buf_array
        .add(bytes.len()) = 0;
}

unsafe fn t1_builtin_enc() -> *mut *mut ::core::ffi::c_char {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };
    let __tekai_state_generated_backend_writettf: *mut crate::generated::backend::writettf::State =
        unsafe { crate::generated::backend::writettf::state() };

    let mut i: ::core::ffi::c_int = 0;
    let mut a: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut counter: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut glyph_names: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    glyph_names = xmalloc(
        (256 as size_t).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
    ) as *mut *mut ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        let ref mut fresh43 = *glyph_names.offset(i as isize);
        *fresh43 =
            &raw mut (*__tekai_state_generated_backend_writettf).notdef as *mut ::core::ffi::c_char;
        i += 1;
    }
    if str_suffix(
        (*__tekai_state_generated_backend_writet1).t1_line_array,
        (*__tekai_state_generated_backend_writet1).t1_line_ptr,
        b"def\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        let encoding = t1_scan_field(predefined_encoding(t1_written_suffix(
            (*__tekai_state_generated_backend_writet1)
                .t1_line_array
                .add(b"/Encoding".len()),
        )));
        t1_copy_scan_word(encoding.unwrap_or_default());
        if encoding.is_some()
            && strcmp(
                (*__tekai_state_generated_backend_writet1).t1_buf_array,
                b"StandardEncoding\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            (*__tekai_state_generated_backend_writet1).t1_encoding = ENC_STANDARD;
            i = 0 as ::core::ffi::c_int;
            while i < 256 as ::core::ffi::c_int {
                if (*__tekai_state_generated_backend_writet1).standard_glyph_names[i as usize]
                    != &raw mut (*__tekai_state_generated_backend_writettf).notdef
                        as *mut ::core::ffi::c_char
                        as *const ::core::ffi::c_char
                {
                    let ref mut fresh44 = *glyph_names.offset(i as isize);
                    *fresh44 = xstrdup(
                        (*__tekai_state_generated_backend_writet1).standard_glyph_names[i as usize],
                    ) as *mut ::core::ffi::c_char;
                }
                i += 1;
            }
            return glyph_names;
        }
        crate::utils::pdftex_fail_args(
            b"cannot subset font (unknown predefined encoding `%s')\0" as *const u8
                as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                (*__tekai_state_generated_backend_writet1).t1_buf_array,
            )],
        );
    }
    (*__tekai_state_generated_backend_writet1).t1_encoding = ENC_BUILTIN;
    if strncmp(
        (*__tekai_state_generated_backend_writet1).t1_line_array,
        b"/Encoding [\0" as *const u8 as *const ::core::ffi::c_char,
        strlen(b"/Encoding [\0" as *const u8 as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int
        || strncmp(
            (*__tekai_state_generated_backend_writet1).t1_line_array,
            b"/Encoding[\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"/Encoding[\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int
    {
        r = strchr(
            (*__tekai_state_generated_backend_writet1).t1_line_array,
            '[' as i32,
        )
        .offset(1 as ::core::ffi::c_int as isize);
        if *r as ::core::ffi::c_int == ' ' as i32 {
            r = r.offset(1);
        }
        loop {
            while *r as ::core::ffi::c_int == '/' as i32 {
                p = (*__tekai_state_generated_backend_writet1).t1_buf_array
                    as *mut ::core::ffi::c_char;
                r = r.offset(1);
                while *r as ::core::ffi::c_int != 32 as ::core::ffi::c_int
                    && *r as ::core::ffi::c_int != 10 as ::core::ffi::c_int
                    && *r as ::core::ffi::c_int != ']' as i32
                    && *r as ::core::ffi::c_int != '/' as i32
                {
                    let fresh45 = r;
                    r = r.offset(1);
                    let fresh46 = p;
                    p = p.offset(1);
                    *fresh46 = *fresh45;
                }
                *p = 0 as ::core::ffi::c_char;
                if *r as ::core::ffi::c_int == ' ' as i32 {
                    r = r.offset(1);
                }
                if counter > 255 as ::core::ffi::c_int {
                    crate::utils::pdftex_fail_args(
                        b"encoding vector contains more than 256 names\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                }
                if strcmp(
                    (*__tekai_state_generated_backend_writet1).t1_buf_array,
                    &raw mut (*__tekai_state_generated_backend_writettf).notdef
                        as *mut ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                {
                    let ref mut fresh47 = *glyph_names.offset(counter as isize);
                    *fresh47 = xstrdup(
                        (*__tekai_state_generated_backend_writet1).t1_buf_array as const_string,
                    ) as *mut ::core::ffi::c_char;
                }
                counter += 1;
            }
            if *r as ::core::ffi::c_int != 10 as ::core::ffi::c_int
                && *r as ::core::ffi::c_int != '%' as i32
            {
                if strncmp(
                    r,
                    b"] def\0" as *const u8 as *const ::core::ffi::c_char,
                    strlen(b"] def\0" as *const u8 as *const ::core::ffi::c_char),
                ) == 0 as ::core::ffi::c_int
                    || strncmp(
                        r,
                        b"] readonly def\0" as *const u8 as *const ::core::ffi::c_char,
                        strlen(b"] readonly def\0" as *const u8 as *const ::core::ffi::c_char),
                    ) == 0 as ::core::ffi::c_int
                {
                    break;
                }
                r = strchr(
                    (*__tekai_state_generated_backend_writet1).t1_line_array,
                    0 as ::core::ffi::c_int,
                )
                .offset(-(1 as ::core::ffi::c_int as isize));
                if *r as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
                    *r = 0 as ::core::ffi::c_char;
                }
                crate::utils::pdftex_fail_args(
                    b"a name or `] def' or `] readonly def' expected: `%s'\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(
                        (*__tekai_state_generated_backend_writet1).t1_line_array,
                    )],
                );
            } else {
                t1_getline();
                r = (*__tekai_state_generated_backend_writet1).t1_line_array
                    as *mut ::core::ffi::c_char;
            }
        }
    } else {
        p = strchr(
            (*__tekai_state_generated_backend_writet1).t1_line_array,
            10 as ::core::ffi::c_int,
        );
        loop {
            if *p as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
                t1_getline();
                p = (*__tekai_state_generated_backend_writet1).t1_line_array
                    as *mut ::core::ffi::c_char;
            }
            let bytes = t1_written_suffix(p);
            let assignment = t1_scan_field(encoding_assignment(bytes)).filter(|parsed| {
                parsed.name.first() == Some(&b'/') && (0..256).contains(&parsed.index)
            });
            let copy = if assignment.is_none() {
                t1_scan_field(encoding_copy(bytes)).filter(|parsed| {
                    (0..256).contains(&parsed.source) && (0..256).contains(&parsed.target)
                })
            } else {
                None
            };
            let interval = if assignment.is_none() && copy.is_none() {
                t1_scan_field(encoding_interval(bytes)).filter(|parsed| {
                    (0..256).contains(&parsed.source)
                        && (0..256).contains(&parsed.target)
                        && (0..256).contains(&parsed.count)
                })
            } else {
                None
            };
            if let Some(assignment) = assignment {
                i = assignment.index;
                t1_copy_scan_word(assignment.name);
                if strcmp(
                    (*__tekai_state_generated_backend_writet1)
                        .t1_buf_array
                        .offset(1 as ::core::ffi::c_int as isize),
                    &raw mut (*__tekai_state_generated_backend_writettf).notdef
                        as *mut ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                {
                    let ref mut fresh48 = *glyph_names.offset(i as isize);
                    *fresh48 = xstrdup(
                        (*__tekai_state_generated_backend_writet1)
                            .t1_buf_array
                            .offset(1 as ::core::ffi::c_int as isize)
                            as const_string,
                    ) as *mut ::core::ffi::c_char;
                }
                p = strstr(p, b" put\0" as *const u8 as *const ::core::ffi::c_char);
                if p.is_null() {
                    crate::utils::pdftex_fail_args(
                        b"invalid pfb, no put found in dup\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                }
                p = p.offset(strlen(b" put\0" as *const u8 as *const ::core::ffi::c_char) as isize);
                if *p as ::core::ffi::c_int == ' ' as i32 {
                    p = p.offset(1);
                }
            } else if let Some(copy) = copy {
                a = copy.source;
                b = copy.target;
                copy_glyph_names(glyph_names, a, b);
                p = strstr(p, b" get put\0" as *const u8 as *const ::core::ffi::c_char);
                if p.is_null() {
                    crate::utils::pdftex_fail_args(
                        b"invalid pfb, no get put found in dup dup\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                }
                p = p.offset(
                    strlen(b" get put\0" as *const u8 as *const ::core::ffi::c_char) as isize,
                );
                if *p as ::core::ffi::c_int == ' ' as i32 {
                    p = p.offset(1);
                }
            } else if let Some(interval) = interval {
                a = interval.source;
                c = interval.count;
                b = interval.target;
                if checked_span(i64::from(a), i64::from(c), 256).is_none()
                    || checked_span(i64::from(b), i64::from(c), 256).is_none()
                {
                    crate::utils::pdftex_fail_args(
                        b"invalid pfb, encoding interval out of range\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                }
                i = 0 as ::core::ffi::c_int;
                while i < c {
                    copy_glyph_names(glyph_names, a + i, b + i);
                    i += 1;
                }
                p = strstr(
                    p,
                    b" putinterval\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if p.is_null() {
                    crate::utils::pdftex_fail_args(
                        b"invalid pfb, no putinterval found in dup dup\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                }
                p = p.offset(
                    strlen(b" putinterval\0" as *const u8 as *const ::core::ffi::c_char) as isize,
                );
                if *p as ::core::ffi::c_int == ' ' as i32 {
                    p = p.offset(1);
                }
            } else if (p == (*__tekai_state_generated_backend_writet1).t1_line_array
                || p > (*__tekai_state_generated_backend_writet1).t1_line_array
                    && *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                        == ' ' as i32)
                && strcmp(p, b"def\n\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
            {
                return glyph_names;
            } else {
                while *p as ::core::ffi::c_int != ' ' as i32
                    && *p as ::core::ffi::c_int != 10 as ::core::ffi::c_int
                {
                    p = p.offset(1);
                }
                if *p as ::core::ffi::c_int == ' ' as i32 {
                    p = p.offset(1);
                }
            }
        }
    }
    return glyph_names;
}
unsafe fn t1_check_end() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    if feof((*__tekai_state_generated_backend_writet1).t1_file) != 0 {
        return;
    }
    t1_getline();
    if strncmp(
        (*__tekai_state_generated_backend_writet1).t1_line_array,
        b"{restore}\0" as *const u8 as *const ::core::ffi::c_char,
        strlen(b"{restore}\0" as *const u8 as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int
    {
        t1_putline();
    }
}
unsafe fn t1_open_fontfile(mut open_name_prefix: *const ::core::ffi::c_char) -> boolean {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    let mut ff: *mut ff_entry = ::core::ptr::null_mut::<ff_entry>();
    ff = check_ff_exist(
        (*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).ff_name,
        ((*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).type_0 as ::core::ffi::c_int
            & F_TRUETYPE
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
    if !(*ff).ff_path.is_null() {
        (*__tekai_state_utils).cur_file_name = (*ff).ff_path;
        (*__tekai_state_generated_backend_writet1).t1_file = xfopen(
            (*__tekai_state_utils).cur_file_name as const_string,
            FOPEN_RBIN_MODE.as_ptr(),
        );
        recorder_record_input((*ff).ff_path as const_string);
    } else {
        (*__tekai_state_utils).cur_file_name =
            (*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).ff_name;
        zpackfilename(
            maketexstring((*__tekai_state_utils).cur_file_name),
            getnullstr(),
            getnullstr(),
        );
        crate::utils::pdftex_fail_args(
            b"cannot open Type 1 font file for reading\0" as *const u8
                as *const ::core::ffi::c_char,
            &[],
        );
    }
    t1_init_params(open_name_prefix);
    return true_0;
}
unsafe fn t1_include() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    loop {
        t1_getline();
        t1_scan_param();
        t1_putline();
        if !((*__tekai_state_generated_backend_writet1).t1_in_eexec == 0 as ::core::ffi::c_int) {
            break;
        }
    }
    t1_start_eexec();
    loop {
        t1_getline();
        t1_scan_param();
        t1_putline();
        if !strstr(
            (*__tekai_state_generated_backend_writet1).t1_line_array,
            &raw mut (*__tekai_state_generated_backend_writet1).charstringname
                as *mut ::core::ffi::c_char,
        )
        .is_null()
            || strncmp(
                (*__tekai_state_generated_backend_writet1).t1_line_array,
                b"/Subrs\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"/Subrs\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0 as ::core::ffi::c_int
        {
            break;
        }
    }
    (*__tekai_state_generated_backend_writet1).t1_cs = true_0 as boolean;
    loop {
        t1_getline();
        t1_putline();
        if !(str_suffix(
            (*__tekai_state_generated_backend_writet1).t1_line_array,
            (*__tekai_state_generated_backend_writet1).t1_line_ptr,
            b"mark currentfile closefile\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0)
        {
            break;
        }
    }
    t1_stop_eexec();
    (*__tekai_state_generated_backend_writet1).t1_length3 = (if fixedcontent != 0 {
        fb_offset() as ::core::ffi::c_int
            - (*__tekai_state_generated_backend_writet1).t1_save_offset as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as integer;
}
unsafe fn check_cs_token_pair() -> *mut *const ::core::ffi::c_char {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let mut p: *mut *const ::core::ffi::c_char =
        &raw mut (*__tekai_state_generated_backend_writet1).cs_token_pairs_list
            as *mut [*const ::core::ffi::c_char; 2] as *mut *const ::core::ffi::c_char;
    while !(*p.offset(0 as ::core::ffi::c_int as isize)).is_null() {
        if strncmp(
            (*__tekai_state_generated_backend_writet1).t1_buf_array,
            *p.offset(0 as ::core::ffi::c_int as isize),
            strlen(*p.offset(0 as ::core::ffi::c_int as isize)),
        ) == 0 as ::core::ffi::c_int
            && str_suffix(
                (*__tekai_state_generated_backend_writet1).t1_buf_array,
                (*__tekai_state_generated_backend_writet1).t1_buf_ptr,
                *p.offset(1 as ::core::ffi::c_int as isize),
            ) != 0
        {
            return p;
        }
        p = p.offset(1);
    }
    return ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
}
unsafe fn cs_store(mut is_subr: boolean) {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };
    let __tekai_state_generated_backend_writettf: *mut crate::generated::backend::writettf::State =
        unsafe { crate::generated::backend::writettf::state() };

    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ptr: *mut cs_entry = ::core::ptr::null_mut::<cs_entry>();
    let mut subr: ::core::ffi::c_int = 0;
    p = (*__tekai_state_generated_backend_writet1).t1_line_array as *mut ::core::ffi::c_char;
    (*__tekai_state_generated_backend_writet1).t1_buf_ptr =
        (*__tekai_state_generated_backend_writet1).t1_buf_array;
    while *p as ::core::ffi::c_int != ' ' as i32 {
        let fresh33 = p;
        p = p.offset(1);
        let fresh34 = (*__tekai_state_generated_backend_writet1).t1_buf_ptr;
        (*__tekai_state_generated_backend_writet1).t1_buf_ptr =
            (*__tekai_state_generated_backend_writet1)
                .t1_buf_ptr
                .offset(1);
        *fresh34 = *fresh33 as t1_buf_entry;
    }
    *(*__tekai_state_generated_backend_writet1).t1_buf_ptr = 0 as t1_buf_entry;
    if is_subr != 0 {
        subr = t1_scan_num(
            p.offset(1 as ::core::ffi::c_int as isize),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) as ::core::ffi::c_int;
        if subr >= (*__tekai_state_generated_backend_writet1).subr_size
            || subr < 0 as ::core::ffi::c_int
        {
            crate::utils::pdftex_fail_args(
                b"Subrs array: entry index out of range (%i)\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(subr)],
            );
        }
        ptr = (*__tekai_state_generated_backend_writet1)
            .subr_tab
            .offset(subr as isize);
    } else {
        let fresh35 = (*__tekai_state_generated_backend_writet1).cs_ptr;
        (*__tekai_state_generated_backend_writet1).cs_ptr =
            (*__tekai_state_generated_backend_writet1).cs_ptr.offset(1);
        ptr = fresh35;
        if (*__tekai_state_generated_backend_writet1)
            .cs_ptr
            .offset_from((*__tekai_state_generated_backend_writet1).cs_tab)
            as ::core::ffi::c_long
            > (*__tekai_state_generated_backend_writet1).cs_size as ::core::ffi::c_long
        {
            crate::utils::pdftex_fail_args(
                b"CharStrings dict: more entries than dict size (%i)\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(
                    (*__tekai_state_generated_backend_writet1).cs_size,
                )],
            );
        }
        if strcmp(
            (*__tekai_state_generated_backend_writet1)
                .t1_buf_array
                .offset(1 as ::core::ffi::c_int as isize),
            &raw mut (*__tekai_state_generated_backend_writettf).notdef as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*ptr).name = &raw mut (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char;
        } else {
            (*ptr).name = xstrdup(
                (*__tekai_state_generated_backend_writet1)
                    .t1_buf_array
                    .offset(1 as ::core::ffi::c_int as isize) as const_string,
            ) as *mut ::core::ffi::c_char;
        }
    }
    memcpy(
        (*__tekai_state_generated_backend_writet1).t1_buf_array as *mut ::core::ffi::c_void,
        (*__tekai_state_generated_backend_writet1)
            .t1_line_array
            .offset((*__tekai_state_generated_backend_writet1).cs_start as isize)
            .offset(-(4 as ::core::ffi::c_int as isize)) as *const ::core::ffi::c_void,
        ((*__tekai_state_generated_backend_writet1).t1_cslen as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int) as ::core::ffi::c_uint as size_t,
    );
    p = (*__tekai_state_generated_backend_writet1)
        .t1_line_array
        .offset((*__tekai_state_generated_backend_writet1).cs_start as isize)
        .offset((*__tekai_state_generated_backend_writet1).t1_cslen as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_char;
    (*__tekai_state_generated_backend_writet1).t1_buf_ptr =
        (*__tekai_state_generated_backend_writet1)
            .t1_buf_array
            .offset(
                (*__tekai_state_generated_backend_writet1).t1_cslen as ::core::ffi::c_int as isize,
            )
            .offset(4 as ::core::ffi::c_int as isize);
    while *p as ::core::ffi::c_int != 10 as ::core::ffi::c_int {
        let fresh36 = p;
        p = p.offset(1);
        let fresh37 = (*__tekai_state_generated_backend_writet1).t1_buf_ptr;
        (*__tekai_state_generated_backend_writet1).t1_buf_ptr =
            (*__tekai_state_generated_backend_writet1)
                .t1_buf_ptr
                .offset(1);
        *fresh37 = *fresh36 as t1_buf_entry;
    }
    let fresh38 = (*__tekai_state_generated_backend_writet1).t1_buf_ptr;
    (*__tekai_state_generated_backend_writet1).t1_buf_ptr =
        (*__tekai_state_generated_backend_writet1)
            .t1_buf_ptr
            .offset(1);
    *fresh38 = 10 as t1_buf_entry;
    if is_subr != 0
        && (*__tekai_state_generated_backend_writet1)
            .cs_token_pair
            .is_null()
    {
        (*__tekai_state_generated_backend_writet1).cs_token_pair = check_cs_token_pair();
    }
    (*ptr).len = (*__tekai_state_generated_backend_writet1)
        .t1_buf_ptr
        .offset_from((*__tekai_state_generated_backend_writet1).t1_buf_array)
        as ::core::ffi::c_long as ::core::ffi::c_ushort;
    (*ptr).cslen = (*__tekai_state_generated_backend_writet1).t1_cslen;
    (*ptr).data =
        xmalloc(((*ptr).len as size_t).wrapping_mul(::core::mem::size_of::<byte>() as size_t))
            as *mut byte;
    memcpy(
        (*ptr).data as *mut ::core::ffi::c_void,
        (*__tekai_state_generated_backend_writet1).t1_buf_array as *const ::core::ffi::c_void,
        (*ptr).len as size_t,
    );
    (*ptr).valid = true_0 as boolean;
}

unsafe fn cc_init() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let mut i: ::core::ffi::c_int = 0;
    if (*__tekai_state_generated_backend_writet1).is_cc_init != 0 {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < CS_MAX {
        (*__tekai_state_generated_backend_writet1).cc_tab[i as usize].valid = false_0 as boolean;
        i += 1;
    }
    (*__tekai_state_generated_backend_writet1).cc_tab[1 as ::core::ffi::c_int as usize].nargs =
        2 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[1 as ::core::ffi::c_int as usize].bottom =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[1 as ::core::ffi::c_int as usize].clear =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[1 as ::core::ffi::c_int as usize].valid =
        true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[3 as ::core::ffi::c_int as usize].nargs =
        2 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[3 as ::core::ffi::c_int as usize].bottom =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[3 as ::core::ffi::c_int as usize].clear =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[3 as ::core::ffi::c_int as usize].valid =
        true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[4 as ::core::ffi::c_int as usize].nargs =
        1 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[4 as ::core::ffi::c_int as usize].bottom =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[4 as ::core::ffi::c_int as usize].clear =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[4 as ::core::ffi::c_int as usize].valid =
        true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[5 as ::core::ffi::c_int as usize].nargs =
        2 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[5 as ::core::ffi::c_int as usize].bottom =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[5 as ::core::ffi::c_int as usize].clear =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[5 as ::core::ffi::c_int as usize].valid =
        true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[6 as ::core::ffi::c_int as usize].nargs =
        1 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[6 as ::core::ffi::c_int as usize].bottom =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[6 as ::core::ffi::c_int as usize].clear =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[6 as ::core::ffi::c_int as usize].valid =
        true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[7 as ::core::ffi::c_int as usize].nargs =
        1 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[7 as ::core::ffi::c_int as usize].bottom =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[7 as ::core::ffi::c_int as usize].clear =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[7 as ::core::ffi::c_int as usize].valid =
        true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[8 as ::core::ffi::c_int as usize].nargs =
        6 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[8 as ::core::ffi::c_int as usize].bottom =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[8 as ::core::ffi::c_int as usize].clear =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[8 as ::core::ffi::c_int as usize].valid =
        true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[9 as ::core::ffi::c_int as usize].nargs =
        0 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[9 as ::core::ffi::c_int as usize].bottom =
        0 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[9 as ::core::ffi::c_int as usize].clear =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[9 as ::core::ffi::c_int as usize].valid =
        true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[10 as ::core::ffi::c_int as usize].nargs =
        1 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[10 as ::core::ffi::c_int as usize].bottom =
        0 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[10 as ::core::ffi::c_int as usize].clear =
        0 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[10 as ::core::ffi::c_int as usize].valid =
        true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[11 as ::core::ffi::c_int as usize].nargs =
        0 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[11 as ::core::ffi::c_int as usize].bottom =
        0 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[11 as ::core::ffi::c_int as usize].clear =
        0 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[11 as ::core::ffi::c_int as usize].valid =
        true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[13 as ::core::ffi::c_int as usize].nargs =
        2 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[13 as ::core::ffi::c_int as usize].bottom =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[13 as ::core::ffi::c_int as usize].clear =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[13 as ::core::ffi::c_int as usize].valid =
        true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[14 as ::core::ffi::c_int as usize].nargs =
        0 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[14 as ::core::ffi::c_int as usize].bottom =
        0 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[14 as ::core::ffi::c_int as usize].clear =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[14 as ::core::ffi::c_int as usize].valid =
        true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[21 as ::core::ffi::c_int as usize].nargs =
        2 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[21 as ::core::ffi::c_int as usize].bottom =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[21 as ::core::ffi::c_int as usize].clear =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[21 as ::core::ffi::c_int as usize].valid =
        true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[22 as ::core::ffi::c_int as usize].nargs =
        1 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[22 as ::core::ffi::c_int as usize].bottom =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[22 as ::core::ffi::c_int as usize].clear =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[22 as ::core::ffi::c_int as usize].valid =
        true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[30 as ::core::ffi::c_int as usize].nargs =
        4 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[30 as ::core::ffi::c_int as usize].bottom =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[30 as ::core::ffi::c_int as usize].clear =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[30 as ::core::ffi::c_int as usize].valid =
        true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[31 as ::core::ffi::c_int as usize].nargs =
        4 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[31 as ::core::ffi::c_int as usize].bottom =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[31 as ::core::ffi::c_int as usize].clear =
        1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[31 as ::core::ffi::c_int as usize].valid =
        true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
        .nargs = 0 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
        .bottom = 0 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
        .clear = 1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
        .valid = true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
        .nargs = 6 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
        .bottom = 1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
        .clear = 1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
        .valid = true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
        .nargs = 6 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
        .bottom = 1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
        .clear = 1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
        .valid = true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as usize]
        .nargs = 5 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as usize]
        .bottom = 1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as usize]
        .clear = 1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as usize]
        .valid = true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as usize]
        .nargs = 4 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as usize]
        .bottom = 1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as usize]
        .clear = 1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab
        [(31 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as usize]
        .valid = true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[(31 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 12 as ::core::ffi::c_int)
        as usize]
        .nargs = 2 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[(31 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 12 as ::core::ffi::c_int)
        as usize]
        .bottom = 0 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[(31 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 12 as ::core::ffi::c_int)
        as usize]
        .clear = 0 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[(31 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 12 as ::core::ffi::c_int)
        as usize]
        .valid = true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[(31 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 16 as ::core::ffi::c_int)
        as usize]
        .nargs = 0 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[(31 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 16 as ::core::ffi::c_int)
        as usize]
        .bottom = 0 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[(31 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 16 as ::core::ffi::c_int)
        as usize]
        .clear = 0 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[(31 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 16 as ::core::ffi::c_int)
        as usize]
        .valid = true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[(31 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 17 as ::core::ffi::c_int)
        as usize]
        .nargs = 0 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[(31 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 17 as ::core::ffi::c_int)
        as usize]
        .bottom = 0 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[(31 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 17 as ::core::ffi::c_int)
        as usize]
        .clear = 0 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[(31 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 17 as ::core::ffi::c_int)
        as usize]
        .valid = true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[(31 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 33 as ::core::ffi::c_int)
        as usize]
        .nargs = 2 as byte;
    (*__tekai_state_generated_backend_writet1).cc_tab[(31 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 33 as ::core::ffi::c_int)
        as usize]
        .bottom = 1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[(31 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 33 as ::core::ffi::c_int)
        as usize]
        .clear = 1 as ::core::ffi::c_int as boolean;
    (*__tekai_state_generated_backend_writet1).cc_tab[(31 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 33 as ::core::ffi::c_int)
        as usize]
        .valid = true_0 as boolean;
    (*__tekai_state_generated_backend_writet1).is_cc_init = true_0 as boolean;
}
unsafe fn cs_fail(
    mut cs_name: *const ::core::ffi::c_char,
    mut subr: ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut arg1: ::core::ffi::c_int,
    mut arg2: ::core::ffi::c_int,
) {
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    crate::utils::sprintf_args(
        &raw mut buf as *mut ::core::ffi::c_char,
        fmt,
        &[
            crate::utils::PrintfArg::from(arg1),
            crate::utils::PrintfArg::from(arg2),
        ],
    );
    if cs_name.is_null() {
        crate::utils::pdftex_fail_args(
            b"Subr (%i): %s\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(subr),
                crate::utils::PrintfArg::from(&raw mut buf as *mut ::core::ffi::c_char),
            ],
        );
    } else {
        crate::utils::pdftex_fail_args(
            b"CharString (/%s): %s\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(cs_name),
                crate::utils::PrintfArg::from(&raw mut buf as *mut ::core::ffi::c_char),
            ],
        );
    };
}
unsafe fn append_cs_return(mut ptr: *mut cs_entry) {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let mut cr: ::core::ffi::c_ushort = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut p: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut q: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut data: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut new_data: *mut byte = ::core::ptr::null_mut::<byte>();
    if !(!ptr.is_null() && (*ptr).valid != 0 && (*ptr).used != 0) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"append_cs_return\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            1108 as ::core::ffi::c_int,
            b"ptr != NULL && ptr->valid && ptr->used\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    p = (*__tekai_state_generated_backend_writet1).t1_buf_array as *mut byte;
    data = (*ptr).data.offset(4 as ::core::ffi::c_int as isize);
    cr = 4330 as ::core::ffi::c_ushort;
    i = 0 as ::core::ffi::c_int;
    while i < (*ptr).cslen as ::core::ffi::c_int {
        let fresh29 = data;
        data = data.offset(1);
        let fresh30 = p;
        p = p.offset(1);
        *fresh30 = cdecrypt(*fresh29, &raw mut cr);
        i += 1;
    }
    *p = CS_RETURN as byte;
    new_data = xmalloc(
        (((*ptr).len as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<byte>() as size_t),
    ) as *mut byte;
    memcpy(
        new_data as *mut ::core::ffi::c_void,
        (*ptr).data as *const ::core::ffi::c_void,
        4 as size_t,
    );
    p = new_data.offset(4 as ::core::ffi::c_int as isize);
    q = (*__tekai_state_generated_backend_writet1).t1_buf_array as *mut byte;
    cr = 4330 as ::core::ffi::c_ushort;
    i = 0 as ::core::ffi::c_int;
    while i < (*ptr).cslen as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
        let fresh31 = q;
        q = q.offset(1);
        let fresh32 = p;
        p = p.offset(1);
        *fresh32 = cencrypt(*fresh31, &raw mut cr);
        i += 1;
    }
    memcpy(
        p as *mut ::core::ffi::c_void,
        (*ptr)
            .data
            .offset(4 as ::core::ffi::c_int as isize)
            .offset((*ptr).cslen as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        ((*ptr).len as ::core::ffi::c_int
            - (*ptr).cslen as ::core::ffi::c_int
            - 4 as ::core::ffi::c_int) as size_t,
    );
    if !(*ptr).data.is_null() {
        free((*ptr).data as *mut ::core::ffi::c_void);
    }
    (*ptr).data = ::core::ptr::null_mut::<byte>();
    (*ptr).data = new_data;
    (*ptr).len = (*ptr).len.wrapping_add(1);
    (*ptr).cslen = (*ptr).cslen.wrapping_add(1);
}
unsafe fn cs_mark(mut cs_name: *const ::core::ffi::c_char, mut subr: ::core::ffi::c_int) {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };
    let __tekai_state_generated_backend_writettf: *mut crate::generated::backend::writettf::State =
        unsafe { crate::generated::backend::writettf::state() };

    let mut current_block: u64;
    let mut data: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut i: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut cs_len: ::core::ffi::c_int = 0;
    let mut last_cmd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut a: integer = 0;
    let mut a1: integer = 0;
    let mut a2: integer = 0;
    let mut cr: ::core::ffi::c_ushort = 0;

    let mut ptr: *mut cs_entry = ::core::ptr::null_mut::<cs_entry>();
    let mut cc: *mut cc_entry = ::core::ptr::null_mut::<cc_entry>();
    if cs_name.is_null() {
        if subr >= (*__tekai_state_generated_backend_writet1).subr_size
            || subr < 0 as ::core::ffi::c_int
        {
            crate::utils::pdftex_fail_args(
                b"Subrs array: entry index out of range (%i)\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(subr)],
            );
        }
        ptr = (*__tekai_state_generated_backend_writet1)
            .subr_tab
            .offset(subr as isize);
        if (*ptr).valid == 0 {
            return;
        }
    } else if !(*__tekai_state_generated_backend_writet1)
        .cs_notdef
        .is_null()
        && (cs_name
            == &raw mut (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char
            || strcmp(
                cs_name,
                &raw mut (*__tekai_state_generated_backend_writettf).notdef
                    as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int)
    {
        ptr = (*__tekai_state_generated_backend_writet1).cs_notdef;
    } else {
        ptr = (*__tekai_state_generated_backend_writet1).cs_tab;
        while ptr < (*__tekai_state_generated_backend_writet1).cs_ptr {
            if strcmp((*ptr).name, cs_name) == 0 as ::core::ffi::c_int {
                break;
            }
            ptr = ptr.offset(1);
        }
        if ptr == (*__tekai_state_generated_backend_writet1).cs_ptr {
            crate::utils::pdftex_warn_args(
                b"glyph `%s' undefined\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(cs_name)],
            );
            return;
        }
        if (*ptr).name
            == &raw mut (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char
        {
            (*__tekai_state_generated_backend_writet1).cs_notdef = ptr;
        }
    }
    if (*ptr).valid == 0 || (*ptr).used != 0 && !cs_name.is_null() {
        return;
    }
    (*ptr).used = true_0 as boolean;
    cr = 4330 as ::core::ffi::c_ushort;
    cs_len = (*ptr).cslen as ::core::ffi::c_int;
    data = (*ptr).data.offset(4 as ::core::ffi::c_int as isize);
    i = 0 as ::core::ffi::c_int;
    while i < (*__tekai_state_generated_backend_writet1).t1_lenIV as ::core::ffi::c_int {
        let fresh17 = data;
        data = data.offset(1);
        cdecrypt(*fresh17, &raw mut cr);
        i += 1;
        cs_len -= 1;
    }
    loop {
        if !(cs_len > 0 as ::core::ffi::c_int) {
            current_block = 1852451392920375136;
            break;
        }
        cs_len -= 1;
        let fresh18 = data;
        data = data.offset(1);
        b = cdecrypt(*fresh18, &raw mut cr) as ::core::ffi::c_int;
        if b >= 32 as ::core::ffi::c_int {
            if b <= 246 as ::core::ffi::c_int {
                a = (b - 139 as ::core::ffi::c_int) as integer;
            } else if b <= 250 as ::core::ffi::c_int {
                cs_len -= 1;
                let fresh19 = data;
                data = data.offset(1);
                a = (((b - 247 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    + 108 as ::core::ffi::c_int
                    + cdecrypt(*fresh19, &raw mut cr) as ::core::ffi::c_int)
                    as integer;
            } else if b <= 254 as ::core::ffi::c_int {
                cs_len -= 1;
                let fresh20 = data;
                data = data.offset(1);
                a = (-((b - 251 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    - 108 as ::core::ffi::c_int
                    - cdecrypt(*fresh20, &raw mut cr) as ::core::ffi::c_int)
                    as integer;
            } else {
                cs_len -= 4 as ::core::ffi::c_int;
                let fresh21 = data;
                data = data.offset(1);
                a = ((cdecrypt(*fresh21, &raw mut cr) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int)
                    << 24 as ::core::ffi::c_int) as integer;
                let fresh22 = data;
                data = data.offset(1);
                a |= (cdecrypt(*fresh22, &raw mut cr) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int)
                    << 16 as ::core::ffi::c_int;
                let fresh23 = data;
                data = data.offset(1);
                a |= (cdecrypt(*fresh23, &raw mut cr) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int;
                let fresh24 = data;
                data = data.offset(1);
                a |= (cdecrypt(*fresh24, &raw mut cr) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int;
                if ::core::mem::size_of::<integer>() as usize > 4 as usize
                    && a as ::core::ffi::c_uint & 0x80000000 as ::core::ffi::c_uint != 0
                {
                    a |= !(0x7fffffff as ::core::ffi::c_int);
                }
            }
            let fresh25 = (*__tekai_state_generated_backend_writet1).stack_ptr;
            (*__tekai_state_generated_backend_writet1).stack_ptr =
                (*__tekai_state_generated_backend_writet1)
                    .stack_ptr
                    .offset(1);
            *fresh25 = a;
        } else {
            if b == CS_ESCAPE {
                let fresh26 = data;
                data = data.offset(1);
                b = cdecrypt(*fresh26, &raw mut cr) as ::core::ffi::c_int + CS_1BYTE_MAX;
                cs_len -= 1;
            }
            if b >= CS_MAX {
                cs_fail(
                    cs_name,
                    subr,
                    b"command value out of range: %i\0" as *const u8 as *const ::core::ffi::c_char,
                    b,
                    0 as ::core::ffi::c_int,
                );
                current_block = 6766438951925407704;
                break;
            } else {
                cc = (&raw mut (*__tekai_state_generated_backend_writet1).cc_tab as *mut cc_entry)
                    .offset(b as isize);
                if (*cc).valid == 0 {
                    cs_fail(
                        cs_name,
                        subr,
                        b"command not valid: %i\0" as *const u8 as *const ::core::ffi::c_char,
                        b,
                        0 as ::core::ffi::c_int,
                    );
                    current_block = 6766438951925407704;
                    break;
                } else {
                    if (*cc).bottom != 0 {
                        if ((*__tekai_state_generated_backend_writet1)
                            .stack_ptr
                            .offset_from(
                                &raw mut (*__tekai_state_generated_backend_writet1).cc_stack
                                    as *mut integer,
                            ) as ::core::ffi::c_long)
                            < (*cc).nargs as ::core::ffi::c_long
                        {
                            cs_fail(
                                cs_name,
                                subr,
                                b"less arguments on stack (%i) than required (%i)\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*__tekai_state_generated_backend_writet1)
                                    .stack_ptr
                                    .offset_from(
                                        &raw mut (*__tekai_state_generated_backend_writet1).cc_stack
                                            as *mut integer,
                                    ) as ::core::ffi::c_long
                                    as ::core::ffi::c_int,
                                (*cc).nargs as ::core::ffi::c_int,
                            );
                        } else if (*__tekai_state_generated_backend_writet1)
                            .stack_ptr
                            .offset_from(
                                &raw mut (*__tekai_state_generated_backend_writet1).cc_stack
                                    as *mut integer,
                            ) as ::core::ffi::c_long
                            > (*cc).nargs as ::core::ffi::c_long
                        {
                            cs_fail(
                                cs_name,
                                subr,
                                b"more arguments on stack (%i) than required (%i)\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*__tekai_state_generated_backend_writet1)
                                    .stack_ptr
                                    .offset_from(
                                        &raw mut (*__tekai_state_generated_backend_writet1).cc_stack
                                            as *mut integer,
                                    ) as ::core::ffi::c_long
                                    as ::core::ffi::c_int,
                                (*cc).nargs as ::core::ffi::c_int,
                            );
                        }
                    }
                    last_cmd = b;
                    match cc.offset_from(
                        &raw mut (*__tekai_state_generated_backend_writet1).cc_tab as *mut cc_entry,
                    ) as ::core::ffi::c_long
                    {
                        10 => {
                            a1 = if -(1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                                *(*__tekai_state_generated_backend_writet1)
                                    .stack_ptr
                                    .offset(-(1 as ::core::ffi::c_int) as isize)
                            } else {
                                *(&raw mut (*__tekai_state_generated_backend_writet1).cc_stack
                                    as *mut integer)
                                    .offset(-(1 as ::core::ffi::c_int) as isize)
                            };
                            if ((*__tekai_state_generated_backend_writet1)
                                .stack_ptr
                                .offset_from(
                                    &raw mut (*__tekai_state_generated_backend_writet1).cc_stack
                                        as *mut integer,
                                ) as ::core::ffi::c_long)
                                < 1 as ::core::ffi::c_long
                            {
                                crate::utils::pdftex_fail_args(
                                    b"CharString: invalid access (%i) to stack (%i entries)\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                    &[
                                        crate::utils::PrintfArg::from(1 as ::core::ffi::c_int),
                                        crate::utils::PrintfArg::from(
                                            (*__tekai_state_generated_backend_writet1)
                                                .stack_ptr
                                                .offset_from(
                                                &raw mut (*__tekai_state_generated_backend_writet1)
                                                    .cc_stack
                                                    as *mut integer,
                                            )
                                                as ::core::ffi::c_long
                                                as ::core::ffi::c_int,
                                        ),
                                    ],
                                );
                            } else {
                                (*__tekai_state_generated_backend_writet1).stack_ptr =
                                    (*__tekai_state_generated_backend_writet1)
                                        .stack_ptr
                                        .offset(-(1 as ::core::ffi::c_int as isize));
                                cs_mark(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    a1 as ::core::ffi::c_int,
                                );
                                if !((*(*__tekai_state_generated_backend_writet1)
                                    .subr_tab
                                    .offset(a1 as isize))
                                .valid
                                    == 0)
                                {
                                    continue;
                                }
                                cs_fail(
                                    cs_name,
                                    subr,
                                    b"cannot call subr (%i)\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    a1,
                                    0 as ::core::ffi::c_int,
                                );
                                current_block = 6766438951925407704;
                                break;
                            }
                        }
                        44 => {
                            if ((*__tekai_state_generated_backend_writet1)
                                .stack_ptr
                                .offset_from(
                                    &raw mut (*__tekai_state_generated_backend_writet1).cc_stack
                                        as *mut integer,
                                ) as ::core::ffi::c_long)
                                < 2 as ::core::ffi::c_long
                            {
                                crate::utils::pdftex_fail_args(
                                    b"CharString: invalid access (%i) to stack (%i entries)\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                    &[
                                        crate::utils::PrintfArg::from(2 as ::core::ffi::c_int),
                                        crate::utils::PrintfArg::from(
                                            (*__tekai_state_generated_backend_writet1)
                                                .stack_ptr
                                                .offset_from(
                                                &raw mut (*__tekai_state_generated_backend_writet1)
                                                    .cc_stack
                                                    as *mut integer,
                                            )
                                                as ::core::ffi::c_long
                                                as ::core::ffi::c_int,
                                        ),
                                    ],
                                );
                            } else {
                                (*__tekai_state_generated_backend_writet1).stack_ptr =
                                    (*__tekai_state_generated_backend_writet1)
                                        .stack_ptr
                                        .offset(-(2 as ::core::ffi::c_int as isize));
                                let fresh27 = (*__tekai_state_generated_backend_writet1).stack_ptr;
                                (*__tekai_state_generated_backend_writet1).stack_ptr =
                                    (*__tekai_state_generated_backend_writet1)
                                        .stack_ptr
                                        .offset(1);
                                *fresh27 = 0 as ::core::ffi::c_int as integer;
                            }
                        }
                        48 => {
                            if (if -(1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                                *(*__tekai_state_generated_backend_writet1)
                                    .stack_ptr
                                    .offset(-(1 as ::core::ffi::c_int) as isize)
                            } else {
                                *(&raw mut (*__tekai_state_generated_backend_writet1).cc_stack
                                    as *mut integer)
                                    .offset(-(1 as ::core::ffi::c_int) as isize)
                            }) == 3 as ::core::ffi::c_int
                            {
                                (*__tekai_state_generated_backend_writet1)
                                    .local_cs_mark_2648_lastargOtherSubr3 =
                                    if -(3 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                                        *(*__tekai_state_generated_backend_writet1)
                                            .stack_ptr
                                            .offset(-(3 as ::core::ffi::c_int) as isize)
                                    } else {
                                        *(&raw mut (*__tekai_state_generated_backend_writet1)
                                            .cc_stack
                                            as *mut integer)
                                            .offset(-(3 as ::core::ffi::c_int) as isize)
                                    };
                            }
                            a1 = ((if -(2 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                                *(*__tekai_state_generated_backend_writet1)
                                    .stack_ptr
                                    .offset(-(2 as ::core::ffi::c_int) as isize)
                            } else {
                                *(&raw mut (*__tekai_state_generated_backend_writet1).cc_stack
                                    as *mut integer)
                                    .offset(-(2 as ::core::ffi::c_int) as isize)
                            }) + 2 as ::core::ffi::c_int)
                                as integer;
                            if ((*__tekai_state_generated_backend_writet1)
                                .stack_ptr
                                .offset_from(
                                    &raw mut (*__tekai_state_generated_backend_writet1).cc_stack
                                        as *mut integer,
                                ) as ::core::ffi::c_long)
                                < a1 as ::core::ffi::c_long
                            {
                                crate::utils::pdftex_fail_args(
                                    b"CharString: invalid access (%i) to stack (%i entries)\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                    &[
                                        crate::utils::PrintfArg::from(a1),
                                        crate::utils::PrintfArg::from(
                                            (*__tekai_state_generated_backend_writet1)
                                                .stack_ptr
                                                .offset_from(
                                                &raw mut (*__tekai_state_generated_backend_writet1)
                                                    .cc_stack
                                                    as *mut integer,
                                            )
                                                as ::core::ffi::c_long
                                                as ::core::ffi::c_int,
                                        ),
                                    ],
                                );
                            } else {
                                (*__tekai_state_generated_backend_writet1).stack_ptr =
                                    (*__tekai_state_generated_backend_writet1)
                                        .stack_ptr
                                        .offset(-(a1 as isize));
                            }
                        }
                        49 => {
                            let fresh28 = (*__tekai_state_generated_backend_writet1).stack_ptr;
                            (*__tekai_state_generated_backend_writet1).stack_ptr =
                                (*__tekai_state_generated_backend_writet1)
                                    .stack_ptr
                                    .offset(1);
                            *fresh28 = (*__tekai_state_generated_backend_writet1)
                                .local_cs_mark_2648_lastargOtherSubr3;
                        }
                        38 => {
                            a1 = if (3 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                                *(*__tekai_state_generated_backend_writet1)
                                    .stack_ptr
                                    .offset(3 as ::core::ffi::c_int as isize)
                            } else {
                                *(&raw mut (*__tekai_state_generated_backend_writet1).cc_stack
                                    as *mut integer)
                                    .offset(3 as ::core::ffi::c_int as isize)
                            };
                            a2 = if (4 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                                *(*__tekai_state_generated_backend_writet1)
                                    .stack_ptr
                                    .offset(4 as ::core::ffi::c_int as isize)
                            } else {
                                *(&raw mut (*__tekai_state_generated_backend_writet1).cc_stack
                                    as *mut integer)
                                    .offset(4 as ::core::ffi::c_int as isize)
                            };
                            (*__tekai_state_generated_backend_writet1).stack_ptr =
                                &raw mut (*__tekai_state_generated_backend_writet1).cc_stack
                                    as *mut integer;
                            cs_mark(
                                (*__tekai_state_generated_backend_writet1).standard_glyph_names
                                    [a1 as usize],
                                0 as ::core::ffi::c_int,
                            );
                            cs_mark(
                                (*__tekai_state_generated_backend_writet1).standard_glyph_names
                                    [a2 as usize],
                                0 as ::core::ffi::c_int,
                            );
                            if !(*(*__tekai_state_generated_backend_writet1).fd_cur)
                                .gl_tree
                                .is_null()
                            {
                                avl_probe(
                                    (*(*__tekai_state_generated_backend_writet1).fd_cur).gl_tree,
                                    (*__tekai_state_generated_backend_writet1).standard_glyph_names
                                        [a1 as usize]
                                        as *mut ::core::ffi::c_void,
                                );
                                avl_probe(
                                    (*(*__tekai_state_generated_backend_writet1).fd_cur).gl_tree,
                                    (*__tekai_state_generated_backend_writet1).standard_glyph_names
                                        [a2 as usize]
                                        as *mut ::core::ffi::c_void,
                                );
                            }
                        }
                        _ => {
                            if (*cc).clear != 0 {
                                (*__tekai_state_generated_backend_writet1).stack_ptr =
                                    &raw mut (*__tekai_state_generated_backend_writet1).cc_stack
                                        as *mut integer;
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        6766438951925407704 => {
            (*__tekai_state_generated_backend_writet1).stack_ptr =
                &raw mut (*__tekai_state_generated_backend_writet1).cc_stack as *mut integer;
            (*ptr).valid = false_0 as boolean;
            (*ptr).used = false_0 as boolean;
            return;
        }
        _ => {
            if cs_name.is_null() && last_cmd != CS_RETURN {
                crate::utils::pdftex_warn_args(b"last command in subr `%i' is not a RETURN; I will add it now but please consider fixing the font\0"
                        as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(subr)]);
                append_cs_return(ptr);
            }
            return;
        }
    };
}
unsafe fn comp_t1_glyphs(
    mut pa: *const ::core::ffi::c_void,
    mut pb: *const ::core::ffi::c_void,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        *(pa as *const *const ::core::ffi::c_char),
        *(pb as *const *const ::core::ffi::c_char),
    );
}
unsafe fn create_t1_glyph_tree(mut glyph_names: *mut *mut ::core::ffi::c_char) -> *mut avl_table {
    let __tekai_state_generated_backend_avlstuff: *mut crate::generated::backend::avlstuff::State =
        unsafe { crate::generated::backend::avlstuff::state() };
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };
    let __tekai_state_generated_backend_writettf: *mut crate::generated::backend::writettf::State =
        unsafe { crate::generated::backend::writettf::state() };

    let mut i: ::core::ffi::c_int = 0;
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();

    (*__tekai_state_generated_backend_writet1).local_create_t1_glyph_tree_3011_gl_tree = avl_create(
        Some(
            comp_t1_glyphs
                as unsafe fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        NULL,
        &raw mut (*__tekai_state_generated_backend_avlstuff).avl_xallocator,
    );
    if (*__tekai_state_generated_backend_writet1)
        .local_create_t1_glyph_tree_3011_gl_tree
        .is_null() as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"create_t1_glyph_tree\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            1299 as ::core::ffi::c_int,
            b"gl_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        if *glyph_names.offset(i as isize)
            != &raw mut (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char
            && (avl_find(
                (*__tekai_state_generated_backend_writet1).local_create_t1_glyph_tree_3011_gl_tree,
                glyph_names.offset(i as isize) as *mut *mut ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
            ) as *mut *mut ::core::ffi::c_char)
                .is_null()
        {
            aa = avl_probe(
                (*__tekai_state_generated_backend_writet1).local_create_t1_glyph_tree_3011_gl_tree,
                glyph_names.offset(i as isize) as *mut *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
            );
            if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                __assert_rtn(
                    b"create_t1_glyph_tree\0" as *const u8 as *const ::core::ffi::c_char,
                    b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1305 as ::core::ffi::c_int,
                    b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
        }
        i += 1;
    }
    return (*__tekai_state_generated_backend_writet1).local_create_t1_glyph_tree_3011_gl_tree;
}
unsafe fn destroy_t1_glyph_tree(mut gl_tree: *mut avl_table) {
    if gl_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"destroy_t1_glyph_tree\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            1313 as ::core::ffi::c_int,
            b"gl_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    avl_destroy(gl_tree, None);
}
unsafe fn t1_subset_ascii_part() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    let mut j: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut glyph: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut gg: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut glyph_names: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut gl_tree: *mut avl_table = ::core::ptr::null_mut::<avl_table>();
    let mut t: avl_traverser = avl_traverser {
        avl_table: ::core::ptr::null_mut::<avl_table>(),
        avl_node: ::core::ptr::null_mut::<avl_node>(),
        avl_stack: [::core::ptr::null_mut::<avl_node>(); 32],
        avl_height: 0,
        avl_generation: 0,
    };
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    if (*__tekai_state_generated_backend_writet1).fd_cur.is_null() as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"t1_subset_ascii_part\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            1326 as ::core::ffi::c_int,
            b"fd_cur != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if (*(*__tekai_state_generated_backend_writet1).fd_cur)
        .gl_tree
        .is_null() as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"t1_subset_ascii_part\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            1327 as ::core::ffi::c_int,
            b"fd_cur->gl_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    t1_getline();
    while !(strncmp(
        (*__tekai_state_generated_backend_writet1).t1_line_array,
        b"/Encoding\0" as *const u8 as *const ::core::ffi::c_char,
        strlen(b"/Encoding\0" as *const u8 as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int)
    {
        t1_scan_param();
        if !(strncmp(
            (*__tekai_state_generated_backend_writet1).t1_line_array,
            b"/UniqueID\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"/UniqueID\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int
            && strncmp(
                (*__tekai_state_generated_backend_writet1)
                    .t1_line_array
                    .offset(
                        strlen((*__tekai_state_generated_backend_writet1).t1_line_array) as isize,
                    )
                    .offset(-(4 as ::core::ffi::c_int as isize)),
                b"def\0" as *const u8 as *const ::core::ffi::c_char,
                3 as size_t,
            ) == 0)
        {
            t1_putline();
        }
        t1_getline();
    }
    glyph_names = t1_builtin_enc();
    (*(*__tekai_state_generated_backend_writet1).fd_cur).builtin_glyph_names = glyph_names;
    if (*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).type_0 as ::core::ffi::c_int
        & F_SUBSETTED
        != 0 as ::core::ffi::c_int
    {
        if !((*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).type_0
            as ::core::ffi::c_int
            & 0x1 as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            __assert_rtn(
                b"t1_subset_ascii_part\0" as *const u8 as *const ::core::ffi::c_char,
                b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
                1339 as ::core::ffi::c_int,
                b"is_included(fd_cur->fm)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        if !(*(*__tekai_state_generated_backend_writet1).fd_cur)
            .tx_tree
            .is_null()
        {
            avl_t_init(
                &raw mut t,
                (*(*__tekai_state_generated_backend_writet1).fd_cur).tx_tree,
            );
            p = avl_t_first(
                &raw mut t,
                (*(*__tekai_state_generated_backend_writet1).fd_cur).tx_tree,
            ) as *mut ::core::ffi::c_int;
            while !p.is_null() {
                if (avl_find(
                    (*(*__tekai_state_generated_backend_writet1).fd_cur).gl_tree,
                    *glyph_names.offset(*p as isize) as *const ::core::ffi::c_void,
                ) as *mut ::core::ffi::c_char)
                    .is_null()
                {
                    glyph = xstrdup(*glyph_names.offset(*p as isize) as const_string)
                        as *mut ::core::ffi::c_char;
                    aa = avl_probe(
                        (*(*__tekai_state_generated_backend_writet1).fd_cur).gl_tree,
                        glyph as *mut ::core::ffi::c_void,
                    );
                    if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                        __assert_rtn(
                            b"t1_subset_ascii_part\0" as *const u8 as *const ::core::ffi::c_char,
                            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
                            1348 as ::core::ffi::c_int,
                            b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                    };
                }
                p = avl_t_next(&raw mut t) as *mut ::core::ffi::c_int;
            }
        }
        make_subset_tag((*__tekai_state_generated_backend_writet1).fd_cur);
        if !((*__tekai_state_generated_backend_writet1).t1_fontname_offset
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            __assert_rtn(
                b"t1_subset_ascii_part\0" as *const u8 as *const ::core::ffi::c_char,
                b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
                1353 as ::core::ffi::c_int,
                b"t1_fontname_offset != 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        strncpy(
            (*__tekai_state_utils)
                .fb_array
                .offset((*__tekai_state_generated_backend_writet1).t1_fontname_offset as isize),
            (*(*__tekai_state_generated_backend_writet1).fd_cur).subset_tag,
            6 as size_t,
        );
    }
    if (*__tekai_state_generated_backend_writet1).t1_encoding as ::core::ffi::c_uint
        == ENC_STANDARD as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        t1_puts(b"/Encoding StandardEncoding def\n\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        t1_puts(
            b"/Encoding 256 array\n0 1 255 {1 index exch /.notdef put} for\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        gl_tree = create_t1_glyph_tree(glyph_names);
        avl_t_init(
            &raw mut t,
            (*(*__tekai_state_generated_backend_writet1).fd_cur).gl_tree,
        );
        j = 0 as ::core::ffi::c_int;
        glyph = avl_t_first(
            &raw mut t,
            (*(*__tekai_state_generated_backend_writet1).fd_cur).gl_tree,
        ) as *mut ::core::ffi::c_char;
        while !glyph.is_null() {
            gg = avl_find(gl_tree, &raw mut glyph as *const ::core::ffi::c_void)
                as *mut *mut ::core::ffi::c_char;
            if !gg.is_null() {
                t1_printf(
                    b"dup %i /%s put\n\0" as *const u8 as *const ::core::ffi::c_char,
                    gg.offset_from(glyph_names) as ::core::ffi::c_long as ::core::ffi::c_int,
                    *gg,
                );
                j += 1;
            }
            glyph = avl_t_next(&raw mut t) as *mut ::core::ffi::c_char;
        }
        destroy_t1_glyph_tree(gl_tree);
        if j == 0 as ::core::ffi::c_int {
            t1_puts(b"dup 0 /.notdef put\n\0" as *const u8 as *const ::core::ffi::c_char);
        }
        t1_puts(b"readonly def\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    loop {
        t1_getline();
        t1_scan_param();
        if !(strncmp(
            (*__tekai_state_generated_backend_writet1).t1_line_array,
            b"/UniqueID\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"/UniqueID\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int)
        {
            t1_putline();
        }
        if !((*__tekai_state_generated_backend_writet1).t1_in_eexec == 0 as ::core::ffi::c_int) {
            break;
        }
    }
}
unsafe fn cs_init() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    (*__tekai_state_generated_backend_writet1).cs_tab = ::core::ptr::null_mut::<cs_entry>();
    (*__tekai_state_generated_backend_writet1).cs_ptr =
        (*__tekai_state_generated_backend_writet1).cs_tab;
    (*__tekai_state_generated_backend_writet1).cs_dict_end =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*__tekai_state_generated_backend_writet1).cs_dict_start =
        (*__tekai_state_generated_backend_writet1).cs_dict_end;
    (*__tekai_state_generated_backend_writet1).cs_size_pos = 0 as ::core::ffi::c_int;
    (*__tekai_state_generated_backend_writet1).cs_size =
        (*__tekai_state_generated_backend_writet1).cs_size_pos;
    (*__tekai_state_generated_backend_writet1).cs_count =
        (*__tekai_state_generated_backend_writet1).cs_size;
    (*__tekai_state_generated_backend_writet1).cs_token_pair =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    (*__tekai_state_generated_backend_writet1).subr_tab = ::core::ptr::null_mut::<cs_entry>();
    (*__tekai_state_generated_backend_writet1).subr_array_end =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*__tekai_state_generated_backend_writet1).subr_array_start =
        (*__tekai_state_generated_backend_writet1).subr_array_end;
    (*__tekai_state_generated_backend_writet1).subr_size_pos = 0 as ::core::ffi::c_int;
    (*__tekai_state_generated_backend_writet1).subr_size =
        (*__tekai_state_generated_backend_writet1).subr_size_pos;
    (*__tekai_state_generated_backend_writet1).subr_max =
        (*__tekai_state_generated_backend_writet1).subr_size;
}
unsafe fn init_cs_entry(mut cs: *mut cs_entry) {
    (*cs).data = ::core::ptr::null_mut::<byte>();
    (*cs).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*cs).len = 0 as ::core::ffi::c_ushort;
    (*cs).cslen = 0 as ::core::ffi::c_ushort;
    (*cs).used = false_0 as boolean;
    (*cs).valid = false_0 as boolean;
}
unsafe fn t1_read_subrs() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    let mut i: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0;
    let mut ptr: *mut cs_entry = ::core::ptr::null_mut::<cs_entry>();
    t1_getline();
    while !(!strstr(
        (*__tekai_state_generated_backend_writet1).t1_line_array,
        &raw mut (*__tekai_state_generated_backend_writet1).charstringname
            as *mut ::core::ffi::c_char,
    )
    .is_null()
        || strncmp(
            (*__tekai_state_generated_backend_writet1).t1_line_array,
            b"/Subrs\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"/Subrs\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int)
    {
        t1_scan_param();
        if !(strncmp(
            (*__tekai_state_generated_backend_writet1).t1_line_array,
            b"/UniqueID\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"/UniqueID\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int)
        {
            t1_putline();
        }
        t1_getline();
    }
    loop {
        (*__tekai_state_generated_backend_writet1).t1_cs = true_0 as boolean;
        (*__tekai_state_generated_backend_writet1).t1_scan = false_0 as boolean;
        if !(strncmp(
            (*__tekai_state_generated_backend_writet1).t1_line_array,
            b"/Subrs\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"/Subrs\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int)
        {
            return;
        }
        (*__tekai_state_generated_backend_writet1).subr_size_pos = strlen(
            b"/Subrs\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .wrapping_add(1 as size_t)
            as ::core::ffi::c_int;
        (*__tekai_state_generated_backend_writet1).subr_size = t1_scan_num(
            (*__tekai_state_generated_backend_writet1)
                .t1_line_array
                .offset((*__tekai_state_generated_backend_writet1).subr_size_pos as isize),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) as ::core::ffi::c_int;
        if (*__tekai_state_generated_backend_writet1).subr_size == 0 as ::core::ffi::c_int {
            while strstr(
                (*__tekai_state_generated_backend_writet1).t1_line_array,
                &raw mut (*__tekai_state_generated_backend_writet1).charstringname
                    as *mut ::core::ffi::c_char,
            )
            .is_null()
            {
                t1_getline();
            }
            return;
        }
        (*__tekai_state_generated_backend_writet1).subr_tab = xmalloc(
            ((*__tekai_state_generated_backend_writet1).subr_size as size_t)
                .wrapping_mul(::core::mem::size_of::<cs_entry>() as size_t),
        ) as *mut cs_entry;
        ptr = (*__tekai_state_generated_backend_writet1).subr_tab;
        while (ptr.offset_from((*__tekai_state_generated_backend_writet1).subr_tab)
            as ::core::ffi::c_long)
            < (*__tekai_state_generated_backend_writet1).subr_size as ::core::ffi::c_long
        {
            init_cs_entry(ptr);
            ptr = ptr.offset(1);
        }
        (*__tekai_state_generated_backend_writet1).subr_array_start =
            xstrdup((*__tekai_state_generated_backend_writet1).t1_line_array as const_string)
                as *mut ::core::ffi::c_char;
        t1_getline();
        while (*__tekai_state_generated_backend_writet1).t1_cslen != 0 {
            cs_store(true_0);
            t1_getline();
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*__tekai_state_generated_backend_writet1).subr_size
            && i < 4 as ::core::ffi::c_int
        {
            (*(*__tekai_state_generated_backend_writet1)
                .subr_tab
                .offset(i as isize))
            .used = true_0 as boolean;
            i += 1;
        }
        s = 0 as ::core::ffi::c_int;
        *(*__tekai_state_generated_backend_writet1).t1_buf_array = 0 as t1_buf_entry;
        i = 0 as ::core::ffi::c_int;
        while i < POST_SUBRS_SCAN {
            if !strstr(
                (*__tekai_state_generated_backend_writet1).t1_line_array,
                &raw mut (*__tekai_state_generated_backend_writet1).charstringname
                    as *mut ::core::ffi::c_char,
            )
            .is_null()
            {
                break;
            }
            s = (s as ::core::ffi::c_long
                + (*__tekai_state_generated_backend_writet1)
                    .t1_line_ptr
                    .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                    as ::core::ffi::c_long) as ::core::ffi::c_int;
            if (*__tekai_state_generated_backend_writet1)
                .t1_buf_array
                .is_null()
            {
                (*__tekai_state_generated_backend_writet1).t1_buf_limit = 0x10 as size_t;
                if s as ::core::ffi::c_uint
                    > (*__tekai_state_generated_backend_writet1).t1_buf_limit as ::core::ffi::c_uint
                {
                    (*__tekai_state_generated_backend_writet1).t1_buf_limit = s as size_t;
                }
                (*__tekai_state_generated_backend_writet1).t1_buf_array = xmalloc(
                    (*__tekai_state_generated_backend_writet1)
                        .t1_buf_limit
                        .wrapping_mul(::core::mem::size_of::<t1_buf_entry>() as size_t),
                )
                    as *mut t1_buf_entry;
                (*__tekai_state_generated_backend_writet1).t1_buf_ptr =
                    (*__tekai_state_generated_backend_writet1).t1_buf_array;
            } else if ((*__tekai_state_generated_backend_writet1)
                .t1_buf_ptr
                .offset_from((*__tekai_state_generated_backend_writet1).t1_buf_array)
                as ::core::ffi::c_long
                + s as ::core::ffi::c_long) as ::core::ffi::c_uint
                > (*__tekai_state_generated_backend_writet1).t1_buf_limit as ::core::ffi::c_uint
            {
                (*__tekai_state_utils).last_ptr_index = (*__tekai_state_generated_backend_writet1)
                    .t1_buf_ptr
                    .offset_from((*__tekai_state_generated_backend_writet1).t1_buf_array)
                    as ::core::ffi::c_long
                    as size_t;
                (*__tekai_state_generated_backend_writet1).t1_buf_limit =
                    (*__tekai_state_generated_backend_writet1)
                        .t1_buf_limit
                        .wrapping_mul(2 as size_t);
                if ((*__tekai_state_generated_backend_writet1)
                    .t1_buf_ptr
                    .offset_from((*__tekai_state_generated_backend_writet1).t1_buf_array)
                    as ::core::ffi::c_long
                    + s as ::core::ffi::c_long) as ::core::ffi::c_uint
                    > (*__tekai_state_generated_backend_writet1).t1_buf_limit as ::core::ffi::c_uint
                {
                    (*__tekai_state_generated_backend_writet1).t1_buf_limit =
                        ((*__tekai_state_generated_backend_writet1)
                            .t1_buf_ptr
                            .offset_from((*__tekai_state_generated_backend_writet1).t1_buf_array)
                            as ::core::ffi::c_long
                            + s as ::core::ffi::c_long) as size_t;
                }
                if (*__tekai_state_generated_backend_writet1).t1_buf_limit as ::core::ffi::c_uint
                    > INT_MAX as ::core::ffi::c_uint
                {
                    crate::utils::pdftex_fail_args(
                        b"t1_buf_array exceeds size limit\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                }
                (*__tekai_state_generated_backend_writet1).t1_buf_array = xrealloc(
                    (*__tekai_state_generated_backend_writet1).t1_buf_array as address,
                    (*__tekai_state_generated_backend_writet1)
                        .t1_buf_limit
                        .wrapping_mul(::core::mem::size_of::<t1_buf_entry>() as size_t),
                )
                    as *mut t1_buf_entry;
                (*__tekai_state_generated_backend_writet1).t1_buf_ptr =
                    (*__tekai_state_generated_backend_writet1)
                        .t1_buf_array
                        .offset((*__tekai_state_utils).last_ptr_index as isize);
            }
            strcat(
                (*__tekai_state_generated_backend_writet1).t1_buf_array as *mut ::core::ffi::c_char,
                (*__tekai_state_generated_backend_writet1).t1_line_array,
            );
            t1_getline();
            i += 1;
        }
        (*__tekai_state_generated_backend_writet1).subr_array_end =
            xstrdup((*__tekai_state_generated_backend_writet1).t1_buf_array as const_string)
                as *mut ::core::ffi::c_char;
        if !(i == POST_SUBRS_SCAN) {
            break;
        }
        ptr = (*__tekai_state_generated_backend_writet1).subr_tab;
        while (ptr.offset_from((*__tekai_state_generated_backend_writet1).subr_tab)
            as ::core::ffi::c_long)
            < (*__tekai_state_generated_backend_writet1).subr_size as ::core::ffi::c_long
        {
            if (*ptr).valid != 0 {
                if !(*ptr).data.is_null() {
                    free((*ptr).data as *mut ::core::ffi::c_void);
                }
                (*ptr).data = ::core::ptr::null_mut::<byte>();
            }
            ptr = ptr.offset(1);
        }
        if !(*__tekai_state_generated_backend_writet1)
            .subr_tab
            .is_null()
        {
            free((*__tekai_state_generated_backend_writet1).subr_tab as *mut ::core::ffi::c_void);
        }
        (*__tekai_state_generated_backend_writet1).subr_tab = ::core::ptr::null_mut::<cs_entry>();
        if !(*__tekai_state_generated_backend_writet1)
            .subr_array_start
            .is_null()
        {
            free(
                (*__tekai_state_generated_backend_writet1).subr_array_start
                    as *mut ::core::ffi::c_void,
            );
        }
        (*__tekai_state_generated_backend_writet1).subr_array_start =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        if !(*__tekai_state_generated_backend_writet1)
            .subr_array_end
            .is_null()
        {
            free(
                (*__tekai_state_generated_backend_writet1).subr_array_end
                    as *mut ::core::ffi::c_void,
            );
        }
        (*__tekai_state_generated_backend_writet1).subr_array_end =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        cs_init();
        (*__tekai_state_generated_backend_writet1).t1_cs = false_0 as boolean;
        (*__tekai_state_generated_backend_writet1).t1_synthetic = true_0 as boolean;
        while !(!strstr(
            (*__tekai_state_generated_backend_writet1).t1_line_array,
            &raw mut (*__tekai_state_generated_backend_writet1).charstringname
                as *mut ::core::ffi::c_char,
        )
        .is_null()
            || strncmp(
                (*__tekai_state_generated_backend_writet1).t1_line_array,
                b"/Subrs\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"/Subrs\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0 as ::core::ffi::c_int)
        {
            t1_getline();
        }
    }
}
pub const POST_SUBRS_SCAN: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

#[cfg(test)]
std::thread_local! {
    static CS_RELEASE_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

unsafe fn release_cs_entries(mut ptr: *mut cs_entry, end: *mut cs_entry) {
    let __tekai_state_generated_backend_writettf: *mut crate::generated::backend::writettf::State =
        unsafe { crate::generated::backend::writettf::state() };

    while ptr < end {
        if !(*ptr).data.is_null() {
            #[cfg(test)]
            CS_RELEASE_CALLS.with(|calls| calls.set(calls.get() + 1));
            free((*ptr).data as *mut ::core::ffi::c_void);
        }
        (*ptr).data = ::core::ptr::null_mut::<byte>();
        if (*ptr).name
            != &raw mut (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char
        {
            if !(*ptr).name.is_null() {
                #[cfg(test)]
                CS_RELEASE_CALLS.with(|calls| calls.set(calls.get() + 1));
                free((*ptr).name as *mut ::core::ffi::c_void);
            }
            (*ptr).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        ptr = ptr.offset(1);
    }
}

unsafe fn t1_flush_cs(mut is_subr: boolean) {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut return_cs: *mut byte = ::core::ptr::null_mut::<byte>();
    let mut tab: *mut cs_entry = ::core::ptr::null_mut::<cs_entry>();
    let mut end_tab: *mut cs_entry = ::core::ptr::null_mut::<cs_entry>();
    let mut ptr: *mut cs_entry = ::core::ptr::null_mut::<cs_entry>();
    let mut start_line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut line_end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut count: ::core::ffi::c_int = 0;
    let mut size_pos: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_ushort = 0;
    let mut cs_len: ::core::ffi::c_ushort = 0 as ::core::ffi::c_ushort;
    if is_subr != 0 {
        start_line = (*__tekai_state_generated_backend_writet1).subr_array_start;
        line_end = (*__tekai_state_generated_backend_writet1).subr_array_end;
        size_pos = (*__tekai_state_generated_backend_writet1).subr_size_pos;
        tab = (*__tekai_state_generated_backend_writet1).subr_tab;
        count = (*__tekai_state_generated_backend_writet1).subr_max + 1 as ::core::ffi::c_int;
        end_tab = (*__tekai_state_generated_backend_writet1)
            .subr_tab
            .offset(count as isize);
    } else {
        start_line = (*__tekai_state_generated_backend_writet1).cs_dict_start;
        line_end = (*__tekai_state_generated_backend_writet1).cs_dict_end;
        size_pos = (*__tekai_state_generated_backend_writet1).cs_size_pos;
        tab = (*__tekai_state_generated_backend_writet1).cs_tab;
        end_tab = (*__tekai_state_generated_backend_writet1).cs_ptr;
        count = (*__tekai_state_generated_backend_writet1).cs_count;
    }
    (*__tekai_state_generated_backend_writet1).t1_line_ptr =
        (*__tekai_state_generated_backend_writet1).t1_line_array;
    p = start_line;
    while (p.offset_from(start_line) as ::core::ffi::c_long) < size_pos as ::core::ffi::c_long {
        let fresh14 = p;
        p = p.offset(1);
        let fresh15 = (*__tekai_state_generated_backend_writet1).t1_line_ptr;
        (*__tekai_state_generated_backend_writet1).t1_line_ptr =
            (*__tekai_state_generated_backend_writet1)
                .t1_line_ptr
                .offset(1);
        *fresh15 = *fresh14 as t1_line_entry;
    }
    while isdigit(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        p = p.offset(1);
    }
    crate::utils::sprintf_args(
        (*__tekai_state_generated_backend_writet1).t1_line_ptr as *mut ::core::ffi::c_char,
        b"%u\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(count)],
    );
    strcat(
        (*__tekai_state_generated_backend_writet1).t1_line_ptr as *mut ::core::ffi::c_char,
        p,
    );
    (*__tekai_state_generated_backend_writet1).t1_line_ptr =
        eol((*__tekai_state_generated_backend_writet1).t1_line_array as *mut ::core::ffi::c_char)
            as *mut t1_line_entry;
    t1_putline();
    if is_subr != 0 {
        cr = 4330 as ::core::ffi::c_ushort;
        cs_len = 0 as ::core::ffi::c_ushort;
        return_cs = xmalloc(
            (((*__tekai_state_generated_backend_writet1).t1_lenIV as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<byte>() as size_t),
        ) as *mut byte;
        cs_len = 0 as ::core::ffi::c_ushort;
        r = return_cs;
        while (cs_len as ::core::ffi::c_int)
            < (*__tekai_state_generated_backend_writet1).t1_lenIV as ::core::ffi::c_int
        {
            *r = cencrypt(0 as byte, &raw mut cr);
            cs_len = cs_len.wrapping_add(1);
            r = r.offset(1);
        }
        *r = cencrypt(CS_RETURN as byte, &raw mut cr);
        cs_len = cs_len.wrapping_add(1);
    }
    ptr = tab;
    while ptr < end_tab {
        if (*ptr).used != 0 {
            if is_subr != 0 {
                crate::utils::sprintf_args(
                    (*__tekai_state_generated_backend_writet1).t1_line_array
                        as *mut ::core::ffi::c_char,
                    b"dup %lu %u\0" as *const u8 as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(
                            ptr.offset_from(tab) as ::core::ffi::c_long as ::core::ffi::c_ulong
                        ),
                        crate::utils::PrintfArg::from((*ptr).cslen as ::core::ffi::c_int),
                    ],
                );
            } else {
                crate::utils::sprintf_args(
                    (*__tekai_state_generated_backend_writet1).t1_line_array
                        as *mut ::core::ffi::c_char,
                    b"/%s %u\0" as *const u8 as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from((*ptr).name),
                        crate::utils::PrintfArg::from((*ptr).cslen as ::core::ffi::c_int),
                    ],
                );
            }
            p = strchr(
                (*__tekai_state_generated_backend_writet1).t1_line_array,
                0 as ::core::ffi::c_int,
            );
            memcpy(
                p as *mut ::core::ffi::c_void,
                (*ptr).data as *const ::core::ffi::c_void,
                (*ptr).len as size_t,
            );
            (*__tekai_state_generated_backend_writet1).t1_line_ptr =
                p.offset((*ptr).len as ::core::ffi::c_int as isize) as *mut t1_line_entry;
            t1_putline();
        } else if is_subr != 0 {
            crate::utils::sprintf_args(
                (*__tekai_state_generated_backend_writet1).t1_line_array
                    as *mut ::core::ffi::c_char,
                b"dup %lu %u%s \0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(
                        ptr.offset_from(tab) as ::core::ffi::c_long as ::core::ffi::c_ulong
                    ),
                    crate::utils::PrintfArg::from(cs_len as ::core::ffi::c_int),
                    crate::utils::PrintfArg::from(
                        *(*__tekai_state_generated_backend_writet1)
                            .cs_token_pair
                            .offset(0 as ::core::ffi::c_int as isize),
                    ),
                ],
            );
            p = strchr(
                (*__tekai_state_generated_backend_writet1).t1_line_array,
                0 as ::core::ffi::c_int,
            );
            memcpy(
                p as *mut ::core::ffi::c_void,
                return_cs as *const ::core::ffi::c_void,
                cs_len as size_t,
            );
            (*__tekai_state_generated_backend_writet1).t1_line_ptr =
                p.offset(cs_len as ::core::ffi::c_int as isize) as *mut t1_line_entry;
            t1_putline();
            crate::utils::sprintf_args(
                (*__tekai_state_generated_backend_writet1).t1_line_array
                    as *mut ::core::ffi::c_char,
                b" %s\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(
                    *(*__tekai_state_generated_backend_writet1)
                        .cs_token_pair
                        .offset(1 as ::core::ffi::c_int as isize),
                )],
            );
            (*__tekai_state_generated_backend_writet1).t1_line_ptr =
                eol((*__tekai_state_generated_backend_writet1).t1_line_array
                    as *mut ::core::ffi::c_char) as *mut t1_line_entry;
            t1_putline();
        }
        release_cs_entries(ptr, ptr.offset(1));
        ptr = ptr.offset(1);
    }
    crate::utils::sprintf_args(
        (*__tekai_state_generated_backend_writet1).t1_line_array as *mut ::core::ffi::c_char,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(line_end)],
    );
    (*__tekai_state_generated_backend_writet1).t1_line_ptr =
        eol((*__tekai_state_generated_backend_writet1).t1_line_array as *mut ::core::ffi::c_char)
            as *mut t1_line_entry;
    t1_putline();
    if is_subr != 0 {
        // Emission uses subr_max + 1, but all subr_size entries own their storage.
        release_cs_entries(
            end_tab,
            tab.offset((*__tekai_state_generated_backend_writet1).subr_size as isize),
        );
        if !return_cs.is_null() {
            free(return_cs as *mut ::core::ffi::c_void);
        }
        return_cs = ::core::ptr::null_mut::<byte>();
    }
    if !tab.is_null() {
        free(tab as *mut ::core::ffi::c_void);
    }
    tab = ::core::ptr::null_mut::<cs_entry>();
    if !start_line.is_null() {
        free(start_line as *mut ::core::ffi::c_void);
    }
    start_line = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !line_end.is_null() {
        free(line_end as *mut ::core::ffi::c_void);
    }
    line_end = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe fn t1_mark_glyphs() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };
    let __tekai_state_generated_backend_writettf: *mut crate::generated::backend::writettf::State =
        unsafe { crate::generated::backend::writettf::state() };

    let mut glyph: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut t: avl_traverser = avl_traverser {
        avl_table: ::core::ptr::null_mut::<avl_table>(),
        avl_node: ::core::ptr::null_mut::<avl_node>(),
        avl_stack: [::core::ptr::null_mut::<avl_node>(); 32],
        avl_height: 0,
        avl_generation: 0,
    };
    let mut ptr: *mut cs_entry = ::core::ptr::null_mut::<cs_entry>();
    if (*__tekai_state_generated_backend_writet1).t1_synthetic != 0
        || (*(*__tekai_state_generated_backend_writet1).fd_cur).all_glyphs != 0
    {
        if !(*__tekai_state_generated_backend_writet1).cs_tab.is_null() {
            ptr = (*__tekai_state_generated_backend_writet1).cs_tab;
            while ptr < (*__tekai_state_generated_backend_writet1).cs_ptr {
                if (*ptr).valid != 0 {
                    (*ptr).used = true_0 as boolean;
                }
                ptr = ptr.offset(1);
            }
        }
        if !(*__tekai_state_generated_backend_writet1)
            .subr_tab
            .is_null()
        {
            ptr = (*__tekai_state_generated_backend_writet1).subr_tab;
            while (ptr.offset_from((*__tekai_state_generated_backend_writet1).subr_tab)
                as ::core::ffi::c_long)
                < (*__tekai_state_generated_backend_writet1).subr_size as ::core::ffi::c_long
            {
                if (*ptr).valid != 0 {
                    (*ptr).used = true_0 as boolean;
                }
                ptr = ptr.offset(1);
            }
            (*__tekai_state_generated_backend_writet1).subr_max =
                (*__tekai_state_generated_backend_writet1).subr_size - 1 as ::core::ffi::c_int;
        }
        return;
    }
    cs_mark(
        &raw mut (*__tekai_state_generated_backend_writettf).notdef as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    avl_t_init(
        &raw mut t,
        (*(*__tekai_state_generated_backend_writet1).fd_cur).gl_tree,
    );
    glyph = avl_t_first(
        &raw mut t,
        (*(*__tekai_state_generated_backend_writet1).fd_cur).gl_tree,
    ) as *mut ::core::ffi::c_char;
    while !glyph.is_null() {
        cs_mark(glyph, 0 as ::core::ffi::c_int);
        glyph = avl_t_next(&raw mut t) as *mut ::core::ffi::c_char;
    }
    if !(*__tekai_state_generated_backend_writet1)
        .subr_tab
        .is_null()
    {
        (*__tekai_state_generated_backend_writet1).subr_max = -(1 as ::core::ffi::c_int);
        ptr = (*__tekai_state_generated_backend_writet1).subr_tab;
        while (ptr.offset_from((*__tekai_state_generated_backend_writet1).subr_tab)
            as ::core::ffi::c_long)
            < (*__tekai_state_generated_backend_writet1).subr_size as ::core::ffi::c_long
        {
            if (*ptr).used != 0
                && ptr.offset_from((*__tekai_state_generated_backend_writet1).subr_tab)
                    as ::core::ffi::c_long
                    > (*__tekai_state_generated_backend_writet1).subr_max as ::core::ffi::c_long
            {
                (*__tekai_state_generated_backend_writet1).subr_max =
                    ptr.offset_from((*__tekai_state_generated_backend_writet1).subr_tab)
                        as ::core::ffi::c_long as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
        }
    }
}
unsafe fn t1_check_unusual_charstring() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    let marker = strstr(
        (*__tekai_state_generated_backend_writet1).t1_line_array,
        &raw mut (*__tekai_state_generated_backend_writet1).charstringname
            as *mut ::core::ffi::c_char,
    );
    if marker.is_null() {
        crate::utils::pdftex_fail_args(c"invalid pfb, CharStrings marker missing".as_ptr(), &[]);
    }
    let mut p = marker.add(strlen(
        &raw mut (*__tekai_state_generated_backend_writet1).charstringname
            as *mut ::core::ffi::c_char,
    ));
    if t1_scan_field(charstring_count(t1_written_suffix(p))).is_none() {
        strcpy(
            (*__tekai_state_generated_backend_writet1).t1_buf_array as *mut ::core::ffi::c_char,
            (*__tekai_state_generated_backend_writet1).t1_line_array,
        );
        *strchr(
            (*__tekai_state_generated_backend_writet1).t1_buf_array,
            0 as ::core::ffi::c_int,
        )
        .offset(-(1 as ::core::ffi::c_int as isize)) = ' ' as i32 as ::core::ffi::c_char;
        t1_getline();
        if (*__tekai_state_generated_backend_writet1)
            .t1_buf_array
            .is_null()
        {
            (*__tekai_state_generated_backend_writet1).t1_buf_limit = 0x10 as size_t;
            if strlen((*__tekai_state_generated_backend_writet1).t1_line_array).wrapping_add(
                (if !(*__tekai_state_generated_backend_writet1)
                    .t1_buf_array
                    .is_null()
                {
                    strlen((*__tekai_state_generated_backend_writet1).t1_buf_array)
                        .wrapping_add(1 as size_t)
                } else {
                    0 as size_t
                }),
            ) as ::core::ffi::c_uint
                > (*__tekai_state_generated_backend_writet1).t1_buf_limit as ::core::ffi::c_uint
            {
                (*__tekai_state_generated_backend_writet1).t1_buf_limit =
                    strlen((*__tekai_state_generated_backend_writet1).t1_line_array).wrapping_add(
                        (if !(*__tekai_state_generated_backend_writet1)
                            .t1_buf_array
                            .is_null()
                        {
                            strlen((*__tekai_state_generated_backend_writet1).t1_buf_array)
                                .wrapping_add(1 as size_t)
                        } else {
                            0 as size_t
                        }),
                    );
            }
            (*__tekai_state_generated_backend_writet1).t1_buf_array = xmalloc(
                (*__tekai_state_generated_backend_writet1)
                    .t1_buf_limit
                    .wrapping_mul(::core::mem::size_of::<t1_buf_entry>() as size_t),
            )
                as *mut t1_buf_entry;
            (*__tekai_state_generated_backend_writet1).t1_buf_ptr =
                (*__tekai_state_generated_backend_writet1).t1_buf_array;
        } else if ((*__tekai_state_generated_backend_writet1)
            .t1_buf_ptr
            .offset_from((*__tekai_state_generated_backend_writet1).t1_buf_array)
            as ::core::ffi::c_long as size_t)
            .wrapping_add(
                strlen((*__tekai_state_generated_backend_writet1).t1_line_array).wrapping_add(
                    (if !(*__tekai_state_generated_backend_writet1)
                        .t1_buf_array
                        .is_null()
                    {
                        strlen((*__tekai_state_generated_backend_writet1).t1_buf_array)
                            .wrapping_add(1 as size_t)
                    } else {
                        0 as size_t
                    }),
                ),
            ) as ::core::ffi::c_uint
            > (*__tekai_state_generated_backend_writet1).t1_buf_limit as ::core::ffi::c_uint
        {
            (*__tekai_state_utils).last_ptr_index = (*__tekai_state_generated_backend_writet1)
                .t1_buf_ptr
                .offset_from((*__tekai_state_generated_backend_writet1).t1_buf_array)
                as ::core::ffi::c_long
                as size_t;
            (*__tekai_state_generated_backend_writet1).t1_buf_limit =
                (*__tekai_state_generated_backend_writet1)
                    .t1_buf_limit
                    .wrapping_mul(2 as size_t);
            if ((*__tekai_state_generated_backend_writet1)
                .t1_buf_ptr
                .offset_from((*__tekai_state_generated_backend_writet1).t1_buf_array)
                as ::core::ffi::c_long as size_t)
                .wrapping_add(
                    strlen((*__tekai_state_generated_backend_writet1).t1_line_array).wrapping_add(
                        (if !(*__tekai_state_generated_backend_writet1)
                            .t1_buf_array
                            .is_null()
                        {
                            strlen((*__tekai_state_generated_backend_writet1).t1_buf_array)
                                .wrapping_add(1 as size_t)
                        } else {
                            0 as size_t
                        }),
                    ),
                ) as ::core::ffi::c_uint
                > (*__tekai_state_generated_backend_writet1).t1_buf_limit as ::core::ffi::c_uint
            {
                (*__tekai_state_generated_backend_writet1).t1_buf_limit =
                    ((*__tekai_state_generated_backend_writet1)
                        .t1_buf_ptr
                        .offset_from((*__tekai_state_generated_backend_writet1).t1_buf_array)
                        as ::core::ffi::c_long as size_t)
                        .wrapping_add(
                            strlen((*__tekai_state_generated_backend_writet1).t1_line_array)
                                .wrapping_add(
                                    (if !(*__tekai_state_generated_backend_writet1)
                                        .t1_buf_array
                                        .is_null()
                                    {
                                        strlen(
                                            (*__tekai_state_generated_backend_writet1).t1_buf_array,
                                        )
                                        .wrapping_add(1 as size_t)
                                    } else {
                                        0 as size_t
                                    }),
                                ),
                        );
            }
            if (*__tekai_state_generated_backend_writet1).t1_buf_limit as ::core::ffi::c_uint
                > INT_MAX as ::core::ffi::c_uint
            {
                crate::utils::pdftex_fail_args(
                    b"t1_buf_array exceeds size limit\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            (*__tekai_state_generated_backend_writet1).t1_buf_array = xrealloc(
                (*__tekai_state_generated_backend_writet1).t1_buf_array as address,
                (*__tekai_state_generated_backend_writet1)
                    .t1_buf_limit
                    .wrapping_mul(::core::mem::size_of::<t1_buf_entry>() as size_t),
            )
                as *mut t1_buf_entry;
            (*__tekai_state_generated_backend_writet1).t1_buf_ptr =
                (*__tekai_state_generated_backend_writet1)
                    .t1_buf_array
                    .offset((*__tekai_state_utils).last_ptr_index as isize);
        }
        strcat(
            (*__tekai_state_generated_backend_writet1).t1_buf_array as *mut ::core::ffi::c_char,
            (*__tekai_state_generated_backend_writet1).t1_line_array,
        );
        if (*__tekai_state_generated_backend_writet1)
            .t1_line_array
            .is_null()
        {
            (*__tekai_state_generated_backend_writet1).t1_line_limit = 0x10 as size_t;
            if strlen((*__tekai_state_generated_backend_writet1).t1_buf_array)
                .wrapping_add(1 as size_t) as ::core::ffi::c_uint
                > (*__tekai_state_generated_backend_writet1).t1_line_limit as ::core::ffi::c_uint
            {
                (*__tekai_state_generated_backend_writet1).t1_line_limit =
                    strlen((*__tekai_state_generated_backend_writet1).t1_buf_array)
                        .wrapping_add(1 as size_t);
            }
            (*__tekai_state_generated_backend_writet1).t1_line_array = xmalloc(
                (*__tekai_state_generated_backend_writet1)
                    .t1_line_limit
                    .wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
            )
                as *mut t1_line_entry;
            (*__tekai_state_generated_backend_writet1).t1_line_ptr =
                (*__tekai_state_generated_backend_writet1).t1_line_array;
        } else if ((*__tekai_state_generated_backend_writet1)
            .t1_line_ptr
            .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
            as ::core::ffi::c_long as size_t)
            .wrapping_add(
                strlen((*__tekai_state_generated_backend_writet1).t1_buf_array)
                    .wrapping_add(1 as size_t),
            ) as ::core::ffi::c_uint
            > (*__tekai_state_generated_backend_writet1).t1_line_limit as ::core::ffi::c_uint
        {
            (*__tekai_state_utils).last_ptr_index = (*__tekai_state_generated_backend_writet1)
                .t1_line_ptr
                .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                as ::core::ffi::c_long
                as size_t;
            (*__tekai_state_generated_backend_writet1).t1_line_limit =
                (*__tekai_state_generated_backend_writet1)
                    .t1_line_limit
                    .wrapping_mul(2 as size_t);
            if ((*__tekai_state_generated_backend_writet1)
                .t1_line_ptr
                .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                as ::core::ffi::c_long as size_t)
                .wrapping_add(
                    strlen((*__tekai_state_generated_backend_writet1).t1_buf_array)
                        .wrapping_add(1 as size_t),
                ) as ::core::ffi::c_uint
                > (*__tekai_state_generated_backend_writet1).t1_line_limit as ::core::ffi::c_uint
            {
                (*__tekai_state_generated_backend_writet1).t1_line_limit =
                    ((*__tekai_state_generated_backend_writet1)
                        .t1_line_ptr
                        .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
                        as ::core::ffi::c_long as size_t)
                        .wrapping_add(
                            strlen((*__tekai_state_generated_backend_writet1).t1_buf_array)
                                .wrapping_add(1 as size_t),
                        );
            }
            if (*__tekai_state_generated_backend_writet1).t1_line_limit as ::core::ffi::c_uint
                > INT_MAX as ::core::ffi::c_uint
            {
                crate::utils::pdftex_fail_args(
                    b"t1_line_array exceeds size limit\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                );
            }
            (*__tekai_state_generated_backend_writet1).t1_line_array = xrealloc(
                (*__tekai_state_generated_backend_writet1).t1_line_array as address,
                (*__tekai_state_generated_backend_writet1)
                    .t1_line_limit
                    .wrapping_mul(::core::mem::size_of::<t1_line_entry>() as size_t),
            )
                as *mut t1_line_entry;
            (*__tekai_state_generated_backend_writet1).t1_line_ptr =
                (*__tekai_state_generated_backend_writet1)
                    .t1_line_array
                    .offset((*__tekai_state_utils).last_ptr_index as isize);
        }
        strcpy(
            (*__tekai_state_generated_backend_writet1).t1_line_array as *mut ::core::ffi::c_char,
            (*__tekai_state_generated_backend_writet1).t1_buf_array,
        );
        (*__tekai_state_generated_backend_writet1).t1_line_ptr =
            eol((*__tekai_state_generated_backend_writet1).t1_line_array
                as *mut ::core::ffi::c_char) as *mut t1_line_entry;
    }
}
unsafe fn t1_subset_charstrings() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    let mut ptr: *mut cs_entry = ::core::ptr::null_mut::<cs_entry>();
    t1_check_unusual_charstring();
    (*__tekai_state_generated_backend_writet1).cs_size_pos = (strstr(
        (*__tekai_state_generated_backend_writet1).t1_line_array,
        &raw mut (*__tekai_state_generated_backend_writet1).charstringname
            as *mut ::core::ffi::c_char,
    )
    .offset(strlen(
        &raw mut (*__tekai_state_generated_backend_writet1).charstringname
            as *mut ::core::ffi::c_char,
    ) as isize)
    .offset_from((*__tekai_state_generated_backend_writet1).t1_line_array)
        as ::core::ffi::c_long
        + 1 as ::core::ffi::c_long)
        as ::core::ffi::c_int;
    (*__tekai_state_generated_backend_writet1).cs_size = t1_scan_num(
        (*__tekai_state_generated_backend_writet1)
            .t1_line_array
            .offset((*__tekai_state_generated_backend_writet1).cs_size_pos as isize),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) as ::core::ffi::c_int;
    (*__tekai_state_generated_backend_writet1).cs_tab = xmalloc(
        ((*__tekai_state_generated_backend_writet1).cs_size as size_t)
            .wrapping_mul(::core::mem::size_of::<cs_entry>() as size_t),
    ) as *mut cs_entry;
    (*__tekai_state_generated_backend_writet1).cs_ptr =
        (*__tekai_state_generated_backend_writet1).cs_tab;
    ptr = (*__tekai_state_generated_backend_writet1).cs_tab;
    while (ptr.offset_from((*__tekai_state_generated_backend_writet1).cs_tab)
        as ::core::ffi::c_long)
        < (*__tekai_state_generated_backend_writet1).cs_size as ::core::ffi::c_long
    {
        init_cs_entry(ptr);
        ptr = ptr.offset(1);
    }
    (*__tekai_state_generated_backend_writet1).cs_notdef = ::core::ptr::null_mut::<cs_entry>();
    (*__tekai_state_generated_backend_writet1).cs_dict_start =
        xstrdup((*__tekai_state_generated_backend_writet1).t1_line_array as const_string)
            as *mut ::core::ffi::c_char;
    t1_getline();
    while (*__tekai_state_generated_backend_writet1).t1_cslen != 0 {
        cs_store(false_0);
        t1_getline();
    }
    (*__tekai_state_generated_backend_writet1).cs_dict_end =
        xstrdup((*__tekai_state_generated_backend_writet1).t1_line_array as const_string)
            as *mut ::core::ffi::c_char;
    t1_mark_glyphs();
    if !(*__tekai_state_generated_backend_writet1)
        .subr_tab
        .is_null()
    {
        if (*__tekai_state_generated_backend_writet1)
            .cs_token_pair
            .is_null()
        {
            crate::utils::pdftex_fail_args(
                b"This Type 1 font uses mismatched subroutine begin/end token pairs.\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
        }
        t1_flush_cs(true_0);
    }
    (*__tekai_state_generated_backend_writet1).cs_count = 0 as ::core::ffi::c_int;
    ptr = (*__tekai_state_generated_backend_writet1).cs_tab;
    while ptr < (*__tekai_state_generated_backend_writet1).cs_ptr {
        if (*ptr).used != 0 {
            (*__tekai_state_generated_backend_writet1).cs_count += 1;
        }
        ptr = ptr.offset(1);
    }
    t1_flush_cs(false_0);
}
unsafe fn t1_subset_end() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    if (*__tekai_state_generated_backend_writet1).t1_synthetic != 0 {
        while strstr(
            (*__tekai_state_generated_backend_writet1).t1_line_array,
            b"definefont\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            t1_getline();
            t1_putline();
        }
        while str_suffix(
            (*__tekai_state_generated_backend_writet1).t1_line_array,
            (*__tekai_state_generated_backend_writet1).t1_line_ptr,
            b"mark currentfile closefile\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        {
            t1_getline();
        }
        t1_putline();
    } else {
        while str_suffix(
            (*__tekai_state_generated_backend_writet1).t1_line_array,
            (*__tekai_state_generated_backend_writet1).t1_line_ptr,
            b"mark currentfile closefile\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        {
            t1_getline();
            t1_putline();
        }
    }
    t1_stop_eexec();
    (*__tekai_state_generated_backend_writet1).t1_length3 = (if fixedcontent != 0 {
        fb_offset() as ::core::ffi::c_int
            - (*__tekai_state_generated_backend_writet1).t1_save_offset as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as integer;
}
pub unsafe fn writet1(mut fd: *mut fd_entry) {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    (*__tekai_state_generated_backend_writet1).fd_cur = fd;
    if (*(*__tekai_state_generated_backend_writet1).fd_cur)
        .fm
        .is_null() as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"writet1\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            1698 as ::core::ffi::c_int,
            b"fd_cur->fm != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !((*(*fd).fm).type_0 as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"writet1\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            1699 as ::core::ffi::c_int,
            b"is_type1(fd->fm)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !((*(*fd).fm).type_0 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"writet1\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet1.c\0" as *const u8 as *const ::core::ffi::c_char,
            1700 as ::core::ffi::c_int,
            b"is_included(fd->fm)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*__tekai_state_generated_backend_writet1).t1_save_offset = 0 as ::core::ffi::c_int as integer;
    if !((*(*(*__tekai_state_generated_backend_writet1).fd_cur).fm).type_0 as ::core::ffi::c_int
        & F_SUBSETTED
        != 0 as ::core::ffi::c_int)
    {
        (*fd).ff_found = t1_open_fontfile(b"<<\0" as *const u8 as *const ::core::ffi::c_char);
        if (*fd).ff_found == 0 {
            return;
        }
        t1_include();
        t1_close_font_file(b">>\0" as *const u8 as *const ::core::ffi::c_char);
        return;
    }
    (*fd).ff_found = t1_open_fontfile(b"<\0" as *const u8 as *const ::core::ffi::c_char);
    if (*fd).ff_found == 0 {
        return;
    }
    t1_subset_ascii_part();
    t1_start_eexec();
    cc_init();
    cs_init();
    t1_read_subrs();
    t1_subset_charstrings();
    t1_subset_end();
    t1_close_font_file(b">\0" as *const u8 as *const ::core::ffi::c_char);
}
pub unsafe fn t1_free() {
    let __tekai_state_generated_backend_writet1: *mut crate::generated::backend::writet1::State =
        unsafe { crate::generated::backend::writet1::state() };

    if !(*__tekai_state_generated_backend_writet1)
        .t1_line_array
        .is_null()
    {
        free((*__tekai_state_generated_backend_writet1).t1_line_array as *mut ::core::ffi::c_void);
    }
    (*__tekai_state_generated_backend_writet1).t1_line_array =
        ::core::ptr::null_mut::<t1_line_entry>();
    if !(*__tekai_state_generated_backend_writet1)
        .t1_buf_array
        .is_null()
    {
        free((*__tekai_state_generated_backend_writet1).t1_buf_array as *mut ::core::ffi::c_void);
    }
    (*__tekai_state_generated_backend_writet1).t1_buf_array =
        ::core::ptr::null_mut::<t1_buf_entry>();
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;

#[cfg(test)]
mod tests {
    use super::{cs_entry, init_cs_entry, release_cs_entries, CS_RELEASE_CALLS};
    use std::ptr;

    struct OwnedEntries(Vec<cs_entry>);

    impl OwnedEntries {
        fn new(count: usize) -> Self {
            let mut entries = Vec::with_capacity(count);
            for _ in 0..count {
                let mut entry = std::mem::MaybeUninit::<cs_entry>::uninit();
                unsafe {
                    init_cs_entry(entry.as_mut_ptr());
                    entries.push(entry.assume_init());
                }
            }
            Self(entries)
        }

        fn add_owned(&mut self, index: usize, with_name: bool) {
            let entry = &mut self.0[index];
            unsafe {
                entry.data = crate::allocation::malloc(1) as *mut super::byte;
                assert!(!entry.data.is_null());
                *entry.data = index as super::byte;
                if with_name {
                    entry.name = crate::allocation::malloc(2) as *mut libc::c_char;
                    assert!(!entry.name.is_null());
                    *entry.name = b'a' as libc::c_char;
                    *entry.name.add(1) = 0;
                }
            }
            entry.len = 1;
            entry.cslen = 1;
            entry.valid = 1;
        }

        fn release_tail(&mut self, emitted_count: usize) {
            assert!(emitted_count <= self.0.len());
            let count = self.0.len();
            let tab = self.0.as_mut_ptr();
            unsafe { release_cs_entries(tab.add(emitted_count), tab.add(count)) };
        }
    }

    impl Drop for OwnedEntries {
        fn drop(&mut self) {
            self.release_tail(0);
        }
    }

    fn reset_release_calls() {
        CS_RELEASE_CALLS.with(|calls| calls.set(0));
    }

    fn release_calls() -> usize {
        CS_RELEASE_CALLS.with(|calls| calls.get())
    }

    #[test]
    fn subroutine_cleanup_releases_unused_tail_only() {
        crate::invocation_state::with_fixture(|| {
            let mut entries = OwnedEntries::new(5);
            for index in 0..5 {
                entries.add_owned(index, true);
            }
            let prefix_data = [entries.0[0].data, entries.0[1].data];
            let prefix_names = [entries.0[0].name, entries.0[1].name];
            reset_release_calls();

            entries.release_tail(2);

            assert_eq!(release_calls(), 6);
            for index in 0..2 {
                assert_eq!(entries.0[index].data, prefix_data[index]);
                assert_eq!(entries.0[index].name, prefix_names[index]);
                assert_eq!(unsafe { *entries.0[index].data }, index as super::byte);
            }
            for entry in &entries.0[2..] {
                assert!(entry.data.is_null());
                assert!(entry.name.is_null());
                assert_eq!(entry.len, 1);
                assert_eq!(entry.cslen, 1);
                assert_eq!(entry.valid, 1);
                assert_eq!(entry.used, 0);
            }
        });
    }

    #[test]
    fn subroutine_cleanup_skips_interior_holes() {
        crate::invocation_state::with_fixture(|| {
            let mut entries = OwnedEntries::new(5);
            entries.add_owned(0, true);
            entries.add_owned(2, false);
            entries.add_owned(4, true);
            reset_release_calls();

            entries.release_tail(0);

            assert_eq!(release_calls(), 5);
            assert!(entries.0.iter().all(|entry| entry.data.is_null()));
            assert!(entries.0.iter().all(|entry| entry.name.is_null()));
            assert_eq!(entries.0[1].valid, 0);
            assert_eq!(entries.0[3].valid, 0);
            entries.release_tail(0);
            assert_eq!(release_calls(), 5);
        });
    }

    #[test]
    fn subroutine_cleanup_handles_null_entries() {
        crate::invocation_state::with_fixture(|| {
            let mut entries = OwnedEntries::new(3);
            reset_release_calls();

            entries.release_tail(0);
            unsafe { release_cs_entries(ptr::null_mut(), ptr::null_mut()) };

            assert_eq!(release_calls(), 0);
        });
    }

    #[test]
    fn subroutine_cleanup_at_full_cutoff_releases_nothing() {
        crate::invocation_state::with_fixture(|| {
            let mut entries = OwnedEntries::new(3);
            for index in 0..3 {
                entries.add_owned(index, true);
            }
            let data = entries.0.iter().map(|entry| entry.data).collect::<Vec<_>>();
            let names = entries.0.iter().map(|entry| entry.name).collect::<Vec<_>>();
            reset_release_calls();

            entries.release_tail(3);

            assert_eq!(release_calls(), 0);
            assert_eq!(
                entries.0.iter().map(|entry| entry.data).collect::<Vec<_>>(),
                data
            );
            assert_eq!(
                entries.0.iter().map(|entry| entry.name).collect::<Vec<_>>(),
                names
            );
        });
    }

    #[test]
    fn subroutine_cleanup_with_zero_emitted_entries_releases_all() {
        crate::invocation_state::with_fixture(|| {
            let mut entries = OwnedEntries::new(3);
            for index in 0..3 {
                entries.add_owned(index, true);
            }
            reset_release_calls();

            entries.release_tail(0);

            assert_eq!(release_calls(), 6);
            assert!(entries.0.iter().all(|entry| entry.data.is_null()));
            assert!(entries.0.iter().all(|entry| entry.name.is_null()));
        });
    }

    #[test]
    fn subroutine_cleanup_preserves_borrowed_notdef_name() {
        crate::invocation_state::with_fixture(|| {
            let __tekai_state_generated_backend_writettf: *mut crate::generated::backend::writettf::State = unsafe { crate::generated::backend::writettf::state() };

            let mut entries = OwnedEntries::new(1);
            entries.add_owned(0, false);
            let borrowed = unsafe {
                &raw mut (*__tekai_state_generated_backend_writettf).notdef as *mut libc::c_char
            };
            entries.0[0].name = borrowed;
            reset_release_calls();

            entries.release_tail(0);

            assert_eq!(release_calls(), 1);
            assert!(entries.0[0].data.is_null());
            assert_eq!(entries.0[0].name, borrowed);
            entries.release_tail(0);
            assert_eq!(release_calls(), 1);
        });
    }

    #[test]
    fn subroutine_cleanup_releases_name_only_entry() {
        crate::invocation_state::with_fixture(|| {
            let mut entries = OwnedEntries::new(1);
            entries.0[0].name = unsafe { crate::allocation::malloc(1) as *mut libc::c_char };
            assert!(!entries.0[0].name.is_null());
            reset_release_calls();

            entries.release_tail(0);

            assert_eq!(release_calls(), 1);
            assert!(entries.0[0].data.is_null());
            assert!(entries.0[0].name.is_null());
            assert_eq!(entries.0[0].valid, 0);
        });
    }
}

pub(crate) struct State {
    font_key: [key_entry; 11],
    standard_glyph_names: [*const ::core::ffi::c_char; 256],
    pub(crate) t1_length1: integer,
    pub(crate) t1_length3: integer,
    pub(crate) t1_length2: integer,
    t1_save_offset: integer,
    t1_fontname_offset: integer,
    fd_cur: *mut fd_entry,
    charstringname: [::core::ffi::c_char; 13],
    t1_encoding: C2RustUnnamed_0,
    t1_dr: ::core::ffi::c_ushort,
    t1_er: ::core::ffi::c_ushort,
    t1_c2: ::core::ffi::c_ushort,
    t1_c1: ::core::ffi::c_ushort,
    t1_cslen: ::core::ffi::c_ushort,
    t1_lenIV: ::core::ffi::c_short,
    enc_line: [::core::ffi::c_char; 4096],
    t1_line_ptr: *mut t1_line_entry,
    t1_line_array: *mut t1_line_entry,
    t1_line_limit: size_t,
    t1_buf_ptr: *mut t1_buf_entry,
    t1_buf_array: *mut t1_buf_entry,
    t1_buf_limit: size_t,
    cs_start: ::core::ffi::c_int,
    cs_tab: *mut cs_entry,
    cs_ptr: *mut cs_entry,
    cs_notdef: *mut cs_entry,
    cs_dict_end: *mut ::core::ffi::c_char,
    cs_dict_start: *mut ::core::ffi::c_char,
    cs_count: ::core::ffi::c_int,
    cs_size_pos: ::core::ffi::c_int,
    cs_size: ::core::ffi::c_int,
    subr_tab: *mut cs_entry,
    subr_array_start: *mut ::core::ffi::c_char,
    subr_array_end: *mut ::core::ffi::c_char,
    subr_max: ::core::ffi::c_int,
    subr_size: ::core::ffi::c_int,
    subr_size_pos: ::core::ffi::c_int,
    cs_token_pairs_list: [[*const ::core::ffi::c_char; 2]; 5],
    cs_token_pair: *mut *const ::core::ffi::c_char,
    t1_eexec_encrypt: boolean,
    t1_scan: boolean,
    t1_synthetic: boolean,
    t1_pfa: boolean,
    t1_cs: boolean,
    t1_in_eexec: ::core::ffi::c_int,
    t1_block_length: ::core::ffi::c_long,
    last_hexbyte: ::core::ffi::c_int,
    t1_file: *mut FILE,
    enc_file: *mut FILE,
    local_t1_getline_993_eexec_str: [::core::ffi::c_char; 18],
    local_t1_getline_996_eexec_len: ::core::ffi::c_int,
    local_t1_scan_param_1806_lenIV: *const ::core::ffi::c_char,
    stack_ptr: *mut integer,
    cc_stack: [integer; 24],
    cc_tab: [cc_entry; 66],
    is_cc_init: boolean,
    local_cs_mark_2648_lastargOtherSubr3: integer,
    local_create_t1_glyph_tree_3011_gl_tree: *mut avl_table,
}

#[inline(always)]
fn __tekai_require_copy<T: Copy>() {}

pub(crate) struct StateInitialization {
    destination: *mut State,
    initialized: [bool; 59],
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
            if self.initialized[58] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).local_create_t1_glyph_tree_3011_gl_tree
                ));
            }
            if self.initialized[57] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).local_cs_mark_2648_lastargOtherSubr3
                ));
            }
            if self.initialized[56] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).is_cc_init));
            }
            if self.initialized[55] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).cc_tab));
            }
            if self.initialized[54] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).cc_stack));
            }
            if self.initialized[53] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).stack_ptr));
            }
            if self.initialized[52] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).local_t1_scan_param_1806_lenIV
                ));
            }
            if self.initialized[51] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).local_t1_getline_996_eexec_len
                ));
            }
            if self.initialized[50] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).local_t1_getline_993_eexec_str
                ));
            }
            if self.initialized[49] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).enc_file));
            }
            if self.initialized[48] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_file));
            }
            if self.initialized[47] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).last_hexbyte));
            }
            if self.initialized[46] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_block_length));
            }
            if self.initialized[45] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_in_eexec));
            }
            if self.initialized[44] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_cs));
            }
            if self.initialized[43] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_pfa));
            }
            if self.initialized[42] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_synthetic));
            }
            if self.initialized[41] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_scan));
            }
            if self.initialized[40] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).t1_eexec_encrypt
                ));
            }
            if self.initialized[39] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).cs_token_pair));
            }
            if self.initialized[38] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).cs_token_pairs_list
                ));
            }
            if self.initialized[37] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).subr_size_pos));
            }
            if self.initialized[36] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).subr_size));
            }
            if self.initialized[35] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).subr_max));
            }
            if self.initialized[34] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).subr_array_end));
            }
            if self.initialized[33] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).subr_array_start
                ));
            }
            if self.initialized[32] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).subr_tab));
            }
            if self.initialized[31] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).cs_size));
            }
            if self.initialized[30] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).cs_size_pos));
            }
            if self.initialized[29] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).cs_count));
            }
            if self.initialized[28] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).cs_dict_start));
            }
            if self.initialized[27] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).cs_dict_end));
            }
            if self.initialized[26] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).cs_notdef));
            }
            if self.initialized[25] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).cs_ptr));
            }
            if self.initialized[24] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).cs_tab));
            }
            if self.initialized[23] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).cs_start));
            }
            if self.initialized[22] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_buf_limit));
            }
            if self.initialized[21] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_buf_array));
            }
            if self.initialized[20] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_buf_ptr));
            }
            if self.initialized[19] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_line_limit));
            }
            if self.initialized[18] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_line_array));
            }
            if self.initialized[17] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_line_ptr));
            }
            if self.initialized[16] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).enc_line));
            }
            if self.initialized[15] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_lenIV));
            }
            if self.initialized[14] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_cslen));
            }
            if self.initialized[13] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_c1));
            }
            if self.initialized[12] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_c2));
            }
            if self.initialized[11] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_er));
            }
            if self.initialized[10] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_dr));
            }
            if self.initialized[9] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_encoding));
            }
            if self.initialized[8] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).charstringname));
            }
            if self.initialized[7] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fd_cur));
            }
            if self.initialized[6] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).t1_fontname_offset
                ));
            }
            if self.initialized[5] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_save_offset));
            }
            if self.initialized[4] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_length2));
            }
            if self.initialized[3] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_length3));
            }
            if self.initialized[2] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t1_length1));
            }
            if self.initialized[1] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).standard_glyph_names
                ));
            }
            if self.initialized[0] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).font_key));
            }
        }
    }
}
impl StateInitialization {
    pub(crate) unsafe fn begin(destination: *mut State) -> Self {
        Self {
            destination,
            initialized: [false; 59],
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
            assert!((11) == 11);
        }
        __tekai_require_copy::<key_entry>();
        const {
            assert!(core::mem::size_of::<key_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).font_key[0]).write(key_entry {
            pdfname: b"Ascent\0" as *const u8 as *const ::core::ffi::c_char,
            t1name: b"Ascender\0" as *const u8 as *const ::core::ffi::c_char,
            value: 0.,
            valid: 0,
        });
        const {
            assert!(core::mem::size_of::<key_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).font_key[1]).write(key_entry {
            pdfname: b"CapHeight\0" as *const u8 as *const ::core::ffi::c_char,
            t1name: b"CapHeight\0" as *const u8 as *const ::core::ffi::c_char,
            value: 0.,
            valid: 0,
        });
        const {
            assert!(core::mem::size_of::<key_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).font_key[2]).write(key_entry {
            pdfname: b"Descent\0" as *const u8 as *const ::core::ffi::c_char,
            t1name: b"Descender\0" as *const u8 as *const ::core::ffi::c_char,
            value: 0.,
            valid: 0,
        });
        const {
            assert!(core::mem::size_of::<key_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).font_key[3]).write(key_entry {
            pdfname: b"ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char,
            t1name: b"ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char,
            value: 0.,
            valid: 0,
        });
        const {
            assert!(core::mem::size_of::<key_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).font_key[4]).write(key_entry {
            pdfname: b"StemV\0" as *const u8 as *const ::core::ffi::c_char,
            t1name: b"StdVW\0" as *const u8 as *const ::core::ffi::c_char,
            value: 0.,
            valid: 0,
        });
        const {
            assert!(core::mem::size_of::<key_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).font_key[5]).write(key_entry {
            pdfname: b"XHeight\0" as *const u8 as *const ::core::ffi::c_char,
            t1name: b"XHeight\0" as *const u8 as *const ::core::ffi::c_char,
            value: 0.,
            valid: 0,
        });
        const {
            assert!(core::mem::size_of::<key_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).font_key[6]).write(key_entry {
            pdfname: b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char,
            t1name: b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char,
            value: 0.,
            valid: 0,
        });
        const {
            assert!(core::mem::size_of::<key_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).font_key[7]).write(key_entry {
            pdfname: b"\0" as *const u8 as *const ::core::ffi::c_char,
            t1name: b"\0" as *const u8 as *const ::core::ffi::c_char,
            value: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
            valid: 0,
        });
        const {
            assert!(core::mem::size_of::<key_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).font_key[8]).write(key_entry {
            pdfname: b"\0" as *const u8 as *const ::core::ffi::c_char,
            t1name: b"\0" as *const u8 as *const ::core::ffi::c_char,
            value: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
            valid: 0,
        });
        const {
            assert!(core::mem::size_of::<key_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).font_key[9]).write(key_entry {
            pdfname: b"\0" as *const u8 as *const ::core::ffi::c_char,
            t1name: b"\0" as *const u8 as *const ::core::ffi::c_char,
            value: 0 as ::core::ffi::c_int as ::core::ffi::c_float,
            valid: 0,
        });
        const {
            assert!(core::mem::size_of::<key_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).font_key[10]).write(key_entry {
            pdfname: b"FontName\0" as *const u8 as *const ::core::ffi::c_char,
            t1name: b"FontName\0" as *const u8 as *const ::core::ffi::c_char,
            value: 0.,
            valid: 0,
        });
        self.initialized[0] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_length1).write(0);
        self.initialized[2] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_length3).write(0);
        self.initialized[3] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_length2).write(0);
        self.initialized[4] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_save_offset).write(0);
        self.initialized[5] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_fontname_offset).write(0);
        self.initialized[6] = true;
        const {
            assert!(core::mem::size_of::<*mut fd_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fd_cur)
            .write(::core::ptr::null::<fd_entry>() as *mut fd_entry);
        self.initialized[7] = true;
        const {
            assert!(core::mem::size_of::<[::core::ffi::c_char; 13]>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).charstringname).write(unsafe {
            ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"/CharStrings\0")
        });
        self.initialized[8] = true;
        const {
            assert!(core::mem::size_of::<C2RustUnnamed_0>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_encoding).write(ENC_STANDARD);
        self.initialized[9] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_ushort>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_dr).write(0);
        self.initialized[10] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_ushort>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_er).write(0);
        self.initialized[11] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_ushort>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_c2).write(22719 as ::core::ffi::c_ushort);
        self.initialized[12] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_ushort>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_c1).write(52845 as ::core::ffi::c_ushort);
        self.initialized[13] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_ushort>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_cslen).write(0);
        self.initialized[14] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_short>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_lenIV).write(0);
        self.initialized[15] = true;
        const {
            assert!((4096) == (4096));
            assert!(core::mem::size_of::<::core::ffi::c_char>() <= 16 * 1024);
        }
        __tekai_require_copy::<::core::ffi::c_char>();
        {
            let __tekai_repeat_0: ::core::ffi::c_char = 0;
            for __tekai_index_0 in 0..(4096) {
                core::ptr::addr_of_mut!((*destination).enc_line[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[16] = true;
        const {
            assert!(core::mem::size_of::<*mut t1_line_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_line_ptr)
            .write(::core::ptr::null::<t1_line_entry>() as *mut t1_line_entry);
        self.initialized[17] = true;
        const {
            assert!(core::mem::size_of::<*mut t1_line_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_line_array)
            .write(::core::ptr::null::<t1_line_entry>() as *mut t1_line_entry);
        self.initialized[18] = true;
        const {
            assert!(core::mem::size_of::<size_t>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_line_limit).write(0);
        self.initialized[19] = true;
        const {
            assert!(core::mem::size_of::<*mut t1_buf_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_buf_ptr)
            .write(::core::ptr::null::<t1_buf_entry>() as *mut t1_buf_entry);
        self.initialized[20] = true;
        const {
            assert!(core::mem::size_of::<*mut t1_buf_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_buf_array)
            .write(::core::ptr::null::<t1_buf_entry>() as *mut t1_buf_entry);
        self.initialized[21] = true;
        const {
            assert!(core::mem::size_of::<size_t>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_buf_limit).write(0);
        self.initialized[22] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_start).write(0);
        self.initialized[23] = true;
        const {
            assert!(core::mem::size_of::<*mut cs_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_tab)
            .write(::core::ptr::null::<cs_entry>() as *mut cs_entry);
        self.initialized[24] = true;
        const {
            assert!(core::mem::size_of::<*mut cs_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_ptr)
            .write(::core::ptr::null::<cs_entry>() as *mut cs_entry);
        self.initialized[25] = true;
        const {
            assert!(core::mem::size_of::<*mut cs_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_notdef)
            .write(::core::ptr::null::<cs_entry>() as *mut cs_entry);
        self.initialized[26] = true;
        const {
            assert!(core::mem::size_of::<*mut ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_dict_end)
            .write(::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char);
        self.initialized[27] = true;
        const {
            assert!(core::mem::size_of::<*mut ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_dict_start)
            .write(::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char);
        self.initialized[28] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_count).write(0);
        self.initialized[29] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_size_pos).write(0);
        self.initialized[30] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_size).write(0);
        self.initialized[31] = true;
        const {
            assert!(core::mem::size_of::<*mut cs_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).subr_tab)
            .write(::core::ptr::null::<cs_entry>() as *mut cs_entry);
        self.initialized[32] = true;
        const {
            assert!(core::mem::size_of::<*mut ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).subr_array_start)
            .write(::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char);
        self.initialized[33] = true;
        const {
            assert!(core::mem::size_of::<*mut ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).subr_array_end)
            .write(::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char);
        self.initialized[34] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).subr_max).write(0);
        self.initialized[35] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).subr_size).write(0);
        self.initialized[36] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).subr_size_pos).write(0);
        self.initialized[37] = true;
        const {
            assert!((5) == 5);
        }
        __tekai_require_copy::<[*const ::core::ffi::c_char; 2]>();
        const {
            assert!((2) == 2);
        }
        __tekai_require_copy::<*const ::core::ffi::c_char>();
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_token_pairs_list[0][0])
            .write(b" RD\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_token_pairs_list[0][1])
            .write(b"NP\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!((2) == 2);
        }
        __tekai_require_copy::<*const ::core::ffi::c_char>();
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_token_pairs_list[1][0])
            .write(b" -|\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_token_pairs_list[1][1])
            .write(b"|\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!((2) == 2);
        }
        __tekai_require_copy::<*const ::core::ffi::c_char>();
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_token_pairs_list[2][0])
            .write(b" RD\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_token_pairs_list[2][1])
            .write(b"noaccess put\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!((2) == 2);
        }
        __tekai_require_copy::<*const ::core::ffi::c_char>();
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_token_pairs_list[3][0])
            .write(b" -|\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_token_pairs_list[3][1])
            .write(b"noaccess put\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!((2) == 2);
        }
        __tekai_require_copy::<*const ::core::ffi::c_char>();
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_token_pairs_list[4][0])
            .write(::core::ptr::null::<::core::ffi::c_char>());
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_token_pairs_list[4][1])
            .write(::core::ptr::null::<::core::ffi::c_char>());
        self.initialized[38] = true;
        const {
            assert!(core::mem::size_of::<*mut *const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cs_token_pair)
            .write(::core::ptr::null::<*const ::core::ffi::c_char>()
                as *mut *const ::core::ffi::c_char);
        self.initialized[39] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_eexec_encrypt).write(0);
        self.initialized[40] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_scan).write(0);
        self.initialized[41] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_synthetic).write(0);
        self.initialized[42] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_pfa).write(0);
        self.initialized[43] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_cs).write(0);
        self.initialized[44] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_in_eexec).write(0);
        self.initialized[45] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_long>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_block_length).write(0);
        self.initialized[46] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).last_hexbyte).write(0);
        self.initialized[47] = true;
        const {
            assert!(core::mem::size_of::<*mut FILE>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t1_file)
            .write(::core::ptr::null::<FILE>() as *mut FILE);
        self.initialized[48] = true;
        const {
            assert!(core::mem::size_of::<*mut FILE>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).enc_file)
            .write(::core::ptr::null::<FILE>() as *mut FILE);
        self.initialized[49] = true;
        const {
            assert!(core::mem::size_of::<[::core::ffi::c_char; 18]>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_t1_getline_993_eexec_str).write(unsafe {
            ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"currentfile eexec\0")
        });
        self.initialized[50] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_t1_getline_996_eexec_len)
            .write(17 as ::core::ffi::c_int);
        self.initialized[51] = true;
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_t1_scan_param_1806_lenIV)
            .write(b"/lenIV\0" as *const u8 as *const ::core::ffi::c_char);
        self.initialized[52] = true;
        const {
            assert!((24) == (24));
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        __tekai_require_copy::<integer>();
        {
            let __tekai_repeat_0: integer = 0;
            for __tekai_index_0 in 0..(24) {
                core::ptr::addr_of_mut!((*destination).cc_stack[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[54] = true;
        const {
            assert!((66) == (66));
            assert!(core::mem::size_of::<cc_entry>() <= 16 * 1024);
        }
        __tekai_require_copy::<cc_entry>();
        {
            let __tekai_repeat_0: cc_entry = cc_entry {
                nargs: 0,
                bottom: 0,
                clear: 0,
                valid: 0,
            };
            for __tekai_index_0 in 0..(66) {
                core::ptr::addr_of_mut!((*destination).cc_tab[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[55] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).is_cc_init).write(false_0);
        self.initialized[56] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_cs_mark_2648_lastargOtherSubr3)
            .write(3 as integer);
        self.initialized[57] = true;
        const {
            assert!(core::mem::size_of::<*mut avl_table>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_create_t1_glyph_tree_3011_gl_tree)
            .write(::core::ptr::null::<avl_table>() as *mut avl_table);
        self.initialized[58] = true;
    }
    pub(crate) unsafe fn initialize_fixups(
        &mut self,
        invocation: *const crate::invocation_state::Storage,
    ) {
        let destination = self.destination;
        let _ = invocation;
        let __tekai_state_generated_backend_writettf = core::cell::UnsafeCell::raw_get(
            core::ptr::addr_of!((*invocation).generated_backend_writettf),
        );
        const {
            assert!((256) == 256);
        }
        __tekai_require_copy::<*const ::core::ffi::c_char>();
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[0]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[1]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[2]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[3]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[4]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[5]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[6]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[7]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[8]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[9]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[10]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[11]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[12]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[13]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[14]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[15]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[16]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[17]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[18]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[19]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[20]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[21]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[22]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[23]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[24]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[25]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[26]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[27]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[28]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[29]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[30]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[31]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[32])
            .write(b"space\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[33])
            .write(b"exclam\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[34])
            .write(b"quotedbl\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[35])
            .write(b"numbersign\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[36])
            .write(b"dollar\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[37])
            .write(b"percent\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[38])
            .write(b"ampersand\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[39])
            .write(b"quoteright\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[40])
            .write(b"parenleft\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[41])
            .write(b"parenright\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[42])
            .write(b"asterisk\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[43])
            .write(b"plus\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[44])
            .write(b"comma\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[45])
            .write(b"hyphen\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[46])
            .write(b"period\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[47])
            .write(b"slash\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[48])
            .write(b"zero\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[49])
            .write(b"one\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[50])
            .write(b"two\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[51])
            .write(b"three\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[52])
            .write(b"four\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[53])
            .write(b"five\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[54])
            .write(b"six\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[55])
            .write(b"seven\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[56])
            .write(b"eight\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[57])
            .write(b"nine\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[58])
            .write(b"colon\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[59])
            .write(b"semicolon\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[60])
            .write(b"less\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[61])
            .write(b"equal\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[62])
            .write(b"greater\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[63])
            .write(b"question\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[64])
            .write(b"at\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[65])
            .write(b"A\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[66])
            .write(b"B\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[67])
            .write(b"C\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[68])
            .write(b"D\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[69])
            .write(b"E\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[70])
            .write(b"F\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[71])
            .write(b"G\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[72])
            .write(b"H\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[73])
            .write(b"I\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[74])
            .write(b"J\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[75])
            .write(b"K\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[76])
            .write(b"L\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[77])
            .write(b"M\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[78])
            .write(b"N\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[79])
            .write(b"O\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[80])
            .write(b"P\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[81])
            .write(b"Q\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[82])
            .write(b"R\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[83])
            .write(b"S\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[84])
            .write(b"T\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[85])
            .write(b"U\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[86])
            .write(b"V\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[87])
            .write(b"W\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[88])
            .write(b"X\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[89])
            .write(b"Y\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[90])
            .write(b"Z\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[91])
            .write(b"bracketleft\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[92])
            .write(b"backslash\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[93])
            .write(b"bracketright\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[94])
            .write(b"asciicircum\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[95])
            .write(b"underscore\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[96])
            .write(b"quoteleft\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[97])
            .write(b"a\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[98])
            .write(b"b\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[99])
            .write(b"c\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[100])
            .write(b"d\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[101])
            .write(b"e\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[102])
            .write(b"f\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[103])
            .write(b"g\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[104])
            .write(b"h\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[105])
            .write(b"i\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[106])
            .write(b"j\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[107])
            .write(b"k\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[108])
            .write(b"l\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[109])
            .write(b"m\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[110])
            .write(b"n\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[111])
            .write(b"o\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[112])
            .write(b"p\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[113])
            .write(b"q\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[114])
            .write(b"r\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[115])
            .write(b"s\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[116])
            .write(b"t\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[117])
            .write(b"u\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[118])
            .write(b"v\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[119])
            .write(b"w\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[120])
            .write(b"x\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[121])
            .write(b"y\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[122])
            .write(b"z\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[123])
            .write(b"braceleft\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[124])
            .write(b"bar\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[125])
            .write(b"braceright\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[126])
            .write(b"asciitilde\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[127]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[128]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[129]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[130]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[131]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[132]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[133]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[134]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[135]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[136]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[137]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[138]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[139]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[140]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[141]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[142]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[143]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[144]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[145]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[146]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[147]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[148]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[149]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[150]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[151]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[152]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[153]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[154]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[155]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[156]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[157]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[158]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[159]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[160]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[161])
            .write(b"exclamdown\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[162])
            .write(b"cent\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[163])
            .write(b"sterling\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[164])
            .write(b"fraction\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[165])
            .write(b"yen\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[166])
            .write(b"florin\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[167])
            .write(b"section\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[168])
            .write(b"currency\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[169])
            .write(b"quotesingle\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[170])
            .write(b"quotedblleft\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[171])
            .write(b"guillemotleft\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[172])
            .write(b"guilsinglleft\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[173])
            .write(b"guilsinglright\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[174])
            .write(b"fi\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[175])
            .write(b"fl\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[176]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[177])
            .write(b"endash\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[178])
            .write(b"dagger\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[179])
            .write(b"daggerdbl\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[180])
            .write(b"periodcentered\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[181]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[182])
            .write(b"paragraph\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[183])
            .write(b"bullet\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[184])
            .write(b"quotesinglbase\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[185])
            .write(b"quotedblbase\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[186])
            .write(b"quotedblright\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[187])
            .write(b"guillemotright\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[188])
            .write(b"ellipsis\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[189])
            .write(b"perthousand\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[190]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[191])
            .write(b"questiondown\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[192]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[193])
            .write(b"grave\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[194])
            .write(b"acute\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[195])
            .write(b"circumflex\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[196])
            .write(b"tilde\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[197])
            .write(b"macron\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[198])
            .write(b"breve\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[199])
            .write(b"dotaccent\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[200])
            .write(b"dieresis\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[201]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[202])
            .write(b"ring\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[203])
            .write(b"cedilla\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[204]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[205])
            .write(b"hungarumlaut\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[206])
            .write(b"ogonek\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[207])
            .write(b"caron\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[208])
            .write(b"emdash\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[209]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[210]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[211]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[212]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[213]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[214]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[215]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[216]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[217]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[218]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[219]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[220]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[221]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[222]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[223]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[224]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[225])
            .write(b"AE\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[226]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[227])
            .write(b"ordfeminine\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[228]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[229]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[230]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[231]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[232])
            .write(b"Lslash\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[233])
            .write(b"Oslash\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[234])
            .write(b"OE\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[235])
            .write(b"ordmasculine\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[236]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[237]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[238]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[239]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[240]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[241])
            .write(b"ae\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[242]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[243]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[244]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[245])
            .write(b"dotlessi\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[246]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[247]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[248])
            .write(b"lslash\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[249])
            .write(b"oslash\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[250])
            .write(b"oe\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[251])
            .write(b"germandbls\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[252]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[253]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[254]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).standard_glyph_names[255]).write(
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
        self.initialized[1] = true;
        let __tekai_state_generated_backend_writet1 = core::cell::UnsafeCell::raw_get(
            core::ptr::addr_of!((*invocation).generated_backend_writet1),
        );
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).stack_ptr).write(unsafe {
            &raw const (*__tekai_state_generated_backend_writet1).cc_stack as *mut integer
        });
        self.initialized[53] = true;
    }
}
pub(crate) unsafe fn state() -> *mut State {
    core::cell::UnsafeCell::raw_get(core::ptr::addr_of!(
        (*crate::invocation_state::current()).generated_backend_writet1
    ))
}
#[cfg(test)]
pub(crate) unsafe fn check_state_construction(phase: u8) {
    let invocation = crate::invocation_state::current();
    let destination = state();
    assert!(!destination.is_null());
    let _ = (invocation, phase);
    __tekai_require_copy::<::core::ffi::c_char>();
    let __tekai_state_generated_backend_writettf = core::cell::UnsafeCell::raw_get(
        core::ptr::addr_of!((*invocation).generated_backend_writettf),
    );
    let expected = unsafe {
        [
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            b"space\0" as *const u8 as *const ::core::ffi::c_char,
            b"exclam\0" as *const u8 as *const ::core::ffi::c_char,
            b"quotedbl\0" as *const u8 as *const ::core::ffi::c_char,
            b"numbersign\0" as *const u8 as *const ::core::ffi::c_char,
            b"dollar\0" as *const u8 as *const ::core::ffi::c_char,
            b"percent\0" as *const u8 as *const ::core::ffi::c_char,
            b"ampersand\0" as *const u8 as *const ::core::ffi::c_char,
            b"quoteright\0" as *const u8 as *const ::core::ffi::c_char,
            b"parenleft\0" as *const u8 as *const ::core::ffi::c_char,
            b"parenright\0" as *const u8 as *const ::core::ffi::c_char,
            b"asterisk\0" as *const u8 as *const ::core::ffi::c_char,
            b"plus\0" as *const u8 as *const ::core::ffi::c_char,
            b"comma\0" as *const u8 as *const ::core::ffi::c_char,
            b"hyphen\0" as *const u8 as *const ::core::ffi::c_char,
            b"period\0" as *const u8 as *const ::core::ffi::c_char,
            b"slash\0" as *const u8 as *const ::core::ffi::c_char,
            b"zero\0" as *const u8 as *const ::core::ffi::c_char,
            b"one\0" as *const u8 as *const ::core::ffi::c_char,
            b"two\0" as *const u8 as *const ::core::ffi::c_char,
            b"three\0" as *const u8 as *const ::core::ffi::c_char,
            b"four\0" as *const u8 as *const ::core::ffi::c_char,
            b"five\0" as *const u8 as *const ::core::ffi::c_char,
            b"six\0" as *const u8 as *const ::core::ffi::c_char,
            b"seven\0" as *const u8 as *const ::core::ffi::c_char,
            b"eight\0" as *const u8 as *const ::core::ffi::c_char,
            b"nine\0" as *const u8 as *const ::core::ffi::c_char,
            b"colon\0" as *const u8 as *const ::core::ffi::c_char,
            b"semicolon\0" as *const u8 as *const ::core::ffi::c_char,
            b"less\0" as *const u8 as *const ::core::ffi::c_char,
            b"equal\0" as *const u8 as *const ::core::ffi::c_char,
            b"greater\0" as *const u8 as *const ::core::ffi::c_char,
            b"question\0" as *const u8 as *const ::core::ffi::c_char,
            b"at\0" as *const u8 as *const ::core::ffi::c_char,
            b"A\0" as *const u8 as *const ::core::ffi::c_char,
            b"B\0" as *const u8 as *const ::core::ffi::c_char,
            b"C\0" as *const u8 as *const ::core::ffi::c_char,
            b"D\0" as *const u8 as *const ::core::ffi::c_char,
            b"E\0" as *const u8 as *const ::core::ffi::c_char,
            b"F\0" as *const u8 as *const ::core::ffi::c_char,
            b"G\0" as *const u8 as *const ::core::ffi::c_char,
            b"H\0" as *const u8 as *const ::core::ffi::c_char,
            b"I\0" as *const u8 as *const ::core::ffi::c_char,
            b"J\0" as *const u8 as *const ::core::ffi::c_char,
            b"K\0" as *const u8 as *const ::core::ffi::c_char,
            b"L\0" as *const u8 as *const ::core::ffi::c_char,
            b"M\0" as *const u8 as *const ::core::ffi::c_char,
            b"N\0" as *const u8 as *const ::core::ffi::c_char,
            b"O\0" as *const u8 as *const ::core::ffi::c_char,
            b"P\0" as *const u8 as *const ::core::ffi::c_char,
            b"Q\0" as *const u8 as *const ::core::ffi::c_char,
            b"R\0" as *const u8 as *const ::core::ffi::c_char,
            b"S\0" as *const u8 as *const ::core::ffi::c_char,
            b"T\0" as *const u8 as *const ::core::ffi::c_char,
            b"U\0" as *const u8 as *const ::core::ffi::c_char,
            b"V\0" as *const u8 as *const ::core::ffi::c_char,
            b"W\0" as *const u8 as *const ::core::ffi::c_char,
            b"X\0" as *const u8 as *const ::core::ffi::c_char,
            b"Y\0" as *const u8 as *const ::core::ffi::c_char,
            b"Z\0" as *const u8 as *const ::core::ffi::c_char,
            b"bracketleft\0" as *const u8 as *const ::core::ffi::c_char,
            b"backslash\0" as *const u8 as *const ::core::ffi::c_char,
            b"bracketright\0" as *const u8 as *const ::core::ffi::c_char,
            b"asciicircum\0" as *const u8 as *const ::core::ffi::c_char,
            b"underscore\0" as *const u8 as *const ::core::ffi::c_char,
            b"quoteleft\0" as *const u8 as *const ::core::ffi::c_char,
            b"a\0" as *const u8 as *const ::core::ffi::c_char,
            b"b\0" as *const u8 as *const ::core::ffi::c_char,
            b"c\0" as *const u8 as *const ::core::ffi::c_char,
            b"d\0" as *const u8 as *const ::core::ffi::c_char,
            b"e\0" as *const u8 as *const ::core::ffi::c_char,
            b"f\0" as *const u8 as *const ::core::ffi::c_char,
            b"g\0" as *const u8 as *const ::core::ffi::c_char,
            b"h\0" as *const u8 as *const ::core::ffi::c_char,
            b"i\0" as *const u8 as *const ::core::ffi::c_char,
            b"j\0" as *const u8 as *const ::core::ffi::c_char,
            b"k\0" as *const u8 as *const ::core::ffi::c_char,
            b"l\0" as *const u8 as *const ::core::ffi::c_char,
            b"m\0" as *const u8 as *const ::core::ffi::c_char,
            b"n\0" as *const u8 as *const ::core::ffi::c_char,
            b"o\0" as *const u8 as *const ::core::ffi::c_char,
            b"p\0" as *const u8 as *const ::core::ffi::c_char,
            b"q\0" as *const u8 as *const ::core::ffi::c_char,
            b"r\0" as *const u8 as *const ::core::ffi::c_char,
            b"s\0" as *const u8 as *const ::core::ffi::c_char,
            b"t\0" as *const u8 as *const ::core::ffi::c_char,
            b"u\0" as *const u8 as *const ::core::ffi::c_char,
            b"v\0" as *const u8 as *const ::core::ffi::c_char,
            b"w\0" as *const u8 as *const ::core::ffi::c_char,
            b"x\0" as *const u8 as *const ::core::ffi::c_char,
            b"y\0" as *const u8 as *const ::core::ffi::c_char,
            b"z\0" as *const u8 as *const ::core::ffi::c_char,
            b"braceleft\0" as *const u8 as *const ::core::ffi::c_char,
            b"bar\0" as *const u8 as *const ::core::ffi::c_char,
            b"braceright\0" as *const u8 as *const ::core::ffi::c_char,
            b"asciitilde\0" as *const u8 as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            b"exclamdown\0" as *const u8 as *const ::core::ffi::c_char,
            b"cent\0" as *const u8 as *const ::core::ffi::c_char,
            b"sterling\0" as *const u8 as *const ::core::ffi::c_char,
            b"fraction\0" as *const u8 as *const ::core::ffi::c_char,
            b"yen\0" as *const u8 as *const ::core::ffi::c_char,
            b"florin\0" as *const u8 as *const ::core::ffi::c_char,
            b"section\0" as *const u8 as *const ::core::ffi::c_char,
            b"currency\0" as *const u8 as *const ::core::ffi::c_char,
            b"quotesingle\0" as *const u8 as *const ::core::ffi::c_char,
            b"quotedblleft\0" as *const u8 as *const ::core::ffi::c_char,
            b"guillemotleft\0" as *const u8 as *const ::core::ffi::c_char,
            b"guilsinglleft\0" as *const u8 as *const ::core::ffi::c_char,
            b"guilsinglright\0" as *const u8 as *const ::core::ffi::c_char,
            b"fi\0" as *const u8 as *const ::core::ffi::c_char,
            b"fl\0" as *const u8 as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            b"endash\0" as *const u8 as *const ::core::ffi::c_char,
            b"dagger\0" as *const u8 as *const ::core::ffi::c_char,
            b"daggerdbl\0" as *const u8 as *const ::core::ffi::c_char,
            b"periodcentered\0" as *const u8 as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            b"paragraph\0" as *const u8 as *const ::core::ffi::c_char,
            b"bullet\0" as *const u8 as *const ::core::ffi::c_char,
            b"quotesinglbase\0" as *const u8 as *const ::core::ffi::c_char,
            b"quotedblbase\0" as *const u8 as *const ::core::ffi::c_char,
            b"quotedblright\0" as *const u8 as *const ::core::ffi::c_char,
            b"guillemotright\0" as *const u8 as *const ::core::ffi::c_char,
            b"ellipsis\0" as *const u8 as *const ::core::ffi::c_char,
            b"perthousand\0" as *const u8 as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            b"questiondown\0" as *const u8 as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            b"grave\0" as *const u8 as *const ::core::ffi::c_char,
            b"acute\0" as *const u8 as *const ::core::ffi::c_char,
            b"circumflex\0" as *const u8 as *const ::core::ffi::c_char,
            b"tilde\0" as *const u8 as *const ::core::ffi::c_char,
            b"macron\0" as *const u8 as *const ::core::ffi::c_char,
            b"breve\0" as *const u8 as *const ::core::ffi::c_char,
            b"dotaccent\0" as *const u8 as *const ::core::ffi::c_char,
            b"dieresis\0" as *const u8 as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            b"ring\0" as *const u8 as *const ::core::ffi::c_char,
            b"cedilla\0" as *const u8 as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            b"hungarumlaut\0" as *const u8 as *const ::core::ffi::c_char,
            b"ogonek\0" as *const u8 as *const ::core::ffi::c_char,
            b"caron\0" as *const u8 as *const ::core::ffi::c_char,
            b"emdash\0" as *const u8 as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            b"AE\0" as *const u8 as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            b"ordfeminine\0" as *const u8 as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            b"Lslash\0" as *const u8 as *const ::core::ffi::c_char,
            b"Oslash\0" as *const u8 as *const ::core::ffi::c_char,
            b"OE\0" as *const u8 as *const ::core::ffi::c_char,
            b"ordmasculine\0" as *const u8 as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            b"ae\0" as *const u8 as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            b"dotlessi\0" as *const u8 as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            b"lslash\0" as *const u8 as *const ::core::ffi::c_char,
            b"oslash\0" as *const u8 as *const ::core::ffi::c_char,
            b"oe\0" as *const u8 as *const ::core::ffi::c_char,
            b"germandbls\0" as *const u8 as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        ]
    };
    assert_eq!((*destination).standard_glyph_names, expected);
    for (pointer, expected_pointer) in (*destination)
        .standard_glyph_names
        .iter()
        .zip(expected.iter())
    {
        assert!(!pointer.is_null());
        assert_eq!(pointer.read(), expected_pointer.read());
    }
    __tekai_require_copy::<integer>();
    let __tekai_state_generated_backend_writet1 = core::cell::UnsafeCell::raw_get(
        core::ptr::addr_of!((*invocation).generated_backend_writet1),
    );
    let expected =
        unsafe { &raw const (*__tekai_state_generated_backend_writet1).cc_stack as *mut integer };
    assert_eq!((*destination).stack_ptr, expected);
    assert!(!(*destination).stack_ptr.is_null());
    assert_eq!((*destination).stack_ptr.read(), expected.read());
    let initial: integer = 0;
    if phase == 0 {
        assert_eq!((*destination).stack_ptr.read(), initial);
        (*destination).stack_ptr.write(initial.wrapping_add(1));
    } else {
        assert_eq!((*destination).stack_ptr.read(), initial.wrapping_add(1));
        (*destination).stack_ptr.write(initial);
    }
}
