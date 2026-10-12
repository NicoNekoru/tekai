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
mod core_types;
mod directory_graph;
mod embedded;
mod engine_options;
mod environment;
pub mod file_identity;
mod float_parse;
mod integer_parse;
mod invocation;
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
mod ttf_name;
mod ttf_post;
pub mod utils;
pub mod web2c;
pub mod xpdf;

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
    // Claim permanent entry and retain all argument backing before any reset
    // or initialization can mutate the process-global engine state.
    let (argc, argv) = unsafe { invocation::retain_for_entry(arguments)? };
    let mut compression = generated::backend::writezip::CompressionSession::new();
    let _compression_scope = generated::backend::writezip::CompressionSession::scope(
        compression.as_mut(),
    )
    .map_err(|error| InvocationError::ResourceSetup {
        resource: "PDF compression",
        message: error.to_string(),
    })?;
    let mut streams = stdio::StreamSession::new();
    let mut regex = regex_session::RegexSession::new();
    let mut environment =
        environment::EnvironmentSession::new().map_err(|error| InvocationError::ResourceSetup {
            resource: "environment",
            message: error.to_string(),
        })?;
    let result = environment::EnvironmentSession::scope(environment.as_mut(), || {
        termination::catch(|| {
            regex_session::RegexSession::scope(regex.as_mut(), || {
                streams.as_mut().scope(|| {
                    lookup::reset();
                    pngshim::reset_decode_cache();
                    let completion = unsafe {
                        generated::pdftexextra::synctexoption = ::core::ffi::c_int::MAX;
                        synctex::reset_for_run();
                        generated::pdftexextra::maininit(argc, argv);
                        generated::pdftexini::mainbody()
                    };
                    // Recorder and other streams can retain buffered output after normal
                    // finalization. Keep them alive until the generated calls have returned.
                    unsafe { stdio::flush_all() };
                    completion
                })
            })
        })
    })
    .map_err(|error| InvocationError::ResourceSetup {
        resource: "environment",
        message: error.to_string(),
    })?;
    let completion = match result {
        Ok(result) => result.map_err(|error| InvocationError::ResourceSetup {
            resource: "regex",
            message: error.to_string(),
        })?,
        Err(completion) => completion,
    };
    streams
        .as_mut()
        .finish()
        .map_err(|error| InvocationError::StreamTeardown {
            message: error.to_string(),
        })?;
    Ok(completion)
}
