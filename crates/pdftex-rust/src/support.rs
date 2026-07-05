//! Cold web2c support routines owned by Rust.

use crate::openclose::xmalloc;
use libc::{c_char, c_double, c_int, FILE};
use std::ffi::{CStr, CString};
use std::ptr;

#[no_mangle]
pub static mut versionstring: *const c_char = c" (TeX Live 2027/dev)".as_ptr();

extern "C" {
    static mut __stderrp: *mut FILE;
    static mut __stdoutp: *mut FILE;
    static kpathsea_version_string: *const c_char;

    fn uexit(unix_code: c_int);
}

unsafe fn copy_bytes_with_nul(bytes: &[u8]) -> *mut c_char {
    let out = unsafe { xmalloc(bytes.len() + 1) as *mut u8 };
    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), out, bytes.len());
        *out.add(bytes.len()) = 0;
    }
    out as *mut c_char
}

unsafe fn c_bytes(s: *const c_char) -> &'static [u8] {
    unsafe { CStr::from_ptr(s).to_bytes() }
}

#[no_mangle]
pub unsafe extern "C" fn xbasename(name: *const c_char) -> *const c_char {
    unsafe {
        let mut base = name;
        let mut p = name;
        while *p != 0 {
            if *p == b'/' as c_char {
                base = p.add(1);
            }
            p = p.add(1);
        }
        base
    }
}

#[no_mangle]
pub unsafe extern "C" fn basenamechangesuffix(
    name: *const c_char,
    old_suffix: *const c_char,
    new_suffix: *const c_char,
) -> *mut c_char {
    unsafe {
        let base = xbasename(name);
        let base_bytes = c_bytes(base);
        let old_bytes = c_bytes(old_suffix);
        let new_bytes = c_bytes(new_suffix);

        let copy_limit =
            if old_bytes.len() <= base_bytes.len() && base_bytes.ends_with(old_bytes) {
                base_bytes.len() - old_bytes.len()
            } else {
                base_bytes.len()
            };

        let out = xmalloc(copy_limit + new_bytes.len() + 1) as *mut u8;
        ptr::copy_nonoverlapping(base_bytes.as_ptr(), out, copy_limit);
        ptr::copy_nonoverlapping(new_bytes.as_ptr(), out.add(copy_limit), new_bytes.len());
        *out.add(copy_limit + new_bytes.len()) = 0;
        out as *mut c_char
    }
}

#[no_mangle]
pub unsafe extern "C" fn chartostring(ch: c_char) -> *mut c_char {
    unsafe { copy_bytes_with_nul(&[ch as u8]) }
}

#[no_mangle]
pub unsafe extern "C" fn fprintreal(f: *mut FILE, r: c_double, n: c_int, m: c_int) {
    let fmt = CString::new(format!("%{n}.{m}lf")).expect("numeric printf format");
    unsafe {
        libc::fprintf(f, fmt.as_ptr(), r);
    }
}

#[no_mangle]
pub unsafe extern "C" fn inputint(f: *mut FILE) -> c_int {
    let mut buffer = [0 as c_char; 64];
    unsafe {
        if libc::fgets(buffer.as_mut_ptr(), buffer.len() as c_int, f).is_null() {
            0
        } else {
            libc::atoi(buffer.as_ptr())
        }
    }
}

unsafe fn discard_stdin_line() -> c_int {
    unsafe {
        let mut ch = libc::getchar();
        while ch != libc::EOF && ch != b'\n' as c_int {
            ch = libc::getchar();
        }
        ch
    }
}

#[no_mangle]
pub unsafe extern "C" fn zinput2ints(a: *mut c_int, b: *mut c_int) {
    unsafe {
        while libc::scanf(c"%d %d".as_ptr(), a, b) != 2 {
            if discard_stdin_line() == libc::EOF {
                return;
            }
            libc::fprintf(__stderrp, c"Please enter two integers.\n".as_ptr());
        }
        discard_stdin_line();
    }
}

#[no_mangle]
pub unsafe extern "C" fn zinput3ints(a: *mut c_int, b: *mut c_int, c: *mut c_int) {
    unsafe {
        while libc::scanf(c"%d %d %d".as_ptr(), a, b, c) != 3 {
            if discard_stdin_line() == libc::EOF {
                return;
            }
            libc::fprintf(__stderrp, c"Please enter three integers.\n".as_ptr());
        }
        discard_stdin_line();
    }
}

#[no_mangle]
pub unsafe extern "C" fn usage(str_: *const c_char) {
    unsafe {
        libc::fprintf(
            __stderrp,
            c"Try `%s --help' for more information.\n".as_ptr(),
            str_,
        );
        uexit(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn usagehelp(mut message: *mut *const c_char, mut bug_email: *const c_char) {
    unsafe {
        if bug_email.is_null() {
            bug_email = c"tex-k@tug.org".as_ptr();
        }

        while !(*message).is_null() {
            libc::printf(c"%s\n".as_ptr(), *message);
            message = message.add(1);
        }

        libc::printf(c"\nEmail bug reports to %s".as_ptr(), bug_email);
        if libc::strlen(bug_email) > 9 {
            let domain = libc::strchr(bug_email, b'@' as c_int);
            if !domain.is_null() && libc::strcmp(domain, c"@tug.org".as_ptr()) == 0 {
                libc::printf(c" (https://lists.tug.org/".as_ptr());
                let mut ptr = bug_email;
                while ptr < domain {
                    libc::putchar(*ptr as c_int);
                    ptr = ptr.add(1);
                }
                libc::printf(c")".as_ptr());
            }
        }
        libc::puts(c".".as_ptr());
        uexit(0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn printversionandexit(
    banner: *const c_char,
    copyright_holder: *const c_char,
    mut author: *const c_char,
    extra_info: *const c_char,
) {
    unsafe {
        let prog_name_end = libc::strchr(banner, b',' as c_int);
        let prog_version = libc::strrchr(banner, b' ' as c_int);
        if prog_name_end.is_null() || prog_version.is_null() {
            libc::abort();
        }

        let len = prog_name_end.offset_from(banner) as usize;
        let mut prog_name = Vec::with_capacity(len + 1);
        prog_name.extend_from_slice(std::slice::from_raw_parts(banner as *const u8, len));
        prog_name.push(0);

        let prog_name_start = libc::strrchr(prog_name.as_ptr() as *const c_char, b' ' as c_int);
        if prog_name_start.is_null() {
            libc::abort();
        }
        let prog_name_start = prog_name_start.add(1);
        let prog_version = prog_version.add(1);

        libc::printf(
            c"%s %s%s\n".as_ptr(),
            prog_name_start,
            prog_version,
            versionstring,
        );
        libc::puts(kpathsea_version_string);

        if !copyright_holder.is_null() {
            libc::printf(c"Copyright 2026 %s.\n".as_ptr(), copyright_holder);
            if author.is_null() {
                author = copyright_holder;
            }
        }

        libc::puts(c"There is NO warranty.  Redistribution of this software is".as_ptr());
        libc::fputs(c"covered by the terms of ".as_ptr(), __stdoutp);
        libc::printf(c"both the %s copyright and\n".as_ptr(), prog_name_start);
        libc::puts(c"the Lesser GNU General Public License.".as_ptr());
        libc::puts(c"For more information about these matters, see the file".as_ptr());
        libc::printf(c"named COPYING and the %s source.\n".as_ptr(), prog_name_start);
        libc::printf(c"Primary author of %s: %s.\n".as_ptr(), prog_name_start, author);

        if !extra_info.is_null() {
            libc::fputs(extra_info, __stdoutp);
        }

        uexit(0);
    }
}

#[cfg(test)]
mod tests {
    use super::{basenamechangesuffix, chartostring};
    use libc::c_void;
    use std::ffi::{CStr, CString};

    #[test]
    fn chartostring_allocates_single_character_c_string() {
        let s = unsafe { chartostring(b'X' as i8) };
        assert_eq!(unsafe { CStr::from_ptr(s) }.to_bytes(), b"X");
        unsafe { libc::free(s as *mut c_void) };
    }

    #[test]
    fn basenamechangesuffix_replaces_matching_suffix() {
        let name = CString::new("/tmp/cmr10.300pk").unwrap();
        let old_suffix = CString::new("pk").unwrap();
        let new_suffix = CString::new("gf").unwrap();
        let changed =
            unsafe { basenamechangesuffix(name.as_ptr(), old_suffix.as_ptr(), new_suffix.as_ptr()) };
        assert_eq!(
            unsafe { CStr::from_ptr(changed) }.to_str().unwrap(),
            "cmr10.300gf"
        );
        unsafe { libc::free(changed as *mut c_void) };
    }
}
