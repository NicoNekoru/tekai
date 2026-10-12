//! Rust-owned web2c file opening and configuration shims.

pub use crate::kpathsea::{kpse_absolute_p, kpse_find_file, kpse_var_value};
pub use crate::support::{dir_p, xdirname, xgetcwd};

use crate::stdio::Stream as FILE;
use crate::support::xfopen;
use libc::{c_char, c_int, c_uint, c_void, size_t};
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
    static mut nameoffile: *mut u8;
    static mut namelength: c_int;

}

#[inline]
unsafe fn fast_fgetc(stream: *mut FILE) -> c_int {
    unsafe { crate::stdio::fgetc(stream) }
}

fn c_strlen(s: *const c_char) -> usize {
    unsafe { crate::byte_string::strlen(s) }
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
        crate::allocation::free(nameoffile as *mut c_void);
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
            crate::stdio::fprintf_args(
                RECORDER_FILE,
                c"%s %s\n".as_ptr(),
                &[
                    crate::utils::PrintfArg::from(prefix),
                    crate::utils::PrintfArg::from(name),
                ],
            );
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
            crate::allocation::free(RECORDER_NAME as *mut c_void);
            RECORDER_NAME = joined;
        }

        RECORDER_FILE = xfopen(RECORDER_NAME, c"wb".as_ptr());
        let cwd = xgetcwd();
        if !cwd.is_null() {
            crate::stdio::fprintf_args(
                RECORDER_FILE,
                c"PWD %s\n".as_ptr(),
                &[crate::utils::PrintfArg::from(cwd)],
            );
            crate::allocation::free(cwd as *mut c_void);
        }
    }
}

pub unsafe fn recorder_change_filename(new_name: *mut c_char) {
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
        crate::allocation::free(RECORDER_NAME as *mut c_void);
        RECORDER_NAME = xstrdup_ptr(final_name);

        if allocated {
            crate::allocation::free(final_name as *mut c_void);
        }
    }
}

pub unsafe fn recorder_record_input(name: *const c_char) {
    unsafe { record_name(c"INPUT".as_ptr(), name) }
}

pub unsafe fn recorder_record_output(name: *const c_char) {
    unsafe { record_name(c"OUTPUT".as_ptr(), name) }
}

pub(crate) unsafe fn open_input(
    f_ptr: *mut *mut FILE,
    filefmt: c_int,
    fopen_mode: *const c_char,
) -> c_int {
    unsafe {
        *f_ptr = ptr::null_mut();
        if !fullnameoffile.is_null() {
            crate::allocation::free(fullnameoffile as *mut c_void);
            fullnameoffile = ptr::null_mut();
        }

        let input_name = nameoffile_body();
        if !output_directory.is_null() && kpse_absolute_p(input_name, FALSE) == 0 {
            let fname = concat3_ptr(output_directory, c"/".as_ptr(), input_name);
            *f_ptr = crate::stdio::fopen(fname, fopen_mode);
            if !(*f_ptr).is_null() && dir_p(fname) != 0 {
                crate::stdio::fclose(*f_ptr);
                *f_ptr = ptr::null_mut();
            }
            if !(*f_ptr).is_null() {
                set_nameoffile_from_c_str(fname);
                fullnameoffile = fname;
            } else {
                crate::allocation::free(fname as *mut c_void);
            }
        }

        if (*f_ptr).is_null() {
            if filefmt < 0 {
                *f_ptr = crate::stdio::fopen(nameoffile_body(), fopen_mode);
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
                    crate::allocation::free(fname as *mut c_void);
                }
            }
        }

        if !(*f_ptr).is_null() {
            if !crate::lookup::record_input(std::path::Path::new(
                CStr::from_ptr(nameoffile_body()).to_string_lossy().as_ref(),
            )) {
                eprintln!("tekai: current-pass generated-input tracking exceeds the 4 MiB limit");
                crate::stdio::fclose(*f_ptr);
                *f_ptr = ptr::null_mut();
                return FALSE;
            }
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

pub(crate) unsafe fn open_input_with_dirname(
    f_ptr: *mut *mut FILE,
    filefmt: c_int,
    fname: *const c_char,
) -> c_int {
    unsafe {
        let mut ret = FALSE;
        let top_dir = xdirname(fname);
        if !top_dir.is_null()
            && *top_dir != 0
            && crate::byte_string::strcmp(top_dir, c".".as_ptr()) != 0
            && kpse_absolute_p(nameoffile_body(), TRUE) == 0
        {
            let newname = concat3_ptr(top_dir, c"/".as_ptr(), nameoffile_body());
            set_nameoffile_from_c_str(newname);
            ret = open_input(f_ptr, filefmt, c"rb".as_ptr());
            crate::allocation::free(newname as *mut c_void);
        }
        crate::allocation::free(top_dir as *mut c_void);
        ret
    }
}

pub(crate) unsafe fn open_output(f_ptr: *mut *mut FILE, fopen_mode: *const c_char) -> c_int {
    unsafe {
        let original = nameoffile_body();
        let absolute = kpse_absolute_p(original, FALSE) != 0;
        let mut fname = original as *mut c_char;
        let mut allocated = false;

        if !output_directory.is_null() && !absolute {
            fname = concat3_ptr(output_directory, c"/".as_ptr(), original);
            allocated = true;
        }

        if !crate::lookup::can_record_output(std::path::Path::new(
            CStr::from_ptr(fname).to_string_lossy().as_ref(),
        )) {
            eprintln!("tekai: current-pass generated-output tracking exceeds the 4 MiB limit; output was not opened");
            if allocated {
                crate::allocation::free(fname as *mut c_void);
            }
            *f_ptr = ptr::null_mut();
            return FALSE;
        }
        *f_ptr = crate::stdio::fopen(fname, fopen_mode);
        if (*f_ptr).is_null() {
            let texmfoutput = kpse_var_value(c"TEXMFOUTPUT".as_ptr());
            if !texmfoutput.is_null() && *texmfoutput != 0 && !absolute {
                if allocated {
                    crate::allocation::free(fname as *mut c_void);
                }
                fname = concat3_ptr(texmfoutput, c"/".as_ptr(), original);
                allocated = true;
                if crate::lookup::can_record_output(std::path::Path::new(
                    CStr::from_ptr(fname).to_string_lossy().as_ref(),
                )) {
                    *f_ptr = crate::stdio::fopen(fname, fopen_mode);
                } else {
                    eprintln!("tekai: current-pass generated-output tracking exceeds the 4 MiB limit; fallback output was not opened");
                }
            }
            crate::allocation::free(texmfoutput as *mut c_void);
        }

        if !(*f_ptr).is_null() {
            if allocated {
                set_nameoffile_from_c_str(fname);
            }
            if !crate::lookup::record_output(std::path::Path::new(
                CStr::from_ptr(fname).to_string_lossy().as_ref(),
            )) {
                eprintln!("tekai: current-pass generated-output tracking exceeds the 4 MiB limit");
                crate::stdio::fclose(*f_ptr);
                *f_ptr = ptr::null_mut();
            } else {
                recorder_record_output(fname);
            }
        }
        if allocated {
            crate::allocation::free(fname as *mut c_void);
        }
        (!(*f_ptr).is_null()) as c_int
    }
}

pub(crate) unsafe fn close_file(f: *mut FILE) {
    unsafe {
        if f.is_null() {
            return;
        }
        if crate::stdio::fclose(f) == libc::EOF {
            crate::stdio::perror(c"fclose".as_ptr());
        }
    }
}

pub unsafe fn setupboundvariable(var: *mut c_int, var_name: *const c_char, dflt: c_int) {
    unsafe {
        *var = dflt;
        let expansion = kpse_var_value(var_name);
        if expansion.is_null() {
            return;
        }

        let conf_val =
            crate::integer_parse::decimal_i32_legacy(CStr::from_ptr(expansion).to_bytes()).value;
        if conf_val >= 0 && (conf_val != 0 || dflt <= 0) {
            *var = conf_val;
        } else {
            crate::stdio::fprintf_args(
                crate::stdio::stderr(),
                c"pdftex: Bad value (%ld) in environment or texmf.cnf for %s, keeping %ld.\n"
                    .as_ptr(),
                &[
                    crate::utils::PrintfArg::from(conf_val as libc::c_long),
                    crate::utils::PrintfArg::from(var_name),
                    crate::utils::PrintfArg::from(dflt as libc::c_long),
                ],
            );
        }
        crate::allocation::free(expansion as *mut c_void);
    }
}

/// Allocate an engine-owned buffer, terminating with the existing diagnostic on
/// overflow or allocation failure.
///
/// # Safety
/// Initialize bytes before reading them. Release storage through
/// `crate::allocation::free`, never a foreign deallocator.
pub(crate) unsafe fn xmalloc(size: size_t) -> *mut c_void {
    unsafe {
        let mem = crate::allocation::malloc(size);
        if mem.is_null() {
            let msg = format!("fatal: memory exhausted (xmalloc of {size} bytes).\n");
            libc::write(2, msg.as_ptr() as *const c_void, msg.len());
            crate::stdio::exit(libc::EXIT_FAILURE);
        }
        mem
    }
}

/// Resize engine-owned storage, terminating on overflow or allocation failure.
///
/// # Safety
/// A non-null pointer must be the exact live pointer produced by the engine's
/// Rust allocator. No references may be used during the resize.
pub(crate) unsafe fn xrealloc(old_ptr: *mut c_void, size: size_t) -> *mut c_void {
    unsafe {
        if old_ptr.is_null() {
            return xmalloc(size);
        }
        let mem = crate::allocation::realloc(old_ptr, size);
        if mem.is_null() {
            let msg = format!("fatal: memory exhausted (realloc of {size} bytes).\n");
            libc::write(2, msg.as_ptr() as *const c_void, msg.len());
            crate::stdio::exit(libc::EXIT_FAILURE);
        }
        mem
    }
}

/// Copy a NUL-terminated string into engine-owned Rust storage.
///
/// # Safety
/// The input must remain valid through the copy. Release the returned pointer
/// through `crate::allocation::free`.
pub(crate) unsafe fn xstrdup(s: *const c_char) -> *mut c_char {
    unsafe { xstrdup_ptr(s) }
}

#[cfg(test)]
mod tests {
    use super::{xmalloc, xrealloc, xstrdup};
    use libc::c_void;
    use proptest::prelude::*;
    use std::ffi::{CStr, CString};

    #[test]
    fn xstrdup_returns_rust_allocated_copy() {
        let source = CString::new("hello").unwrap();
        let dup = unsafe { xstrdup(source.as_ptr()) };
        assert_eq!(unsafe { CStr::from_ptr(dup) }.to_str().unwrap(), "hello");
        unsafe { crate::allocation::free(dup as *mut c_void) };
    }

    #[test]
    fn zero_sized_allocations_still_return_storage() {
        let ptr = unsafe { xmalloc(0) };
        assert!(!ptr.is_null());
        let ptr = unsafe { xrealloc(ptr, 0) };
        assert!(!ptr.is_null());
        unsafe { crate::allocation::free(ptr) };
    }

    proptest! {
        #[test]
        fn duplicated_strings_resize_and_release_with_rust_allocator(
            bytes in prop::collection::vec(1u8..=255, 0..4096),
            extra in 1usize..129,
        ) {
            let source = CString::new(bytes.clone()).unwrap();
            let duplicate = unsafe { xstrdup(source.as_ptr()) };
            let resized = unsafe { xrealloc(duplicate.cast(), bytes.len() + extra + 1) };
            assert!(!resized.is_null());
            let preserved = unsafe { CStr::from_ptr(resized.cast()) }.to_bytes();
            let matches = preserved == bytes.as_slice();
            unsafe { crate::allocation::free(resized) };
            prop_assert!(matches);
        }
    }
}
