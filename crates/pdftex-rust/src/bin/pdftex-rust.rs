use std::ffi::CString;

#[cfg(unix)]
fn arg_to_bytes(arg: &std::ffi::OsStr) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;
    arg.as_bytes().to_vec()
}

#[cfg(not(unix))]
fn arg_to_bytes(arg: &std::ffi::OsStr) -> Vec<u8> {
    arg.to_string_lossy().into_owned().into_bytes()
}

fn main() {
    let args = std::env::args_os()
        .map(|arg| CString::new(arg_to_bytes(&arg)).expect("process argument contained NUL"))
        .collect::<Vec<_>>();
    let mut argv = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect::<Vec<_>>();
    argv.push(std::ptr::null_mut());

    let code =
        unsafe { pdftex_rust::run_from_c_args(args.len() as libc::c_int, argv.as_mut_ptr()) };
    std::process::exit(code);
}
