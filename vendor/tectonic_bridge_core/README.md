# Rust-only BibTeX bridge

This package adapts `tectonic_bridge_core` 0.5.3 from the Tectonic Project.
The upstream crate records revision `8c0126a9653239a2e6e0a5274af9b8510f643030`
and source directory `crates/bridge_core` in `.cargo_vcs_info.json`.
The original source is available in
[Tectonic's repository](https://github.com/tectonic-typesetting/tectonic/tree/8c0126a9653239a2e6e0a5274af9b8510f643030/crates/bridge_core).
The [upstream MIT license](../LICENSE.tectonic) and copyright notices apply.

Tekai uses Tectonic's Rust BibTeX engine. That engine calls the bridge's Rust
methods directly and does not enter Tectonic's C callbacks. This adaptation
keeps the input/output handles, file-format extension lookup and driver APIs
that BibTeX needs. It removes the native support archive, C callback exports,
global raw state pointer and setjmp/longjmp error reporting. Callback errors
retain their Rust error chains. Each callback gets fresh handles and the
upstream engine lock still serializes invocations.

Compression uses flate2's Rust backend. This package forbids unsafe Rust.
It intentionally does not support Tectonic's C/C++ engines or their callback
API. The registry BibTeX crate remains unchanged through a Cargo patch.

Update this adaptation by comparing the Rust BibTeX caller's required APIs
with the pinned upstream version. Keep copyright notices and record the new
crate versions and revision. Test callback failures, extension lookup and
BibTeX outcomes, then run the workspace correctness and rendered-PDF gates.
