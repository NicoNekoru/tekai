# Rust-backed Tectonic I/O

This package adapts `tectonic_io_base` 0.6.0 from the Tectonic Project.
The upstream crate records revision `8c0126a9653239a2e6e0a5274af9b8510f643030`
and source directory `crates/io_base` in `.cargo_vcs_info.json`.
The original source is available in
[Tectonic's repository](https://github.com/tectonic-typesetting/tectonic/tree/8c0126a9653239a2e6e0a5274af9b8510f643030/crates/io_base).
The [upstream MIT license](../LICENSE.tectonic) and copyright notices apply.

The Rust source is unchanged. Its manifest selects flate2's Rust backend
instead of `zlib`, so importing these handles cannot enable native libz-sys
and a C compiler through Cargo feature unification. The remaining libc use
checks platform error-code constants, it does not compile C source.

Keep this package and Tekai's Rust-only bridge on compatible upstream API
versions. Copy upstream source changes, record their exact provenance, keep
the Rust compression backend and run both packages' tests with the workspace.
