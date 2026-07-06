//! Minimal Rust-owned kpathsea ABI.
//!
//! This keeps pdfTeX's generated web2c boundary satisfied without linking the
//! native kpathsea archive. File lookup starts with explicit/local paths and
//! then uses TeX Live `ls-R` databases, which is the part of kpathsea that
//! matters most for fast package/font discovery.

use crate::generated::pdftexextra::{
    cache_entry, const_string, expansion_type, hash_table_type, kpathsea, kpathsea_instance,
    kpse_file_format_type, kpse_format_info_type, kpse_src_type, p_record_input, p_record_output,
    str_list_type, string, FILE,
};
use libc::{c_char, c_double, c_int, c_uint, c_void, size_t};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;
use std::sync::{Mutex, OnceLock};

const TRUE: c_int = 1;
const FALSE: c_int = 0;
const KPSE_GF_FORMAT: c_uint = 0;
const KPSE_PK_FORMAT: c_uint = 1;
const KPSE_ANY_GLYPH_FORMAT: c_uint = 2;
const KPSE_TFM_FORMAT: c_uint = 3;
const KPSE_BIB_FORMAT: c_uint = 6;
const KPSE_BST_FORMAT: c_uint = 7;
const KPSE_FMT_FORMAT: c_uint = 10;
const KPSE_FONTMAP_FORMAT: c_uint = 11;
const KPSE_TEX_FORMAT: c_uint = 26;
const KPSE_TEXPOOL_FORMAT: c_uint = 28;
const KPSE_TYPE1_FORMAT: c_uint = 32;
const KPSE_VF_FORMAT: c_uint = 33;
const KPSE_TRUETYPE_FORMAT: c_uint = 36;
const KPSE_WEB2C_FORMAT: c_uint = 38;
const KPSE_OPENTYPE_FORMAT: c_uint = 47;
const KPSE_PDFTEX_CONFIG_FORMAT: c_uint = 48;
const KPSE_GLYPH_SOURCE_NORMAL: c_uint = 0;

const ZERO_HASH: hash_table_type = hash_table_type {
    buckets: ptr::null_mut(),
    size: 0,
};
const ZERO_STR_LIST: str_list_type = str_list_type {
    length: 0,
    list: ptr::null_mut(),
};
const ZERO_FORMAT_INFO: kpse_format_info_type = kpse_format_info_type {
    type_0: ptr::null(),
    path: ptr::null_mut(),
    raw_path: ptr::null(),
    path_source: ptr::null(),
    override_path: ptr::null(),
    client_path: ptr::null(),
    cnf_path: ptr::null(),
    default_path: ptr::null(),
    suffix: ptr::null_mut(),
    alt_suffix: ptr::null_mut(),
    suffix_search_only: 0,
    program: ptr::null(),
    argc: 0,
    argv: ptr::null_mut(),
    program_enabled_p: 0,
    program_enable_level: 0,
    binmode: 0,
};

#[no_mangle]
pub static mut kpathsea_version_string: *const c_char =
    b"kpathsea version 6.4.3/dev\0" as *const u8 as *const c_char;

#[no_mangle]
pub static mut kpse_def_inst: kpathsea_instance = kpathsea_instance {
    record_input: None as p_record_input,
    record_output: None as p_record_output,
    cnf_hash: ZERO_HASH,
    doing_cnf_init: 0,
    db: ZERO_HASH,
    alias_db: ZERO_HASH,
    db_dir_list: ZERO_STR_LIST,
    debug: 0,
    link_table: ZERO_HASH,
    the_cache: ptr::null_mut::<cache_entry>(),
    cache_length: 0,
    map: ZERO_HASH,
    map_path: ptr::null(),
    debug_hash_lookup_int: 0,
    elt: ptr::null_mut(),
    elt_alloc: 0,
    path: ptr::null(),
    followup_search: 0,
    log_file: ptr::null_mut::<FILE>(),
    log_opened: 0,
    invocation_name: ptr::null_mut(),
    invocation_short_name: ptr::null_mut(),
    program_name: ptr::null_mut(),
    ll_verbose: 0,
    fallback_font: ptr::null(),
    fallback_resolutions_string: ptr::null(),
    fallback_resolutions: ptr::null_mut(),
    format_info: [ZERO_FORMAT_INFO; 59],
    make_tex_discard_errors: 0,
    missfont: ptr::null_mut::<FILE>(),
    expansions: ptr::null_mut::<expansion_type>(),
    expansion_len: 0,
    saved_env: ptr::null_mut(),
    saved_count: 0,
};

#[no_mangle]
pub static mut kpse_def: kpathsea = ptr::null_mut();

#[repr(C)]
pub struct KpseGlyphFileType {
    name: const_string,
    dpi: c_uint,
    format: kpse_file_format_type,
    source: c_uint,
}

extern "C" {
    fn xmalloc(size: size_t) -> *mut c_void;
    fn xstrdup(s: const_string) -> string;
}

static CNF_OVERRIDES: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
static FILE_INDEX: OnceLock<FileIndex> = OnceLock::new();
static EXPLICIT_SEARCH_DIRS: OnceLock<Vec<PathBuf>> = OnceLock::new();
static FORMAT_SEARCH_DIRS: OnceLock<Vec<PathBuf>> = OnceLock::new();

thread_local! {
    static INDEX_LOOKUP_CACHE: RefCell<HashMap<c_uint, HashMap<String, Option<PathBuf>>>> =
        RefCell::new(HashMap::new());
    static INDEX_READABLE_CACHE: RefCell<HashMap<PathBuf, bool>> = RefCell::new(HashMap::new());
}

struct FileIndex {
    by_name: HashMap<String, Vec<PathBuf>>,
}

fn overrides() -> &'static Mutex<HashMap<String, String>> {
    CNF_OVERRIDES.get_or_init(|| Mutex::new(HashMap::new()))
}

unsafe fn ensure_instance() {
    unsafe {
        if kpse_def.is_null() {
            kpse_def = &raw mut kpse_def_inst;
        }
    }
}

fn c_str_to_string(s: const_string) -> Option<String> {
    if s.is_null() {
        return None;
    }
    Some(unsafe { CStr::from_ptr(s) }.to_string_lossy().into_owned())
}

unsafe fn alloc_bytes(bytes: &[u8]) -> string {
    unsafe {
        let out = xmalloc(bytes.len() + 1) as *mut u8;
        ptr::copy_nonoverlapping(bytes.as_ptr(), out, bytes.len());
        *out.add(bytes.len()) = 0;
        out as string
    }
}

unsafe fn alloc_string(value: &str) -> string {
    unsafe { alloc_bytes(value.as_bytes()) }
}

fn basename(path: &str) -> &str {
    path.rsplit(['/', '\\']).next().unwrap_or(path)
}

fn format_suffixes(format: c_uint) -> &'static [&'static str] {
    match format {
        KPSE_GF_FORMAT => &[".gf"],
        KPSE_PK_FORMAT | KPSE_ANY_GLYPH_FORMAT => &[".pk", ".gf"],
        KPSE_TFM_FORMAT => &[".tfm"],
        KPSE_BIB_FORMAT => &[".bib"],
        KPSE_BST_FORMAT => &[".bst"],
        KPSE_FMT_FORMAT => &[".fmt"],
        KPSE_FONTMAP_FORMAT => &[".map"],
        KPSE_TEX_FORMAT => &[".tex", ".sty", ".cls", ".ltx", ".def", ".fd", ".cfg"],
        KPSE_TEXPOOL_FORMAT => &[".pool"],
        KPSE_TYPE1_FORMAT => &[".pfb", ".pfa"],
        KPSE_VF_FORMAT => &[".vf"],
        KPSE_TRUETYPE_FORMAT => &[".ttf", ".ttc"],
        KPSE_OPENTYPE_FORMAT => &[".otf"],
        KPSE_PDFTEX_CONFIG_FORMAT => &[".cfg", ".map"],
        _ => &[],
    }
}

fn with_candidate_names<T>(
    name: &str,
    format: c_uint,
    mut visit: impl FnMut(&str) -> Option<T>,
) -> Option<T> {
    if let Some(found) = visit(name) {
        return Some(found);
    }
    if Path::new(name).extension().is_some() {
        return None;
    }

    let suffixes = format_suffixes(format);
    let max_suffix_len = suffixes
        .iter()
        .map(|suffix| suffix.len())
        .max()
        .unwrap_or(0);
    let mut candidate = String::with_capacity(name.len() + max_suffix_len);
    for suffix in suffixes {
        candidate.clear();
        candidate.push_str(name);
        candidate.push_str(suffix);
        if let Some(found) = visit(&candidate) {
            return Some(found);
        }
    }
    None
}

fn path_is_readable(path: &Path) -> bool {
    std::fs::metadata(path)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

fn indexed_path_is_readable(path: &Path) -> bool {
    INDEX_READABLE_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(readable) = cache.get(path) {
            return *readable;
        }
        let readable = path_is_readable(path);
        cache.insert(path.to_path_buf(), readable);
        readable
    })
}

fn check_direct_path(path: &Path) -> Option<PathBuf> {
    if path_is_readable(path) {
        return Some(path.to_path_buf());
    }
    None
}

fn explicit_search_dirs() -> &'static [PathBuf] {
    EXPLICIT_SEARCH_DIRS
        .get_or_init(build_explicit_search_dirs)
        .as_slice()
}

fn build_explicit_search_dirs() -> Vec<PathBuf> {
    let mut dirs = vec![PathBuf::from(".")];
    if let Some(texinputs) = std::env::var_os("TEXINPUTS") {
        for item in std::env::split_paths(&texinputs) {
            if item.as_os_str().is_empty() {
                continue;
            }
            dirs.push(item);
        }
    }
    dirs
}

fn format_search_dirs() -> &'static [PathBuf] {
    FORMAT_SEARCH_DIRS
        .get_or_init(build_format_search_dirs)
        .as_slice()
}

fn build_format_search_dirs() -> Vec<PathBuf> {
    let mut dirs = vec![PathBuf::from(".")];
    for key in ["PDFTEX_RUST_FORMATS", "TEXFORMATS"] {
        if let Some(paths) = std::env::var_os(key) {
            for item in std::env::split_paths(&paths) {
                if item.as_os_str().is_empty() {
                    continue;
                }
                dirs.push(item);
            }
        }
    }
    dirs.push(PathBuf::from("formats"));
    dirs.push(PathBuf::from("target/pdftex-port/formats"));
    if let Ok(exe) = std::env::current_exe() {
        if let Some(bin_dir) = exe.parent() {
            dirs.push(bin_dir.join("../pdftex-port/formats"));
            dirs.push(bin_dir.join("../share/pdftex-rust/formats"));
        }
    }
    dirs
}

fn texlive_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    for key in ["TEXMFCONFIG", "TEXMFVAR", "TEXMFDIST"] {
        if let Some(value) = std::env::var_os(key) {
            roots.push(PathBuf::from(value));
        }
    }
    if roots.iter().any(|root| root.join("ls-R").exists()) {
        return roots;
    }
    if let Some(root) = latest_texlive_root() {
        roots.push(root.join("texmf-config"));
        roots.push(root.join("texmf-var"));
        roots.push(root.join("texmf-dist"));
    }
    roots
}

fn latest_texlive_root() -> Option<PathBuf> {
    let base = Path::new("/usr/local/texlive");
    let mut versions = std::fs::read_dir(base)
        .ok()?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.join("texmf-dist/ls-R").exists())
        .collect::<Vec<_>>();
    versions.sort();
    versions.pop()
}

fn file_index() -> &'static FileIndex {
    FILE_INDEX.get_or_init(|| {
        let mut by_name: HashMap<String, Vec<PathBuf>> = HashMap::new();
        for root in texlive_roots() {
            parse_ls_r(&root, &mut by_name);
        }
        FileIndex { by_name }
    })
}

fn parse_ls_r(root: &Path, by_name: &mut HashMap<String, Vec<PathBuf>>) {
    let Ok(text) = std::fs::read_to_string(root.join("ls-R")) else {
        return;
    };
    let mut current_dir = root.to_path_buf();
    let mut current_dir_is_runtime = true;
    for line in text.lines() {
        if line.is_empty() || line.starts_with('%') {
            continue;
        }
        if let Some(dir) = line.strip_suffix(':') {
            let dir = dir.strip_prefix("./").unwrap_or(dir);
            current_dir_is_runtime = is_runtime_ls_r_dir(dir);
            if current_dir_is_runtime {
                current_dir = root.join(dir);
            }
            continue;
        }
        if !current_dir_is_runtime {
            continue;
        }
        let path = current_dir.join(line);
        by_name.entry(line.to_owned()).or_default().push(path);
    }
}

fn is_runtime_ls_r_dir(dir: &str) -> bool {
    !dir.split('/').any(|part| part == "doc" || part == "source")
}

fn find_in_index(candidate: &str, format: c_uint) -> Option<PathBuf> {
    let cached = INDEX_LOOKUP_CACHE.with(|cache| {
        let cache = cache.borrow();
        cache
            .get(&format)
            .and_then(|by_candidate| by_candidate.get(candidate).cloned())
    });
    if let Some(found) = cached {
        return found;
    }

    let found = find_in_index_uncached(candidate, format);
    INDEX_LOOKUP_CACHE.with(|cache| {
        cache
            .borrow_mut()
            .entry(format)
            .or_default()
            .insert(candidate.to_owned(), found.clone());
    });
    found
}

fn find_in_index_uncached(candidate: &str, format: c_uint) -> Option<PathBuf> {
    let index = file_index();
    let key = basename(candidate);
    let matches = index.by_name.get(key)?;
    if candidate == key {
        return best_index_match(matches.iter(), format);
    }
    best_index_match(
        matches
            .iter()
            .filter(|path| path.to_string_lossy().ends_with(candidate)),
        format,
    )
}

fn best_index_match<'a>(
    matches: impl Iterator<Item = &'a PathBuf>,
    format: c_uint,
) -> Option<PathBuf> {
    matches
        .filter(|path| index_path_is_runtime(path, format))
        .filter(|path| indexed_path_is_readable(path))
        .min_by_key(|path| index_path_rank(path, format))
        .cloned()
}

fn index_path_is_runtime(path: &Path, format: c_uint) -> bool {
    if !matches!(
        format,
        KPSE_TEX_FORMAT
            | KPSE_TEXPOOL_FORMAT
            | KPSE_WEB2C_FORMAT
            | KPSE_BIB_FORMAT
            | KPSE_BST_FORMAT
            | KPSE_TFM_FORMAT
            | KPSE_GF_FORMAT
            | KPSE_PK_FORMAT
            | KPSE_ANY_GLYPH_FORMAT
            | KPSE_TYPE1_FORMAT
            | KPSE_VF_FORMAT
            | KPSE_TRUETYPE_FORMAT
            | KPSE_OPENTYPE_FORMAT
            | KPSE_FONTMAP_FORMAT
            | KPSE_PDFTEX_CONFIG_FORMAT
    ) {
        return true;
    }
    let path = path.to_string_lossy();
    !(path.contains("/doc/") || path.contains("/source/"))
}

fn index_path_rank(path: &Path, format: c_uint) -> (u8, u8, usize) {
    let path = path.to_string_lossy();
    let rank = if path.contains("/doc/") || path.contains("/source/") {
        200
    } else {
        match format {
            KPSE_TEX_FORMAT => {
                if path.contains("/tex/latex/") {
                    0
                } else if path.contains("/tex/generic/") {
                    1
                } else if path.contains("/tex/plain/") {
                    2
                } else if path.contains("/tex/") {
                    3
                } else {
                    80
                }
            }
            KPSE_TEXPOOL_FORMAT | KPSE_WEB2C_FORMAT => {
                if path.contains("/web2c/") {
                    0
                } else {
                    80
                }
            }
            KPSE_BIB_FORMAT | KPSE_BST_FORMAT => {
                if path.contains("/bibtex/") {
                    0
                } else {
                    80
                }
            }
            KPSE_TFM_FORMAT
            | KPSE_GF_FORMAT
            | KPSE_PK_FORMAT
            | KPSE_ANY_GLYPH_FORMAT
            | KPSE_TYPE1_FORMAT
            | KPSE_VF_FORMAT
            | KPSE_TRUETYPE_FORMAT
            | KPSE_OPENTYPE_FORMAT => {
                if path.contains("/fonts/") {
                    0
                } else {
                    80
                }
            }
            KPSE_FONTMAP_FORMAT | KPSE_PDFTEX_CONFIG_FORMAT => {
                if path.contains("/fonts/map/") {
                    0
                } else if path.contains("/web2c/") {
                    1
                } else if path.contains("/tex/") {
                    2
                } else {
                    80
                }
            }
            _ => 50,
        }
    };
    (rank, texlive_tree_rank(&path), path.len())
}

fn texlive_tree_rank(path: &str) -> u8 {
    if path.contains("/texmf-config/") {
        0
    } else if path.contains("/texmf-var/") {
        1
    } else if path.contains("/texmf-dist/") {
        2
    } else {
        3
    }
}

fn find_file_path(name: &str, format: c_uint) -> Option<PathBuf> {
    if let Some(found) = with_candidate_names(name, format, |candidate| {
        let path = Path::new(candidate);
        if path.is_absolute() || candidate.contains('/') {
            return check_direct_path(path);
        }
        None
    }) {
        return Some(found);
    }

    if format == KPSE_FMT_FORMAT {
        for dir in format_search_dirs() {
            if let Some(found) = with_candidate_names(name, format, |candidate| {
                check_direct_path(&dir.join(candidate))
            }) {
                return Some(found);
            }
        }
        return None;
    }

    for dir in explicit_search_dirs() {
        if let Some(found) = with_candidate_names(name, format, |candidate| {
            check_direct_path(&dir.join(candidate))
        }) {
            return Some(found);
        }
    }
    with_candidate_names(name, format, |candidate| find_in_index(candidate, format))
}

#[no_mangle]
pub unsafe extern "C" fn kpse_set_program_name(argv0: const_string, progname: const_string) {
    unsafe {
        ensure_instance();
        let invocation = c_str_to_string(argv0).unwrap_or_else(|| "pdftex-rust".to_owned());
        let short = basename(&invocation).to_owned();
        let program = c_str_to_string(progname).unwrap_or_else(|| short.clone());
        kpse_def_inst.invocation_name = alloc_string(&short);
        kpse_def_inst.invocation_short_name = alloc_string(&short);
        kpse_def_inst.program_name = alloc_string(&program);
    }
}

#[no_mangle]
pub unsafe extern "C" fn kpse_reset_program_name(progname: const_string) {
    unsafe {
        ensure_instance();
        if let Some(program) = c_str_to_string(progname) {
            kpse_def_inst.program_name = alloc_string(&program);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn kpse_init_prog(
    _: const_string,
    _: c_uint,
    _: const_string,
    _: const_string,
) {
    unsafe {
        ensure_instance();
    }
}

#[no_mangle]
pub unsafe extern "C" fn kpse_set_program_enabled(
    fmt: kpse_file_format_type,
    value: c_int,
    level: kpse_src_type,
) {
    unsafe {
        ensure_instance();
        if let Some(info) = kpse_def_inst.format_info.get_mut(fmt as usize) {
            info.program_enabled_p = value;
            info.program_enable_level = level;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn kpse_maketex_option(_: const_string, _: c_int) {}

#[no_mangle]
pub unsafe extern "C" fn kpathsea_cnf_line_env_progname(_: kpathsea, line: string) {
    if let Some(line) = c_str_to_string(line) {
        if let Some((key, value)) = line.split_once('=') {
            overrides()
                .lock()
                .unwrap()
                .insert(key.trim().to_owned(), value.trim().to_owned());
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn kpse_var_value(var: const_string) -> string {
    let Some(key) = c_str_to_string(var) else {
        return ptr::null_mut();
    };
    if let Ok(value) = std::env::var(&key) {
        return unsafe { alloc_string(&value) };
    }
    if let Some(value) = overrides().lock().unwrap().get(&key).cloned() {
        return unsafe { alloc_string(&value) };
    }
    let fallback = match key.as_str() {
        "shell_escape" => Some("f"),
        "shell_escape_commands" => Some(""),
        "file_line_error_style" => Some("f"),
        "parse_first_line" => Some("f"),
        "output_comment" => Some(""),
        "main_memory" => Some("5000000"),
        "extra_mem_top" => Some("0"),
        "extra_mem_bot" => Some("0"),
        "pool_size" => Some("6250000"),
        "string_vacancies" => Some("90000"),
        "pool_free" => Some("47500"),
        "max_strings" => Some("500000"),
        "strings_free" => Some("100"),
        "font_mem_size" => Some("8000000"),
        "font_max" => Some("9000"),
        "trie_size" => Some("1100000"),
        "hyph_size" => Some("8191"),
        "buf_size" => Some("200000"),
        "nest_size" => Some("1000"),
        "max_in_open" => Some("15"),
        "param_size" => Some("20000"),
        "save_size" => Some("200000"),
        "stack_size" => Some("10000"),
        "dvi_buf_size" => Some("16384"),
        "error_line" => Some("79"),
        "half_error_line" => Some("50"),
        "max_print_line" => Some("79"),
        "hash_extra" => Some("600000"),
        "expand_depth" => Some("10000"),
        "pk_dpi" => Some("72"),
        "TEXMFOUTPUT" => None,
        _ => None,
    };
    fallback.map_or(ptr::null_mut(), |value| unsafe { alloc_string(value) })
}

#[no_mangle]
pub unsafe extern "C" fn kpse_absolute_p(filename: const_string, relative_ok: c_int) -> c_int {
    if filename.is_null() {
        return FALSE;
    }
    let bytes = unsafe { CStr::from_ptr(filename) }.to_bytes();
    if bytes.starts_with(b"/") || bytes.starts_with(b"~") {
        return TRUE;
    }
    if relative_ok != 0 && (bytes.starts_with(b"./") || bytes.starts_with(b"../")) {
        return TRUE;
    }
    FALSE
}

#[no_mangle]
pub unsafe extern "C" fn kpse_in_name_ok(_: const_string) -> c_int {
    TRUE
}

#[no_mangle]
pub unsafe extern "C" fn kpse_out_name_ok(_: const_string) -> c_int {
    TRUE
}

#[no_mangle]
pub unsafe extern "C" fn kpse_readable_file(name: string) -> string {
    if name.is_null() {
        return ptr::null_mut();
    }
    let bytes = unsafe { CStr::from_ptr(name) }.to_bytes();
    if path_is_readable(Path::new(OsStr::from_bytes(bytes))) {
        return unsafe { xstrdup(name as const_string) };
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn kpse_find_file(
    name: const_string,
    format: kpse_file_format_type,
    _: c_int,
) -> string {
    let Some(name) = c_str_to_string(name) else {
        return ptr::null_mut();
    };
    match find_file_path(&name, format) {
        Some(path) => {
            let value = path.to_string_lossy();
            unsafe { alloc_string(&value) }
        }
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn kpse_find_glyph(
    font_name: const_string,
    dpi: c_uint,
    format: kpse_file_format_type,
    glyph_file: *mut KpseGlyphFileType,
) -> string {
    let Some(name) = c_str_to_string(font_name) else {
        return ptr::null_mut();
    };
    let candidates = if format == KPSE_GF_FORMAT {
        vec![format!("{name}.{dpi}gf"), format!("{name}.gf")]
    } else {
        vec![format!("{name}.{dpi}pk"), format!("{name}.pk")]
    };
    for candidate in candidates {
        if let Some(path) = find_file_path(&candidate, format) {
            let value = path.to_string_lossy().into_owned();
            let out = unsafe { alloc_string(&value) };
            if !glyph_file.is_null() {
                unsafe {
                    (*glyph_file).name = font_name;
                    (*glyph_file).dpi = dpi;
                    (*glyph_file).format = format;
                    (*glyph_file).source = KPSE_GLYPH_SOURCE_NORMAL;
                }
            }
            return out;
        }
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn kpse_bitmap_tolerance(actual: c_double, expected: c_double) -> c_int {
    ((actual - expected).abs() <= 1.0) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn kpse_magstep_fix(dpi: c_int, _: c_int, _: const_string) -> c_int {
    dpi
}
