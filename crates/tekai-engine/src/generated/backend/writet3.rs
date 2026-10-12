pub use crate::generated::backend::avlstuff::comp_string_entry;
pub use crate::generated::backend::pkin::readchar;
pub use crate::generated::backend::tounicode::write_tounicode;
pub use crate::generated::backend::writeenc::get_fe_entry;
pub use crate::generated::pdftex0::{
    getnullstr, makenamestring, pdfbeginstream, pdfenddict, pdfendobj, pdfendstream, pdfflush,
    pdfnewobjnum, zdividescaled, zgetcharwidth, zpackfilename, zpdfbegindict, zpdfbeginobj,
    zpdfnewdict, zpdfosgetosbuf, zpdfprint, zpdfprintreal,
};
pub use crate::generated::pdftexextra::makecstring;
pub(crate) use crate::kpathsea::{kpse_bitmap_tolerance, kpse_find_glyph, kpse_magstep_fix};
pub use crate::openclose::recorder_record_input;
pub use crate::utils::{maketexstring, pdf_puts};

pub use crate::generated::backend::avl::{avl_create, avl_destroy, avl_find, avl_probe, avl_table};

use crate::byte_string::{strchr, strcmp, strlen, strncmp};

use crate::backend_grammar::glyph_preamble;
use crate::float_parse::font_scale_line;
use crate::openclose::open_input;
use crate::stdio::feof;
use crate::support::{xfclose, xfopen};
use crate::utils::xgetc;
use crate::web2c::zround;

use crate::allocation::free;
use crate::openclose::{xmalloc, xrealloc};

use crate::termination::__assert_rtn;

pub type __int64_t = i64;
pub type __darwin_size_t = usize;
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
pub type eightbits = ::core::ffi::c_uchar;
pub type strnumber = integer;
pub type scaled = integer;
pub type halfword = integer;
pub type internalfontnumber = integer;
pub type charusedarray = [eightbits; 32];
pub use crate::backend_types::_subfont_entry;
pub use crate::backend_types::key_entry;
pub type subfont_entry = _subfont_entry;
pub use crate::backend_types::chardesc;
pub use crate::backend_types::fe_entry;
pub use crate::backend_types::fm_entry;
pub type t3_line_entry = ::core::ffi::c_char;
pub use crate::kpathsea_types::KpseGlyphFileType as kpse_glyph_file_type;
pub type kpse_glyph_source_type = ::core::ffi::c_uint;
pub const kpse_glyph_source_fallback: kpse_glyph_source_type = 4;
pub const kpse_glyph_source_fallback_res: kpse_glyph_source_type = 3;
pub const kpse_glyph_source_maketex: kpse_glyph_source_type = 2;
pub const kpse_glyph_source_alias: kpse_glyph_source_type = 1;
pub const kpse_glyph_source_normal: kpse_glyph_source_type = 0;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const FOPEN_RBIN_MODE: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"rb\0") };
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FONTBBOX1_CODE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

unsafe fn t3_getline() {
    let __tekai_state_generated_backend_writet3: *mut crate::generated::backend::writet3::State =
        unsafe { crate::generated::backend::writet3::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    let mut c: ::core::ffi::c_int = 0;
    loop {
        (*__tekai_state_generated_backend_writet3).t3_line_ptr =
            (*__tekai_state_generated_backend_writet3).t3_line_array;
        c = xgetc((*__tekai_state_generated_backend_writet3).t3_file);
        while feof((*__tekai_state_generated_backend_writet3).t3_file) == 0 {
            if (*__tekai_state_generated_backend_writet3)
                .t3_line_array
                .is_null()
            {
                (*__tekai_state_generated_backend_writet3).t3_line_limit = 1024 as size_t;
                if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                    > (*__tekai_state_generated_backend_writet3).t3_line_limit
                        as ::core::ffi::c_uint
                {
                    (*__tekai_state_generated_backend_writet3).t3_line_limit = 1 as size_t;
                }
                (*__tekai_state_generated_backend_writet3).t3_line_array = xmalloc(
                    (*__tekai_state_generated_backend_writet3)
                        .t3_line_limit
                        .wrapping_mul(::core::mem::size_of::<t3_line_entry>() as size_t),
                )
                    as *mut t3_line_entry;
                (*__tekai_state_generated_backend_writet3).t3_line_ptr =
                    (*__tekai_state_generated_backend_writet3).t3_line_array;
            } else if ((*__tekai_state_generated_backend_writet3)
                .t3_line_ptr
                .offset_from((*__tekai_state_generated_backend_writet3).t3_line_array)
                as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                > (*__tekai_state_generated_backend_writet3).t3_line_limit as ::core::ffi::c_uint
            {
                (*__tekai_state_utils).last_ptr_index = (*__tekai_state_generated_backend_writet3)
                    .t3_line_ptr
                    .offset_from((*__tekai_state_generated_backend_writet3).t3_line_array)
                    as ::core::ffi::c_long
                    as size_t;
                (*__tekai_state_generated_backend_writet3).t3_line_limit =
                    (*__tekai_state_generated_backend_writet3)
                        .t3_line_limit
                        .wrapping_mul(2 as size_t);
                if ((*__tekai_state_generated_backend_writet3)
                    .t3_line_ptr
                    .offset_from((*__tekai_state_generated_backend_writet3).t3_line_array)
                    as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                    > (*__tekai_state_generated_backend_writet3).t3_line_limit
                        as ::core::ffi::c_uint
                {
                    (*__tekai_state_generated_backend_writet3).t3_line_limit =
                        ((*__tekai_state_generated_backend_writet3)
                            .t3_line_ptr
                            .offset_from((*__tekai_state_generated_backend_writet3).t3_line_array)
                            as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as size_t;
                }
                if (*__tekai_state_generated_backend_writet3).t3_line_limit as ::core::ffi::c_uint
                    > INT_MAX as ::core::ffi::c_uint
                {
                    crate::utils::pdftex_fail_args(
                        b"t3_line_array exceeds size limit\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                }
                (*__tekai_state_generated_backend_writet3).t3_line_array = xrealloc(
                    (*__tekai_state_generated_backend_writet3).t3_line_array as address,
                    (*__tekai_state_generated_backend_writet3)
                        .t3_line_limit
                        .wrapping_mul(::core::mem::size_of::<t3_line_entry>() as size_t),
                )
                    as *mut t3_line_entry;
                (*__tekai_state_generated_backend_writet3).t3_line_ptr =
                    (*__tekai_state_generated_backend_writet3)
                        .t3_line_array
                        .offset((*__tekai_state_utils).last_ptr_index as isize);
            }
            if c == 9 as ::core::ffi::c_int {
                c = 32 as ::core::ffi::c_int;
            }
            if c == 13 as ::core::ffi::c_int || c == EOF {
                c = 10 as ::core::ffi::c_int;
            }
            if c != ' ' as i32
                || (*__tekai_state_generated_backend_writet3).t3_line_ptr
                    > (*__tekai_state_generated_backend_writet3).t3_line_array
                    && *(*__tekai_state_generated_backend_writet3)
                        .t3_line_ptr
                        .offset(-(1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        != 32 as ::core::ffi::c_int
            {
                if ((*__tekai_state_generated_backend_writet3)
                    .t3_line_ptr
                    .offset_from((*__tekai_state_generated_backend_writet3).t3_line_array)
                    as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                    > (*__tekai_state_generated_backend_writet3).t3_line_limit
                        as ::core::ffi::c_uint
                {
                    crate::utils::pdftex_fail_args(
                        b"buffer overflow at file %s, line %d\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[
                            crate::utils::PrintfArg::from(
                                b"tekai-engine/generated/backend/writet3.rs\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            ),
                            crate::utils::PrintfArg::from(60 as ::core::ffi::c_int),
                        ],
                    );
                }
                let fresh0 = (*__tekai_state_generated_backend_writet3).t3_line_ptr;
                (*__tekai_state_generated_backend_writet3).t3_line_ptr =
                    (*__tekai_state_generated_backend_writet3)
                        .t3_line_ptr
                        .offset(1);
                *fresh0 = c as t3_line_entry;
            }
            if c == 10 as ::core::ffi::c_int {
                break;
            }
            c = xgetc((*__tekai_state_generated_backend_writet3).t3_file);
        }
        if (*__tekai_state_generated_backend_writet3)
            .t3_line_array
            .is_null()
        {
            (*__tekai_state_generated_backend_writet3).t3_line_limit = 1024 as size_t;
            if 2 as ::core::ffi::c_int as ::core::ffi::c_uint
                > (*__tekai_state_generated_backend_writet3).t3_line_limit as ::core::ffi::c_uint
            {
                (*__tekai_state_generated_backend_writet3).t3_line_limit = 2 as size_t;
            }
            (*__tekai_state_generated_backend_writet3).t3_line_array = xmalloc(
                (*__tekai_state_generated_backend_writet3)
                    .t3_line_limit
                    .wrapping_mul(::core::mem::size_of::<t3_line_entry>() as size_t),
            )
                as *mut t3_line_entry;
            (*__tekai_state_generated_backend_writet3).t3_line_ptr =
                (*__tekai_state_generated_backend_writet3).t3_line_array;
        } else if ((*__tekai_state_generated_backend_writet3)
            .t3_line_ptr
            .offset_from((*__tekai_state_generated_backend_writet3).t3_line_array)
            as ::core::ffi::c_long
            + 2 as ::core::ffi::c_long) as ::core::ffi::c_uint
            > (*__tekai_state_generated_backend_writet3).t3_line_limit as ::core::ffi::c_uint
        {
            (*__tekai_state_utils).last_ptr_index = (*__tekai_state_generated_backend_writet3)
                .t3_line_ptr
                .offset_from((*__tekai_state_generated_backend_writet3).t3_line_array)
                as ::core::ffi::c_long
                as size_t;
            (*__tekai_state_generated_backend_writet3).t3_line_limit =
                (*__tekai_state_generated_backend_writet3)
                    .t3_line_limit
                    .wrapping_mul(2 as size_t);
            if ((*__tekai_state_generated_backend_writet3)
                .t3_line_ptr
                .offset_from((*__tekai_state_generated_backend_writet3).t3_line_array)
                as ::core::ffi::c_long
                + 2 as ::core::ffi::c_long) as ::core::ffi::c_uint
                > (*__tekai_state_generated_backend_writet3).t3_line_limit as ::core::ffi::c_uint
            {
                (*__tekai_state_generated_backend_writet3).t3_line_limit =
                    ((*__tekai_state_generated_backend_writet3)
                        .t3_line_ptr
                        .offset_from((*__tekai_state_generated_backend_writet3).t3_line_array)
                        as ::core::ffi::c_long
                        + 2 as ::core::ffi::c_long) as size_t;
            }
            if (*__tekai_state_generated_backend_writet3).t3_line_limit as ::core::ffi::c_uint
                > INT_MAX as ::core::ffi::c_uint
            {
                crate::utils::pdftex_fail_args(
                    b"t3_line_array exceeds size limit\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                );
            }
            (*__tekai_state_generated_backend_writet3).t3_line_array = xrealloc(
                (*__tekai_state_generated_backend_writet3).t3_line_array as address,
                (*__tekai_state_generated_backend_writet3)
                    .t3_line_limit
                    .wrapping_mul(::core::mem::size_of::<t3_line_entry>() as size_t),
            )
                as *mut t3_line_entry;
            (*__tekai_state_generated_backend_writet3).t3_line_ptr =
                (*__tekai_state_generated_backend_writet3)
                    .t3_line_array
                    .offset((*__tekai_state_utils).last_ptr_index as isize);
        }
        if ((*__tekai_state_generated_backend_writet3)
            .t3_line_ptr
            .offset_from((*__tekai_state_generated_backend_writet3).t3_line_array)
            as ::core::ffi::c_long
            + 2 as ::core::ffi::c_long) as ::core::ffi::c_uint
            > 1024 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::utils::pdftex_fail_args(
                b"buffer overflow at file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(
                        b"tekai-engine/generated/backend/writet3.rs\0" as *const u8
                            as *const ::core::ffi::c_char,
                    ),
                    crate::utils::PrintfArg::from(66 as ::core::ffi::c_int),
                ],
            );
        }
        if (*__tekai_state_generated_backend_writet3)
            .t3_line_ptr
            .offset_from((*__tekai_state_generated_backend_writet3).t3_line_array)
            as ::core::ffi::c_long
            > 1 as ::core::ffi::c_long
            && *(*__tekai_state_generated_backend_writet3)
                .t3_line_ptr
                .offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                != 10 as ::core::ffi::c_int
        {
            let fresh1 = (*__tekai_state_generated_backend_writet3).t3_line_ptr;
            (*__tekai_state_generated_backend_writet3).t3_line_ptr =
                (*__tekai_state_generated_backend_writet3)
                    .t3_line_ptr
                    .offset(1);
            *fresh1 = 10 as t3_line_entry;
        }
        if (*__tekai_state_generated_backend_writet3)
            .t3_line_ptr
            .offset_from((*__tekai_state_generated_backend_writet3).t3_line_array)
            as ::core::ffi::c_long
            > 2 as ::core::ffi::c_long
            && *(*__tekai_state_generated_backend_writet3)
                .t3_line_ptr
                .offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == 32 as ::core::ffi::c_int
        {
            *(*__tekai_state_generated_backend_writet3)
                .t3_line_ptr
                .offset(-(2 as ::core::ffi::c_int) as isize) = 10 as t3_line_entry;
            (*__tekai_state_generated_backend_writet3).t3_line_ptr =
                (*__tekai_state_generated_backend_writet3)
                    .t3_line_ptr
                    .offset(-1);
        }
        *(*__tekai_state_generated_backend_writet3).t3_line_ptr = 0 as t3_line_entry;
        if !(((*__tekai_state_generated_backend_writet3)
            .t3_line_ptr
            .offset_from((*__tekai_state_generated_backend_writet3).t3_line_array)
            as ::core::ffi::c_long)
            < 2 as ::core::ffi::c_long
            || *(*__tekai_state_generated_backend_writet3).t3_line_array as ::core::ffi::c_int
                == '%' as i32)
        {
            break;
        }
        if !(feof((*__tekai_state_generated_backend_writet3).t3_file) == 0) {
            break;
        }
    }
}
unsafe fn t3_putline() {
    let __tekai_state_generated_backend_writet3: *mut crate::generated::backend::writet3::State =
        unsafe { crate::generated::backend::writet3::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut p: *mut ::core::ffi::c_char =
        (*__tekai_state_generated_backend_writet3).t3_line_array as *mut ::core::ffi::c_char;
    while p < (*__tekai_state_generated_backend_writet3).t3_line_ptr {
        if (1 as integer + (*__tekai_state_generated_pdftexextra).pdfptr) as ::core::ffi::c_uint
            > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
        {
            if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                zpdfosgetosbuf(1 as ::core::ffi::c_int);
            } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
            {
                crate::utils::pdftex_fail_args(
                    b"PDF output buffer overflowed\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            } else {
                pdfflush();
            }
        }
        let fresh2 = p;
        p = p.offset(1);
        let fresh3 = (*__tekai_state_generated_pdftexextra).pdfptr;
        (*__tekai_state_generated_pdftexextra).pdfptr =
            (*__tekai_state_generated_pdftexextra).pdfptr + 1;
        *(*__tekai_state_generated_pdftexextra)
            .pdfbuf
            .offset(fresh3 as isize) = *fresh2 as eightbits;
    }
}
unsafe fn update_bbox(
    mut llx: integer,
    mut lly: integer,
    mut urx: integer,
    mut ury: integer,
    mut is_first_glyph: boolean,
) {
    let __tekai_state_generated_backend_writet3: *mut crate::generated::backend::writet3::State =
        unsafe { crate::generated::backend::writet3::state() };

    if is_first_glyph != 0 {
        (*__tekai_state_generated_backend_writet3).t3_b0 = llx;
        (*__tekai_state_generated_backend_writet3).t3_b1 = lly;
        (*__tekai_state_generated_backend_writet3).t3_b2 = urx;
        (*__tekai_state_generated_backend_writet3).t3_b3 = ury;
    } else {
        if llx < (*__tekai_state_generated_backend_writet3).t3_b0 {
            (*__tekai_state_generated_backend_writet3).t3_b0 = llx;
        }
        if lly < (*__tekai_state_generated_backend_writet3).t3_b1 {
            (*__tekai_state_generated_backend_writet3).t3_b1 = lly;
        }
        if urx > (*__tekai_state_generated_backend_writet3).t3_b2 {
            (*__tekai_state_generated_backend_writet3).t3_b2 = urx;
        }
        if ury > (*__tekai_state_generated_backend_writet3).t3_b3 {
            (*__tekai_state_generated_backend_writet3).t3_b3 = ury;
        }
    };
}
unsafe fn t3_write_glyph(mut f: internalfontnumber) {
    let __tekai_state_generated_backend_writet3: *mut crate::generated::backend::writet3::State =
        unsafe { crate::generated::backend::writet3::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut glyph_index: ::core::ffi::c_int = 0;
    let mut width: ::core::ffi::c_int = 0;
    let mut height: ::core::ffi::c_int = 0;
    let mut depth: ::core::ffi::c_int = 0;
    let mut llx: ::core::ffi::c_int = 0;
    let mut lly: ::core::ffi::c_int = 0;
    let mut urx: ::core::ffi::c_int = 0;
    let mut ury: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    t3_getline();
    if strncmp(
        (*__tekai_state_generated_backend_writet3).t3_line_array,
        &raw mut (*__tekai_state_generated_backend_writet3)
            .local_t3_write_glyph_461_t3_begin_glyph_str as *mut ::core::ffi::c_char,
        strlen(
            &raw mut (*__tekai_state_generated_backend_writet3)
                .local_t3_write_glyph_461_t3_begin_glyph_str
                as *mut ::core::ffi::c_char,
        ),
    ) == 0 as ::core::ffi::c_int
    {
        let written = (*__tekai_state_generated_backend_writet3).t3_line_ptr
            .addr()
            .checked_sub((*__tekai_state_generated_backend_writet3).t3_line_array.addr())
            .filter(|&written| {
let __tekai_state_generated_backend_writet3: *mut crate::generated::backend::writet3::State = unsafe { crate::generated::backend::writet3::state() };
!(*__tekai_state_generated_backend_writet3).t3_line_array.is_null() && written < (*__tekai_state_generated_backend_writet3).t3_line_limit
})
            .unwrap_or_else(|| {
                crate::utils::pdftex_fail_args(c"invalid glyph initialized line span".as_ptr(), &[])
            });
        let line = std::slice::from_raw_parts(
            (*__tekai_state_generated_backend_writet3)
                .t3_line_array
                .cast::<u8>(),
            written,
        );
        let preamble = line
            .get(b"\\pdfglyph".len() + 1..)
            .and_then(|bytes| glyph_preamble(bytes).ok());
        let Some(preamble) = preamble else {
            p = strchr(
                (*__tekai_state_generated_backend_writet3).t3_line_array,
                0 as ::core::ffi::c_int,
            )
            .offset(-(1 as ::core::ffi::c_int as isize));
            if *p as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
                *p = 0 as ::core::ffi::c_char;
            }
            crate::utils::pdftex_fail_args(
                b"invalid glyph preamble: `%s'\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(
                    (*__tekai_state_generated_backend_writet3).t3_line_array,
                )],
            );
        };
        glyph_index = preamble.index;
        width = preamble.width;
        height = preamble.height;
        depth = preamble.depth;
        llx = preamble.llx;
        lly = preamble.lly;
        urx = preamble.urx;
        ury = preamble.ury;
        if glyph_index
            < *(*__tekai_state_generated_pdftexextra)
                .fontbc
                .offset(f as isize) as ::core::ffi::c_int
            || glyph_index
                > *(*__tekai_state_generated_pdftexextra)
                    .fontec
                    .offset(f as isize) as ::core::ffi::c_int
        {
            return;
        }
    } else {
        return;
    }
    if (*(*__tekai_state_generated_pdftexextra)
        .pdfcharused
        .offset(f as isize))[(glyph_index / 8 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << glyph_index % 8 as ::core::ffi::c_int
        == 0
    {
        while !(strncmp(
            (*__tekai_state_generated_backend_writet3).t3_line_array,
            &raw mut (*__tekai_state_generated_backend_writet3)
                .local_t3_write_glyph_463_t3_end_glyph_str as *mut ::core::ffi::c_char,
            strlen(
                &raw mut (*__tekai_state_generated_backend_writet3)
                    .local_t3_write_glyph_463_t3_end_glyph_str
                    as *mut ::core::ffi::c_char,
            ),
        ) == 0 as ::core::ffi::c_int)
        {
            if feof((*__tekai_state_generated_backend_writet3).t3_file) != 0 {
                crate::utils::pdftex_fail_args(
                    b"unexpected end of file\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            t3_getline();
        }
        return;
    }
    update_bbox(
        llx as integer,
        lly as integer,
        urx as integer,
        ury as integer,
        ((*__tekai_state_generated_backend_writet3).t3_glyph_num == 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int,
    );
    (*__tekai_state_generated_backend_writet3).t3_glyph_num += 1;
    zpdfnewdict(
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    (*__tekai_state_generated_backend_writet3).t3_char_procs[glyph_index as usize] =
        (*__tekai_state_generated_pdftexextra).objptr;
    if width == 0 as ::core::ffi::c_int {
        (*__tekai_state_generated_backend_writet3).t3_char_widths[glyph_index as usize] =
            zgetcharwidth(f, glyph_index as eightbits) as ::core::ffi::c_float
                / (*__tekai_state_generated_backend_writet3).t3_font_scale
                / *(*__tekai_state_generated_pdftexextra)
                    .pdffontsize
                    .offset(f as isize) as ::core::ffi::c_float;
    } else {
        (*__tekai_state_generated_backend_writet3).t3_char_widths[glyph_index as usize] =
            width as ::core::ffi::c_float;
    }
    pdfbeginstream();
    t3_getline();
    crate::utils::pdf_printf_args(
        b"%i 0 %i %i %i %i d1\nq\n\0" as *const u8 as *const ::core::ffi::c_char,
        &[
            crate::utils::PrintfArg::from(
                (*__tekai_state_generated_backend_writet3).t3_char_widths[glyph_index as usize]
                    as ::core::ffi::c_int,
            ),
            crate::utils::PrintfArg::from(llx),
            crate::utils::PrintfArg::from(lly),
            crate::utils::PrintfArg::from(urx),
            crate::utils::PrintfArg::from(ury),
        ],
    );
    while !(strncmp(
        (*__tekai_state_generated_backend_writet3).t3_line_array,
        &raw mut (*__tekai_state_generated_backend_writet3)
            .local_t3_write_glyph_463_t3_end_glyph_str as *mut ::core::ffi::c_char,
        strlen(
            &raw mut (*__tekai_state_generated_backend_writet3)
                .local_t3_write_glyph_463_t3_end_glyph_str as *mut ::core::ffi::c_char,
        ),
    ) == 0 as ::core::ffi::c_int)
    {
        if feof((*__tekai_state_generated_backend_writet3).t3_file) != 0 {
            crate::utils::pdftex_fail_args(
                b"unexpected end of file\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        if strncmp(
            (*__tekai_state_generated_backend_writet3).t3_line_array,
            b"BI\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"BI\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int
        {
            (*__tekai_state_generated_backend_writet3).t3_image_used = true_0 as boolean;
        }
        t3_putline();
        t3_getline();
    }
    pdf_puts(b"Q\n\0" as *const u8 as *const ::core::ffi::c_char);
    pdfendstream();
}
unsafe fn get_pk_font_scale(mut f: internalfontnumber) -> integer {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    return zdividescaled(
        (*__tekai_state_generated_pdftexextra).pkscalefactor,
        zdividescaled(
            *(*__tekai_state_generated_pdftexextra)
                .pdffontsize
                .offset(f as isize),
            (*__tekai_state_generated_pdftexextra).onehundredbp,
            (*__tekai_state_generated_pdftexextra).fixeddecimaldigits as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int,
        ),
        0 as ::core::ffi::c_int,
    ) as integer;
}
unsafe fn pk_char_width(mut f: internalfontnumber, mut w: scaled) -> integer {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    return zdividescaled(
        zdividescaled(
            w,
            *(*__tekai_state_generated_pdftexextra)
                .pdffontsize
                .offset(f as isize),
            7 as ::core::ffi::c_int,
        ),
        get_pk_font_scale(f),
        0 as ::core::ffi::c_int,
    ) as integer;
}
pub unsafe fn getpkcharwidth(mut f: internalfontnumber, mut w: scaled) -> scaled {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    return (get_pk_font_scale(f) as ::core::ffi::c_double / 100000.0f64
        * (pk_char_width(f, w) as ::core::ffi::c_double / 100.0f64)
        * *(*__tekai_state_generated_pdftexextra)
            .pdffontsize
            .offset(f as isize) as ::core::ffi::c_double) as scaled;
}
unsafe fn writepk(mut f: internalfontnumber) -> boolean {
    let __tekai_state_generated_backend_writet3: *mut crate::generated::backend::writet3::State =
        unsafe { crate::generated::backend::writet3::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    let mut font_ret: kpse_glyph_file_type = kpse_glyph_file_type {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        dpi: 0,
        format: kpse_gf_format,
        source: kpse_glyph_source_normal,
    };
    let mut llx: integer = 0;
    let mut lly: integer = 0;
    let mut urx: integer = 0;
    let mut ury: integer = 0;
    let mut cw: integer = 0;
    let mut rw: integer = 0;
    let mut i: integer = 0;
    let mut j: integer = 0;
    let mut row: *mut halfword = ::core::ptr::null_mut::<halfword>();
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cd: chardesc = chardesc {
        charcode: 0,
        cwidth: 0,
        cheight: 0,
        xoff: 0,
        yoff: 0,
        xescape: 0,
        rastersize: 0,
        raster: ::core::ptr::null_mut::<halfword>(),
    };
    let mut is_null_glyph: boolean = 0;
    let mut check_preamble: boolean = 0;
    let mut dpi: integer = 0;
    dpi = kpse_magstep_fix(
        zround(
            ((*__tekai_state_generated_pdftexextra).fixedpkresolution as ::core::ffi::c_float
                * (*(*__tekai_state_generated_pdftexextra)
                    .pdffontsize
                    .offset(f as isize) as ::core::ffi::c_float
                    / *(*__tekai_state_generated_pdftexextra)
                        .fontdsize
                        .offset(f as isize) as ::core::ffi::c_float))
                as ::core::ffi::c_double,
        ) as ::core::ffi::c_uint,
        (*__tekai_state_generated_pdftexextra).fixedpkresolution as ::core::ffi::c_uint,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    ) as integer;
    (*__tekai_state_utils).cur_file_name = makecstring(
        *(*__tekai_state_generated_pdftexextra)
            .fontname
            .offset(f as isize) as integer,
    );
    name = kpse_find_glyph(
        (*__tekai_state_utils).cur_file_name as const_string,
        dpi as ::core::ffi::c_uint,
        kpse_pk_format,
        &raw mut font_ret,
    ) as *mut ::core::ffi::c_char;
    if name.is_null()
        || !(!(*__tekai_state_utils).cur_file_name.is_null()
            && !font_ret.name.is_null()
            && strcmp(
                (*__tekai_state_utils).cur_file_name,
                font_ret.name as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int)
        || kpse_bitmap_tolerance(
            font_ret.dpi as ::core::ffi::c_float as ::core::ffi::c_double,
            dpi as ::core::ffi::c_float as ::core::ffi::c_double,
        ) == 0
    {
        crate::utils::pdftex_fail_args(
            b"Font %s at %i not found\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from((*__tekai_state_utils).cur_file_name),
                crate::utils::PrintfArg::from(dpi),
            ],
        );
    }
    (*__tekai_state_generated_backend_writet3).t3_file =
        xfopen(name as const_string, FOPEN_RBIN_MODE.as_ptr());
    recorder_record_input(name as const_string);
    (*__tekai_state_generated_backend_writet3).t3_image_used = true_0 as boolean;
    (*__tekai_state_generated_backend_writet3).is_pk_font = true_0 as boolean;
    crate::utils::tex_printf_args(
        b" <%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(name)],
    );
    cd.rastersize = 256 as ::core::ffi::c_int as integer;
    cd.raster = xmalloc(
        (cd.rastersize as size_t).wrapping_mul(::core::mem::size_of::<halfword>() as size_t),
    ) as *mut halfword;
    check_preamble = true_0 as boolean;
    while readchar(check_preamble, &raw mut cd) != 0 as ::core::ffi::c_int {
        check_preamble = false_0 as boolean;
        if (*(*__tekai_state_generated_pdftexextra)
            .pdfcharused
            .offset(f as isize))
            [(cd.charcode as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << cd.charcode as ::core::ffi::c_int % 8 as ::core::ffi::c_int
            == 0
        {
            continue;
        }
        (*__tekai_state_generated_backend_writet3).t3_char_widths[cd.charcode as usize] =
            pk_char_width(f, zgetcharwidth(f, cd.charcode as eightbits)) as ::core::ffi::c_float;
        if cd.cwidth < 1 as ::core::ffi::c_int || cd.cheight < 1 as ::core::ffi::c_int {
            cd.cwidth = zround(
                (*__tekai_state_generated_backend_writet3).t3_char_widths[cd.charcode as usize]
                    as ::core::ffi::c_double
                    / 100.0f64,
            );
            cd.xescape = cd.cwidth;
            cd.cheight = 1 as ::core::ffi::c_int as integer;
            cd.xoff = 0 as ::core::ffi::c_int as integer;
            cd.yoff = 0 as ::core::ffi::c_int as integer;
            is_null_glyph = true_0 as boolean;
        } else {
            is_null_glyph = false_0 as boolean;
        }
        llx = -cd.xoff;
        lly = (cd.yoff as ::core::ffi::c_int - cd.cheight as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as integer;
        urx = (cd.cwidth as ::core::ffi::c_int
            + llx as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as integer;
        ury = cd.cheight + lly;
        update_bbox(
            llx,
            lly,
            urx,
            ury,
            ((*__tekai_state_generated_backend_writet3).t3_glyph_num == 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int,
        );
        (*__tekai_state_generated_backend_writet3).t3_glyph_num += 1;
        zpdfnewdict(
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        (*__tekai_state_generated_backend_writet3).t3_char_procs[cd.charcode as usize] =
            (*__tekai_state_generated_pdftexextra).objptr;
        pdfbeginstream();
        zpdfprintreal(
            (*__tekai_state_generated_backend_writet3).t3_char_widths[cd.charcode as usize]
                as integer,
            2 as ::core::ffi::c_int,
        );
        crate::utils::pdf_printf_args(
            b" 0 %i %i %i %i d1\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(llx),
                crate::utils::PrintfArg::from(lly),
                crate::utils::PrintfArg::from(urx),
                crate::utils::PrintfArg::from(ury),
            ],
        );
        if !(is_null_glyph != 0) {
            crate::utils::pdf_printf_args(
                b"q\n%i 0 0 %i %i %i cm\nBI\n\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(cd.cwidth),
                    crate::utils::PrintfArg::from(cd.cheight),
                    crate::utils::PrintfArg::from(llx),
                    crate::utils::PrintfArg::from(lly),
                ],
            );
            crate::utils::pdf_printf_args(
                b"/W %i\n/H %i\n\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(cd.cwidth),
                    crate::utils::PrintfArg::from(cd.cheight),
                ],
            );
            pdf_puts(
                b"/IM true\n/BPC 1\n/D [1 0]\nID \0" as *const u8 as *const ::core::ffi::c_char,
            );
            cw = ((cd.cwidth as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
                / 8 as ::core::ffi::c_int) as integer;
            rw = ((cd.cwidth as ::core::ffi::c_int + 15 as ::core::ffi::c_int)
                / 16 as ::core::ffi::c_int) as integer;
            row = cd.raster;
            i = 0 as ::core::ffi::c_int as integer;
            while i < cd.cheight {
                j = 0 as ::core::ffi::c_int as integer;
                while j < rw as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
                    if (1 as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                        as ::core::ffi::c_uint
                        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                    {
                        if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                            zpdfosgetosbuf(1 as ::core::ffi::c_int);
                        } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                            > (*__tekai_state_generated_pdftexextra).pdfbufsize
                                as ::core::ffi::c_uint
                        {
                            crate::utils::pdftex_fail_args(
                                b"PDF output buffer overflowed\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[],
                            );
                        } else {
                            pdfflush();
                        }
                    }
                    let fresh4 = (*__tekai_state_generated_pdftexextra).pdfptr;
                    (*__tekai_state_generated_pdftexextra).pdfptr =
                        (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                    *(*__tekai_state_generated_pdftexextra)
                        .pdfbuf
                        .offset(fresh4 as isize) = (*row / 256 as ::core::ffi::c_int) as eightbits;
                    if (1 as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                        as ::core::ffi::c_uint
                        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                    {
                        if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                            zpdfosgetosbuf(1 as ::core::ffi::c_int);
                        } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                            > (*__tekai_state_generated_pdftexextra).pdfbufsize
                                as ::core::ffi::c_uint
                        {
                            crate::utils::pdftex_fail_args(
                                b"PDF output buffer overflowed\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[],
                            );
                        } else {
                            pdfflush();
                        }
                    }
                    let fresh5 = (*__tekai_state_generated_pdftexextra).pdfptr;
                    (*__tekai_state_generated_pdftexextra).pdfptr =
                        (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                    *(*__tekai_state_generated_pdftexextra)
                        .pdfbuf
                        .offset(fresh5 as isize) = (*row % 256 as ::core::ffi::c_int) as eightbits;
                    row = row.offset(1);
                    j += 1;
                }
                if (1 as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                    as ::core::ffi::c_uint
                    > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                {
                    if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                        zpdfosgetosbuf(1 as ::core::ffi::c_int);
                    } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                    {
                        crate::utils::pdftex_fail_args(
                            b"PDF output buffer overflowed\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    } else {
                        pdfflush();
                    }
                }
                let fresh6 = (*__tekai_state_generated_pdftexextra).pdfptr;
                (*__tekai_state_generated_pdftexextra).pdfptr =
                    (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                *(*__tekai_state_generated_pdftexextra)
                    .pdfbuf
                    .offset(fresh6 as isize) = (*row / 256 as ::core::ffi::c_int) as eightbits;
                if 2 as integer * rw == cw {
                    if (1 as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                        as ::core::ffi::c_uint
                        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                    {
                        if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                            zpdfosgetosbuf(1 as ::core::ffi::c_int);
                        } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                            > (*__tekai_state_generated_pdftexextra).pdfbufsize
                                as ::core::ffi::c_uint
                        {
                            crate::utils::pdftex_fail_args(
                                b"PDF output buffer overflowed\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[],
                            );
                        } else {
                            pdfflush();
                        }
                    }
                    let fresh7 = (*__tekai_state_generated_pdftexextra).pdfptr;
                    (*__tekai_state_generated_pdftexextra).pdfptr =
                        (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                    *(*__tekai_state_generated_pdftexextra)
                        .pdfbuf
                        .offset(fresh7 as isize) = (*row % 256 as ::core::ffi::c_int) as eightbits;
                }
                row = row.offset(1);
                i += 1;
            }
            pdf_puts(b"\nEI\nQ\n\0" as *const u8 as *const ::core::ffi::c_char);
        }
        pdfendstream();
    }
    if !cd.raster.is_null() {
        free(cd.raster as *mut ::core::ffi::c_void);
    }
    cd.raster = ::core::ptr::null_mut::<halfword>();
    (*__tekai_state_utils).cur_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return true_0;
}
unsafe fn remove_duplicate_glyph_names(
    mut g: *mut *mut ::core::ffi::c_char,
    mut encname: *const ::core::ffi::c_char,
) {
    let __tekai_state_generated_backend_avlstuff: *mut crate::generated::backend::avlstuff::State =
        unsafe { crate::generated::backend::avlstuff::state() };
    let __tekai_state_generated_backend_writettf: *mut crate::generated::backend::writettf::State =
        unsafe { crate::generated::backend::writettf::state() };

    let mut gl_tree: *mut avl_table = ::core::ptr::null_mut::<avl_table>();
    let mut aa: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    gl_tree = avl_create(
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
    if gl_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"remove_duplicate_glyph_names\0" as *const u8 as *const ::core::ffi::c_char,
            b"writet3.c\0" as *const u8 as *const ::core::ffi::c_char,
            266 as ::core::ffi::c_int,
            b"gl_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        if !(*g.offset(i as isize)
            == &raw mut (*__tekai_state_generated_backend_writettf).notdef
                as *mut ::core::ffi::c_char)
        {
            aa = avl_find(gl_tree, *g.offset(i as isize) as *const ::core::ffi::c_void)
                as *mut ::core::ffi::c_char;
            if aa.is_null() {
                aa = avl_probe(gl_tree, *g.offset(i as isize) as *mut ::core::ffi::c_void)
                    as *mut ::core::ffi::c_char;
                if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                    __assert_rtn(
                        b"remove_duplicate_glyph_names\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"writet3.c\0" as *const u8 as *const ::core::ffi::c_char,
                        273 as ::core::ffi::c_int,
                        b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                };
            } else {
                crate::utils::pdftex_warn_args(
                    b"%s: duplicate glyph name at position %d: %s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(encname),
                        crate::utils::PrintfArg::from(i),
                        crate::utils::PrintfArg::from(*g.offset(i as isize)),
                    ],
                );
                if !(*g.offset(i as isize)).is_null() {
                    free(*g.offset(i as isize) as *mut ::core::ffi::c_void);
                }
                let ref mut fresh8 = *g.offset(i as isize);
                *fresh8 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                let ref mut fresh9 = *g.offset(i as isize);
                *fresh9 = &raw mut (*__tekai_state_generated_backend_writettf).notdef
                    as *mut ::core::ffi::c_char;
            }
        }
        i += 1;
    }
    avl_destroy(gl_tree, None);
}
pub unsafe fn writet3(
    mut fm: *mut fm_entry,
    mut objnum: ::core::ffi::c_int,
    mut f: internalfontnumber,
) {
    let __tekai_state_generated_backend_writet3: *mut crate::generated::backend::writet3::State =
        unsafe { crate::generated::backend::writet3::state() };
    let __tekai_state_generated_backend_writettf: *mut crate::generated::backend::writettf::State =
        unsafe { crate::generated::backend::writettf::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    let mut i: ::core::ffi::c_int = 0;
    let mut wptr: integer = 0;
    let mut eptr: integer = 0;
    let mut cptr: integer = 0;
    let mut first_char: ::core::ffi::c_int = 0;
    let mut last_char: ::core::ffi::c_int = 0;
    let mut pk_font_scale: integer = 0;
    let mut is_notdef: boolean = 0;
    let mut fe: *mut fe_entry = ::core::ptr::null_mut::<fe_entry>();
    let mut glyph_names: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut tounicode_objnum: integer = 0;
    (*__tekai_state_generated_backend_writet3).t3_glyph_num = 0 as ::core::ffi::c_int;
    (*__tekai_state_generated_backend_writet3).t3_image_used = false_0 as boolean;
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        (*__tekai_state_generated_backend_writet3).t3_char_procs[i as usize] =
            0 as ::core::ffi::c_int as integer;
        (*__tekai_state_generated_backend_writet3).t3_char_widths[i as usize] =
            0 as ::core::ffi::c_int as ::core::ffi::c_float;
        i += 1;
    }
    fe = if !fm.is_null() && !(*fm).encname.is_null() {
        get_fe_entry((*fm).encname)
    } else {
        ::core::ptr::null_mut::<fe_entry>()
    };
    glyph_names = if !fe.is_null() {
        (*fe).glyph_names
    } else {
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>()
    };
    if !glyph_names.is_null() {
        remove_duplicate_glyph_names(glyph_names, (*fm).encname);
    }
    zpackfilename(
        *(*__tekai_state_generated_pdftexextra)
            .fontname
            .offset(f as isize),
        getnullstr(),
        maketexstring(b".pgc\0" as *const u8 as *const ::core::ffi::c_char),
    );
    (*__tekai_state_utils).cur_file_name = makecstring(makenamestring() as integer);
    (*__tekai_state_generated_backend_writet3).is_pk_font = false_0 as boolean;
    if open_input(
        &raw mut (*__tekai_state_generated_backend_writet3).t3_file,
        kpse_miscfonts_format as ::core::ffi::c_int,
        FOPEN_RBIN_MODE.as_ptr(),
    ) == 0
    {
        if !(writepk(f) != 0) {
            (*__tekai_state_utils).cur_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return;
        }
    } else {
        crate::utils::tex_printf_args(
            b"<%s\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                (*__tekai_state_generated_pdftexextra)
                    .nameoffile
                    .offset(1 as ::core::ffi::c_int as isize),
            )],
        );
        t3_getline();
        let written = (*__tekai_state_generated_backend_writet3).t3_line_ptr
            .addr()
            .checked_sub((*__tekai_state_generated_backend_writet3).t3_line_array.addr())
            .filter(|&written| {
let __tekai_state_generated_backend_writet3: *mut crate::generated::backend::writet3::State = unsafe { crate::generated::backend::writet3::state() };
!(*__tekai_state_generated_backend_writet3).t3_line_array.is_null() && written < (*__tekai_state_generated_backend_writet3).t3_line_limit
})
            .unwrap_or_else(|| {
                crate::utils::pdftex_fail_args(
                    c"invalid font scale initialized line span".as_ptr(),
                    &[],
                )
            });
        let line = std::slice::from_raw_parts(
            (*__tekai_state_generated_backend_writet3)
                .t3_line_array
                .cast::<u8>(),
            written,
        );
        let Ok(scale) = font_scale_line(line) else {
            crate::utils::pdftex_fail_args(
                b"missing or invalid font scale\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        };
        (*__tekai_state_generated_backend_writet3).t3_font_scale = scale;
        while feof((*__tekai_state_generated_backend_writet3).t3_file) == 0 {
            t3_write_glyph(f);
        }
    }
    i = *(*__tekai_state_generated_pdftexextra)
        .fontbc
        .offset(f as isize) as ::core::ffi::c_int;
    while i
        <= *(*__tekai_state_generated_pdftexextra)
            .fontec
            .offset(f as isize) as ::core::ffi::c_int
    {
        if (*(*__tekai_state_generated_pdftexextra)
            .pdfcharused
            .offset(f as isize))[(i / 8 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << i % 8 as ::core::ffi::c_int
            != 0
        {
            break;
        }
        i += 1;
    }
    first_char = i;
    i = *(*__tekai_state_generated_pdftexextra)
        .fontec
        .offset(f as isize) as ::core::ffi::c_int;
    while i > first_char {
        if (*(*__tekai_state_generated_pdftexextra)
            .pdfcharused
            .offset(f as isize))[(i / 8 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << i % 8 as ::core::ffi::c_int
            != 0
        {
            break;
        }
        i -= 1;
    }
    last_char = i;
    if (*__tekai_state_generated_pdftexextra).fixedgentounicode > 0 as ::core::ffi::c_int
        && *(*__tekai_state_generated_pdftexextra)
            .pdffontnobuiltintounicode
            .offset(f as isize)
            == 0
        && !fe.is_null()
    {
        tounicode_objnum = write_tounicode(glyph_names, (*fm).tfm_name, (*fe).name);
    } else {
        tounicode_objnum = 0 as ::core::ffi::c_int as integer;
    }
    zpdfbegindict(objnum, 1 as ::core::ffi::c_int);
    pdf_puts(b"/Type /Font\n/Subtype /Type3\n\0" as *const u8 as *const ::core::ffi::c_char);
    crate::utils::pdf_printf_args(
        b"/Name /F%i\n\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(f)],
    );
    if *(*__tekai_state_generated_pdftexextra)
        .pdffontattr
        .offset(f as isize)
        != getnullstr()
    {
        zpdfprint(
            *(*__tekai_state_generated_pdftexextra)
                .pdffontattr
                .offset(f as isize),
        );
        pdf_puts(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if (*__tekai_state_generated_backend_writet3).is_pk_font != 0 {
        pk_font_scale = get_pk_font_scale(f);
        pdf_puts(b"/FontMatrix [\0" as *const u8 as *const ::core::ffi::c_char);
        zpdfprintreal(pk_font_scale, 5 as ::core::ffi::c_int);
        pdf_puts(b" 0 0 \0" as *const u8 as *const ::core::ffi::c_char);
        zpdfprintreal(pk_font_scale, 5 as ::core::ffi::c_int);
        pdf_puts(b" 0 0]\n\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        crate::utils::pdf_printf_args(
            b"/FontMatrix [%g 0 0 %g 0 0]\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(
                    (*__tekai_state_generated_backend_writet3).t3_font_scale
                        as ::core::ffi::c_double,
                ),
                crate::utils::PrintfArg::from(
                    (*__tekai_state_generated_backend_writet3).t3_font_scale
                        as ::core::ffi::c_double,
                ),
            ],
        );
    }
    crate::utils::pdf_printf_args(
        b"/%s [ %i %i %i %i ]\n\0" as *const u8 as *const ::core::ffi::c_char,
        &[
            crate::utils::PrintfArg::from(
                (*__tekai_state_generated_backend_writet3).font_key[FONTBBOX1_CODE as usize]
                    .pdfname,
            ),
            crate::utils::PrintfArg::from((*__tekai_state_generated_backend_writet3).t3_b0),
            crate::utils::PrintfArg::from((*__tekai_state_generated_backend_writet3).t3_b1),
            crate::utils::PrintfArg::from((*__tekai_state_generated_backend_writet3).t3_b2),
            crate::utils::PrintfArg::from((*__tekai_state_generated_backend_writet3).t3_b3),
        ],
    );
    crate::utils::pdf_printf_args(
        b"/Resources << /ProcSet [ /PDF %s] >>\n\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(
            if (*__tekai_state_generated_backend_writet3).t3_image_used != 0 {
                b"/ImageB \0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
        )],
    );
    crate::utils::pdf_printf_args(
        b"/FirstChar %i\n/LastChar %i\n\0" as *const u8 as *const ::core::ffi::c_char,
        &[
            crate::utils::PrintfArg::from(first_char),
            crate::utils::PrintfArg::from(last_char),
        ],
    );
    wptr = pdfnewobjnum();
    eptr = pdfnewobjnum();
    cptr = pdfnewobjnum();
    crate::utils::pdf_printf_args(
        b"/Widths %i 0 R\n/Encoding %i 0 R\n/CharProcs %i 0 R\n\0" as *const u8
            as *const ::core::ffi::c_char,
        &[
            crate::utils::PrintfArg::from(wptr),
            crate::utils::PrintfArg::from(eptr),
            crate::utils::PrintfArg::from(cptr),
        ],
    );
    if tounicode_objnum != 0 as ::core::ffi::c_int {
        crate::utils::pdf_printf_args(
            b"/ToUnicode %i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(tounicode_objnum)],
        );
    }
    pdfenddict();
    zpdfbeginobj(wptr, 1 as ::core::ffi::c_int);
    pdf_puts(b"[\0" as *const u8 as *const ::core::ffi::c_char);
    if (*__tekai_state_generated_backend_writet3).is_pk_font != 0 {
        i = first_char;
        while i <= last_char {
            zpdfprintreal(
                (*__tekai_state_generated_backend_writet3).t3_char_widths[i as usize] as integer,
                2 as ::core::ffi::c_int,
            );
            pdf_puts(b" \0" as *const u8 as *const ::core::ffi::c_char);
            i += 1;
        }
    } else {
        i = first_char;
        while i <= last_char {
            crate::utils::pdf_printf_args(
                b"%i \0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(
                    (*__tekai_state_generated_backend_writet3).t3_char_widths[i as usize]
                        as ::core::ffi::c_int,
                )],
            );
            i += 1;
        }
    }
    pdf_puts(b"]\n\0" as *const u8 as *const ::core::ffi::c_char);
    pdfendobj();
    zpdfbegindict(eptr, 1 as ::core::ffi::c_int);
    crate::utils::pdf_printf_args(
        b"/Type /Encoding\n/Differences [%i\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(first_char)],
    );
    if (*__tekai_state_generated_backend_writet3).t3_char_procs[first_char as usize]
        == 0 as ::core::ffi::c_int
    {
        crate::utils::pdf_printf_args(
            b"/%s\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                &raw mut (*__tekai_state_generated_backend_writettf).notdef
                    as *mut ::core::ffi::c_char,
            )],
        );
        is_notdef = true_0 as boolean;
    } else {
        if !glyph_names.is_null()
            && !(*glyph_names.offset(first_char as isize)).is_null()
            && *glyph_names.offset(first_char as isize)
                != &raw mut (*__tekai_state_generated_backend_writettf).notdef
                    as *mut ::core::ffi::c_char
        {
            crate::utils::pdf_printf_args(
                b"/%s\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(
                    *glyph_names.offset(first_char as isize),
                )],
            );
        } else {
            crate::utils::pdf_printf_args(
                b"/a%i\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(first_char)],
            );
        }
        is_notdef = false_0 as boolean;
    }
    i = first_char + 1 as ::core::ffi::c_int;
    while i <= last_char {
        if (*__tekai_state_generated_backend_writet3).t3_char_procs[i as usize]
            == 0 as ::core::ffi::c_int
        {
            if is_notdef == 0 {
                crate::utils::pdf_printf_args(
                    b" %i/%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(i),
                        crate::utils::PrintfArg::from(
                            &raw mut (*__tekai_state_generated_backend_writettf).notdef
                                as *mut ::core::ffi::c_char,
                        ),
                    ],
                );
                is_notdef = true_0 as boolean;
            }
        } else {
            if is_notdef != 0 {
                crate::utils::pdf_printf_args(
                    b" %i\0" as *const u8 as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(i)],
                );
                is_notdef = false_0 as boolean;
            }
            if !glyph_names.is_null()
                && !(*glyph_names.offset(i as isize)).is_null()
                && *glyph_names.offset(i as isize)
                    != &raw mut (*__tekai_state_generated_backend_writettf).notdef
                        as *mut ::core::ffi::c_char
            {
                crate::utils::pdf_printf_args(
                    b"/%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(
                        *glyph_names.offset(i as isize),
                    )],
                );
            } else {
                crate::utils::pdf_printf_args(
                    b"/a%i\0" as *const u8 as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(i)],
                );
            }
        }
        i += 1;
    }
    pdf_puts(b"]\n\0" as *const u8 as *const ::core::ffi::c_char);
    pdfenddict();
    zpdfbegindict(cptr, 1 as ::core::ffi::c_int);
    i = first_char;
    while i <= last_char {
        if (*__tekai_state_generated_backend_writet3).t3_char_procs[i as usize]
            != 0 as ::core::ffi::c_int
        {
            if !glyph_names.is_null()
                && !(*glyph_names.offset(i as isize)).is_null()
                && *glyph_names.offset(i as isize)
                    != &raw mut (*__tekai_state_generated_backend_writettf).notdef
                        as *mut ::core::ffi::c_char
            {
                crate::utils::pdf_printf_args(
                    b"/%s %i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(*glyph_names.offset(i as isize)),
                        crate::utils::PrintfArg::from(
                            (*__tekai_state_generated_backend_writet3).t3_char_procs[i as usize],
                        ),
                    ],
                );
            } else {
                crate::utils::pdf_printf_args(
                    b"/a%i %i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(i),
                        crate::utils::PrintfArg::from(
                            (*__tekai_state_generated_backend_writet3).t3_char_procs[i as usize],
                        ),
                    ],
                );
            }
        }
        i += 1;
    }
    pdfenddict();
    xfclose(
        (*__tekai_state_generated_backend_writet3).t3_file,
        (*__tekai_state_utils).cur_file_name as const_string,
    );
    crate::utils::tex_printf_args(b">\0" as *const u8 as *const ::core::ffi::c_char, &[]);
    (*__tekai_state_utils).cur_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;

pub(crate) struct State {
    font_key: [key_entry; 11],
    t3_line_ptr: *mut t3_line_entry,
    t3_line_array: *mut t3_line_entry,
    t3_line_limit: size_t,
    pub(crate) t3_file: *mut FILE,
    t3_image_used: boolean,
    t3_char_procs: [integer; 256],
    t3_char_widths: [::core::ffi::c_float; 256],
    t3_glyph_num: ::core::ffi::c_int,
    t3_font_scale: ::core::ffi::c_float,
    t3_b3: integer,
    t3_b2: integer,
    t3_b0: integer,
    t3_b1: integer,
    is_pk_font: boolean,
    local_t3_write_glyph_461_t3_begin_glyph_str: [::core::ffi::c_char; 10],
    local_t3_write_glyph_463_t3_end_glyph_str: [::core::ffi::c_char; 10],
}

#[inline(always)]
fn __tekai_require_copy<T: Copy>() {}

pub(crate) struct StateInitialization {
    destination: *mut State,
    initialized: [bool; 17],
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
            if self.initialized[16] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).local_t3_write_glyph_463_t3_end_glyph_str
                ));
            }
            if self.initialized[15] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).local_t3_write_glyph_461_t3_begin_glyph_str
                ));
            }
            if self.initialized[14] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).is_pk_font));
            }
            if self.initialized[13] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t3_b1));
            }
            if self.initialized[12] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t3_b0));
            }
            if self.initialized[11] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t3_b2));
            }
            if self.initialized[10] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t3_b3));
            }
            if self.initialized[9] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t3_font_scale));
            }
            if self.initialized[8] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t3_glyph_num));
            }
            if self.initialized[7] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t3_char_widths));
            }
            if self.initialized[6] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t3_char_procs));
            }
            if self.initialized[5] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t3_image_used));
            }
            if self.initialized[4] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t3_file));
            }
            if self.initialized[3] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t3_line_limit));
            }
            if self.initialized[2] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t3_line_array));
            }
            if self.initialized[1] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).t3_line_ptr));
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
            initialized: [false; 17],
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
            assert!(core::mem::size_of::<*mut t3_line_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t3_line_ptr)
            .write(::core::ptr::null::<t3_line_entry>() as *mut t3_line_entry);
        self.initialized[1] = true;
        const {
            assert!(core::mem::size_of::<*mut t3_line_entry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t3_line_array)
            .write(::core::ptr::null::<t3_line_entry>() as *mut t3_line_entry);
        self.initialized[2] = true;
        const {
            assert!(core::mem::size_of::<size_t>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t3_line_limit).write(0);
        self.initialized[3] = true;
        const {
            assert!(core::mem::size_of::<*mut FILE>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t3_file)
            .write(::core::ptr::null::<FILE>() as *mut FILE);
        self.initialized[4] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t3_image_used).write(0);
        self.initialized[5] = true;
        const {
            assert!((256) == (256));
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        __tekai_require_copy::<integer>();
        {
            let __tekai_repeat_0: integer = 0;
            for __tekai_index_0 in 0..(256) {
                core::ptr::addr_of_mut!((*destination).t3_char_procs[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[6] = true;
        const {
            assert!((256) == (256));
            assert!(core::mem::size_of::<::core::ffi::c_float>() <= 16 * 1024);
        }
        __tekai_require_copy::<::core::ffi::c_float>();
        {
            let __tekai_repeat_0: ::core::ffi::c_float = 0.;
            for __tekai_index_0 in 0..(256) {
                core::ptr::addr_of_mut!((*destination).t3_char_widths[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[7] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t3_glyph_num).write(0);
        self.initialized[8] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_float>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t3_font_scale).write(0.);
        self.initialized[9] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t3_b3).write(0);
        self.initialized[10] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t3_b2).write(0);
        self.initialized[11] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t3_b0).write(0);
        self.initialized[12] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).t3_b1).write(0);
        self.initialized[13] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).is_pk_font).write(0);
        self.initialized[14] = true;
        const {
            assert!(core::mem::size_of::<[::core::ffi::c_char; 10]>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_t3_write_glyph_461_t3_begin_glyph_str).write(
            unsafe {
                ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"\\pdfglyph\0")
            },
        );
        self.initialized[15] = true;
        const {
            assert!(core::mem::size_of::<[::core::ffi::c_char; 10]>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_t3_write_glyph_463_t3_end_glyph_str).write(
            unsafe {
                ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"\\endglyph\0")
            },
        );
        self.initialized[16] = true;
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
        (*crate::invocation_state::current()).generated_backend_writet3
    ))
}
#[cfg(test)]
pub(crate) unsafe fn check_state_construction(phase: u8) {
    let invocation = crate::invocation_state::current();
    let destination = state();
    assert!(!destination.is_null());
    let _ = (invocation, phase);
}
