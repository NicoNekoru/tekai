// Includes adapted upstream algorithms and handwritten Rust replacements.
// Upstream TeX Live source revision 52dac44dde13a8310bbb8231e81c58be28cebe20.
// Origin texk/web2c/pdftexdir/utils.c for retained pdfTeX algorithms.
// GPL-2.0-or-later.
// Modified for Tekai. Licensing and provenance notice restored 2026-10-13.
// Original copyrights and license terms are retained in the crate's
// NOTICE.md and licenses/upstream-notices.txt.

//! Typed printf helpers for generated pdfTeX Rust.

pub use crate::generated::backend::avlstuff::comp_string_entry;
pub use crate::generated::backend::epdf::epdf_free;
pub use crate::generated::backend::mapfile::fm_free;
pub use crate::generated::backend::subfont::sfd_free;
pub use crate::generated::backend::tounicode::glyph_unicode_free;
pub use crate::generated::backend::vfpacket::vf_free;
pub use crate::generated::backend::writeenc::enc_free;
pub use crate::generated::backend::writeimg::img_free;
pub use crate::generated::backend::writet1::t1_free;
pub use crate::generated::backend::writettf::ttf_free;
pub use crate::generated::pdftex0::{
    getnullstr, makestring, pdfflush, println, zflushstr, zpdfosgetosbuf, zprint, ztokenstostring,
};
pub use crate::generated::pdftexextra::makecstring;

pub use crate::generated::backend::avl::{
    avl_create, avl_find, avl_probe, avl_t_first, avl_t_init, avl_t_next, avl_table as AvlTable,
    avl_traverser as AvlTraverser,
};

use crate::regex_session::{self, MatchState};
use crate::stdio::exit;
use crate::stdio::Stream as FILE;
use crate::support::xfclose;
use libc::{c_char, c_int, c_longlong, c_uchar, c_uint, c_void};
use std::ffi::{CStr, CString};
use std::{mem, ptr, slice};

use crate::generated::backend::writezip::zip_free;
use crate::md5::{md5_append, md5_finish, md5_init, Md5State};

const MAX_PSTRING_LEN: usize = 1024;
const SMALL_ARRAY_SIZE: usize = 256;
const STACK_INCREMENT: usize = 8;
const MAX_COLORSTACKS: usize = 32768;
const COLOR_DEFAULT: &[u8] = b"0 g 0 G";
const DIRECT_ALWAYS: c_int = 2;
const EXIT_FAILURE: c_int = 1;
const TRUE: c_int = 1;
const FALSE: c_int = 0;

type Integer = c_int;
type LongInteger = libc::off_t;
type PoolPointer = c_int;
type StrNumber = c_int;
type Scaled = c_int;
type Boolean = c_int;
type InternalFontNumber = c_int;
type SizeT = usize;
type Address = *mut c_void;

#[path = "printf.rs"]
mod printf;
pub use printf::PrintfArg;

pub use crate::backend_types::{fd_entry as FdEntry, fm_entry as FmEntry};
#[cfg(test)]
pub use crate::backend_types::{fe_entry as FeEntry, intparm as IntParm};

#[derive(Clone)]
struct ColorStack {
    page_stack: Vec<Option<Vec<u8>>>,
    form_stack: Vec<Option<Vec<u8>>>,
    page_current: Option<Vec<u8>>,
    form_current: Option<Vec<u8>>,
    form_init: Option<Vec<u8>>,
    literal_mode: Integer,
    page_start: Boolean,
}

#[derive(Clone, Copy)]
struct MatrixEntry {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
}

#[derive(Clone, Copy)]
struct PosEntry {
    pos_h: c_int,
    pos_v: c_int,
    matrix_stack: usize,
}

unsafe fn kpse_invocation_name() -> *const c_char {
    let __tekai_state_kpathsea: *mut crate::kpathsea::State = unsafe { crate::kpathsea::state() };

    unsafe {
        if (*__tekai_state_kpathsea)
            .kpse_def_inst
            .invocation_name
            .is_null()
        {
            c"pdftex".as_ptr()
        } else {
            (*__tekai_state_kpathsea).kpse_def_inst.invocation_name
        }
    }
}

unsafe fn kpathsea_debug_enabled() -> bool {
    let __tekai_state_kpathsea: *mut crate::kpathsea::State = unsafe { crate::kpathsea::state() };

    unsafe { (*__tekai_state_kpathsea).kpse_def_inst.debug != 0 }
}

unsafe fn c_string_bytes(ptr: *const c_char) -> &'static [u8] {
    if ptr.is_null() {
        b""
    } else {
        unsafe { CStr::from_ptr(ptr).to_bytes() }
    }
}

unsafe fn print_tex_bytes(bytes: &[u8]) {
    let mut nul_terminated = Vec::with_capacity(bytes.len() + 1);
    nul_terminated.extend_from_slice(bytes);
    nul_terminated.push(0);
    unsafe {
        let s = maketexstring(nul_terminated.as_ptr() as *const c_char);
        zprint(s);
        zflushstr(s);
    }
}

unsafe fn xmalloc_bytes(size: usize) -> *mut c_void {
    let ptr = unsafe { crate::allocation::malloc(size) };
    if ptr.is_null() {
        unsafe { pdftex_fail_args(c"out of memory".as_ptr(), &[]) }
    }
    ptr
}

unsafe fn c_strdup(ptr: *const c_char) -> *mut c_char {
    let bytes = unsafe { c_string_bytes(ptr) };
    let out = unsafe { xmalloc_bytes(bytes.len() + 1) as *mut c_char };
    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), out as *mut u8, bytes.len());
        *out.add(bytes.len()) = 0;
    }
    out
}

unsafe fn malloced_c_string(bytes: &[u8]) -> *mut c_char {
    let out = unsafe { xmalloc_bytes(bytes.len() + 1) as *mut c_char };
    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), out as *mut u8, bytes.len());
        *out.add(bytes.len()) = 0;
    }
    out
}

unsafe fn c_str_len(ptr: *const c_char) -> usize {
    unsafe { crate::byte_string::strlen(ptr) }
}

fn check_pool_room(extra: usize) -> bool {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        if (*__tekai_state_generated_pdftexextra).poolptr as usize + extra
            >= (*__tekai_state_generated_pdftexextra).poolsize as usize
        {
            (*__tekai_state_generated_pdftexextra).poolptr =
                (*__tekai_state_generated_pdftexextra).poolsize;
            false
        } else {
            true
        }
    }
}

unsafe fn pdf_room(n: usize) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        if n + (*__tekai_state_generated_pdftexextra).pdfptr as usize
            > (*__tekai_state_generated_pdftexextra).pdfbufsize as usize
        {
            if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                zpdfosgetosbuf(n as Integer);
            } else if n > (*__tekai_state_generated_pdftexextra).pdfbufsize as usize {
                pdftex_fail_args(c"PDF output buffer overflowed".as_ptr(), &[]);
            } else {
                pdfflush();
            }
        }
    }
}

pub unsafe fn pdf_out(byte: u8) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        pdf_room(1);
        *(*__tekai_state_generated_pdftexextra)
            .pdfbuf
            .add((*__tekai_state_generated_pdftexextra).pdfptr as usize) = byte;
        (*__tekai_state_generated_pdftexextra).pdfptr += 1;
        (*__tekai_state_generated_pdftexextra).pdflastbyte = byte;
    }
}

#[inline(always)]
unsafe fn pdf_write_bytes_impl<const TRACK_LAST_BYTE: bool>(mut src: *const u8, mut len: usize) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        while len > 0 {
            if (*__tekai_state_generated_pdftexextra).pdfptr as usize
                >= (*__tekai_state_generated_pdftexextra).pdfbufsize as usize
            {
                if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                    zpdfosgetosbuf(
                        len.min((*__tekai_state_generated_pdftexextra).pdfbufsize as usize)
                            as Integer,
                    );
                } else {
                    pdfflush();
                }
            }

            let available = ((*__tekai_state_generated_pdftexextra).pdfbufsize
                - (*__tekai_state_generated_pdftexextra).pdfptr)
                as usize;
            if available == 0 {
                continue;
            }
            let chunk = len.min(available);
            ptr::copy_nonoverlapping(
                src,
                (*__tekai_state_generated_pdftexextra)
                    .pdfbuf
                    .add((*__tekai_state_generated_pdftexextra).pdfptr as usize),
                chunk,
            );
            (*__tekai_state_generated_pdftexextra).pdfptr += chunk as Integer;
            if TRACK_LAST_BYTE {
                (*__tekai_state_generated_pdftexextra).pdflastbyte = *src.add(chunk - 1);
            }
            src = src.add(chunk);
            len -= chunk;
        }
    }
}

pub unsafe fn pdf_write_bytes(src: *const u8, len: usize) {
    unsafe {
        pdf_write_bytes_impl::<true>(src, len);
    }
}

pub unsafe fn pdf_write_bytes_untracked(src: *const u8, len: usize) {
    unsafe {
        pdf_write_bytes_impl::<false>(src, len);
    }
}

pub unsafe fn pdf_puts(s: *const c_char) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    if s.is_null() {
        return;
    }
    let bytes = unsafe { CStr::from_ptr(s).to_bytes() };
    unsafe {
        pdf_room(bytes.len() + 1);
        ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            (*__tekai_state_generated_pdftexextra)
                .pdfbuf
                .add((*__tekai_state_generated_pdftexextra).pdfptr as usize),
            bytes.len(),
        );
        (*__tekai_state_generated_pdftexextra).pdfptr += bytes.len() as Integer;
        if let Some(&byte) = bytes.last() {
            (*__tekai_state_generated_pdftexextra).pdflastbyte = byte;
        }
    }
}

pub unsafe fn pdf_newline() {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        if (*__tekai_state_generated_pdftexextra).pdflastbyte != b'\n' {
            pdf_puts(c"\n".as_ptr());
        }
    }
}

pub unsafe fn maketexstring(s: *const c_char) -> StrNumber {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        if s.is_null() || *s == 0 {
            return getnullstr();
        }
        let bytes = CStr::from_ptr(s).to_bytes();
        if !check_pool_room(bytes.len()) {
            pdftex_fail_args(c"string pool overflow".as_ptr(), &[]);
        }
        ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            (*__tekai_state_generated_pdftexextra)
                .strpool
                .add((*__tekai_state_generated_pdftexextra).poolptr as usize),
            bytes.len(),
        );
        (*__tekai_state_generated_pdftexextra).poolptr += bytes.len() as PoolPointer;
        (*__tekai_state_utils).LAST_TEX_STRING = makestring();
        (*__tekai_state_utils).LAST_TEX_STRING
    }
}

unsafe fn safe_print(ptr: *const c_char) {
    unsafe {
        print_tex_bytes(c_string_bytes(ptr));
    }
}

unsafe fn safe_print_bytes(bytes: &[u8]) {
    unsafe {
        print_tex_bytes(bytes);
    }
}

/// # Safety
/// `fmt` and every non-null `%s` argument must point to a valid C string.
pub(crate) unsafe fn format_printf(fmt: *const c_char, args: &[PrintfArg]) -> Vec<u8> {
    let fmt = unsafe { c_string_bytes(fmt) };
    unsafe { printf::format(fmt, args) }.unwrap_or_else(|error| panic!("{error}"))
}

/// Rust-owned equivalent of snprintf with the original byte-count contract.
///
/// # Safety
/// `output` must be writable for `capacity` bytes unless capacity is zero.
/// `fmt` and every non-null `%s` argument must remain valid C strings.
pub unsafe fn snprintf_args(
    output: *mut c_char,
    capacity: usize,
    fmt: *const c_char,
    args: &[PrintfArg],
) -> c_int {
    let bytes = unsafe { format_printf(fmt, args) };
    if capacity != 0 {
        let count = bytes.len().min(capacity - 1);
        unsafe {
            ptr::copy_nonoverlapping(bytes.as_ptr(), output.cast::<u8>(), count);
            *output.add(count) = 0;
        }
    }
    c_int::try_from(bytes.len()).unwrap_or(-1)
}

/// Rust-owned equivalent of sprintf for generated callers with known buffers.
///
/// # Safety
/// `output` must hold the complete formatted bytes plus a trailing zero byte.
/// `fmt` and every non-null `%s` argument must remain valid C strings.
pub unsafe fn sprintf_args(output: *mut c_char, fmt: *const c_char, args: &[PrintfArg]) -> c_int {
    let bytes = unsafe { format_printf(fmt, args) };
    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), output.cast::<u8>(), bytes.len());
        *output.add(bytes.len()) = 0;
    }
    c_int::try_from(bytes.len()).unwrap_or(-1)
}

pub unsafe fn pdf_printf_args(fmt: *const c_char, args: &[PrintfArg]) {
    let bytes = unsafe { format_printf(fmt, args) };
    let mut c_bytes = Vec::with_capacity(bytes.len() + 1);
    c_bytes.extend_from_slice(&bytes);
    c_bytes.push(0);
    unsafe {
        pdf_puts(c_bytes.as_ptr() as *const c_char);
    }
}

pub unsafe fn tex_printf_args(fmt: *const c_char, args: &[PrintfArg]) {
    let bytes = unsafe { format_printf(fmt, args) };
    unsafe {
        safe_print_bytes(&bytes);
        xfflush(crate::stdio::stdout());
    }
}

pub unsafe fn pdftex_warn_args(fmt: *const c_char, args: &[PrintfArg]) {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    let bytes = unsafe { format_printf(fmt, args) };
    unsafe {
        println();
        println();
        tex_printf_args(
            c"pdfTeX warning: %s".as_ptr(),
            &[PrintfArg::from(kpse_invocation_name())],
        );
        if !(*__tekai_state_utils).cur_file_name.is_null() {
            tex_printf_args(
                c" (file %s)".as_ptr(),
                &[PrintfArg::from((*__tekai_state_utils).cur_file_name)],
            );
        }
        tex_printf_args(c": ".as_ptr(), &[]);
        safe_print_bytes(&bytes);
        println();
    }
}

pub unsafe fn pdftex_fail_args(fmt: *const c_char, args: &[PrintfArg]) -> ! {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    let bytes = unsafe { format_printf(fmt, args) };
    unsafe {
        println();
        safe_print(c"!pdfTeX error: ".as_ptr());
        safe_print(kpse_invocation_name());
        if !(*__tekai_state_utils).cur_file_name.is_null() {
            safe_print(c" (file ".as_ptr());
            safe_print((*__tekai_state_utils).cur_file_name);
            safe_print(c")".as_ptr());
        }
        safe_print(c": ".as_ptr());
        safe_print_bytes(&bytes);
        println();
        removepdffile();
        safe_print(c" ==> Fatal error occurred, no output PDF file produced!".as_ptr());
        println();
        if kpathsea_debug_enabled() {
            safe_print(c"kpathsea_debug enabled, reporting internal failure...".as_ptr());
            println();
            crate::termination::abort();
        } else {
            exit(EXIT_FAILURE);
        }
    }
}

pub unsafe fn removepdffile() {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        if !kpathsea_debug_enabled()
            && (*__tekai_state_generated_pdftexextra).outputfilename != 0
            && (*__tekai_state_generated_pdftexextra).fixedpdfdraftmode == 0
        {
            let name = makecstring((*__tekai_state_generated_pdftexextra).outputfilename);
            xfclose((*__tekai_state_generated_pdftexextra).pdffile, name);
            use std::os::unix::ffi::OsStrExt;
            let path =
                std::path::Path::new(std::ffi::OsStr::from_bytes(CStr::from_ptr(name).to_bytes()));
            if let Ok(path) = crate::working_directory::resolve(path) {
                let _ = std::fs::remove_file(path);
            }
        }
    }
}

pub unsafe fn garbagewarning() {
    unsafe {
        pdftex_warn_args(
            c"dangling objects discarded, no output file produced.".as_ptr(),
            &[],
        );
        removepdffile();
    }
}

pub(crate) unsafe fn xfwrite(
    ptr: *mut c_void,
    size: SizeT,
    nmemb: SizeT,
    stream: *mut FILE,
) -> SizeT {
    unsafe {
        if crate::stdio::fwrite(ptr, size, nmemb, stream) != nmemb {
            pdftex_fail_args(c"fwrite() failed".as_ptr(), &[]);
        }
    }
    nmemb
}

pub(crate) unsafe fn xfflush(stream: *mut FILE) -> c_int {
    unsafe {
        if crate::stdio::fflush(stream) != 0 {
            let err = std::io::Error::last_os_error()
                .raw_os_error()
                .unwrap_or(libc::EIO);
            let message = CString::new(crate::stdio::error_text(err))
                .unwrap_or_else(|_| c"I/O error".to_owned());
            pdftex_fail_args(
                c"fflush() failed (%s)".as_ptr(),
                &[PrintfArg::from(message.as_ptr())],
            );
        }
    }
    0
}

pub(crate) unsafe fn xgetc(stream: *mut FILE) -> c_int {
    unsafe { crate::stdio::fgetc(stream) }
}

pub unsafe fn writestreamlength(length: LongInteger, offset: LongInteger) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        if (*__tekai_state_utils).JOBNAME_CSTR.is_null() {
            (*__tekai_state_utils).JOBNAME_CSTR =
                c_strdup(makecstring((*__tekai_state_generated_pdftexextra).jobname));
        }
        if (*__tekai_state_generated_pdftexextra).fixedpdfdraftmode == 0 {
            if crate::stdio::fseeko(
                (*__tekai_state_generated_pdftexextra).pdffile,
                offset,
                libc::SEEK_SET,
            ) != 0
            {
                pdftex_fail_args(c"fseeko() failed".as_ptr(), &[]);
            }
            crate::stdio::fprintf_args(
                (*__tekai_state_generated_pdftexextra).pdffile,
                c"%lli".as_ptr(),
                &[crate::utils::PrintfArg::from(length as c_longlong)],
            );
            if crate::stdio::fseeko(
                (*__tekai_state_generated_pdftexextra).pdffile,
                (*__tekai_state_generated_pdftexextra).pdfgone
                    + (*__tekai_state_generated_pdftexextra).pdfptr as LongInteger,
                libc::SEEK_SET,
            ) != 0
            {
                pdftex_fail_args(c"fseeko() failed".as_ptr(), &[]);
            }
        }
    }
}

pub unsafe fn convertStringToPDFString(input: *const c_char, len: c_int) -> *mut c_char {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        let out = &raw mut (*__tekai_state_utils).PSTRING_BUF as *mut c_char;
        let mut j = 0usize;
        for i in 0..len.max(0) as usize {
            if j + 4 >= MAX_PSTRING_LEN {
                pdftex_fail_args(
                    c"buffer overflow at file %s, line %d".as_ptr(),
                    &[
                        PrintfArg::from(c"utils.rs".as_ptr()),
                        PrintfArg::from(line!() as c_int),
                    ],
                );
            }
            let ch = *input.add(i) as u8;
            if !(b'!'..=b'~').contains(&ch) {
                let frag = format!("\\{:03o}", ch);
                for &byte in frag.as_bytes() {
                    *out.add(j) = byte as c_char;
                    j += 1;
                }
            } else if ch == b'(' || ch == b')' {
                *out.add(j) = b'\\' as c_char;
                j += 1;
                *out.add(j) = ch as c_char;
                j += 1;
            } else if ch == b'\\' {
                *out.add(j) = b'\\' as c_char;
                j += 1;
                *out.add(j) = b'\\' as c_char;
                j += 1;
            } else {
                *out.add(j) = ch as c_char;
                j += 1;
            }
        }
        *out.add(j) = 0;
        out
    }
}

pub unsafe fn escapestring(input: PoolPointer) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        let out = (*__tekai_state_generated_pdftexextra).poolptr;
        let mut pos = input;
        while pos < out {
            if !check_pool_room(4) {
                return;
            }
            let ch = *(*__tekai_state_generated_pdftexextra)
                .strpool
                .add(pos as usize);
            pos += 1;
            if !(b'!'..=b'~').contains(&ch) {
                let frag = format!("\\{:03o}", ch);
                for &byte in frag.as_bytes() {
                    *(*__tekai_state_generated_pdftexextra)
                        .strpool
                        .add((*__tekai_state_generated_pdftexextra).poolptr as usize) = byte;
                    (*__tekai_state_generated_pdftexextra).poolptr += 1;
                }
                continue;
            }
            if ch == b'(' || ch == b')' || ch == b'\\' {
                *(*__tekai_state_generated_pdftexextra)
                    .strpool
                    .add((*__tekai_state_generated_pdftexextra).poolptr as usize) = b'\\';
                (*__tekai_state_generated_pdftexextra).poolptr += 1;
            }
            *(*__tekai_state_generated_pdftexextra)
                .strpool
                .add((*__tekai_state_generated_pdftexextra).poolptr as usize) = ch;
            (*__tekai_state_generated_pdftexextra).poolptr += 1;
        }
    }
}

pub unsafe fn escapename(input: PoolPointer) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        let out = (*__tekai_state_generated_pdftexextra).poolptr;
        let mut pos = input;
        while pos < out {
            if !check_pool_room(3) {
                return;
            }
            let ch = *(*__tekai_state_generated_pdftexextra)
                .strpool
                .add(pos as usize);
            pos += 1;
            let escape = (1..=32).contains(&ch)
                || ch >= 127
                || matches!(
                    ch,
                    b'#' | b'%' | b'(' | b')' | b'/' | b'<' | b'>' | b'[' | b']' | b'{' | b'}'
                );
            if ch == 0 {
                continue;
            }
            if escape {
                let frag = format!("#{:02X}", ch);
                for &byte in frag.as_bytes() {
                    *(*__tekai_state_generated_pdftexextra)
                        .strpool
                        .add((*__tekai_state_generated_pdftexextra).poolptr as usize) = byte;
                    (*__tekai_state_generated_pdftexextra).poolptr += 1;
                }
            } else {
                *(*__tekai_state_generated_pdftexextra)
                    .strpool
                    .add((*__tekai_state_generated_pdftexextra).poolptr as usize) = ch;
                (*__tekai_state_generated_pdftexextra).poolptr += 1;
            }
        }
    }
}

pub unsafe fn escapehex(input: PoolPointer) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        let out = (*__tekai_state_generated_pdftexextra).poolptr;
        let mut pos = input;
        while pos < out {
            if !check_pool_room(2) {
                return;
            }
            let frag = format!(
                "{:02X}",
                *(*__tekai_state_generated_pdftexextra)
                    .strpool
                    .add(pos as usize)
            );
            pos += 1;
            for &byte in frag.as_bytes() {
                *(*__tekai_state_generated_pdftexextra)
                    .strpool
                    .add((*__tekai_state_generated_pdftexextra).poolptr as usize) = byte;
                (*__tekai_state_generated_pdftexextra).poolptr += 1;
            }
        }
    }
}

pub unsafe fn unescapehex(input: PoolPointer) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        let out = (*__tekai_state_generated_pdftexextra).poolptr;
        let mut pos = input;
        let mut first = true;
        let mut high = 0u8;
        while pos < out {
            if !check_pool_room(1) {
                return;
            }
            let ch = *(*__tekai_state_generated_pdftexextra)
                .strpool
                .add(pos as usize);
            pos += 1;
            let val = match ch {
                b'0'..=b'9' => ch - b'0',
                b'A'..=b'F' => ch - b'A' + 10,
                b'a'..=b'f' => ch - b'a' + 10,
                _ => continue,
            };
            if first {
                high = val << 4;
                first = false;
            } else {
                *(*__tekai_state_generated_pdftexextra)
                    .strpool
                    .add((*__tekai_state_generated_pdftexextra).poolptr as usize) = high + val;
                (*__tekai_state_generated_pdftexextra).poolptr += 1;
                first = true;
            }
        }
        if !first {
            *(*__tekai_state_generated_pdftexextra)
                .strpool
                .add((*__tekai_state_generated_pdftexextra).poolptr as usize) = high;
            (*__tekai_state_generated_pdftexextra).poolptr += 1;
        }
    }
}

fn hex_string(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push_str(&format!("{byte:02X}"));
    }
    out
}

pub unsafe fn printID(filename: StrNumber) {
    unsafe {
        let mut state = mem::MaybeUninit::<Md5State>::zeroed();
        let state_ptr = state.as_mut_ptr();
        md5_init(state_ptr);
        let start = crate::generated::pdftexextra::start_date();
        md5_append(
            state_ptr,
            start.as_bytes().as_ptr(),
            start.as_bytes().len() as c_int,
        );
        let file_name = makecstring(filename);
        md5_append(
            state_ptr,
            file_name as *const u8,
            c_str_len(file_name) as c_int,
        );
        let mut digest = [0u8; 16];
        md5_finish(state_ptr, digest.as_mut_ptr());
        let id = CString::new(hex_string(&digest)).expect("hex has no nul");
        pdf_printf_args(
            c"/ID [<%s> <%s>]".as_ptr(),
            &[PrintfArg::from(id.as_ptr()), PrintfArg::from(id.as_ptr())],
        );
    }
}

pub unsafe fn printIDalt(toks: Integer) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        let tex_string = ztokenstostring(toks);
        let s = makecstring(tex_string);
        zflushstr((*__tekai_state_generated_pdftexextra).lasttokensstring);
        if c_str_len(s) == 0 {
            return;
        }
        let mut state = mem::MaybeUninit::<Md5State>::zeroed();
        let state_ptr = state.as_mut_ptr();
        md5_init(state_ptr);
        md5_append(state_ptr, s as *const u8, c_str_len(s) as c_int);
        let mut digest = [0u8; 16];
        md5_finish(state_ptr, digest.as_mut_ptr());
        let id = CString::new(hex_string(&digest)).expect("hex has no nul");
        pdf_printf_args(
            c"/ID [<%s> <%s>]".as_ptr(),
            &[PrintfArg::from(id.as_ptr()), PrintfArg::from(id.as_ptr())],
        );
    }
}

pub unsafe fn printcreationdate() {
    unsafe {
        let start = crate::generated::pdftexextra::start_date();
        pdf_printf_args(
            c"/CreationDate (%s)\n".as_ptr(),
            &[PrintfArg::from(start.as_ptr())],
        );
    }
}

pub unsafe fn printmoddate() {
    unsafe {
        let start = crate::generated::pdftexextra::start_date();
        pdf_printf_args(
            c"/ModDate (%s)\n".as_ptr(),
            &[PrintfArg::from(start.as_ptr())],
        );
    }
}

pub unsafe fn setjobid(year: c_int, month: c_int, day: c_int, time: c_int) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_kpathsea: *mut crate::kpathsea::State = unsafe { crate::kpathsea::state() };
    let __tekai_state_support: *mut crate::support::State = unsafe { crate::support::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        if !(*__tekai_state_utils).JOB_ID_STRING.is_null() {
            return;
        }
        let name = c_string_bytes(makecstring((*__tekai_state_generated_pdftexextra).jobname));
        let format = c_string_bytes(makecstring(
            (*__tekai_state_generated_pdftexextra).formatident,
        ));
        let banner = c_string_bytes((*__tekai_state_generated_pdftexextra).ptexbanner);
        let version = c_string_bytes((*__tekai_state_support).versionstring);
        let kpse = c_string_bytes((*__tekai_state_kpathsea).kpathsea_version_string);
        let s = format!(
            "{year:04}/{month:02}/{day:02} {:02}:{:02} {} {} {}{} {}",
            time / 60,
            time % 60,
            String::from_utf8_lossy(name),
            String::from_utf8_lossy(format),
            String::from_utf8_lossy(banner),
            String::from_utf8_lossy(version),
            String::from_utf8_lossy(kpse)
        );
        (*__tekai_state_utils).JOB_ID_STRING = malloced_c_string(s.as_bytes());
    }
}

pub unsafe fn makepdftexbanner() {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_kpathsea: *mut crate::kpathsea::State = unsafe { crate::kpathsea::state() };
    let __tekai_state_support: *mut crate::support::State = unsafe { crate::support::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        if (*__tekai_state_utils).local_makepdftexbanner_793_PDFTEX_BANNER_INIT != 0 {
            return;
        }
        let s = format!(
            "{}{} {}",
            String::from_utf8_lossy(c_string_bytes(
                (*__tekai_state_generated_pdftexextra).ptexbanner
            )),
            String::from_utf8_lossy(c_string_bytes((*__tekai_state_support).versionstring)),
            String::from_utf8_lossy(c_string_bytes(
                (*__tekai_state_kpathsea).kpathsea_version_string
            ))
        );
        let c = CString::new(s).expect("banner has no nul");
        (*__tekai_state_generated_pdftexextra).pdftexbanner = maketexstring(c.as_ptr());
        (*__tekai_state_utils).local_makepdftexbanner_793_PDFTEX_BANNER_INIT = TRUE;
    }
}

fn resource_name_prefix(id: &[u8]) -> [u8; 6] {
    const NAME_BYTES: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut crc = crc32fast::hash(id);
    let mut prefix = [0u8; 6];
    for byte in &mut prefix {
        *byte = NAME_BYTES[(crc % 62) as usize];
        crc /= 62;
    }
    prefix
}

pub unsafe fn getresnameprefix() -> StrNumber {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        let prefix = resource_name_prefix(c_string_bytes((*__tekai_state_utils).JOB_ID_STRING));
        let c = CString::new(prefix.as_slice()).expect("prefix has no nul");
        maketexstring(c.as_ptr())
    }
}

pub unsafe fn initversionstring(versions: *mut *mut c_char) {
    unsafe {
        *versions = malloced_c_string(
            b"PNG decoder: Rust png\nCompression backend: Rust zlib-rs\nPDF reader: Rust lopdf\n",
        );
    }
}

pub unsafe fn extxnoverd(x: Scaled, n: Scaled, d: Scaled) -> Scaled {
    let mut r = (x as f64 * n as f64) / d as f64;
    if r > f64::EPSILON {
        r += 0.5;
    } else {
        r -= 0.5;
    }
    if r >= 0x7fff_ffff as f64 || r <= -(0x7fff_ffff as f64) {
        unsafe {
            pdftex_warn_args(c"arithmetic: number too big".as_ptr(), &[]);
        }
    }
    r as Scaled
}

fn regex_pool_room(extra: usize) -> Option<usize> {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        let start = usize::try_from((*__tekai_state_generated_pdftexextra).poolptr).ok()?;
        let size = usize::try_from((*__tekai_state_generated_pdftexextra).poolsize).ok()?;
        (!(*__tekai_state_generated_pdftexextra).strpool.is_null()
            && start.checked_add(extra)? < size)
            .then_some(start)
    }
}

fn warn_regex_stderr(error: &impl std::fmt::Display) {
    use std::io::Write;
    let _ = writeln!(
        std::io::stderr().lock(),
        "pdfTeX warning: \\pdfmatch: {error}"
    );
}

unsafe fn append_regex_result(prefix: &[u8], bytes: &[u8]) -> bool {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let start = prefix
        .len()
        .checked_add(bytes.len())
        .and_then(regex_pool_room);
    let Some(start) = start else {
        // The normal warning path allocates strings in this very pool.
        // Report exhaustion without recursively trying to allocate a warning.
        warn_regex_stderr(&"string pool overflow");
        unsafe {
            (*__tekai_state_generated_pdftexextra).poolptr =
                (*__tekai_state_generated_pdftexextra).poolsize.max(0)
        };
        return false;
    };
    unsafe {
        ptr::copy_nonoverlapping(
            prefix.as_ptr(),
            (*__tekai_state_generated_pdftexextra).strpool.add(start),
            prefix.len(),
        );
        ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            (*__tekai_state_generated_pdftexextra)
                .strpool
                .add(start + prefix.len()),
            bytes.len(),
        );
        (*__tekai_state_generated_pdftexextra).poolptr =
            (start + prefix.len() + bytes.len()) as PoolPointer;
    }
    true
}

fn warn_regex(error: &impl std::fmt::Display) {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    use std::fmt::Write;
    struct Diagnostic {
        bytes: [u8; 256],
        len: usize,
    }
    impl Write for Diagnostic {
        fn write_str(&mut self, text: &str) -> std::fmt::Result {
            let end = self.len.checked_add(text.len()).ok_or(std::fmt::Error)?;
            if end >= self.bytes.len() {
                return Err(std::fmt::Error);
            }
            for byte in text.bytes() {
                self.bytes[self.len] = if byte == 0 { b'?' } else { byte };
                self.len += 1;
            }
            Ok(())
        }
    }
    let mut diagnostic = Diagnostic {
        bytes: [0; 256],
        len: 0,
    };
    if write!(&mut diagnostic, "{error}").is_err() {
        diagnostic = Diagnostic {
            bytes: [0; 256],
            len: 0,
        };
        let _ = diagnostic.write_str("regular expression diagnostic exceeds its size limit");
    }
    unsafe {
        // Each warning fragment is printed then flushed from the TeX pool.
        // Check the largest fragment before using the traditional printer.
        let invocation = CStr::from_ptr(kpse_invocation_name()).to_bytes().len();
        let file = if (*__tekai_state_utils).cur_file_name.is_null() {
            0
        } else {
            CStr::from_ptr((*__tekai_state_utils).cur_file_name)
                .to_bytes()
                .len()
        };
        let required = invocation
            .saturating_add(16)
            .max(file.saturating_add(8))
            .max(diagnostic.len + 11);
        if regex_pool_room(required).is_none() {
            warn_regex_stderr(error);
            return;
        }
        pdftex_warn_args(
            c"\\pdfmatch: %s".as_ptr(),
            &[PrintfArg::from(diagnostic.bytes.as_ptr())],
        );
    }
}

fn regex_offset_prefix(start: usize, output: &mut [u8; 24]) -> &[u8] {
    let mut at = output.len() - 2;
    output[at..].copy_from_slice(b"->");
    let mut value = start;
    loop {
        at -= 1;
        output[at] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    &output[at..]
}

/// # Safety
/// The generated TeX string numbers and pool storage must be valid. An active
/// invocation owns regex state, and makecstring provides terminated backing.
pub unsafe fn matchstrings(s: StrNumber, t: StrNumber, subcount: c_int, icase: Boolean) {
    let outcome = regex_session::with_state(|state| unsafe {
        state.clear();
        let pattern = makecstring(s);
        if pattern.is_null() {
            return Err(crate::posix_regex::RegexError::Invalid(
                "missing regular expression",
            ));
        }
        // makecstring reuses its buffer. Compile the pattern before rendering
        // text, and never retain either borrowed C-string slice in MatchState.
        let regex = state.prepare(CStr::from_ptr(pattern).to_bytes(), subcount, icase != 0)?;
        let text = makecstring(t);
        if text.is_null() {
            return Err(crate::posix_regex::RegexError::Invalid(
                "missing regex input",
            ));
        }
        state.execute(&regex, CStr::from_ptr(text).to_bytes())
    });
    let result: &[u8] = match outcome {
        Ok(Ok(true)) => b"1",
        Ok(Ok(false)) => b"0",
        Ok(Err(error)) => {
            warn_regex(&error);
            b"-1"
        }
        Err(error) => {
            warn_regex(&error);
            b"-1"
        }
    };
    if !unsafe { append_regex_result(result, b"") } {
        let _ = regex_session::with_state(MatchState::clear);
    }
}

/// # Safety
/// The generated TeX pool must be valid and the invocation scope must be active.
pub unsafe fn getmatch(i: c_int) {
    let outcome = regex_session::with_state(|state| {
        let appended = if let Some((start, bytes)) = state.capture(i) {
            let mut buffer = [0u8; 24];
            let prefix = regex_offset_prefix(start, &mut buffer);
            unsafe { append_regex_result(prefix, bytes) }
        } else {
            unsafe { append_regex_result(b"-1->", b"") }
        };
        if !appended {
            state.clear();
        }
    });
    if let Err(error) = outcome {
        warn_regex(&error);
        unsafe { append_regex_result(b"-1->", b"") };
    }
}

pub unsafe fn fb_offset() -> Integer {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        (*__tekai_state_utils)
            .fb_ptr
            .offset_from((*__tekai_state_utils).fb_array) as Integer
    }
}

pub unsafe fn fb_seek(offset: Integer) {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        (*__tekai_state_utils).fb_ptr = (*__tekai_state_utils).fb_array.add(offset as usize);
    }
}

unsafe fn ensure_fb_room(n: usize) {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        grow_array(
            &mut (*__tekai_state_utils).fb_array,
            &mut (*__tekai_state_utils).fb_ptr,
            &mut (*__tekai_state_utils).fb_limit,
            n,
            SMALL_ARRAY_SIZE,
            c"fb_array exceeds size limit".as_ptr(),
        );
    }
}

pub unsafe fn fb_putchar(b: c_uchar) {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        ensure_fb_room(1);
        *(*__tekai_state_utils).fb_ptr = b as c_char;
        (*__tekai_state_utils).fb_ptr = (*__tekai_state_utils).fb_ptr.add(1);
    }
}

pub unsafe fn fb_flush() {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        let mut p = (*__tekai_state_utils).fb_array;
        while p < (*__tekai_state_utils).fb_ptr {
            let mut n = ((*__tekai_state_generated_pdftexextra).pdfbufsize
                - (*__tekai_state_generated_pdftexextra).pdfptr) as usize;
            let remaining = (*__tekai_state_utils).fb_ptr.offset_from(p) as usize;
            if remaining < n {
                n = remaining;
            }
            ptr::copy_nonoverlapping(
                p as *const c_uchar,
                (*__tekai_state_generated_pdftexextra)
                    .pdfbuf
                    .add((*__tekai_state_generated_pdftexextra).pdfptr as usize),
                n,
            );
            (*__tekai_state_generated_pdftexextra).pdfptr += n as Integer;
            if (*__tekai_state_generated_pdftexextra).pdfptr
                == (*__tekai_state_generated_pdftexextra).pdfbufsize
            {
                pdfflush();
            }
            p = p.add(n);
        }
        (*__tekai_state_utils).fb_ptr = (*__tekai_state_utils).fb_array;
    }
}

fn growth_capacity(limit: usize, used: usize, extra: usize, initial: usize) -> Option<usize> {
    let required = used.checked_add(extra)?;
    if limit == 0 {
        Some(initial.max(required))
    } else if required <= limit {
        Some(limit)
    } else {
        Some(limit.checked_mul(2)?.max(required))
    }
}

unsafe fn grow_array<T>(
    array: &mut *mut T,
    ptr_value: &mut *mut T,
    limit: &mut SizeT,
    n: usize,
    initial: usize,
    error_message: *const c_char,
) {
    unsafe {
        let was_null = (*array).is_null();
        let used = if was_null {
            0
        } else {
            (*ptr_value).offset_from(*array) as usize
        };
        let old_limit = if was_null { 0 } else { *limit };
        let Some(new_limit) = growth_capacity(old_limit, used, n, initial) else {
            pdftex_fail_args(error_message, &[]);
        };
        if !was_null && new_limit == old_limit {
            return;
        }
        let Some(bytes) = new_limit.checked_mul(mem::size_of::<T>()) else {
            pdftex_fail_args(error_message, &[]);
        };
        let resized = if was_null {
            xmalloc_bytes(bytes).cast::<T>()
        } else {
            crate::allocation::realloc((*array).cast(), bytes).cast::<T>()
        };
        if resized.is_null() {
            pdftex_fail_args(error_message, &[]);
        }
        *array = resized;
        *ptr_value = resized.add(used);
        *limit = new_limit;
    }
}

pub unsafe fn allocvffnts() {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        if (*__tekai_state_utils).VF_E_FNTS_ARRAY.is_null() {
            (*__tekai_state_utils).VF_E_FNTS_ARRAY = (*__tekai_state_generated_pdftexextra).vfefnts;
            (*__tekai_state_utils).VF_E_FNTS_LIMIT =
                (*__tekai_state_generated_pdftexextra).fontmax as SizeT;
            (*__tekai_state_utils).VF_E_FNTS_PTR = (*__tekai_state_utils).VF_E_FNTS_ARRAY;
            (*__tekai_state_utils).VF_I_FNTS_ARRAY = (*__tekai_state_generated_pdftexextra).vfifnts;
            (*__tekai_state_utils).VF_I_FNTS_LIMIT =
                (*__tekai_state_generated_pdftexextra).fontmax as SizeT;
            (*__tekai_state_utils).VF_I_FNTS_PTR = (*__tekai_state_utils).VF_I_FNTS_ARRAY;
        }
        grow_array(
            &mut (*__tekai_state_utils).VF_E_FNTS_ARRAY,
            &mut (*__tekai_state_utils).VF_E_FNTS_PTR,
            &mut (*__tekai_state_utils).VF_E_FNTS_LIMIT,
            1,
            (*__tekai_state_generated_pdftexextra).fontmax as usize,
            c"vf_e_fnts_array exceeds size limit".as_ptr(),
        );
        (*__tekai_state_utils).VF_E_FNTS_PTR = (*__tekai_state_utils).VF_E_FNTS_PTR.add(1);
        grow_array(
            &mut (*__tekai_state_utils).VF_I_FNTS_ARRAY,
            &mut (*__tekai_state_utils).VF_I_FNTS_PTR,
            &mut (*__tekai_state_utils).VF_I_FNTS_LIMIT,
            1,
            (*__tekai_state_generated_pdftexextra).fontmax as usize,
            c"vf_i_fnts_array exceeds size limit".as_ptr(),
        );
        (*__tekai_state_utils).VF_I_FNTS_PTR = (*__tekai_state_utils).VF_I_FNTS_PTR.add(1);
        if (*__tekai_state_utils).VF_E_FNTS_ARRAY != (*__tekai_state_generated_pdftexextra).vfefnts
        {
            (*__tekai_state_generated_pdftexextra).vfefnts = (*__tekai_state_utils).VF_E_FNTS_ARRAY;
            (*__tekai_state_generated_pdftexextra).vfifnts = (*__tekai_state_utils).VF_I_FNTS_ARRAY;
        }
    }
}

pub unsafe fn libpdffinish() {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        crate::allocation::free((*__tekai_state_utils).fb_array as *mut c_void);
        (*__tekai_state_utils).fb_array = ptr::null_mut();
        (*__tekai_state_utils).fb_ptr = ptr::null_mut();
        crate::allocation::free((*__tekai_state_utils).JOB_ID_STRING as *mut c_void);
        (*__tekai_state_utils).JOB_ID_STRING = ptr::null_mut();
        fm_free();
        t1_free();
        enc_free();
        img_free();
        vf_free();
        epdf_free();
        ttf_free();
        sfd_free();
        glyph_unicode_free();
        zip_free();
    }
}

pub unsafe fn make_subset_tag(fd: *mut FdEntry) {
    let __tekai_state_generated_backend_avlstuff: *mut crate::generated::backend::avlstuff::State =
        unsafe { crate::generated::backend::avlstuff::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    const SUBSET_TAG_LENGTH: usize = 6;
    unsafe {
        if (*__tekai_state_utils).SUBSET_TAG_TREE.is_null() {
            (*__tekai_state_utils).SUBSET_TAG_TREE = avl_create(
                Some(comp_string_entry),
                ptr::null_mut(),
                &raw mut (*__tekai_state_generated_backend_avlstuff).avl_xallocator,
            );
        }
        if fd.is_null() || (*fd).gl_tree.is_null() || (*fd).fontname.is_null() {
            pdftex_fail_args(c"make_subset_tag(): invalid font descriptor".as_ptr(), &[]);
        }
        let subset = xmalloc_bytes(SUBSET_TAG_LENGTH + 1) as *mut c_char;
        (*fd).subset_tag = subset;
        let mut digest = [0u8; 16];
        let mut round = 0i32;
        loop {
            let mut state = mem::MaybeUninit::<Md5State>::zeroed();
            let state_ptr = state.as_mut_ptr();
            md5_init(state_ptr);
            let mut trav = mem::MaybeUninit::<AvlTraverser>::zeroed().assume_init();
            avl_t_init(&mut trav, (*fd).gl_tree);
            let mut glyph = avl_t_first(&mut trav, (*fd).gl_tree) as *mut c_char;
            while !glyph.is_null() {
                md5_append(state_ptr, glyph as *const u8, c_str_len(glyph) as c_int);
                md5_append(state_ptr, c" ".as_ptr() as *const u8, 1);
                glyph = avl_t_next(&mut trav) as *mut c_char;
            }
            md5_append(
                state_ptr,
                (*fd).fontname as *const u8,
                c_str_len((*fd).fontname) as c_int,
            );
            md5_append(
                state_ptr,
                &round as *const i32 as *const u8,
                mem::size_of::<i32>() as c_int,
            );
            md5_finish(state_ptr, digest.as_mut_ptr());
            let mut a = [0i32; SUBSET_TAG_LENGTH];
            for i in 0..13 {
                a[0] += digest[i] as i32;
            }
            for i in 1..SUBSET_TAG_LENGTH {
                a[i] = a[i - 1] - digest[i - 1] as i32 + digest[(i + 12) % 16] as i32;
            }
            for i in 0..SUBSET_TAG_LENGTH {
                *subset.add(i) = ((a[i] % 26) as u8 + b'A') as c_char;
            }
            *subset.add(SUBSET_TAG_LENGTH) = 0;
            round += 1;
            if avl_find(
                (*__tekai_state_utils).SUBSET_TAG_TREE,
                subset as *const c_void,
            )
            .is_null()
            {
                break;
            }
            if round >= 100 {
                pdftex_fail_args(c"make_subset_tag(): subset-tag collision".as_ptr(), &[]);
            }
        }
        let slot = avl_probe(
            (*__tekai_state_utils).SUBSET_TAG_TREE,
            subset as *mut c_void,
        );
        if slot.is_null() {
            pdftex_fail_args(c"make_subset_tag(): avl_probe failed".as_ptr(), &[]);
        }
        if round > 2 {
            pdftex_warn_args(
                c"\nmake_subset_tag(): subset-tag collision, resolved in round %d.\n".as_ptr(),
                &[PrintfArg::from(round)],
            );
        }
    }
}

// The raw handle is valid only within the current invocation. Callers form
// short local borrows, never a process-lifetime mutable reference.
unsafe fn init_colorstacks() -> *mut Vec<ColorStack> {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        if (*__tekai_state_utils).COLOR_STACKS.is_none() {
            (*__tekai_state_utils).COLOR_STACKS = Some(vec![ColorStack {
                page_stack: Vec::new(),
                form_stack: Vec::new(),
                page_current: Some(COLOR_DEFAULT.to_vec()),
                form_current: Some(COLOR_DEFAULT.to_vec()),
                form_init: Some(COLOR_DEFAULT.to_vec()),
                literal_mode: DIRECT_ALWAYS,
                page_start: TRUE,
            }]);
        }
        (*__tekai_state_utils)
            .COLOR_STACKS
            .as_mut()
            .expect("initialized") as *mut Vec<ColorStack>
    }
}

unsafe fn pool_put_bytes(start: PoolPointer, bytes: Option<&[u8]>) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        let Some(bytes) = bytes else { return };
        if bytes.is_empty() {
            return;
        }
        (*__tekai_state_generated_pdftexextra).poolptr = start + bytes.len() as PoolPointer;
        if (*__tekai_state_generated_pdftexextra).poolptr
            >= (*__tekai_state_generated_pdftexextra).poolsize
        {
            (*__tekai_state_generated_pdftexextra).poolptr =
                (*__tekai_state_generated_pdftexextra).poolsize;
            return;
        }
        ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            (*__tekai_state_generated_pdftexextra)
                .strpool
                .add(start as usize),
            bytes.len(),
        );
    }
}

unsafe fn tex_string_bytes(s: StrNumber) -> Option<Vec<u8>> {
    unsafe {
        let ptr = makecstring(s);
        let bytes = c_string_bytes(ptr);
        if bytes.is_empty() {
            None
        } else {
            Some(bytes.to_vec())
        }
    }
}

pub unsafe fn colorstackused() -> c_int {
    unsafe { (*init_colorstacks()).len() as c_int }
}

pub unsafe fn newcolorstack(s: Integer, literal_mode: Integer, page_start: Boolean) -> c_int {
    unsafe {
        let stacks = &mut *init_colorstacks();
        if stacks.len() == MAX_COLORSTACKS {
            return -1;
        }
        let initial = tex_string_bytes(s);
        let num = stacks.len();
        stacks.push(ColorStack {
            page_stack: Vec::new(),
            form_stack: Vec::new(),
            page_current: initial.clone(),
            form_current: initial.clone(),
            form_init: initial,
            literal_mode,
            page_start,
        });
        num as c_int
    }
}

pub unsafe fn colorstackset(colstack_no: c_int, s: Integer) -> Integer {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        let stacks = &mut *init_colorstacks();
        let stack = &mut stacks[colstack_no as usize];
        if (*__tekai_state_utils).PAGE_MODE != 0 {
            stack.page_current = tex_string_bytes(s);
        } else {
            stack.form_current = tex_string_bytes(s);
        }
        stack.literal_mode
    }
}

pub unsafe fn colorstackcurrent(colstack_no: c_int) -> Integer {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        let stacks = &mut *init_colorstacks();
        let stack = &mut stacks[colstack_no as usize];
        if (*__tekai_state_utils).PAGE_MODE != 0 {
            pool_put_bytes(
                (*__tekai_state_generated_pdftexextra).poolptr,
                stack.page_current.as_deref(),
            );
        } else {
            pool_put_bytes(
                (*__tekai_state_generated_pdftexextra).poolptr,
                stack.form_current.as_deref(),
            );
        }
        stack.literal_mode
    }
}

pub unsafe fn colorstackpush(colstack_no: c_int, s: Integer) -> Integer {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        let stacks = &mut *init_colorstacks();
        let stack = &mut stacks[colstack_no as usize];
        if (*__tekai_state_utils).PAGE_MODE != 0 {
            stack.page_stack.push(stack.page_current.take());
            stack.page_current = tex_string_bytes(s);
        } else {
            stack.form_stack.push(stack.form_current.take());
            stack.form_current = tex_string_bytes(s);
        }
        stack.literal_mode
    }
}

pub unsafe fn colorstackpop(colstack_no: c_int) -> Integer {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        let stacks = &mut *init_colorstacks();
        let stack = &mut stacks[colstack_no as usize];
        if (*__tekai_state_utils).PAGE_MODE != 0 {
            let Some(value) = stack.page_stack.pop() else {
                pdftex_warn_args(
                    c"pop empty color page stack %u".as_ptr(),
                    &[PrintfArg::from(colstack_no as c_uint)],
                );
                return stack.literal_mode;
            };
            stack.page_current = value;
            pool_put_bytes(
                (*__tekai_state_generated_pdftexextra).poolptr,
                stack.page_current.as_deref(),
            );
        } else {
            let Some(value) = stack.form_stack.pop() else {
                pdftex_warn_args(
                    c"pop empty color form stack %u".as_ptr(),
                    &[PrintfArg::from(colstack_no as c_uint)],
                );
                return stack.literal_mode;
            };
            stack.form_current = value;
            pool_put_bytes(
                (*__tekai_state_generated_pdftexextra).poolptr,
                stack.form_current.as_deref(),
            );
        }
        stack.literal_mode
    }
}

unsafe fn colorstackpagestart() {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        if (*__tekai_state_utils).PAGE_MODE != 0 {
            return;
        }
        for stack in (&mut *init_colorstacks()).iter_mut() {
            stack.form_stack.clear();
            stack.form_current = stack.form_init.clone();
        }
    }
}

pub unsafe fn colorstackskippagestart(colstack_no: c_int) -> Integer {
    unsafe {
        let stack = &(&*init_colorstacks())[colstack_no as usize];
        if stack.page_start == 0 {
            return 1;
        }
        let Some(current) = stack.page_current.as_deref() else {
            return 0;
        };
        if current == COLOR_DEFAULT {
            return 2;
        }
        0
    }
}

pub unsafe fn matrixused() -> Boolean {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe { (!(*__tekai_state_utils).MATRIX_STACK.is_empty()) as Boolean }
}

pub unsafe fn checkpdfsave(cur_h: c_int, cur_v: c_int) {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        (*__tekai_state_utils).POS_STACK.push(PosEntry {
            pos_h: cur_h,
            pos_v: cur_v,
            matrix_stack: if (*__tekai_state_utils).PAGE_MODE != 0 {
                (*__tekai_state_utils).MATRIX_STACK.len()
            } else {
                0
            },
        });
    }
}

pub unsafe fn checkpdfrestore(cur_h: c_int, cur_v: c_int) {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        let Some(pos) = (*__tekai_state_utils).POS_STACK.pop() else {
            pdftex_warn_args(
                c"%s".as_ptr(),
                &[PrintfArg::from(c"\\pdfrestore: missing \\pdfsave".as_ptr())],
            );
            return;
        };
        let diff_h = cur_h - pos.pos_h;
        let diff_v = cur_v - pos.pos_v;
        if diff_h != 0 || diff_v != 0 {
            pdftex_warn_args(
                c"Misplaced \\pdfrestore by (%usp, %usp)".as_ptr(),
                &[
                    PrintfArg::from(diff_h as c_uint),
                    PrintfArg::from(diff_v as c_uint),
                ],
            );
        }
        if (*__tekai_state_utils).PAGE_MODE != 0 {
            (*__tekai_state_utils)
                .MATRIX_STACK
                .truncate(pos.matrix_stack);
        }
    }
}

pub unsafe fn pdfshipoutbegin(shipping_page: Boolean) {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        (*__tekai_state_utils).POS_STACK.clear();
        (*__tekai_state_utils).PAGE_MODE = shipping_page;
        if shipping_page != 0 {
            colorstackpagestart();
        }
    }
}

pub unsafe fn pdfshipoutend(shipping_page: Boolean) {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        if !(*__tekai_state_utils).POS_STACK.is_empty() {
            pdftex_fail_args(
                c"%u unmatched \\pdfsave after %s shipout".as_ptr(),
                &[
                    PrintfArg::from((*__tekai_state_utils).POS_STACK.len() as c_uint),
                    PrintfArg::from(if shipping_page != 0 {
                        c"page".as_ptr()
                    } else {
                        c"form".as_ptr()
                    }),
                ],
            );
        }
    }
}

pub unsafe fn pdfsetmatrix(input: PoolPointer, cur_h: Scaled, cur_v: Scaled) -> Integer {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        if (*__tekai_state_utils).PAGE_MODE == 0 {
            return 1;
        }
        let len = ((*__tekai_state_generated_pdftexextra).poolptr - input).max(0) as usize;
        let bytes = slice::from_raw_parts(
            (*__tekai_state_generated_pdftexextra)
                .strpool
                .add(input as usize),
            len,
        );
        let text = String::from_utf8_lossy(bytes);
        let mut parts = text.split_whitespace();
        let (Some(a), Some(b), Some(c), Some(d), None) = (
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
        ) else {
            return 0;
        };
        let (Ok(a), Ok(b), Ok(c), Ok(d)) = (
            a.parse::<f64>(),
            b.parse::<f64>(),
            c.parse::<f64>(),
            d.parse::<f64>(),
        ) else {
            return 0;
        };
        let x = MatrixEntry {
            a,
            b,
            c,
            d,
            e: cur_h as f64 * (1.0 - a) - cur_v as f64 * c,
            f: cur_v as f64 * (1.0 - d) - cur_h as f64 * b,
        };
        let z = if let Some(y) = (*__tekai_state_utils).MATRIX_STACK.last().copied() {
            MatrixEntry {
                a: x.a * y.a + x.b * y.c,
                b: x.a * y.b + x.b * y.d,
                c: x.c * y.a + x.d * y.c,
                d: x.c * y.b + x.d * y.d,
                e: x.e * y.a + x.f * y.c + y.e,
                f: x.e * y.b + x.f * y.d + y.f,
            }
        } else {
            x
        };
        (*__tekai_state_utils).MATRIX_STACK.push(z);
        1
    }
}

fn matrix_round(x: f64) -> Scaled {
    if x > 0.0 {
        (x + 0.5) as Scaled
    } else {
        (x - 0.5) as Scaled
    }
}

unsafe fn do_matrixtransform(x: Scaled, y: Scaled) -> (Scaled, Scaled) {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        let m = *(*__tekai_state_utils)
            .MATRIX_STACK
            .last()
            .expect("matrix exists");
        let x_old = x as f64;
        let y_old = y as f64;
        (
            matrix_round(x_old * m.a + y_old * m.c + m.e),
            matrix_round(x_old * m.b + y_old * m.d + m.f),
        )
    }
}

pub unsafe fn matrixtransformrect(llx: Scaled, lly: Scaled, urx: Scaled, ury: Scaled) {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        if (*__tekai_state_utils).PAGE_MODE != 0 && !(*__tekai_state_utils).MATRIX_STACK.is_empty()
        {
            (*__tekai_state_utils).LAST_LLX = llx;
            (*__tekai_state_utils).LAST_LLY = lly;
            (*__tekai_state_utils).LAST_URX = urx;
            (*__tekai_state_utils).LAST_URY = ury;
            let (x1, y1) = do_matrixtransform(llx, lly);
            let (x2, y2) = do_matrixtransform(llx, ury);
            let (x3, y3) = do_matrixtransform(urx, lly);
            let (x4, y4) = do_matrixtransform(urx, ury);
            (*__tekai_state_utils).RET_LLX = x1.min(x2).min(x3.min(x4));
            (*__tekai_state_utils).RET_LLY = y1.min(y2).min(y3.min(y4));
            (*__tekai_state_utils).RET_URX = x1.max(x2).max(x3.max(x4));
            (*__tekai_state_utils).RET_URY = y1.max(y2).max(y3.max(y4));
        } else {
            (*__tekai_state_utils).RET_LLX = llx;
            (*__tekai_state_utils).RET_LLY = lly;
            (*__tekai_state_utils).RET_URX = urx;
            (*__tekai_state_utils).RET_URY = ury;
        }
    }
}

pub unsafe fn matrixtransformpoint(x: Scaled, y: Scaled) {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        if (*__tekai_state_utils).PAGE_MODE != 0 && !(*__tekai_state_utils).MATRIX_STACK.is_empty()
        {
            let (x, y) = do_matrixtransform(x, y);
            (*__tekai_state_utils).RET_LLX = x;
            (*__tekai_state_utils).RET_LLY = y;
        } else {
            (*__tekai_state_utils).RET_LLX = x;
            (*__tekai_state_utils).RET_LLY = y;
        }
    }
}

pub unsafe fn matrixrecalculate(urx: Scaled) {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe {
        matrixtransformrect(
            (*__tekai_state_utils).LAST_LLX,
            (*__tekai_state_utils).LAST_LLY,
            urx,
            (*__tekai_state_utils).LAST_URY,
        );
    }
}

pub unsafe fn getllx() -> Scaled {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe { (*__tekai_state_utils).RET_LLX }
}

pub unsafe fn getlly() -> Scaled {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe { (*__tekai_state_utils).RET_LLY }
}

pub unsafe fn geturx() -> Scaled {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe { (*__tekai_state_utils).RET_URX }
}

pub unsafe fn getury() -> Scaled {
    let __tekai_state_utils: *mut crate::utils::State = unsafe { crate::utils::state() };

    unsafe { (*__tekai_state_utils).RET_URY }
}

pub unsafe fn stripzeros(a: *mut c_char) -> *mut c_char {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum State {
        Nonum,
        Dotnonum,
        Int,
        Dot,
        Leaddot,
        Frac,
    }
    unsafe {
        let mut s = State::Nonum;
        let mut t = State::Nonum;
        let mut p = a;
        let mut q = a;
        let mut r = a;
        while *p != 0 {
            let ch = *p as u8;
            match s {
                State::Nonum => {
                    if ch.is_ascii_digit() {
                        s = State::Int;
                    } else if ch == b'.' {
                        s = State::Leaddot;
                    }
                }
                State::Dotnonum => {
                    if ch != b'.' && !ch.is_ascii_digit() {
                        s = State::Nonum;
                    }
                }
                State::Int => {
                    if ch == b'.' {
                        s = State::Dot;
                    } else if !ch.is_ascii_digit() {
                        s = State::Nonum;
                    }
                }
                State::Dot | State::Leaddot => {
                    if ch.is_ascii_digit() {
                        s = State::Frac;
                    } else if ch == b'.' {
                        s = State::Dotnonum;
                    } else {
                        s = State::Nonum;
                    }
                }
                State::Frac => {
                    if ch == b'.' {
                        s = State::Dotnonum;
                    } else if !ch.is_ascii_digit() {
                        s = State::Nonum;
                    }
                }
            }
            match s {
                State::Dot => r = q,
                State::Leaddot => r = q.add(1),
                State::Frac => {
                    if ch > b'0' {
                        r = q.add(1);
                    }
                }
                State::Nonum => {
                    if (t == State::Frac || t == State::Dot) && r != a {
                        q = r;
                        r = r.sub(1);
                        if *r == b'.' as c_char {
                            *r = b'0' as c_char;
                        }
                        r = a;
                    }
                }
                _ => {}
            }
            *q = *p;
            q = q.add(1);
            p = p.add(1);
            t = s;
        }
        *q = 0;
        a
    }
}

#[cfg(test)]
mod tests {
    use super::{
        format_printf, grow_array, growth_capacity, regex_offset_prefix, resource_name_prefix,
        snprintf_args, sprintf_args, PrintfArg,
    };
    use proptest::prelude::*;
    use std::ptr;

    #[test]
    fn resource_prefix_preserves_zero_crc_and_legacy_digit_order() {
        assert_eq!(resource_name_prefix(b""), *b"000000");
        // The classic IEEE CRC-32 test vector has checksum 0xcbf43926.
        let alphabet = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        let checksum = 0xcbf4_3926u32;
        let expected = std::array::from_fn(|position| {
            alphabet[((checksum / 62u32.pow(position as u32)) % 62) as usize]
        });
        assert_eq!(resource_name_prefix(b"123456789"), expected);
    }

    proptest! {
        #[test]
        fn resource_prefix_matches_prior_rust_zlib_adapter(
            id in proptest::collection::vec(any::<u8>(), 0..16_384),
        ) {
            let prior = unsafe { crate::zlib::crc32(0, id.as_ptr(), id.len() as u32) } as u32;
            prop_assert_eq!(crc32fast::hash(&id), prior);
            let alphabet = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
            let expected: [u8; 6] = std::array::from_fn(|position| {
                alphabet[((u64::from(prior) / 62u64.pow(position as u32)) % 62) as usize]
            });
            prop_assert_eq!(resource_name_prefix(&id), expected);
        }
    }

    #[test]
    fn growth_rejects_capacity_overflow_before_allocating() {
        assert_eq!(growth_capacity(8, 3, 2, 4), Some(8));
        assert_eq!(growth_capacity(8, 8, 1, 4), Some(16));
        assert_eq!(growth_capacity(8, 8, 30, 4), Some(38));
        assert_eq!(growth_capacity(0, 0, 0, 4), Some(4));
        assert_eq!(growth_capacity(8, usize::MAX, 1, 4), None);
        assert_eq!(growth_capacity(usize::MAX - 1, usize::MAX - 1, 1, 4), None);
    }

    #[test]
    fn regex_offsets_use_bounded_decimal_prefixes() {
        let mut output = [0; 24];
        assert_eq!(regex_offset_prefix(0, &mut output), b"0->");
        assert_eq!(regex_offset_prefix(123, &mut output), b"123->");
        assert_eq!(
            regex_offset_prefix(usize::MAX, &mut output),
            format!("{}->", usize::MAX).as_bytes()
        );
    }

    proptest! {
        #[test]
        fn regex_offset_prefix_matches_decimal_format(
            offset in any::<usize>(),
        ) {
            let mut output = [0; 24];
            let expected = format!("{offset}->");
            prop_assert_eq!(regex_offset_prefix(offset, &mut output), expected.as_bytes());
        }

        #[test]
        fn typed_growth_preserves_elements_and_cursor(
            operations in prop::collection::vec((0usize..65, any::<i32>()), 1..64),
        ) {
            struct OwnedArray(*mut i32);
            impl Drop for OwnedArray {
                fn drop(&mut self) {
                    unsafe { crate::allocation::free(self.0.cast()) };
                }
            }

            let mut array = OwnedArray(ptr::null_mut());
            let mut cursor = ptr::null_mut();
            let mut capacity = 0;
            let mut expected = Vec::new();
            for (extra, value) in operations {
                unsafe {
                    grow_array(&mut array.0, &mut cursor, &mut capacity, extra, 2,
                        c"test array exceeds size limit".as_ptr());
                }
                prop_assert!(capacity >= expected.len() + extra);
                prop_assert_eq!(unsafe { cursor.offset_from(array.0) } as usize, expected.len());
                let prefix = unsafe { std::slice::from_raw_parts(array.0, expected.len()) };
                prop_assert_eq!(prefix, expected.as_slice());
                for _ in 0..extra {
                    unsafe {
                        cursor.write(value);
                        cursor = cursor.add(1);
                    }
                    expected.push(value);
                }
            }
        }
    }

    #[test]
    fn printf_helper_formats_mixed_pdf_fragments() {
        let out = unsafe {
            format_printf(
                c"/Name /%s %04X %ld %.2x %%".as_ptr(),
                &[
                    PrintfArg::from(c"Font".as_ptr()),
                    PrintfArg::from(15),
                    PrintfArg::from(123_i64),
                    PrintfArg::from(10),
                ],
            )
        };
        assert_eq!(String::from_utf8(out).unwrap(), "/Name /Font 000F 123 0a %");
    }

    #[test]
    fn printf_helper_formats_pdf_real_values() {
        let out = unsafe {
            format_printf(
                c"%.1f %.6g".as_ptr(),
                &[PrintfArg::from(1.75), PrintfArg::from(0.5)],
            )
        };
        assert_eq!(String::from_utf8(out).unwrap(), "1.8 0.5");
    }

    #[test]
    fn buffer_printf_reports_full_length_and_preserves_outside_bytes() {
        for capacity in 0..16 {
            let mut output = [b'!'; 20];
            let written = unsafe {
                snprintf_args(
                    output[2..].as_mut_ptr().cast(),
                    capacity,
                    c"%04X:%s".as_ptr(),
                    &[15.into(), PrintfArg::from(c"abc".as_ptr())],
                )
            };
            assert_eq!(written, 8);
            assert_eq!(&output[..2], b"!!");
            assert!(output[2 + capacity..].iter().all(|byte| *byte == b'!'));
            if capacity != 0 {
                let count = 8.min(capacity - 1);
                assert_eq!(&output[2..2 + count], &b"000F:abc"[..count]);
                assert_eq!(output[2 + count], 0);
            }
        }
        assert_eq!(
            unsafe { snprintf_args(std::ptr::null_mut(), 0, c"%d".as_ptr(), &[123.into()]) },
            3
        );
        let mut output = [b'!'; 8];
        assert_eq!(
            unsafe {
                sprintf_args(
                    output.as_mut_ptr().cast(),
                    c"%c:%d".as_ptr(),
                    &[0.into(), 7.into()],
                )
            },
            3
        );
        assert_eq!(&output, b"\0:7\0!!!!");
    }

    proptest::proptest! {
        #[test]
        fn bounded_snprintf_matches_native_return_and_truncation(
            value in proptest::prelude::any::<u64>(),
            capacity in 0usize..80,
            width in 0usize..64,
            precision in 0usize..32,
        ) {
            let fmt = std::ffi::CString::new(format!("%#{width}.{precision}llX")).unwrap();
            let mut native = [b'!'; 80];
            let mut rust = native;
            let expected = unsafe { libc::snprintf(native.as_mut_ptr().cast(), capacity, fmt.as_ptr(), value) };
            let actual = unsafe { snprintf_args(rust.as_mut_ptr().cast(), capacity, fmt.as_ptr(), &[value.into()]) };
            proptest::prop_assert_eq!(actual, expected);
            proptest::prop_assert_eq!(rust, native);
        }
    }
}

pub(crate) struct State {
    pub(crate) cur_file_name: *mut c_char,
    pub(crate) last_ptr_index: SizeT,
    fb_ptr: *mut c_char,
    pub(crate) fb_array: *mut c_char,
    fb_limit: SizeT,
    LAST_TEX_STRING: StrNumber,
    JOB_ID_STRING: *mut c_char,
    JOBNAME_CSTR: *mut c_char,
    PSTRING_BUF: [c_char; MAX_PSTRING_LEN],
    VF_E_FNTS_PTR: *mut Integer,
    VF_E_FNTS_ARRAY: *mut Integer,
    VF_E_FNTS_LIMIT: SizeT,
    VF_I_FNTS_PTR: *mut InternalFontNumber,
    VF_I_FNTS_ARRAY: *mut InternalFontNumber,
    VF_I_FNTS_LIMIT: SizeT,
    PAGE_MODE: Boolean,
    COLOR_STACKS: Option<Vec<ColorStack>>,
    MATRIX_STACK: Vec<MatrixEntry>,
    POS_STACK: Vec<PosEntry>,
    RET_LLX: Scaled,
    RET_LLY: Scaled,
    RET_URX: Scaled,
    RET_URY: Scaled,
    LAST_LLX: Scaled,
    LAST_LLY: Scaled,
    LAST_URX: Scaled,
    LAST_URY: Scaled,
    SUBSET_TAG_TREE: *mut AvlTable,
    local_makepdftexbanner_793_PDFTEX_BANNER_INIT: Boolean,
}

#[inline(always)]
fn __tekai_require_copy<T: Copy>() {}

pub(crate) struct StateInitialization {
    destination: *mut State,
    initialized: [bool; 29],
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
            if self.initialized[28] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).local_makepdftexbanner_793_PDFTEX_BANNER_INIT
                ));
            }
            if self.initialized[27] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).SUBSET_TAG_TREE));
            }
            if self.initialized[26] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).LAST_URY));
            }
            if self.initialized[25] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).LAST_URX));
            }
            if self.initialized[24] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).LAST_LLY));
            }
            if self.initialized[23] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).LAST_LLX));
            }
            if self.initialized[22] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).RET_URY));
            }
            if self.initialized[21] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).RET_URX));
            }
            if self.initialized[20] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).RET_LLY));
            }
            if self.initialized[19] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).RET_LLX));
            }
            if self.initialized[18] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).POS_STACK));
            }
            if self.initialized[17] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).MATRIX_STACK));
            }
            if self.initialized[16] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).COLOR_STACKS));
            }
            if self.initialized[15] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).PAGE_MODE));
            }
            if self.initialized[14] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).VF_I_FNTS_LIMIT));
            }
            if self.initialized[13] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).VF_I_FNTS_ARRAY));
            }
            if self.initialized[12] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).VF_I_FNTS_PTR));
            }
            if self.initialized[11] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).VF_E_FNTS_LIMIT));
            }
            if self.initialized[10] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).VF_E_FNTS_ARRAY));
            }
            if self.initialized[9] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).VF_E_FNTS_PTR));
            }
            if self.initialized[8] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).PSTRING_BUF));
            }
            if self.initialized[7] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).JOBNAME_CSTR));
            }
            if self.initialized[6] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).JOB_ID_STRING));
            }
            if self.initialized[5] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).LAST_TEX_STRING));
            }
            if self.initialized[4] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fb_limit));
            }
            if self.initialized[3] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fb_array));
            }
            if self.initialized[2] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fb_ptr));
            }
            if self.initialized[1] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).last_ptr_index));
            }
            if self.initialized[0] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).cur_file_name));
            }
        }
    }
}
impl StateInitialization {
    pub(crate) unsafe fn begin(destination: *mut State) -> Self {
        Self {
            destination,
            initialized: [false; 29],
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
            assert!(core::mem::size_of::<*mut c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).cur_file_name).write(ptr::null_mut());
        self.initialized[0] = true;
        const {
            assert!(core::mem::size_of::<SizeT>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).last_ptr_index).write(0);
        self.initialized[1] = true;
        const {
            assert!(core::mem::size_of::<*mut c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fb_ptr).write(ptr::null_mut());
        self.initialized[2] = true;
        const {
            assert!(core::mem::size_of::<*mut c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fb_array).write(ptr::null_mut());
        self.initialized[3] = true;
        const {
            assert!(core::mem::size_of::<SizeT>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fb_limit).write(0);
        self.initialized[4] = true;
        const {
            assert!(core::mem::size_of::<StrNumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).LAST_TEX_STRING).write(0);
        self.initialized[5] = true;
        const {
            assert!(core::mem::size_of::<*mut c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).JOB_ID_STRING).write(ptr::null_mut());
        self.initialized[6] = true;
        const {
            assert!(core::mem::size_of::<*mut c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).JOBNAME_CSTR).write(ptr::null_mut());
        self.initialized[7] = true;
        const {
            assert!((MAX_PSTRING_LEN) == (MAX_PSTRING_LEN));
            assert!(core::mem::size_of::<c_char>() <= 16 * 1024);
        }
        __tekai_require_copy::<c_char>();
        {
            let __tekai_repeat_0: c_char = 0;
            for __tekai_index_0 in 0..(MAX_PSTRING_LEN) {
                core::ptr::addr_of_mut!((*destination).PSTRING_BUF[__tekai_index_0])
                    .write(__tekai_repeat_0);
            }
        }
        self.initialized[8] = true;
        const {
            assert!(core::mem::size_of::<*mut Integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).VF_E_FNTS_PTR).write(ptr::null_mut());
        self.initialized[9] = true;
        const {
            assert!(core::mem::size_of::<*mut Integer>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).VF_E_FNTS_ARRAY).write(ptr::null_mut());
        self.initialized[10] = true;
        const {
            assert!(core::mem::size_of::<SizeT>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).VF_E_FNTS_LIMIT).write(0);
        self.initialized[11] = true;
        const {
            assert!(core::mem::size_of::<*mut InternalFontNumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).VF_I_FNTS_PTR).write(ptr::null_mut());
        self.initialized[12] = true;
        const {
            assert!(core::mem::size_of::<*mut InternalFontNumber>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).VF_I_FNTS_ARRAY).write(ptr::null_mut());
        self.initialized[13] = true;
        const {
            assert!(core::mem::size_of::<SizeT>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).VF_I_FNTS_LIMIT).write(0);
        self.initialized[14] = true;
        const {
            assert!(core::mem::size_of::<Boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).PAGE_MODE).write(FALSE);
        self.initialized[15] = true;
        const {
            assert!(core::mem::size_of::<Option<Vec<ColorStack>>>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).COLOR_STACKS).write(None);
        self.initialized[16] = true;
        const {
            assert!(core::mem::size_of::<Vec<MatrixEntry>>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).MATRIX_STACK).write(Vec::new());
        self.initialized[17] = true;
        const {
            assert!(core::mem::size_of::<Vec<PosEntry>>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).POS_STACK).write(Vec::new());
        self.initialized[18] = true;
        const {
            assert!(core::mem::size_of::<Scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).RET_LLX).write(0);
        self.initialized[19] = true;
        const {
            assert!(core::mem::size_of::<Scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).RET_LLY).write(0);
        self.initialized[20] = true;
        const {
            assert!(core::mem::size_of::<Scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).RET_URX).write(0);
        self.initialized[21] = true;
        const {
            assert!(core::mem::size_of::<Scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).RET_URY).write(0);
        self.initialized[22] = true;
        const {
            assert!(core::mem::size_of::<Scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).LAST_LLX).write(0);
        self.initialized[23] = true;
        const {
            assert!(core::mem::size_of::<Scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).LAST_LLY).write(0);
        self.initialized[24] = true;
        const {
            assert!(core::mem::size_of::<Scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).LAST_URX).write(0);
        self.initialized[25] = true;
        const {
            assert!(core::mem::size_of::<Scaled>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).LAST_URY).write(0);
        self.initialized[26] = true;
        const {
            assert!(core::mem::size_of::<*mut AvlTable>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).SUBSET_TAG_TREE).write(ptr::null_mut());
        self.initialized[27] = true;
        const {
            assert!(core::mem::size_of::<Boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).local_makepdftexbanner_793_PDFTEX_BANNER_INIT)
            .write(FALSE);
        self.initialized[28] = true;
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
        (*crate::invocation_state::current()).utils
    ))
}
#[cfg(test)]
pub(crate) unsafe fn check_state_construction(phase: u8) {
    let invocation = crate::invocation_state::current();
    let destination = state();
    assert!(!destination.is_null());
    let _ = (invocation, phase);
}
