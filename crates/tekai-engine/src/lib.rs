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
mod cache;
mod core_types;
mod directory_graph;
mod embedded;
pub mod file_identity;
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
pub mod utils;
pub mod web2c;
pub mod xpdf;

pub use libz_rs_sys as zlib;

pub unsafe fn run_from_c_args(
    argc: ::core::ffi::c_int,
    argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    lookup::reset();
    pngshim::reset_decode_cache();
    unsafe {
        generated::pdftexextra::synctexoption = ::core::ffi::c_int::MAX;
        synctex::reset_for_run();
        generated::pdftexextra::maininit(argc, argv);
        generated::pdftexini::mainbody();
    }
    libc::EXIT_SUCCESS
}
