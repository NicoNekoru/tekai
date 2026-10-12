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
    const WORKER: &str = "--__tekai-engine-worker";
    let is_worker = std::env::args_os().nth(1).as_deref() == Some(std::ffi::OsStr::new(WORKER));
    if !is_worker && !tekai_engine::cli_supervisor::inherited_group_is_supervised() {
        let result = (|| -> Result<_, Box<dyn std::error::Error>> {
            let mut command = std::process::Command::new(std::env::current_exe()?);
            command.arg(WORKER).args(std::env::args_os().skip(1));
            Ok(tekai_engine::cli_supervisor::supervise(&mut command)?)
        })();
        let code = match result {
            Ok(status) => {
                use std::os::unix::process::ExitStatusExt;
                status
                    .code()
                    .unwrap_or_else(|| 128 + status.signal().unwrap_or(1))
            }
            Err(error) => {
                eprintln!("{error}");
                1
            }
        };
        std::process::exit(code);
    }
    let args = std::env::args_os()
        .enumerate()
        .filter(|(index, _)| !is_worker || *index != 1)
        .map(|(_, arg)| arg_to_bytes(&arg))
        .collect::<Vec<_>>();
    let code = match run(args) {
        Ok(completion) => completion.exit_code(),
        Err(error) => {
            eprintln!("{error}");
            1
        }
    };
    std::process::exit(code);
}

fn run(args: Vec<Vec<u8>>) -> Result<tekai_engine::EngineCompletion, Box<dyn std::error::Error>> {
    let options = tekai_engine::InvocationOptions {
        pipe_process_policy: tekai_engine::PipeProcessPolicy::SupervisedProcess,
        environment: vec![(
            tekai_engine::cli_supervisor::SUPERVISED_GROUP_ENV.into(),
            Some(
                rustix::process::getpgrp()
                    .as_raw_nonzero()
                    .to_string()
                    .into(),
            ),
        )],
        ..tekai_engine::InvocationOptions::default()
    };
    #[cfg(unix)]
    let _handler = InterruptHandler(signal_hook::flag::register(
        signal_hook::consts::SIGINT,
        options.interrupt.signal_flag(),
    )?);
    // SAFETY This CLI expects trusted TeX input and format files. Its signal
    // registration is outside the library and ends on return or unwind.
    Ok(unsafe { tekai_engine::run_with_options(args, options) }?)
}

#[cfg(unix)]
struct InterruptHandler(signal_hook::SigId);

#[cfg(unix)]
impl Drop for InterruptHandler {
    fn drop(&mut self) {
        signal_hook::low_level::unregister(self.0);
    }
}
