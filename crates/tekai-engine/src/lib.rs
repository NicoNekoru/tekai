//! Self-contained exact typesetting engine for Tekai.
//!
//! The CLI and standalone runner link this crate as a Rust library. The
//! initialization functions and shared core state use checked Rust module
//! interfaces. Legacy backend connections still use web2c calling conventions.
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
mod cache;
mod core_types;
mod directory_graph;
mod embedded;
mod engine_options;
pub mod file_identity;
mod float_parse;
mod integer_parse;
mod invocation;
mod jpeg_exif;
pub mod kpathsea;
pub mod lookup;
pub mod md5;
pub mod openclose;
pub mod pdftoepdf;
pub mod pngshim;
pub mod runtime;
pub mod search;
mod stdio;
pub mod support;
pub mod synctex;
mod ttf_name;
mod ttf_post;
pub mod utils;
pub mod web2c;
pub mod xpdf;

pub use invocation::{EngineCompletion, InvocationError};
pub use libz_rs_sys as zlib;

/// Enter the process-isolated engine with owned argument bytes, including argv0.
///
/// This entry accepts one invocation per process. Normal completion returns a
/// typed result after flushing all engine streams. The accepted argument storage
/// remains alive for generated globals that borrow it. Other engine allocations
/// and process state are not torn down, and fatal paths still exit the process.
///
/// # Safety
///
/// The caller must exclude concurrent access to engine globals and run in a
/// dedicated CLI process. Initialization changes environment and signal state.
/// This entry does not make the generated or backend interfaces safe to call
/// concurrently, and callers must not catch engine panics across extern C calls.
pub unsafe fn run_from_args(arguments: Vec<Vec<u8>>) -> Result<EngineCompletion, InvocationError> {
    // Claim permanent entry and retain all argument backing before any reset
    // or initialization can mutate the process-global engine state.
    let (argc, argv) = unsafe { invocation::retain_for_entry(arguments)? };
    lookup::reset();
    pngshim::reset_decode_cache();
    let completion = unsafe {
        generated::pdftexextra::synctexoption = ::core::ffi::c_int::MAX;
        synctex::reset_for_run();
        generated::pdftexextra::maininit(argc, argv);
        generated::pdftexini::mainbody()
    };
    // Preserve the old uexit path's fflush(NULL) behavior. Recorder and other
    // still-open streams can have buffered output after normal finalization.
    unsafe { stdio::flush_all() };
    Ok(completion)
}
