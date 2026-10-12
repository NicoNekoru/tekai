//! Self-contained exact typesetting engine for Tekai.
//!
//! The CLI and standalone runner link this crate as a Rust library. The
//! core and backend calls use checked Rust module interfaces. Remaining global
//! data aliases still require a process-isolated invocation.
//! This crate does not produce a C-linkable archive or export a C entrypoint.

#![allow(
    clashing_extern_declarations,
    dead_code,
    improper_ctypes,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    overflowing_literals,
    path_statements,
    static_mut_refs,
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

pub mod generated {
    pub mod backend {
        pub mod avl;
        pub mod avlstuff;
        pub mod epdf;
        pub mod mapfile;
        pub mod pkin;
        pub mod subfont;
        pub mod tounicode;
        pub mod vfpacket;
        pub mod writeenc;
        pub mod writefont;
        pub mod writeimg;
        pub mod writejbig2;
        pub mod writejpg;
        pub mod writepng;
        pub mod writet1;
        pub mod writet3;
        pub mod writettf;
        pub mod writezip;
    }
    pub mod pdftex0;
    pub mod pdftex_pool;
    pub mod pdftexextra;
    pub mod pdftexini;
}

mod allocation;
mod backend_grammar;
mod backend_parse;
mod backend_types;
mod byte_string;
mod cache;
mod checked_width;
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
mod ipc;
mod jpeg_exif;
pub mod kpathsea;
mod kpathsea_types;
pub mod lookup;
pub mod md5;
pub mod openclose;
pub mod pdftoepdf;
pub mod pngshim;
mod posix_regex;
mod regex_session;
pub mod runtime;
pub mod search;
mod stdio;
pub mod support;
pub mod synctex;
mod termination;
mod time;
mod ttf_name;
mod ttf_post;
pub mod utils;
pub mod web2c;
mod working_directory;
pub mod xpdf;

pub use control::InterruptToken;
pub use invocation::{EngineCompletion, InvocationError};
#[cfg(test)]
use libz_rs_sys as zlib;

/// Enter the process-isolated engine with owned argument bytes, including argv0.
///
/// This entry accepts one invocation per process. Normal completion returns a
/// typed result after closing invocation-owned engine streams. The argument storage
/// remains alive for generated globals that borrow it. Streams, compression,
/// regex results and environment overrides have invocation owners. Other engine
/// allocations are not yet torn down. Fatal and help/version exits unwind to
/// this boundary without terminating the host. Unexpected panics propagate.
///
/// # Safety
///
/// The caller must exclude concurrent access to engine globals and run in a
/// dedicated CLI process. Initialization does not mutate host environment or
/// signal handlers.
/// This entry does not make the generated or backend interfaces safe to call
/// concurrently or allow a second invocation to reuse the legacy global state.
pub unsafe fn run_from_args(arguments: Vec<Vec<u8>>) -> Result<EngineCompletion, InvocationError> {
    use invocation::resource;
    // The permanent latch remains until the core's globals have been moved.
    let (argc, argv) = unsafe { invocation::retain_for_entry(arguments)? };
    let mut compression = generated::backend::writezip::CompressionSession::new();
    let mut streams = stdio::StreamSession::new();
    let mut regex = regex_session::RegexSession::new();
    let mut environment = resource("environment", environment::EnvironmentSession::new())?;
    let mut directory = resource(
        "working directory",
        working_directory::WorkingDirectory::new(None),
    )?;
    let mut runtime = runtime::RuntimeSession::new();
    let mut index = kpathsea::IndexSession::new();
    let mut lookup = lookup::LookupOwner::new();
    let mut png = pngshim::PngSession::new();
    let mut sync = synctex::SyncTexSession::new();
    let mut xpdf = xpdf::XpdfSession::new();
    let mut inclusion = pdftoepdf::PdfInclusionSession::new();
    let mut control = control::ControlSession::new(InterruptToken::default());
    let mut ipc = ipc::IpcSession::new();
    let result = resource(
        "environment",
        environment::EnvironmentSession::scope(environment.as_mut(), || {
            resource("working directory", directory.as_mut().scope(|| {
            let mut time = resource("time", time::TimeSession::new())?;
            resource("time", time::TimeSession::scope(time.as_mut(), || {
                resource("interrupt", control::ControlSession::scope(control.as_mut(), || {
                    resource("IPC", ipc.as_mut().scope(|| {
                        resource("PDF compression", generated::backend::writezip::CompressionSession::scope(compression.as_mut(), || {
                            resource("regex", regex_session::RegexSession::scope(regex.as_mut(), || {
                                runtime::RuntimeSession::scope(runtime.as_mut(), || {
                                    kpathsea::IndexSession::scope(index.as_mut(), || {
                                        lookup::LookupOwner::scope(lookup.as_mut(), || {
                                            pngshim::PngSession::scope(png.as_mut(), || {
                                                synctex::SyncTexSession::scope(sync.as_mut(), || {
                                                    xpdf::XpdfSession::scope(xpdf.as_mut(), || {
                                                        pdftoepdf::PdfInclusionSession::scope(inclusion.as_mut(), || {
                                                            streams.as_mut().scope(|| {
                                                                let completion = termination::catch(|| unsafe {
                                                                    generated::pdftexextra::synctexoption = ::core::ffi::c_int::MAX;
                                                                    generated::pdftexextra::maininit(argc, argv);
                                                                    generated::pdftexini::mainbody()
                                                                }).unwrap_or_else(|completion| completion);
                                                                unsafe { stdio::flush_all() };
                                                                Ok::<_, InvocationError>(completion)
                                                            })
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
        }),
    )?;
    let completion = result?;
    streams
        .as_mut()
        .finish()
        .map_err(|error| InvocationError::StreamTeardown {
            message: error.to_string(),
        })?;
    Ok(completion)
}
