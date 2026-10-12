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
pub use crate::generated::pdftexextra::{initstarttime, makecstring};

pub use crate::generated::backend::avl::{
    avl_comparison_func as AvlComparisonFunc, avl_create, avl_find, avl_node as AvlNode, avl_probe,
    avl_t_first, avl_t_init, avl_t_next, avl_table as AvlTable, avl_traverser as AvlTraverser,
    libavl_allocator as LibAvlAllocator,
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

extern "C" {
    static kpse_def_inst: KpathseaInstancePrefix;

    static mut pdfbuf: *mut c_uchar;
    static mut pdfbufsize: Integer;
    static mut pdfptr: Integer;
    static mut pdfosmode: Boolean;
    static mut pdflastbyte: c_uchar;
    static mut pdfgone: LongInteger;
    static mut pdffile: *mut FILE;
    static mut outputfilename: StrNumber;
    static mut fixedpdfdraftmode: Integer;

    static mut poolsize: Integer;
    static mut poolptr: PoolPointer;
    static mut strpool: *mut c_uchar;
    static mut jobname: StrNumber;
    static mut formatident: StrNumber;
    static mut pdftexbanner: StrNumber;
    static mut lasttokensstring: StrNumber;
    static mut start_time_str: [c_char; 30];
    static mut vfefnts: *mut Integer;
    static mut vfifnts: *mut InternalFontNumber;
    static mut fontmax: Integer;
    static ptexbanner: *const c_char;
    static versionstring: *const c_char;
    static kpathsea_version_string: *const c_char;

    static mut avl_xallocator: LibAvlAllocator;

}
pub use crate::backend_types::{
    fd_entry as FdEntry, fe_entry as FeEntry, fm_entry as FmEntry, intparm as IntParm,
};
use crate::kpathsea_types::kpathsea_instance as KpathseaInstancePrefix;

#[no_mangle]
pub static mut cur_file_name: *mut c_char = ptr::null_mut();

#[no_mangle]
pub static mut last_ptr_index: SizeT = 0;

#[no_mangle]
pub static mut fb_ptr: *mut c_char = ptr::null_mut();
#[no_mangle]
pub static mut fb_array: *mut c_char = ptr::null_mut();
#[no_mangle]
pub static mut fb_limit: SizeT = 0;

static mut LAST_TEX_STRING: StrNumber = 0;
static mut JOB_ID_STRING: *mut c_char = ptr::null_mut();
static mut JOBNAME_CSTR: *mut c_char = ptr::null_mut();
static mut PSTRING_BUF: [c_char; MAX_PSTRING_LEN] = [0; MAX_PSTRING_LEN];

static mut VF_E_FNTS_PTR: *mut Integer = ptr::null_mut();
static mut VF_E_FNTS_ARRAY: *mut Integer = ptr::null_mut();
static mut VF_E_FNTS_LIMIT: SizeT = 0;
static mut VF_I_FNTS_PTR: *mut InternalFontNumber = ptr::null_mut();
static mut VF_I_FNTS_ARRAY: *mut InternalFontNumber = ptr::null_mut();
static mut VF_I_FNTS_LIMIT: SizeT = 0;

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

static mut PAGE_MODE: Boolean = FALSE;
static mut COLOR_STACKS: Option<Vec<ColorStack>> = None;
static mut MATRIX_STACK: Vec<MatrixEntry> = Vec::new();
static mut POS_STACK: Vec<PosEntry> = Vec::new();
static mut RET_LLX: Scaled = 0;
static mut RET_LLY: Scaled = 0;
static mut RET_URX: Scaled = 0;
static mut RET_URY: Scaled = 0;
static mut LAST_LLX: Scaled = 0;
static mut LAST_LLY: Scaled = 0;
static mut LAST_URX: Scaled = 0;
static mut LAST_URY: Scaled = 0;
static mut SUBSET_TAG_TREE: *mut AvlTable = ptr::null_mut();

unsafe fn kpse_invocation_name() -> *const c_char {
    unsafe {
        if kpse_def_inst.invocation_name.is_null() {
            c"pdftex".as_ptr()
        } else {
            kpse_def_inst.invocation_name
        }
    }
}

unsafe fn kpathsea_debug_enabled() -> bool {
    unsafe { kpse_def_inst.debug != 0 }
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
    unsafe {
        if poolptr as usize + extra >= poolsize as usize {
            poolptr = poolsize;
            false
        } else {
            true
        }
    }
}

unsafe fn pdf_room(n: usize) {
    unsafe {
        if n + pdfptr as usize > pdfbufsize as usize {
            if pdfosmode != 0 {
                zpdfosgetosbuf(n as Integer);
            } else if n > pdfbufsize as usize {
                pdftex_fail_args(c"PDF output buffer overflowed".as_ptr(), &[]);
            } else {
                pdfflush();
            }
        }
    }
}

pub unsafe fn pdf_out(byte: u8) {
    unsafe {
        pdf_room(1);
        *pdfbuf.add(pdfptr as usize) = byte;
        pdfptr += 1;
        pdflastbyte = byte;
    }
}

#[inline(always)]
unsafe fn pdf_write_bytes_impl<const TRACK_LAST_BYTE: bool>(mut src: *const u8, mut len: usize) {
    unsafe {
        while len > 0 {
            if pdfptr as usize >= pdfbufsize as usize {
                if pdfosmode != 0 {
                    zpdfosgetosbuf(len.min(pdfbufsize as usize) as Integer);
                } else {
                    pdfflush();
                }
            }

            let available = (pdfbufsize - pdfptr) as usize;
            if available == 0 {
                continue;
            }
            let chunk = len.min(available);
            ptr::copy_nonoverlapping(src, pdfbuf.add(pdfptr as usize), chunk);
            pdfptr += chunk as Integer;
            if TRACK_LAST_BYTE {
                pdflastbyte = *src.add(chunk - 1);
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
    if s.is_null() {
        return;
    }
    let bytes = unsafe { CStr::from_ptr(s).to_bytes() };
    unsafe {
        pdf_room(bytes.len() + 1);
        ptr::copy_nonoverlapping(bytes.as_ptr(), pdfbuf.add(pdfptr as usize), bytes.len());
        pdfptr += bytes.len() as Integer;
        if let Some(&byte) = bytes.last() {
            pdflastbyte = byte;
        }
    }
}

pub unsafe fn pdf_newline() {
    unsafe {
        if pdflastbyte != b'\n' {
            pdf_puts(c"\n".as_ptr());
        }
    }
}

pub unsafe fn maketexstring(s: *const c_char) -> StrNumber {
    unsafe {
        if s.is_null() || *s == 0 {
            return getnullstr();
        }
        let bytes = CStr::from_ptr(s).to_bytes();
        if !check_pool_room(bytes.len()) {
            pdftex_fail_args(c"string pool overflow".as_ptr(), &[]);
        }
        ptr::copy_nonoverlapping(bytes.as_ptr(), strpool.add(poolptr as usize), bytes.len());
        poolptr += bytes.len() as PoolPointer;
        LAST_TEX_STRING = makestring();
        LAST_TEX_STRING
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
    let bytes = unsafe { format_printf(fmt, args) };
    unsafe {
        println();
        println();
        tex_printf_args(
            c"pdfTeX warning: %s".as_ptr(),
            &[PrintfArg::from(kpse_invocation_name())],
        );
        if !cur_file_name.is_null() {
            tex_printf_args(c" (file %s)".as_ptr(), &[PrintfArg::from(cur_file_name)]);
        }
        tex_printf_args(c": ".as_ptr(), &[]);
        safe_print_bytes(&bytes);
        println();
    }
}

pub unsafe fn pdftex_fail_args(fmt: *const c_char, args: &[PrintfArg]) -> ! {
    let bytes = unsafe { format_printf(fmt, args) };
    unsafe {
        println();
        safe_print(c"!pdfTeX error: ".as_ptr());
        safe_print(kpse_invocation_name());
        if !cur_file_name.is_null() {
            safe_print(c" (file ".as_ptr());
            safe_print(cur_file_name);
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
    unsafe {
        if !kpathsea_debug_enabled() && outputfilename != 0 && fixedpdfdraftmode == 0 {
            let name = makecstring(outputfilename);
            xfclose(pdffile, name);
            libc::remove(name);
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
    unsafe {
        if JOBNAME_CSTR.is_null() {
            JOBNAME_CSTR = c_strdup(makecstring(jobname));
        }
        if fixedpdfdraftmode == 0 {
            if crate::stdio::fseeko(pdffile, offset, libc::SEEK_SET) != 0 {
                pdftex_fail_args(c"fseeko() failed".as_ptr(), &[]);
            }
            crate::stdio::fprintf_args(
                pdffile,
                c"%lli".as_ptr(),
                &[crate::utils::PrintfArg::from(length as c_longlong)],
            );
            if crate::stdio::fseeko(pdffile, pdfgone + pdfptr as LongInteger, libc::SEEK_SET) != 0 {
                pdftex_fail_args(c"fseeko() failed".as_ptr(), &[]);
            }
        }
    }
}

pub unsafe fn convertStringToPDFString(input: *const c_char, len: c_int) -> *mut c_char {
    unsafe {
        let out = &raw mut PSTRING_BUF as *mut c_char;
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
    unsafe {
        let out = poolptr;
        let mut pos = input;
        while pos < out {
            if !check_pool_room(4) {
                return;
            }
            let ch = *strpool.add(pos as usize);
            pos += 1;
            if !(b'!'..=b'~').contains(&ch) {
                let frag = format!("\\{:03o}", ch);
                for &byte in frag.as_bytes() {
                    *strpool.add(poolptr as usize) = byte;
                    poolptr += 1;
                }
                continue;
            }
            if ch == b'(' || ch == b')' || ch == b'\\' {
                *strpool.add(poolptr as usize) = b'\\';
                poolptr += 1;
            }
            *strpool.add(poolptr as usize) = ch;
            poolptr += 1;
        }
    }
}

pub unsafe fn escapename(input: PoolPointer) {
    unsafe {
        let out = poolptr;
        let mut pos = input;
        while pos < out {
            if !check_pool_room(3) {
                return;
            }
            let ch = *strpool.add(pos as usize);
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
                    *strpool.add(poolptr as usize) = byte;
                    poolptr += 1;
                }
            } else {
                *strpool.add(poolptr as usize) = ch;
                poolptr += 1;
            }
        }
    }
}

pub unsafe fn escapehex(input: PoolPointer) {
    unsafe {
        let out = poolptr;
        let mut pos = input;
        while pos < out {
            if !check_pool_room(2) {
                return;
            }
            let frag = format!("{:02X}", *strpool.add(pos as usize));
            pos += 1;
            for &byte in frag.as_bytes() {
                *strpool.add(poolptr as usize) = byte;
                poolptr += 1;
            }
        }
    }
}

pub unsafe fn unescapehex(input: PoolPointer) {
    unsafe {
        let out = poolptr;
        let mut pos = input;
        let mut first = true;
        let mut high = 0u8;
        while pos < out {
            if !check_pool_room(1) {
                return;
            }
            let ch = *strpool.add(pos as usize);
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
                *strpool.add(poolptr as usize) = high + val;
                poolptr += 1;
                first = true;
            }
        }
        if !first {
            *strpool.add(poolptr as usize) = high;
            poolptr += 1;
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
        initstarttime();
        let start = &raw mut start_time_str as *mut c_char;
        md5_append(state_ptr, start as *const u8, c_str_len(start) as c_int);
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
    unsafe {
        let tex_string = ztokenstostring(toks);
        let s = makecstring(tex_string);
        zflushstr(lasttokensstring);
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
        initstarttime();
        pdf_printf_args(
            c"/CreationDate (%s)\n".as_ptr(),
            &[PrintfArg::from(&raw mut start_time_str as *mut c_char)],
        );
    }
}

pub unsafe fn printmoddate() {
    unsafe {
        initstarttime();
        pdf_printf_args(
            c"/ModDate (%s)\n".as_ptr(),
            &[PrintfArg::from(&raw mut start_time_str as *mut c_char)],
        );
    }
}

pub unsafe fn setjobid(year: c_int, month: c_int, day: c_int, time: c_int) {
    unsafe {
        if !JOB_ID_STRING.is_null() {
            return;
        }
        let name = c_string_bytes(makecstring(jobname));
        let format = c_string_bytes(makecstring(formatident));
        let banner = c_string_bytes(ptexbanner);
        let version = c_string_bytes(versionstring);
        let kpse = c_string_bytes(kpathsea_version_string);
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
        JOB_ID_STRING = malloced_c_string(s.as_bytes());
    }
}

pub unsafe fn makepdftexbanner() {
    static mut PDFTEX_BANNER_INIT: Boolean = FALSE;
    unsafe {
        if PDFTEX_BANNER_INIT != 0 {
            return;
        }
        let s = format!(
            "{}{} {}",
            String::from_utf8_lossy(c_string_bytes(ptexbanner)),
            String::from_utf8_lossy(c_string_bytes(versionstring)),
            String::from_utf8_lossy(c_string_bytes(kpathsea_version_string))
        );
        let c = CString::new(s).expect("banner has no nul");
        pdftexbanner = maketexstring(c.as_ptr());
        PDFTEX_BANNER_INIT = TRUE;
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
    unsafe {
        let prefix = resource_name_prefix(c_string_bytes(JOB_ID_STRING));
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
    unsafe {
        let start = usize::try_from(poolptr).ok()?;
        let size = usize::try_from(poolsize).ok()?;
        (!strpool.is_null() && start.checked_add(extra)? < size).then_some(start)
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
    let start = prefix
        .len()
        .checked_add(bytes.len())
        .and_then(regex_pool_room);
    let Some(start) = start else {
        // The normal warning path allocates strings in this very pool.
        // Report exhaustion without recursively trying to allocate a warning.
        warn_regex_stderr(&"string pool overflow");
        unsafe { poolptr = poolsize.max(0) };
        return false;
    };
    unsafe {
        ptr::copy_nonoverlapping(prefix.as_ptr(), strpool.add(start), prefix.len());
        ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            strpool.add(start + prefix.len()),
            bytes.len(),
        );
        poolptr = (start + prefix.len() + bytes.len()) as PoolPointer;
    }
    true
}

fn warn_regex(error: &impl std::fmt::Display) {
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
        let file = if cur_file_name.is_null() {
            0
        } else {
            CStr::from_ptr(cur_file_name).to_bytes().len()
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
    unsafe { fb_ptr.offset_from(fb_array) as Integer }
}

pub unsafe fn fb_seek(offset: Integer) {
    unsafe {
        fb_ptr = fb_array.add(offset as usize);
    }
}

unsafe fn ensure_fb_room(n: usize) {
    unsafe {
        grow_array(
            &mut fb_array,
            &mut fb_ptr,
            &mut fb_limit,
            n,
            SMALL_ARRAY_SIZE,
            c"fb_array exceeds size limit".as_ptr(),
        );
    }
}

pub unsafe fn fb_putchar(b: c_uchar) {
    unsafe {
        ensure_fb_room(1);
        *fb_ptr = b as c_char;
        fb_ptr = fb_ptr.add(1);
    }
}

pub unsafe fn fb_flush() {
    unsafe {
        let mut p = fb_array;
        while p < fb_ptr {
            let mut n = (pdfbufsize - pdfptr) as usize;
            let remaining = fb_ptr.offset_from(p) as usize;
            if remaining < n {
                n = remaining;
            }
            ptr::copy_nonoverlapping(p as *const c_uchar, pdfbuf.add(pdfptr as usize), n);
            pdfptr += n as Integer;
            if pdfptr == pdfbufsize {
                pdfflush();
            }
            p = p.add(n);
        }
        fb_ptr = fb_array;
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
    unsafe {
        if VF_E_FNTS_ARRAY.is_null() {
            VF_E_FNTS_ARRAY = vfefnts;
            VF_E_FNTS_LIMIT = fontmax as SizeT;
            VF_E_FNTS_PTR = VF_E_FNTS_ARRAY;
            VF_I_FNTS_ARRAY = vfifnts;
            VF_I_FNTS_LIMIT = fontmax as SizeT;
            VF_I_FNTS_PTR = VF_I_FNTS_ARRAY;
        }
        grow_array(
            &mut VF_E_FNTS_ARRAY,
            &mut VF_E_FNTS_PTR,
            &mut VF_E_FNTS_LIMIT,
            1,
            fontmax as usize,
            c"vf_e_fnts_array exceeds size limit".as_ptr(),
        );
        VF_E_FNTS_PTR = VF_E_FNTS_PTR.add(1);
        grow_array(
            &mut VF_I_FNTS_ARRAY,
            &mut VF_I_FNTS_PTR,
            &mut VF_I_FNTS_LIMIT,
            1,
            fontmax as usize,
            c"vf_i_fnts_array exceeds size limit".as_ptr(),
        );
        VF_I_FNTS_PTR = VF_I_FNTS_PTR.add(1);
        if VF_E_FNTS_ARRAY != vfefnts {
            vfefnts = VF_E_FNTS_ARRAY;
            vfifnts = VF_I_FNTS_ARRAY;
        }
    }
}

pub unsafe fn libpdffinish() {
    unsafe {
        crate::allocation::free(fb_array as *mut c_void);
        fb_array = ptr::null_mut();
        fb_ptr = ptr::null_mut();
        crate::allocation::free(JOB_ID_STRING as *mut c_void);
        JOB_ID_STRING = ptr::null_mut();
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
    const SUBSET_TAG_LENGTH: usize = 6;
    unsafe {
        if SUBSET_TAG_TREE.is_null() {
            SUBSET_TAG_TREE = avl_create(
                Some(comp_string_entry),
                ptr::null_mut(),
                &raw mut avl_xallocator,
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
            if avl_find(SUBSET_TAG_TREE, subset as *const c_void).is_null() {
                break;
            }
            if round >= 100 {
                pdftex_fail_args(c"make_subset_tag(): subset-tag collision".as_ptr(), &[]);
            }
        }
        let slot = avl_probe(SUBSET_TAG_TREE, subset as *mut c_void);
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

unsafe fn init_colorstacks() -> &'static mut Vec<ColorStack> {
    unsafe {
        if COLOR_STACKS.is_none() {
            COLOR_STACKS = Some(vec![ColorStack {
                page_stack: Vec::new(),
                form_stack: Vec::new(),
                page_current: Some(COLOR_DEFAULT.to_vec()),
                form_current: Some(COLOR_DEFAULT.to_vec()),
                form_init: Some(COLOR_DEFAULT.to_vec()),
                literal_mode: DIRECT_ALWAYS,
                page_start: TRUE,
            }]);
        }
        COLOR_STACKS.as_mut().expect("initialized")
    }
}

unsafe fn pool_put_bytes(start: PoolPointer, bytes: Option<&[u8]>) {
    unsafe {
        let Some(bytes) = bytes else { return };
        if bytes.is_empty() {
            return;
        }
        poolptr = start + bytes.len() as PoolPointer;
        if poolptr >= poolsize {
            poolptr = poolsize;
            return;
        }
        ptr::copy_nonoverlapping(bytes.as_ptr(), strpool.add(start as usize), bytes.len());
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
    unsafe { init_colorstacks().len() as c_int }
}

pub unsafe fn newcolorstack(s: Integer, literal_mode: Integer, page_start: Boolean) -> c_int {
    unsafe {
        let stacks = init_colorstacks();
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
    unsafe {
        let stacks = init_colorstacks();
        let stack = &mut stacks[colstack_no as usize];
        if PAGE_MODE != 0 {
            stack.page_current = tex_string_bytes(s);
        } else {
            stack.form_current = tex_string_bytes(s);
        }
        stack.literal_mode
    }
}

pub unsafe fn colorstackcurrent(colstack_no: c_int) -> Integer {
    unsafe {
        let stacks = init_colorstacks();
        let stack = &mut stacks[colstack_no as usize];
        if PAGE_MODE != 0 {
            pool_put_bytes(poolptr, stack.page_current.as_deref());
        } else {
            pool_put_bytes(poolptr, stack.form_current.as_deref());
        }
        stack.literal_mode
    }
}

pub unsafe fn colorstackpush(colstack_no: c_int, s: Integer) -> Integer {
    unsafe {
        let stacks = init_colorstacks();
        let stack = &mut stacks[colstack_no as usize];
        if PAGE_MODE != 0 {
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
    unsafe {
        let stacks = init_colorstacks();
        let stack = &mut stacks[colstack_no as usize];
        if PAGE_MODE != 0 {
            let Some(value) = stack.page_stack.pop() else {
                pdftex_warn_args(
                    c"pop empty color page stack %u".as_ptr(),
                    &[PrintfArg::from(colstack_no as c_uint)],
                );
                return stack.literal_mode;
            };
            stack.page_current = value;
            pool_put_bytes(poolptr, stack.page_current.as_deref());
        } else {
            let Some(value) = stack.form_stack.pop() else {
                pdftex_warn_args(
                    c"pop empty color form stack %u".as_ptr(),
                    &[PrintfArg::from(colstack_no as c_uint)],
                );
                return stack.literal_mode;
            };
            stack.form_current = value;
            pool_put_bytes(poolptr, stack.form_current.as_deref());
        }
        stack.literal_mode
    }
}

unsafe fn colorstackpagestart() {
    unsafe {
        if PAGE_MODE != 0 {
            return;
        }
        for stack in init_colorstacks().iter_mut() {
            stack.form_stack.clear();
            stack.form_current = stack.form_init.clone();
        }
    }
}

pub unsafe fn colorstackskippagestart(colstack_no: c_int) -> Integer {
    unsafe {
        let stack = &init_colorstacks()[colstack_no as usize];
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
    unsafe { (!MATRIX_STACK.is_empty()) as Boolean }
}

pub unsafe fn checkpdfsave(cur_h: c_int, cur_v: c_int) {
    unsafe {
        POS_STACK.push(PosEntry {
            pos_h: cur_h,
            pos_v: cur_v,
            matrix_stack: if PAGE_MODE != 0 {
                MATRIX_STACK.len()
            } else {
                0
            },
        });
    }
}

pub unsafe fn checkpdfrestore(cur_h: c_int, cur_v: c_int) {
    unsafe {
        let Some(pos) = POS_STACK.pop() else {
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
        if PAGE_MODE != 0 {
            MATRIX_STACK.truncate(pos.matrix_stack);
        }
    }
}

pub unsafe fn pdfshipoutbegin(shipping_page: Boolean) {
    unsafe {
        POS_STACK.clear();
        PAGE_MODE = shipping_page;
        if shipping_page != 0 {
            colorstackpagestart();
        }
    }
}

pub unsafe fn pdfshipoutend(shipping_page: Boolean) {
    unsafe {
        if !POS_STACK.is_empty() {
            pdftex_fail_args(
                c"%u unmatched \\pdfsave after %s shipout".as_ptr(),
                &[
                    PrintfArg::from(POS_STACK.len() as c_uint),
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
    unsafe {
        if PAGE_MODE == 0 {
            return 1;
        }
        let len = (poolptr - input).max(0) as usize;
        let bytes = slice::from_raw_parts(strpool.add(input as usize), len);
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
        let z = if let Some(y) = MATRIX_STACK.last().copied() {
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
        MATRIX_STACK.push(z);
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
    unsafe {
        let m = *MATRIX_STACK.last().expect("matrix exists");
        let x_old = x as f64;
        let y_old = y as f64;
        (
            matrix_round(x_old * m.a + y_old * m.c + m.e),
            matrix_round(x_old * m.b + y_old * m.d + m.f),
        )
    }
}

pub unsafe fn matrixtransformrect(llx: Scaled, lly: Scaled, urx: Scaled, ury: Scaled) {
    unsafe {
        if PAGE_MODE != 0 && !MATRIX_STACK.is_empty() {
            LAST_LLX = llx;
            LAST_LLY = lly;
            LAST_URX = urx;
            LAST_URY = ury;
            let (x1, y1) = do_matrixtransform(llx, lly);
            let (x2, y2) = do_matrixtransform(llx, ury);
            let (x3, y3) = do_matrixtransform(urx, lly);
            let (x4, y4) = do_matrixtransform(urx, ury);
            RET_LLX = x1.min(x2).min(x3.min(x4));
            RET_LLY = y1.min(y2).min(y3.min(y4));
            RET_URX = x1.max(x2).max(x3.max(x4));
            RET_URY = y1.max(y2).max(y3.max(y4));
        } else {
            RET_LLX = llx;
            RET_LLY = lly;
            RET_URX = urx;
            RET_URY = ury;
        }
    }
}

pub unsafe fn matrixtransformpoint(x: Scaled, y: Scaled) {
    unsafe {
        if PAGE_MODE != 0 && !MATRIX_STACK.is_empty() {
            let (x, y) = do_matrixtransform(x, y);
            RET_LLX = x;
            RET_LLY = y;
        } else {
            RET_LLX = x;
            RET_LLY = y;
        }
    }
}

pub unsafe fn matrixrecalculate(urx: Scaled) {
    unsafe {
        matrixtransformrect(LAST_LLX, LAST_LLY, urx, LAST_URY);
    }
}

pub unsafe fn getllx() -> Scaled {
    unsafe { RET_LLX }
}

pub unsafe fn getlly() -> Scaled {
    unsafe { RET_LLY }
}

pub unsafe fn geturx() -> Scaled {
    unsafe { RET_URX }
}

pub unsafe fn getury() -> Scaled {
    unsafe { RET_URY }
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
