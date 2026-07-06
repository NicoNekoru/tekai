//! Typed printf helpers for generated pdfTeX Rust.

use libc::{
    c_char, c_double, c_int, c_long, c_longlong, c_uchar, c_uint, c_ulong, c_ulonglong, c_void,
    FILE,
};
use std::ffi::{CStr, CString};
use std::{mem, ptr, slice};

use crate::md5::{md5_append, md5_finish, md5_init, Md5State};

#[cfg(unix)]
extern "C" {
    fn getc_unlocked(stream: *mut FILE) -> c_int;
}

const PRINTF_BUF_SIZE: usize = 1024;
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
type AvlComparisonFunc = unsafe extern "C" fn(*const c_void, *const c_void, *mut c_void) -> c_int;

#[derive(Clone, Copy)]
pub enum PrintfArg {
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    Ptr(*const c_void),
}

macro_rules! signed_arg {
    ($($ty:ty),* $(,)?) => {
        $(impl From<$ty> for PrintfArg {
            fn from(value: $ty) -> Self {
                Self::Signed(value as i64)
            }
        })*
    };
}

macro_rules! unsigned_arg {
    ($($ty:ty),* $(,)?) => {
        $(impl From<$ty> for PrintfArg {
            fn from(value: $ty) -> Self {
                Self::Unsigned(value as u64)
            }
        })*
    };
}

signed_arg!(i8, i16, i32, i64, isize);
unsigned_arg!(u8, u16, u32, u64, usize);

impl From<f32> for PrintfArg {
    fn from(value: f32) -> Self {
        Self::Float(value as f64)
    }
}

impl From<f64> for PrintfArg {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl<T> From<*const T> for PrintfArg {
    fn from(value: *const T) -> Self {
        Self::Ptr(value.cast())
    }
}

impl<T> From<*mut T> for PrintfArg {
    fn from(value: *mut T) -> Self {
        Self::Ptr(value.cast())
    }
}

extern "C" {
    static mut __stdoutp: *mut FILE;
    static kpse_def_inst: KpathseaInstancePrefix;

    fn abort() -> !;
    fn exit(_: c_int) -> !;
    fn println();
    fn zflushstr(_: c_int);
    fn zprint(_: c_int);

    static mut pdfbuf: *mut c_uchar;
    static mut pdfbufsize: Integer;
    static mut pdfptr: Integer;
    static mut pdfosmode: Boolean;
    static mut pdflastbyte: c_uchar;
    static mut pdfgone: LongInteger;
    static mut pdffile: *mut FILE;
    static mut outputfilename: StrNumber;
    static mut fixedpdfdraftmode: Integer;
    fn pdfflush();
    fn zpdfosgetosbuf(_: Integer);

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

    fn makecstring(_: StrNumber) -> *mut c_char;
    fn xfclose(_: *mut FILE, _: *const c_char);
    fn makestring() -> StrNumber;
    fn getnullstr() -> StrNumber;
    fn ztokenstostring(_: Integer) -> StrNumber;
    fn initstarttime();
    fn close_file_or_pipe(_: *mut FILE);

    fn fm_free();
    fn t1_free();
    fn enc_free();
    fn img_free();
    fn vf_free();
    fn epdf_free();
    fn ttf_free();
    fn sfd_free();
    fn glyph_unicode_free();
    fn zip_free();

    fn avl_create(
        _: Option<AvlComparisonFunc>,
        _: *mut c_void,
        _: *mut LibAvlAllocator,
    ) -> *mut AvlTable;
    fn avl_probe(_: *mut AvlTable, _: *mut c_void) -> *mut *mut c_void;
    fn avl_find(_: *const AvlTable, _: *const c_void) -> *mut c_void;
    fn avl_t_init(_: *mut AvlTraverser, _: *mut AvlTable);
    fn avl_t_first(_: *mut AvlTraverser, _: *mut AvlTable) -> *mut c_void;
    fn avl_t_next(_: *mut AvlTraverser) -> *mut c_void;
    static mut avl_xallocator: LibAvlAllocator;
    fn comp_string_entry(_: *const c_void, _: *const c_void, _: *mut c_void) -> c_int;

    fn png_get_libpng_ver(_: *mut c_void) -> *const c_char;
    fn zlibVersion() -> *const c_char;
    fn crc32(_: c_ulong, _: *const c_uchar, _: c_uint) -> c_ulong;
}

type RecorderFn = Option<unsafe extern "C" fn(*const c_char)>;

#[repr(C)]
struct HashTable {
    buckets: *mut *mut c_void,
    size: c_uint,
}

#[repr(C)]
struct StrList {
    length: c_uint,
    list: *mut *mut c_char,
}

#[repr(C)]
struct KpathseaInstancePrefix {
    record_input: RecorderFn,
    record_output: RecorderFn,
    cnf_hash: HashTable,
    doing_cnf_init: c_int,
    db: HashTable,
    alias_db: HashTable,
    db_dir_list: StrList,
    debug: c_uint,
    link_table: HashTable,
    the_cache: *mut c_void,
    cache_length: c_uint,
    map: HashTable,
    map_path: *const c_char,
    debug_hash_lookup_int: c_int,
    elt: *mut c_char,
    elt_alloc: c_uint,
    path: *const c_char,
    followup_search: c_int,
    log_file: *mut FILE,
    log_opened: c_int,
    invocation_name: *mut c_char,
}

#[repr(C)]
pub struct LibAvlAllocator {
    pub libavl_malloc: Option<unsafe extern "C" fn(*mut LibAvlAllocator, SizeT) -> *mut c_void>,
    pub libavl_free: Option<unsafe extern "C" fn(*mut LibAvlAllocator, *mut c_void)>,
}

#[repr(C)]
pub struct AvlTable {
    pub avl_root: *mut AvlNode,
    pub avl_compare: Option<AvlComparisonFunc>,
    pub avl_param: *mut c_void,
    pub avl_alloc: *mut LibAvlAllocator,
    pub avl_count: SizeT,
    pub avl_generation: c_ulong,
}

#[repr(C)]
pub struct AvlNode {
    pub avl_link: [*mut AvlNode; 2],
    pub avl_data: *mut c_void,
    pub avl_balance: i8,
}

#[repr(C)]
pub struct AvlTraverser {
    pub avl_table: *mut AvlTable,
    pub avl_node: *mut AvlNode,
    pub avl_stack: [*mut AvlNode; 32],
    pub avl_height: SizeT,
    pub avl_generation: c_ulong,
}

#[repr(C)]
pub struct IntParm {
    pub val: Integer,
    pub set: Boolean,
}

#[repr(C)]
pub struct FeEntry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FmEntry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FdEntry {
    pub fd_objnum: Integer,
    pub fontname: *mut c_char,
    pub subset_tag: *mut c_char,
    pub ff_found: Boolean,
    pub ff_objnum: Integer,
    pub fn_objnum: Integer,
    pub all_glyphs: Boolean,
    pub write_ttf_glyph_names: Boolean,
    pub font_dim: [IntParm; 11],
    pub fe: *mut FeEntry,
    pub builtin_glyph_names: *mut *mut c_char,
    pub fm: *mut FmEntry,
    pub tx_tree: *mut AvlTable,
    pub gl_tree: *mut AvlTable,
}

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
static mut SUB_MATCH_COUNT: c_int = 10;
static mut PMATCH: Vec<libc::regmatch_t> = Vec::new();
static mut MATCH_STRING: Option<Vec<u8>> = None;
static mut LAST_MATCH_SUCCEEDED: Boolean = FALSE;

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
    let ptr = unsafe { libc::malloc(size) };
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
    unsafe { libc::strlen(ptr) as usize }
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

#[no_mangle]
pub unsafe extern "C" fn pdf_puts(s: *const c_char) {
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

#[no_mangle]
pub unsafe extern "C" fn pdf_newline() {
    unsafe {
        if pdflastbyte != b'\n' {
            pdf_puts(c"\n".as_ptr());
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn maketexstring(s: *const c_char) -> StrNumber {
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

fn format_printf(fmt: *const c_char, args: &[PrintfArg]) -> Vec<u8> {
    let fmt = unsafe { c_string_bytes(fmt) };
    let mut out = Vec::with_capacity(PRINTF_BUF_SIZE);
    let mut arg_index = 0usize;
    let mut i = 0usize;
    while i < fmt.len() {
        if fmt[i] != b'%' {
            out.push(fmt[i]);
            i += 1;
            continue;
        }
        if i + 1 < fmt.len() && fmt[i + 1] == b'%' {
            out.push(b'%');
            i += 2;
            continue;
        }
        let start = i;
        i += 1;
        while i < fmt.len() && matches!(fmt[i], b'#' | b'0' | b'-' | b'+' | b' ' | b'\'') {
            i += 1;
        }
        while i < fmt.len() && fmt[i].is_ascii_digit() {
            i += 1;
        }
        if i < fmt.len() && fmt[i] == b'.' {
            i += 1;
            while i < fmt.len() && fmt[i].is_ascii_digit() {
                i += 1;
            }
        }
        if i + 1 < fmt.len()
            && ((fmt[i] == b'h' && fmt[i + 1] == b'h') || (fmt[i] == b'l' && fmt[i + 1] == b'l'))
        {
            i += 2;
        } else if i < fmt.len() && matches!(fmt[i], b'h' | b'l' | b'j' | b'z' | b't' | b'L') {
            i += 1;
        }
        if i >= fmt.len() {
            out.extend_from_slice(&fmt[start..]);
            break;
        }
        let conversion = fmt[i];
        let spec = &fmt[start..=i];
        i += 1;
        let Some(arg) = args.get(arg_index).copied() else {
            out.extend_from_slice(spec);
            continue;
        };
        arg_index += 1;
        out.extend_from_slice(&format_one(spec, conversion, arg));
    }
    out
}

fn format_one(spec: &[u8], conversion: u8, arg: PrintfArg) -> Vec<u8> {
    let mut c_spec = Vec::with_capacity(spec.len() + 1);
    c_spec.extend_from_slice(spec);
    c_spec.push(0);
    let long_spec = spec
        .iter()
        .any(|&b| b == b'l' || b == b'z' || b == b't' || b == b'j');
    let long_long_spec = spec.windows(2).any(|w| w == b"ll");
    unsafe {
        match conversion {
            b'd' | b'i' => {
                let value = arg.as_i64();
                if long_long_spec {
                    snprintf_one(c_spec.as_ptr(), |buf, len, fmt| {
                        libc::snprintf(buf, len, fmt, value as c_longlong)
                    })
                } else if long_spec {
                    snprintf_one(c_spec.as_ptr(), |buf, len, fmt| {
                        libc::snprintf(buf, len, fmt, value as c_long)
                    })
                } else {
                    snprintf_one(c_spec.as_ptr(), |buf, len, fmt| {
                        libc::snprintf(buf, len, fmt, value as c_int)
                    })
                }
            }
            b'u' | b'o' | b'x' | b'X' => {
                let value = arg.as_u64();
                if long_long_spec {
                    snprintf_one(c_spec.as_ptr(), |buf, len, fmt| {
                        libc::snprintf(buf, len, fmt, value as c_ulonglong)
                    })
                } else if long_spec {
                    snprintf_one(c_spec.as_ptr(), |buf, len, fmt| {
                        libc::snprintf(buf, len, fmt, value as c_ulong)
                    })
                } else {
                    snprintf_one(c_spec.as_ptr(), |buf, len, fmt| {
                        libc::snprintf(buf, len, fmt, value as c_uint)
                    })
                }
            }
            b'c' => snprintf_one(c_spec.as_ptr(), |buf, len, fmt| {
                libc::snprintf(buf, len, fmt, arg.as_i64() as c_int)
            }),
            b's' => snprintf_one(c_spec.as_ptr(), |buf, len, fmt| {
                libc::snprintf(buf, len, fmt, arg.as_ptr() as *const c_char)
            }),
            b'p' => snprintf_one(c_spec.as_ptr(), |buf, len, fmt| {
                libc::snprintf(buf, len, fmt, arg.as_ptr())
            }),
            b'f' | b'F' | b'e' | b'E' | b'g' | b'G' | b'a' | b'A' => {
                snprintf_one(c_spec.as_ptr(), |buf, len, fmt| {
                    libc::snprintf(buf, len, fmt, arg.as_f64() as c_double)
                })
            }
            _ => spec.to_vec(),
        }
    }
}

unsafe fn snprintf_one(
    fmt: *const u8,
    mut call: impl FnMut(*mut c_char, usize, *const c_char) -> c_int,
) -> Vec<u8> {
    let fmt = fmt as *const c_char;
    let mut buf = vec![0u8; PRINTF_BUF_SIZE];
    loop {
        let written = call(buf.as_mut_ptr() as *mut c_char, buf.len(), fmt);
        if written < 0 {
            return Vec::new();
        }
        let written = written as usize;
        if written < buf.len() {
            buf.truncate(written);
            return buf;
        }
        buf.resize(written + 1, 0);
    }
}

impl PrintfArg {
    fn as_i64(self) -> i64 {
        match self {
            Self::Signed(value) => value,
            Self::Unsigned(value) => value as i64,
            Self::Float(value) => value as i64,
            Self::Ptr(value) => value as isize as i64,
        }
    }

    fn as_u64(self) -> u64 {
        match self {
            Self::Signed(value) => value as u64,
            Self::Unsigned(value) => value,
            Self::Float(value) => value as u64,
            Self::Ptr(value) => value as usize as u64,
        }
    }

    fn as_f64(self) -> f64 {
        match self {
            Self::Signed(value) => value as f64,
            Self::Unsigned(value) => value as f64,
            Self::Float(value) => value,
            Self::Ptr(value) => value as usize as f64,
        }
    }

    fn as_ptr(self) -> *const c_void {
        match self {
            Self::Ptr(value) => value,
            Self::Signed(value) => value as isize as *const c_void,
            Self::Unsigned(value) => value as usize as *const c_void,
            Self::Float(_) => ptr::null(),
        }
    }
}

pub unsafe fn pdf_printf_args(fmt: *const c_char, args: &[PrintfArg]) {
    let bytes = format_printf(fmt, args);
    let mut c_bytes = Vec::with_capacity(bytes.len() + 1);
    c_bytes.extend_from_slice(&bytes);
    c_bytes.push(0);
    unsafe {
        pdf_puts(c_bytes.as_ptr() as *const c_char);
    }
}

pub unsafe fn tex_printf_args(fmt: *const c_char, args: &[PrintfArg]) {
    let bytes = format_printf(fmt, args);
    unsafe {
        safe_print_bytes(&bytes);
        xfflush(__stdoutp);
    }
}

pub unsafe fn pdftex_warn_args(fmt: *const c_char, args: &[PrintfArg]) {
    let bytes = format_printf(fmt, args);
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
    let bytes = format_printf(fmt, args);
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
            safe_print(c"kpathsea_debug enabled, calling abort()...".as_ptr());
            println();
            abort();
        } else {
            exit(EXIT_FAILURE);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn removepdffile() {
    unsafe {
        if !kpathsea_debug_enabled() && outputfilename != 0 && fixedpdfdraftmode == 0 {
            let name = makecstring(outputfilename);
            xfclose(pdffile, name);
            libc::remove(name);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn garbagewarning() {
    unsafe {
        pdftex_warn_args(
            c"dangling objects discarded, no output file produced.".as_ptr(),
            &[],
        );
        removepdffile();
    }
}

#[no_mangle]
pub unsafe extern "C" fn xfwrite(
    ptr: *mut c_void,
    size: SizeT,
    nmemb: SizeT,
    stream: *mut FILE,
) -> SizeT {
    unsafe {
        if libc::fwrite(ptr, size, nmemb, stream) != nmemb {
            pdftex_fail_args(c"fwrite() failed".as_ptr(), &[]);
        }
    }
    nmemb
}

#[no_mangle]
pub unsafe extern "C" fn xfflush(stream: *mut FILE) -> c_int {
    unsafe {
        if libc::fflush(stream) != 0 {
            let err = *libc::__error();
            pdftex_fail_args(
                c"fflush() failed (%s)".as_ptr(),
                &[PrintfArg::from(libc::strerror(err))],
            );
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn xgetc(stream: *mut FILE) -> c_int {
    #[cfg(unix)]
    unsafe {
        getc_unlocked(stream)
    }

    #[cfg(not(unix))]
    unsafe {
        libc::fgetc(stream)
    }
}

#[no_mangle]
pub unsafe extern "C" fn writestreamlength(length: LongInteger, offset: LongInteger) {
    unsafe {
        if JOBNAME_CSTR.is_null() {
            JOBNAME_CSTR = c_strdup(makecstring(jobname));
        }
        if fixedpdfdraftmode == 0 {
            if libc::fseeko(pdffile, offset, libc::SEEK_SET) != 0 {
                pdftex_fail_args(c"fseeko() failed".as_ptr(), &[]);
            }
            libc::fprintf(pdffile, c"%lli".as_ptr(), length as c_longlong);
            if libc::fseeko(pdffile, pdfgone + pdfptr as LongInteger, libc::SEEK_SET) != 0 {
                pdftex_fail_args(c"fseeko() failed".as_ptr(), &[]);
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn convertStringToPDFString(input: *const c_char, len: c_int) -> *mut c_char {
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

#[no_mangle]
pub unsafe extern "C" fn escapestring(input: PoolPointer) {
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

#[no_mangle]
pub unsafe extern "C" fn escapename(input: PoolPointer) {
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

#[no_mangle]
pub unsafe extern "C" fn escapehex(input: PoolPointer) {
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

#[no_mangle]
pub unsafe extern "C" fn unescapehex(input: PoolPointer) {
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

#[no_mangle]
pub unsafe extern "C" fn printID(filename: StrNumber) {
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

#[no_mangle]
pub unsafe extern "C" fn printIDalt(toks: Integer) {
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

#[no_mangle]
pub unsafe extern "C" fn printcreationdate() {
    unsafe {
        initstarttime();
        pdf_printf_args(
            c"/CreationDate (%s)\n".as_ptr(),
            &[PrintfArg::from(&raw mut start_time_str as *mut c_char)],
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn printmoddate() {
    unsafe {
        initstarttime();
        pdf_printf_args(
            c"/ModDate (%s)\n".as_ptr(),
            &[PrintfArg::from(&raw mut start_time_str as *mut c_char)],
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn setjobid(year: c_int, month: c_int, day: c_int, time: c_int) {
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

#[no_mangle]
pub unsafe extern "C" fn makepdftexbanner() {
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

#[no_mangle]
pub unsafe extern "C" fn getresnameprefix() -> StrNumber {
    static NAME_BYTES: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    unsafe {
        let id = c_string_bytes(JOB_ID_STRING);
        let mut crc = crc32(0, ptr::null(), 0);
        crc = crc32(crc, id.as_ptr(), id.len() as c_uint);
        let mut prefix = [0u8; 6];
        let base = NAME_BYTES.len() as c_ulong;
        for byte in &mut prefix {
            *byte = NAME_BYTES[(crc % base) as usize];
            crc /= base;
        }
        let c = CString::new(prefix.as_slice()).expect("prefix has no nul");
        maketexstring(c.as_ptr())
    }
}

#[no_mangle]
pub unsafe extern "C" fn initversionstring(versions: *mut *mut c_char) {
    unsafe {
        let text = format!(
            "Compiled with libpng 1.6.58; using libpng {}\nCompiled with zlib 1.3.2; using zlib {}\nCompiled with xpdf version 4.06\n",
            String::from_utf8_lossy(c_string_bytes(png_get_libpng_ver(ptr::null_mut()))),
            String::from_utf8_lossy(c_string_bytes(zlibVersion())),
        );
        *versions = malloced_c_string(text.as_bytes());
    }
}

#[no_mangle]
pub unsafe extern "C" fn extxnoverd(x: Scaled, n: Scaled, d: Scaled) -> Scaled {
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

#[no_mangle]
pub unsafe extern "C" fn matchstrings(s: StrNumber, t: StrNumber, subcount: c_int, icase: Boolean) {
    unsafe {
        if !check_pool_room(10) {
            return;
        }
        let pattern = makecstring(s);
        let mut preg = mem::MaybeUninit::<libc::regex_t>::zeroed().assume_init();
        let mut cflags = libc::REG_EXTENDED;
        if icase != 0 {
            cflags |= libc::REG_ICASE;
        }
        let ret = libc::regcomp(&mut preg, pattern, cflags);
        if ret != 0 {
            let size = libc::regerror(ret, &preg, ptr::null_mut(), 0);
            let mut buf = vec![0 as c_char; size];
            libc::regerror(ret, &preg, buf.as_mut_ptr(), size);
            pdftex_warn_args(
                c"%s%s".as_ptr(),
                &[
                    PrintfArg::from(c"\\pdfmatch: ".as_ptr()),
                    PrintfArg::from(buf.as_ptr()),
                ],
            );
            *strpool.add(poolptr as usize) = b'-';
            poolptr += 1;
            *strpool.add(poolptr as usize) = b'1';
            poolptr += 1;
        } else {
            let text = makecstring(t);
            SUB_MATCH_COUNT = if subcount < 0 { 10 } else { subcount };
            PMATCH.clear();
            if SUB_MATCH_COUNT > 0 {
                PMATCH.resize(
                    SUB_MATCH_COUNT as usize,
                    libc::regmatch_t { rm_so: 0, rm_eo: 0 },
                );
            }
            let exec_ret = libc::regexec(
                &preg,
                text,
                SUB_MATCH_COUNT as usize,
                if PMATCH.is_empty() {
                    ptr::null_mut()
                } else {
                    PMATCH.as_mut_ptr()
                },
                0,
            );
            MATCH_STRING = Some(c_string_bytes(text).to_vec());
            LAST_MATCH_SUCCEEDED = (exec_ret == 0) as Boolean;
            *strpool.add(poolptr as usize) = if exec_ret == 0 { b'1' } else { b'0' };
            poolptr += 1;
        }
        libc::regfree(&mut preg);
    }
}

#[no_mangle]
pub unsafe extern "C" fn getmatch(i: c_int) {
    unsafe {
        let found = i >= 0
            && i < SUB_MATCH_COUNT
            && MATCH_STRING.is_some()
            && LAST_MATCH_SUCCEEDED != 0
            && !PMATCH.is_empty()
            && PMATCH[i as usize].rm_so >= 0
            && PMATCH[i as usize].rm_eo >= PMATCH[i as usize].rm_so;
        if found {
            let m = PMATCH[i as usize];
            let start = m.rm_so as usize;
            let end = m.rm_eo as usize;
            let text = MATCH_STRING.as_ref().expect("checked above");
            let prefix = format!("{}->", start);
            if !check_pool_room(prefix.len() + end.saturating_sub(start)) {
                return;
            }
            ptr::copy_nonoverlapping(prefix.as_ptr(), strpool.add(poolptr as usize), prefix.len());
            poolptr += prefix.len() as PoolPointer;
            if end <= text.len() && start <= end {
                ptr::copy_nonoverlapping(
                    text[start..end].as_ptr(),
                    strpool.add(poolptr as usize),
                    end - start,
                );
                poolptr += (end - start) as PoolPointer;
            }
        } else {
            if !check_pool_room(4) {
                return;
            }
            for &byte in b"-1->" {
                *strpool.add(poolptr as usize) = byte;
                poolptr += 1;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn fb_offset() -> Integer {
    unsafe { fb_ptr.offset_from(fb_array) as Integer }
}

#[no_mangle]
pub unsafe extern "C" fn fb_seek(offset: Integer) {
    unsafe {
        fb_ptr = fb_array.add(offset as usize);
    }
}

unsafe fn ensure_fb_room(n: usize) {
    unsafe {
        if fb_array.is_null() {
            fb_limit = SMALL_ARRAY_SIZE.max(n);
            fb_array = xmalloc_bytes(fb_limit) as *mut c_char;
            fb_ptr = fb_array;
        } else if fb_ptr.offset_from(fb_array) as usize + n > fb_limit {
            last_ptr_index = fb_ptr.offset_from(fb_array) as usize;
            fb_limit = (fb_limit * 2).max(last_ptr_index + n);
            fb_array = libc::realloc(fb_array as *mut c_void, fb_limit) as *mut c_char;
            if fb_array.is_null() {
                pdftex_fail_args(c"fb_array exceeds size limit".as_ptr(), &[]);
            }
            fb_ptr = fb_array.add(last_ptr_index);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn fb_putchar(b: c_uchar) {
    unsafe {
        ensure_fb_room(1);
        *fb_ptr = b as c_char;
        fb_ptr = fb_ptr.add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn fb_flush() {
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

unsafe fn grow_i32_array(
    array: &mut *mut Integer,
    ptr_value: &mut *mut Integer,
    limit: &mut SizeT,
    n: usize,
    initial: usize,
) {
    unsafe {
        if (*array).is_null() {
            *limit = initial.max(n);
            *array = xmalloc_bytes(*limit * mem::size_of::<Integer>()) as *mut Integer;
            *ptr_value = *array;
        } else if (*ptr_value).offset_from(*array) as usize + n > *limit {
            last_ptr_index = (*ptr_value).offset_from(*array) as usize;
            *limit = (*limit * 2).max(last_ptr_index + n);
            *array = libc::realloc(*array as *mut c_void, *limit * mem::size_of::<Integer>())
                as *mut Integer;
            if (*array).is_null() {
                pdftex_fail_args(c"vf_e_fnts_array exceeds size limit".as_ptr(), &[]);
            }
            *ptr_value = (*array).add(last_ptr_index);
        }
    }
}

unsafe fn grow_internal_font_array(
    array: &mut *mut InternalFontNumber,
    ptr_value: &mut *mut InternalFontNumber,
    limit: &mut SizeT,
    n: usize,
    initial: usize,
) {
    unsafe {
        if (*array).is_null() {
            *limit = initial.max(n);
            *array = xmalloc_bytes(*limit * mem::size_of::<InternalFontNumber>())
                as *mut InternalFontNumber;
            *ptr_value = *array;
        } else if (*ptr_value).offset_from(*array) as usize + n > *limit {
            last_ptr_index = (*ptr_value).offset_from(*array) as usize;
            *limit = (*limit * 2).max(last_ptr_index + n);
            *array = libc::realloc(
                *array as *mut c_void,
                *limit * mem::size_of::<InternalFontNumber>(),
            ) as *mut InternalFontNumber;
            if (*array).is_null() {
                pdftex_fail_args(c"vf_i_fnts_array exceeds size limit".as_ptr(), &[]);
            }
            *ptr_value = (*array).add(last_ptr_index);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn allocvffnts() {
    unsafe {
        if VF_E_FNTS_ARRAY.is_null() {
            VF_E_FNTS_ARRAY = vfefnts;
            VF_E_FNTS_LIMIT = fontmax as SizeT;
            VF_E_FNTS_PTR = VF_E_FNTS_ARRAY;
            VF_I_FNTS_ARRAY = vfifnts;
            VF_I_FNTS_LIMIT = fontmax as SizeT;
            VF_I_FNTS_PTR = VF_I_FNTS_ARRAY;
        }
        grow_i32_array(
            &mut VF_E_FNTS_ARRAY,
            &mut VF_E_FNTS_PTR,
            &mut VF_E_FNTS_LIMIT,
            1,
            fontmax as usize,
        );
        VF_E_FNTS_PTR = VF_E_FNTS_PTR.add(1);
        grow_internal_font_array(
            &mut VF_I_FNTS_ARRAY,
            &mut VF_I_FNTS_PTR,
            &mut VF_I_FNTS_LIMIT,
            1,
            fontmax as usize,
        );
        VF_I_FNTS_PTR = VF_I_FNTS_PTR.add(1);
        if VF_E_FNTS_ARRAY != vfefnts {
            vfefnts = VF_E_FNTS_ARRAY;
            vfifnts = VF_I_FNTS_ARRAY;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn libpdffinish() {
    unsafe {
        libc::free(fb_array as *mut c_void);
        fb_array = ptr::null_mut();
        fb_ptr = ptr::null_mut();
        libc::free(JOB_ID_STRING as *mut c_void);
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

#[no_mangle]
pub unsafe extern "C" fn make_subset_tag(fd: *mut FdEntry) {
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

#[no_mangle]
pub unsafe extern "C" fn colorstackused() -> c_int {
    unsafe { init_colorstacks().len() as c_int }
}

#[no_mangle]
pub unsafe extern "C" fn newcolorstack(
    s: Integer,
    literal_mode: Integer,
    page_start: Boolean,
) -> c_int {
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

#[no_mangle]
pub unsafe extern "C" fn colorstackset(colstack_no: c_int, s: Integer) -> Integer {
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

#[no_mangle]
pub unsafe extern "C" fn colorstackcurrent(colstack_no: c_int) -> Integer {
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

#[no_mangle]
pub unsafe extern "C" fn colorstackpush(colstack_no: c_int, s: Integer) -> Integer {
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

#[no_mangle]
pub unsafe extern "C" fn colorstackpop(colstack_no: c_int) -> Integer {
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

#[no_mangle]
pub unsafe extern "C" fn colorstackskippagestart(colstack_no: c_int) -> Integer {
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

#[no_mangle]
pub unsafe extern "C" fn matrixused() -> Boolean {
    unsafe { (!MATRIX_STACK.is_empty()) as Boolean }
}

#[no_mangle]
pub unsafe extern "C" fn checkpdfsave(cur_h: c_int, cur_v: c_int) {
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

#[no_mangle]
pub unsafe extern "C" fn checkpdfrestore(cur_h: c_int, cur_v: c_int) {
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

#[no_mangle]
pub unsafe extern "C" fn pdfshipoutbegin(shipping_page: Boolean) {
    unsafe {
        POS_STACK.clear();
        PAGE_MODE = shipping_page;
        if shipping_page != 0 {
            colorstackpagestart();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn pdfshipoutend(shipping_page: Boolean) {
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

#[no_mangle]
pub unsafe extern "C" fn pdfsetmatrix(input: PoolPointer, cur_h: Scaled, cur_v: Scaled) -> Integer {
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

#[no_mangle]
pub unsafe extern "C" fn matrixtransformrect(llx: Scaled, lly: Scaled, urx: Scaled, ury: Scaled) {
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

#[no_mangle]
pub unsafe extern "C" fn matrixtransformpoint(x: Scaled, y: Scaled) {
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

#[no_mangle]
pub unsafe extern "C" fn matrixrecalculate(urx: Scaled) {
    unsafe {
        matrixtransformrect(LAST_LLX, LAST_LLY, urx, LAST_URY);
    }
}

#[no_mangle]
pub unsafe extern "C" fn getllx() -> Scaled {
    unsafe { RET_LLX }
}

#[no_mangle]
pub unsafe extern "C" fn getlly() -> Scaled {
    unsafe { RET_LLY }
}

#[no_mangle]
pub unsafe extern "C" fn geturx() -> Scaled {
    unsafe { RET_URX }
}

#[no_mangle]
pub unsafe extern "C" fn getury() -> Scaled {
    unsafe { RET_URY }
}

#[no_mangle]
pub unsafe extern "C" fn stripzeros(a: *mut c_char) -> *mut c_char {
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
    use super::{format_printf, PrintfArg};

    #[test]
    fn printf_helper_formats_mixed_pdf_fragments() {
        let out = format_printf(
            c"/Name /%s %04X %ld %.2x %%".as_ptr(),
            &[
                PrintfArg::from(c"Font".as_ptr()),
                PrintfArg::from(15),
                PrintfArg::from(123_i64),
                PrintfArg::from(10),
            ],
        );
        assert_eq!(String::from_utf8(out).unwrap(), "/Name /Font 000F 123 0a %");
    }

    #[test]
    fn printf_helper_formats_pdf_real_values() {
        let out = format_printf(
            c"%.1f %.6g".as_ptr(),
            &[PrintfArg::from(1.75), PrintfArg::from(0.5)],
        );
        assert_eq!(String::from_utf8(out).unwrap(), "1.8 0.5");
    }
}
