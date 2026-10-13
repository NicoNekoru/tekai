// Rust translation and subsequent modifications for Tekai.
// Upstream TeX Live source revision 52dac44dde13a8310bbb8231e81c58be28cebe20.
// Origin texk/web2c/pdftexdir/pdftexextra.c, lib/texmfmp.c, and engine headers.
// GPL-2.0-or-later.
// Modified for Tekai. Licensing and provenance notice restored 2026-10-13.
// Original copyrights and license terms are retained in the crate's
// NOTICE.md and licenses/upstream-notices.txt.

pub(crate) use crate::kpathsea::{
    kpathsea_cnf_line_env_progname, kpse_absolute_p, kpse_find_file, kpse_in_name_ok,
    kpse_maketex_option, kpse_readable_file, kpse_reset_program_name, kpse_set_program_enabled,
    kpse_set_program_name, kpse_var_value,
};
pub use crate::md5::{md5_append, md5_finish, md5_init};
pub use crate::openclose::{recorder_record_input, recorder_record_output};
pub use crate::support::{
    concat, concat3, dir_p, find_suffix, printversionandexit, usagehelp, xputenv,
};
pub use crate::utils::{initversionstring, maketexstring};

use crate::byte_string::{memcpy, strchr, strcmp, strcpy, strlen, strncmp, strncpy};
use crate::environment::getenv;
use crate::stdio::system;

pub use crate::core_types::{
    AsciiCode as ASCIIcode, Boolean as boolean, CharUsedArray as charusedarray,
    DestinationNameEntry as destnameentry, EightBits as eightbits, FmEntryPtr as fmentryptr,
    FontIndex as fontindex, FontMemoryWord as fmemoryword, FourQuarters as fourquarters,
    GlueOrder as glueord, GroupCode as groupcode, Halfword as halfword,
    HalfwordPair as C2RustUnnamed_1, HyphPointer as hyphpointer, InputStateRecord as instaterecord,
    Integer as integer, InternalFontNumber as internalfontnumber,
    ListStateRecord as liststaterecord, LongInteger as longinteger, MemoryWord as memoryword,
    NineBits as ninebits, ObjectEntry as objentry, PackedAsciiCode as packedASCIIcode,
    PdfLinkStackRecord as pdflinkstackrecord, PoolPointer as poolpointer,
    QuarterBytes as C2RustUnnamed_2, Quarterword as quarterword, Real as real, SChar as schar,
    SavePointer as savepointer, Scaled as scaled, SmallNumber as smallnumber,
    StrNumber as strnumber, Text as text, TrieOpcode as trieopcode, TriePointer as triepointer,
    TwoHalves as twohalves, VfStackIndex as vfstackindex, VfStackRecord as vfstackrecord,
    WordFile as wordfile,
};

use crate::integer_parse::{decimal_i32_legacy, signed_prefix, IntegerBase};
use std::ffi::CStr;
use std::os::unix::ffi::OsStrExt;

fn readable_path(path: &std::path::Path) -> bool {
    crate::working_directory::with_resolved(path, |path| {
        let path =
            std::ffi::CString::new(path.as_os_str().as_bytes()).map_err(std::io::Error::other)?;
        // This permission query is an OS interface. Preserve access(R_OK)'s
        // real-user-ID semantics instead of substituting an effective-ID open.
        if unsafe { libc::access(path.as_ptr(), libc::R_OK) } == 0 {
            Ok(())
        } else {
            Err(std::io::Error::last_os_error())
        }
    })
    .is_ok()
}

use crate::openclose::{close_file, open_input, open_output};
use crate::stdio::{gzread, gzwrite};
use crate::support::{read_line, xfclose, xfopen};
use crate::web2c::uexit;

use crate::stdio::{
    exit, fclose, fflush, fopen, fputs, fread, fseek, pclose, popen, setvbuf, ungetc,
};

use crate::allocation::free;
use crate::openclose::{xmalloc, xrealloc, xstrdup};

use crate::termination::__assert_rtn;

#[inline(always)]
unsafe fn fast_fgetc(stream: *mut FILE) -> ::core::ffi::c_int {
    unsafe { crate::stdio::fgetc(stream) }
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __darwin_size_t = usize;
pub type __darwin_wchar_t = ::libc::wchar_t;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = isize;
pub type __darwin_time_t = ::core::ffi::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
pub use crate::stdio::Stream as FILE;
pub type off_t = __darwin_off_t;
pub type ssize_t = __darwin_ssize_t;
pub type dev_t = __darwin_dev_t;
pub type blkcnt_t = __darwin_blkcnt_t;
pub type blksize_t = __darwin_blksize_t;
pub type gid_t = __darwin_gid_t;
pub type mode_t = __darwin_mode_t;
pub type nlink_t = __uint16_t;
pub type uid_t = __darwin_uid_t;
pub type time_t = __darwin_time_t;
pub type string = *mut ::core::ffi::c_char;
pub type const_string = *const ::core::ffi::c_char;
pub type address = *mut ::core::ffi::c_void;
pub use crate::kpathsea_types::cache_entry;
pub use crate::kpathsea_types::expansion_type;
#[cfg(test)]
pub use crate::kpathsea_types::hash_element_struct;
pub use crate::kpathsea_types::hash_table_type;
pub use crate::kpathsea_types::kpse_file_format_type;
pub use crate::kpathsea_types::p_record_input;
pub use crate::kpathsea_types::p_record_output;
pub use crate::kpathsea_types::str_list_type;
#[cfg(test)]
pub use crate::kpathsea_types::str_llist_elt;
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
pub use crate::kpathsea_types::kpse_src_type;
pub const kpse_src_cmdline: kpse_src_type = 6;
pub const kpse_src_x: kpse_src_type = 5;
pub const kpse_src_env: kpse_src_type = 4;
pub const kpse_src_client_cnf: kpse_src_type = 3;
pub const kpse_src_texmf_cnf: kpse_src_type = 2;
pub const kpse_src_compile: kpse_src_type = 1;
pub const kpse_src_implicit: kpse_src_type = 0;
pub use crate::kpathsea_types::kpathsea;
pub use crate::kpathsea_types::kpathsea_instance;
pub use crate::kpathsea_types::kpse_format_info_type;
pub type md5_byte_t = ::core::ffi::c_uchar;
pub type md5_state_t = md5_state_s;
pub use crate::md5::Md5State as md5_state_s;
pub type md5_word_t = ::core::ffi::c_uint;
pub type socklen_t = __darwin_socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_len: __uint8_t,
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
pub type sa_family_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub msg: msg,
    pub more_data: [::core::ffi::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msg {
    pub namelength: ::core::ffi::c_int,
    pub eof: ::core::ffi::c_int,
}
pub type voidpc = *const ::core::ffi::c_void;
pub type voidp = *mut ::core::ffi::c_void;
pub use crate::stdio::Stream as gzFile_s;
pub type gzFile = *mut gzFile_s;
pub type alphafile = text;
pub type bytefile = text;
pub const MAKE_TEX_FMT_BY_DEFAULT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MAKE_TEX_TEX_BY_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MAKE_TEX_TFM_BY_DEFAULT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const _IONBF: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const R_OK: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EDITOR: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"vi +%d '%s'\0") };
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FOPEN_R_MODE: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"r\0") };
pub const FOPEN_RBIN_MODE: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"rb\0") };
pub const O_NONBLOCK: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const F_SETFL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DIR_SEP: ::core::ffi::c_int = '/' as i32;
pub const DIR_SEP_STRING: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"/\0") };
pub const TEXMFENGINENAME: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"pdftex\0") };

pub const COPYRIGHT_HOLDER: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"Tekai contributors\0")
};
pub const AUTHOR: *mut ::core::ffi::c_void = NULL;
pub const BUG_ADDRESS: [::core::ffi::c_char; 43] = unsafe {
    ::core::mem::transmute::<[u8; 43], [::core::ffi::c_char; 43]>(
        *b"https://github.com/NicoNekoru/tekai/issues\0",
    )
};
pub const DUMP_OPTION: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"fmt\0") };
pub const DUMP_EXT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b".fmt\0") };

pub const edit_var: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"TEXEDIT\0") };
unsafe fn Isspace(mut c_0: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return (c_0 as ::core::ffi::c_int == ' ' as i32 || c_0 as ::core::ffi::c_int == '\t' as i32)
        as ::core::ffi::c_int;
}

unsafe fn mk_shellcmdlist(mut v: *mut ::core::ffi::c_char) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut p: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: size_t = 0;
    q = v;
    n = 1 as size_t;
    loop {
        r = strchr(q, ',' as i32);
        if r.is_null() {
            break;
        }
        n = n.wrapping_add(1);
        q = r.offset(1 as ::core::ffi::c_int as isize);
    }
    if *q != 0 {
        n = n.wrapping_add(1);
    }
    (*__tekai_state_generated_pdftexextra).cmdlist =
        xmalloc(n.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t))
            as *mut *mut ::core::ffi::c_char;
    p = (*__tekai_state_generated_pdftexextra).cmdlist;
    q = v;
    loop {
        r = strchr(q, ',' as i32);
        if r.is_null() {
            break;
        }
        *r = '\0' as i32 as ::core::ffi::c_char;
        let fresh20 = p;
        p = p.offset(1);
        *fresh20 = xstrdup(q as const_string) as *mut ::core::ffi::c_char;
        q = r.offset(1 as ::core::ffi::c_int as isize);
    }
    if *q != 0 {
        let fresh21 = p;
        p = p.offset(1);
        *fresh21 = xstrdup(q as const_string) as *mut ::core::ffi::c_char;
    }
    *p = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe fn init_shell_escape() {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    if (*__tekai_state_generated_pdftexextra).shellenabledp < 0 as ::core::ffi::c_int {
        (*__tekai_state_generated_pdftexextra).shellenabledp = 0 as ::core::ffi::c_int;
    } else {
        if (*__tekai_state_generated_pdftexextra).shellenabledp == 0 as ::core::ffi::c_int {
            let mut v1: *mut ::core::ffi::c_char =
                kpse_var_value(b"shell_escape\0" as *const u8 as const_string)
                    as *mut ::core::ffi::c_char;
            if !v1.is_null() {
                if *v1 as ::core::ffi::c_int == 't' as i32
                    || *v1 as ::core::ffi::c_int == 'y' as i32
                    || *v1 as ::core::ffi::c_int == '1' as i32
                {
                    (*__tekai_state_generated_pdftexextra).shellenabledp = 1 as ::core::ffi::c_int;
                } else if *v1 as ::core::ffi::c_int == 'p' as i32 {
                    (*__tekai_state_generated_pdftexextra).shellenabledp = 1 as ::core::ffi::c_int;
                    (*__tekai_state_generated_pdftexextra).restrictedshell =
                        1 as ::core::ffi::c_int;
                }
                free(v1 as *mut ::core::ffi::c_void);
            }
        }
        if (*__tekai_state_generated_pdftexextra).shellenabledp != 0
            && (*__tekai_state_generated_pdftexextra).restrictedshell == 1 as ::core::ffi::c_int
        {
            let mut v2: *mut ::core::ffi::c_char =
                kpse_var_value(b"shell_escape_commands\0" as *const u8 as const_string)
                    as *mut ::core::ffi::c_char;
            if !v2.is_null() {
                mk_shellcmdlist(v2);
                free(v2 as *mut ::core::ffi::c_void);
            }
        }
    };
}
pub const QUOTE: ::core::ffi::c_int = '\'' as i32;
unsafe fn shell_cmd_is_allowed(
    mut cmd: *const ::core::ffi::c_char,
    mut safecmd: *mut *mut ::core::ffi::c_char,
    mut cmdname: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut p: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut c_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut d: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut pre: ::core::ffi::c_int = 0;
    let mut spaces: ::core::ffi::c_int = 0;
    let mut allow: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    buf = xmalloc(strlen(cmd).wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    strcpy(buf, cmd);
    c_0 = buf;
    while Isspace(*c_0) != 0 {
        c_0 = c_0.offset(1);
    }
    d = c_0;
    while Isspace(*d) == 0 && *d as ::core::ffi::c_int != 0 {
        d = d.offset(1);
    }
    *d = '\0' as i32 as ::core::ffi::c_char;
    *cmdname = xstrdup(c_0 as const_string) as *mut ::core::ffi::c_char;
    free(buf as *mut ::core::ffi::c_void);
    p = (*__tekai_state_generated_pdftexextra).cmdlist;
    if !p.is_null() {
        while !(*p).is_null() {
            if strcmp(*p, *cmdname) == 0 as ::core::ffi::c_int {
                allow = 2 as ::core::ffi::c_int;
                break;
            } else {
                p = p.offset(1);
            }
        }
    }
    if allow == 2 as ::core::ffi::c_int {
        spaces = 0 as ::core::ffi::c_int;
        s = cmd;
        while *s != 0 {
            if Isspace(*s) != 0 {
                spaces += 1;
            }
            s = s.offset(1);
        }
        *safecmd = xmalloc(
            strlen(cmd)
                .wrapping_add(3 as size_t)
                .wrapping_add((2 as ::core::ffi::c_int * spaces) as size_t),
        ) as *mut ::core::ffi::c_char;
        s = cmd;
        while Isspace(*s) != 0 {
            s = s.offset(1);
        }
        d = *safecmd;
        while Isspace(*s) == 0 && *s as ::core::ffi::c_int != 0 {
            let fresh5 = s;
            s = s.offset(1);
            let fresh6 = d;
            d = d.offset(1);
            *fresh6 = *fresh5;
        }
        pre = 1 as ::core::ffi::c_int;
        while *s != 0 {
            if *s as ::core::ffi::c_int == '\'' as i32 {
                return -(1 as ::core::ffi::c_int);
            }
            if *s as ::core::ffi::c_int == '"' as i32 {
                if pre == 0 as ::core::ffi::c_int {
                    let fresh7 = d;
                    d = d.offset(1);
                    *fresh7 = QUOTE as ::core::ffi::c_char;
                }
                pre = 0 as ::core::ffi::c_int;
                let fresh8 = d;
                d = d.offset(1);
                *fresh8 = QUOTE as ::core::ffi::c_char;
                s = s.offset(1);
                while *s as ::core::ffi::c_int != '"' as i32 {
                    if *s as ::core::ffi::c_int == '\'' as i32
                        || *s as ::core::ffi::c_int == '\0' as i32
                    {
                        return -(1 as ::core::ffi::c_int);
                    }
                    let fresh9 = s;
                    s = s.offset(1);
                    let fresh10 = d;
                    d = d.offset(1);
                    *fresh10 = *fresh9;
                }
                s = s.offset(1);
                if Isspace(*s) == 0 && *s as ::core::ffi::c_int != 0 {
                    return -(1 as ::core::ffi::c_int);
                }
            } else if pre == 1 as ::core::ffi::c_int && Isspace(*s) == 0 {
                pre = 0 as ::core::ffi::c_int;
                let fresh11 = d;
                d = d.offset(1);
                *fresh11 = QUOTE as ::core::ffi::c_char;
                let fresh12 = s;
                s = s.offset(1);
                let fresh13 = d;
                d = d.offset(1);
                *fresh13 = *fresh12;
            } else if pre == 0 as ::core::ffi::c_int && Isspace(*s) != 0 {
                pre = 1 as ::core::ffi::c_int;
                let fresh14 = d;
                d = d.offset(1);
                *fresh14 = QUOTE as ::core::ffi::c_char;
                let fresh15 = s;
                s = s.offset(1);
                let fresh16 = d;
                d = d.offset(1);
                *fresh16 = *fresh15;
            } else {
                let fresh17 = s;
                s = s.offset(1);
                let fresh18 = d;
                d = d.offset(1);
                *fresh18 = *fresh17;
            }
        }
        if pre == 0 as ::core::ffi::c_int {
            let fresh19 = d;
            d = d.offset(1);
            *fresh19 = QUOTE as ::core::ffi::c_char;
        }
        *d = '\0' as i32 as ::core::ffi::c_char;
    }
    return allow;
}
pub unsafe fn runsystem(mut cmd: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut allow: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut safecmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cmdname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*__tekai_state_generated_pdftexextra).shellenabledp <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (*__tekai_state_generated_pdftexextra).restrictedshell == 0 as ::core::ffi::c_int {
        allow = 1 as ::core::ffi::c_int;
    } else {
        allow = shell_cmd_is_allowed(cmd, &raw mut safecmd, &raw mut cmdname);
    }
    if allow == 1 as ::core::ffi::c_int {
        status = system(cmd);
        crate::lookup::reset();
    } else if allow == 2 as ::core::ffi::c_int {
        let mut k_0: size_t = 0;
        k_0 = 0 as size_t;
        while k_0 < strlen(safecmd) {
            if *safecmd.offset(k_0 as isize) as ::core::ffi::c_int == '|' as i32 {
                return 0 as ::core::ffi::c_int;
            }
            k_0 = k_0.wrapping_add(1);
        }
        status = system(safecmd);
        crate::lookup::reset();
    }
    if status != 0 as ::core::ffi::c_int {
        crate::stdio::fprintf_args(
            crate::stdio::stderr(),
            b"system returned with code %d\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(status)],
        );
    }
    if !safecmd.is_null() {
        free(safecmd as *mut ::core::ffi::c_void);
    }
    if !cmdname.is_null() {
        free(cmdname as *mut ::core::ffi::c_void);
    }
    return allow;
}
unsafe fn runpopen(
    mut cmd: *mut ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> *mut FILE {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut f_0: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut safecmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cmdname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut allow: ::core::ffi::c_int = 0;
    if (*__tekai_state_generated_pdftexextra).restrictedshell == 0 as ::core::ffi::c_int {
        allow = 1 as ::core::ffi::c_int;
    } else {
        allow = shell_cmd_is_allowed(cmd, &raw mut safecmd, &raw mut cmdname);
    }
    if allow == 1 as ::core::ffi::c_int {
        f_0 = popen(cmd, mode);
    } else if allow == 2 as ::core::ffi::c_int {
        f_0 = popen(safecmd, mode);
    } else if allow == -(1 as ::core::ffi::c_int) {
        crate::stdio::fprintf_args(
            crate::stdio::stderr(),
            b"\nrunpopen quotation error in command line: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(cmd)],
        );
    } else {
        crate::stdio::fprintf_args(
            crate::stdio::stderr(),
            b"\nrunpopen command not allowed: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(cmdname)],
        );
    }
    if !safecmd.is_null() {
        free(safecmd as *mut ::core::ffi::c_void);
    }
    if !cmdname.is_null() {
        free(cmdname as *mut ::core::ffi::c_void);
    }
    return f_0;
}

// Temporary generated caller state. Move this into invocation-owned state with
// argv; parse_options resets it for each invocation and never touches libc.

pub unsafe fn texmf_yesno(mut var: const_string) -> boolean {
    let mut value: string = kpse_var_value(var);
    return (!value.is_null()
        && (*value as ::core::ffi::c_int == 't' as i32
            || *value as ::core::ffi::c_int == 'y' as i32
            || *value as ::core::ffi::c_int == '1' as i32)) as ::core::ffi::c_int;
}

pub unsafe fn maininit(mut ac: ::core::ffi::c_int, mut av: *mut string) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_kpathsea: *mut crate::kpathsea::State = unsafe { crate::kpathsea::state() };

    let mut main_input_file: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*__tekai_state_generated_pdftexextra).argc = ac;
    (*__tekai_state_generated_pdftexextra).argv = av as *mut *mut ::core::ffi::c_char;
    (*__tekai_state_generated_pdftexextra).interactionoption = 4 as ::core::ffi::c_uchar;
    (*__tekai_state_kpathsea).kpse_def_inst.record_input =
        Some(recorder_record_input as unsafe fn(const_string) -> ()) as p_record_input;
    (*__tekai_state_kpathsea).kpse_def_inst.record_output =
        Some(recorder_record_output as unsafe fn(const_string) -> ()) as p_record_output;
    parse_options(ac, av);
    if (*__tekai_state_generated_pdftexextra)
        .user_progname
        .is_null()
    {
        (*__tekai_state_generated_pdftexextra).user_progname =
            (*__tekai_state_generated_pdftexextra).dump_name;
    }
    kpse_set_program_name(
        *(*__tekai_state_generated_pdftexextra)
            .argv
            .offset(0 as ::core::ffi::c_int as isize) as const_string,
        (*__tekai_state_generated_pdftexextra).user_progname,
    );
    xputenv(
        b"engine\0" as *const u8 as const_string,
        TEXMFENGINENAME.as_ptr(),
    );
    if !(*__tekai_state_generated_pdftexextra)
        .user_cnf_lines
        .is_null()
    {
        let mut i: ::core::ffi::c_uint = 0;
        i = 0 as ::core::ffi::c_uint;
        while i < (*__tekai_state_generated_pdftexextra).user_cnf_nlines {
            kpathsea_cnf_line_env_progname(
                (*__tekai_state_kpathsea).kpse_def,
                *(*__tekai_state_generated_pdftexextra)
                    .user_cnf_lines
                    .offset(i as isize),
            );
            free(
                *(*__tekai_state_generated_pdftexextra)
                    .user_cnf_lines
                    .offset(i as isize) as *mut ::core::ffi::c_void,
            );
            i = i.wrapping_add(1);
        }
    }
    main_input_file = get_input_file_name();
    if (*__tekai_state_generated_pdftexextra).filelineerrorstylep < 0 as ::core::ffi::c_int {
        (*__tekai_state_generated_pdftexextra).filelineerrorstylep = 0 as ::core::ffi::c_int;
    } else if (*__tekai_state_generated_pdftexextra).filelineerrorstylep == 0 {
        (*__tekai_state_generated_pdftexextra).filelineerrorstylep =
            texmf_yesno(b"file_line_error_style\0" as *const u8 as const_string)
                as ::core::ffi::c_int;
    }
    if (*__tekai_state_generated_pdftexextra).parsefirstlinep < 0 as ::core::ffi::c_int {
        (*__tekai_state_generated_pdftexextra).parsefirstlinep = 0 as ::core::ffi::c_int;
    } else if (*__tekai_state_generated_pdftexextra).parsefirstlinep == 0 {
        (*__tekai_state_generated_pdftexextra).parsefirstlinep =
            texmf_yesno(b"parse_first_line\0" as *const u8 as const_string) as ::core::ffi::c_int;
    }
    if (*__tekai_state_generated_pdftexextra).parsefirstlinep != 0
        && ((*__tekai_state_generated_pdftexextra).dump_name.is_null()
            || (*__tekai_state_generated_pdftexextra)
                .translate_filename
                .is_null())
    {
        parse_first_line(main_input_file as const_string);
    }
    if (*__tekai_state_generated_pdftexextra)
        .translate_filename
        .is_null()
    {
        (*__tekai_state_generated_pdftexextra).translate_filename =
            (*__tekai_state_generated_pdftexextra).default_translate_filename;
    }
    if (*__tekai_state_generated_pdftexextra).readyalready != 314159 as ::core::ffi::c_int {
        let mut virversion: boolean = false_0;
        if !(*__tekai_state_kpathsea)
            .kpse_def_inst
            .program_name
            .is_null()
            && !(b"pdfinitex\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                (*__tekai_state_kpathsea).kpse_def_inst.program_name as *const ::core::ffi::c_char,
                b"pdfinitex\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            (*__tekai_state_generated_pdftexextra).iniversion = true_0 as boolean;
        } else if !(*__tekai_state_kpathsea)
            .kpse_def_inst
            .program_name
            .is_null()
            && !(b"pdfvirtex\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                (*__tekai_state_kpathsea).kpse_def_inst.program_name as *const ::core::ffi::c_char,
                b"pdfvirtex\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            virversion = true_0 as boolean;
        } else if !(*__tekai_state_kpathsea)
            .kpse_def_inst
            .program_name
            .is_null()
            && !(b"initex\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                (*__tekai_state_kpathsea).kpse_def_inst.program_name as *const ::core::ffi::c_char,
                b"initex\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            (*__tekai_state_generated_pdftexextra).iniversion = true_0 as boolean;
        } else if !(*__tekai_state_kpathsea)
            .kpse_def_inst
            .program_name
            .is_null()
            && !(b"virtex\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                (*__tekai_state_kpathsea).kpse_def_inst.program_name as *const ::core::ffi::c_char,
                b"virtex\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            virversion = true_0 as boolean;
        } else if !(*__tekai_state_kpathsea)
            .kpse_def_inst
            .program_name
            .is_null()
            && !(b"mltex\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                (*__tekai_state_kpathsea).kpse_def_inst.program_name as *const ::core::ffi::c_char,
                b"mltex\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            (*__tekai_state_generated_pdftexextra).mltexp = true_0 as boolean;
        }
        if main_input_file.is_null() {
            if !(*(*__tekai_state_generated_pdftexextra)
                .argv
                .offset(1 as ::core::ffi::c_int as isize))
            .is_null()
                && **(*__tekai_state_generated_pdftexextra)
                    .argv
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '&' as i32
            {
                (*__tekai_state_generated_pdftexextra).dump_name =
                    (*(*__tekai_state_generated_pdftexextra)
                        .argv
                        .offset(1 as ::core::ffi::c_int as isize))
                    .offset(1 as ::core::ffi::c_int as isize) as const_string;
            }
        }
        if (*__tekai_state_generated_pdftexextra).dump_name.is_null() {
            (*__tekai_state_generated_pdftexextra).dump_name = (if virversion != 0 {
                b"plain\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                (*__tekai_state_kpathsea).kpse_def_inst.program_name as *const ::core::ffi::c_char
            }) as const_string;
        }
    }
    if (*__tekai_state_generated_pdftexextra).iniversion == 0 {
        if (*__tekai_state_generated_pdftexextra).mltexp != 0 {
            crate::stdio::fprintf_args(
                crate::stdio::stderr(),
                b"-mltex only works with -ini\n\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        if (*__tekai_state_generated_pdftexextra).enctexp != 0 {
            crate::stdio::fprintf_args(
                crate::stdio::stderr(),
                b"-enc only works with -ini\n\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        if (*__tekai_state_generated_pdftexextra).etexp != 0 {
            crate::stdio::fprintf_args(
                crate::stdio::stderr(),
                b"-etex only works with -ini\n\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
    }
    if !(*__tekai_state_generated_pdftexextra).dump_name.is_null() {
        let mut with_ext: const_string = ::core::ptr::null::<::core::ffi::c_char>();
        let mut name_len: ::core::ffi::c_uint =
            strlen((*__tekai_state_generated_pdftexextra).dump_name as *const ::core::ffi::c_char)
                as ::core::ffi::c_uint;
        let mut ext_len: ::core::ffi::c_uint = strlen(DUMP_EXT.as_ptr()) as ::core::ffi::c_uint;
        if name_len > ext_len
            && (!(*__tekai_state_generated_pdftexextra)
                .dump_name
                .offset(name_len as isize)
                .offset(-(ext_len as isize))
                .is_null()
                && !(b".fmt\0" as *const u8 as *const ::core::ffi::c_char).is_null()
                && strcmp(
                    (*__tekai_state_generated_pdftexextra)
                        .dump_name
                        .offset(name_len as isize)
                        .offset(-(ext_len as isize)),
                    b".fmt\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int)
        {
            with_ext = (*__tekai_state_generated_pdftexextra).dump_name;
        } else {
            with_ext = concat(
                (*__tekai_state_generated_pdftexextra).dump_name,
                DUMP_EXT.as_ptr(),
            ) as const_string;
        }
        (*__tekai_state_generated_pdftexextra).TEXformatdefault =
            concat(b" \0" as *const u8 as const_string, with_ext);
        (*__tekai_state_generated_pdftexextra).formatdefaultlength = strlen(
            (*__tekai_state_generated_pdftexextra)
                .TEXformatdefault
                .offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
        ) as integer;
    } else {
        crate::termination::abort();
    }
    kpse_set_program_enabled(kpse_tfm_format, MAKE_TEX_TFM_BY_DEFAULT, kpse_src_compile);
    kpse_set_program_enabled(kpse_tex_format, MAKE_TEX_TEX_BY_DEFAULT, kpse_src_compile);
    kpse_set_program_enabled(kpse_fmt_format, MAKE_TEX_FMT_BY_DEFAULT, kpse_src_compile);
    init_shell_escape();
    if (*__tekai_state_generated_pdftexextra)
        .outputcomment
        .is_null()
    {
        (*__tekai_state_generated_pdftexextra).outputcomment =
            kpse_var_value(b"output_comment\0" as *const u8 as const_string)
                as *mut ::core::ffi::c_char;
    }
}
pub unsafe fn topenin() {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut i: ::core::ffi::c_int = 0;
    *(*__tekai_state_generated_pdftexextra)
        .buffer
        .offset((*__tekai_state_generated_pdftexextra).first as isize) = 0 as ASCIIcode;
    if (*__tekai_state_generated_pdftexextra).optind < (*__tekai_state_generated_pdftexextra).argc {
        let mut k_0: ::core::ffi::c_int =
            (*__tekai_state_generated_pdftexextra).first as ::core::ffi::c_int;
        i = (*__tekai_state_generated_pdftexextra).optind;
        while i < (*__tekai_state_generated_pdftexextra).argc {
            let mut ptr: *mut ::core::ffi::c_char = (*(*__tekai_state_generated_pdftexextra)
                .argv
                .offset(i as isize))
            .offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char;
            while *ptr != 0 {
                let fresh30 = ptr;
                ptr = ptr.offset(1);
                let fresh31 = k_0;
                k_0 = k_0 + 1;
                *(*__tekai_state_generated_pdftexextra)
                    .buffer
                    .offset(fresh31 as isize) = *fresh30 as ASCIIcode;
            }
            let fresh32 = k_0;
            k_0 = k_0 + 1;
            *(*__tekai_state_generated_pdftexextra)
                .buffer
                .offset(fresh32 as isize) = ' ' as i32 as ASCIIcode;
            i += 1;
        }
        (*__tekai_state_generated_pdftexextra).argc = 0 as ::core::ffi::c_int;
        *(*__tekai_state_generated_pdftexextra)
            .buffer
            .offset(k_0 as isize) = 0 as ASCIIcode;
    }
    (*__tekai_state_generated_pdftexextra).last = (*__tekai_state_generated_pdftexextra).first;
    while *(*__tekai_state_generated_pdftexextra)
        .buffer
        .offset((*__tekai_state_generated_pdftexextra).last as isize)
        != 0
    {
        (*__tekai_state_generated_pdftexextra).last += 1;
    }
    (*__tekai_state_generated_pdftexextra).last -= 1;
    while (*__tekai_state_generated_pdftexextra).last
        >= (*__tekai_state_generated_pdftexextra).first
        && (*(*__tekai_state_generated_pdftexextra)
            .buffer
            .offset((*__tekai_state_generated_pdftexextra).last as isize)
            as ::core::ffi::c_int
            == ' ' as i32
            || *(*__tekai_state_generated_pdftexextra)
                .buffer
                .offset((*__tekai_state_generated_pdftexextra).last as isize)
                as ::core::ffi::c_int
                == '\r' as i32
            || *(*__tekai_state_generated_pdftexextra)
                .buffer
                .offset((*__tekai_state_generated_pdftexextra).last as isize)
                as ::core::ffi::c_int
                == '\n' as i32)
    {
        (*__tekai_state_generated_pdftexextra).last -= 1;
    }
    (*__tekai_state_generated_pdftexextra).last += 1;
    i = (*__tekai_state_generated_pdftexextra).first as ::core::ffi::c_int;
    while i < (*__tekai_state_generated_pdftexextra).last {
        *(*__tekai_state_generated_pdftexextra)
            .buffer
            .offset(i as isize) = (*__tekai_state_generated_pdftexextra).xord
            [*(*__tekai_state_generated_pdftexextra)
                .buffer
                .offset(i as isize) as usize];
        i += 1;
    }
}
pub const SOCK_STREAM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AF_UNIX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IPC_AF: ::core::ffi::c_int = AF_UNIX;
pub const IPC_PIPE_NAME: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"/.TeXview_Pipe\0") };
pub const IPC_SERVER_CMD: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"open `which TeXview`\0")
};
unsafe fn ipc_is_open() -> ::core::ffi::c_int {
    i32::from(crate::ipc::is_open())
}
unsafe fn ipc_open_out() {
    crate::ipc::open();
}
pub unsafe fn ipcpage(is_eof: ::core::ffi::c_int) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    if crate::ipc::wants_name() {
        use std::os::unix::ffi::OsStrExt;
        let name = gettexstring((*__tekai_state_generated_pdftexextra).outputfilename);
        let path =
            std::path::Path::new(std::ffi::OsStr::from_bytes(CStr::from_ptr(name).to_bytes()));
        let path = crate::working_directory::resolve(path).ok();
        crate::ipc::page(path.as_deref(), is_eof);
        free(name.cast());
    } else {
        crate::ipc::page(None, is_eof);
    }
}
unsafe fn tcx_get_num(
    mut upb: ::core::ffi::c_int,
    mut line_count: ::core::ffi::c_uint,
    mut start: string,
    mut post: *mut string,
) -> ::core::ffi::c_int {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let parsed = signed_prefix(CStr::from_ptr(start).to_bytes(), IntegerBase::Auto);
    if !post.is_null() {
        *post = start.add(parsed.consumed);
    }
    let mut num: ::core::ffi::c_int = parsed.value as ::core::ffi::c_int;
    if !(!post.is_null() && !(*post).is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"tcx_get_num\0" as *const u8 as *const ::core::ffi::c_char,
            b"texmfmp.c\0" as *const u8 as *const ::core::ffi::c_char,
            1570 as ::core::ffi::c_int,
            b"post && *post\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if *post == start {
        let mut p: string = start;
        while *p as ::core::ffi::c_int != 0
            && (*p as ::core::ffi::c_uint <= 127 as ::core::ffi::c_uint
                && crate::integer_parse::is_c_whitespace(*p as u8))
        {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            crate::stdio::fprintf_args(
                crate::stdio::stderr(),
                b"%s:%d: Expected numeric constant, not `%s'.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(
                        (*__tekai_state_generated_pdftexextra).translate_filename,
                    ),
                    crate::utils::PrintfArg::from(line_count),
                    crate::utils::PrintfArg::from(start),
                ],
            );
        }
        num = -(1 as ::core::ffi::c_int);
    } else if num < 0 as ::core::ffi::c_int || num > upb {
        crate::stdio::fprintf_args(
            crate::stdio::stderr(),
            b"%s:%d: Destination charcode %d <0 or >%d.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(
                    (*__tekai_state_generated_pdftexextra).translate_filename,
                ),
                crate::utils::PrintfArg::from(line_count),
                crate::utils::PrintfArg::from(num),
                crate::utils::PrintfArg::from(upb),
            ],
        );
        num = -(1 as ::core::ffi::c_int);
    }
    return num;
}
pub unsafe fn readtcxfile() {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut orig_filename: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if find_suffix((*__tekai_state_generated_pdftexextra).translate_filename as const_string)
        .is_null()
    {
        (*__tekai_state_generated_pdftexextra).translate_filename = concat(
            (*__tekai_state_generated_pdftexextra).translate_filename as const_string,
            b".tcx\0" as *const u8 as const_string,
        );
    }
    orig_filename = (*__tekai_state_generated_pdftexextra).translate_filename;
    (*__tekai_state_generated_pdftexextra).translate_filename = kpse_find_file(
        (*__tekai_state_generated_pdftexextra).translate_filename as const_string,
        kpse_web2c_format,
        true_0,
    );
    if !(*__tekai_state_generated_pdftexextra)
        .translate_filename
        .is_null()
    {
        let mut line_0: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut line_count: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut translate_file: *mut FILE = xfopen(
            (*__tekai_state_generated_pdftexextra).translate_filename as const_string,
            FOPEN_R_MODE.as_ptr(),
        );
        loop {
            line_0 = read_line(translate_file);
            if line_0.is_null() {
                break;
            }
            let mut first_0: ::core::ffi::c_int = 0;
            let mut start2: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut comment_loc: string =
                strchr(line_0 as *const ::core::ffi::c_char, '%' as i32) as string;
            if !comment_loc.is_null() {
                *comment_loc = 0 as ::core::ffi::c_char;
            }
            line_count = line_count.wrapping_add(1);
            first_0 = tcx_get_num(
                255 as ::core::ffi::c_int,
                line_count,
                line_0,
                &raw mut start2,
            );
            if first_0 >= 0 as ::core::ffi::c_int {
                let mut start3: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut second: ::core::ffi::c_int = 0;
                let mut printable: ::core::ffi::c_int = 0;
                second = tcx_get_num(
                    255 as ::core::ffi::c_int,
                    line_count,
                    start2,
                    &raw mut start3,
                );
                if second >= 0 as ::core::ffi::c_int {
                    let mut extra: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    (*__tekai_state_generated_pdftexextra).xord[first_0 as usize] =
                        second as ASCIIcode;
                    (*__tekai_state_generated_pdftexextra).xchr[second as usize] =
                        first_0 as ASCIIcode;
                    printable =
                        tcx_get_num(1 as ::core::ffi::c_int, line_count, start3, &raw mut extra);
                    if printable == -(1 as ::core::ffi::c_int) {
                        printable = 1 as ::core::ffi::c_int;
                    }
                    if 32 as ::core::ffi::c_int <= second && second <= 126 as ::core::ffi::c_int {
                        printable = 1 as ::core::ffi::c_int;
                    }
                } else {
                    second = first_0;
                    printable = 1 as ::core::ffi::c_int;
                }
                (*__tekai_state_generated_pdftexextra).xprn[second as usize] =
                    printable as ASCIIcode;
            }
            free(line_0 as *mut ::core::ffi::c_void);
        }
        xfclose(
            translate_file,
            (*__tekai_state_generated_pdftexextra).translate_filename as const_string,
        );
    } else {
        fputs(
            b"warning: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
            crate::stdio::stderr(),
        );
        crate::stdio::fprintf_args(
            crate::stdio::stderr(),
            b"Could not open char translation file `%s'\0" as *const u8
                as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(orig_filename)],
        );
        fputs(
            b".\n\0" as *const u8 as *const ::core::ffi::c_char,
            crate::stdio::stderr(),
        );
        fflush(crate::stdio::stderr());
    };
}
unsafe fn normalize_quotes(mut name: const_string, mut mesg: const_string) -> string {
    let mut quoted: boolean = false_0;
    let mut must_quote: boolean = (strchr(name as *const ::core::ffi::c_char, ' ' as i32)
        != NULL as *mut ::core::ffi::c_char)
        as ::core::ffi::c_int;
    let mut ret: string =
        xmalloc(strlen(name as *const ::core::ffi::c_char).wrapping_add(3 as size_t)) as string;
    let mut p: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: const_string = ::core::ptr::null::<::core::ffi::c_char>();
    p = ret;
    if must_quote != 0 {
        let fresh25 = p;
        p = p.offset(1);
        *fresh25 = '"' as i32 as ::core::ffi::c_char;
    }
    q = name;
    while *q != 0 {
        if *q as ::core::ffi::c_int == '"' as i32 {
            quoted = (quoted == 0) as ::core::ffi::c_int as boolean;
        } else {
            let fresh26 = p;
            p = p.offset(1);
            *fresh26 = *q;
        }
        q = q.offset(1);
    }
    if must_quote != 0 {
        let fresh27 = p;
        p = p.offset(1);
        *fresh27 = '"' as i32 as ::core::ffi::c_char;
    }
    *p = '\0' as i32 as ::core::ffi::c_char;
    if quoted != 0 {
        crate::stdio::fprintf_args(
            crate::stdio::stderr(),
            b"! Unbalanced quotes in %s %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(mesg),
                crate::utils::PrintfArg::from(name),
            ],
        );
        uexit(1 as ::core::ffi::c_int);
    }
    return ret;
}
unsafe fn get_input_file_name() -> string {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut input_file_name: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*(*__tekai_state_generated_pdftexextra)
        .argv
        .offset((*__tekai_state_generated_pdftexextra).optind as isize))
    .is_null()
        && *(*(*__tekai_state_generated_pdftexextra)
            .argv
            .offset((*__tekai_state_generated_pdftexextra).optind as isize))
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '&' as i32
        && *(*(*__tekai_state_generated_pdftexextra)
            .argv
            .offset((*__tekai_state_generated_pdftexextra).optind as isize))
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '\\' as i32
    {
        let mut name: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut quoted: boolean = 0;
        name = normalize_quotes(
            *(*__tekai_state_generated_pdftexextra)
                .argv
                .offset((*__tekai_state_generated_pdftexextra).optind as isize)
                as const_string,
            b"argument\0" as *const u8 as const_string,
        );
        quoted = (*name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '"' as i32) as ::core::ffi::c_int as boolean;
        if quoted != 0 {
            *name.offset(
                strlen(name as *const ::core::ffi::c_char).wrapping_sub(1 as size_t) as isize,
            ) = '\0' as i32 as ::core::ffi::c_char;
            name = name.offset(1);
        }
        input_file_name = kpse_find_file(name as const_string, kpse_tex_format, false_0);
        if quoted != 0 {
            *name.offset(strlen(name as *const ::core::ffi::c_char) as isize) =
                '"' as i32 as ::core::ffi::c_char;
            name = name.offset(-1);
        }
        let ref mut fresh24 = *(*__tekai_state_generated_pdftexextra)
            .argv
            .offset((*__tekai_state_generated_pdftexextra).optind as isize);
        *fresh24 = name as *mut ::core::ffi::c_char;
    }
    return input_file_name;
}
unsafe fn parse_options(argc_0: ::core::ffi::c_int, argv_0: *mut string) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_kpathsea: *mut crate::kpathsea::State = unsafe { crate::kpathsea::state() };
    let __tekai_state_openclose: *mut crate::openclose::State =
        unsafe { crate::openclose::state() };

    use crate::engine_options::{EngineOption, Parser};
    use std::os::unix::ffi::OsStrExt;

    // The invocation owner retains the NUL-terminated backing buffers. The
    // Rust parser borrows their bytes and never reorders or mutates argv.
    let arguments = (0..argc_0)
        .map(|index| CStr::from_ptr(*argv_0.add(index as usize)).to_bytes())
        .collect::<Vec<_>>();
    let mut parser = Parser::new(&arguments);
    let process_program = std::env::args_os()
        .next()
        .unwrap_or_else(|| "tekai-engine".into());
    (*__tekai_state_generated_pdftexextra).optind = parser.next_index() as ::core::ffi::c_int;
    loop {
        let result = parser.next_option();
        (*__tekai_state_generated_pdftexextra).optind = parser.next_index() as ::core::ffi::c_int;
        let parsed = match result {
            Ok(Some(parsed)) => parsed,
            Ok(None) => break,
            Err(error) => {
                // As before, option errors print a diagnostic and parsing
                // continues. They do not force the engine's exit status.
                let _ = std::io::Write::write_all(
                    &mut std::io::stderr().lock(),
                    &error.diagnostic(process_program.as_os_str().as_bytes()),
                );
                continue;
            }
        };
        // Only this generated caller boundary needs a legacy pointer. Every
        // accepted argument is a suffix of an invocation-owned C string, so
        // its original NUL terminator and address remain valid after return.
        let optarg: *mut ::core::ffi::c_char =
            parsed.argument.map_or(::core::ptr::null_mut(), |argument| {
                argument.as_ptr().cast_mut().cast()
            });
        match parsed.option {
            EngineOption::Ini => (*__tekai_state_generated_pdftexextra).iniversion = true_0,
            EngineOption::HaltOnError => {
                (*__tekai_state_generated_pdftexextra).haltonerrorp = true_0
            }
            EngineOption::Recorder => (*__tekai_state_openclose).recorder_enabled = true_0,
            EngineOption::Ipc => (*__tekai_state_generated_pdftexextra).ipcon = 1,
            EngineOption::Mltex => (*__tekai_state_generated_pdftexextra).mltexp = true_0,
            EngineOption::Enc => (*__tekai_state_generated_pdftexextra).enctexp = true_0,
            EngineOption::Etex => (*__tekai_state_generated_pdftexextra).etexp = true_0,
            EngineOption::ShellEscape | EngineOption::EnableWrite18 => {
                (*__tekai_state_generated_pdftexextra).shellenabledp = 1
            }
            EngineOption::NoShellEscape | EngineOption::DisableWrite18 => {
                (*__tekai_state_generated_pdftexextra).shellenabledp = -1
            }
            EngineOption::DebugFormat => {
                (*__tekai_state_generated_pdftexextra).debugformatfile = true_0
            }
            EngineOption::EightBit => (*__tekai_state_generated_pdftexextra).eightbitp = true_0,
            EngineOption::FileLineErrorStyle | EngineOption::FileLineError => {
                (*__tekai_state_generated_pdftexextra).filelineerrorstylep = 1
            }
            EngineOption::NoFileLineErrorStyle | EngineOption::NoFileLineError => {
                (*__tekai_state_generated_pdftexextra).filelineerrorstylep = -1
            }
            EngineOption::ParseFirstLine => {
                (*__tekai_state_generated_pdftexextra).parsefirstlinep = 1
            }
            EngineOption::NoParseFirstLine => {
                (*__tekai_state_generated_pdftexextra).parsefirstlinep = -1
            }
            EngineOption::KpathseaDebug => {
                (*__tekai_state_kpathsea).kpse_def_inst.debug |=
                    decimal_i32_legacy(parsed.argument.unwrap()).value as ::core::ffi::c_uint;
            }
            EngineOption::Progname => {
                (*__tekai_state_generated_pdftexextra).user_progname = optarg as const_string
            }
            EngineOption::CnfLine => {
                if (*__tekai_state_generated_pdftexextra)
                    .user_cnf_lines
                    .is_null()
                {
                    (*__tekai_state_generated_pdftexextra).user_cnf_nlines = 1;
                    (*__tekai_state_generated_pdftexextra).user_cnf_lines =
                        xmalloc(::core::mem::size_of::<const_string>() as size_t) as *mut string;
                } else {
                    (*__tekai_state_generated_pdftexextra).user_cnf_nlines =
                        (*__tekai_state_generated_pdftexextra)
                            .user_cnf_nlines
                            .wrapping_add(1);
                    (*__tekai_state_generated_pdftexextra).user_cnf_lines = xrealloc(
                        (*__tekai_state_generated_pdftexextra).user_cnf_lines as address,
                        ((*__tekai_state_generated_pdftexextra).user_cnf_nlines as size_t)
                            .wrapping_mul(::core::mem::size_of::<const_string>() as size_t),
                    )
                        as *mut string;
                }
                *(*__tekai_state_generated_pdftexextra)
                    .user_cnf_lines
                    .offset(
                        (*__tekai_state_generated_pdftexextra)
                            .user_cnf_nlines
                            .wrapping_sub(1) as isize,
                    ) = xstrdup(optarg as const_string);
            }
            EngineOption::Jobname => {
                (*__tekai_state_generated_pdftexextra).c_job_name = normalize_quotes(
                    optarg as const_string,
                    b"jobname\0" as *const u8 as const_string,
                )
                    as const_string;
            }
            EngineOption::Fmt | EngineOption::Efmt => {
                (*__tekai_state_generated_pdftexextra).dump_name = optarg as const_string;
                (*__tekai_state_generated_pdftexextra).dumpoption = true_0;
            }
            EngineOption::OutputDirectory => {
                (*__tekai_state_openclose).output_directory = optarg as string
            }
            EngineOption::OutputComment => {
                let len = parsed.argument.unwrap().len();
                if len < 256 {
                    (*__tekai_state_generated_pdftexextra).outputcomment = optarg;
                } else {
                    fputs(
                        b"warning: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
                        crate::stdio::stderr(),
                    );
                    crate::stdio::fprintf_args(
                        crate::stdio::stderr(),
                        b"Comment truncated to 255 characters from %d. (%s)\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[
                            crate::utils::PrintfArg::from(len as ::core::ffi::c_uint),
                            crate::utils::PrintfArg::from(optarg),
                        ],
                    );
                    fputs(
                        b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                        crate::stdio::stderr(),
                    );
                    fflush(crate::stdio::stderr());
                    (*__tekai_state_generated_pdftexextra).outputcomment =
                        xmalloc(256 as size_t) as *mut ::core::ffi::c_char;
                    strncpy(
                        (*__tekai_state_generated_pdftexextra).outputcomment,
                        optarg,
                        255 as size_t,
                    );
                    *(*__tekai_state_generated_pdftexextra)
                        .outputcomment
                        .add(255) = 0;
                }
            }
            EngineOption::IpcStart => {
                (*__tekai_state_generated_pdftexextra).ipcon = 2;
                // OS interaction stays in the caller, outside option parsing.
                ipc_open_out();
                if ipc_is_open() == 0 && system(IPC_SERVER_CMD.as_ptr()) == 0 {
                    let mut i = 0;
                    while i < 20 && ipc_is_open() == 0 {
                        std::thread::sleep(std::time::Duration::from_secs(2));
                        ipc_open_out();
                        i += 1;
                    }
                }
            }
            EngineOption::ShellRestricted => {
                (*__tekai_state_generated_pdftexextra).shellenabledp = 1;
                (*__tekai_state_generated_pdftexextra).restrictedshell = 1;
            }
            EngineOption::SrcSpecials => {
                (*__tekai_state_generated_pdftexextra).last_source_name =
                    xstrdup(b"\0" as *const u8 as const_string) as *mut ::core::ffi::c_char;
                if parsed.argument.is_none() {
                    (*__tekai_state_generated_pdftexextra).insertsrcspecialeverypar = true_0;
                    (*__tekai_state_generated_pdftexextra).insertsrcspecialauto = true_0;
                    (*__tekai_state_generated_pdftexextra).srcspecialsoption = true_0;
                    (*__tekai_state_generated_pdftexextra).srcspecialsp = true_0;
                } else {
                    parse_src_specials_option(optarg as const_string);
                }
            }
            EngineOption::OutputFormat => {
                (*__tekai_state_generated_pdftexextra).pdfoutputoption = 1;
                match parsed.argument.unwrap() {
                    b"dvi" => (*__tekai_state_generated_pdftexextra).pdfoutputvalue = 0,
                    b"pdf" => (*__tekai_state_generated_pdftexextra).pdfoutputvalue = 2,
                    _ => {
                        fputs(
                            b"warning: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
                            crate::stdio::stderr(),
                        );
                        crate::stdio::fprintf_args(
                            crate::stdio::stderr(),
                            b"Ignoring unknown value `%s' for --output-format\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[crate::utils::PrintfArg::from(optarg)],
                        );
                        fputs(
                            b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                            crate::stdio::stderr(),
                        );
                        fflush(crate::stdio::stderr());
                        (*__tekai_state_generated_pdftexextra).pdfoutputoption = 0;
                    }
                }
            }
            EngineOption::Draftmode => {
                (*__tekai_state_generated_pdftexextra).pdfdraftmodeoption = 1;
                (*__tekai_state_generated_pdftexextra).pdfdraftmodevalue = 1;
            }
            EngineOption::TranslateFile => {
                (*__tekai_state_generated_pdftexextra).translate_filename = optarg as string
            }
            EngineOption::DefaultTranslateFile => {
                (*__tekai_state_generated_pdftexextra).default_translate_filename = optarg as string
            }
            EngineOption::Mktex => kpse_maketex_option(optarg as const_string, true_0),
            EngineOption::NoMktex => kpse_maketex_option(optarg as const_string, false_0),
            EngineOption::Interaction => match parsed.argument.unwrap() {
                b"batchmode" => (*__tekai_state_generated_pdftexextra).interactionoption = 0,
                b"nonstopmode" => (*__tekai_state_generated_pdftexextra).interactionoption = 1,
                b"scrollmode" => (*__tekai_state_generated_pdftexextra).interactionoption = 2,
                b"errorstopmode" => (*__tekai_state_generated_pdftexextra).interactionoption = 3,
                _ => {
                    fputs(
                        b"warning: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
                        crate::stdio::stderr(),
                    );
                    crate::stdio::fprintf_args(
                        crate::stdio::stderr(),
                        b"Ignoring unknown argument `%s' to --interaction\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[crate::utils::PrintfArg::from(optarg)],
                    );
                    fputs(
                        b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                        crate::stdio::stderr(),
                    );
                    fflush(crate::stdio::stderr());
                }
            },
            EngineOption::Synctex => {
                (*__tekai_state_generated_pdftexextra).synctexoption =
                    signed_prefix(parsed.argument.unwrap(), IntegerBase::Auto).value as integer;
            }
            EngineOption::Help => usagehelp(
                &raw mut (*__tekai_state_generated_pdftexextra).PDFTEXHELP as *mut const_string,
                BUG_ADDRESS.as_ptr(),
            ),
            EngineOption::Version => {
                let mut versions: *mut ::core::ffi::c_char = ::core::ptr::null_mut();
                initversionstring(&raw mut versions);
                printversionandexit(
                    b"This is pdfTeX, Version 3.141592653-2.6-1.40.29\0" as *const u8
                        as const_string,
                    COPYRIGHT_HOLDER.as_ptr(),
                    ::core::ptr::null(),
                    versions as const_string,
                );
            }
        }
    }
    if !(*__tekai_state_openclose).output_directory.is_null() {
        xputenv(
            b"TEXMF_OUTPUT_DIRECTORY\0" as *const u8 as const_string,
            (*__tekai_state_openclose).output_directory as const_string,
        );
    } else if !getenv(b"TEXMF_OUTPUT_DIRECTORY\0" as *const u8 as *const ::core::ffi::c_char)
        .is_null()
    {
        (*__tekai_state_openclose).output_directory =
            getenv(b"TEXMF_OUTPUT_DIRECTORY\0" as *const u8 as *const ::core::ffi::c_char)
                as string;
    }
}
unsafe fn parse_src_specials_option(opt_list: const_string) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let bytes = CStr::from_ptr(opt_list).to_bytes();
    (*__tekai_state_generated_pdftexextra).insertsrcspecialauto = false_0;
    for token in crate::byte_string::Tokenizer::new(bytes, b", ") {
        match token {
            b"everypar" | b"par" | b"auto" => {
                (*__tekai_state_generated_pdftexextra).insertsrcspecialauto = true_0;
                (*__tekai_state_generated_pdftexextra).insertsrcspecialeverypar = true_0;
            }
            b"everyparend" | b"parend" => {
                (*__tekai_state_generated_pdftexextra).insertsrcspecialeveryparend = true_0
            }
            b"everycr" | b"cr" => {
                (*__tekai_state_generated_pdftexextra).insertsrcspecialeverycr = true_0
            }
            b"everymath" | b"math" => {
                (*__tekai_state_generated_pdftexextra).insertsrcspecialeverymath = true_0
            }
            b"everyhbox" | b"hbox" => {
                (*__tekai_state_generated_pdftexextra).insertsrcspecialeveryhbox = true_0
            }
            b"everyvbox" | b"vbox" => {
                (*__tekai_state_generated_pdftexextra).insertsrcspecialeveryvbox = true_0
            }
            b"everydisplay" | b"display" => {
                (*__tekai_state_generated_pdftexextra).insertsrcspecialeverydisplay = true_0
            }
            b"none" => {
                (*__tekai_state_generated_pdftexextra).insertsrcspecialeverydisplay = false_0;
                (*__tekai_state_generated_pdftexextra).insertsrcspecialeveryvbox = false_0;
                (*__tekai_state_generated_pdftexextra).insertsrcspecialeveryhbox = false_0;
                (*__tekai_state_generated_pdftexextra).insertsrcspecialeverymath = false_0;
                (*__tekai_state_generated_pdftexextra).insertsrcspecialeverycr = false_0;
                (*__tekai_state_generated_pdftexextra).insertsrcspecialeveryparend = false_0;
                (*__tekai_state_generated_pdftexextra).insertsrcspecialeverypar = false_0;
                (*__tekai_state_generated_pdftexextra).insertsrcspecialauto = false_0;
            }
            _ => {
                // The tokenizer stops at NUL. Only this synchronous legacy
                // diagnostic needs a temporary terminated copy of the token.
                let token = std::ffi::CString::new(token).expect("token excludes NUL");
                fputs(
                    b"warning: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
                    crate::stdio::stderr(),
                );
                crate::stdio::fprintf_args(
                    crate::stdio::stderr(),
                    b"Ignoring unknown argument `%s' to --src-specials\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(token.as_ptr())],
                );
                fputs(
                    b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                    crate::stdio::stderr(),
                );
                fflush(crate::stdio::stderr());
            }
        }
    }
    (*__tekai_state_generated_pdftexextra).srcspecialsp = (*__tekai_state_generated_pdftexextra)
        .insertsrcspecialauto
        | (*__tekai_state_generated_pdftexextra).insertsrcspecialeverypar
        | (*__tekai_state_generated_pdftexextra).insertsrcspecialeveryparend
        | (*__tekai_state_generated_pdftexextra).insertsrcspecialeverycr
        | (*__tekai_state_generated_pdftexextra).insertsrcspecialeverymath
        | (*__tekai_state_generated_pdftexextra).insertsrcspecialeveryhbox
        | (*__tekai_state_generated_pdftexextra).insertsrcspecialeveryvbox
        | (*__tekai_state_generated_pdftexextra).insertsrcspecialeverydisplay;
    (*__tekai_state_generated_pdftexextra).srcspecialsoption = true_0;
}
unsafe fn parse_first_line(mut filename: const_string) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut f_0: *mut FILE = if !filename.is_null() {
        fopen(
            filename as *const ::core::ffi::c_char,
            FOPEN_R_MODE.as_ptr(),
        ) as *mut FILE
    } else {
        ::core::ptr::null_mut::<FILE>()
    };
    if !f_0.is_null() {
        let mut first_line: string = read_line(f_0);
        xfclose(f_0, filename);
        if !first_line.is_null()
            && *first_line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '%' as i32
            && *first_line.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '&' as i32
        {
            let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut part: [*mut ::core::ffi::c_char; 4] =
                [::core::ptr::null_mut::<::core::ffi::c_char>(); 4];
            let mut npart: ::core::ffi::c_int = 0;
            let mut parse: *mut *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
            s = first_line.offset(2 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char;
            while *s as ::core::ffi::c_uint <= 127 as ::core::ffi::c_uint
                && (*s as ::core::ffi::c_uchar as ::core::ffi::c_int == ' ' as i32
                    || *s as ::core::ffi::c_uchar as ::core::ffi::c_int == '\t' as i32)
            {
                s = s.offset(1);
            }
            npart = 0 as ::core::ffi::c_int;
            while *s as ::core::ffi::c_int != 0 && npart != 3 as ::core::ffi::c_int {
                let fresh22 = npart;
                npart = npart + 1;
                part[fresh22 as usize] = s;
                while *s as ::core::ffi::c_int != 0 && *s as ::core::ffi::c_int != ' ' as i32 {
                    s = s.offset(1);
                }
                while *s as ::core::ffi::c_int == ' ' as i32 {
                    let fresh23 = s;
                    s = s.offset(1);
                    *fresh23 = '\0' as i32 as ::core::ffi::c_char;
                }
            }
            part[npart as usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
            parse = &raw mut part as *mut *mut ::core::ffi::c_char;
            if !(*parse).is_null() && **parse as ::core::ffi::c_int != '-' as i32 {
                if (*__tekai_state_generated_pdftexextra).dump_name.is_null() {
                    let mut f_name: string = concat(
                        part[0 as ::core::ffi::c_int as usize] as const_string,
                        DUMP_EXT.as_ptr(),
                    );
                    let mut d_name: string =
                        kpse_find_file(f_name as const_string, kpse_fmt_format, false_0);
                    if !d_name.is_null() && !kpse_readable_file(d_name).is_null() {
                        (*__tekai_state_generated_pdftexextra).dump_name =
                            xstrdup(part[0 as ::core::ffi::c_int as usize] as const_string)
                                as const_string;
                        kpse_reset_program_name((*__tekai_state_generated_pdftexextra).dump_name);
                        (*__tekai_state_generated_pdftexextra).dumpline = true_0 as boolean;
                    }
                    free(f_name as *mut ::core::ffi::c_void);
                }
                parse = parse.offset(1);
            }
            if !(*parse).is_null() {
                s = ::core::ptr::null_mut::<::core::ffi::c_char>();
                if (*__tekai_state_generated_pdftexextra)
                    .translate_filename
                    .is_null()
                {
                    if !(*parse).is_null()
                        && !(b"--translate-file\0" as *const u8 as *const ::core::ffi::c_char)
                            .is_null()
                        && strcmp(
                            *parse,
                            b"--translate-file\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        s = *parse.offset(1 as ::core::ffi::c_int as isize);
                    } else if !(*parse).is_null()
                        && !(b"-translate-file\0" as *const u8 as *const ::core::ffi::c_char)
                            .is_null()
                        && strcmp(
                            *parse,
                            b"-translate-file\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        s = *parse.offset(1 as ::core::ffi::c_int as isize);
                    } else if !(*parse).is_null()
                        && !(b"--translate-file=\0" as *const u8 as *const ::core::ffi::c_char)
                            .is_null()
                        && strncmp(
                            *parse,
                            b"--translate-file=\0" as *const u8 as *const ::core::ffi::c_char,
                            17 as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        s = (*parse).offset(17 as ::core::ffi::c_int as isize);
                    } else if !(*parse).is_null()
                        && !(b"-translate-file=\0" as *const u8 as *const ::core::ffi::c_char)
                            .is_null()
                        && strncmp(
                            *parse,
                            b"-translate-file=\0" as *const u8 as *const ::core::ffi::c_char,
                            16 as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        s = (*parse).offset(16 as ::core::ffi::c_int as isize);
                    }
                }
                if !s.is_null() && *s as ::core::ffi::c_int != 0 {
                    (*__tekai_state_generated_pdftexextra).translate_filename =
                        xstrdup(s as const_string);
                }
            }
        }
        if !first_line.is_null() {
            free(first_line as *mut ::core::ffi::c_void);
        }
    }
}
pub const NUM_PIPES: ::core::ffi::c_int = 16 as ::core::ffi::c_int;

pub(crate) unsafe fn open_in_or_pipe(
    mut f_ptr: *mut *mut FILE,
    mut filefmt: ::core::ffi::c_int,
    mut fopen_mode: const_string,
) -> boolean {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_openclose: *mut crate::openclose::State =
        unsafe { crate::openclose::state() };

    let mut fname: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    if (*__tekai_state_generated_pdftexextra).shellenabledp != 0
        && *(*__tekai_state_generated_pdftexextra)
            .nameoffile
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '|' as i32
    {
        *f_ptr = ::core::ptr::null_mut::<FILE>();
        fname = xmalloc(
            strlen(
                (*__tekai_state_generated_pdftexextra)
                    .nameoffile
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char,
            )
            .wrapping_add(1 as size_t),
        ) as string;
        strcpy(
            fname as *mut ::core::ffi::c_char,
            (*__tekai_state_generated_pdftexextra)
                .nameoffile
                .offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
        );
        if !(*__tekai_state_openclose).fullnameoffile.is_null() {
            free((*__tekai_state_openclose).fullnameoffile as *mut ::core::ffi::c_void);
        }
        (*__tekai_state_openclose).fullnameoffile = xstrdup(fname as const_string);
        recorder_record_input(fname.offset(1 as ::core::ffi::c_int as isize) as const_string);
        *f_ptr = runpopen(
            fname.offset(1 as ::core::ffi::c_int as isize),
            b"r\0" as *const u8 as *const ::core::ffi::c_char,
        );
        free(fname as *mut ::core::ffi::c_void);
        i = 0 as ::core::ffi::c_int;
        while i < NUM_PIPES {
            if (*__tekai_state_generated_pdftexextra).pipes[i as usize].is_null() {
                (*__tekai_state_generated_pdftexextra).pipes[i as usize] = *f_ptr;
                break;
            } else {
                i += 1;
            }
        }
        if !(*f_ptr).is_null() {
            setvbuf(
                *f_ptr,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                _IONBF,
                0 as size_t,
            );
        }
        return (*f_ptr != NULL as *mut FILE) as ::core::ffi::c_int;
    }
    return open_input(f_ptr, filefmt, fopen_mode);
}
pub(crate) unsafe fn open_out_or_pipe(
    mut f_ptr: *mut *mut FILE,
    mut fopen_mode: const_string,
) -> boolean {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut fname: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    if (*__tekai_state_generated_pdftexextra).shellenabledp != 0
        && *(*__tekai_state_generated_pdftexextra)
            .nameoffile
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '|' as i32
    {
        fname = xmalloc(
            strlen(
                (*__tekai_state_generated_pdftexextra)
                    .nameoffile
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char,
            )
            .wrapping_add(1 as size_t),
        ) as string;
        strcpy(
            fname as *mut ::core::ffi::c_char,
            (*__tekai_state_generated_pdftexextra)
                .nameoffile
                .offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
        );
        if strchr(fname as *const ::core::ffi::c_char, ' ' as i32).is_null()
            && strchr(fname as *const ::core::ffi::c_char, '>' as i32).is_null()
        {
            if !fname
                .offset(strlen(fname as *const ::core::ffi::c_char) as isize)
                .offset(-(4 as ::core::ffi::c_int as isize))
                .is_null()
                && !(b".tex\0" as *const u8 as *const ::core::ffi::c_char).is_null()
                && strcmp(
                    fname
                        .offset(strlen(fname as *const ::core::ffi::c_char) as isize)
                        .offset(-(4 as ::core::ffi::c_int as isize))
                        as *const ::core::ffi::c_char,
                    b".tex\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                *fname
                    .offset(strlen(fname as *const ::core::ffi::c_char) as isize)
                    .offset(-(4 as ::core::ffi::c_int as isize)) = 0 as ::core::ffi::c_char;
            }
            *f_ptr = runpopen(
                fname.offset(1 as ::core::ffi::c_int as isize),
                b"w\0" as *const u8 as *const ::core::ffi::c_char,
            );
            *fname.offset(strlen(fname as *const ::core::ffi::c_char) as isize) =
                '.' as i32 as ::core::ffi::c_char;
        } else {
            *f_ptr = runpopen(
                fname.offset(1 as ::core::ffi::c_int as isize),
                b"w\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        recorder_record_output(fname.offset(1 as ::core::ffi::c_int as isize) as const_string);
        free(fname as *mut ::core::ffi::c_void);
        i = 0 as ::core::ffi::c_int;
        while i < NUM_PIPES {
            if (*__tekai_state_generated_pdftexextra).pipes[i as usize].is_null() {
                (*__tekai_state_generated_pdftexextra).pipes[i as usize] = *f_ptr;
                break;
            } else {
                i += 1;
            }
        }
        if !(*f_ptr).is_null() {
            setvbuf(
                *f_ptr,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                _IONBF,
                0 as size_t,
            );
        }
        return (*f_ptr != NULL as *mut FILE) as ::core::ffi::c_int;
    }
    return open_output(f_ptr, fopen_mode);
}
pub(crate) unsafe fn close_file_or_pipe(mut f_0: *mut FILE) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut i: ::core::ffi::c_int = 0;
    if (*__tekai_state_generated_pdftexextra).shellenabledp != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < NUM_PIPES {
            if (*__tekai_state_generated_pdftexextra).pipes[i as usize] == f_0 {
                if !f_0.is_null() {
                    pclose(f_0);
                    crate::lookup::reset();
                }
                (*__tekai_state_generated_pdftexextra).pipes[i as usize] =
                    ::core::ptr::null_mut::<FILE>();
                return;
            }
            i += 1;
        }
    }
    close_file(f_0);
}
unsafe fn time_result<T>(result: Result<T, crate::time::TimeError>) -> T {
    let __tekai_state_kpathsea: *mut crate::kpathsea::State = unsafe { crate::kpathsea::state() };

    match result {
        Ok(value) => value,
        Err(error) => {
            use std::io::Write;
            use std::os::unix::ffi::OsStrExt;
            crate::stdio::fprintf_args(
                crate::stdio::stderr(),
                c"%s: fatal: (kpathsea) ".as_ptr(),
                &[crate::utils::PrintfArg::from(
                    (*(*__tekai_state_kpathsea).kpse_def).invocation_name,
                )],
            );
            let mut stderr = std::io::stderr().lock();
            if let crate::time::TimeError::InvalidEpoch(value) = error {
                let _ = stderr.write_all(b"invalid epoch-seconds-timezone value for environment variable $SOURCE_DATE_EPOCH: ");
                let _ = stderr.write_all(value.as_bytes());
                let _ = stderr.write_all(b".\n");
            } else {
                let _ = writeln!(stderr, "{error}.");
            }
            exit(1);
        }
    }
}
pub unsafe fn init_start_time() {
    time_result(crate::time::initialize_start());
}
pub unsafe fn get_date_and_time(
    minutes: *mut integer,
    day: *mut integer,
    month: *mut integer,
    year: *mut integer,
) {
    let date = time_result(crate::time::tex_date());
    if let Some(value) = time_result(crate::time::invalid_force_source()) {
        use std::io::Write;
        use std::os::unix::ffi::OsStrExt;
        let mut stderr = std::io::stderr().lock();
        let _ = stderr.write_all(b"warning: (kpathsea) invalid value (expected 0 or 1) for environment variable $FORCE_SOURCE_DATE: ");
        let _ = stderr.write_all(value.as_bytes());
        let _ = stderr.write_all(b".\n");
        let _ = stderr.flush();
    }
    *minutes = date.hour * 60 + date.minute;
    *day = date.day;
    *month = date.month;
    *year = date.year;
}
pub unsafe fn get_seconds_and_micros(seconds: *mut integer, micros: *mut integer) {
    let (whole, fraction) = time_result(crate::time::seconds_and_micros());
    // Preserve the generated elapsed-time primitive's 32-bit second counter.
    *seconds = whole as integer;
    *micros = fraction;
}
pub(crate) unsafe fn input_line(mut f_0: *mut FILE) -> boolean {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut i: ::core::ffi::c_int = EOF;
    (*__tekai_state_generated_pdftexextra).last = (*__tekai_state_generated_pdftexextra).first;
    loop {
        crate::stdio::clear_errno();
        while (*__tekai_state_generated_pdftexextra).last
            < (*__tekai_state_generated_pdftexextra).bufsize
            && {
                i = fast_fgetc(f_0);
                i != EOF
            }
            && i != '\n' as i32
            && i != '\r' as i32
        {
            let fresh29 = (*__tekai_state_generated_pdftexextra).last;
            (*__tekai_state_generated_pdftexextra).last =
                (*__tekai_state_generated_pdftexextra).last + 1;
            *(*__tekai_state_generated_pdftexextra)
                .buffer
                .offset(fresh29 as isize) = i as ASCIIcode;
        }
        if !(i == EOF && crate::stdio::errno() == EINTR) {
            break;
        }
    }
    if i == EOF
        && (*__tekai_state_generated_pdftexextra).last
            == (*__tekai_state_generated_pdftexextra).first
    {
        return false_0;
    }
    if i != EOF && i != '\n' as i32 && i != '\r' as i32 {
        crate::stdio::fprintf_args(
            crate::stdio::stderr(),
            b"! Unable to read an entire line---bufsize=%u.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                (*__tekai_state_generated_pdftexextra).bufsize as ::core::ffi::c_uint,
            )],
        );
        fputs(
            b"Please increase buf_size in texmf.cnf.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            crate::stdio::stderr(),
        );
        uexit(1 as ::core::ffi::c_int);
    }
    *(*__tekai_state_generated_pdftexextra)
        .buffer
        .offset((*__tekai_state_generated_pdftexextra).last as isize) = ' ' as i32 as ASCIIcode;
    if (*__tekai_state_generated_pdftexextra).last
        >= (*__tekai_state_generated_pdftexextra).maxbufstack
    {
        (*__tekai_state_generated_pdftexextra).maxbufstack =
            (*__tekai_state_generated_pdftexextra).last;
    }
    if i == '\r' as i32 {
        loop {
            i = fast_fgetc(f_0);
            if !(i == EOF && crate::stdio::errno() == EINTR) {
                break;
            }
        }
        if i != '\n' as i32 {
            ungetc(i, f_0);
        }
    }
    while (*__tekai_state_generated_pdftexextra).last > (*__tekai_state_generated_pdftexextra).first
        && *(*__tekai_state_generated_pdftexextra).buffer.offset(
            ((*__tekai_state_generated_pdftexextra).last as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as isize,
        ) as ::core::ffi::c_int
            == ' ' as i32
    {
        (*__tekai_state_generated_pdftexextra).last -= 1;
    }
    i = (*__tekai_state_generated_pdftexextra).first as ::core::ffi::c_int;
    while i <= (*__tekai_state_generated_pdftexextra).last {
        *(*__tekai_state_generated_pdftexextra)
            .buffer
            .offset(i as isize) = (*__tekai_state_generated_pdftexextra).xord
            [*(*__tekai_state_generated_pdftexextra)
                .buffer
                .offset(i as isize) as usize];
        i += 1;
    }
    return true_0;
}

pub unsafe fn calledit(
    mut filename: *mut packedASCIIcode,
    mut fnstart: poolpointer,
    mut fnlength: integer,
    mut linenumber: integer,
) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_kpathsea: *mut crate::kpathsea::State = unsafe { crate::kpathsea::state() };

    let mut temp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut command: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fullcmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut c_0: ::core::ffi::c_char = 0;
    let mut sdone: ::core::ffi::c_int = 0;
    let mut ddone: ::core::ffi::c_int = 0;
    ddone = 0 as ::core::ffi::c_int;
    sdone = ddone;
    filename = filename.offset(fnstart as isize);
    let mut is_ptr: ::core::ffi::c_int = 0;
    is_ptr = 0 as ::core::ffi::c_int;
    while is_ptr < (*__tekai_state_generated_pdftexextra).inputptr {
        if !((*(*__tekai_state_generated_pdftexextra)
            .inputstack
            .offset(is_ptr as isize))
        .statefield as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || (*(*__tekai_state_generated_pdftexextra)
                .inputstack
                .offset(is_ptr as isize))
            .namefield
                <= 255 as ::core::ffi::c_int)
        {
            let mut f_0: *mut FILE = ::core::ptr::null_mut::<FILE>();
            let mut if_ptr: ::core::ffi::c_int = (*(*__tekai_state_generated_pdftexextra)
                .inputstack
                .offset(is_ptr as isize))
            .indexfield as ::core::ffi::c_int;
            if if_ptr < 1 as ::core::ffi::c_int
                || if_ptr > (*__tekai_state_generated_pdftexextra).inopen
            {
                crate::stdio::fprintf_args(
                    crate::stdio::stderr(),
                    b"%s:calledit: unexpected if_ptr=%d not in range 1..%d,\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(
                            *(*__tekai_state_generated_pdftexextra)
                                .argv
                                .offset(0 as ::core::ffi::c_int as isize),
                        ),
                        crate::utils::PrintfArg::from(if_ptr),
                        crate::utils::PrintfArg::from(
                            (*__tekai_state_generated_pdftexextra).inopen,
                        ),
                    ],
                );
                crate::stdio::fprintf_args(
                    crate::stdio::stderr(),
                    b"from input_stack[%d].namefield=%d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(is_ptr),
                        crate::utils::PrintfArg::from(
                            (*(*__tekai_state_generated_pdftexextra)
                                .inputstack
                                .offset(is_ptr as isize))
                            .namefield,
                        ),
                    ],
                );
                exit(1 as ::core::ffi::c_int);
            }
            f_0 = *(*__tekai_state_generated_pdftexextra)
                .inputfile
                .offset(if_ptr as isize) as *mut FILE;
            if !f_0.is_null() {
                xfclose(f_0, b"inputfile\0" as *const u8 as const_string);
            } else {
                crate::stdio::fprintf_args(
                    crate::stdio::stderr(),
                    b"%s:calledit: not closing unexpected zero\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(
                        *(*__tekai_state_generated_pdftexextra)
                            .argv
                            .offset(0 as ::core::ffi::c_int as isize),
                    )],
                );
                crate::stdio::fprintf_args(
                    crate::stdio::stderr(),
                    b" input_file[%d] from input_stack[%d].namefield=%d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(if_ptr),
                        crate::utils::PrintfArg::from(is_ptr),
                        crate::utils::PrintfArg::from(
                            (*(*__tekai_state_generated_pdftexextra)
                                .inputstack
                                .offset(is_ptr as isize))
                            .namefield,
                        ),
                    ],
                );
            }
        }
        is_ptr += 1;
    }
    temp = kpse_var_value(edit_var.as_ptr()) as *mut ::core::ffi::c_char;
    if !temp.is_null() {
        (*__tekai_state_generated_pdftexextra).edit_value = temp as const_string;
    }
    command = xmalloc(
        strlen((*__tekai_state_generated_pdftexextra).edit_value as *const ::core::ffi::c_char)
            .wrapping_add(fnlength as size_t)
            .wrapping_add(11 as size_t),
    ) as *mut ::core::ffi::c_char;
    temp = command;
    loop {
        let fresh37 = (*__tekai_state_generated_pdftexextra).edit_value;
        (*__tekai_state_generated_pdftexextra).edit_value =
            (*__tekai_state_generated_pdftexextra).edit_value.offset(1);
        c_0 = *fresh37;
        if !(c_0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            break;
        }
        if c_0 as ::core::ffi::c_int == '%' as i32 {
            let mut i: ::core::ffi::c_int = 0;
            let fresh38 = (*__tekai_state_generated_pdftexextra).edit_value;
            (*__tekai_state_generated_pdftexextra).edit_value =
                (*__tekai_state_generated_pdftexextra).edit_value.offset(1);
            c_0 = *fresh38;
            match c_0 as ::core::ffi::c_int {
                100 => {
                    if ddone != 0 {
                        crate::stdio::fprintf_args(
                            crate::stdio::stderr(),
                            b"%s: fatal: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
                            &[crate::utils::PrintfArg::from(
                                (*(*__tekai_state_kpathsea).kpse_def).invocation_name,
                            )],
                        );
                        fputs(
                            b"call_edit: `%%d' appears twice in editor command\0" as *const u8
                                as *const ::core::ffi::c_char,
                            crate::stdio::stderr(),
                        );
                        fputs(
                            b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                            crate::stdio::stderr(),
                        );
                        exit(1 as ::core::ffi::c_int);
                    }
                    crate::utils::sprintf_args(
                        temp,
                        b"%ld\0" as *const u8 as *const ::core::ffi::c_char,
                        &[crate::utils::PrintfArg::from(
                            linenumber as ::core::ffi::c_long,
                        )],
                    );
                    while *temp as ::core::ffi::c_int != '\0' as i32 {
                        temp = temp.offset(1);
                    }
                    ddone = 1 as ::core::ffi::c_int;
                }
                115 => {
                    if sdone != 0 {
                        crate::stdio::fprintf_args(
                            crate::stdio::stderr(),
                            b"%s: fatal: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
                            &[crate::utils::PrintfArg::from(
                                (*(*__tekai_state_kpathsea).kpse_def).invocation_name,
                            )],
                        );
                        fputs(
                            b"call_edit: `%%s' appears twice in editor command\0" as *const u8
                                as *const ::core::ffi::c_char,
                            crate::stdio::stderr(),
                        );
                        fputs(
                            b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                            crate::stdio::stderr(),
                        );
                        exit(1 as ::core::ffi::c_int);
                    }
                    i = 0 as ::core::ffi::c_int;
                    while i < fnlength {
                        let fresh39 = temp;
                        temp = temp.offset(1);
                        *fresh39 = (*__tekai_state_generated_pdftexextra).xchr
                            [*filename.offset(i as isize) as usize]
                            as ::core::ffi::c_char;
                        i += 1;
                    }
                    sdone = 1 as ::core::ffi::c_int;
                }
                0 => {
                    let fresh40 = temp;
                    temp = temp.offset(1);
                    *fresh40 = '%' as i32 as ::core::ffi::c_char;
                    (*__tekai_state_generated_pdftexextra).edit_value =
                        (*__tekai_state_generated_pdftexextra).edit_value.offset(-1);
                }
                _ => {
                    let fresh41 = temp;
                    temp = temp.offset(1);
                    *fresh41 = '%' as i32 as ::core::ffi::c_char;
                    let fresh42 = temp;
                    temp = temp.offset(1);
                    *fresh42 = c_0;
                }
            }
        } else {
            let fresh43 = temp;
            temp = temp.offset(1);
            *fresh43 = c_0;
        }
    }
    *temp = 0 as ::core::ffi::c_char;
    fullcmd = command;
    if system(fullcmd) != 0 as ::core::ffi::c_int {
        crate::stdio::fprintf_args(
            crate::stdio::stderr(),
            b"! Trouble executing `%s'.\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(command)],
        );
    }
    uexit(1 as ::core::ffi::c_int);
}
unsafe fn swap_items(
    mut p: *mut ::core::ffi::c_char,
    mut nitems: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
) {
    let __tekai_state_kpathsea: *mut crate::kpathsea::State = unsafe { crate::kpathsea::state() };

    let mut temp: ::core::ffi::c_char = 0;
    match size {
        16 => loop {
            let fresh33 = nitems;
            nitems = nitems - 1;
            if !(fresh33 != 0) {
                break;
            }
            temp = *p.offset(0 as ::core::ffi::c_int as isize);
            *p.offset(0 as ::core::ffi::c_int as isize) =
                *p.offset(15 as ::core::ffi::c_int as isize);
            *p.offset(15 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(1 as ::core::ffi::c_int as isize);
            *p.offset(1 as ::core::ffi::c_int as isize) =
                *p.offset(14 as ::core::ffi::c_int as isize);
            *p.offset(14 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(2 as ::core::ffi::c_int as isize);
            *p.offset(2 as ::core::ffi::c_int as isize) =
                *p.offset(13 as ::core::ffi::c_int as isize);
            *p.offset(13 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(3 as ::core::ffi::c_int as isize);
            *p.offset(3 as ::core::ffi::c_int as isize) =
                *p.offset(12 as ::core::ffi::c_int as isize);
            *p.offset(12 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(4 as ::core::ffi::c_int as isize);
            *p.offset(4 as ::core::ffi::c_int as isize) =
                *p.offset(11 as ::core::ffi::c_int as isize);
            *p.offset(11 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(5 as ::core::ffi::c_int as isize);
            *p.offset(5 as ::core::ffi::c_int as isize) =
                *p.offset(10 as ::core::ffi::c_int as isize);
            *p.offset(10 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(6 as ::core::ffi::c_int as isize);
            *p.offset(6 as ::core::ffi::c_int as isize) =
                *p.offset(9 as ::core::ffi::c_int as isize);
            *p.offset(9 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(7 as ::core::ffi::c_int as isize);
            *p.offset(7 as ::core::ffi::c_int as isize) =
                *p.offset(8 as ::core::ffi::c_int as isize);
            *p.offset(8 as ::core::ffi::c_int as isize) = temp;
            p = p.offset(size as isize);
        },
        8 => loop {
            let fresh34 = nitems;
            nitems = nitems - 1;
            if !(fresh34 != 0) {
                break;
            }
            temp = *p.offset(0 as ::core::ffi::c_int as isize);
            *p.offset(0 as ::core::ffi::c_int as isize) =
                *p.offset(7 as ::core::ffi::c_int as isize);
            *p.offset(7 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(1 as ::core::ffi::c_int as isize);
            *p.offset(1 as ::core::ffi::c_int as isize) =
                *p.offset(6 as ::core::ffi::c_int as isize);
            *p.offset(6 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(2 as ::core::ffi::c_int as isize);
            *p.offset(2 as ::core::ffi::c_int as isize) =
                *p.offset(5 as ::core::ffi::c_int as isize);
            *p.offset(5 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(3 as ::core::ffi::c_int as isize);
            *p.offset(3 as ::core::ffi::c_int as isize) =
                *p.offset(4 as ::core::ffi::c_int as isize);
            *p.offset(4 as ::core::ffi::c_int as isize) = temp;
            p = p.offset(size as isize);
        },
        4 => loop {
            let fresh35 = nitems;
            nitems = nitems - 1;
            if !(fresh35 != 0) {
                break;
            }
            temp = *p.offset(0 as ::core::ffi::c_int as isize);
            *p.offset(0 as ::core::ffi::c_int as isize) =
                *p.offset(3 as ::core::ffi::c_int as isize);
            *p.offset(3 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(1 as ::core::ffi::c_int as isize);
            *p.offset(1 as ::core::ffi::c_int as isize) =
                *p.offset(2 as ::core::ffi::c_int as isize);
            *p.offset(2 as ::core::ffi::c_int as isize) = temp;
            p = p.offset(size as isize);
        },
        2 => loop {
            let fresh36 = nitems;
            nitems = nitems - 1;
            if !(fresh36 != 0) {
                break;
            }
            temp = *p.offset(0 as ::core::ffi::c_int as isize);
            *p.offset(0 as ::core::ffi::c_int as isize) =
                *p.offset(1 as ::core::ffi::c_int as isize);
            *p.offset(1 as ::core::ffi::c_int as isize) = temp;
            p = p.offset(size as isize);
        },
        1 => {}
        _ => {
            crate::stdio::fprintf_args(
                crate::stdio::stderr(),
                b"%s: fatal: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(
                    (*(*__tekai_state_kpathsea).kpse_def).invocation_name,
                )],
            );
            crate::stdio::fprintf_args(
                crate::stdio::stderr(),
                b"Can't swap a %d-byte item for (un)dumping\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(size)],
            );
            fputs(
                b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                crate::stdio::stderr(),
            );
            exit(1 as ::core::ffi::c_int);
        }
    };
}
pub(crate) unsafe fn do_dump(
    mut p: *mut ::core::ffi::c_char,
    mut item_size: ::core::ffi::c_int,
    mut nitems: ::core::ffi::c_int,
    mut out_file: gzFile,
) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    swap_items(p, nitems, item_size);
    if gzwrite(
        out_file,
        p as voidpc,
        (item_size * nitems) as ::core::ffi::c_uint,
    ) != item_size * nitems
    {
        crate::stdio::fprintf_args(
            crate::stdio::stderr(),
            b"! Could not write %d %d-byte item(s) to %s.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(nitems),
                crate::utils::PrintfArg::from(item_size),
                crate::utils::PrintfArg::from(
                    (*__tekai_state_generated_pdftexextra)
                        .nameoffile
                        .offset(1 as ::core::ffi::c_int as isize),
                ),
            ],
        );
        uexit(1 as ::core::ffi::c_int);
    }
    swap_items(p, nitems, item_size);
}
pub(crate) unsafe fn do_undump(
    mut p: *mut ::core::ffi::c_char,
    mut item_size: ::core::ffi::c_int,
    mut nitems: ::core::ffi::c_int,
    mut in_file: gzFile,
) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_kpathsea: *mut crate::kpathsea::State = unsafe { crate::kpathsea::state() };

    if gzread(
        in_file,
        p as voidp,
        (item_size * nitems) as ::core::ffi::c_uint,
    ) != item_size * nitems
    {
        crate::stdio::fprintf_args(
            crate::stdio::stderr(),
            b"%s: fatal: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                (*(*__tekai_state_kpathsea).kpse_def).invocation_name,
            )],
        );
        crate::stdio::fprintf_args(
            crate::stdio::stderr(),
            b"Could not undump %d %d-byte item(s) from %s\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(nitems),
                crate::utils::PrintfArg::from(item_size),
                crate::utils::PrintfArg::from(
                    (*__tekai_state_generated_pdftexextra)
                        .nameoffile
                        .offset(1 as ::core::ffi::c_int as isize),
                ),
            ],
        );
        fputs(
            b".\n\0" as *const u8 as *const ::core::ffi::c_char,
            crate::stdio::stderr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    swap_items(p, nitems, item_size);
}
pub unsafe fn makefullnamestring() -> strnumber {
    let __tekai_state_openclose: *mut crate::openclose::State =
        unsafe { crate::openclose::state() };

    return maketexstring((*__tekai_state_openclose).fullnameoffile as *const ::core::ffi::c_char);
}
pub unsafe fn getjobname(mut name: strnumber) -> strnumber {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut ret: strnumber = name;
    let mut i: ::core::ffi::c_int = 0;
    let mut l_0: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    if !(*__tekai_state_generated_pdftexextra).c_job_name.is_null() {
        ret = maketexstring(
            (*__tekai_state_generated_pdftexextra).c_job_name as *const ::core::ffi::c_char,
        );
    }
    return ret;
}
unsafe fn compare_paths(mut p1: const_string, mut p2: const_string) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    loop {
        ret = *p1 as ::core::ffi::c_int - *p2 as ::core::ffi::c_int;
        if !(ret == 0 as ::core::ffi::c_int && *p2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            || *p1 as ::core::ffi::c_int == DIR_SEP && *p2 as ::core::ffi::c_int == DIR_SEP)
        {
            break;
        }
        p1 = p1.offset(1);
        p2 = p2.offset(1);
    }
    ret = if ret < 0 as ::core::ffi::c_int {
        -(1 as ::core::ffi::c_int)
    } else if ret > 0 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    return ret;
}
pub unsafe fn gettexstring(mut s: strnumber) -> string {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut len: poolpointer = 0;
    let mut name: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    len = *(*__tekai_state_generated_pdftexextra)
        .strstart
        .offset((s as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        - *(*__tekai_state_generated_pdftexextra)
            .strstart
            .offset(s as isize);
    name = xmalloc((len as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t) as string;
    strncpy(
        name as *mut ::core::ffi::c_char,
        (*__tekai_state_generated_pdftexextra).strpool.offset(
            *(*__tekai_state_generated_pdftexextra)
                .strstart
                .offset(s as isize) as isize,
        ) as *mut packedASCIIcode as string as *const ::core::ffi::c_char,
        len as size_t,
    );
    *name.offset(len as isize) = 0 as ::core::ffi::c_char;
    return name;
}
pub unsafe fn isnewsource(mut srcfilename: strnumber, mut lineno: ::core::ffi::c_int) -> boolean {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut name: *mut ::core::ffi::c_char = gettexstring(srcfilename) as *mut ::core::ffi::c_char;
    return (compare_paths(
        name as const_string,
        (*__tekai_state_generated_pdftexextra).last_source_name as const_string,
    ) != 0 as ::core::ffi::c_int
        || lineno != (*__tekai_state_generated_pdftexextra).last_lineno)
        as ::core::ffi::c_int;
}
pub unsafe fn remembersourceinfo(mut srcfilename: strnumber, mut lineno: ::core::ffi::c_int) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    if !(*__tekai_state_generated_pdftexextra)
        .last_source_name
        .is_null()
    {
        free((*__tekai_state_generated_pdftexextra).last_source_name as *mut ::core::ffi::c_void);
    }
    (*__tekai_state_generated_pdftexextra).last_source_name =
        gettexstring(srcfilename) as *mut ::core::ffi::c_char;
    (*__tekai_state_generated_pdftexextra).last_lineno = lineno;
}
pub unsafe fn makesrcspecial(
    mut srcfilename: strnumber,
    mut lineno: ::core::ffi::c_int,
) -> poolpointer {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut oldpoolptr: poolpointer = (*__tekai_state_generated_pdftexextra).poolptr as poolpointer;
    let mut filename: *mut ::core::ffi::c_char =
        gettexstring(srcfilename) as *mut ::core::ffi::c_char;
    let mut buf: [::core::ffi::c_char; 40] = [0; 40];
    let mut s: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
    crate::utils::sprintf_args(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"src:%d \0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(lineno)],
    );
    if ((*__tekai_state_generated_pdftexextra).poolptr as size_t)
        .wrapping_add(strlen(&raw mut buf as *mut ::core::ffi::c_char))
        .wrapping_add(strlen(filename))
        >= (*__tekai_state_generated_pdftexextra).poolsize as size_t
    {
        crate::stdio::fprintf_args(
            crate::stdio::stderr(),
            b"\nstring pool overflow\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
        exit(1 as ::core::ffi::c_int);
    }
    s = &raw mut buf as *mut ::core::ffi::c_char;
    while *s != 0 {
        let fresh44 = s;
        s = s.offset(1);
        let fresh45 = (*__tekai_state_generated_pdftexextra).poolptr;
        (*__tekai_state_generated_pdftexextra).poolptr =
            (*__tekai_state_generated_pdftexextra).poolptr + 1;
        *(*__tekai_state_generated_pdftexextra)
            .strpool
            .offset(fresh45 as isize) = *fresh44 as packedASCIIcode;
    }
    s = filename;
    while *s != 0 {
        let fresh46 = s;
        s = s.offset(1);
        let fresh47 = (*__tekai_state_generated_pdftexextra).poolptr;
        (*__tekai_state_generated_pdftexextra).poolptr =
            (*__tekai_state_generated_pdftexextra).poolptr + 1;
        *(*__tekai_state_generated_pdftexextra)
            .strpool
            .offset(fresh47 as isize) = *fresh46 as packedASCIIcode;
    }
    return oldpoolptr;
}
pub const MAX_CSTRING_LEN: ::core::ffi::c_int =
    1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
pub(crate) unsafe fn start_date() -> crate::time::PdfDate {
    time_result(crate::time::creation_date())
}
pub unsafe fn initstarttime() {
    let _ = start_date();
}
pub unsafe fn find_input_file(mut s: integer) -> string {
    let __tekai_state_openclose: *mut crate::openclose::State =
        unsafe { crate::openclose::state() };

    let mut filename: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    filename = makecfilename(s) as string;
    if !(*__tekai_state_openclose).output_directory.is_null()
        && kpse_absolute_p(filename as const_string, false_0) == 0
    {
        let mut pathname: string = concat3(
            (*__tekai_state_openclose).output_directory as const_string,
            DIR_SEP_STRING.as_ptr(),
            filename as const_string,
        );
        if readable_path(std::path::Path::new(std::ffi::OsStr::from_bytes(
            CStr::from_ptr(pathname).to_bytes(),
        ))) && dir_p(pathname) == 0
        {
            return pathname;
        }
        if !pathname.is_null() {
            free(pathname as *mut ::core::ffi::c_void);
        }
        pathname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if kpse_in_name_ok(filename as const_string) == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return kpse_find_file(filename as const_string, kpse_tex_format, true_0);
}
pub unsafe fn makecstring(mut s: integer) -> *mut ::core::ffi::c_char {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();

    let mut allocgrow: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut l_0: ::core::ffi::c_int = *(*__tekai_state_generated_pdftexextra)
        .strstart
        .offset((s as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int
        - *(*__tekai_state_generated_pdftexextra)
            .strstart
            .offset(s as isize) as ::core::ffi::c_int;
    if (l_0 + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint
        > (1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as ::core::ffi::c_uint
    {
        crate::utils::pdftex_fail_args(
            b"buffer overflow at file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(
                    b"tekai-engine/generated/texmfmp.rs\0" as *const u8
                        as *const ::core::ffi::c_char,
                ),
                crate::utils::PrintfArg::from(3494 as ::core::ffi::c_int),
            ],
        );
    }
    if (*__tekai_state_generated_pdftexextra)
        .local_makecstring_3837_cstrbuf
        .is_null()
    {
        (*__tekai_state_generated_pdftexextra).local_makecstring_3840_allocsize =
            l_0 + 1 as ::core::ffi::c_int;
        (*__tekai_state_generated_pdftexextra).local_makecstring_3837_cstrbuf = xmalloc(
            (((*__tekai_state_generated_pdftexextra).local_makecstring_3840_allocsize
                + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
        )
            as *mut ::core::ffi::c_char;
    } else if l_0 + 1 as ::core::ffi::c_int
        > (*__tekai_state_generated_pdftexextra).local_makecstring_3840_allocsize
    {
        allocgrow = ((*__tekai_state_generated_pdftexextra).local_makecstring_3840_allocsize
            as ::core::ffi::c_double
            * 0.2f64) as ::core::ffi::c_int;
        if l_0 + 1 as ::core::ffi::c_int - allocgrow
            > (*__tekai_state_generated_pdftexextra).local_makecstring_3840_allocsize
        {
            (*__tekai_state_generated_pdftexextra).local_makecstring_3840_allocsize =
                l_0 + 1 as ::core::ffi::c_int;
        } else if (*__tekai_state_generated_pdftexextra).local_makecstring_3840_allocsize
            < MAX_CSTRING_LEN - allocgrow
        {
            (*__tekai_state_generated_pdftexextra).local_makecstring_3840_allocsize += allocgrow;
        } else {
            (*__tekai_state_generated_pdftexextra).local_makecstring_3840_allocsize =
                MAX_CSTRING_LEN;
        }
        (*__tekai_state_generated_pdftexextra).local_makecstring_3837_cstrbuf = xrealloc(
            (*__tekai_state_generated_pdftexextra).local_makecstring_3837_cstrbuf as address,
            (((*__tekai_state_generated_pdftexextra).local_makecstring_3840_allocsize
                + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
        )
            as *mut ::core::ffi::c_char;
    }
    p = (*__tekai_state_generated_pdftexextra).local_makecstring_3837_cstrbuf;
    i = 0 as ::core::ffi::c_int;
    while i < l_0 {
        let fresh2 = p;
        p = p.offset(1);
        *fresh2 = *(*__tekai_state_generated_pdftexextra).strpool.offset(
            (i as poolpointer
                + *(*__tekai_state_generated_pdftexextra)
                    .strstart
                    .offset(s as isize)) as isize,
        ) as ::core::ffi::c_char;
        i += 1;
    }
    *p = 0 as ::core::ffi::c_char;
    return (*__tekai_state_generated_pdftexextra).local_makecstring_3837_cstrbuf;
}
pub unsafe fn makecfilename(mut s: integer) -> *mut ::core::ffi::c_char {
    let mut name: *mut ::core::ffi::c_char = makecstring(s);
    let mut p: *mut ::core::ffi::c_char = name;
    let mut q: *mut ::core::ffi::c_char = name;
    while *p != 0 {
        if *p as ::core::ffi::c_int != '"' as i32 {
            let fresh1 = q;
            q = q.offset(1);
            *fresh1 = *p;
        }
        p = p.offset(1);
    }
    *q = '\0' as i32 as ::core::ffi::c_char;
    return name;
}
unsafe fn append_metadata_bytes(bytes: &[u8]) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let Some(end) = crate::time::pool_end(
        (*__tekai_state_generated_pdftexextra).poolptr,
        (*__tekai_state_generated_pdftexextra).poolsize,
        bytes.len(),
    ) else {
        (*__tekai_state_generated_pdftexextra).poolptr =
            (*__tekai_state_generated_pdftexextra).poolsize;
        return;
    };
    std::ptr::copy_nonoverlapping(
        bytes.as_ptr(),
        (*__tekai_state_generated_pdftexextra)
            .strpool
            .add((*__tekai_state_generated_pdftexextra).poolptr as usize),
        bytes.len(),
    );
    (*__tekai_state_generated_pdftexextra).poolptr = end;
}
pub unsafe fn getcreationdate() {
    let date = start_date();
    append_metadata_bytes(date.as_bytes());
}
unsafe fn input_file_metadata(s: integer) -> Option<crate::time::FileMetadata> {
    use std::os::unix::ffi::OsStrExt;
    let file_name = find_input_file(s);
    if file_name.is_null() {
        return None;
    }
    recorder_record_input(file_name as const_string);
    let path = std::path::Path::new(std::ffi::OsStr::from_bytes(
        CStr::from_ptr(file_name).to_bytes(),
    ));
    let metadata = crate::time::file_metadata(path).ok();
    free(file_name.cast());
    metadata
}
pub unsafe fn getfilemoddate(s: integer) {
    if let Some(metadata) = input_file_metadata(s) {
        let date = time_result(crate::time::modification_date(metadata.modified_seconds));
        append_metadata_bytes(date.as_bytes());
    }
}
pub unsafe fn getfilesize(s: integer) {
    if let Some(metadata) = input_file_metadata(s) {
        let mut bytes = [0u8; 20];
        let mut at = bytes.len();
        let mut size = metadata.len;
        loop {
            at -= 1;
            bytes[at] = b'0' + (size % 10) as u8;
            size /= 10;
            if size == 0 {
                break;
            }
        }
        append_metadata_bytes(&bytes[at..]);
    }
}
pub unsafe fn getfiledump(
    mut s: integer,
    mut offset: ::core::ffi::c_int,
    mut length: ::core::ffi::c_int,
) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut f_0: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut read: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut data_ptr: poolpointer = 0;
    let mut data_end: poolpointer = 0;
    let mut file_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if length == 0 as ::core::ffi::c_int {
        return;
    }
    if (*__tekai_state_generated_pdftexextra).poolptr as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int * length
        + 1 as ::core::ffi::c_int
        >= (*__tekai_state_generated_pdftexextra).poolsize
    {
        (*__tekai_state_generated_pdftexextra).poolptr =
            (*__tekai_state_generated_pdftexextra).poolsize;
        return;
    }
    file_name = find_input_file(s) as *mut ::core::ffi::c_char;
    if file_name.is_null() {
        return;
    }
    f_0 = fopen(file_name, FOPEN_RBIN_MODE.as_ptr()) as *mut FILE;
    if f_0.is_null() {
        if !file_name.is_null() {
            free(file_name as *mut ::core::ffi::c_void);
        }
        file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return;
    }
    recorder_record_input(file_name as const_string);
    if fseek(f_0, offset as ::core::ffi::c_long, SEEK_SET) != 0 as ::core::ffi::c_int {
        if !file_name.is_null() {
            free(file_name as *mut ::core::ffi::c_void);
        }
        file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return;
    }
    data_ptr = ((*__tekai_state_generated_pdftexextra).poolptr as ::core::ffi::c_int + length)
        as poolpointer;
    read = fread(
        (*__tekai_state_generated_pdftexextra)
            .strpool
            .offset(data_ptr as isize) as *mut packedASCIIcode as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
        length as size_t,
        f_0,
    ) as ::core::ffi::c_int;
    fclose(f_0);
    data_end = (data_ptr as ::core::ffi::c_int + read) as poolpointer;
    while data_ptr < data_end {
        i = crate::utils::snprintf_args(
            (*__tekai_state_generated_pdftexextra)
                .strpool
                .offset((*__tekai_state_generated_pdftexextra).poolptr as isize)
                as *mut packedASCIIcode as *mut ::core::ffi::c_char,
            3 as size_t,
            b"%.2X\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                *(*__tekai_state_generated_pdftexextra)
                    .strpool
                    .offset(data_ptr as isize) as ::core::ffi::c_uint,
            )],
        );
        if i as ::core::ffi::c_uint >= 3 as ::core::ffi::c_int as ::core::ffi::c_uint {
            crate::utils::pdftex_fail_args(
                b"snprintf failed: file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(
                        b"tekai-engine/generated/texmfmp.rs\0" as *const u8
                            as *const ::core::ffi::c_char,
                    ),
                    crate::utils::PrintfArg::from(3725 as ::core::ffi::c_int),
                ],
            );
        }
        (*__tekai_state_generated_pdftexextra).poolptr += i;
        data_ptr += 1;
    }
    if !file_name.is_null() {
        free(file_name as *mut ::core::ffi::c_void);
    }
    file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
pub unsafe fn convertStringToHexString(
    mut in_0: *const ::core::ffi::c_char,
    mut out: *mut ::core::ffi::c_char,
    mut lin: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k_0: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 3] = [0; 3];
    j = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < lin {
        k_0 = crate::utils::snprintf_args(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t,
            b"%02X\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                *in_0.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_uint,
            )],
        );
        if k_0 as ::core::ffi::c_uint
            >= ::core::mem::size_of::<[::core::ffi::c_char; 3]>() as ::core::ffi::c_uint
        {
            crate::utils::pdftex_fail_args(
                b"snprintf failed: file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(
                        b"tekai-engine/generated/texmfmp.rs\0" as *const u8
                            as *const ::core::ffi::c_char,
                    ),
                    crate::utils::PrintfArg::from(3745 as ::core::ffi::c_int),
                ],
            );
        }
        let fresh3 = j;
        j = j + 1;
        *out.offset(fresh3 as isize) = buf[0 as ::core::ffi::c_int as usize];
        let fresh4 = j;
        j = j + 1;
        *out.offset(fresh4 as isize) = buf[1 as ::core::ffi::c_int as usize];
        i += 1;
    }
    *out.offset(j as isize) = '\0' as i32 as ::core::ffi::c_char;
}
pub const DIGEST_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const FILE_BUF_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub unsafe fn getmd5sum(mut s: strnumber, mut file: boolean) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut state: md5_state_t = md5_state_s {
        count: [0; 2],
        abcd: [0; 4],
        buf: [0; 64],
    };
    let mut digest: [md5_byte_t; 16] = [0; 16];
    let mut outbuf: [::core::ffi::c_char; 33] = [0; 33];
    let mut len: ::core::ffi::c_int = 2 as ::core::ffi::c_int * DIGEST_SIZE;
    if file != 0 {
        let mut file_buf: [::core::ffi::c_char; 1024] = [0; 1024];
        let mut read: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut f_0: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut file_name: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        file_name = find_input_file(s as integer) as *mut ::core::ffi::c_char;
        if file_name.is_null() {
            return;
        }
        f_0 = fopen(file_name, FOPEN_RBIN_MODE.as_ptr()) as *mut FILE;
        if f_0.is_null() {
            if !file_name.is_null() {
                free(file_name as *mut ::core::ffi::c_void);
            }
            file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return;
        }
        recorder_record_input(file_name as const_string);
        md5_init(&raw mut state);
        loop {
            read = fread(
                &raw mut file_buf as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
                FILE_BUF_SIZE as size_t,
                f_0,
            ) as ::core::ffi::c_int;
            if !(read > 0 as ::core::ffi::c_int) {
                break;
            }
            md5_append(
                &raw mut state,
                &raw mut file_buf as *mut ::core::ffi::c_char as *const md5_byte_t,
                read,
            );
        }
        md5_finish(&raw mut state, &raw mut digest as *mut md5_byte_t);
        fclose(f_0);
        if !file_name.is_null() {
            free(file_name as *mut ::core::ffi::c_void);
        }
        file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        md5_init(&raw mut state);
        md5_append(
            &raw mut state,
            (*__tekai_state_generated_pdftexextra).strpool.offset(
                *(*__tekai_state_generated_pdftexextra)
                    .strstart
                    .offset(s as isize) as isize,
            ) as *mut packedASCIIcode as *mut md5_byte_t,
            *(*__tekai_state_generated_pdftexextra)
                .strstart
                .offset((s as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - *(*__tekai_state_generated_pdftexextra)
                    .strstart
                    .offset(s as isize) as ::core::ffi::c_int,
        );
        md5_finish(&raw mut state, &raw mut digest as *mut md5_byte_t);
    }
    if (*__tekai_state_generated_pdftexextra).poolptr as ::core::ffi::c_int + len
        >= (*__tekai_state_generated_pdftexextra).poolsize
    {
        return;
    }
    convertStringToHexString(
        &raw mut digest as *mut md5_byte_t as *mut ::core::ffi::c_char,
        &raw mut outbuf as *mut ::core::ffi::c_char,
        DIGEST_SIZE,
    );
    memcpy(
        (*__tekai_state_generated_pdftexextra)
            .strpool
            .offset((*__tekai_state_generated_pdftexextra).poolptr as isize)
            as *mut packedASCIIcode as *mut ::core::ffi::c_void,
        &raw mut outbuf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len as size_t,
    );
    (*__tekai_state_generated_pdftexextra).poolptr += len;
}

pub(crate) struct State {
    pub(crate) bad: integer,
    pub(crate) xord: [ASCIIcode; 256],
    pub(crate) xchr: [ASCIIcode; 256],
    pub(crate) xprn: [ASCIIcode; 256],
    pub(crate) mubyteread: [halfword; 256],
    pub(crate) mubytewrite: [strnumber; 256],
    pub(crate) mubytecswrite: [halfword; 128],
    pub(crate) mubyteskip: integer,
    pub(crate) mubytekeep: integer,
    pub(crate) mubyteskeep: integer,
    pub(crate) mubyteprefix: integer,
    pub(crate) mubytetablein: boolean,
    pub(crate) mubytetableout: boolean,
    pub(crate) mubyterelax: boolean,
    pub(crate) mubytestart: boolean,
    pub(crate) mubytesstart: boolean,
    pub(crate) mubytetoken: halfword,
    pub(crate) mubytestoken: halfword,
    pub(crate) mubytesout: integer,
    pub(crate) mubyteslog: integer,
    pub(crate) specsout: integer,
    pub(crate) noconvert: boolean,
    pub(crate) activenoconvert: boolean,
    pub(crate) writenoexpanding: boolean,
    pub(crate) csconverting: boolean,
    pub(crate) specialprinting: boolean,
    pub(crate) messageprinting: boolean,
    pub(crate) nameoffile: *mut ASCIIcode,
    pub(crate) namelength: integer,
    pub(crate) buffer: *mut ASCIIcode,
    pub(crate) first: integer,
    pub(crate) last: integer,
    pub(crate) maxbufstack: integer,
    pub(crate) iniversion: boolean,
    dumpoption: boolean,
    pub(crate) dumpline: boolean,
    pub(crate) dump_name: const_string,
    pub(crate) bounddefault: integer,
    pub(crate) boundname: const_string,
    pub(crate) membot: integer,
    pub(crate) mainmemory: integer,
    pub(crate) extramembot: integer,
    pub(crate) memmin: integer,
    pub(crate) memtop: integer,
    pub(crate) extramemtop: integer,
    pub(crate) memmax: integer,
    pub(crate) errorline: integer,
    pub(crate) halferrorline: integer,
    pub(crate) maxprintline: integer,
    pub(crate) maxstrings: integer,
    pub(crate) stringsfree: integer,
    pub(crate) stringvacancies: integer,
    pub(crate) poolsize: integer,
    pub(crate) poolfree: integer,
    pub(crate) fontmemsize: integer,
    pub(crate) fontmax: integer,
    pub(crate) fontk: integer,
    pub(crate) hyphsize: integer,
    pub(crate) triesize: integer,
    pub(crate) bufsize: integer,
    pub(crate) stacksize: integer,
    pub(crate) maxinopen: integer,
    pub(crate) paramsize: integer,
    pub(crate) nestsize: integer,
    pub(crate) savesize: integer,
    pub(crate) dvibufsize: integer,
    pub(crate) expanddepth: integer,
    pub(crate) parsefirstlinep: ::core::ffi::c_int,
    pub(crate) filelineerrorstylep: ::core::ffi::c_int,
    pub(crate) eightbitp: ::core::ffi::c_int,
    pub(crate) haltonerrorp: ::core::ffi::c_int,
    pub(crate) haltingonerrorp: boolean,
    pub(crate) quotedfilename: boolean,
    pub(crate) srcspecialsp: boolean,
    pub(crate) insertsrcspecialauto: boolean,
    pub(crate) insertsrcspecialeverypar: boolean,
    insertsrcspecialeveryparend: boolean,
    insertsrcspecialeverycr: boolean,
    pub(crate) insertsrcspecialeverymath: boolean,
    insertsrcspecialeveryhbox: boolean,
    pub(crate) insertsrcspecialeveryvbox: boolean,
    insertsrcspecialeverydisplay: boolean,
    pub(crate) strpool: *mut packedASCIIcode,
    pub(crate) strstart: *mut poolpointer,
    pub(crate) poolptr: poolpointer,
    pub(crate) strptr: strnumber,
    pub(crate) initpoolptr: poolpointer,
    pub(crate) initstrptr: strnumber,
    poolfile: alphafile,
    pub(crate) logfile: alphafile,
    pub(crate) selector: ::core::ffi::c_uchar,
    pub(crate) dig: [::core::ffi::c_uchar; 23],
    pub(crate) tally: integer,
    pub(crate) termoffset: integer,
    pub(crate) fileoffset: integer,
    pub(crate) trickbuf: [ASCIIcode; 256],
    pub(crate) trickcount: integer,
    pub(crate) firstcount: integer,
    pub(crate) interaction: ::core::ffi::c_uchar,
    pub(crate) interactionoption: ::core::ffi::c_uchar,
    pub(crate) deletionsallowed: boolean,
    pub(crate) setboxallowed: boolean,
    pub(crate) history: ::core::ffi::c_uchar,
    pub(crate) errorcount: schar,
    pub(crate) helpline: [strnumber; 6],
    pub(crate) helpptr: ::core::ffi::c_uchar,
    pub(crate) useerrhelp: boolean,
    pub(crate) interrupt: integer,
    pub(crate) OKtointerrupt: boolean,
    pub(crate) savearitherror: boolean,
    pub(crate) aritherror: boolean,
    pub(crate) texremainder: scaled,
    pub(crate) randoms: [integer; 55],
    pub(crate) jrandom: ::core::ffi::c_uchar,
    pub(crate) randomseed: scaled,
    pub(crate) twotothe: [integer; 31],
    pub(crate) speclog: [integer; 29],
    pub(crate) tempptr: halfword,
    pub(crate) yzmem: *mut memoryword,
    pub(crate) zmem: *mut memoryword,
    pub(crate) lomemmax: halfword,
    pub(crate) himemmin: halfword,
    pub(crate) varused: integer,
    pub(crate) dynused: integer,
    pub(crate) avail: halfword,
    pub(crate) memend: halfword,
    pub(crate) rover: halfword,
    pub(crate) fontinshortdisplay: integer,
    pub(crate) depththreshold: integer,
    pub(crate) breadthmax: integer,
    pub(crate) nest: *mut liststaterecord,
    pub(crate) nestptr: integer,
    pub(crate) maxneststack: integer,
    pub(crate) curlist: liststaterecord,
    pub(crate) shownmode: ::core::ffi::c_short,
    pub(crate) savetail: halfword,
    pub(crate) prevtail: halfword,
    pub(crate) oldsetting: ::core::ffi::c_uchar,
    pub(crate) oldselectorignorederr: ::core::ffi::c_uchar,
    pub(crate) systime: integer,
    pub(crate) sysday: integer,
    pub(crate) sysmonth: integer,
    pub(crate) sysyear: integer,
    pub(crate) zeqtb: *mut memoryword,
    pub(crate) zzzaa: [quarterword; 916],
    pub(crate) hash: *mut twohalves,
    pub(crate) yhash: *mut twohalves,
    pub(crate) hashused: halfword,
    pub(crate) hashextra: halfword,
    pub(crate) hashtop: halfword,
    pub(crate) eqtbtop: halfword,
    pub(crate) hashhigh: halfword,
    pub(crate) nonewcontrolsequence: boolean,
    pub(crate) cscount: integer,
    pub(crate) prim: [twohalves; 2101],
    pub(crate) primused: halfword,
    pub(crate) savestack: *mut memoryword,
    pub(crate) saveptr: integer,
    pub(crate) maxsavestack: integer,
    pub(crate) curlevel: quarterword,
    pub(crate) curgroup: groupcode,
    pub(crate) curboundary: integer,
    pub(crate) magset: integer,
    pub(crate) curcmd: eightbits,
    pub(crate) curchr: halfword,
    pub(crate) curcs: halfword,
    pub(crate) curtok: halfword,
    pub(crate) inputstack: *mut instaterecord,
    pub(crate) inputptr: integer,
    pub(crate) maxinstack: integer,
    pub(crate) curinput: instaterecord,
    pub(crate) inopen: integer,
    pub(crate) openparens: integer,
    pub(crate) inputfile: *mut alphafile,
    pub(crate) line: integer,
    pub(crate) linestack: *mut integer,
    pub(crate) sourcefilenamestack: *mut strnumber,
    pub(crate) fullsourcefilenamestack: *mut strnumber,
    pub(crate) scannerstatus: ::core::ffi::c_uchar,
    pub(crate) warningindex: halfword,
    pub(crate) defref: halfword,
    pub(crate) paramstack: *mut halfword,
    pub(crate) paramptr: integer,
    pub(crate) maxparamstack: integer,
    pub(crate) alignstate: integer,
    pub(crate) baseptr: integer,
    pub(crate) parloc: halfword,
    pub(crate) partoken: halfword,
    pub(crate) forceeof: boolean,
    pub(crate) isincsname: boolean,
    pub(crate) curmark: [halfword; 5],
    pub(crate) longstate: ::core::ffi::c_uchar,
    pub(crate) pstack: [halfword; 9],
    pub(crate) curval: integer,
    pub(crate) curvallevel: ::core::ffi::c_uchar,
    pub(crate) radix: smallnumber,
    pub(crate) curorder: glueord,
    pub(crate) readfile: [alphafile; 16],
    pub(crate) readopen: [::core::ffi::c_uchar; 17],
    pub(crate) condptr: halfword,
    pub(crate) iflimit: ::core::ffi::c_uchar,
    pub(crate) curif: smallnumber,
    pub(crate) ifline: integer,
    pub(crate) skipline: integer,
    pub(crate) curname: strnumber,
    pub(crate) curarea: strnumber,
    pub(crate) curext: strnumber,
    pub(crate) areadelimiter: poolpointer,
    pub(crate) extdelimiter: poolpointer,
    pub(crate) formatdefaultlength: integer,
    pub(crate) TEXformatdefault: string,
    pub(crate) nameinprogress: boolean,
    pub(crate) jobname: strnumber,
    pub(crate) logopened: boolean,
    pub(crate) dvifile: bytefile,
    pub(crate) outputfilename: strnumber,
    pub(crate) texmflogname: strnumber,
    pub(crate) tfmfile: bytefile,
    pub(crate) fontinfo: *mut fmemoryword,
    pub(crate) fmemptr: fontindex,
    pub(crate) fontptr: internalfontnumber,
    pub(crate) fontcheck: *mut fourquarters,
    pub(crate) fontsize: *mut scaled,
    pub(crate) fontdsize: *mut scaled,
    pub(crate) fontparams: *mut fontindex,
    pub(crate) fontname: *mut strnumber,
    pub(crate) fontarea: *mut strnumber,
    pub(crate) fontbc: *mut eightbits,
    pub(crate) fontec: *mut eightbits,
    pub(crate) fontglue: *mut halfword,
    pub(crate) fontused: *mut boolean,
    pub(crate) hyphenchar: *mut integer,
    pub(crate) skewchar: *mut integer,
    pub(crate) bcharlabel: *mut fontindex,
    pub(crate) fontbchar: *mut ninebits,
    pub(crate) fontfalsebchar: *mut ninebits,
    pub(crate) charbase: *mut integer,
    pub(crate) widthbase: *mut integer,
    pub(crate) heightbase: *mut integer,
    pub(crate) depthbase: *mut integer,
    pub(crate) italicbase: *mut integer,
    pub(crate) ligkernbase: *mut integer,
    pub(crate) kernbase: *mut integer,
    pub(crate) extenbase: *mut integer,
    pub(crate) parambase: *mut integer,
    pub(crate) nullcharacter: fourquarters,
    pub(crate) totalpages: integer,
    pub(crate) maxv: scaled,
    pub(crate) maxh: scaled,
    pub(crate) maxpush: integer,
    pub(crate) lastbop: integer,
    pub(crate) deadcycles: integer,
    pub(crate) doingleaders: boolean,
    pub(crate) c: quarterword,
    pub(crate) f: internalfontnumber,
    pub(crate) ruleht: scaled,
    pub(crate) ruledp: scaled,
    pub(crate) rulewd: scaled,
    pub(crate) g: halfword,
    pub(crate) lq: integer,
    pub(crate) lr: integer,
    pub(crate) dvibuf: *mut eightbits,
    pub(crate) halfbuf: integer,
    pub(crate) dvilimit: integer,
    pub(crate) dviptr: integer,
    pub(crate) dvioffset: integer,
    pub(crate) dvigone: integer,
    pub(crate) downptr: halfword,
    pub(crate) rightptr: halfword,
    pub(crate) dvih: scaled,
    pub(crate) dviv: scaled,
    pub(crate) curh: scaled,
    pub(crate) curv: scaled,
    pub(crate) dvif: internalfontnumber,
    pub(crate) curs: integer,
    pub(crate) pdfmemsize: integer,
    pub(crate) pdfmem: *mut integer,
    pub(crate) pdfmemptr: integer,
    pub(crate) pdffile: bytefile,
    pub(crate) pdfbuf: *mut eightbits,
    pub(crate) pdfbufsize: integer,
    pub(crate) pdfptr: integer,
    pub(crate) pdfopbuf: *mut eightbits,
    pub(crate) pdfosbuf: *mut eightbits,
    pub(crate) pdfosbufsize: integer,
    pub(crate) pdfosobjnum: *mut integer,
    pub(crate) pdfosobjoff: *mut integer,
    pub(crate) pdfosobjidx: halfword,
    pub(crate) pdfoscntr: integer,
    pub(crate) pdfopptr: integer,
    pub(crate) pdfosptr: integer,
    pub(crate) pdfosmode: boolean,
    pub(crate) pdfosenable: boolean,
    pub(crate) pdfoscurobjnum: integer,
    pub(crate) pdfgone: longinteger,
    pub(crate) pdfsaveoffset: longinteger,
    pub(crate) zipwritestate: integer,
    pub(crate) fixedpdfmajorversion: integer,
    pub(crate) fixedpdfminorversion: integer,
    pub(crate) fixedpdfobjcompresslevel: integer,
    pub(crate) pdfversionwritten: boolean,
    pub(crate) fixedpdfoutput: integer,
    pub(crate) fixedpdfoutputset: boolean,
    pub(crate) fixedgamma: integer,
    pub(crate) fixedimagegamma: integer,
    pub(crate) fixedimagehicolor: boolean,
    pub(crate) fixedimageapplygamma: integer,
    pub(crate) epochseconds: integer,
    pub(crate) microseconds: integer,
    pub(crate) fixedpdfdraftmode: integer,
    pub(crate) fixedpdfdraftmodeset: boolean,
    pub(crate) pdfpagegroupval: integer,
    pub(crate) onebp: scaled,
    pub(crate) onehundredbp: scaled,
    pub(crate) onehundredinch: scaled,
    pub(crate) oneinch: integer,
    pub(crate) tenpow: [integer; 10],
    pub(crate) scaledout: integer,
    pub(crate) initpdfoutput: boolean,
    pub(crate) advcharwidths: integer,
    pub(crate) advcharwidthsout: scaled,
    pub(crate) pdff: internalfontnumber,
    pub(crate) pdfh: scaled,
    pub(crate) pdfv: scaled,
    pub(crate) pdftjstarth: scaled,
    curdeltah: scaled,
    pub(crate) pdfdeltah: scaled,
    pub(crate) pdforiginh: scaled,
    pub(crate) pdforiginv: scaled,
    pub(crate) pdfdoingstring: boolean,
    pub(crate) pdfdoingtext: boolean,
    pub(crate) minbpval: scaled,
    minfontval: scaled,
    pub(crate) fixedpkresolution: integer,
    pub(crate) fixeddecimaldigits: integer,
    pub(crate) fixedgentounicode: integer,
    pub(crate) fixedinclusioncopyfont: integer,
    pub(crate) pkscalefactor: integer,
    pub(crate) pdfoutputoption: integer,
    pub(crate) pdfoutputvalue: integer,
    pub(crate) pdfdraftmodeoption: integer,
    pub(crate) pdfdraftmodevalue: integer,
    pub(crate) pdfcurTma: integer,
    pub(crate) pdflastf: internalfontnumber,
    pub(crate) pdflastfs: internalfontnumber,
    pub(crate) pdfdummyfont: internalfontnumber,
    pub(crate) objtabsize: integer,
    pub(crate) objtab: *mut objentry,
    pub(crate) headtab: [integer; 11],
    pub(crate) pagestail: integer,
    pub(crate) objptr: integer,
    pub(crate) sysobjptr: integer,
    pub(crate) pdflastpages: integer,
    pub(crate) pdflastpage: integer,
    pub(crate) pdflaststream: integer,
    pub(crate) pdfstreamlength: longinteger,
    pub(crate) pdfstreamlengthoffset: longinteger,
    pub(crate) pdfseekwritelength: boolean,
    pub(crate) pdflastbyte: eightbits,
    pub(crate) pdfappendlistarg: integer,
    pub(crate) ff: integer,
    pub(crate) pdfboxspecmedia: integer,
    pub(crate) pdfboxspeccrop: integer,
    pub(crate) pdfboxspecbleed: integer,
    pub(crate) pdfboxspectrim: integer,
    pub(crate) pdfboxspecart: integer,
    pub(crate) pdfimageprocset: integer,
    pub(crate) pdftextprocset: boolean,
    pub(crate) pdffonttype: *mut eightbits,
    pub(crate) pdffontattr: *mut strnumber,
    pub(crate) pdffontnobuiltintounicode: *mut boolean,
    pub(crate) pdfcharused: *mut charusedarray,
    pub(crate) pdffontsize: *mut scaled,
    pub(crate) pdffontnum: *mut integer,
    pub(crate) pdffontmap: *mut fmentryptr,
    pub(crate) pdffontlist: halfword,
    pub(crate) pdfresnameprefix: strnumber,
    pub(crate) lasttokensstring: strnumber,
    pub(crate) vfpacketbase: *mut integer,
    pub(crate) vfdefaultfont: *mut internalfontnumber,
    pub(crate) vflocalfontnum: *mut internalfontnumber,
    pub(crate) vfpacketlength: integer,
    pub(crate) vffile: bytefile,
    pub(crate) vfnf: internalfontnumber,
    pub(crate) vfefnts: *mut integer,
    pub(crate) vfifnts: *mut internalfontnumber,
    pub(crate) tmpw: memoryword,
    pub(crate) vfcurs: integer,
    pub(crate) vfstack: [vfstackrecord; 101],
    pub(crate) vfstackptr: vfstackindex,
    pub(crate) savedpdfcurform: integer,
    pub(crate) pdftexbanner: strnumber,
    pub(crate) totalstretch: [scaled; 4],
    pub(crate) totalshrink: [scaled; 4],
    pub(crate) lastbadness: integer,
    pub(crate) adjusttail: halfword,
    pub(crate) pdffontblink: *mut internalfontnumber,
    pub(crate) pdffontelink: *mut internalfontnumber,
    pub(crate) pdffonthasspacechar: *mut boolean,
    pub(crate) pdffontstretch: *mut integer,
    pub(crate) pdffontshrink: *mut integer,
    pub(crate) pdffontstep: *mut integer,
    pub(crate) pdffontexpandratio: *mut integer,
    pub(crate) pdffontautoexpand: *mut boolean,
    pub(crate) pdffontlpbase: *mut integer,
    pub(crate) pdffontrpbase: *mut integer,
    pub(crate) pdffontefbase: *mut integer,
    pub(crate) pdffontknbsbase: *mut integer,
    pub(crate) pdffontstbsbase: *mut integer,
    pub(crate) pdffontshbsbase: *mut integer,
    pub(crate) pdffontknbcbase: *mut integer,
    pub(crate) pdffontknacbase: *mut integer,
    pub(crate) fontexpandratio: integer,
    pub(crate) lastleftmostchar: halfword,
    pub(crate) lastrightmostchar: halfword,
    pub(crate) hliststack: [halfword; 513],
    pub(crate) hliststacklevel: ::core::ffi::c_short,
    pub(crate) preadjusttail: halfword,
    pub(crate) packbeginline: integer,
    pub(crate) emptyfield: twohalves,
    pub(crate) nulldelimiter: fourquarters,
    pub(crate) curmlist: halfword,
    pub(crate) curstyle: smallnumber,
    pub(crate) cursize: smallnumber,
    pub(crate) curmu: scaled,
    pub(crate) mlistpenalties: boolean,
    pub(crate) curf: internalfontnumber,
    pub(crate) curc: quarterword,
    pub(crate) curi: fourquarters,
    pub(crate) magicoffset: integer,
    pub(crate) curalign: halfword,
    pub(crate) curspan: halfword,
    pub(crate) curloop: halfword,
    pub(crate) alignptr: halfword,
    pub(crate) curhead: halfword,
    pub(crate) curtail: halfword,
    pub(crate) curprehead: halfword,
    pub(crate) curpretail: halfword,
    pub(crate) justbox: halfword,
    pub(crate) passive: halfword,
    pub(crate) printednode: halfword,
    pub(crate) passnumber: halfword,
    pub(crate) activewidth: [scaled; 9],
    pub(crate) curactivewidth: [scaled; 9],
    pub(crate) background: [scaled; 9],
    pub(crate) breakwidth: [scaled; 9],
    pub(crate) autobreaking: boolean,
    pub(crate) prevp: halfword,
    pub(crate) firstp: halfword,
    pub(crate) prevcharp: halfword,
    nextcharp: halfword,
    pub(crate) tryprevbreak: boolean,
    pub(crate) prevlegal: halfword,
    prevprevlegal: halfword,
    prevautobreaking: boolean,
    prevactivewidth: [scaled; 9],
    pub(crate) rejectedcurp: halfword,
    pub(crate) beforerejectedcurp: boolean,
    pub(crate) maxstretchratio: integer,
    pub(crate) maxshrinkratio: integer,
    pub(crate) curfontstep: integer,
    pub(crate) noshrinkerroryet: boolean,
    pub(crate) curp: halfword,
    pub(crate) secondpass: boolean,
    pub(crate) finalpass: boolean,
    pub(crate) threshold: integer,
    pub(crate) minimaldemerits: [integer; 4],
    pub(crate) minimumdemerits: integer,
    pub(crate) bestplace: [halfword; 4],
    pub(crate) bestplline: [halfword; 4],
    pub(crate) discwidth: [scaled; 9],
    pub(crate) easyline: halfword,
    pub(crate) lastspecialline: halfword,
    pub(crate) firstwidth: scaled,
    pub(crate) secondwidth: scaled,
    pub(crate) firstindent: scaled,
    pub(crate) secondindent: scaled,
    pub(crate) bestbet: halfword,
    pub(crate) fewestdemerits: integer,
    pub(crate) bestline: halfword,
    pub(crate) actuallooseness: integer,
    pub(crate) linediff: integer,
    pub(crate) hc: [::core::ffi::c_short; 66],
    pub(crate) hn: ::core::ffi::c_uchar,
    pub(crate) ha: halfword,
    pub(crate) hb: halfword,
    pub(crate) hf: internalfontnumber,
    pub(crate) hu: [::core::ffi::c_short; 64],
    pub(crate) hyfchar: integer,
    pub(crate) initcurlang: ASCIIcode,
    pub(crate) curlang: ASCIIcode,
    pub(crate) lhyf: integer,
    pub(crate) rhyf: integer,
    pub(crate) initlhyf: integer,
    pub(crate) initrhyf: integer,
    pub(crate) hyfbchar: halfword,
    pub(crate) hyf: [::core::ffi::c_uchar; 65],
    pub(crate) initlist: halfword,
    pub(crate) initlig: boolean,
    pub(crate) initlft: boolean,
    pub(crate) hyphenpassed: smallnumber,
    pub(crate) curl: halfword,
    pub(crate) curr: halfword,
    pub(crate) curq: halfword,
    pub(crate) ligstack: halfword,
    pub(crate) ligaturepresent: boolean,
    pub(crate) lfthit: boolean,
    pub(crate) rthit: boolean,
    pub(crate) trietrl: *mut triepointer,
    pub(crate) trietro: *mut triepointer,
    pub(crate) trietrc: *mut quarterword,
    pub(crate) hyfdistance: [smallnumber; 35112],
    pub(crate) hyfnum: [smallnumber; 35112],
    pub(crate) hyfnext: [trieopcode; 35112],
    pub(crate) opstart: [integer; 256],
    pub(crate) hyphword: *mut strnumber,
    pub(crate) hyphlist: *mut halfword,
    pub(crate) hyphlink: *mut hyphpointer,
    pub(crate) hyphcount: integer,
    pub(crate) hyphnext: integer,
    pub(crate) zzzab: [integer; 70223],
    pub(crate) trieused: [trieopcode; 256],
    pub(crate) trieoplang: [ASCIIcode; 35112],
    pub(crate) trieopval: [trieopcode; 35112],
    pub(crate) trieopptr: integer,
    pub(crate) maxopused: trieopcode,
    smallop: boolean,
    pub(crate) triec: *mut packedASCIIcode,
    pub(crate) trieo: *mut trieopcode,
    pub(crate) triel: *mut triepointer,
    pub(crate) trier: *mut triepointer,
    pub(crate) trieptr: triepointer,
    pub(crate) triehash: *mut triepointer,
    pub(crate) trietaken: *mut boolean,
    pub(crate) triemin: [triepointer; 256],
    pub(crate) triemax: triepointer,
    pub(crate) trienotready: boolean,
    pub(crate) bestheightplusdepth: scaled,
    pub(crate) pagetail: halfword,
    pub(crate) pagecontents: ::core::ffi::c_uchar,
    pub(crate) pagemaxdepth: scaled,
    pub(crate) bestpagebreak: halfword,
    pub(crate) leastpagecost: integer,
    pub(crate) bestsize: scaled,
    pub(crate) pagesofar: [scaled; 8],
    pub(crate) lastglue: halfword,
    pub(crate) lastpenalty: integer,
    pub(crate) lastkern: scaled,
    pub(crate) lastnodetype: integer,
    pub(crate) insertpenalties: integer,
    pub(crate) outputactive: boolean,
    pub(crate) outputcanend: boolean,
    pub(crate) mainf: internalfontnumber,
    pub(crate) maini: fourquarters,
    pub(crate) mainj: fourquarters,
    pub(crate) maink: fontindex,
    pub(crate) mainp: halfword,
    pub(crate) mains: integer,
    pub(crate) bchar: halfword,
    pub(crate) falsebchar: halfword,
    pub(crate) cancelboundary: boolean,
    pub(crate) insdisc: boolean,
    pub(crate) curbox: halfword,
    pub(crate) aftertoken: halfword,
    pub(crate) longhelpseen: boolean,
    pub(crate) formatident: strnumber,
    pub(crate) fmtfile: wordfile,
    pub(crate) readyalready: integer,
    pub(crate) writefile: [alphafile; 16],
    pub(crate) writeopen: [boolean; 18],
    pub(crate) writeloc: halfword,
    pub(crate) pdflastobj: integer,
    pub(crate) pdflastxform: integer,
    pub(crate) pdflastximage: integer,
    pub(crate) pdflastximagepages: integer,
    pub(crate) pdflastximagecolordepth: integer,
    pub(crate) altrule: halfword,
    pub(crate) warnpdfpagebox: boolean,
    pub(crate) pdflastannot: integer,
    pub(crate) pdflastlink: integer,
    pub(crate) pdflastxpos: integer,
    pub(crate) pdflastypos: integer,
    pub(crate) pdfsnapxrefpos: integer,
    pub(crate) pdfsnapyrefpos: integer,
    pub(crate) countdosnapy: integer,
    pub(crate) pdfretval: integer,
    pub(crate) curpagewidth: scaled,
    pub(crate) curpageheight: scaled,
    pub(crate) curhoffset: scaled,
    pub(crate) curvoffset: scaled,
    pub(crate) pdfobjlist: halfword,
    pub(crate) pdfxformlist: halfword,
    pub(crate) pdfximagelist: halfword,
    pub(crate) lastthread: halfword,
    pub(crate) pdfthreadht: scaled,
    pub(crate) pdfthreaddp: scaled,
    pub(crate) pdfthreadwd: scaled,
    pub(crate) pdflastthreadid: halfword,
    pub(crate) pdflastthreadnamedid: boolean,
    pub(crate) pdfthreadlevel: integer,
    pub(crate) pdfannotlist: halfword,
    pub(crate) pdflinklist: halfword,
    pub(crate) pdfdestlist: halfword,
    pub(crate) pdfbeadlist: halfword,
    pub(crate) pdfobjcount: integer,
    pub(crate) pdfxformcount: integer,
    pub(crate) pdfximagecount: integer,
    pub(crate) pdfcurform: integer,
    pub(crate) pdflastoutline: integer,
    pub(crate) pdffirstoutline: integer,
    pub(crate) pdfparentoutline: integer,
    pub(crate) pdfxformwidth: scaled,
    pub(crate) pdfxformdepth: scaled,
    pub(crate) pdfxformheight: scaled,
    pub(crate) pdfinfotoks: halfword,
    pub(crate) pdfcatalogtoks: halfword,
    pub(crate) pdfcatalogopenaction: integer,
    pub(crate) pdfnamestoks: halfword,
    pub(crate) pdfdestnamesptr: integer,
    pub(crate) destnamessize: integer,
    pub(crate) destnames: *mut destnameentry,
    pub(crate) pkdpi: integer,
    imageorigy: integer,
    imageorigx: integer,
    pub(crate) pdftrailertoks: halfword,
    pub(crate) pdftraileridtoks: halfword,
    pub(crate) genfakedinterwordspace: boolean,
    pub(crate) genrunninglink: boolean,
    pub(crate) pdfspacefontname: strnumber,
    pub(crate) pdflinkstack: [pdflinkstackrecord; 11],
    pub(crate) pdflinkstackptr: smallnumber,
    pub(crate) isshippingpage: boolean,
    pub(crate) eTeXmode: ::core::ffi::c_uchar,
    pub(crate) etexp: boolean,
    pub(crate) eofseen: *mut boolean,
    pub(crate) LRptr: halfword,
    pub(crate) LRproblems: integer,
    pub(crate) curdir: smallnumber,
    pub(crate) pseudofiles: halfword,
    pub(crate) grpstack: *mut savepointer,
    pub(crate) ifstack: *mut halfword,
    pub(crate) maxregnum: halfword,
    pub(crate) maxreghelpline: strnumber,
    pub(crate) saroot: [halfword; 7],
    pub(crate) curptr: halfword,
    pub(crate) sanull: memoryword,
    pub(crate) sachain: halfword,
    pub(crate) salevel: quarterword,
    pub(crate) lastlinefill: halfword,
    pub(crate) dolastlinefit: boolean,
    pub(crate) activenodesize: smallnumber,
    pub(crate) fillwidth: [scaled; 3],
    pub(crate) bestplshort: [scaled; 4],
    pub(crate) bestplglue: [scaled; 4],
    pub(crate) hyphstart: triepointer,
    pub(crate) hyphindex: triepointer,
    pub(crate) discptr: [halfword; 4],
    pub(crate) editnamestart: poolpointer,
    pub(crate) editline: integer,
    pub(crate) editnamelength: integer,
    pub(crate) ipcon: ::core::ffi::c_int,
    pub(crate) stopatspace: boolean,
    savestrptr: strnumber,
    savepoolptr: poolpointer,
    pub(crate) shellenabledp: ::core::ffi::c_int,
    pub(crate) restrictedshell: ::core::ffi::c_int,
    pub(crate) outputcomment: *mut ::core::ffi::c_char,
    pub(crate) k: ::core::ffi::c_uchar,
    pub(crate) l: ::core::ffi::c_uchar,
    pub(crate) debugformatfile: boolean,
    pub(crate) expanddepthcount: integer,
    pub(crate) mltexp: boolean,
    pub(crate) mltexenabledp: boolean,
    pub(crate) accentc: integer,
    pub(crate) basec: integer,
    replacec: integer,
    pub(crate) iac: fourquarters,
    pub(crate) ibc: fourquarters,
    pub(crate) baseslant: real,
    pub(crate) accentslant: real,
    pub(crate) basexheight: scaled,
    pub(crate) basewidth: scaled,
    pub(crate) baseheight: scaled,
    pub(crate) accentwidth: scaled,
    pub(crate) accentheight: scaled,
    pub(crate) delta: scaled,
    pub(crate) enctexp: boolean,
    pub(crate) enctexenabledp: boolean,
    pub(crate) synctexoption: integer,
    pub(crate) synctexoffset: integer,
    PDFTEXHELP: [const_string; 57],
    cmdlist: *mut *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    argc: ::core::ffi::c_int,
    optind: ::core::ffi::c_int,
    user_progname: const_string,
    user_cnf_lines: *mut string,
    user_cnf_nlines: ::core::ffi::c_uint,
    c_job_name: const_string,
    pub(crate) translate_filename: string,
    default_translate_filename: string,
    last_source_name: *mut ::core::ffi::c_char,
    last_lineno: ::core::ffi::c_int,
    srcspecialsoption: boolean,
    pub(crate) ptexbanner: *const ::core::ffi::c_char,
    pipes: [*mut FILE; 16],
    edit_value: const_string,
    local_makecstring_3837_cstrbuf: *mut ::core::ffi::c_char,
    local_makecstring_3840_allocsize: ::core::ffi::c_int,
}

#[inline(always)]
fn __tekai_require_copy<T: Copy>() {}

pub(crate) struct StateInitialization {
    destination: *mut State,
    initialized: [bool; 709],
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
            if self.initialized[708] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).local_makecstring_3840_allocsize
                ));
            }
            if self.initialized[707] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).local_makecstring_3837_cstrbuf
                ));
            }
            if self.initialized[706] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).edit_value));
            }
            if self.initialized[705] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pipes));
            }
            if self.initialized[704] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).ptexbanner));
            }
            if self.initialized[703] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).srcspecialsoption
                ));
            }
            if self.initialized[702] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).last_lineno));
            }
            if self.initialized[701] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).last_source_name
                ));
            }
            if self.initialized[700] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).default_translate_filename
                ));
            }
            if self.initialized[699] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).translate_filename
                ));
            }
            if self.initialized[698] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).c_job_name));
            }
            if self.initialized[697] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).user_cnf_nlines));
            }
            if self.initialized[696] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).user_cnf_lines));
            }
            if self.initialized[695] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).user_progname));
            }
            if self.initialized[694] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).optind));
            }
            if self.initialized[693] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).argc));
            }
            if self.initialized[692] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).argv));
            }
            if self.initialized[691] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).cmdlist));
            }
            if self.initialized[690] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).PDFTEXHELP));
            }
            if self.initialized[689] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).synctexoffset));
            }
            if self.initialized[688] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).synctexoption));
            }
            if self.initialized[687] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).enctexenabledp));
            }
            if self.initialized[686] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).enctexp));
            }
            if self.initialized[685] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).delta));
            }
            if self.initialized[684] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).accentheight));
            }
            if self.initialized[683] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).accentwidth));
            }
            if self.initialized[682] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).baseheight));
            }
            if self.initialized[681] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).basewidth));
            }
            if self.initialized[680] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).basexheight));
            }
            if self.initialized[679] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).accentslant));
            }
            if self.initialized[678] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).baseslant));
            }
            if self.initialized[677] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).ibc));
            }
            if self.initialized[676] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).iac));
            }
            if self.initialized[675] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).replacec));
            }
            if self.initialized[674] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).basec));
            }
            if self.initialized[673] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).accentc));
            }
            if self.initialized[672] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mltexenabledp));
            }
            if self.initialized[671] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mltexp));
            }
            if self.initialized[670] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).expanddepthcount
                ));
            }
            if self.initialized[669] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).debugformatfile));
            }
            if self.initialized[668] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).l));
            }
            if self.initialized[667] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).k));
            }
            if self.initialized[666] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).outputcomment));
            }
            if self.initialized[665] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).restrictedshell));
            }
            if self.initialized[664] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).shellenabledp));
            }
            if self.initialized[663] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).savepoolptr));
            }
            if self.initialized[662] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).savestrptr));
            }
            if self.initialized[661] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).stopatspace));
            }
            if self.initialized[660] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).ipcon));
            }
            if self.initialized[659] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).editnamelength));
            }
            if self.initialized[658] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).editline));
            }
            if self.initialized[657] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).editnamestart));
            }
            if self.initialized[656] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).discptr));
            }
            if self.initialized[655] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hyphindex));
            }
            if self.initialized[654] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hyphstart));
            }
            if self.initialized[653] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).bestplglue));
            }
            if self.initialized[652] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).bestplshort));
            }
            if self.initialized[651] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fillwidth));
            }
            if self.initialized[650] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).activenodesize));
            }
            if self.initialized[649] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).dolastlinefit));
            }
            if self.initialized[648] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).lastlinefill));
            }
            if self.initialized[647] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).salevel));
            }
            if self.initialized[646] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).sachain));
            }
            if self.initialized[645] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).sanull));
            }
            if self.initialized[644] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curptr));
            }
            if self.initialized[643] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).saroot));
            }
            if self.initialized[642] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).maxreghelpline));
            }
            if self.initialized[641] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).maxregnum));
            }
            if self.initialized[640] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).ifstack));
            }
            if self.initialized[639] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).grpstack));
            }
            if self.initialized[638] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pseudofiles));
            }
            if self.initialized[637] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curdir));
            }
            if self.initialized[636] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).LRproblems));
            }
            if self.initialized[635] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).LRptr));
            }
            if self.initialized[634] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).eofseen));
            }
            if self.initialized[633] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).etexp));
            }
            if self.initialized[632] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).eTeXmode));
            }
            if self.initialized[631] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).isshippingpage));
            }
            if self.initialized[630] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdflinkstackptr));
            }
            if self.initialized[629] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdflinkstack));
            }
            if self.initialized[628] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).pdfspacefontname
                ));
            }
            if self.initialized[627] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).genrunninglink));
            }
            if self.initialized[626] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).genfakedinterwordspace
                ));
            }
            if self.initialized[625] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).pdftraileridtoks
                ));
            }
            if self.initialized[624] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdftrailertoks));
            }
            if self.initialized[623] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).imageorigx));
            }
            if self.initialized[622] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).imageorigy));
            }
            if self.initialized[621] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pkdpi));
            }
            if self.initialized[620] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).destnames));
            }
            if self.initialized[619] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).destnamessize));
            }
            if self.initialized[618] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfdestnamesptr));
            }
            if self.initialized[617] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfnamestoks));
            }
            if self.initialized[616] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).pdfcatalogopenaction
                ));
            }
            if self.initialized[615] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfcatalogtoks));
            }
            if self.initialized[614] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfinfotoks));
            }
            if self.initialized[613] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfxformheight));
            }
            if self.initialized[612] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfxformdepth));
            }
            if self.initialized[611] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfxformwidth));
            }
            if self.initialized[610] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).pdfparentoutline
                ));
            }
            if self.initialized[609] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffirstoutline));
            }
            if self.initialized[608] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdflastoutline));
            }
            if self.initialized[607] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfcurform));
            }
            if self.initialized[606] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfximagecount));
            }
            if self.initialized[605] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfxformcount));
            }
            if self.initialized[604] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfobjcount));
            }
            if self.initialized[603] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfbeadlist));
            }
            if self.initialized[602] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfdestlist));
            }
            if self.initialized[601] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdflinklist));
            }
            if self.initialized[600] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfannotlist));
            }
            if self.initialized[599] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfthreadlevel));
            }
            if self.initialized[598] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).pdflastthreadnamedid
                ));
            }
            if self.initialized[597] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdflastthreadid));
            }
            if self.initialized[596] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfthreadwd));
            }
            if self.initialized[595] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfthreaddp));
            }
            if self.initialized[594] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfthreadht));
            }
            if self.initialized[593] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).lastthread));
            }
            if self.initialized[592] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfximagelist));
            }
            if self.initialized[591] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfxformlist));
            }
            if self.initialized[590] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfobjlist));
            }
            if self.initialized[589] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curvoffset));
            }
            if self.initialized[588] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curhoffset));
            }
            if self.initialized[587] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curpageheight));
            }
            if self.initialized[586] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curpagewidth));
            }
            if self.initialized[585] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfretval));
            }
            if self.initialized[584] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).countdosnapy));
            }
            if self.initialized[583] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfsnapyrefpos));
            }
            if self.initialized[582] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfsnapxrefpos));
            }
            if self.initialized[581] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdflastypos));
            }
            if self.initialized[580] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdflastxpos));
            }
            if self.initialized[579] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdflastlink));
            }
            if self.initialized[578] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdflastannot));
            }
            if self.initialized[577] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).warnpdfpagebox));
            }
            if self.initialized[576] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).altrule));
            }
            if self.initialized[575] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).pdflastximagecolordepth
                ));
            }
            if self.initialized[574] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).pdflastximagepages
                ));
            }
            if self.initialized[573] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdflastximage));
            }
            if self.initialized[572] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdflastxform));
            }
            if self.initialized[571] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdflastobj));
            }
            if self.initialized[570] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).writeloc));
            }
            if self.initialized[569] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).writeopen));
            }
            if self.initialized[568] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).writefile));
            }
            if self.initialized[567] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).readyalready));
            }
            if self.initialized[566] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fmtfile));
            }
            if self.initialized[565] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).formatident));
            }
            if self.initialized[564] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).longhelpseen));
            }
            if self.initialized[563] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).aftertoken));
            }
            if self.initialized[562] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curbox));
            }
            if self.initialized[561] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).insdisc));
            }
            if self.initialized[560] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).cancelboundary));
            }
            if self.initialized[559] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).falsebchar));
            }
            if self.initialized[558] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).bchar));
            }
            if self.initialized[557] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mains));
            }
            if self.initialized[556] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mainp));
            }
            if self.initialized[555] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).maink));
            }
            if self.initialized[554] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mainj));
            }
            if self.initialized[553] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).maini));
            }
            if self.initialized[552] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mainf));
            }
            if self.initialized[551] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).outputcanend));
            }
            if self.initialized[550] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).outputactive));
            }
            if self.initialized[549] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).insertpenalties));
            }
            if self.initialized[548] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).lastnodetype));
            }
            if self.initialized[547] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).lastkern));
            }
            if self.initialized[546] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).lastpenalty));
            }
            if self.initialized[545] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).lastglue));
            }
            if self.initialized[544] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pagesofar));
            }
            if self.initialized[543] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).bestsize));
            }
            if self.initialized[542] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).leastpagecost));
            }
            if self.initialized[541] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).bestpagebreak));
            }
            if self.initialized[540] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pagemaxdepth));
            }
            if self.initialized[539] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pagecontents));
            }
            if self.initialized[538] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pagetail));
            }
            if self.initialized[537] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).bestheightplusdepth
                ));
            }
            if self.initialized[536] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).trienotready));
            }
            if self.initialized[535] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).triemax));
            }
            if self.initialized[534] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).triemin));
            }
            if self.initialized[533] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).trietaken));
            }
            if self.initialized[532] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).triehash));
            }
            if self.initialized[531] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).trieptr));
            }
            if self.initialized[530] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).trier));
            }
            if self.initialized[529] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).triel));
            }
            if self.initialized[528] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).trieo));
            }
            if self.initialized[527] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).triec));
            }
            if self.initialized[526] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).smallop));
            }
            if self.initialized[525] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).maxopused));
            }
            if self.initialized[524] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).trieopptr));
            }
            if self.initialized[523] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).trieopval));
            }
            if self.initialized[522] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).trieoplang));
            }
            if self.initialized[521] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).trieused));
            }
            if self.initialized[520] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).zzzab));
            }
            if self.initialized[519] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hyphnext));
            }
            if self.initialized[518] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hyphcount));
            }
            if self.initialized[517] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hyphlink));
            }
            if self.initialized[516] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hyphlist));
            }
            if self.initialized[515] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hyphword));
            }
            if self.initialized[514] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).opstart));
            }
            if self.initialized[513] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hyfnext));
            }
            if self.initialized[512] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hyfnum));
            }
            if self.initialized[511] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hyfdistance));
            }
            if self.initialized[510] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).trietrc));
            }
            if self.initialized[509] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).trietro));
            }
            if self.initialized[508] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).trietrl));
            }
            if self.initialized[507] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).rthit));
            }
            if self.initialized[506] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).lfthit));
            }
            if self.initialized[505] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).ligaturepresent));
            }
            if self.initialized[504] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).ligstack));
            }
            if self.initialized[503] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curq));
            }
            if self.initialized[502] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curr));
            }
            if self.initialized[501] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curl));
            }
            if self.initialized[500] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hyphenpassed));
            }
            if self.initialized[499] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).initlft));
            }
            if self.initialized[498] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).initlig));
            }
            if self.initialized[497] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).initlist));
            }
            if self.initialized[496] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hyf));
            }
            if self.initialized[495] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hyfbchar));
            }
            if self.initialized[494] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).initrhyf));
            }
            if self.initialized[493] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).initlhyf));
            }
            if self.initialized[492] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).rhyf));
            }
            if self.initialized[491] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).lhyf));
            }
            if self.initialized[490] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curlang));
            }
            if self.initialized[489] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).initcurlang));
            }
            if self.initialized[488] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hyfchar));
            }
            if self.initialized[487] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hu));
            }
            if self.initialized[486] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hf));
            }
            if self.initialized[485] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hb));
            }
            if self.initialized[484] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).ha));
            }
            if self.initialized[483] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hn));
            }
            if self.initialized[482] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hc));
            }
            if self.initialized[481] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).linediff));
            }
            if self.initialized[480] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).actuallooseness));
            }
            if self.initialized[479] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).bestline));
            }
            if self.initialized[478] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fewestdemerits));
            }
            if self.initialized[477] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).bestbet));
            }
            if self.initialized[476] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).secondindent));
            }
            if self.initialized[475] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).firstindent));
            }
            if self.initialized[474] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).secondwidth));
            }
            if self.initialized[473] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).firstwidth));
            }
            if self.initialized[472] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).lastspecialline));
            }
            if self.initialized[471] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).easyline));
            }
            if self.initialized[470] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).discwidth));
            }
            if self.initialized[469] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).bestplline));
            }
            if self.initialized[468] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).bestplace));
            }
            if self.initialized[467] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).minimumdemerits));
            }
            if self.initialized[466] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).minimaldemerits));
            }
            if self.initialized[465] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).threshold));
            }
            if self.initialized[464] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).finalpass));
            }
            if self.initialized[463] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).secondpass));
            }
            if self.initialized[462] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curp));
            }
            if self.initialized[461] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).noshrinkerroryet
                ));
            }
            if self.initialized[460] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curfontstep));
            }
            if self.initialized[459] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).maxshrinkratio));
            }
            if self.initialized[458] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).maxstretchratio));
            }
            if self.initialized[457] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).beforerejectedcurp
                ));
            }
            if self.initialized[456] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).rejectedcurp));
            }
            if self.initialized[455] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).prevactivewidth));
            }
            if self.initialized[454] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).prevautobreaking
                ));
            }
            if self.initialized[453] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).prevprevlegal));
            }
            if self.initialized[452] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).prevlegal));
            }
            if self.initialized[451] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).tryprevbreak));
            }
            if self.initialized[450] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).nextcharp));
            }
            if self.initialized[449] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).prevcharp));
            }
            if self.initialized[448] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).firstp));
            }
            if self.initialized[447] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).prevp));
            }
            if self.initialized[446] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).autobreaking));
            }
            if self.initialized[445] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).breakwidth));
            }
            if self.initialized[444] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).background));
            }
            if self.initialized[443] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curactivewidth));
            }
            if self.initialized[442] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).activewidth));
            }
            if self.initialized[441] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).passnumber));
            }
            if self.initialized[440] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).printednode));
            }
            if self.initialized[439] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).passive));
            }
            if self.initialized[438] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).justbox));
            }
            if self.initialized[437] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curpretail));
            }
            if self.initialized[436] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curprehead));
            }
            if self.initialized[435] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curtail));
            }
            if self.initialized[434] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curhead));
            }
            if self.initialized[433] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).alignptr));
            }
            if self.initialized[432] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curloop));
            }
            if self.initialized[431] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curspan));
            }
            if self.initialized[430] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curalign));
            }
            if self.initialized[429] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).magicoffset));
            }
            if self.initialized[428] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curi));
            }
            if self.initialized[427] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curc));
            }
            if self.initialized[426] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curf));
            }
            if self.initialized[425] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mlistpenalties));
            }
            if self.initialized[424] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curmu));
            }
            if self.initialized[423] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).cursize));
            }
            if self.initialized[422] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curstyle));
            }
            if self.initialized[421] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curmlist));
            }
            if self.initialized[420] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).nulldelimiter));
            }
            if self.initialized[419] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).emptyfield));
            }
            if self.initialized[418] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).packbeginline));
            }
            if self.initialized[417] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).preadjusttail));
            }
            if self.initialized[416] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hliststacklevel));
            }
            if self.initialized[415] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hliststack));
            }
            if self.initialized[414] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).lastrightmostchar
                ));
            }
            if self.initialized[413] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).lastleftmostchar
                ));
            }
            if self.initialized[412] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fontexpandratio));
            }
            if self.initialized[411] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffontknacbase));
            }
            if self.initialized[410] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffontknbcbase));
            }
            if self.initialized[409] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffontshbsbase));
            }
            if self.initialized[408] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffontstbsbase));
            }
            if self.initialized[407] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffontknbsbase));
            }
            if self.initialized[406] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffontefbase));
            }
            if self.initialized[405] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffontrpbase));
            }
            if self.initialized[404] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffontlpbase));
            }
            if self.initialized[403] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).pdffontautoexpand
                ));
            }
            if self.initialized[402] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).pdffontexpandratio
                ));
            }
            if self.initialized[401] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffontstep));
            }
            if self.initialized[400] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffontshrink));
            }
            if self.initialized[399] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffontstretch));
            }
            if self.initialized[398] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).pdffonthasspacechar
                ));
            }
            if self.initialized[397] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffontelink));
            }
            if self.initialized[396] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffontblink));
            }
            if self.initialized[395] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).adjusttail));
            }
            if self.initialized[394] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).lastbadness));
            }
            if self.initialized[393] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).totalshrink));
            }
            if self.initialized[392] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).totalstretch));
            }
            if self.initialized[391] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdftexbanner));
            }
            if self.initialized[390] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).savedpdfcurform));
            }
            if self.initialized[389] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).vfstackptr));
            }
            if self.initialized[388] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).vfstack));
            }
            if self.initialized[387] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).vfcurs));
            }
            if self.initialized[386] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).tmpw));
            }
            if self.initialized[385] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).vfifnts));
            }
            if self.initialized[384] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).vfefnts));
            }
            if self.initialized[383] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).vfnf));
            }
            if self.initialized[382] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).vffile));
            }
            if self.initialized[381] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).vfpacketlength));
            }
            if self.initialized[380] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).vflocalfontnum));
            }
            if self.initialized[379] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).vfdefaultfont));
            }
            if self.initialized[378] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).vfpacketbase));
            }
            if self.initialized[377] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).lasttokensstring
                ));
            }
            if self.initialized[376] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).pdfresnameprefix
                ));
            }
            if self.initialized[375] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffontlist));
            }
            if self.initialized[374] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffontmap));
            }
            if self.initialized[373] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffontnum));
            }
            if self.initialized[372] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffontsize));
            }
            if self.initialized[371] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfcharused));
            }
            if self.initialized[370] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).pdffontnobuiltintounicode
                ));
            }
            if self.initialized[369] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffontattr));
            }
            if self.initialized[368] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffonttype));
            }
            if self.initialized[367] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdftextprocset));
            }
            if self.initialized[366] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfimageprocset));
            }
            if self.initialized[365] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfboxspecart));
            }
            if self.initialized[364] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfboxspectrim));
            }
            if self.initialized[363] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfboxspecbleed));
            }
            if self.initialized[362] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfboxspeccrop));
            }
            if self.initialized[361] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfboxspecmedia));
            }
            if self.initialized[360] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).ff));
            }
            if self.initialized[359] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).pdfappendlistarg
                ));
            }
            if self.initialized[358] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdflastbyte));
            }
            if self.initialized[357] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).pdfseekwritelength
                ));
            }
            if self.initialized[356] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).pdfstreamlengthoffset
                ));
            }
            if self.initialized[355] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfstreamlength));
            }
            if self.initialized[354] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdflaststream));
            }
            if self.initialized[353] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdflastpage));
            }
            if self.initialized[352] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdflastpages));
            }
            if self.initialized[351] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).sysobjptr));
            }
            if self.initialized[350] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).objptr));
            }
            if self.initialized[349] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pagestail));
            }
            if self.initialized[348] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).headtab));
            }
            if self.initialized[347] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).objtab));
            }
            if self.initialized[346] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).objtabsize));
            }
            if self.initialized[345] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfdummyfont));
            }
            if self.initialized[344] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdflastfs));
            }
            if self.initialized[343] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdflastf));
            }
            if self.initialized[342] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfcurTma));
            }
            if self.initialized[341] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).pdfdraftmodevalue
                ));
            }
            if self.initialized[340] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).pdfdraftmodeoption
                ));
            }
            if self.initialized[339] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfoutputvalue));
            }
            if self.initialized[338] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfoutputoption));
            }
            if self.initialized[337] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pkscalefactor));
            }
            if self.initialized[336] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).fixedinclusioncopyfont
                ));
            }
            if self.initialized[335] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).fixedgentounicode
                ));
            }
            if self.initialized[334] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).fixeddecimaldigits
                ));
            }
            if self.initialized[333] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).fixedpkresolution
                ));
            }
            if self.initialized[332] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).minfontval));
            }
            if self.initialized[331] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).minbpval));
            }
            if self.initialized[330] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfdoingtext));
            }
            if self.initialized[329] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfdoingstring));
            }
            if self.initialized[328] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdforiginv));
            }
            if self.initialized[327] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdforiginh));
            }
            if self.initialized[326] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfdeltah));
            }
            if self.initialized[325] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curdeltah));
            }
            if self.initialized[324] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdftjstarth));
            }
            if self.initialized[323] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfv));
            }
            if self.initialized[322] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfh));
            }
            if self.initialized[321] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdff));
            }
            if self.initialized[320] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).advcharwidthsout
                ));
            }
            if self.initialized[319] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).advcharwidths));
            }
            if self.initialized[318] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).initpdfoutput));
            }
            if self.initialized[317] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).scaledout));
            }
            if self.initialized[316] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).tenpow));
            }
            if self.initialized[315] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).oneinch));
            }
            if self.initialized[314] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).onehundredinch));
            }
            if self.initialized[313] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).onehundredbp));
            }
            if self.initialized[312] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).onebp));
            }
            if self.initialized[311] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfpagegroupval));
            }
            if self.initialized[310] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).fixedpdfdraftmodeset
                ));
            }
            if self.initialized[309] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).fixedpdfdraftmode
                ));
            }
            if self.initialized[308] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).microseconds));
            }
            if self.initialized[307] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).epochseconds));
            }
            if self.initialized[306] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).fixedimageapplygamma
                ));
            }
            if self.initialized[305] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).fixedimagehicolor
                ));
            }
            if self.initialized[304] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fixedimagegamma));
            }
            if self.initialized[303] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fixedgamma));
            }
            if self.initialized[302] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).fixedpdfoutputset
                ));
            }
            if self.initialized[301] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fixedpdfoutput));
            }
            if self.initialized[300] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).pdfversionwritten
                ));
            }
            if self.initialized[299] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).fixedpdfobjcompresslevel
                ));
            }
            if self.initialized[298] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).fixedpdfminorversion
                ));
            }
            if self.initialized[297] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).fixedpdfmajorversion
                ));
            }
            if self.initialized[296] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).zipwritestate));
            }
            if self.initialized[295] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfsaveoffset));
            }
            if self.initialized[294] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfgone));
            }
            if self.initialized[293] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfoscurobjnum));
            }
            if self.initialized[292] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfosenable));
            }
            if self.initialized[291] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfosmode));
            }
            if self.initialized[290] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfosptr));
            }
            if self.initialized[289] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfopptr));
            }
            if self.initialized[288] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfoscntr));
            }
            if self.initialized[287] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfosobjidx));
            }
            if self.initialized[286] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfosobjoff));
            }
            if self.initialized[285] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfosobjnum));
            }
            if self.initialized[284] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfosbufsize));
            }
            if self.initialized[283] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfosbuf));
            }
            if self.initialized[282] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfopbuf));
            }
            if self.initialized[281] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfptr));
            }
            if self.initialized[280] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfbufsize));
            }
            if self.initialized[279] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfbuf));
            }
            if self.initialized[278] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdffile));
            }
            if self.initialized[277] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfmemptr));
            }
            if self.initialized[276] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfmem));
            }
            if self.initialized[275] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pdfmemsize));
            }
            if self.initialized[274] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curs));
            }
            if self.initialized[273] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).dvif));
            }
            if self.initialized[272] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curv));
            }
            if self.initialized[271] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curh));
            }
            if self.initialized[270] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).dviv));
            }
            if self.initialized[269] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).dvih));
            }
            if self.initialized[268] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).rightptr));
            }
            if self.initialized[267] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).downptr));
            }
            if self.initialized[266] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).dvigone));
            }
            if self.initialized[265] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).dvioffset));
            }
            if self.initialized[264] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).dviptr));
            }
            if self.initialized[263] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).dvilimit));
            }
            if self.initialized[262] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).halfbuf));
            }
            if self.initialized[261] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).dvibuf));
            }
            if self.initialized[260] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).lr));
            }
            if self.initialized[259] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).lq));
            }
            if self.initialized[258] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).g));
            }
            if self.initialized[257] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).rulewd));
            }
            if self.initialized[256] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).ruledp));
            }
            if self.initialized[255] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).ruleht));
            }
            if self.initialized[254] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).f));
            }
            if self.initialized[253] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).c));
            }
            if self.initialized[252] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).doingleaders));
            }
            if self.initialized[251] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).deadcycles));
            }
            if self.initialized[250] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).lastbop));
            }
            if self.initialized[249] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).maxpush));
            }
            if self.initialized[248] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).maxh));
            }
            if self.initialized[247] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).maxv));
            }
            if self.initialized[246] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).totalpages));
            }
            if self.initialized[245] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).nullcharacter));
            }
            if self.initialized[244] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).parambase));
            }
            if self.initialized[243] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).extenbase));
            }
            if self.initialized[242] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).kernbase));
            }
            if self.initialized[241] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).ligkernbase));
            }
            if self.initialized[240] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).italicbase));
            }
            if self.initialized[239] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).depthbase));
            }
            if self.initialized[238] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).heightbase));
            }
            if self.initialized[237] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).widthbase));
            }
            if self.initialized[236] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).charbase));
            }
            if self.initialized[235] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fontfalsebchar));
            }
            if self.initialized[234] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fontbchar));
            }
            if self.initialized[233] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).bcharlabel));
            }
            if self.initialized[232] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).skewchar));
            }
            if self.initialized[231] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hyphenchar));
            }
            if self.initialized[230] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fontused));
            }
            if self.initialized[229] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fontglue));
            }
            if self.initialized[228] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fontec));
            }
            if self.initialized[227] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fontbc));
            }
            if self.initialized[226] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fontarea));
            }
            if self.initialized[225] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fontname));
            }
            if self.initialized[224] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fontparams));
            }
            if self.initialized[223] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fontdsize));
            }
            if self.initialized[222] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fontsize));
            }
            if self.initialized[221] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fontcheck));
            }
            if self.initialized[220] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fontptr));
            }
            if self.initialized[219] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fmemptr));
            }
            if self.initialized[218] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fontinfo));
            }
            if self.initialized[217] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).tfmfile));
            }
            if self.initialized[216] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).texmflogname));
            }
            if self.initialized[215] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).outputfilename));
            }
            if self.initialized[214] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).dvifile));
            }
            if self.initialized[213] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).logopened));
            }
            if self.initialized[212] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).jobname));
            }
            if self.initialized[211] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).nameinprogress));
            }
            if self.initialized[210] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).TEXformatdefault
                ));
            }
            if self.initialized[209] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).formatdefaultlength
                ));
            }
            if self.initialized[208] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).extdelimiter));
            }
            if self.initialized[207] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).areadelimiter));
            }
            if self.initialized[206] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curext));
            }
            if self.initialized[205] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curarea));
            }
            if self.initialized[204] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curname));
            }
            if self.initialized[203] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).skipline));
            }
            if self.initialized[202] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).ifline));
            }
            if self.initialized[201] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curif));
            }
            if self.initialized[200] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).iflimit));
            }
            if self.initialized[199] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).condptr));
            }
            if self.initialized[198] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).readopen));
            }
            if self.initialized[197] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).readfile));
            }
            if self.initialized[196] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curorder));
            }
            if self.initialized[195] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).radix));
            }
            if self.initialized[194] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curvallevel));
            }
            if self.initialized[193] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curval));
            }
            if self.initialized[192] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).pstack));
            }
            if self.initialized[191] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).longstate));
            }
            if self.initialized[190] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curmark));
            }
            if self.initialized[189] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).isincsname));
            }
            if self.initialized[188] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).forceeof));
            }
            if self.initialized[187] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).partoken));
            }
            if self.initialized[186] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).parloc));
            }
            if self.initialized[185] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).baseptr));
            }
            if self.initialized[184] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).alignstate));
            }
            if self.initialized[183] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).maxparamstack));
            }
            if self.initialized[182] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).paramptr));
            }
            if self.initialized[181] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).paramstack));
            }
            if self.initialized[180] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).defref));
            }
            if self.initialized[179] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).warningindex));
            }
            if self.initialized[178] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).scannerstatus));
            }
            if self.initialized[177] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).fullsourcefilenamestack
                ));
            }
            if self.initialized[176] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).sourcefilenamestack
                ));
            }
            if self.initialized[175] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).linestack));
            }
            if self.initialized[174] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).line));
            }
            if self.initialized[173] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).inputfile));
            }
            if self.initialized[172] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).openparens));
            }
            if self.initialized[171] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).inopen));
            }
            if self.initialized[170] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curinput));
            }
            if self.initialized[169] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).maxinstack));
            }
            if self.initialized[168] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).inputptr));
            }
            if self.initialized[167] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).inputstack));
            }
            if self.initialized[166] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curtok));
            }
            if self.initialized[165] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curcs));
            }
            if self.initialized[164] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curchr));
            }
            if self.initialized[163] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curcmd));
            }
            if self.initialized[162] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).magset));
            }
            if self.initialized[161] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curboundary));
            }
            if self.initialized[160] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curgroup));
            }
            if self.initialized[159] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curlevel));
            }
            if self.initialized[158] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).maxsavestack));
            }
            if self.initialized[157] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).saveptr));
            }
            if self.initialized[156] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).savestack));
            }
            if self.initialized[155] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).primused));
            }
            if self.initialized[154] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).prim));
            }
            if self.initialized[153] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).cscount));
            }
            if self.initialized[152] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).nonewcontrolsequence
                ));
            }
            if self.initialized[151] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hashhigh));
            }
            if self.initialized[150] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).eqtbtop));
            }
            if self.initialized[149] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hashtop));
            }
            if self.initialized[148] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hashextra));
            }
            if self.initialized[147] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hashused));
            }
            if self.initialized[146] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).yhash));
            }
            if self.initialized[145] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hash));
            }
            if self.initialized[144] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).zzzaa));
            }
            if self.initialized[143] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).zeqtb));
            }
            if self.initialized[142] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).sysyear));
            }
            if self.initialized[141] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).sysmonth));
            }
            if self.initialized[140] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).sysday));
            }
            if self.initialized[139] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).systime));
            }
            if self.initialized[138] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).oldselectorignorederr
                ));
            }
            if self.initialized[137] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).oldsetting));
            }
            if self.initialized[136] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).prevtail));
            }
            if self.initialized[135] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).savetail));
            }
            if self.initialized[134] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).shownmode));
            }
            if self.initialized[133] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).curlist));
            }
            if self.initialized[132] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).maxneststack));
            }
            if self.initialized[131] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).nestptr));
            }
            if self.initialized[130] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).nest));
            }
            if self.initialized[129] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).breadthmax));
            }
            if self.initialized[128] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).depththreshold));
            }
            if self.initialized[127] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).fontinshortdisplay
                ));
            }
            if self.initialized[126] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).rover));
            }
            if self.initialized[125] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).memend));
            }
            if self.initialized[124] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).avail));
            }
            if self.initialized[123] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).dynused));
            }
            if self.initialized[122] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).varused));
            }
            if self.initialized[121] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).himemmin));
            }
            if self.initialized[120] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).lomemmax));
            }
            if self.initialized[119] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).zmem));
            }
            if self.initialized[118] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).yzmem));
            }
            if self.initialized[117] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).tempptr));
            }
            if self.initialized[116] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).speclog));
            }
            if self.initialized[115] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).twotothe));
            }
            if self.initialized[114] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).randomseed));
            }
            if self.initialized[113] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).jrandom));
            }
            if self.initialized[112] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).randoms));
            }
            if self.initialized[111] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).texremainder));
            }
            if self.initialized[110] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).aritherror));
            }
            if self.initialized[109] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).savearitherror));
            }
            if self.initialized[108] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).OKtointerrupt));
            }
            if self.initialized[107] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).interrupt));
            }
            if self.initialized[106] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).useerrhelp));
            }
            if self.initialized[105] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).helpptr));
            }
            if self.initialized[104] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).helpline));
            }
            if self.initialized[103] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).errorcount));
            }
            if self.initialized[102] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).history));
            }
            if self.initialized[101] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).setboxallowed));
            }
            if self.initialized[100] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).deletionsallowed
                ));
            }
            if self.initialized[99] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).interactionoption
                ));
            }
            if self.initialized[98] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).interaction));
            }
            if self.initialized[97] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).firstcount));
            }
            if self.initialized[96] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).trickcount));
            }
            if self.initialized[95] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).trickbuf));
            }
            if self.initialized[94] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fileoffset));
            }
            if self.initialized[93] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).termoffset));
            }
            if self.initialized[92] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).tally));
            }
            if self.initialized[91] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).dig));
            }
            if self.initialized[90] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).selector));
            }
            if self.initialized[89] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).logfile));
            }
            if self.initialized[88] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).poolfile));
            }
            if self.initialized[87] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).initstrptr));
            }
            if self.initialized[86] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).initpoolptr));
            }
            if self.initialized[85] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).strptr));
            }
            if self.initialized[84] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).poolptr));
            }
            if self.initialized[83] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).strstart));
            }
            if self.initialized[82] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).strpool));
            }
            if self.initialized[81] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).insertsrcspecialeverydisplay
                ));
            }
            if self.initialized[80] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).insertsrcspecialeveryvbox
                ));
            }
            if self.initialized[79] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).insertsrcspecialeveryhbox
                ));
            }
            if self.initialized[78] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).insertsrcspecialeverymath
                ));
            }
            if self.initialized[77] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).insertsrcspecialeverycr
                ));
            }
            if self.initialized[76] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).insertsrcspecialeveryparend
                ));
            }
            if self.initialized[75] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).insertsrcspecialeverypar
                ));
            }
            if self.initialized[74] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).insertsrcspecialauto
                ));
            }
            if self.initialized[73] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).srcspecialsp));
            }
            if self.initialized[72] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).quotedfilename));
            }
            if self.initialized[71] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).haltingonerrorp));
            }
            if self.initialized[70] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).haltonerrorp));
            }
            if self.initialized[69] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).eightbitp));
            }
            if self.initialized[68] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).filelineerrorstylep
                ));
            }
            if self.initialized[67] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).parsefirstlinep));
            }
            if self.initialized[66] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).expanddepth));
            }
            if self.initialized[65] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).dvibufsize));
            }
            if self.initialized[64] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).savesize));
            }
            if self.initialized[63] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).nestsize));
            }
            if self.initialized[62] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).paramsize));
            }
            if self.initialized[61] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).maxinopen));
            }
            if self.initialized[60] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).stacksize));
            }
            if self.initialized[59] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).bufsize));
            }
            if self.initialized[58] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).triesize));
            }
            if self.initialized[57] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).hyphsize));
            }
            if self.initialized[56] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fontk));
            }
            if self.initialized[55] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fontmax));
            }
            if self.initialized[54] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fontmemsize));
            }
            if self.initialized[53] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).poolfree));
            }
            if self.initialized[52] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).poolsize));
            }
            if self.initialized[51] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).stringvacancies));
            }
            if self.initialized[50] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).stringsfree));
            }
            if self.initialized[49] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).maxstrings));
            }
            if self.initialized[48] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).maxprintline));
            }
            if self.initialized[47] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).halferrorline));
            }
            if self.initialized[46] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).errorline));
            }
            if self.initialized[45] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).memmax));
            }
            if self.initialized[44] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).extramemtop));
            }
            if self.initialized[43] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).memtop));
            }
            if self.initialized[42] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).memmin));
            }
            if self.initialized[41] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).extramembot));
            }
            if self.initialized[40] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mainmemory));
            }
            if self.initialized[39] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).membot));
            }
            if self.initialized[38] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).boundname));
            }
            if self.initialized[37] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).bounddefault));
            }
            if self.initialized[36] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).dump_name));
            }
            if self.initialized[35] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).dumpline));
            }
            if self.initialized[34] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).dumpoption));
            }
            if self.initialized[33] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).iniversion));
            }
            if self.initialized[32] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).maxbufstack));
            }
            if self.initialized[31] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).last));
            }
            if self.initialized[30] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).first));
            }
            if self.initialized[29] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).buffer));
            }
            if self.initialized[28] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).namelength));
            }
            if self.initialized[27] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).nameoffile));
            }
            if self.initialized[26] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).messageprinting));
            }
            if self.initialized[25] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).specialprinting));
            }
            if self.initialized[24] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).csconverting));
            }
            if self.initialized[23] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).writenoexpanding
                ));
            }
            if self.initialized[22] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).activenoconvert));
            }
            if self.initialized[21] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).noconvert));
            }
            if self.initialized[20] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).specsout));
            }
            if self.initialized[19] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mubyteslog));
            }
            if self.initialized[18] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mubytesout));
            }
            if self.initialized[17] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mubytestoken));
            }
            if self.initialized[16] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mubytetoken));
            }
            if self.initialized[15] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mubytesstart));
            }
            if self.initialized[14] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mubytestart));
            }
            if self.initialized[13] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mubyterelax));
            }
            if self.initialized[12] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mubytetableout));
            }
            if self.initialized[11] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mubytetablein));
            }
            if self.initialized[10] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mubyteprefix));
            }
            if self.initialized[9] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mubyteskeep));
            }
            if self.initialized[8] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mubytekeep));
            }
            if self.initialized[7] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mubyteskip));
            }
            if self.initialized[6] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mubytecswrite));
            }
            if self.initialized[5] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mubytewrite));
            }
            if self.initialized[4] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).mubyteread));
            }
            if self.initialized[3] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).xprn));
            }
            if self.initialized[2] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).xchr));
            }
            if self.initialized[1] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).xord));
            }
            if self.initialized[0] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).bad));
            }
        }
    }
}
impl StateInitialization {
    pub(crate) unsafe fn begin(destination: *mut State) -> Self {
        Self {
            destination,
            initialized: [false; 709],
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
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).bad).write(0);
        self.initialized[0] = true;
        const {
            assert!((256) == (256));
            assert!(core::mem::size_of::<ASCIIcode>() <= 16 * 1024);
        }
        __tekai_require_copy::<ASCIIcode>();
        {
            let __tekai_repeat_0: ASCIIcode = 0;
            for __tekai_index_0 in 0..(256) {
                core::ptr::addr_of_mut!((*destination).xord[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[1] = true;
        const {
            assert!((256) == (256));
            assert!(core::mem::size_of::<ASCIIcode>() <= 16 * 1024);
        }
        __tekai_require_copy::<ASCIIcode>();
        {
            let __tekai_repeat_0: ASCIIcode = 0;
            for __tekai_index_0 in 0..(256) {
                core::ptr::addr_of_mut!((*destination).xchr[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[2] = true;
        const {
            assert!((256) == (256));
            assert!(core::mem::size_of::<ASCIIcode>() <= 16 * 1024);
        }
        __tekai_require_copy::<ASCIIcode>();
        {
            let __tekai_repeat_0: ASCIIcode = 0;
            for __tekai_index_0 in 0..(256) {
                core::ptr::addr_of_mut!((*destination).xprn[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[3] = true;
        const {
            assert!((256) == (256));
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        __tekai_require_copy::<halfword>();
        {
            let __tekai_repeat_0: halfword = 0;
            for __tekai_index_0 in 0..(256) {
                core::ptr::addr_of_mut!((*destination).mubyteread[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[4] = true;
        const {
            assert!((256) == (256));
            assert!(core::mem::size_of::<strnumber>() <= 16 * 1024);
        }
        __tekai_require_copy::<strnumber>();
        {
            let __tekai_repeat_0: strnumber = 0;
            for __tekai_index_0 in 0..(256) {
                core::ptr::addr_of_mut!((*destination).mubytewrite[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[5] = true;
        const {
            assert!((128) == (128));
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        __tekai_require_copy::<halfword>();
        {
            let __tekai_repeat_0: halfword = 0;
            for __tekai_index_0 in 0..(128) {
                core::ptr::addr_of_mut!((*destination).mubytecswrite[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[6] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mubyteskip).write(0);
        self.initialized[7] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mubytekeep).write(0);
        self.initialized[8] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mubyteskeep).write(0);
        self.initialized[9] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mubyteprefix).write(0);
        self.initialized[10] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mubytetablein).write(0);
        self.initialized[11] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mubytetableout).write(0);
        self.initialized[12] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mubyterelax).write(0);
        self.initialized[13] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mubytestart).write(0);
        self.initialized[14] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mubytesstart).write(0);
        self.initialized[15] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mubytetoken).write(0);
        self.initialized[16] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mubytestoken).write(0);
        self.initialized[17] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mubytesout).write(0);
        self.initialized[18] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mubyteslog).write(0);
        self.initialized[19] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).specsout).write(0);
        self.initialized[20] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).noconvert).write(0);
        self.initialized[21] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).activenoconvert).write(0);
        self.initialized[22] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).writenoexpanding).write(0);
        self.initialized[23] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).csconverting).write(0);
        self.initialized[24] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).specialprinting).write(0);
        self.initialized[25] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).messageprinting).write(0);
        self.initialized[26] = true;
        const {
            assert!(core::mem::size_of::<*mut ASCIIcode>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).nameoffile)
            .write(::core::ptr::null::<ASCIIcode>() as *mut ASCIIcode);
        self.initialized[27] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).namelength).write(0);
        self.initialized[28] = true;
        const {
            assert!(core::mem::size_of::<*mut ASCIIcode>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).buffer)
            .write(::core::ptr::null::<ASCIIcode>() as *mut ASCIIcode);
        self.initialized[29] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).first).write(0);
        self.initialized[30] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).last).write(0);
        self.initialized[31] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).maxbufstack).write(0);
        self.initialized[32] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).iniversion).write(0);
        self.initialized[33] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).dumpoption).write(0);
        self.initialized[34] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).dumpline).write(0);
        self.initialized[35] = true;
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).dump_name)
            .write(::core::ptr::null::<::core::ffi::c_char>());
        self.initialized[36] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).bounddefault).write(0);
        self.initialized[37] = true;
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).boundname)
            .write(::core::ptr::null::<::core::ffi::c_char>());
        self.initialized[38] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).membot).write(0);
        self.initialized[39] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mainmemory).write(0);
        self.initialized[40] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).extramembot).write(0);
        self.initialized[41] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).memmin).write(0);
        self.initialized[42] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).memtop).write(0);
        self.initialized[43] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).extramemtop).write(0);
        self.initialized[44] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).memmax).write(0);
        self.initialized[45] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).errorline).write(0);
        self.initialized[46] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).halferrorline).write(0);
        self.initialized[47] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).maxprintline).write(0);
        self.initialized[48] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).maxstrings).write(0);
        self.initialized[49] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).stringsfree).write(0);
        self.initialized[50] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).stringvacancies).write(0);
        self.initialized[51] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolsize).write(0);
        self.initialized[52] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfree).write(0);
        self.initialized[53] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fontmemsize).write(0);
        self.initialized[54] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fontmax).write(0);
        self.initialized[55] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fontk).write(0);
        self.initialized[56] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hyphsize).write(0);
        self.initialized[57] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).triesize).write(0);
        self.initialized[58] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).bufsize).write(0);
        self.initialized[59] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).stacksize).write(0);
        self.initialized[60] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).maxinopen).write(0);
        self.initialized[61] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).paramsize).write(0);
        self.initialized[62] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).nestsize).write(0);
        self.initialized[63] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).savesize).write(0);
        self.initialized[64] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).dvibufsize).write(0);
        self.initialized[65] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).expanddepth).write(0);
        self.initialized[66] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).parsefirstlinep).write(0);
        self.initialized[67] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).filelineerrorstylep).write(0);
        self.initialized[68] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).eightbitp).write(0);
        self.initialized[69] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).haltonerrorp).write(0);
        self.initialized[70] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).haltingonerrorp).write(0);
        self.initialized[71] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).quotedfilename).write(0);
        self.initialized[72] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).srcspecialsp).write(0);
        self.initialized[73] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).insertsrcspecialauto).write(0);
        self.initialized[74] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).insertsrcspecialeverypar).write(0);
        self.initialized[75] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).insertsrcspecialeveryparend).write(0);
        self.initialized[76] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).insertsrcspecialeverycr).write(0);
        self.initialized[77] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).insertsrcspecialeverymath).write(0);
        self.initialized[78] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).insertsrcspecialeveryhbox).write(0);
        self.initialized[79] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).insertsrcspecialeveryvbox).write(0);
        self.initialized[80] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).insertsrcspecialeverydisplay).write(0);
        self.initialized[81] = true;
        const {
            assert!(core::mem::size_of::<*mut packedASCIIcode>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).strpool)
            .write(::core::ptr::null::<packedASCIIcode>() as *mut packedASCIIcode);
        self.initialized[82] = true;
        const {
            assert!(core::mem::size_of::<*mut poolpointer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).strstart)
            .write(::core::ptr::null::<poolpointer>() as *mut poolpointer);
        self.initialized[83] = true;
        const {
            assert!(core::mem::size_of::<poolpointer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolptr).write(0);
        self.initialized[84] = true;
        const {
            assert!(core::mem::size_of::<strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).strptr).write(0);
        self.initialized[85] = true;
        const {
            assert!(core::mem::size_of::<poolpointer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).initpoolptr).write(0);
        self.initialized[86] = true;
        const {
            assert!(core::mem::size_of::<strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).initstrptr).write(0);
        self.initialized[87] = true;
        const {
            assert!(core::mem::size_of::<alphafile>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfile)
            .write(::core::ptr::null::<FILE>() as *mut FILE);
        self.initialized[88] = true;
        const {
            assert!(core::mem::size_of::<alphafile>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).logfile)
            .write(::core::ptr::null::<FILE>() as *mut FILE);
        self.initialized[89] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).selector).write(0);
        self.initialized[90] = true;
        const {
            assert!((23) == (23));
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        __tekai_require_copy::<::core::ffi::c_uchar>();
        {
            let __tekai_repeat_0: ::core::ffi::c_uchar = 0;
            for __tekai_index_0 in 0..(23) {
                core::ptr::addr_of_mut!((*destination).dig[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[91] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).tally).write(0);
        self.initialized[92] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).termoffset).write(0);
        self.initialized[93] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fileoffset).write(0);
        self.initialized[94] = true;
        const {
            assert!((256) == (256));
            assert!(core::mem::size_of::<ASCIIcode>() <= 16 * 1024);
        }
        __tekai_require_copy::<ASCIIcode>();
        {
            let __tekai_repeat_0: ASCIIcode = 0;
            for __tekai_index_0 in 0..(256) {
                core::ptr::addr_of_mut!((*destination).trickbuf[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[95] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).trickcount).write(0);
        self.initialized[96] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).firstcount).write(0);
        self.initialized[97] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).interaction).write(0);
        self.initialized[98] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).interactionoption).write(0);
        self.initialized[99] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).deletionsallowed).write(0);
        self.initialized[100] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).setboxallowed).write(0);
        self.initialized[101] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).history).write(0);
        self.initialized[102] = true;
        const {
            assert!(core::mem::size_of::<schar>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).errorcount).write(0);
        self.initialized[103] = true;
        const {
            assert!((6) == (6));
            assert!(core::mem::size_of::<strnumber>() <= 16 * 1024);
        }
        __tekai_require_copy::<strnumber>();
        {
            let __tekai_repeat_0: strnumber = 0;
            for __tekai_index_0 in 0..(6) {
                core::ptr::addr_of_mut!((*destination).helpline[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[104] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).helpptr).write(0);
        self.initialized[105] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).useerrhelp).write(0);
        self.initialized[106] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).interrupt).write(0);
        self.initialized[107] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).OKtointerrupt).write(0);
        self.initialized[108] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).savearitherror).write(0);
        self.initialized[109] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).aritherror).write(0);
        self.initialized[110] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).texremainder).write(0);
        self.initialized[111] = true;
        const {
            assert!((55) == (55));
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        __tekai_require_copy::<integer>();
        {
            let __tekai_repeat_0: integer = 0;
            for __tekai_index_0 in 0..(55) {
                core::ptr::addr_of_mut!((*destination).randoms[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[112] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).jrandom).write(0);
        self.initialized[113] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).randomseed).write(0);
        self.initialized[114] = true;
        const {
            assert!((31) == (31));
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        __tekai_require_copy::<integer>();
        {
            let __tekai_repeat_0: integer = 0;
            for __tekai_index_0 in 0..(31) {
                core::ptr::addr_of_mut!((*destination).twotothe[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[115] = true;
        const {
            assert!((29) == (29));
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        __tekai_require_copy::<integer>();
        {
            let __tekai_repeat_0: integer = 0;
            for __tekai_index_0 in 0..(29) {
                core::ptr::addr_of_mut!((*destination).speclog[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[116] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).tempptr).write(0);
        self.initialized[117] = true;
        const {
            assert!(core::mem::size_of::<*mut memoryword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).yzmem)
            .write(::core::ptr::null::<memoryword>() as *mut memoryword);
        self.initialized[118] = true;
        const {
            assert!(core::mem::size_of::<*mut memoryword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).zmem)
            .write(::core::ptr::null::<memoryword>() as *mut memoryword);
        self.initialized[119] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).lomemmax).write(0);
        self.initialized[120] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).himemmin).write(0);
        self.initialized[121] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).varused).write(0);
        self.initialized[122] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).dynused).write(0);
        self.initialized[123] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).avail).write(0);
        self.initialized[124] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).memend).write(0);
        self.initialized[125] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).rover).write(0);
        self.initialized[126] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fontinshortdisplay).write(0);
        self.initialized[127] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).depththreshold).write(0);
        self.initialized[128] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).breadthmax).write(0);
        self.initialized[129] = true;
        const {
            assert!(core::mem::size_of::<*mut liststaterecord>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).nest)
            .write(::core::ptr::null::<liststaterecord>() as *mut liststaterecord);
        self.initialized[130] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).nestptr).write(0);
        self.initialized[131] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).maxneststack).write(0);
        self.initialized[132] = true;
        const {
            assert!(core::mem::size_of::<liststaterecord>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curlist).write(liststaterecord {
            modefield: 0,
            headfield: 0,
            tailfield: 0,
            eTeXauxfield: 0,
            pgfield: 0,
            mlfield: 0,
            auxfield: memoryword { gr: 0. },
        });
        self.initialized[133] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_short>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).shownmode).write(0);
        self.initialized[134] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).savetail).write(0);
        self.initialized[135] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).prevtail).write(0);
        self.initialized[136] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).oldsetting).write(0);
        self.initialized[137] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).oldselectorignorederr).write(0);
        self.initialized[138] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).systime).write(0);
        self.initialized[139] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).sysday).write(0);
        self.initialized[140] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).sysmonth).write(0);
        self.initialized[141] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).sysyear).write(0);
        self.initialized[142] = true;
        const {
            assert!(core::mem::size_of::<*mut memoryword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).zeqtb)
            .write(::core::ptr::null::<memoryword>() as *mut memoryword);
        self.initialized[143] = true;
        const {
            assert!((916) == (916));
            assert!(core::mem::size_of::<quarterword>() <= 16 * 1024);
        }
        __tekai_require_copy::<quarterword>();
        {
            let __tekai_repeat_0: quarterword = 0;
            for __tekai_index_0 in 0..(916) {
                core::ptr::addr_of_mut!((*destination).zzzaa[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[144] = true;
        const {
            assert!(core::mem::size_of::<*mut twohalves>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hash)
            .write(::core::ptr::null::<twohalves>() as *mut twohalves);
        self.initialized[145] = true;
        const {
            assert!(core::mem::size_of::<*mut twohalves>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).yhash)
            .write(::core::ptr::null::<twohalves>() as *mut twohalves);
        self.initialized[146] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hashused).write(0);
        self.initialized[147] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hashextra).write(0);
        self.initialized[148] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hashtop).write(0);
        self.initialized[149] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).eqtbtop).write(0);
        self.initialized[150] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hashhigh).write(0);
        self.initialized[151] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).nonewcontrolsequence).write(0);
        self.initialized[152] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cscount).write(0);
        self.initialized[153] = true;
        const {
            assert!((2101) == (2101));
            assert!(core::mem::size_of::<twohalves>() <= 16 * 1024);
        }
        __tekai_require_copy::<twohalves>();
        {
            let __tekai_repeat_0: twohalves = twohalves {
                v: C2RustUnnamed_1 { LH: 0, RH: 0 },
            };
            for __tekai_index_0 in 0..(2101) {
                core::ptr::addr_of_mut!((*destination).prim[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[154] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).primused).write(0);
        self.initialized[155] = true;
        const {
            assert!(core::mem::size_of::<*mut memoryword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).savestack)
            .write(::core::ptr::null::<memoryword>() as *mut memoryword);
        self.initialized[156] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).saveptr).write(0);
        self.initialized[157] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).maxsavestack).write(0);
        self.initialized[158] = true;
        const {
            assert!(core::mem::size_of::<quarterword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curlevel).write(0);
        self.initialized[159] = true;
        const {
            assert!(core::mem::size_of::<groupcode>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curgroup).write(0);
        self.initialized[160] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curboundary).write(0);
        self.initialized[161] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).magset).write(0);
        self.initialized[162] = true;
        const {
            assert!(core::mem::size_of::<eightbits>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curcmd).write(0);
        self.initialized[163] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curchr).write(0);
        self.initialized[164] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curcs).write(0);
        self.initialized[165] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curtok).write(0);
        self.initialized[166] = true;
        const {
            assert!(core::mem::size_of::<*mut instaterecord>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).inputstack)
            .write(::core::ptr::null::<instaterecord>() as *mut instaterecord);
        self.initialized[167] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).inputptr).write(0);
        self.initialized[168] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).maxinstack).write(0);
        self.initialized[169] = true;
        const {
            assert!(core::mem::size_of::<instaterecord>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curinput).write(instaterecord {
            statefield: 0,
            indexfield: 0,
            startfield: 0,
            locfield: 0,
            limitfield: 0,
            namefield: 0,
            synctextagfield: 0,
        });
        self.initialized[170] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).inopen).write(0);
        self.initialized[171] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).openparens).write(0);
        self.initialized[172] = true;
        const {
            assert!(core::mem::size_of::<*mut alphafile>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).inputfile)
            .write(::core::ptr::null::<alphafile>() as *mut alphafile);
        self.initialized[173] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).line).write(0);
        self.initialized[174] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).linestack)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[175] = true;
        const {
            assert!(core::mem::size_of::<*mut strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).sourcefilenamestack)
            .write(::core::ptr::null::<strnumber>() as *mut strnumber);
        self.initialized[176] = true;
        const {
            assert!(core::mem::size_of::<*mut strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fullsourcefilenamestack)
            .write(::core::ptr::null::<strnumber>() as *mut strnumber);
        self.initialized[177] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).scannerstatus).write(0);
        self.initialized[178] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).warningindex).write(0);
        self.initialized[179] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).defref).write(0);
        self.initialized[180] = true;
        const {
            assert!(core::mem::size_of::<*mut halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).paramstack)
            .write(::core::ptr::null::<halfword>() as *mut halfword);
        self.initialized[181] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).paramptr).write(0);
        self.initialized[182] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).maxparamstack).write(0);
        self.initialized[183] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).alignstate).write(0);
        self.initialized[184] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).baseptr).write(0);
        self.initialized[185] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).parloc).write(0);
        self.initialized[186] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).partoken).write(0);
        self.initialized[187] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).forceeof).write(0);
        self.initialized[188] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).isincsname).write(0);
        self.initialized[189] = true;
        const {
            assert!((5) == (5));
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        __tekai_require_copy::<halfword>();
        {
            let __tekai_repeat_0: halfword = 0;
            for __tekai_index_0 in 0..(5) {
                core::ptr::addr_of_mut!((*destination).curmark[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[190] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).longstate).write(0);
        self.initialized[191] = true;
        const {
            assert!((9) == (9));
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        __tekai_require_copy::<halfword>();
        {
            let __tekai_repeat_0: halfword = 0;
            for __tekai_index_0 in 0..(9) {
                core::ptr::addr_of_mut!((*destination).pstack[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[192] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curval).write(0);
        self.initialized[193] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curvallevel).write(0);
        self.initialized[194] = true;
        const {
            assert!(core::mem::size_of::<smallnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).radix).write(0);
        self.initialized[195] = true;
        const {
            assert!(core::mem::size_of::<glueord>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curorder).write(0);
        self.initialized[196] = true;
        const {
            assert!((16) == (16));
            assert!(core::mem::size_of::<alphafile>() <= 16 * 1024);
        }
        __tekai_require_copy::<alphafile>();
        {
            let __tekai_repeat_0: alphafile = ::core::ptr::null::<FILE>() as *mut FILE;
            for __tekai_index_0 in 0..(16) {
                core::ptr::addr_of_mut!((*destination).readfile[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[197] = true;
        const {
            assert!((17) == (17));
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        __tekai_require_copy::<::core::ffi::c_uchar>();
        {
            let __tekai_repeat_0: ::core::ffi::c_uchar = 0;
            for __tekai_index_0 in 0..(17) {
                core::ptr::addr_of_mut!((*destination).readopen[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[198] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).condptr).write(0);
        self.initialized[199] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).iflimit).write(0);
        self.initialized[200] = true;
        const {
            assert!(core::mem::size_of::<smallnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curif).write(0);
        self.initialized[201] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).ifline).write(0);
        self.initialized[202] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).skipline).write(0);
        self.initialized[203] = true;
        const {
            assert!(core::mem::size_of::<strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curname).write(0);
        self.initialized[204] = true;
        const {
            assert!(core::mem::size_of::<strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curarea).write(0);
        self.initialized[205] = true;
        const {
            assert!(core::mem::size_of::<strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curext).write(0);
        self.initialized[206] = true;
        const {
            assert!(core::mem::size_of::<poolpointer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).areadelimiter).write(0);
        self.initialized[207] = true;
        const {
            assert!(core::mem::size_of::<poolpointer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).extdelimiter).write(0);
        self.initialized[208] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).formatdefaultlength).write(0);
        self.initialized[209] = true;
        const {
            assert!(core::mem::size_of::<string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).TEXformatdefault)
            .write(::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char);
        self.initialized[210] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).nameinprogress).write(0);
        self.initialized[211] = true;
        const {
            assert!(core::mem::size_of::<strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).jobname).write(0);
        self.initialized[212] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).logopened).write(0);
        self.initialized[213] = true;
        const {
            assert!(core::mem::size_of::<bytefile>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).dvifile)
            .write(::core::ptr::null::<FILE>() as *mut FILE);
        self.initialized[214] = true;
        const {
            assert!(core::mem::size_of::<strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).outputfilename).write(0);
        self.initialized[215] = true;
        const {
            assert!(core::mem::size_of::<strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).texmflogname).write(0);
        self.initialized[216] = true;
        const {
            assert!(core::mem::size_of::<bytefile>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).tfmfile)
            .write(::core::ptr::null::<FILE>() as *mut FILE);
        self.initialized[217] = true;
        const {
            assert!(core::mem::size_of::<*mut fmemoryword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fontinfo)
            .write(::core::ptr::null::<fmemoryword>() as *mut fmemoryword);
        self.initialized[218] = true;
        const {
            assert!(core::mem::size_of::<fontindex>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fmemptr).write(0);
        self.initialized[219] = true;
        const {
            assert!(core::mem::size_of::<internalfontnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fontptr).write(0);
        self.initialized[220] = true;
        const {
            assert!(core::mem::size_of::<*mut fourquarters>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fontcheck)
            .write(::core::ptr::null::<fourquarters>() as *mut fourquarters);
        self.initialized[221] = true;
        const {
            assert!(core::mem::size_of::<*mut scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fontsize)
            .write(::core::ptr::null::<scaled>() as *mut scaled);
        self.initialized[222] = true;
        const {
            assert!(core::mem::size_of::<*mut scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fontdsize)
            .write(::core::ptr::null::<scaled>() as *mut scaled);
        self.initialized[223] = true;
        const {
            assert!(core::mem::size_of::<*mut fontindex>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fontparams)
            .write(::core::ptr::null::<fontindex>() as *mut fontindex);
        self.initialized[224] = true;
        const {
            assert!(core::mem::size_of::<*mut strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fontname)
            .write(::core::ptr::null::<strnumber>() as *mut strnumber);
        self.initialized[225] = true;
        const {
            assert!(core::mem::size_of::<*mut strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fontarea)
            .write(::core::ptr::null::<strnumber>() as *mut strnumber);
        self.initialized[226] = true;
        const {
            assert!(core::mem::size_of::<*mut eightbits>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fontbc)
            .write(::core::ptr::null::<eightbits>() as *mut eightbits);
        self.initialized[227] = true;
        const {
            assert!(core::mem::size_of::<*mut eightbits>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fontec)
            .write(::core::ptr::null::<eightbits>() as *mut eightbits);
        self.initialized[228] = true;
        const {
            assert!(core::mem::size_of::<*mut halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fontglue)
            .write(::core::ptr::null::<halfword>() as *mut halfword);
        self.initialized[229] = true;
        const {
            assert!(core::mem::size_of::<*mut boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fontused)
            .write(::core::ptr::null::<boolean>() as *mut boolean);
        self.initialized[230] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hyphenchar)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[231] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).skewchar)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[232] = true;
        const {
            assert!(core::mem::size_of::<*mut fontindex>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).bcharlabel)
            .write(::core::ptr::null::<fontindex>() as *mut fontindex);
        self.initialized[233] = true;
        const {
            assert!(core::mem::size_of::<*mut ninebits>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fontbchar)
            .write(::core::ptr::null::<ninebits>() as *mut ninebits);
        self.initialized[234] = true;
        const {
            assert!(core::mem::size_of::<*mut ninebits>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fontfalsebchar)
            .write(::core::ptr::null::<ninebits>() as *mut ninebits);
        self.initialized[235] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).charbase)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[236] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).widthbase)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[237] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).heightbase)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[238] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).depthbase)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[239] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).italicbase)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[240] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).ligkernbase)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[241] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).kernbase)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[242] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).extenbase)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[243] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).parambase)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[244] = true;
        const {
            assert!(core::mem::size_of::<fourquarters>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).nullcharacter).write(fourquarters {
            u: C2RustUnnamed_2 {
                B3: 0,
                B2: 0,
                B1: 0,
                B0: 0,
            },
        });
        self.initialized[245] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).totalpages).write(0);
        self.initialized[246] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).maxv).write(0);
        self.initialized[247] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).maxh).write(0);
        self.initialized[248] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).maxpush).write(0);
        self.initialized[249] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).lastbop).write(0);
        self.initialized[250] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).deadcycles).write(0);
        self.initialized[251] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).doingleaders).write(0);
        self.initialized[252] = true;
        const {
            assert!(core::mem::size_of::<quarterword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).c).write(0);
        self.initialized[253] = true;
        const {
            assert!(core::mem::size_of::<internalfontnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).f).write(0);
        self.initialized[254] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).ruleht).write(0);
        self.initialized[255] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).ruledp).write(0);
        self.initialized[256] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).rulewd).write(0);
        self.initialized[257] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).g).write(0);
        self.initialized[258] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).lq).write(0);
        self.initialized[259] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).lr).write(0);
        self.initialized[260] = true;
        const {
            assert!(core::mem::size_of::<*mut eightbits>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).dvibuf)
            .write(::core::ptr::null::<eightbits>() as *mut eightbits);
        self.initialized[261] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).halfbuf).write(0);
        self.initialized[262] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).dvilimit).write(0);
        self.initialized[263] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).dviptr).write(0);
        self.initialized[264] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).dvioffset).write(0);
        self.initialized[265] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).dvigone).write(0);
        self.initialized[266] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).downptr).write(0);
        self.initialized[267] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).rightptr).write(0);
        self.initialized[268] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).dvih).write(0);
        self.initialized[269] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).dviv).write(0);
        self.initialized[270] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curh).write(0);
        self.initialized[271] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curv).write(0);
        self.initialized[272] = true;
        const {
            assert!(core::mem::size_of::<internalfontnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).dvif).write(0);
        self.initialized[273] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curs).write(0);
        self.initialized[274] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfmemsize).write(0);
        self.initialized[275] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfmem)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[276] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfmemptr).write(0);
        self.initialized[277] = true;
        const {
            assert!(core::mem::size_of::<bytefile>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffile)
            .write(::core::ptr::null::<FILE>() as *mut FILE);
        self.initialized[278] = true;
        const {
            assert!(core::mem::size_of::<*mut eightbits>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfbuf)
            .write(::core::ptr::null::<eightbits>() as *mut eightbits);
        self.initialized[279] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfbufsize).write(0);
        self.initialized[280] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfptr).write(0);
        self.initialized[281] = true;
        const {
            assert!(core::mem::size_of::<*mut eightbits>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfopbuf)
            .write(::core::ptr::null::<eightbits>() as *mut eightbits);
        self.initialized[282] = true;
        const {
            assert!(core::mem::size_of::<*mut eightbits>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfosbuf)
            .write(::core::ptr::null::<eightbits>() as *mut eightbits);
        self.initialized[283] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfosbufsize).write(0);
        self.initialized[284] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfosobjnum)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[285] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfosobjoff)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[286] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfosobjidx).write(0);
        self.initialized[287] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfoscntr).write(0);
        self.initialized[288] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfopptr).write(0);
        self.initialized[289] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfosptr).write(0);
        self.initialized[290] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfosmode).write(0);
        self.initialized[291] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfosenable).write(0);
        self.initialized[292] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfoscurobjnum).write(0);
        self.initialized[293] = true;
        const {
            assert!(core::mem::size_of::<longinteger>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfgone).write(0);
        self.initialized[294] = true;
        const {
            assert!(core::mem::size_of::<longinteger>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfsaveoffset).write(0);
        self.initialized[295] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).zipwritestate).write(0);
        self.initialized[296] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fixedpdfmajorversion).write(0);
        self.initialized[297] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fixedpdfminorversion).write(0);
        self.initialized[298] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fixedpdfobjcompresslevel).write(0);
        self.initialized[299] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfversionwritten).write(0);
        self.initialized[300] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fixedpdfoutput).write(0);
        self.initialized[301] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fixedpdfoutputset).write(0);
        self.initialized[302] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fixedgamma).write(0);
        self.initialized[303] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fixedimagegamma).write(0);
        self.initialized[304] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fixedimagehicolor).write(0);
        self.initialized[305] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fixedimageapplygamma).write(0);
        self.initialized[306] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).epochseconds).write(0);
        self.initialized[307] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).microseconds).write(0);
        self.initialized[308] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fixedpdfdraftmode).write(0);
        self.initialized[309] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fixedpdfdraftmodeset).write(0);
        self.initialized[310] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfpagegroupval).write(0);
        self.initialized[311] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).onebp).write(0);
        self.initialized[312] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).onehundredbp).write(0);
        self.initialized[313] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).onehundredinch).write(0);
        self.initialized[314] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).oneinch).write(0);
        self.initialized[315] = true;
        const {
            assert!((10) == (10));
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        __tekai_require_copy::<integer>();
        {
            let __tekai_repeat_0: integer = 0;
            for __tekai_index_0 in 0..(10) {
                core::ptr::addr_of_mut!((*destination).tenpow[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[316] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).scaledout).write(0);
        self.initialized[317] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).initpdfoutput).write(0);
        self.initialized[318] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).advcharwidths).write(0);
        self.initialized[319] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).advcharwidthsout).write(0);
        self.initialized[320] = true;
        const {
            assert!(core::mem::size_of::<internalfontnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdff).write(0);
        self.initialized[321] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfh).write(0);
        self.initialized[322] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfv).write(0);
        self.initialized[323] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdftjstarth).write(0);
        self.initialized[324] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curdeltah).write(0);
        self.initialized[325] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfdeltah).write(0);
        self.initialized[326] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdforiginh).write(0);
        self.initialized[327] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdforiginv).write(0);
        self.initialized[328] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfdoingstring).write(0);
        self.initialized[329] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfdoingtext).write(0);
        self.initialized[330] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).minbpval).write(0);
        self.initialized[331] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).minfontval).write(0);
        self.initialized[332] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fixedpkresolution).write(0);
        self.initialized[333] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fixeddecimaldigits).write(0);
        self.initialized[334] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fixedgentounicode).write(0);
        self.initialized[335] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fixedinclusioncopyfont).write(0);
        self.initialized[336] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pkscalefactor).write(0);
        self.initialized[337] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfoutputoption).write(0);
        self.initialized[338] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfoutputvalue).write(0);
        self.initialized[339] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfdraftmodeoption).write(0);
        self.initialized[340] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfdraftmodevalue).write(0);
        self.initialized[341] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfcurTma).write(0);
        self.initialized[342] = true;
        const {
            assert!(core::mem::size_of::<internalfontnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflastf).write(0);
        self.initialized[343] = true;
        const {
            assert!(core::mem::size_of::<internalfontnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflastfs).write(0);
        self.initialized[344] = true;
        const {
            assert!(core::mem::size_of::<internalfontnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfdummyfont).write(0);
        self.initialized[345] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).objtabsize).write(0);
        self.initialized[346] = true;
        const {
            assert!(core::mem::size_of::<*mut objentry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).objtab)
            .write(::core::ptr::null::<objentry>() as *mut objentry);
        self.initialized[347] = true;
        const {
            assert!((11) == (11));
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        __tekai_require_copy::<integer>();
        {
            let __tekai_repeat_0: integer = 0;
            for __tekai_index_0 in 0..(11) {
                core::ptr::addr_of_mut!((*destination).headtab[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[348] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pagestail).write(0);
        self.initialized[349] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).objptr).write(0);
        self.initialized[350] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).sysobjptr).write(0);
        self.initialized[351] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflastpages).write(0);
        self.initialized[352] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflastpage).write(0);
        self.initialized[353] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflaststream).write(0);
        self.initialized[354] = true;
        const {
            assert!(core::mem::size_of::<longinteger>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfstreamlength).write(0);
        self.initialized[355] = true;
        const {
            assert!(core::mem::size_of::<longinteger>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfstreamlengthoffset).write(0);
        self.initialized[356] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfseekwritelength).write(0);
        self.initialized[357] = true;
        const {
            assert!(core::mem::size_of::<eightbits>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflastbyte).write(0);
        self.initialized[358] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfappendlistarg).write(0);
        self.initialized[359] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).ff).write(0);
        self.initialized[360] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfboxspecmedia).write(0);
        self.initialized[361] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfboxspeccrop).write(0);
        self.initialized[362] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfboxspecbleed).write(0);
        self.initialized[363] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfboxspectrim).write(0);
        self.initialized[364] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfboxspecart).write(0);
        self.initialized[365] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfimageprocset).write(0);
        self.initialized[366] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdftextprocset).write(0);
        self.initialized[367] = true;
        const {
            assert!(core::mem::size_of::<*mut eightbits>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffonttype)
            .write(::core::ptr::null::<eightbits>() as *mut eightbits);
        self.initialized[368] = true;
        const {
            assert!(core::mem::size_of::<*mut strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontattr)
            .write(::core::ptr::null::<strnumber>() as *mut strnumber);
        self.initialized[369] = true;
        const {
            assert!(core::mem::size_of::<*mut boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontnobuiltintounicode)
            .write(::core::ptr::null::<boolean>() as *mut boolean);
        self.initialized[370] = true;
        const {
            assert!(core::mem::size_of::<*mut charusedarray>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfcharused)
            .write(::core::ptr::null::<charusedarray>() as *mut charusedarray);
        self.initialized[371] = true;
        const {
            assert!(core::mem::size_of::<*mut scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontsize)
            .write(::core::ptr::null::<scaled>() as *mut scaled);
        self.initialized[372] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontnum)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[373] = true;
        const {
            assert!(core::mem::size_of::<*mut fmentryptr>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontmap)
            .write(::core::ptr::null::<fmentryptr>() as *mut fmentryptr);
        self.initialized[374] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontlist).write(0);
        self.initialized[375] = true;
        const {
            assert!(core::mem::size_of::<strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfresnameprefix).write(0);
        self.initialized[376] = true;
        const {
            assert!(core::mem::size_of::<strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).lasttokensstring).write(0);
        self.initialized[377] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).vfpacketbase)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[378] = true;
        const {
            assert!(core::mem::size_of::<*mut internalfontnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).vfdefaultfont)
            .write(::core::ptr::null::<internalfontnumber>() as *mut internalfontnumber);
        self.initialized[379] = true;
        const {
            assert!(core::mem::size_of::<*mut internalfontnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).vflocalfontnum)
            .write(::core::ptr::null::<internalfontnumber>() as *mut internalfontnumber);
        self.initialized[380] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).vfpacketlength).write(0);
        self.initialized[381] = true;
        const {
            assert!(core::mem::size_of::<bytefile>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).vffile)
            .write(::core::ptr::null::<FILE>() as *mut FILE);
        self.initialized[382] = true;
        const {
            assert!(core::mem::size_of::<internalfontnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).vfnf).write(0);
        self.initialized[383] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).vfefnts)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[384] = true;
        const {
            assert!(core::mem::size_of::<*mut internalfontnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).vfifnts)
            .write(::core::ptr::null::<internalfontnumber>() as *mut internalfontnumber);
        self.initialized[385] = true;
        const {
            assert!(core::mem::size_of::<memoryword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).tmpw).write(memoryword { gr: 0. });
        self.initialized[386] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).vfcurs).write(0);
        self.initialized[387] = true;
        const {
            assert!((101) == (101));
            assert!(core::mem::size_of::<vfstackrecord>() <= 16 * 1024);
        }
        __tekai_require_copy::<vfstackrecord>();
        {
            let __tekai_repeat_0: vfstackrecord = vfstackrecord {
                stackh: 0,
                stackv: 0,
                stackw: 0,
                stackx: 0,
                stacky: 0,
                stackz: 0,
            };
            for __tekai_index_0 in 0..(101) {
                core::ptr::addr_of_mut!((*destination).vfstack[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[388] = true;
        const {
            assert!(core::mem::size_of::<vfstackindex>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).vfstackptr).write(0);
        self.initialized[389] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).savedpdfcurform).write(0);
        self.initialized[390] = true;
        const {
            assert!(core::mem::size_of::<strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdftexbanner).write(0);
        self.initialized[391] = true;
        const {
            assert!((4) == (4));
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        __tekai_require_copy::<scaled>();
        {
            let __tekai_repeat_0: scaled = 0;
            for __tekai_index_0 in 0..(4) {
                core::ptr::addr_of_mut!((*destination).totalstretch[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[392] = true;
        const {
            assert!((4) == (4));
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        __tekai_require_copy::<scaled>();
        {
            let __tekai_repeat_0: scaled = 0;
            for __tekai_index_0 in 0..(4) {
                core::ptr::addr_of_mut!((*destination).totalshrink[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[393] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).lastbadness).write(0);
        self.initialized[394] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).adjusttail).write(0);
        self.initialized[395] = true;
        const {
            assert!(core::mem::size_of::<*mut internalfontnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontblink)
            .write(::core::ptr::null::<internalfontnumber>() as *mut internalfontnumber);
        self.initialized[396] = true;
        const {
            assert!(core::mem::size_of::<*mut internalfontnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontelink)
            .write(::core::ptr::null::<internalfontnumber>() as *mut internalfontnumber);
        self.initialized[397] = true;
        const {
            assert!(core::mem::size_of::<*mut boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffonthasspacechar)
            .write(::core::ptr::null::<boolean>() as *mut boolean);
        self.initialized[398] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontstretch)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[399] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontshrink)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[400] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontstep)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[401] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontexpandratio)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[402] = true;
        const {
            assert!(core::mem::size_of::<*mut boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontautoexpand)
            .write(::core::ptr::null::<boolean>() as *mut boolean);
        self.initialized[403] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontlpbase)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[404] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontrpbase)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[405] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontefbase)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[406] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontknbsbase)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[407] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontstbsbase)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[408] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontshbsbase)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[409] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontknbcbase)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[410] = true;
        const {
            assert!(core::mem::size_of::<*mut integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffontknacbase)
            .write(::core::ptr::null::<integer>() as *mut integer);
        self.initialized[411] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fontexpandratio).write(0);
        self.initialized[412] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).lastleftmostchar).write(0);
        self.initialized[413] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).lastrightmostchar).write(0);
        self.initialized[414] = true;
        const {
            assert!((513) == (513));
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        __tekai_require_copy::<halfword>();
        {
            let __tekai_repeat_0: halfword = 0;
            for __tekai_index_0 in 0..(513) {
                core::ptr::addr_of_mut!((*destination).hliststack[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[415] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_short>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hliststacklevel).write(0);
        self.initialized[416] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).preadjusttail).write(0);
        self.initialized[417] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).packbeginline).write(0);
        self.initialized[418] = true;
        const {
            assert!(core::mem::size_of::<twohalves>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).emptyfield).write(twohalves {
            v: C2RustUnnamed_1 { LH: 0, RH: 0 },
        });
        self.initialized[419] = true;
        const {
            assert!(core::mem::size_of::<fourquarters>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).nulldelimiter).write(fourquarters {
            u: C2RustUnnamed_2 {
                B3: 0,
                B2: 0,
                B1: 0,
                B0: 0,
            },
        });
        self.initialized[420] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curmlist).write(0);
        self.initialized[421] = true;
        const {
            assert!(core::mem::size_of::<smallnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curstyle).write(0);
        self.initialized[422] = true;
        const {
            assert!(core::mem::size_of::<smallnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cursize).write(0);
        self.initialized[423] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curmu).write(0);
        self.initialized[424] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mlistpenalties).write(0);
        self.initialized[425] = true;
        const {
            assert!(core::mem::size_of::<internalfontnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curf).write(0);
        self.initialized[426] = true;
        const {
            assert!(core::mem::size_of::<quarterword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curc).write(0);
        self.initialized[427] = true;
        const {
            assert!(core::mem::size_of::<fourquarters>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curi).write(fourquarters {
            u: C2RustUnnamed_2 {
                B3: 0,
                B2: 0,
                B1: 0,
                B0: 0,
            },
        });
        self.initialized[428] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).magicoffset).write(0);
        self.initialized[429] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curalign).write(0);
        self.initialized[430] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curspan).write(0);
        self.initialized[431] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curloop).write(0);
        self.initialized[432] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).alignptr).write(0);
        self.initialized[433] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curhead).write(0);
        self.initialized[434] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curtail).write(0);
        self.initialized[435] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curprehead).write(0);
        self.initialized[436] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curpretail).write(0);
        self.initialized[437] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).justbox).write(0);
        self.initialized[438] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).passive).write(0);
        self.initialized[439] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).printednode).write(0);
        self.initialized[440] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).passnumber).write(0);
        self.initialized[441] = true;
        const {
            assert!((9) == (9));
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        __tekai_require_copy::<scaled>();
        {
            let __tekai_repeat_0: scaled = 0;
            for __tekai_index_0 in 0..(9) {
                core::ptr::addr_of_mut!((*destination).activewidth[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[442] = true;
        const {
            assert!((9) == (9));
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        __tekai_require_copy::<scaled>();
        {
            let __tekai_repeat_0: scaled = 0;
            for __tekai_index_0 in 0..(9) {
                core::ptr::addr_of_mut!((*destination).curactivewidth[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[443] = true;
        const {
            assert!((9) == (9));
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        __tekai_require_copy::<scaled>();
        {
            let __tekai_repeat_0: scaled = 0;
            for __tekai_index_0 in 0..(9) {
                core::ptr::addr_of_mut!((*destination).background[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[444] = true;
        const {
            assert!((9) == (9));
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        __tekai_require_copy::<scaled>();
        {
            let __tekai_repeat_0: scaled = 0;
            for __tekai_index_0 in 0..(9) {
                core::ptr::addr_of_mut!((*destination).breakwidth[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[445] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).autobreaking).write(0);
        self.initialized[446] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).prevp).write(0);
        self.initialized[447] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).firstp).write(0);
        self.initialized[448] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).prevcharp).write(0);
        self.initialized[449] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).nextcharp).write(0);
        self.initialized[450] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).tryprevbreak).write(0);
        self.initialized[451] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).prevlegal).write(0);
        self.initialized[452] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).prevprevlegal).write(0);
        self.initialized[453] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).prevautobreaking).write(0);
        self.initialized[454] = true;
        const {
            assert!((9) == (9));
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        __tekai_require_copy::<scaled>();
        {
            let __tekai_repeat_0: scaled = 0;
            for __tekai_index_0 in 0..(9) {
                core::ptr::addr_of_mut!((*destination).prevactivewidth[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[455] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).rejectedcurp).write(0);
        self.initialized[456] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).beforerejectedcurp).write(0);
        self.initialized[457] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).maxstretchratio).write(0);
        self.initialized[458] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).maxshrinkratio).write(0);
        self.initialized[459] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curfontstep).write(0);
        self.initialized[460] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).noshrinkerroryet).write(0);
        self.initialized[461] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curp).write(0);
        self.initialized[462] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).secondpass).write(0);
        self.initialized[463] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).finalpass).write(0);
        self.initialized[464] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).threshold).write(0);
        self.initialized[465] = true;
        const {
            assert!((4) == (4));
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        __tekai_require_copy::<integer>();
        {
            let __tekai_repeat_0: integer = 0;
            for __tekai_index_0 in 0..(4) {
                core::ptr::addr_of_mut!((*destination).minimaldemerits[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[466] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).minimumdemerits).write(0);
        self.initialized[467] = true;
        const {
            assert!((4) == (4));
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        __tekai_require_copy::<halfword>();
        {
            let __tekai_repeat_0: halfword = 0;
            for __tekai_index_0 in 0..(4) {
                core::ptr::addr_of_mut!((*destination).bestplace[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[468] = true;
        const {
            assert!((4) == (4));
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        __tekai_require_copy::<halfword>();
        {
            let __tekai_repeat_0: halfword = 0;
            for __tekai_index_0 in 0..(4) {
                core::ptr::addr_of_mut!((*destination).bestplline[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[469] = true;
        const {
            assert!((9) == (9));
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        __tekai_require_copy::<scaled>();
        {
            let __tekai_repeat_0: scaled = 0;
            for __tekai_index_0 in 0..(9) {
                core::ptr::addr_of_mut!((*destination).discwidth[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[470] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).easyline).write(0);
        self.initialized[471] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).lastspecialline).write(0);
        self.initialized[472] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).firstwidth).write(0);
        self.initialized[473] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).secondwidth).write(0);
        self.initialized[474] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).firstindent).write(0);
        self.initialized[475] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).secondindent).write(0);
        self.initialized[476] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).bestbet).write(0);
        self.initialized[477] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fewestdemerits).write(0);
        self.initialized[478] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).bestline).write(0);
        self.initialized[479] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).actuallooseness).write(0);
        self.initialized[480] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).linediff).write(0);
        self.initialized[481] = true;
        const {
            assert!((66) == (66));
            assert!(core::mem::size_of::<::core::ffi::c_short>() <= 16 * 1024);
        }
        __tekai_require_copy::<::core::ffi::c_short>();
        {
            let __tekai_repeat_0: ::core::ffi::c_short = 0;
            for __tekai_index_0 in 0..(66) {
                core::ptr::addr_of_mut!((*destination).hc[__tekai_index_0]).write(__tekai_repeat_0);
            }
        }
        self.initialized[482] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hn).write(0);
        self.initialized[483] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).ha).write(0);
        self.initialized[484] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hb).write(0);
        self.initialized[485] = true;
        const {
            assert!(core::mem::size_of::<internalfontnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hf).write(0);
        self.initialized[486] = true;
        const {
            assert!((64) == (64));
            assert!(core::mem::size_of::<::core::ffi::c_short>() <= 16 * 1024);
        }
        __tekai_require_copy::<::core::ffi::c_short>();
        {
            let __tekai_repeat_0: ::core::ffi::c_short = 0;
            for __tekai_index_0 in 0..(64) {
                core::ptr::addr_of_mut!((*destination).hu[__tekai_index_0]).write(__tekai_repeat_0);
            }
        }
        self.initialized[487] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hyfchar).write(0);
        self.initialized[488] = true;
        const {
            assert!(core::mem::size_of::<ASCIIcode>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).initcurlang).write(0);
        self.initialized[489] = true;
        const {
            assert!(core::mem::size_of::<ASCIIcode>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curlang).write(0);
        self.initialized[490] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).lhyf).write(0);
        self.initialized[491] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).rhyf).write(0);
        self.initialized[492] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).initlhyf).write(0);
        self.initialized[493] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).initrhyf).write(0);
        self.initialized[494] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hyfbchar).write(0);
        self.initialized[495] = true;
        const {
            assert!((65) == (65));
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        __tekai_require_copy::<::core::ffi::c_uchar>();
        {
            let __tekai_repeat_0: ::core::ffi::c_uchar = 0;
            for __tekai_index_0 in 0..(65) {
                core::ptr::addr_of_mut!((*destination).hyf[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[496] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).initlist).write(0);
        self.initialized[497] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).initlig).write(0);
        self.initialized[498] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).initlft).write(0);
        self.initialized[499] = true;
        const {
            assert!(core::mem::size_of::<smallnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hyphenpassed).write(0);
        self.initialized[500] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curl).write(0);
        self.initialized[501] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curr).write(0);
        self.initialized[502] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curq).write(0);
        self.initialized[503] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).ligstack).write(0);
        self.initialized[504] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).ligaturepresent).write(0);
        self.initialized[505] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).lfthit).write(0);
        self.initialized[506] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).rthit).write(0);
        self.initialized[507] = true;
        const {
            assert!(core::mem::size_of::<*mut triepointer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).trietrl)
            .write(::core::ptr::null::<triepointer>() as *mut triepointer);
        self.initialized[508] = true;
        const {
            assert!(core::mem::size_of::<*mut triepointer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).trietro)
            .write(::core::ptr::null::<triepointer>() as *mut triepointer);
        self.initialized[509] = true;
        const {
            assert!(core::mem::size_of::<*mut quarterword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).trietrc)
            .write(::core::ptr::null::<quarterword>() as *mut quarterword);
        self.initialized[510] = true;
        const {
            assert!((35112) == (35112));
            assert!(core::mem::size_of::<smallnumber>() <= 16 * 1024);
        }
        __tekai_require_copy::<smallnumber>();
        {
            let __tekai_repeat_0: smallnumber = 0;
            for __tekai_index_0 in 0..(35112) {
                core::ptr::addr_of_mut!((*destination).hyfdistance[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[511] = true;
        const {
            assert!((35112) == (35112));
            assert!(core::mem::size_of::<smallnumber>() <= 16 * 1024);
        }
        __tekai_require_copy::<smallnumber>();
        {
            let __tekai_repeat_0: smallnumber = 0;
            for __tekai_index_0 in 0..(35112) {
                core::ptr::addr_of_mut!((*destination).hyfnum[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[512] = true;
        const {
            assert!((35112) == (35112));
            assert!(core::mem::size_of::<trieopcode>() <= 16 * 1024);
        }
        __tekai_require_copy::<trieopcode>();
        {
            let __tekai_repeat_0: trieopcode = 0;
            for __tekai_index_0 in 0..(35112) {
                core::ptr::addr_of_mut!((*destination).hyfnext[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[513] = true;
        const {
            assert!((256) == (256));
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        __tekai_require_copy::<integer>();
        {
            let __tekai_repeat_0: integer = 0;
            for __tekai_index_0 in 0..(256) {
                core::ptr::addr_of_mut!((*destination).opstart[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[514] = true;
        const {
            assert!(core::mem::size_of::<*mut strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hyphword)
            .write(::core::ptr::null::<strnumber>() as *mut strnumber);
        self.initialized[515] = true;
        const {
            assert!(core::mem::size_of::<*mut halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hyphlist)
            .write(::core::ptr::null::<halfword>() as *mut halfword);
        self.initialized[516] = true;
        const {
            assert!(core::mem::size_of::<*mut hyphpointer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hyphlink)
            .write(::core::ptr::null::<hyphpointer>() as *mut hyphpointer);
        self.initialized[517] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hyphcount).write(0);
        self.initialized[518] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hyphnext).write(0);
        self.initialized[519] = true;
        const {
            assert!((70223) == (70223));
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        __tekai_require_copy::<integer>();
        {
            let __tekai_repeat_0: integer = 0;
            for __tekai_index_0 in 0..(70223) {
                core::ptr::addr_of_mut!((*destination).zzzab[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[520] = true;
        const {
            assert!((256) == (256));
            assert!(core::mem::size_of::<trieopcode>() <= 16 * 1024);
        }
        __tekai_require_copy::<trieopcode>();
        {
            let __tekai_repeat_0: trieopcode = 0;
            for __tekai_index_0 in 0..(256) {
                core::ptr::addr_of_mut!((*destination).trieused[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[521] = true;
        const {
            assert!((35112) == (35112));
            assert!(core::mem::size_of::<ASCIIcode>() <= 16 * 1024);
        }
        __tekai_require_copy::<ASCIIcode>();
        {
            let __tekai_repeat_0: ASCIIcode = 0;
            for __tekai_index_0 in 0..(35112) {
                core::ptr::addr_of_mut!((*destination).trieoplang[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[522] = true;
        const {
            assert!((35112) == (35112));
            assert!(core::mem::size_of::<trieopcode>() <= 16 * 1024);
        }
        __tekai_require_copy::<trieopcode>();
        {
            let __tekai_repeat_0: trieopcode = 0;
            for __tekai_index_0 in 0..(35112) {
                core::ptr::addr_of_mut!((*destination).trieopval[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[523] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).trieopptr).write(0);
        self.initialized[524] = true;
        const {
            assert!(core::mem::size_of::<trieopcode>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).maxopused).write(0);
        self.initialized[525] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).smallop).write(0);
        self.initialized[526] = true;
        const {
            assert!(core::mem::size_of::<*mut packedASCIIcode>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).triec)
            .write(::core::ptr::null::<packedASCIIcode>() as *mut packedASCIIcode);
        self.initialized[527] = true;
        const {
            assert!(core::mem::size_of::<*mut trieopcode>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).trieo)
            .write(::core::ptr::null::<trieopcode>() as *mut trieopcode);
        self.initialized[528] = true;
        const {
            assert!(core::mem::size_of::<*mut triepointer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).triel)
            .write(::core::ptr::null::<triepointer>() as *mut triepointer);
        self.initialized[529] = true;
        const {
            assert!(core::mem::size_of::<*mut triepointer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).trier)
            .write(::core::ptr::null::<triepointer>() as *mut triepointer);
        self.initialized[530] = true;
        const {
            assert!(core::mem::size_of::<triepointer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).trieptr).write(0);
        self.initialized[531] = true;
        const {
            assert!(core::mem::size_of::<*mut triepointer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).triehash)
            .write(::core::ptr::null::<triepointer>() as *mut triepointer);
        self.initialized[532] = true;
        const {
            assert!(core::mem::size_of::<*mut boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).trietaken)
            .write(::core::ptr::null::<boolean>() as *mut boolean);
        self.initialized[533] = true;
        const {
            assert!((256) == (256));
            assert!(core::mem::size_of::<triepointer>() <= 16 * 1024);
        }
        __tekai_require_copy::<triepointer>();
        {
            let __tekai_repeat_0: triepointer = 0;
            for __tekai_index_0 in 0..(256) {
                core::ptr::addr_of_mut!((*destination).triemin[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[534] = true;
        const {
            assert!(core::mem::size_of::<triepointer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).triemax).write(0);
        self.initialized[535] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).trienotready).write(0);
        self.initialized[536] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).bestheightplusdepth).write(0);
        self.initialized[537] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pagetail).write(0);
        self.initialized[538] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pagecontents).write(0);
        self.initialized[539] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pagemaxdepth).write(0);
        self.initialized[540] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).bestpagebreak).write(0);
        self.initialized[541] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).leastpagecost).write(0);
        self.initialized[542] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).bestsize).write(0);
        self.initialized[543] = true;
        const {
            assert!((8) == (8));
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        __tekai_require_copy::<scaled>();
        {
            let __tekai_repeat_0: scaled = 0;
            for __tekai_index_0 in 0..(8) {
                core::ptr::addr_of_mut!((*destination).pagesofar[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[544] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).lastglue).write(0);
        self.initialized[545] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).lastpenalty).write(0);
        self.initialized[546] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).lastkern).write(0);
        self.initialized[547] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).lastnodetype).write(0);
        self.initialized[548] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).insertpenalties).write(0);
        self.initialized[549] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).outputactive).write(0);
        self.initialized[550] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).outputcanend).write(0);
        self.initialized[551] = true;
        const {
            assert!(core::mem::size_of::<internalfontnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mainf).write(0);
        self.initialized[552] = true;
        const {
            assert!(core::mem::size_of::<fourquarters>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).maini).write(fourquarters {
            u: C2RustUnnamed_2 {
                B3: 0,
                B2: 0,
                B1: 0,
                B0: 0,
            },
        });
        self.initialized[553] = true;
        const {
            assert!(core::mem::size_of::<fourquarters>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mainj).write(fourquarters {
            u: C2RustUnnamed_2 {
                B3: 0,
                B2: 0,
                B1: 0,
                B0: 0,
            },
        });
        self.initialized[554] = true;
        const {
            assert!(core::mem::size_of::<fontindex>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).maink).write(0);
        self.initialized[555] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mainp).write(0);
        self.initialized[556] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mains).write(0);
        self.initialized[557] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).bchar).write(0);
        self.initialized[558] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).falsebchar).write(0);
        self.initialized[559] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cancelboundary).write(0);
        self.initialized[560] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).insdisc).write(0);
        self.initialized[561] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curbox).write(0);
        self.initialized[562] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).aftertoken).write(0);
        self.initialized[563] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).longhelpseen).write(0);
        self.initialized[564] = true;
        const {
            assert!(core::mem::size_of::<strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).formatident).write(0);
        self.initialized[565] = true;
        const {
            assert!(core::mem::size_of::<wordfile>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fmtfile)
            .write(::core::ptr::null::<gzFile_s>() as *mut gzFile_s);
        self.initialized[566] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).readyalready).write(0);
        self.initialized[567] = true;
        const {
            assert!((16) == (16));
            assert!(core::mem::size_of::<alphafile>() <= 16 * 1024);
        }
        __tekai_require_copy::<alphafile>();
        {
            let __tekai_repeat_0: alphafile = ::core::ptr::null::<FILE>() as *mut FILE;
            for __tekai_index_0 in 0..(16) {
                core::ptr::addr_of_mut!((*destination).writefile[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[568] = true;
        const {
            assert!((18) == (18));
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        __tekai_require_copy::<boolean>();
        {
            let __tekai_repeat_0: boolean = 0;
            for __tekai_index_0 in 0..(18) {
                core::ptr::addr_of_mut!((*destination).writeopen[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[569] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).writeloc).write(0);
        self.initialized[570] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflastobj).write(0);
        self.initialized[571] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflastxform).write(0);
        self.initialized[572] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflastximage).write(0);
        self.initialized[573] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflastximagepages).write(0);
        self.initialized[574] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflastximagecolordepth).write(0);
        self.initialized[575] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).altrule).write(0);
        self.initialized[576] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).warnpdfpagebox).write(0);
        self.initialized[577] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflastannot).write(0);
        self.initialized[578] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflastlink).write(0);
        self.initialized[579] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflastxpos).write(0);
        self.initialized[580] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflastypos).write(0);
        self.initialized[581] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfsnapxrefpos).write(0);
        self.initialized[582] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfsnapyrefpos).write(0);
        self.initialized[583] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).countdosnapy).write(0);
        self.initialized[584] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfretval).write(0);
        self.initialized[585] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curpagewidth).write(0);
        self.initialized[586] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curpageheight).write(0);
        self.initialized[587] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curhoffset).write(0);
        self.initialized[588] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curvoffset).write(0);
        self.initialized[589] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfobjlist).write(0);
        self.initialized[590] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfxformlist).write(0);
        self.initialized[591] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfximagelist).write(0);
        self.initialized[592] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).lastthread).write(0);
        self.initialized[593] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfthreadht).write(0);
        self.initialized[594] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfthreaddp).write(0);
        self.initialized[595] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfthreadwd).write(0);
        self.initialized[596] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflastthreadid).write(0);
        self.initialized[597] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflastthreadnamedid).write(0);
        self.initialized[598] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfthreadlevel).write(0);
        self.initialized[599] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfannotlist).write(0);
        self.initialized[600] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflinklist).write(0);
        self.initialized[601] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfdestlist).write(0);
        self.initialized[602] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfbeadlist).write(0);
        self.initialized[603] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfobjcount).write(0);
        self.initialized[604] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfxformcount).write(0);
        self.initialized[605] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfximagecount).write(0);
        self.initialized[606] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfcurform).write(0);
        self.initialized[607] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflastoutline).write(0);
        self.initialized[608] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdffirstoutline).write(0);
        self.initialized[609] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfparentoutline).write(0);
        self.initialized[610] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfxformwidth).write(0);
        self.initialized[611] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfxformdepth).write(0);
        self.initialized[612] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfxformheight).write(0);
        self.initialized[613] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfinfotoks).write(0);
        self.initialized[614] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfcatalogtoks).write(0);
        self.initialized[615] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfcatalogopenaction).write(0);
        self.initialized[616] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfnamestoks).write(0);
        self.initialized[617] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfdestnamesptr).write(0);
        self.initialized[618] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).destnamessize).write(0);
        self.initialized[619] = true;
        const {
            assert!(core::mem::size_of::<*mut destnameentry>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).destnames)
            .write(::core::ptr::null::<destnameentry>() as *mut destnameentry);
        self.initialized[620] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pkdpi).write(0);
        self.initialized[621] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).imageorigy).write(0);
        self.initialized[622] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).imageorigx).write(0);
        self.initialized[623] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdftrailertoks).write(0);
        self.initialized[624] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdftraileridtoks).write(0);
        self.initialized[625] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).genfakedinterwordspace).write(0);
        self.initialized[626] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).genrunninglink).write(0);
        self.initialized[627] = true;
        const {
            assert!(core::mem::size_of::<strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdfspacefontname).write(0);
        self.initialized[628] = true;
        const {
            assert!((11) == (11));
            assert!(core::mem::size_of::<pdflinkstackrecord>() <= 16 * 1024);
        }
        __tekai_require_copy::<pdflinkstackrecord>();
        {
            let __tekai_repeat_0: pdflinkstackrecord = pdflinkstackrecord {
                nestinglevel: 0,
                linknode: 0,
                reflinknode: 0,
            };
            for __tekai_index_0 in 0..(11) {
                core::ptr::addr_of_mut!((*destination).pdflinkstack[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[629] = true;
        const {
            assert!(core::mem::size_of::<smallnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pdflinkstackptr).write(0);
        self.initialized[630] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).isshippingpage).write(0);
        self.initialized[631] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).eTeXmode).write(0);
        self.initialized[632] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).etexp).write(0);
        self.initialized[633] = true;
        const {
            assert!(core::mem::size_of::<*mut boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).eofseen)
            .write(::core::ptr::null::<boolean>() as *mut boolean);
        self.initialized[634] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).LRptr).write(0);
        self.initialized[635] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).LRproblems).write(0);
        self.initialized[636] = true;
        const {
            assert!(core::mem::size_of::<smallnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curdir).write(0);
        self.initialized[637] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).pseudofiles).write(0);
        self.initialized[638] = true;
        const {
            assert!(core::mem::size_of::<*mut savepointer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).grpstack)
            .write(::core::ptr::null::<savepointer>() as *mut savepointer);
        self.initialized[639] = true;
        const {
            assert!(core::mem::size_of::<*mut halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).ifstack)
            .write(::core::ptr::null::<halfword>() as *mut halfword);
        self.initialized[640] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).maxregnum).write(0);
        self.initialized[641] = true;
        const {
            assert!(core::mem::size_of::<strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).maxreghelpline).write(0);
        self.initialized[642] = true;
        const {
            assert!((7) == (7));
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        __tekai_require_copy::<halfword>();
        {
            let __tekai_repeat_0: halfword = 0;
            for __tekai_index_0 in 0..(7) {
                core::ptr::addr_of_mut!((*destination).saroot[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[643] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).curptr).write(0);
        self.initialized[644] = true;
        const {
            assert!(core::mem::size_of::<memoryword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).sanull).write(memoryword { gr: 0. });
        self.initialized[645] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).sachain).write(0);
        self.initialized[646] = true;
        const {
            assert!(core::mem::size_of::<quarterword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).salevel).write(0);
        self.initialized[647] = true;
        const {
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).lastlinefill).write(0);
        self.initialized[648] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).dolastlinefit).write(0);
        self.initialized[649] = true;
        const {
            assert!(core::mem::size_of::<smallnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).activenodesize).write(0);
        self.initialized[650] = true;
        const {
            assert!((3) == (3));
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        __tekai_require_copy::<scaled>();
        {
            let __tekai_repeat_0: scaled = 0;
            for __tekai_index_0 in 0..(3) {
                core::ptr::addr_of_mut!((*destination).fillwidth[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[651] = true;
        const {
            assert!((4) == (4));
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        __tekai_require_copy::<scaled>();
        {
            let __tekai_repeat_0: scaled = 0;
            for __tekai_index_0 in 0..(4) {
                core::ptr::addr_of_mut!((*destination).bestplshort[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[652] = true;
        const {
            assert!((4) == (4));
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        __tekai_require_copy::<scaled>();
        {
            let __tekai_repeat_0: scaled = 0;
            for __tekai_index_0 in 0..(4) {
                core::ptr::addr_of_mut!((*destination).bestplglue[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[653] = true;
        const {
            assert!(core::mem::size_of::<triepointer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hyphstart).write(0);
        self.initialized[654] = true;
        const {
            assert!(core::mem::size_of::<triepointer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).hyphindex).write(0);
        self.initialized[655] = true;
        const {
            assert!((4) == (4));
            assert!(core::mem::size_of::<halfword>() <= 16 * 1024);
        }
        __tekai_require_copy::<halfword>();
        {
            let __tekai_repeat_0: halfword = 0;
            for __tekai_index_0 in 0..(4) {
                core::ptr::addr_of_mut!((*destination).discptr[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[656] = true;
        const {
            assert!(core::mem::size_of::<poolpointer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).editnamestart).write(0);
        self.initialized[657] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).editline).write(0);
        self.initialized[658] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).editnamelength).write(0);
        self.initialized[659] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).ipcon).write(0);
        self.initialized[660] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).stopatspace).write(0);
        self.initialized[661] = true;
        const {
            assert!(core::mem::size_of::<strnumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).savestrptr).write(0);
        self.initialized[662] = true;
        const {
            assert!(core::mem::size_of::<poolpointer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).savepoolptr).write(0);
        self.initialized[663] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).shellenabledp).write(0);
        self.initialized[664] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).restrictedshell).write(0);
        self.initialized[665] = true;
        const {
            assert!(core::mem::size_of::<*mut ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).outputcomment)
            .write(::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char);
        self.initialized[666] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).k).write(0);
        self.initialized[667] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_uchar>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).l).write(0);
        self.initialized[668] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).debugformatfile).write(0);
        self.initialized[669] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).expanddepthcount).write(0);
        self.initialized[670] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mltexp).write(0);
        self.initialized[671] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).mltexenabledp).write(0);
        self.initialized[672] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).accentc).write(0);
        self.initialized[673] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).basec).write(0);
        self.initialized[674] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).replacec).write(0);
        self.initialized[675] = true;
        const {
            assert!(core::mem::size_of::<fourquarters>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).iac).write(fourquarters {
            u: C2RustUnnamed_2 {
                B3: 0,
                B2: 0,
                B1: 0,
                B0: 0,
            },
        });
        self.initialized[676] = true;
        const {
            assert!(core::mem::size_of::<fourquarters>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).ibc).write(fourquarters {
            u: C2RustUnnamed_2 {
                B3: 0,
                B2: 0,
                B1: 0,
                B0: 0,
            },
        });
        self.initialized[677] = true;
        const {
            assert!(core::mem::size_of::<real>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).baseslant).write(0.);
        self.initialized[678] = true;
        const {
            assert!(core::mem::size_of::<real>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).accentslant).write(0.);
        self.initialized[679] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).basexheight).write(0);
        self.initialized[680] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).basewidth).write(0);
        self.initialized[681] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).baseheight).write(0);
        self.initialized[682] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).accentwidth).write(0);
        self.initialized[683] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).accentheight).write(0);
        self.initialized[684] = true;
        const {
            assert!(core::mem::size_of::<scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).delta).write(0);
        self.initialized[685] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).enctexp).write(0);
        self.initialized[686] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).enctexenabledp).write(0);
        self.initialized[687] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).synctexoption).write(0);
        self.initialized[688] = true;
        const {
            assert!(core::mem::size_of::<integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).synctexoffset).write(0);
        self.initialized[689] = true;
        const {
            assert!((57) == 57);
        }
        __tekai_require_copy::<const_string>();
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[0]).write(
            b"Usage: pdftex [OPTION]... [TEXNAME[.tex]] [COMMANDS]\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[1]).write(
            b"   or: pdftex [OPTION]... \\FIRST-LINE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[2]).write(
            b"   or: pdftex [OPTION]... &FMT ARGS\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[3]).write(
            b"  Run pdfTeX on TEXNAME, usually creating TEXNAME.pdf.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[4]).write(
            b"  Any remaining COMMANDS are processed as pdfTeX input, after TEXNAME is read.\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[5]).write(
            b"  If the first line of TEXNAME is %&FMT, and FMT is an existing .fmt file,\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[6]).write(
            b"  use it.  Else use `NAME.fmt', where NAME is the program invocation name,\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[7])
            .write(b"  most commonly `pdftex'.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[8])
            .write(b"\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[9]).write(
            b"  Alternatively, if the first non-option argument begins with a backslash,\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[10]).write(
            b"  interpret all non-option arguments as a line of pdfTeX input.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[11])
            .write(b"\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[12]).write(
            b"  Alternatively, if the first non-option argument begins with a &, the\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[13]).write(
            b"  next word is taken as the FMT to read, overriding all else.  Any\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[14]).write(
            b"  remaining arguments are processed as above.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[15])
            .write(b"\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[16]).write(
            b"  If no arguments or options are specified, prompt for input.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[17])
            .write(b"\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[18]).write(
            b"-cnf-line=STRING        parse STRING as a configuration file line\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[19]).write(
            b"-draftmode              switch on draft mode (generates no output PDF)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[20]).write(
            b"-enc                    enable encTeX extensions such as \\mubyte\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[21]).write(
            b"-etex                   enable e-TeX extensions\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[22]).write(
            b"[-no]-file-line-error   disable/enable file:line:error style messages\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[23]).write(
            b"-fmt=FMTNAME            use FMTNAME instead of program name or a %& line\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[24]).write(
            b"-halt-on-error          stop processing at the first error\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[25]).write(
            b"-ini                    be pdfinitex, for dumping formats; this is implicitly\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[26]).write(
            b"                          true if the program name is `pdfinitex'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[27]).write(
            b"-interaction=STRING     set interaction mode (STRING=batchmode/nonstopmode/\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[28]).write(
            b"                          scrollmode/errorstopmode)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[29]).write(
            b"-ipc                    send DVI output to a socket as well as the usual\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[30]).write(
            b"                          output file\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[31]).write(
            b"-ipc-start              as -ipc, and also start the server at the other end\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[32]).write(
            b"-jobname=STRING         set the job name to STRING\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[33]).write(
            b"-kpathsea-debug=NUMBER  set path searching debugging flags according to\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[34]).write(
            b"                          the bits of NUMBER\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[35]).write(
            b"[-no]-mktex=FMT         disable/enable mktexFMT generation (FMT=tex/tfm/pk)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[36]).write(
            b"-mltex                  enable MLTeX extensions such as \\charsubdef\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[37]).write(
            b"-output-comment=STRING  use STRING for DVI file comment instead of date\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[38]).write(
            b"                          (no effect for PDF)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[39]).write(
            b"-output-directory=DIR   use existing DIR as the directory to write files in\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[40]).write(
            b"-output-format=FORMAT   use FORMAT for job output; FORMAT is `dvi' or `pdf'\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[41]).write(
            b"[-no]-parse-first-line  disable/enable parsing of first line of input file\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[42]).write(
            b"-progname=STRING        set program (and fmt) name to STRING\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[43]).write(
            b"-recorder               enable filename recorder\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[44]).write(
            b"[-no]-shell-escape      disable/enable \\write18{SHELL COMMAND}\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[45]).write(
            b"-shell-restricted       enable restricted \\write18\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[46]).write(
            b"-src-specials           insert source specials into the DVI file\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[47]).write(
            b"-src-specials=WHERE     insert source specials in certain places of\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[48]).write(
            b"                          the DVI file. WHERE is a comma-separated value\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[49]).write(
            b"                          list: cr display hbox math par parend vbox\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[50]).write(
            b"-translate-file=TCXNAME use the TCX file TCXNAME\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[51]).write(
            b"-8bit                   make all characters printable by default\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[52]).write(
            b"-help                   display this help and exit\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[53]).write(
            b"-version                output version information and exit\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[54])
            .write(b"\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[55]).write(
            b"pdfTeX home page: <https://pdftex.org>\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PDFTEXHELP[56])
            .write(::core::ptr::null::<::core::ffi::c_char>());
        self.initialized[690] = true;
        const {
            assert!(core::mem::size_of::<*mut *mut ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cmdlist).write(:: core :: ptr :: null :: < * mut :: core :: ffi :: c_char > () as * mut * mut :: core :: ffi :: c_char);
        self.initialized[691] = true;
        const {
            assert!(core::mem::size_of::<*mut *mut ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).argv).write(:: core :: ptr :: null :: < * mut :: core :: ffi :: c_char > () as * mut * mut :: core :: ffi :: c_char);
        self.initialized[692] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).argc).write(0);
        self.initialized[693] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).optind).write(1);
        self.initialized[694] = true;
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).user_progname)
            .write(::core::ptr::null::<::core::ffi::c_char>());
        self.initialized[695] = true;
        const {
            assert!(core::mem::size_of::<*mut string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).user_cnf_lines)
            .write(::core::ptr::null::<string>() as *mut string);
        self.initialized[696] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_uint>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).user_cnf_nlines).write(0 as ::core::ffi::c_uint);
        self.initialized[697] = true;
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).c_job_name)
            .write(::core::ptr::null::<::core::ffi::c_char>());
        self.initialized[698] = true;
        const {
            assert!(core::mem::size_of::<string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).translate_filename)
            .write(::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char);
        self.initialized[699] = true;
        const {
            assert!(core::mem::size_of::<string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).default_translate_filename)
            .write(::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char);
        self.initialized[700] = true;
        const {
            assert!(core::mem::size_of::<*mut ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).last_source_name)
            .write(::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char);
        self.initialized[701] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).last_lineno).write(0);
        self.initialized[702] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).srcspecialsoption).write(false_0);
        self.initialized[703] = true;
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).ptexbanner).write(
            b"This is pdfTeX, Version 3.141592653-2.6-1.40.29\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        self.initialized[704] = true;
        const {
            assert!((16) == (16));
            assert!(core::mem::size_of::<*mut FILE>() <= 16 * 1024);
        }
        __tekai_require_copy::<*mut FILE>();
        {
            let __tekai_repeat_0: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
            for __tekai_index_0 in 0..(16) {
                core::ptr::addr_of_mut!((*destination).pipes[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[705] = true;
        const {
            assert!(core::mem::size_of::<const_string>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).edit_value).write(EDITOR.as_ptr());
        self.initialized[706] = true;
        const {
            assert!(core::mem::size_of::<*mut ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_makecstring_3837_cstrbuf)
            .write(::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char);
        self.initialized[707] = true;
        const {
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_makecstring_3840_allocsize).write(0);
        self.initialized[708] = true;
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
        (*crate::invocation_state::current()).generated_pdftexextra
    ))
}
#[cfg(test)]
pub(crate) unsafe fn check_state_construction(phase: u8) {
    let invocation = crate::invocation_state::current();
    let destination = state();
    assert!(!destination.is_null());
    let _ = (invocation, phase);
}
