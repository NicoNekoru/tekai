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
        .map(|arg| arg_to_bytes(&arg))
        .collect::<Vec<_>>();
    // SAFETY: this runner is a dedicated single-invocation engine process.
    let code = match unsafe { tekai_engine::run_from_args(args) } {
        Ok(completion) => completion.exit_code(),
        Err(error) => {
            eprintln!("{error}");
            1
        }
    };
    std::process::exit(code);
}
