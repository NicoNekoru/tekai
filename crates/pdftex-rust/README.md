# pdftex-rust

This crate is the Rust-owned pdfTeX engine used by the workspace. It contains
the generated Rust port of the pdfTeX web2c core plus Rust replacements for the
runtime boundaries that previously pulled in native TeX Live support code. The
checked-in Rust sources replace the generated C entry/wrapper object and the
generated TeX core objects from the historical TeX Live pdfTeX link:

- `pdftex-pdftexextra.o`
- `pdftex-pdftexini.o`
- `pdftex-pdftex0.o`
- `pdftex-pdftex-pool.o`

The Rust archive keeps the C ABI (`main`, `maininit`, `mainbody`,
`maincontrol`, `loadpoolstrings`, and the other web2c symbols) for the generated
core boundary, but the shipped executable is built by Cargo and does not require
the TeX Live source tree.
It also owns the SyncTeX ABI as no-op Rust exports; SyncTeX is sidecar
compatibility, not final PDF semantics, so the fast path does not link the C
sidecar writer.
The small `libmd5` support ABI is implemented in Rust as well, so the link no
longer needs TeX Live's `libmd5.a`.
The web2c support archive `lib/lib.a` is no longer linked: file open/search
adapters, recorder hooks, configuration bounds, allocation helpers, version/help
printing, `zround`, `uexit`, Pascal `eof`, and small string/input helpers are
Rust-owned now.
The native kpathsea archive is no longer linked either: program setup,
environment/config lookup, path checks, recorder callbacks, and `kpse_find_file`
are implemented in Rust, with package/font lookup backed by installed TeX Live
`ls-R` databases. The engine also owns the zlib/libpng/PDF inclusion facades and
uses Rust-owned format files instead of loading system `.fmt` binaries.

Build the standalone executable with:

```sh
cargo build --release -p pdftex-rust --bin pdftex-rust --no-default-features --features rust-binary
```

The crate's Rust tests and the workspace integration tests are now the supported
verification path:

```sh
cargo test -p pdftex-rust
cargo test --workspace
```

The old C2Rust regeneration harness and TeX Live source submodule have been
removed from the repository. Future port work should happen in Rust in this
crate rather than by reintroducing checked-in C or script-based build steps.
