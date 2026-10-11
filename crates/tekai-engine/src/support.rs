//! Cold web2c support routines owned by Rust.

use crate::openclose::{xmalloc, xrealloc};
use crate::stdio::Stream as FILE;
use libc::{c_char, c_double, c_int, c_long, c_void, off_t, size_t};
use std::ffi::{CStr, CString};
use std::ptr;

#[no_mangle]
pub static mut versionstring: *const c_char = c" (TeX Live 2027/dev)".as_ptr();

extern "C" {
    static kpathsea_version_string: *const c_char;

    fn uexit(unix_code: c_int);
}

#[inline]
unsafe fn fast_fgetc(stream: *mut FILE) -> c_int {
    unsafe { crate::stdio::fgetc(stream) }
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
pub unsafe extern "C" fn concat3(
    s1: *const c_char,
    s2: *const c_char,
    s3: *const c_char,
) -> *mut c_char {
    let s1 = unsafe { c_bytes(s1) };
    let s2 = unsafe { c_bytes(s2) };
    let s3 = unsafe { c_bytes(s3) };
    let len = s1.len() + s2.len() + s3.len();
    let out = unsafe { xmalloc(len + 1) as *mut u8 };
    unsafe {
        ptr::copy_nonoverlapping(s1.as_ptr(), out, s1.len());
        ptr::copy_nonoverlapping(s2.as_ptr(), out.add(s1.len()), s2.len());
        ptr::copy_nonoverlapping(s3.as_ptr(), out.add(s1.len() + s2.len()), s3.len());
        *out.add(len) = 0;
    }
    out as *mut c_char
}

#[no_mangle]
pub unsafe extern "C" fn concat(s1: *const c_char, s2: *const c_char) -> *mut c_char {
    unsafe { concat3(s1, s2, c"".as_ptr()) }
}

#[no_mangle]
pub unsafe extern "C" fn xdirname(name: *const c_char) -> *mut c_char {
    unsafe {
        let bytes = c_bytes(name);
        let slash = bytes.iter().rposition(|&b| b == b'/');
        match slash {
            Some(0) => copy_bytes_with_nul(b"/"),
            Some(pos) => copy_bytes_with_nul(&bytes[..pos]),
            None => copy_bytes_with_nul(b"."),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn xgetcwd() -> *mut c_char {
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let cwd = CString::new(cwd.to_string_lossy().as_bytes()).expect("cwd contained NUL");
    unsafe { copy_bytes_with_nul(cwd.as_bytes()) }
}

#[no_mangle]
pub unsafe extern "C" fn xputenv(var: *const c_char, value: *const c_char) {
    unsafe {
        if var.is_null() || value.is_null() {
            return;
        }
        libc::setenv(var, value, 1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn dir_p(name: *mut c_char) -> c_int {
    if name.is_null() {
        return 0;
    }
    let path = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };
    std::fs::metadata(path)
        .map(|metadata| metadata.is_dir() as c_int)
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn find_suffix(name: *const c_char) -> *const c_char {
    unsafe {
        if name.is_null() {
            return ptr::null();
        }
        let mut dot = ptr::null();
        let mut p = name;
        while *p != 0 {
            if *p == b'/' as c_char {
                dot = ptr::null();
            } else if *p == b'.' as c_char {
                dot = p.add(1);
            }
            p = p.add(1);
        }
        dot
    }
}

unsafe fn fatal_perror(filename: *const c_char) -> ! {
    unsafe {
        crate::stdio::perror(filename);
        crate::stdio::exit(libc::EXIT_FAILURE);
    }
}

pub(crate) unsafe fn xfopen(filename: *const c_char, mode: *const c_char) -> *mut FILE {
    unsafe {
        let file = crate::stdio::fopen(filename, mode);
        if file.is_null() {
            fatal_perror(filename);
        }
        file
    }
}

pub(crate) unsafe fn xfclose(fp: *mut FILE, filename: *const c_char) {
    unsafe {
        if crate::stdio::fclose(fp) == libc::EOF {
            fatal_perror(filename);
        }
    }
}

pub(crate) unsafe fn xfseek(
    fp: *mut FILE,
    offset: c_long,
    wherefrom: c_int,
    filename: *const c_char,
) {
    unsafe {
        if crate::stdio::fseek(fp, offset, wherefrom) < 0 {
            fatal_perror(filename);
        }
    }
}

pub(crate) unsafe fn xfseeko(
    fp: *mut FILE,
    offset: off_t,
    wherefrom: c_int,
    filename: *const c_char,
) {
    unsafe {
        if crate::stdio::fseeko(fp, offset, wherefrom) < 0 {
            fatal_perror(filename);
        }
    }
}

pub(crate) unsafe fn xftell(fp: *mut FILE, filename: *const c_char) -> c_long {
    unsafe {
        let offset = crate::stdio::ftell(fp);
        if offset < 0 {
            fatal_perror(filename);
        }
        offset
    }
}

pub(crate) unsafe fn xftello(fp: *mut FILE, filename: *const c_char) -> off_t {
    unsafe {
        let offset = crate::stdio::ftello(fp);
        if offset < 0 {
            fatal_perror(filename);
        }
        offset
    }
}

pub(crate) unsafe fn read_line(f: *mut FILE) -> *mut c_char {
    const BLOCK_SIZE: usize = 75;
    let mut limit = BLOCK_SIZE;
    let mut loc = 0usize;
    let mut line = unsafe { xmalloc(limit) as *mut u8 };

    unsafe {
        let mut ch = fast_fgetc(f);
        while ch != libc::EOF && ch != b'\n' as c_int && ch != b'\r' as c_int {
            if ch != 0 {
                *line.add(loc) = ch as u8;
                loc += 1;
                if loc == limit {
                    limit += BLOCK_SIZE;
                    line = xrealloc(line as *mut c_void, limit as size_t) as *mut u8;
                }
            }
            ch = fast_fgetc(f);
        }
        if loc == 0 && ch == libc::EOF {
            crate::allocation::free(line as *mut c_void);
            return ptr::null_mut();
        }
        *line.add(loc) = 0;
        if ch == b'\r' as c_int {
            let next = fast_fgetc(f);
            if next != b'\n' as c_int && next != libc::EOF {
                crate::stdio::ungetc(next, f);
            }
        }
    }
    line as *mut c_char
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

        let copy_limit = if old_bytes.len() <= base_bytes.len() && base_bytes.ends_with(old_bytes) {
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

pub(crate) unsafe fn fprintreal(f: *mut FILE, r: c_double, n: c_int, m: c_int) {
    let fmt = CString::new(format!("%{n}.{m}lf")).expect("numeric printf format");
    unsafe {
        crate::stdio::fprintf_args(f, fmt.as_ptr(), &[crate::utils::PrintfArg::from(r)]);
    }
}

pub(crate) unsafe fn inputint(f: *mut FILE) -> c_int {
    let mut buffer = [0 as c_char; 64];
    unsafe {
        if crate::stdio::fgets(buffer.as_mut_ptr(), buffer.len() as c_int, f).is_null() {
            0
        } else {
            libc::atoi(buffer.as_ptr())
        }
    }
}

unsafe fn discard_stdin_line() -> c_int {
    unsafe {
        let mut ch = crate::stdio::fgetc(crate::stdio::stdin());
        while ch != libc::EOF && ch != b'\n' as c_int {
            ch = crate::stdio::fgetc(crate::stdio::stdin());
        }
        ch
    }
}

unsafe fn scan_integers(destinations: &[*mut c_int]) -> usize {
    for (index, &destination) in destinations.iter().enumerate() {
        let Some(value) = (unsafe { crate::stdio::scan_integer(crate::stdio::stdin()) }) else {
            return index;
        };
        unsafe {
            *destination = value;
        }
    }
    destinations.len()
}

#[no_mangle]
pub unsafe extern "C" fn zinput2ints(a: *mut c_int, b: *mut c_int) {
    unsafe {
        while scan_integers(&[a, b]) != 2 {
            if discard_stdin_line() == libc::EOF {
                return;
            }
            crate::stdio::fprintf_args(
                crate::stdio::stderr(),
                c"Please enter two integers.\n".as_ptr(),
                &[],
            );
        }
        discard_stdin_line();
    }
}

#[no_mangle]
pub unsafe extern "C" fn zinput3ints(a: *mut c_int, b: *mut c_int, c: *mut c_int) {
    unsafe {
        while scan_integers(&[a, b, c]) != 3 {
            if discard_stdin_line() == libc::EOF {
                return;
            }
            crate::stdio::fprintf_args(
                crate::stdio::stderr(),
                c"Please enter three integers.\n".as_ptr(),
                &[],
            );
        }
        discard_stdin_line();
    }
}

#[no_mangle]
pub unsafe extern "C" fn usage(str_: *const c_char) {
    unsafe {
        crate::stdio::fprintf_args(
            crate::stdio::stderr(),
            c"Try `%s --help' for more information.\n".as_ptr(),
            &[crate::utils::PrintfArg::from(str_)],
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
            crate::stdio::printf_args(c"%s\n".as_ptr(), &[crate::utils::PrintfArg::from(*message)]);
            message = message.add(1);
        }

        crate::stdio::printf_args(
            c"\nEmail bug reports to %s".as_ptr(),
            &[crate::utils::PrintfArg::from(bug_email)],
        );
        if libc::strlen(bug_email) > 9 {
            let domain = libc::strchr(bug_email, b'@' as c_int);
            if !domain.is_null() && libc::strcmp(domain, c"@tug.org".as_ptr()) == 0 {
                crate::stdio::printf_args(c" (https://lists.tug.org/".as_ptr(), &[]);
                let mut ptr = bug_email;
                while ptr < domain {
                    crate::stdio::putc(*ptr as c_int, crate::stdio::stdout());
                    ptr = ptr.add(1);
                }
                crate::stdio::printf_args(c")".as_ptr(), &[]);
            }
        }
        crate::stdio::puts(c".".as_ptr());
        uexit(0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn printversionandexit(
    banner: *const c_char,
    _copyright_holder: *const c_char,
    _author: *const c_char,
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

        crate::stdio::printf_args(
            c"%s %s%s\n".as_ptr(),
            &[
                crate::utils::PrintfArg::from(prog_name_start),
                crate::utils::PrintfArg::from(prog_version),
                crate::utils::PrintfArg::from(versionstring),
            ],
        );
        crate::stdio::puts(kpathsea_version_string);

        crate::stdio::puts(c"Copyright 2026 Tekai contributors.".as_ptr());
        crate::stdio::puts(c"Licensed under the MIT License.".as_ptr());
        crate::stdio::puts(c"https://github.com/NicoNekoru/tekai".as_ptr());

        if !extra_info.is_null() {
            crate::stdio::fputs(extra_info, crate::stdio::stdout());
        }

        uexit(0);
    }
}

#[cfg(test)]
mod tests {
    use super::{basenamechangesuffix, chartostring, read_line, xgetcwd};
    use libc::c_void;
    use proptest::prelude::*;
    use std::ffi::{CStr, CString};

    #[test]
    fn chartostring_allocates_single_character_c_string() {
        let s = unsafe { chartostring(b'X' as i8) };
        assert_eq!(unsafe { CStr::from_ptr(s) }.to_bytes(), b"X");
        unsafe { crate::allocation::free(s as *mut c_void) };
    }

    #[test]
    fn basenamechangesuffix_replaces_matching_suffix() {
        let name = CString::new("/tmp/cmr10.300pk").unwrap();
        let old_suffix = CString::new("pk").unwrap();
        let new_suffix = CString::new("gf").unwrap();
        let changed = unsafe {
            basenamechangesuffix(name.as_ptr(), old_suffix.as_ptr(), new_suffix.as_ptr())
        };
        assert_eq!(
            unsafe { CStr::from_ptr(changed) }.to_str().unwrap(),
            "cmr10.300gf"
        );
        unsafe { crate::allocation::free(changed as *mut c_void) };
    }

    #[test]
    fn current_directory_returns_an_owned_copy() {
        let expected = std::env::current_dir().unwrap();
        let directory = unsafe { xgetcwd() };
        let actual = unsafe { CStr::from_ptr(directory) }.to_bytes().to_vec();
        unsafe { crate::allocation::free(directory.cast()) };
        assert_eq!(actual, expected.to_string_lossy().as_bytes());
    }

    proptest! {
        #[test]
        fn line_buffers_grow_and_preserve_legacy_nul_and_crlf_handling(
            payload in prop::collection::vec(any::<u8>().prop_filter("line byte", |b| *b != b'\r' && *b != b'\n'), 0..4096),
            crlf in any::<bool>(),
        ) {
            struct File(*mut crate::stdio::Stream);
            impl Drop for File {
                fn drop(&mut self) {
                    unsafe { crate::stdio::fclose(self.0) };
                }
            }

            let file = File(unsafe { crate::stdio::tmpfile() });
            assert!(!file.0.is_null());
            let mut input = payload.clone();
            input.extend_from_slice(if crlf { b"\r\n" } else { b"\n" });
            input.extend_from_slice(b"tail\n");
            assert_eq!(unsafe { crate::stdio::fwrite(input.as_ptr().cast(), 1, input.len(), file.0) }, input.len());
            unsafe { crate::stdio::rewind(file.0) };
            let line = unsafe { read_line(file.0) };
            assert!(!line.is_null());
            let actual = unsafe { CStr::from_ptr(line) }.to_bytes().to_vec();
            unsafe { crate::allocation::free(line.cast()) };
            let tail = unsafe { read_line(file.0) };
            assert!(!tail.is_null());
            let actual_tail = unsafe { CStr::from_ptr(tail) }.to_bytes().to_vec();
            unsafe { crate::allocation::free(tail.cast()) };
            let eof = unsafe { read_line(file.0) };
            let at_eof = eof.is_null();
            unsafe { crate::allocation::free(eof.cast()) };
            let expected = payload.into_iter().filter(|byte| *byte != 0).collect::<Vec<_>>();
            prop_assert_eq!(actual, expected);
            prop_assert_eq!(actual_tail, b"tail");
            prop_assert!(at_eof);
        }
    }
}
