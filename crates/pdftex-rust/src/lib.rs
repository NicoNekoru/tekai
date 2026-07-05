//! Generated Rust port of the pdfTeX web2c core.
//!
//! This crate intentionally preserves the web2c ABI. It exports the same
//! symbols as `pdftexini.o`, `pdftex0.o`, and `pdftex-pool.o`, so the archive
//! can replace those generated C objects in TeX Live's normal `pdftex` link
//! while the remaining C/C++ support libraries are ported behind it.

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
    pub mod pdftexextra;
    pub mod pdftex_pool;
    pub mod pdftexini;
}

pub mod md5;
pub mod openclose;
pub mod support;
pub mod synctex;
pub mod web2c;

#[cfg(not(test))]
#[no_mangle]
pub unsafe extern "C" fn main(
    argc: ::core::ffi::c_int,
    argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        generated::pdftexextra::maininit(argc, argv);
        generated::pdftexini::mainbody();
    }
    libc::EXIT_SUCCESS
}
