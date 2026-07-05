# pdftex-rust

This crate is the generated Rust port of the pdfTeX web2c core. It replaces the
three generated C objects in the TeX Live pdfTeX link:

- `pdftex-pdftexini.o`
- `pdftex-pdftex0.o`
- `pdftex-pdftex-pool.o`

The Rust archive keeps the C ABI (`mainbody`, `maincontrol`,
`loadpoolstrings`, and the other web2c symbols), so it can be linked against the
existing TeX Live support layer while those libraries are ported or replaced.
It also owns the SyncTeX ABI as no-op Rust exports; SyncTeX is sidecar
compatibility, not final PDF semantics, so the fast path does not link the C
sidecar writer.
The small `libmd5` support ABI is implemented in Rust as well, so the link no
longer needs TeX Live's `libmd5.a`.
Small scalar web2c support shims such as `zround`, `uexit`, and Pascal `eof`
are also Rust-owned now; the larger file-search/open/configuration layer still
comes from TeX Live while it is being replaced.

The reproducible path is:

```sh
scripts/pdftex_port.py smoke
```

That command builds TeX Live's pdfTeX from `third_party/texlive-source`, builds
this Rust archive, links `pdftex-rust-full`, and verifies a deterministic
INITEX fixture against canonical C pdfTeX by byte-comparing the PDF and log. It
also runs a `-synctex=1` fixture to prove the Rust no-op SyncTeX boundary keeps
the final PDF byte-identical while omitting the sidecar.

To regenerate the Rust source from the TeX Live submodule:

```sh
scripts/pdftex_port.py transpile --write-crate
```

This requires `c2rust` built against a compatible LLVM, currently LLVM 16.
