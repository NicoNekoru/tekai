# tekai-pdftex

`tekai-pdftex` is the experimental native TeX/STY-to-PDF replacement-engine
track for the `tekai` workspace.

It is intentionally separate from [`tekai-engine`](../tekai-engine/README.md):

- `tekai-engine` preserves pdfTeX's algorithms and powers the default exact
  `--engine tekai-engine --runner direct` path;
- `tekai-pdftex` explores a faster fused expansion, in-memory document,
  layout, asset, and PDF architecture.

## CLI modes

```sh
cargo run -- build path/to/main.tex --engine tekai-pdftex
cargo run -- build path/to/main.tex --engine tekai-pdftex-certified
```

`tekai-pdftex` runs the native renderer for its supported subset. Supported
native output is approximate and must not be described as general pdfTeX
parity. Unsupported documents fall back to the exact embedded pdfTeX path.

`tekai-pdftex-certified` runs the native backend for trace/coverage evidence,
then produces the delivered PDF with the exact pdfTeX engine. Its fidelity comes
from that final oracle artifact, not from certifying the approximate native
rendering.

## Correctness boundary

The target is a high-performance TeX/STY-to-PDF engine, not a preview-only
renderer or a source-level LaTeX parser. Features belong on the native hot path
when they improve executed TeX semantics or visible PDF fidelity. Reproducing
legacy `.aux`, `.log`, `.fls`, bibliography, or SyncTeX file shape is optional
unless it affects the delivered PDF or required interoperability.

The native renderer already supports the two checked-in large papers as a
useful development corpus, but it still substitutes simplified font, math,
paragraph, page-building, float, environment, and bibliography behavior. The
systematic differences are documented in the
[native divergence audit](../../output/pdf/pdftex-native-divergence-audit.md).

## Architecture

[`ARCHITECTURE.md`](ARCHITECTURE.md) specifies the proposed engine-v2 dataflow:

1. memory-mapped format snapshot and indexed distribution/project resolver;
2. fused token executor with compact token frames;
3. typed in-memory reference, bibliography, index, and document state;
4. fidelity paragraph/page builders;
5. parallel asset and PDF-object pipelines;
6. fixed-DPI rendered parity gates with exact pdfTeX as verifier and fallback.

The `engine` module currently exposes report-only counters and stage boundaries
for evaluating that design without claiming it is production typesetting.

## Development

```sh
cargo test -p tekai-pdftex
cargo test --test compiler_tekai_pdftex
cargo clippy -p tekai-pdftex --all-targets --locked -- -D warnings
```

Any change presented as a fidelity improvement should include fixed-DPI
rendered comparison against the exact engine. Any change presented as a
performance improvement should include matched release-mode timings on the
large examples and name the unsupported/fallback boundary.
