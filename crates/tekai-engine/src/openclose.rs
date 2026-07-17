//! Rust-owned web2c file opening and configuration shims.

use libc::{c_char, c_int, c_uint, c_void, size_t, FILE};
use std::ffi::{CStr, CString};
use std::ptr;

const TRUE: c_int = 1;
const FALSE: c_int = 0;
const KPSE_TFM_FORMAT: c_int = 3;
const KPSE_OCP_FORMAT: c_int = 19;
const KPSE_OFM_FORMAT: c_int = 20;
const KPSE_TEX_FORMAT: c_int = 26;
const KPSE_VF_FORMAT: c_int = 33;

#[no_mangle]
pub static mut fullnameoffile: *mut c_char = ptr::null_mut();
#[no_mangle]
pub static mut output_directory: *mut c_char = ptr::null_mut();
#[no_mangle]
pub static mut recorder_enabled: c_int = 0;
#[no_mangle]
pub static mut tfmtemp: c_int = 0;
#[no_mangle]
pub static mut ocptemp: c_int = 0;
#[no_mangle]
pub static mut texinputtype: c_int = 0;

static mut RECORDER_NAME: *mut c_char = ptr::null_mut();
static mut RECORDER_FILE: *mut FILE = ptr::null_mut();

extern "C" {
    static mut __stderrp: *mut FILE;
    static mut nameoffile: *mut u8;
    static mut namelength: c_int;

    fn dir_p(name: *mut c_char) -> c_int;
    fn kpse_absolute_p(filename: *const c_char, relative_ok: c_int) -> c_int;
    fn kpse_find_file(name: *const c_char, format: c_uint, must_exist: c_int) -> *mut c_char;
    fn kpse_var_value(var: *const c_char) -> *mut c_char;
    fn xfopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    fn xgetcwd() -> *mut c_char;
    fn xdirname(name: *const c_char) -> *mut c_char;
}

#[cfg(unix)]
extern "C" {
    fn getc_unlocked(stream: *mut FILE) -> c_int;
}

unsafe fn fast_fgetc(stream: *mut FILE) -> c_int {
    #[cfg(unix)]
    {
        unsafe { getc_unlocked(stream) }
    }

    #[cfg(not(unix))]
    {
        unsafe { libc::fgetc(stream) }
    }
}

fn c_strlen(s: *const c_char) -> usize {
    unsafe { libc::strlen(s) as usize }
}

fn c_bytes(s: *const c_char) -> &'static [u8] {
    unsafe { CStr::from_ptr(s).to_bytes() }
}

unsafe fn alloc_copy(bytes: &[u8]) -> *mut c_char {
    let ptr = unsafe { xmalloc(bytes.len() + 1) as *mut c_char };
    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), ptr as *mut u8, bytes.len());
        *ptr.add(bytes.len()) = 0;
    }
    ptr
}

unsafe fn xstrdup_ptr(s: *const c_char) -> *mut c_char {
    unsafe { alloc_copy(c_bytes(s)) }
}

unsafe fn concat3_ptr(a: *const c_char, b: *const c_char, c: *const c_char) -> *mut c_char {
    let a_bytes = c_bytes(a);
    let b_bytes = if b.is_null() { &[][..] } else { c_bytes(b) };
    let c_bytes = if c.is_null() { &[][..] } else { c_bytes(c) };
    let len = a_bytes.len() + b_bytes.len() + c_bytes.len();
    let out = unsafe { xmalloc(len + 1) as *mut u8 };
    unsafe {
        ptr::copy_nonoverlapping(a_bytes.as_ptr(), out, a_bytes.len());
        ptr::copy_nonoverlapping(b_bytes.as_ptr(), out.add(a_bytes.len()), b_bytes.len());
        ptr::copy_nonoverlapping(
            c_bytes.as_ptr(),
            out.add(a_bytes.len() + b_bytes.len()),
            c_bytes.len(),
        );
        *out.add(len) = 0;
    }
    out as *mut c_char
}

unsafe fn set_nameoffile_from_c_str(src: *const c_char) {
    let len = c_strlen(src);
    unsafe {
        libc::free(nameoffile as *mut c_void);
        nameoffile = xmalloc(len + 2) as *mut u8;
        ptr::copy_nonoverlapping(src as *const u8, nameoffile.add(1), len + 1);
        namelength = len as c_int;
    }
}

unsafe fn nameoffile_body() -> *const c_char {
    unsafe { nameoffile.add(1) as *const c_char }
}

unsafe fn is_name_prefixed_dot_slash() -> bool {
    unsafe { *nameoffile.add(1) == b'.' && *nameoffile.add(2) == b'/' }
}

unsafe fn strip_leading_dot_slash_in_place(fname: *mut c_char) {
    let mut i = 0usize;
    unsafe {
        while *fname.add(i + 2) != 0 {
            *fname.add(i) = *fname.add(i + 2);
            i += 1;
        }
        *fname.add(i) = 0;
    }
}

unsafe fn record_name(prefix: *const c_char, name: *const c_char) {
    unsafe {
        if recorder_enabled == 0 {
            return;
        }
        if RECORDER_FILE.is_null() {
            recorder_start();
        }
        if !RECORDER_FILE.is_null() {
            libc::fprintf(RECORDER_FILE, c"%s %s\n".as_ptr(), prefix, name);
        }
    }
}

unsafe fn recorder_start() {
    unsafe {
        let pid = libc::getpid();
        let base = CString::new(format!("pdftex{pid}.fls")).expect("static recorder name");
        RECORDER_NAME = xstrdup_ptr(base.as_ptr());

        if !output_directory.is_null() {
            let joined = concat3_ptr(output_directory, c"/".as_ptr(), RECORDER_NAME);
            libc::free(RECORDER_NAME as *mut c_void);
            RECORDER_NAME = joined;
        }

        RECORDER_FILE = xfopen(RECORDER_NAME, c"wb".as_ptr());
        let cwd = xgetcwd();
        if !cwd.is_null() {
            libc::fprintf(RECORDER_FILE, c"PWD %s\n".as_ptr(), cwd);
            libc::free(cwd as *mut c_void);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn recorder_change_filename(new_name: *mut c_char) {
    unsafe {
        if RECORDER_FILE.is_null() {
            return;
        }

        let mut final_name = new_name;
        let mut allocated = false;
        if !output_directory.is_null() {
            final_name = concat3_ptr(output_directory, c"/".as_ptr(), new_name);
            allocated = true;
        }

        libc::rename(RECORDER_NAME, final_name);
        libc::free(RECORDER_NAME as *mut c_void);
        RECORDER_NAME = xstrdup_ptr(final_name);

        if allocated {
            libc::free(final_name as *mut c_void);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn recorder_record_input(name: *const c_char) {
    unsafe { record_name(c"INPUT".as_ptr(), name) }
}

#[no_mangle]
pub unsafe extern "C" fn recorder_record_output(name: *const c_char) {
    unsafe { record_name(c"OUTPUT".as_ptr(), name) }
}

#[no_mangle]
pub unsafe extern "C" fn open_input(
    f_ptr: *mut *mut FILE,
    filefmt: c_int,
    fopen_mode: *const c_char,
) -> c_int {
    unsafe {
        *f_ptr = ptr::null_mut();
        if !fullnameoffile.is_null() {
            libc::free(fullnameoffile as *mut c_void);
            fullnameoffile = ptr::null_mut();
        }

        let input_name = nameoffile_body();
        if !output_directory.is_null() && kpse_absolute_p(input_name, FALSE) == 0 {
            let fname = concat3_ptr(output_directory, c"/".as_ptr(), input_name);
            *f_ptr = libc::fopen(fname, fopen_mode);
            if !(*f_ptr).is_null() && dir_p(fname) != 0 {
                libc::fclose(*f_ptr);
                *f_ptr = ptr::null_mut();
            }
            if !(*f_ptr).is_null() {
                set_nameoffile_from_c_str(fname);
                fullnameoffile = fname;
            } else {
                libc::free(fname as *mut c_void);
            }
        }

        if (*f_ptr).is_null() {
            if filefmt < 0 {
                *f_ptr = libc::fopen(nameoffile_body(), fopen_mode);
            } else {
                let must_exist = ((filefmt != KPSE_TEX_FORMAT || texinputtype != 0)
                    && filefmt != KPSE_VF_FORMAT) as c_int;
                let fname = kpse_find_file(nameoffile_body(), filefmt as c_uint, must_exist);
                if !fname.is_null() {
                    fullnameoffile = xstrdup_ptr(fname);
                    if *fname == b'.' as c_char
                        && *fname.add(1) == b'/' as c_char
                        && !is_name_prefixed_dot_slash()
                    {
                        strip_leading_dot_slash_in_place(fname);
                    }
                    *f_ptr = xfopen(fname, fopen_mode);
                    set_nameoffile_from_c_str(fname);
                    libc::free(fname as *mut c_void);
                }
            }
        }

        if !(*f_ptr).is_null() {
            recorder_record_input(nameoffile_body());
            if filefmt == KPSE_TFM_FORMAT {
                tfmtemp = fast_fgetc(*f_ptr);
            } else if filefmt == KPSE_OCP_FORMAT {
                ocptemp = fast_fgetc(*f_ptr);
            } else if filefmt == KPSE_OFM_FORMAT {
                tfmtemp = fast_fgetc(*f_ptr);
            }
        }

        (!(*f_ptr).is_null()) as c_int
    }
}

#[no_mangle]
pub unsafe extern "C" fn open_input_with_dirname(
    f_ptr: *mut *mut FILE,
    filefmt: c_int,
    fname: *const c_char,
) -> c_int {
    unsafe {
        let mut ret = FALSE;
        let top_dir = xdirname(fname);
        if !top_dir.is_null()
            && *top_dir != 0
            && libc::strcmp(top_dir, c".".as_ptr()) != 0
            && kpse_absolute_p(nameoffile_body(), TRUE) == 0
        {
            let newname = concat3_ptr(top_dir, c"/".as_ptr(), nameoffile_body());
            set_nameoffile_from_c_str(newname);
            ret = open_input(f_ptr, filefmt, c"rb".as_ptr());
            libc::free(newname as *mut c_void);
        }
        libc::free(top_dir as *mut c_void);
        ret
    }
}

#[no_mangle]
pub unsafe extern "C" fn open_output(f_ptr: *mut *mut FILE, fopen_mode: *const c_char) -> c_int {
    unsafe {
        let original = nameoffile_body();
        let absolute = kpse_absolute_p(original, FALSE) != 0;
        let mut fname = original as *mut c_char;
        let mut allocated = false;

        if !output_directory.is_null() && !absolute {
            fname = concat3_ptr(output_directory, c"/".as_ptr(), original);
            allocated = true;
        }

        *f_ptr = libc::fopen(fname, fopen_mode);
        if (*f_ptr).is_null() {
            let texmfoutput = kpse_var_value(c"TEXMFOUTPUT".as_ptr());
            if !texmfoutput.is_null() && *texmfoutput != 0 && !absolute {
                if allocated {
                    libc::free(fname as *mut c_void);
                }
                fname = concat3_ptr(texmfoutput, c"/".as_ptr(), original);
                allocated = true;
                *f_ptr = libc::fopen(fname, fopen_mode);
            }
            libc::free(texmfoutput as *mut c_void);
        }

        if !(*f_ptr).is_null() {
            if allocated {
                set_nameoffile_from_c_str(fname);
            }
            recorder_record_output(fname);
        }
        if allocated {
            libc::free(fname as *mut c_void);
        }
        (!(*f_ptr).is_null()) as c_int
    }
}

#[no_mangle]
pub unsafe extern "C" fn close_file(f: *mut FILE) {
    unsafe {
        if f.is_null() {
            return;
        }
        if libc::fclose(f) == libc::EOF {
            libc::perror(c"fclose".as_ptr());
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn setupboundvariable(var: *mut c_int, var_name: *const c_char, dflt: c_int) {
    unsafe {
        *var = dflt;
        let expansion = kpse_var_value(var_name);
        if expansion.is_null() {
            return;
        }

        let conf_val = libc::atoi(expansion);
        if conf_val >= 0 && (conf_val != 0 || dflt <= 0) {
            *var = conf_val;
        } else {
            libc::fprintf(
                __stderrp,
                c"pdftex: Bad value (%ld) in environment or texmf.cnf for %s, keeping %ld.\n"
                    .as_ptr(),
                conf_val as libc::c_long,
                var_name,
                dflt as libc::c_long,
            );
        }
        libc::free(expansion as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn xmalloc(size: size_t) -> *mut c_void {
    unsafe {
        let actual = if size == 0 { 1 } else { size };
        let mem = libc::malloc(actual);
        if mem.is_null() {
            let msg = format!("fatal: memory exhausted (xmalloc of {size} bytes).\n");
            libc::write(2, msg.as_ptr() as *const c_void, msg.len());
            libc::exit(libc::EXIT_FAILURE);
        }
        mem
    }
}

#[no_mangle]
pub unsafe extern "C" fn xrealloc(old_ptr: *mut c_void, size: size_t) -> *mut c_void {
    unsafe {
        if old_ptr.is_null() {
            return xmalloc(size);
        }
        let actual = if size == 0 { 1 } else { size };
        let mem = libc::realloc(old_ptr, actual);
        if mem.is_null() {
            let msg = format!("fatal: memory exhausted (realloc of {size} bytes).\n");
            libc::write(2, msg.as_ptr() as *const c_void, msg.len());
            libc::exit(libc::EXIT_FAILURE);
        }
        mem
    }
}

#[no_mangle]
pub unsafe extern "C" fn xstrdup(s: *const c_char) -> *mut c_char {
    unsafe { xstrdup_ptr(s) }
}

#[cfg(test)]
mod tests {
    use super::{xmalloc, xrealloc, xstrdup};
    use libc::c_void;
    use std::ffi::{CStr, CString};

    #[test]
    fn xstrdup_returns_malloc_owned_copy() {
        let source = CString::new("hello").unwrap();
        let dup = unsafe { xstrdup(source.as_ptr()) };
        assert_eq!(unsafe { CStr::from_ptr(dup) }.to_str().unwrap(), "hello");
        unsafe { libc::free(dup as *mut c_void) };
    }

    #[test]
    fn zero_sized_allocations_still_return_storage() {
        let ptr = unsafe { xmalloc(0) };
        assert!(!ptr.is_null());
        let ptr = unsafe { xrealloc(ptr, 0) };
        assert!(!ptr.is_null());
        unsafe { libc::free(ptr) };
    }
}
