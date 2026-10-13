// Rust translation and subsequent modifications for Tekai.
// Upstream TeX Live source revision 52dac44dde13a8310bbb8231e81c58be28cebe20.
// Origin texk/web2c/pdftexdir/subfont.c.
// GPL-2.0-or-later.
// Modified for Tekai. Licensing and provenance notice restored 2026-10-13.
// Original copyrights and license terms are retained in the crate's
// NOTICE.md and licenses/upstream-notices.txt.

pub use crate::generated::backend::mapfile::{avl_do_entry, delete_fm_entry, new_fm_entry};
pub use crate::generated::pdftex0::{getnullstr, zpackfilename};
pub use crate::utils::maketexstring;

pub use crate::generated::backend::avl::{avl_create, avl_destroy, avl_find, avl_probe, avl_table};

use crate::byte_string::{strchr, strcmp};

use crate::backend_parse::{checked_inclusive_span, checked_span, Cursor};
use crate::integer_parse::IntegerBase;
use crate::openclose::open_input;
use crate::stdio::feof;
use crate::support::xfclose;
use std::ffi::CStr;

use crate::allocation::free;
use crate::openclose::{xmalloc, xstrdup};

use crate::termination::__assert_rtn;

#[inline(always)]
unsafe fn fast_fgetc(stream: *mut FILE) -> ::core::ffi::c_int {
    unsafe { crate::stdio::fgetc(stream) }
}
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
pub type strnumber = integer;
pub use crate::backend_types::_subfont_entry;
pub type subfont_entry = _subfont_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfd_entry {
    pub name: *mut ::core::ffi::c_char,
    pub subfont: *mut subfont_entry,
}
pub use crate::backend_types::fm_entry;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const FOPEN_RBIN_MODE: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"rb\0") };
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const F_SUBFONT: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

unsafe fn new_subfont_entry() -> *mut subfont_entry {
    let mut i: ::core::ffi::c_int = 0;
    let mut subfont: *mut subfont_entry = ::core::ptr::null_mut::<subfont_entry>();
    subfont = xmalloc((1 as size_t).wrapping_mul(::core::mem::size_of::<subfont_entry>() as size_t))
        as *mut subfont_entry;
    (*subfont).infix = ::core::ptr::null_mut::<::core::ffi::c_char>();
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        (*subfont).charcodes[i as usize] = -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
        i += 1;
    }
    (*subfont).next = ::core::ptr::null_mut::<subfont_entry>();
    return subfont;
}
unsafe fn new_sfd_entry() -> *mut sfd_entry {
    let mut sfd: *mut sfd_entry = ::core::ptr::null_mut::<sfd_entry>();
    sfd = xmalloc((1 as size_t).wrapping_mul(::core::mem::size_of::<sfd_entry>() as size_t))
        as *mut sfd_entry;
    (*sfd).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*sfd).subfont = ::core::ptr::null_mut::<subfont_entry>();
    return sfd;
}
unsafe fn destroy_sfd_entry(mut pa: *mut ::core::ffi::c_void, mut pb: *mut ::core::ffi::c_void) {
    let mut p: *mut subfont_entry = ::core::ptr::null_mut::<subfont_entry>();
    let mut q: *mut subfont_entry = ::core::ptr::null_mut::<subfont_entry>();
    let mut sfd: *mut sfd_entry = ::core::ptr::null_mut::<sfd_entry>();
    sfd = pa as *mut sfd_entry;
    p = (*sfd).subfont;
    while !p.is_null() {
        q = (*p).next;
        if !(*p).infix.is_null() {
            free((*p).infix as *mut ::core::ffi::c_void);
        }
        (*p).infix = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if !p.is_null() {
            free(p as *mut ::core::ffi::c_void);
        }
        p = ::core::ptr::null_mut::<subfont_entry>();
        p = q;
    }
    if !(*sfd).name.is_null() {
        free((*sfd).name as *mut ::core::ffi::c_void);
    }
    (*sfd).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe fn comp_sfd_entry(
    mut pa: *const ::core::ffi::c_void,
    mut pb: *const ::core::ffi::c_void,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        (*(pa as *const sfd_entry)).name,
        (*(pb as *const sfd_entry)).name,
    );
}
pub unsafe fn sfd_free() {
    let __tekai_state_generated_backend_subfont: *mut crate::generated::backend::subfont::State =
        unsafe { crate::generated::backend::subfont::state() };

    if !(*__tekai_state_generated_backend_subfont)
        .sfd_tree
        .is_null()
    {
        avl_destroy(
            (*__tekai_state_generated_backend_subfont).sfd_tree,
            Some(
                destroy_sfd_entry
                    as unsafe fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
            ),
        );
    }
}
unsafe fn sfd_getline(mut expect_eof: boolean) {
    let __tekai_state_generated_backend_subfont: *mut crate::generated::backend::subfont::State =
        unsafe { crate::generated::backend::subfont::state() };

    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut c: ::core::ffi::c_int = 0;
    loop {
        if feof((*__tekai_state_generated_backend_subfont).sfd_file) != 0 {
            if expect_eof != 0 {
                return;
            } else {
                crate::utils::pdftex_fail_args(
                    b"unexpected end of file\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
        }
        p = &raw mut (*__tekai_state_generated_backend_subfont).sfd_line
            as *mut ::core::ffi::c_char;
        loop {
            c = fast_fgetc((*__tekai_state_generated_backend_subfont).sfd_file);
            if c == 9 as ::core::ffi::c_int {
                c = 32 as ::core::ffi::c_int;
            }
            if c == 13 as ::core::ffi::c_int || c == EOF {
                c = 10 as ::core::ffi::c_int;
            }
            if c != ' ' as i32
                || p > &raw mut (*__tekai_state_generated_backend_subfont).sfd_line
                    as *mut ::core::ffi::c_char
                    && *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                        != 32 as ::core::ffi::c_int
            {
                if (p.offset_from(
                    &raw mut (*__tekai_state_generated_backend_subfont).sfd_line
                        as *mut ::core::ffi::c_char,
                ) as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                    > 256 as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    crate::utils::pdftex_fail_args(
                        b"buffer overflow at file %s, line %d\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[
                            crate::utils::PrintfArg::from(
                                b"tekai-engine/generated/backend/subfont.rs\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            ),
                            crate::utils::PrintfArg::from(95 as ::core::ffi::c_int),
                        ],
                    );
                }
                let fresh3 = p;
                p = p.offset(1);
                *fresh3 = c as ::core::ffi::c_char;
            }
            if !(c != 10 as ::core::ffi::c_int) {
                break;
            }
        }
        if (p.offset_from(
            &raw mut (*__tekai_state_generated_backend_subfont).sfd_line
                as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_long
            + 2 as ::core::ffi::c_long) as ::core::ffi::c_uint
            > 256 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::utils::pdftex_fail_args(
                b"buffer overflow at file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(
                        b"tekai-engine/generated/backend/subfont.rs\0" as *const u8
                            as *const ::core::ffi::c_char,
                    ),
                    crate::utils::PrintfArg::from(97 as ::core::ffi::c_int),
                ],
            );
        }
        if p.offset_from(
            &raw mut (*__tekai_state_generated_backend_subfont).sfd_line
                as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_long
            > 1 as ::core::ffi::c_long
            && *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                != 10 as ::core::ffi::c_int
        {
            let fresh4 = p;
            p = p.offset(1);
            *fresh4 = 10 as ::core::ffi::c_char;
        }
        if p.offset_from(
            &raw mut (*__tekai_state_generated_backend_subfont).sfd_line
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
            &raw mut (*__tekai_state_generated_backend_subfont).sfd_line
                as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_long)
            < 2 as ::core::ffi::c_long
            || *(&raw mut (*__tekai_state_generated_backend_subfont).sfd_line
                as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                == '#' as i32)
        {
            break;
        }
    }
}
unsafe fn read_sfd(mut sfd_name: *mut ::core::ffi::c_char) -> *mut sfd_entry {
    let __tekai_state_generated_backend_avlstuff: *mut crate::generated::backend::avlstuff::State =
        unsafe { crate::generated::backend::avlstuff::state() };
    let __tekai_state_generated_backend_subfont: *mut crate::generated::backend::subfont::State =
        unsafe { crate::generated::backend::subfont::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut sfd: *mut sfd_entry = ::core::ptr::null_mut::<sfd_entry>();
    let mut tmp_sfd: sfd_entry = sfd_entry {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        subfont: ::core::ptr::null_mut::<subfont_entry>(),
    };
    let mut sf: *mut subfont_entry = ::core::ptr::null_mut::<subfont_entry>();
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_long = 0;
    let mut j: ::core::ffi::c_long = 0;
    let mut k: ::core::ffi::c_long = 0;
    tmp_sfd.name = sfd_name;
    if (*__tekai_state_generated_backend_subfont)
        .sfd_tree
        .is_null()
    {
        (*__tekai_state_generated_backend_subfont).sfd_tree = avl_create(
            Some(
                comp_sfd_entry
                    as unsafe fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
            &raw mut (*__tekai_state_generated_backend_avlstuff).avl_xallocator,
        );
        if (*__tekai_state_generated_backend_subfont)
            .sfd_tree
            .is_null() as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            __assert_rtn(
                b"read_sfd\0" as *const u8 as *const ::core::ffi::c_char,
                b"subfont.c\0" as *const u8 as *const ::core::ffi::c_char,
                114 as ::core::ffi::c_int,
                b"sfd_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    sfd = avl_find(
        (*__tekai_state_generated_backend_subfont).sfd_tree,
        &raw mut tmp_sfd as *const ::core::ffi::c_void,
    ) as *mut sfd_entry;
    if !sfd.is_null() {
        return sfd;
    }
    (*__tekai_state_utils).cur_file_name = sfd_name;
    zpackfilename(
        maketexstring((*__tekai_state_utils).cur_file_name),
        getnullstr(),
        getnullstr(),
    );
    if open_input(
        &raw mut (*__tekai_state_generated_backend_subfont).sfd_file,
        kpse_sfd_format as ::core::ffi::c_int,
        FOPEN_RBIN_MODE.as_ptr(),
    ) == 0
    {
        crate::utils::pdftex_warn_args(
            b"cannot open SFD file for reading\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
        (*__tekai_state_utils).cur_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return ::core::ptr::null_mut::<sfd_entry>();
    }
    crate::utils::tex_printf_args(b"{\0" as *const u8 as *const ::core::ffi::c_char, &[]);
    crate::utils::tex_printf_args(
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(
            (*__tekai_state_utils).cur_file_name,
        )],
    );
    sfd = new_sfd_entry();
    (*sfd).name = xstrdup(sfd_name as const_string) as *mut ::core::ffi::c_char;
    while feof((*__tekai_state_generated_backend_subfont).sfd_file) == 0 {
        sfd_getline(true_0);
        if *(&raw mut (*__tekai_state_generated_backend_subfont).sfd_line
            as *mut ::core::ffi::c_char) as ::core::ffi::c_int
            == 10 as ::core::ffi::c_int
        {
            break;
        }
        sf = new_subfont_entry();
        (*sf).next = (*sfd).subfont;
        (*sfd).subfont = sf;
        let mut cursor = Cursor::new(
            CStr::from_ptr((&raw const (*__tekai_state_generated_backend_subfont).sfd_line).cast())
                .to_bytes(),
        );
        let infix = match cursor.word(None) {
            Ok(infix) if infix.len() < buf.len() => infix,
            _ => crate::utils::pdftex_fail_args(
                b"invalid token:\n%s\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(
                    (&raw const (*__tekai_state_generated_backend_subfont).sfd_line)
                        .cast::<::core::ffi::c_char>(),
                )],
            ),
        };
        ::core::ptr::copy_nonoverlapping(infix.as_ptr(), buf.as_mut_ptr().cast(), infix.len());
        buf[infix.len()] = 0;
        cursor.skip_whitespace();
        (*sf).infix = xstrdup(&raw mut buf as *mut ::core::ffi::c_char as const_string)
            as *mut ::core::ffi::c_char;
        p = (&raw mut (*__tekai_state_generated_backend_subfont).sfd_line
            as *mut ::core::ffi::c_char)
            .add(cursor.offset());
        k = 0 as ::core::ffi::c_long;
        loop {
            if *p as ::core::ffi::c_int == '\\' as i32 {
                sfd_getline(false_0);
                p = &raw mut (*__tekai_state_generated_backend_subfont).sfd_line
                    as *mut ::core::ffi::c_char;
            } else {
                if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    break;
                }
                let mut cursor = Cursor::new(CStr::from_ptr(p).to_bytes());
                i = match cursor.integer(IntegerBase::Auto) {
                    Ok(value) => value,
                    Err(_) => crate::utils::pdftex_fail_args(
                        b"invalid token:\n%s\0" as *const u8 as *const ::core::ffi::c_char,
                        &[crate::utils::PrintfArg::from(p)],
                    ),
                };
                cursor.skip_whitespace();
                if cursor.literal(b":").is_ok() {
                    if checked_span(i, 0, (*sf).charcodes.len()).is_none() {
                        crate::utils::pdftex_fail_args(
                            b"invalid index:\n%s\0" as *const u8 as *const ::core::ffi::c_char,
                            &[crate::utils::PrintfArg::from(p)],
                        );
                    }
                    k = i;
                } else if cursor.literal(b"_").is_ok() {
                    j = match cursor.integer(IntegerBase::Auto) {
                        Ok(value) => value,
                        Err(_) => crate::utils::pdftex_fail_args(
                            b"invalid token:\n%s\0" as *const u8 as *const ::core::ffi::c_char,
                            &[crate::utils::PrintfArg::from(p)],
                        ),
                    };
                    cursor.skip_whitespace();
                    let span = match checked_inclusive_span(k, i, j, (*sf).charcodes.len()) {
                        Some(span) => span,
                        None => crate::utils::pdftex_fail_args(
                            b"invalid range:\n%s\0" as *const u8 as *const ::core::ffi::c_char,
                            &[crate::utils::PrintfArg::from(p)],
                        ),
                    };
                    for (offset, slot) in span.clone().enumerate() {
                        (*sf).charcodes[slot] = i + offset as ::core::ffi::c_long;
                    }
                    k = span.end as ::core::ffi::c_long;
                } else {
                    let span = match checked_span(k, 1, (*sf).charcodes.len()) {
                        Some(span) => span,
                        None => crate::utils::pdftex_fail_args(
                            b"invalid index:\n%s\0" as *const u8 as *const ::core::ffi::c_char,
                            &[crate::utils::PrintfArg::from(p)],
                        ),
                    };
                    (*sf).charcodes[span.start] = i;
                    k = span.end as ::core::ffi::c_long;
                }
                p = p.add(cursor.offset());
            }
        }
    }
    xfclose(
        (*__tekai_state_generated_backend_subfont).sfd_file,
        (*__tekai_state_utils).cur_file_name as const_string,
    );
    crate::utils::tex_printf_args(b"}\0" as *const u8 as *const ::core::ffi::c_char, &[]);
    aa = avl_probe(
        (*__tekai_state_generated_backend_subfont).sfd_tree,
        sfd as *mut ::core::ffi::c_void,
    );
    if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"read_sfd\0" as *const u8 as *const ::core::ffi::c_char,
            b"subfont.c\0" as *const u8 as *const ::core::ffi::c_char,
            169 as ::core::ffi::c_int,
            b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    return sfd;
}
pub unsafe fn handle_subfont_fm(mut fm: *mut fm_entry, mut mode: ::core::ffi::c_int) -> boolean {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sfd: *mut sfd_entry = ::core::ptr::null_mut::<sfd_entry>();
    let mut sf: *mut subfont_entry = ::core::ptr::null_mut::<subfont_entry>();
    let mut fm2: *mut fm_entry = ::core::ptr::null_mut::<fm_entry>();
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    if (*fm).tfm_name.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"handle_subfont_fm\0" as *const u8 as *const ::core::ffi::c_char,
            b"subfont.c\0" as *const u8 as *const ::core::ffi::c_char,
            181 as ::core::ffi::c_int,
            b"fm->tfm_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    p = (*fm).tfm_name;
    q = strchr(p, '@' as i32);
    if q.is_null() {
        return false_0;
    }
    r = strchr(q.offset(1 as ::core::ffi::c_int as isize), '@' as i32);
    if r.is_null() {
        return false_0;
    }
    let tfm_name = CStr::from_ptr(p).to_bytes();
    if q <= p
        || r <= q.offset(1 as ::core::ffi::c_int as isize)
        || r.offset_from(p) as usize != tfm_name.len() - 1
    {
        return false_0;
    }
    let first_at = q.offset_from(p) as usize;
    let second_at = r.offset_from(p) as usize;
    let sfd_name = &tfm_name[first_at + 1..second_at];
    let suffix = b".sfd\0";
    if sfd_name
        .len()
        .checked_add(suffix.len())
        .is_none_or(|length| length > buf.len())
    {
        crate::utils::pdftex_fail_args(
            b"buffer overflow at file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(
                    b"tekai-engine/generated/backend/subfont.rs\0" as *const u8
                        as *const ::core::ffi::c_char,
                ),
                crate::utils::PrintfArg::from(196 as ::core::ffi::c_int),
            ],
        );
    }
    ::core::ptr::copy_nonoverlapping(sfd_name.as_ptr(), buf.as_mut_ptr().cast(), sfd_name.len());
    ::core::ptr::copy_nonoverlapping(
        suffix.as_ptr(),
        buf.as_mut_ptr().cast::<u8>().add(sfd_name.len()),
        suffix.len(),
    );
    sfd = read_sfd(&raw mut buf as *mut ::core::ffi::c_char);
    if sfd.is_null() {
        return false_0;
    }
    (*fm).type_0 = ((*fm).type_0 as ::core::ffi::c_int | F_SUBFONT) as ::core::ffi::c_ushort;
    if !(*fm).ps_name.is_null() {
        free((*fm).ps_name as *mut ::core::ffi::c_void);
    }
    (*fm).ps_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*fm).ps_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*fm).pid as ::core::ffi::c_int == -(1 as ::core::ffi::c_int) {
        (*fm).pid = 3 as ::core::ffi::c_short;
        (*fm).eid = 1 as ::core::ffi::c_short;
    }
    let prefix = &tfm_name[..first_at];
    sf = (*sfd).subfont;
    while !sf.is_null() {
        let infix = CStr::from_ptr((*sf).infix).to_bytes();
        let length = match prefix.len().checked_add(infix.len()) {
            Some(length) if length < buf.len() => length,
            _ => crate::utils::pdftex_fail_args(
                b"buffer overflow at file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(
                        b"tekai-engine/generated/backend/subfont.rs\0" as *const u8
                            as *const ::core::ffi::c_char,
                    ),
                    crate::utils::PrintfArg::from(196 as ::core::ffi::c_int),
                ],
            ),
        };
        ::core::ptr::copy_nonoverlapping(prefix.as_ptr(), buf.as_mut_ptr().cast(), prefix.len());
        ::core::ptr::copy_nonoverlapping(
            infix.as_ptr(),
            buf.as_mut_ptr().cast::<u8>().add(prefix.len()),
            infix.len(),
        );
        buf[length] = 0;
        fm2 = new_fm_entry();
        (*fm2).tfm_name = xstrdup(&raw mut buf as *mut ::core::ffi::c_char as const_string)
            as *mut ::core::ffi::c_char;
        (*fm2).ff_name = xstrdup((*fm).ff_name as const_string) as *mut ::core::ffi::c_char;
        (*fm2).type_0 = (*fm).type_0;
        (*fm2).pid = (*fm).pid;
        (*fm2).eid = (*fm).eid;
        (*fm2).subfont = sf;
        if avl_do_entry(fm2, mode) != 0 as ::core::ffi::c_int {
            delete_fm_entry(fm2);
        }
        sf = (*sf).next;
    }
    delete_fm_entry(fm);
    return true_0;
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;

pub(crate) struct State {
    sfd_tree: *mut avl_table,
    sfd_file: *mut FILE,
    sfd_line: [::core::ffi::c_char; 256],
}

#[inline(always)]
fn __tekai_require_copy<T: Copy>() {}

pub(crate) struct StateInitialization {
    destination: *mut State,
    initialized: [bool; 3],
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
            if self.initialized[2] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).sfd_line));
            }
            if self.initialized[1] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).sfd_file));
            }
            if self.initialized[0] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).sfd_tree));
            }
        }
    }
}
impl StateInitialization {
    pub(crate) unsafe fn begin(destination: *mut State) -> Self {
        Self {
            destination,
            initialized: [false; 3],
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
        core::ptr::addr_of_mut!((*destination).sfd_tree)
            .write(::core::ptr::null::<avl_table>() as *mut avl_table);
        self.initialized[0] = true;
        const {
            assert!(core::mem::size_of::<*mut FILE>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).sfd_file)
            .write(::core::ptr::null::<FILE>() as *mut FILE);
        self.initialized[1] = true;
        const {
            assert!((256) == (256));
            assert!(core::mem::size_of::<::core::ffi::c_char>() <= 16 * 1024);
        }
        __tekai_require_copy::<::core::ffi::c_char>();
        {
            let __tekai_repeat_0: ::core::ffi::c_char = 0;
            for __tekai_index_0 in 0..(256) {
                core::ptr::addr_of_mut!((*destination).sfd_line[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[2] = true;
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
        (*crate::invocation_state::current()).generated_backend_subfont
    ))
}
#[cfg(test)]
pub(crate) unsafe fn check_state_construction(phase: u8) {
    let invocation = crate::invocation_state::current();
    let destination = state();
    assert!(!destination.is_null());
    let _ = (invocation, phase);
}
