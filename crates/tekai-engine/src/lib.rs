//! Self-contained exact typesetting engine for Tekai.
//!
//! The CLI and standalone runner link this crate as a Rust library. The
//! core and backend calls use Rust module interfaces and invocation-owned state.
//! This crate does not produce a C-linkable archive or export a C entrypoint.

#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    overflowing_literals,
    path_statements,
    unreachable_code,
    unused_assignments,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_unsafe,
    unused_variables,
    useless_ptr_null_checks
)]
#![allow(clippy::all)]

#[cfg(panic = "abort")]
compile_error!("tekai-engine requires panic=unwind for controlled exits and invocation teardown");

mod generated {
    pub(crate) mod backend {
        pub(crate) mod avl;
        pub(crate) mod avlstuff;
        pub(crate) mod epdf;
        pub(crate) mod mapfile;
        pub(crate) mod pkin;
        pub(crate) mod subfont;
        pub(crate) mod tounicode;
        pub(crate) mod vfpacket;
        pub(crate) mod writeenc;
        pub(crate) mod writefont;
        pub(crate) mod writeimg;
        pub(crate) mod writejbig2;
        pub(crate) mod writejpg;
        pub(crate) mod writepng;
        pub(crate) mod writet1;
        pub(crate) mod writet3;
        pub(crate) mod writettf;
        pub(crate) mod writezip;
    }
    pub(crate) mod pdftex0;
    pub(crate) mod pdftex_pool;
    pub(crate) mod pdftexextra;
    pub(crate) mod pdftexini;
}

mod allocation;
mod backend_grammar;
mod backend_parse;
mod backend_types;
mod byte_string;
mod cache;
mod checked_width;
#[cfg(all(feature = "cli-supervisor", unix))]
pub mod cli_supervisor;
mod control;
mod core_types;
mod directory_graph;
mod embedded;
mod engine_options;
mod environment;
pub mod file_identity;
mod float_parse;
mod integer_parse;
mod invocation;
mod invocation_state;
mod ipc;
mod jpeg_exif;
pub mod kpathsea;
mod kpathsea_types;
pub mod lookup;
mod md5;
mod openclose;
mod pdftoepdf;
mod pngshim;
mod posix_regex;
mod process_group;
mod regex_session;
pub mod runtime;
pub mod search;
mod stdio;
mod support;
mod synctex;
mod termination;
mod time;
mod ttf_name;
mod ttf_post;
mod utils;
mod web2c;
mod working_directory;
mod xpdf;

pub use control::InterruptToken;
pub use invocation::{EngineCompletion, InvocationError, InvocationOptions};
#[cfg(test)]
use libz_rs_sys as zlib;
pub use stdio::PipeProcessPolicy;

/// Run the engine with owned argument bytes, including argv0.
///
/// Every call owns its core state, buffers, caches, streams and argument storage.
/// Repeated calls and independent threads do not share mutable engine state.
/// Nested calls on the same thread are rejected. Fatal and help/version exits
/// return without terminating the host. Unexpected Rust panics propagate after
/// invocation teardown. Host environment, directory and signal handlers are not
/// changed.
///
/// # Safety
///
/// The translated engine still uses unchecked pointers and format data. The
/// caller must provide trusted input and format files and must not call internal
/// engine functions outside their invocation scope. Invocation ownership is not
/// a sandbox for untrusted TeX or shell commands.
/// Recovery requires unwind-enabled builds. The host must not independently
/// reap engine pipe children or configure SIGCHLD to reap them automatically.
pub unsafe fn run_from_args(arguments: Vec<Vec<u8>>) -> Result<EngineCompletion, InvocationError> {
    unsafe { run_with_options(arguments, InvocationOptions::default()) }
}

/// Run with an invocation-local directory, environment and interrupt token.
///
/// # Safety
/// The same trusted-input requirements as [`run_from_args`] apply.
pub unsafe fn run_with_options(
    arguments: Vec<Vec<u8>>,
    options: InvocationOptions,
) -> Result<EngineCompletion, InvocationError> {
    if invocation_state::is_active() {
        return Err(InvocationError::NestedInvocation);
    }
    let invocation = Invocation::new(arguments, &options)?;
    unsafe { invocation.run(options) }
}

// Field order is teardown order. No resource may outlive the raw-buffer arena,
// and argument bytes outlive core aliases. Routing scopes always end first.
struct Invocation {
    core: std::pin::Pin<Box<invocation_state::InvocationState>>,
    compression: std::pin::Pin<Box<generated::backend::writezip::CompressionSession>>,
    streams: std::pin::Pin<Box<stdio::StreamSession>>,
    regex: std::pin::Pin<Box<regex_session::RegexSession>>,
    environment: std::pin::Pin<Box<environment::EnvironmentSession>>,
    directory: std::pin::Pin<Box<working_directory::WorkingDirectory>>,
    time: Option<std::pin::Pin<Box<time::TimeSession>>>,
    runtime: Option<std::pin::Pin<Box<runtime::RuntimeSession>>>,
    index: std::pin::Pin<Box<kpathsea::IndexSession>>,
    lookup: std::pin::Pin<Box<lookup::LookupOwner>>,
    png: std::pin::Pin<Box<pngshim::PngSession>>,
    sync: std::pin::Pin<Box<synctex::SyncTexSession>>,
    xpdf: std::pin::Pin<Box<xpdf::XpdfSession>>,
    inclusion: std::pin::Pin<Box<pdftoepdf::PdfInclusionSession>>,
    control: std::pin::Pin<Box<control::ControlSession>>,
    ipc: std::pin::Pin<Box<ipc::IpcSession>>,
    arena: std::pin::Pin<Box<allocation::AllocationArena>>,
    arguments: invocation::OwnedArguments,
}

impl Invocation {
    fn new(arguments: Vec<Vec<u8>>, options: &InvocationOptions) -> Result<Self, InvocationError> {
        use invocation::resource;
        let arguments = invocation::OwnedArguments::new(arguments)?;
        Ok(Self {
            core: invocation_state::InvocationState::new(),
            compression: generated::backend::writezip::CompressionSession::new(),
            streams: stdio::StreamSession::new_with_pipe_policy(options.pipe_process_policy),
            regex: regex_session::RegexSession::new(),
            environment: resource("environment", environment::EnvironmentSession::new())?,
            directory: resource(
                "working directory",
                working_directory::WorkingDirectory::new(options.working_directory.clone()),
            )?,
            time: None,
            runtime: None,
            index: kpathsea::IndexSession::new(),
            lookup: lookup::LookupOwner::new(),
            png: pngshim::PngSession::new(),
            sync: synctex::SyncTexSession::new(),
            xpdf: xpdf::XpdfSession::new(),
            inclusion: pdftoepdf::PdfInclusionSession::new(),
            control: control::ControlSession::new(options.interrupt.clone()),
            ipc: ipc::IpcSession::new(),
            arena: allocation::AllocationArena::new(),
            arguments,
        })
    }

    unsafe fn run(self, options: InvocationOptions) -> Result<EngineCompletion, InvocationError> {
        self.run_with_operation(options, |argc, argv| unsafe {
            (*generated::pdftexextra::state()).synctexoption = ::core::ffi::c_int::MAX;
            generated::pdftexextra::maininit(argc, argv);
            generated::pdftexini::mainbody()
        })
    }

    fn run_with_operation(
        mut self,
        options: InvocationOptions,
        operation: impl FnOnce(std::ffi::c_int, *mut *mut std::ffi::c_char) -> EngineCompletion,
    ) -> Result<EngineCompletion, InvocationError> {
        use invocation::resource;
        let Self {
            core,
            compression,
            streams,
            regex,
            environment,
            directory,
            time,
            runtime,
            index,
            lookup,
            png,
            sync,
            xpdf,
            inclusion,
            control,
            ipc,
            arena,
            arguments,
        } = &mut self;
        let (argc, argv) = arguments.engine_args();
        resource("core state", core.as_mut().scope(|| arena.as_mut().scope(|| {
            resource("environment", environment::EnvironmentSession::scope(environment.as_mut(), || {
                for (name, value) in options.environment {
                    resource("environment override", match value {
                        Some(value) => environment::set(&name, &value),
                        None => environment::remove(&name),
                    })?;
                }
                resource("working directory", directory.as_mut().scope(|| {
                    *runtime = Some(runtime::RuntimeSession::new());
                    *time = Some(resource("time", time::TimeSession::new())?);
                    resource("time", time::TimeSession::scope(time.as_mut().unwrap().as_mut(), || {
                        resource("interrupt", control::ControlSession::scope(control.as_mut(), || {
                            resource("IPC", ipc.as_mut().scope(|| {
                                resource("PDF compression", generated::backend::writezip::CompressionSession::scope(compression.as_mut(), || {
                                    resource("regex", regex_session::RegexSession::scope(regex.as_mut(), || {
                                        runtime::RuntimeSession::scope(runtime.as_mut().unwrap().as_mut(), || {
                                            kpathsea::IndexSession::scope(index.as_mut(), || {
                                                lookup::LookupOwner::scope(lookup.as_mut(), || {
                                                    pngshim::PngSession::scope(png.as_mut(), || {
                                                        synctex::SyncTexSession::scope(sync.as_mut(), || {
                                                            xpdf::XpdfSession::scope(xpdf.as_mut(), || {
                                                                pdftoepdf::PdfInclusionSession::scope(inclusion.as_mut(), || {
                                                                    let completion = streams.as_mut().scope(|| {
                                                                        let completion = termination::catch(|| operation(argc, argv)).unwrap_or_else(|completion| completion);
                                                                        unsafe { stdio::flush_for_exit() };
                                                                        completion
                                                                    });
                                                                    streams.as_mut().finish().map_err(|error| InvocationError::StreamTeardown { message: error.to_string() })?;
                                                                    Ok(completion)
                                                                })
                                                            })
                                                        })
                                                    })
                                                })
                                            })
                                        })
                                    }))?
                                }))?
                            }))?
                        }))?
                    }))?
                }))?
            }))?
        })))?
    }
}

#[cfg(test)]
mod entry_tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn nested_public_entry_is_rejected_without_replacing_the_active_core() {
        invocation_state::with_fixture(|| {
            let before = unsafe { invocation_state::current() };
            assert_eq!(
                unsafe { run_from_args(vec![b"pdflatex".to_vec(), b"--help".to_vec()]) },
                Err(InvocationError::NestedInvocation)
            );
            assert_eq!(unsafe { invocation_state::current() }, before);
        });
        assert!(!invocation_state::is_active());
    }

    #[test]
    fn unexpected_panic_tears_down_the_complete_invocation_and_preserves_payload() {
        let options = InvocationOptions::default();
        let invocation = Invocation::new(vec![b"pdflatex".to_vec()], &options).unwrap();
        let payload = catch_unwind(AssertUnwindSafe(|| {
            invocation.run_with_operation(options, |argc, argv| unsafe {
                generated::pdftexextra::maininit(argc, argv);
                assert!(!allocation::malloc(73).is_null());
                let stream = stdio::tmpfile();
                assert!(!stream.is_null());
                assert_eq!(stdio::fwrite(c"unclosed".as_ptr().cast(), 1, 8, stream), 8);
                (*generated::pdftexextra::state()).history = 3;
                std::panic::resume_unwind(Box::new(0x51a7_u64));
            })
        }))
        .unwrap_err();
        assert_eq!(*payload.downcast::<u64>().unwrap(), 0x51a7);
        assert!(!invocation_state::is_active());
        let options = InvocationOptions::default();
        let next = Invocation::new(vec![b"pdflatex".to_vec()], &options).unwrap();
        assert_eq!(
            next.run_with_operation(options, |_, _| {
                assert_eq!(unsafe { (*generated::pdftexextra::state()).history }, 0);
                assert!(!stdio::stdout().is_null());
                EngineCompletion::Clean
            })
            .unwrap(),
            EngineCompletion::Clean
        );
    }
}
