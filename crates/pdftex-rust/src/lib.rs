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
    unused_variables
)]
#![allow(clippy::all)]

pub mod generated {
    pub mod pdftex0;
    pub mod pdftex_pool;
    pub mod pdftexini;
}

pub mod md5;
pub mod openclose;
pub mod support;
pub mod synctex;
pub mod web2c;
