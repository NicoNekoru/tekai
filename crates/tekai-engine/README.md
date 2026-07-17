# tekai-engine

`tekai-engine` is the self-contained exact typesetting engine embedded in the
`tekai` CLI. It owns the TeX execution core, format loading, file discovery,
font handling, PDF inclusion and writing, compression, image decoding, and the
runtime interfaces needed by the default `tekai-engine` build path.

The engine is built entirely by Cargo and does not link a system TeX engine,
Kpathsea, zlib, libpng, or PDF library. An installed TeX distribution remains a
data source for LaTeX packages, fonts, maps, encodings, and filename databases.

The default CLI invokes this engine in process through an internal entrypoint.
For focused debugging, build its standalone executable with:

```sh
cargo build --release \
  -p tekai-engine \
  --bin tekai-engine \
  --no-default-features \
  --features standalone-binary
```

Validation is behavior-first:

```sh
cargo test -p tekai-engine
cargo test --workspace
```

Changes that can affect layout or PDF output must also pass the repository's
fixed-DPI rendered-page comparison on both large paper fixtures. See
[`docs/development.md`](../../docs/development.md).
