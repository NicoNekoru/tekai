# texpilot

`texpilot` is a fast Rust CLI for LaTeX projects. It deliberately separates two
jobs:

- faster build orchestration, using a direct `pdflatex` runner by default and
  keeping `latexmk` as an optional compatibility baseline;
- fast opinionated linting, implemented as a single-pass scanner over TeX-like
  sources.

## Why not a traditional pdflatex clone here?

Functional `pdflatex` replacement is much larger than parsing LaTeX syntax. A
new engine has to understand TeX tokenization with mutable category codes,
macro expansion, assignments, grouping, registers, file I/O, output routines,
hyphenation, line breaking, page breaking, fonts, package behavior, and the PDF
primitive surface. The compatibility test is the rendered PDF: for the same
TeX/STY inputs, the native backend should produce a near-identical PDF quickly.
Legacy artifacts such as `.aux`, `.fls`, `.log`, `.bbl`, `.idx`, and SyncTeX
are useful adapters or diagnostics, but they are not the hot-path contract.

The practical compatibility boundary is:

- use `pdflatex`/XeLaTeX/LuaLaTeX for the stable default engine path today;
- replace `latexmk`'s orchestration for `.aux`, `.bbl`, `.idx`, `.ind`,
  BibTeX/Biber, MakeIndex, and rerun detection where doing so is faster;
- use Rust for orchestration, linting, dependency filtering, and eventually a
  partial TeX tokenizer where fast local analysis matters;
- build the new engine as an explicit engine track whose correctness contract is
  near-identical rendered PDF output, not exact sidecar/log compatibility.

That engine track now lives in [`crates/texpilot-pdftex`](crates/texpilot-pdftex).
It documents the current pdfTeX boundary, the capabilities a compatible rewrite
needs, and the performance strategy required to make clean final builds faster
than the external `pdflatex` lower bound. The default runner still calls real TeX
engines until that backend can pass rendered-PDF equivalence and performance
gates on the target corpus.

## Are separate full parsing steps necessary?

No, not for TeX compatibility. TeX is fundamentally expansion-driven: category
codes can change how later bytes are tokenized, and macros can synthesize syntax.
A rigid `parse -> analyze -> render` pipeline will be wrong for real documents.
The efficient architecture is streaming and incremental:

1. scan bytes into TeX tokens under the active category-code table;
2. expand macros only as much as the consumer needs;
3. maintain semantic state for lint/build dependencies;
4. hand true typesetting to a TeX engine unless implementing the engine itself.

The linter in this repo follows that philosophy: it scans only enough structure
to catch useful issues quickly.

## Language choice

Rust is the best default here: native speed, predictable memory use, good CLI and
watcher libraries, safe concurrency, easy static binaries, and enough control to
write a real tokenizer later. C++ is viable for a full engine but more expensive
to maintain safely. Go is excellent for tooling but weaker for parser/tokenizer
micro-control. Zig is interesting, but the ecosystem is thinner.

## Usage

Build a document:

```sh
cargo run -- build path/to/main.tex
```

Emit machine-readable pass/cache counts for editor integrations and benchmarks:

```sh
cargo run -- build path/to/main.tex --report-json
cargo run -- check path/to/main.tex --report-json --allow-warnings
cargo run -- lint path/to/paper --report-json --allow-warnings
```

The JSON report includes elapsed time, PDF path, total plus draft/final-layout
and PDF-producing TeX-pass counts, per-pass elapsed/TeX/aux timing, convergence
diagnostics, BibTeX/index/external-tool pass counts, cache-hit status, and
whether a draft prepass or auxiliary-tool preflight was used. In
`check --report-json`, lint diagnostics are written to stderr so stdout remains
parseable JSON when lint passes. In `lint --report-json`, stdout contains a
diagnostic array plus warning/error counts.

Use the old `latexmk` backend for comparison:

```sh
cargo run -- build path/to/main.tex --runner latexmk
```

Remove the configured output directory:

```sh
cargo run -- clean
cargo run -- clean --dry-run --report-json
```

Project build defaults can live in `texpilot.toml`, so a paper can check in the
direct-runner policy it expects:

```toml
[build]
engine = "pdflatex"
runner = "direct"
bib = "auto"
out_dir = "build"
draft_prepass = "auto"
max_runs = 8
fast = false
once = false
synctex = false
shell_escape = false

[build.env]
TEXINPUTS = "tex//:"
BIBINPUTS = "bib//:"
```

`build`, `check`, and `watch` read `[build]` from the default
`texpilot.toml`; pass `--config path/to/texpilot.toml` to use another file.
Explicit CLI flags override config defaults, which makes checked-in fast-preview
or direct-runner defaults easy to keep while still allowing one-off benchmark
commands. `[build.env]` values are applied before TeX and auxiliary tools run,
so project-local Kpathsea search paths such as `TEXINPUTS`, `BIBINPUTS`,
`BSTINPUTS`, `INDEXSTYLE`, and `TEXINDEXSTYLE` can be checked in instead of
hidden in a shell wrapper. The experimental native `texpilot-pdftex` engine
resolves project-local and explicit `TEXINPUTS`/`BIBINPUTS`/`BSTINPUTS` roots
in process before using external lookup, keeping common project discovery on
the PDF-producing hot path rather than in a subprocess.

Build with XeLaTeX:

```sh
cargo run -- build path/to/main.tex --engine xe-latex
```

Override TeX's output job name, matching the `pdflatex -jobname=...` convention:

```sh
cargo run -- build path/to/main.tex --job-name paper-final
```

Custom job names must be single filename components, since TeX aux files,
generated PDFs, and `texpilot` cache files are all keyed from that name.

Fast compile with graphic inclusion, TikZ externalization, and `minted`
highlighting disabled:

```sh
cargo run -- build path/to/main.tex --fast
cargo run -- build path/to/main.tex --no-images
```

`--fast` and its explicit alias `--no-images` use `graphicx` demo placeholders
rather than ordinary draft mode, patch the `svg` package's `\includesvg`
command and the `pdfpages` package's `\includepdf` commands to lightweight
placeholders, temporarily disable TikZ externalization during the preview pass,
and ask `minted` to typeset without Pygments highlighting while replacing
`\inputminted` source-file imports with placeholders. The `animate` package's
`\animategraphics` frame sequence imports and `standalone` package
`\includestandalone` subdocuments are also replaced in preview mode, along with
`attachfile`/`attachfile2` PDF attachments. They avoid file lookup for missing
or not-yet-generated images, embedded PDFs, code snippets, animation frames,
standalone figure documents, `media9` multimedia files, and attachment
payloads, skip shell-escaped TikZ figure jobs and syntax-highlighting helper
work, and still win when the document explicitly loads `graphicx` with `final`.

By default, direct final builds use `--draft-prepass auto`: if the source scan
sees multipass signals such as citations, references, or auxiliary tables,
TeX convergence can run without producing throwaway PDFs before the final
PDF-producing pass. Graphics-using multipass documents, including
back-reference documents that update `.brf` data, first stay in `graphicx`
draft/no-image mode until auxiliary files settle; text-only multipass documents
use full-layout no-PDF convergence directly. During non-final passes,
`pdflatex`/LuaLaTeX use engine `-draftmode` and XeLaTeX uses `-no-pdf`, so
convergence work still writes aux/log files but avoids image inclusion when
applicable and avoids throwaway PDFs. After no-PDF passes have stable auxiliary
and generated-file inputs, `texpilot` promotes the next likely settling pass to
PDF output instead of paying for another no-PDF pass whose only result would be
discovering that the document has settled. Once a compatible settled build
state exists, auto mode skips draft work on incremental rebuilds so text and
bibliography edits can keep the one-pass direct-cache path. Force or disable
the auto policy with:

```sh
cargo run -- build path/to/main.tex --draft-prepass always
cargo run -- build path/to/main.tex --draft-prepass never
```

One-pass edit preview, skipping bibliography and rerun convergence:

```sh
cargo run -- build path/to/main.tex --once
```

For final direct builds, `--max-runs` is a settlement limit, not a preview mode:
if the document still needs another TeX or auxiliary-tool pass after that many
runs, `texpilot` exits with an error and does not write a successful cache
state. Use `--once` when an intentionally incomplete preview is desired.

Fastest preview mode, one pass with images disabled:

```sh
cargo run -- build path/to/main.tex --once --fast
```

Experimental native pdfTeX rewrite path:

```sh
cargo run -- build path/to/main.tex --engine texpilot-pdftex
```

This engine first tries the in-process `crates/texpilot-pdftex` backend, which
currently supports a pragmatic paper-rendering subset with plain text,
sectioning, recursive local `\input`, `\include`, and `\includeonly`, focused
`\newcommand`/`\renewcommand`
`\providecommand`/`\DeclareRobustCommand`, `\DeclareMathOperator`, `\def`,
`\gdef`, `\edef`/`\xdef`, and `\protected@edef` macro expansion, including
positional arguments for common paper macros, permissive inline math segment
cleanup, common math wrapper/operator/script normalization in rendered equations,
including compact fraction/root/accent atoms, layout/style declaration cleanup,
relation/operator spacing, labeled arrow normalization, derivative/gradient and
set-operator cleanup, transpose cleanup, and under/over brace annotation cleanup,
native PDF Symbol-font rendering for common Greek, relation/operator, arrow,
perpendicular, partial, gradient, square-root, and plus-minus math glyphs plus
minus-plus ASCII fallback in body text, captions, equations, pseudocode, and tables,
package declarations with local `.sty` discovery, input tracking, and
native-safe adapters for selected document-command packages,
bibliography loading with numeric BibTeX/BibLaTeX citation rendering,
natbib-style optional citation notes, `plainnat` author-year citation labels,
and basic BibTeX brace/accent/text-command cleanup,
standard `\title`/`\author`/`\date` maketitle text, section/equation
`\label`/`\ref`/`\pageref` resolution, run-in `\paragraph` heading rendering,
native `\footnote`/`\thanks` compact numeric markers with rendered notes, one-pass
`\tableofcontents` rendering with `\addcontentsline` TOC entries, numbered
appendix sectioning, numbered `equation`, `align`, and `multline`
displays, lightweight raw `$$...$$` display-math blocks, approximate
table/list/algorithm/listing wrappers, grouped `lstfloat` blocks,
itemize/enumerate/description markers,
numbered captions and `\captionof`, with conservative wrapping for extreme
caption overflows,
one-pass `\listoffigures`/`\listoftables` rendering,
transparent color/style text wrappers with visible `\textsc` small-caps text,
common hyperref link-text commands with native PDF outlines,
common `\pdfinfo{...}`/`\hypersetup{pdf...=...}` metadata as PDF `/Info`
entries plus safe no-op PDF catalog/map declarations,
theorem-like tcolorbox/amsthm headings with native counters and labels, plus
colon-style titled graphical theorem boxes for image-heavy definition panels,
including theorem boxes nested inside wide figure floats,
JPEG/PNG `\includegraphics` as real PDF image XObjects, including PNG alpha
soft masks, first-page PDF `\includegraphics` as Form XObjects with imported
resources and `page=N` selection, common `graphicx` sizing options (`width`,
`height`, `scale`, `keepaspectratio`, `angle`, `\linewidth`, `\textwidth`, and
`\columnwidth`) plus PDF-visible `trim`/`viewport` boxes with `clip`,
style-aware built-in PDF base-font selection for the current large
example profiles, caption-sized figure/table/listing captions with
source-derived `\lstlistingname` for listing captions and single listing
references, single `\cref` prefixes for section/figure/table/theorem/listing
labels, plus local style `labelsep=period` caption punctuation, other resolved
graphics as tracked placeholders, centered native title/author/abstract lines,
full-width ICML-style title panels with shaded abstract boxes and native teaser
image/table rows, single-column graphic figure row packing, deferred leading
wide top-float placement, nested inline layout-scaffold cleanup for
teaser-style minipage/tabular content that reaches text-cleaning paths, typed
in-memory state for labels, citations, contents,
bibliography metadata, and basic
`\makeindex`/`\index`/`\printindex` rendering. The main CLI uses a PDF-first
artifact policy: supported native builds emit the final PDF, a compact native
trace, and SyncTeX only when explicitly requested. The low-level backend can
still emit legacy files such as `.aux`, `.toc`, `.lof`, `.lot`, `.out`, `.brf`,
`.idx`, `.fls`, and `.log` for debugging/interoperability, but those files are
adapters rather than the correctness model. Rendered PDF parity is the
compatibility test for this track; legacy sidecars are optional evidence, not
the contract. The native hot path now computes one in-memory layout pass and
reuses its placements for page count, PDF streams, SyncTeX boxes, and trace
metrics, so the output-routine facts we optimize are the same facts used to
write the PDF. Explicit `\openout`/`\write` streams are stripped from the
PDF-only hot path and only materialized by the lower-level legacy-artifact mode.
Caption placement is traced from rendered layout placements so float/page-builder
debugging uses the native output routine's actual decisions. The native trace
also counts two-column graphic float bodies that still fall through the
approximate path via `layout_two_column_graphic_float_fallbacks` and
`layout_two_column_wide_graphic_float_fallbacks`, which keeps the remaining
ICML-style output-routine work framed as TeX/STY-to-PDF functionality rather
than legacy sidecar compatibility. The same trace records shadow native sizing
for those skipped bodies through estimated native-slot aggregates and
per-fallback row/image/caption slot details, so the next float work can target
the output-routine footprint without first changing rendered PDF flow.
Unsupported documents still fall back to real
`pdflatex` instead of silently rendering a wrong PDF. The fallback leaves a
`*.texpilot-pdftex.trace` file in the output directory so the missing native
capability is visible. Requested shell-escape semantics also fall back until the
native engine implements them. On the bundled large examples this native path
now completes as one final native PDF pass without external
`pdflatex`, BibTeX, or draft-prepass convergence; it is a fast functional
renderer moving toward near-identical PDF output. Native `\input`,
file-existence probes, local/search-path class/package/style chains, read
streams, code inputs, graphics, and
bibliography files also use `TEXINPUTS`/`BIBINPUTS` Kpathsea lookup after local
checks, which starts moving the rewrite toward pdfTeX-compatible file-system
semantics without walking standard distribution classes merely to recreate
legacy discovery side effects. Package-option detection also carries simple
`\PassOptionsToPackage{...}{...}` declarations through traversed class/style
loads, including simple `\RequirePackageWithOptions{...}` package loads.
Bibliography style files from `\bibliographystyle{...}` are also recorded from
local paths or `BSTINPUTS`.

The native renderer now runs a focused p1 `ExpansionEngine` pre-expansion pass
with mutable catcodes, `\def`/`\gdef`, `\edef`/`\xdef`, `\protected` definition
prefixes, `\protected@edef`, `\csname`/`\endcsname`, `\expandafter`,
`\futurelet`, `\aftergroup`, `\relax`, `\newcommand`/`\renewcommand`, `\let`,
local brace/`\begingroup` scopes, global assignment forms such as `\global\def`,
token-level conditionals such as `\iftrue`, `\iffalse`, `\ifx`, `\ifdefined`,
`\ifcsname`, `\if`, `\ifcat`, `\unless`, `\ifnum`, `\ifodd`, `\ifcase`, and
`\newif`, raw and aliased count registers
with `\count<number>`, `\countdef`, `\newcount`, assignment, `\advance`,
`\number`, `\numexpr`, and `\the`, dimension registers with `\dimen<number>`,
`\dimendef`, `\newdimen`, `\ifdim`, `\dimexpr`, and common TeX units, integer
skip/glue registers with `\skip<number>`, `\skipdef`, and `\newskip`,
token-list registers with `\toks<number>`, `\toksdef`, and `\newtoks`,
expandable primitives such as `\romannumeral`, `\string`, `\meaning`,
`\detokenize`, `\unexpanded`, `\expanded`, `\noexpand`, `\jobname`,
`\pdfprimitive`, `\ifpdfprimitive`, `\pdfcreationdate`, `\pdffilesize`,
`\pdffilemoddate`, `\pdffiledump`, `\pdfstrcmp`, `\pdfescapehex`, `\pdfunescapehex`,
`\pdfescapestring`, `\pdfescapename`, `\pdfmdfivesum`, and
`\pdfmdfivesum file {...}`, pdfTeX version constants such as
`\pdftexversion` and `\pdftexrevision`,
constants from `\chardef`/`\mathchardef`, and focused LaTeX definition forms such as
`\providecommand`, protected `\DeclareRobustCommand`, and `\DeclareMathOperator`
before document modeling. It also injects a native-safe `pdftexcmds`
adapter for installed-package wrappers around supported pdfTeX primitives. That
file-aware engine is the migration path from string-level rendering toward a
real TeX expansion pipeline.

For large papers with expensive preambles, direct pdfLaTeX builds can also
cache a `mylatexformat` preamble dump:

```sh
cargo run -- build path/to/main.tex --precompile-preamble
cargo run -- build path/to/main.tex --once --fast --precompile-preamble
```

The first run builds the preamble format under the output directory; later
passes reuse it while the recorded preamble inputs remain unchanged. In a full
build this only removes repeated package/class preamble loading; TeX still has
to typeset every page, include images, write/read aux files, and converge
references. In `--fast --once` mode it remains an edit-loop preview contract:
images/highlighting are placeholders and bibliography/rerun convergence is
intentionally skipped. `watch --preview` enables the fast preview cache for
pdfLaTeX automatically and falls back to the normal fast preview path when a
format cannot be built.

On the bundled large examples, the cached preamble preview takes the realistic
sub-second preview path as far as pdfTeX allows: `arXiv-2605.26379v1` drops
below one second end-to-end, while `arXiv-2511.08544v3` remains near the
one-second boundary because the single formatted pdfTeX preview pass is itself
about a second. Full final-quality clean builds are different: on these papers,
one ordinary PDF-producing pdfTeX pass is already multiple seconds. Reaching a
sub-second full build requires a different renderer or reuse model, such as
active-section/page-window builds, PDF page patch/reuse for unaffected pages, or
a persistent TeX process/daemon that keeps macro and font state warm between
edits.

Lint sources:

```sh
cargo run -- lint path/to/paper
```

Run lint and then compile:

```sh
cargo run -- check path/to/main.tex
```

Watch and rebuild:

```sh
cargo run -- watch path/to/main.tex --fast
```

Fast watch preview:

```sh
cargo run -- watch path/to/main.tex --preview
```

Hybrid live preview plus final-quality settle build:

```sh
cargo run -- watch path/to/main.tex --preview --final-after-idle-ms 1500
```

This is closer to live rebuild than browser HMR. PDF viewers such as Skim,
Preview alternatives, or editor-integrated viewers can auto-refresh the emitted
PDF from `build/`.

Watch mode seeds its dependency filter from the direct runner's source scan, so
local and Kpathsea-discovered `\input`/`\include` dependencies such as
`values.dat` can trigger rebuilds immediately even when their extension is not
in the generic fallback watch list. Common BibTeX/BibLaTeX resource declarations
such as `\bibliography{refs}`, `\bibliographystyle{custom}`, and
`\addbibresource[...]{refs.bib}` are seeded the same way, as are explicit local
or `TEXINPUTS`-discovered package/class declarations such as
`\usepackage{localpkg}` and `\documentclass{customclass}`. TeX-like source,
package, and class files inside explicit `TEXINPUTS` roots are followed
recursively during this pre-recorder scan. Source-declared graphics through
`\includegraphics`, `\animategraphics`, `\includesvg`,
`\graphicspath`, and `\svgpath`, plus PDF inclusions through
`\includepdf`/`\includepdfmerge`, are resolved through local paths plus Kpathsea
and seeded before the first recorder file exists. Literal source/listing/data
inputs such as `\inputminted`, `\lstinputlisting`, `\verbatiminput`,
`\pgfplotstableread`, and PGFPlots
`\addplot table {file}` payloads are seeded local-first and through Kpathsea
when they name an explicit file. Common `datatool` loaders such as
`\DTLloaddb{db}{file}` and `csvsimple` readers such as
`\csvreader{file}{...}{...}` are covered the same way. External-reference aux files
from `xr`/`xr-hyper` `\externaldocument{file}` and `zref-xr`
`\zexternaldocument{file}` declarations are seeded before the recorder exists,
including nested aux files reached through their `\@input{...}` lines, alongside
standalone subdocuments, `media9` payloads, and
`attachfile`/`attachfile2` attachments.
After each successful build, watch mode also merges the TeX recorder's stronger
dependency state. If a recorded dependency lives outside the project root, watch
mode subscribes to that dependency's parent directory after the first build.
Editor save bursts are coalesced until relevant filesystem events go quiet, with
a hard cap so noisy filesystems cannot delay rebuilds indefinitely. Watch mode
runs the `--preview` build immediately when requested; with
`--final-after-idle-ms`, it then waits for the file stream to stay quiet for that
duration before running the normal final-quality direct build. In that hybrid
mode, dependency refresh merges whichever recorded state is current for the
preview and final build modes, so a final settle build does not make watch lose
its recorded dependencies. Watch mode runs a full lint pass at startup. After
that, rebuild bursts lint only the changed TeX-like files (`.tex`, `.ltx`,
`.sty`, `.cls`, matched case-insensitively); bibliography, MakeIndex style files
(`.ist`, `.xdy`), project-local Biber config files (`biber.conf`,
`.biber.conf`), image and SVG/SVGZ assets, external-tool source files, and
recorded data-dependency changes skip lint and go straight to the direct build.
Only the configured output directory is ignored; a source directory named
`build` is still watched when it is not the output directory.

## pdflatex parity checks

`texpilot` is not a drop-in replacement TeX engine. It preserves compatibility
by calling real TeX engines, but it now avoids `latexmk` by default. To guard
rendered-output parity, run:

```sh
scripts/verify_pdflatex_parity.sh
```

For a focused check while iterating on the harness, set
`TEXPILOT_PARITY_CASES` to one or more case names:

```sh
TEXPILOT_PARITY_CASES="iclr arxiv-2605" scripts/verify_pdflatex_parity.sh
```

That script downloads the official 2026 ICLR, NeurIPS, and ICML sample paper
zips, also checks a local BibLaTeX/Biber fixture and the included arXiv
examples, builds each document to `pdflatex` convergence plus BibTeX or Biber
when needed, and once with `texpilot build`. It then rasterizes both PDFs with
`pdftoppm` and compares page hashes. The comparison is visual/rendered-page
equality, not byte-for-byte PDF equality, because PDF metadata and object IDs
are not stable across build runners.

For the experimental native `texpilot-pdftex` engine, use the rendered-similarity
harness instead:

```sh
scripts/native_pdftex_parity.py --smoke
scripts/native_pdftex_parity.py --strict examples/arXiv-2605.26379v1/main.tex
scripts/native_pdftex_parity.py --top-pages 3 --write-comparison-pages
scripts/native_pdftex_parity.py --fail-on-warn --require-page-count-match
scripts/native_pdftex_parity.py --max-two-column-graphic-float-fallbacks 0 --max-two-column-wide-graphic-float-fallbacks 0 --max-two-column-graphic-float-fallback-native-slots 0 --max-two-column-wide-graphic-float-fallback-native-slots 0
```

That harness builds a converged external `pdflatex` baseline, builds the native
engine without fallback, rasterizes both PDFs with `pdftoppm`, and reports page
counts, dimensions, exact rendered-page hashes, RMSE, and different-pixel ratios.
Use `--top-pages N` to print the worst rendered pages by RMSE, and
`--write-comparison-pages` to emit side-by-side baseline/native PPMs under each
case workdir for direct visual inspection.
Without thresholds it is a measurement tool; `--strict` or explicit
`--max-mean-rmse`, `--max-page-rmse`, and
`--max-different-pixel-ratio` options turn it into the near-identical PDF gate
for the native rewrite. Use `--fail-on-warn` when an iteration should also fail
warning-class rendered-output regressions, such as page-count or page-dimension
drift, without requiring strict RMSE thresholds yet.
It also reads native trace coverage counters and can gate remaining
two-column graphic float bypasses with
`--max-two-column-graphic-float-fallbacks` and
`--max-two-column-wide-graphic-float-fallbacks`, plus the corresponding
estimated native-slot debt with
`--max-two-column-graphic-float-fallback-native-slots` and
`--max-two-column-wide-graphic-float-fallback-native-slots`, so uncovered
output-routine functionality stays tied to rendered PDF parity rather than
sidecar shape.

The latest focused conference-template run in this workspace passed on June 28,
2026:

| case | source | `texpilot` direct pass summary | rendered parity |
| --- | --- | --- | --- |
| `iclr` | official ICLR 2026 sample paper zip | 3 TeX runs: 2 draft, 1 final-layout, 1 PDF, 1 bibliography | matched |
| `neurips` | official NeurIPS 2026 sample paper zip | 3 TeX runs: 1 draft, 2 final-layout, 1 PDF | matched |
| `icml` | official ICML 2026 sample paper zip | 3 TeX runs: 2 draft, 1 final-layout, 1 PDF, 1 bibliography | matched |

## Release verification

To rerun the full local proof suite before shipping a change:

```sh
scripts/verify_release.sh
```

That runs `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, the
rendered `pdflatex` parity suite, and the `latexmk` performance gate. For a
quick local edit check that skips the expensive paper builds, set
`TEXPILOT_VERIFY_SKIP_PARITY=1` and/or `TEXPILOT_VERIFY_SKIP_PERF=1`.

The checked-in GitHub Actions workflow runs that light verifier on pushes and
pull requests. A manual `workflow_dispatch` input, `full_release=true`, installs
the TeX/PDF toolchain and runs the complete parity plus performance suite. The
full job is intentionally manual because it downloads conference templates and
builds several multi-pass papers.

## Performance comparison

To compare clean-build wall-clock time on the included arXiv paper:

```sh
scripts/benchmark_paper.py --runs 5
```

To compare warmed no-op, metadata-only touch, a trailing comment after
`\end{document}`, an in-document comment before `\end{document}`,
inline-comment padding edits, inline-verb comment edits, unused-bibliography
edits, and cited-bibliography rebuilds:

```sh
scripts/benchmark_paper.py --scenario warm-edits --runs 5
```

`warm` and `warm-edit` are accepted aliases for `warm-edits`.
The clean-build table includes compact pass summaries for `texpilot` rows, for
example `5T+2d/3f/1p+1B` means five TeX passes split as two draft/no-PDF, three
final-layout, and one PDF-producing pass plus one bibliography run. The
`slowest` column identifies the slowest reported TeX pass (`d` draft, `f`
full-layout/no-PDF, `p` PDF-producing). Add `--json` to include raw timings and
per-pass diagnostics. `texpilot` rows also include build reports with cache
status, draft-prepass use, and auxiliary preflight use.
Add `--gate` to turn the comparison into a regression check: the script exits
nonzero when the `texpilot-direct` median exceeds the allowed multiple of the
`latexmk` median. The default threshold is `--max-latexmk-ratio 1.0`, which
requires direct mode to be no slower than `latexmk`, plus a small
`--gate-absolute-tolerance` for noisy timing ties; use a lower ratio such as
`0.75` when you want CI to enforce a minimum speedup margin. In `warm-edits`
mode, the gate is checked independently for each edit type so a no-op/cache
regression cannot hide behind a slower bibliography rebuild.
For a ready-made local or CI check over clean builds and warmed edit loops on
the included larger papers, run:

```sh
scripts/performance_gate.sh
```

Set `TEXPILOT_PERF_RUNS`, `TEXPILOT_MAX_LATEXMK_RATIO`,
`TEXPILOT_GATE_ABSOLUTE_TOLERANCE`, `TEXPILOT_PERF_SCENARIOS`,
`TEXPILOT_PERF_PAPERS`, or `TEXPILOT_BIN` to tune the gate without editing the
script.

The latest full performance gate run in this workspace passed on both included
larger papers:

| paper | `latexmk` clean median | `texpilot` direct clean median | ratio |
| --- | ---: | ---: | ---: |
| `examples/arXiv-2605.26379v1/main.tex` | 14.203s | 6.379s | 0.449x |
| `examples/arXiv-2511.08544v3/main.tex` | 16.188s | 8.320s | 0.514x |

The same gate also passed warmed edit loops independently for no-op touches,
comment-only edits, unused-bibliography edits, and cited-bibliography rebuilds.
Comment/no-op rebuilds stayed on the direct-cache path, while cited-bibliography
edits still rebuilt the affected bibliography and final PDF.

The clean benchmark times fresh source-tree copies for `pdflatex` one-pass,
`pdflatex` three-pass full recipe, `pdflatex` rerun-to-convergence, direct
`latexmk`, optimized `texpilot` direct final builds, conservative
`texpilot --draft-prepass never`, forced `texpilot --draft-prepass always`,
`texpilot --runner latexmk`, direct builds with a precompiled preamble, and
one-pass preview modes. It builds and times
`target/release/texpilot` by
default; pass `--profile debug` or `--texpilot <path>` when you intentionally
want another binary. By default each run uses a fresh `/tmp/texpilot-bench-*`
workspace so separate benchmark invocations can run concurrently; pass
`--workdir <path>` when you want a stable scratch directory that should be
replaced on each run.

On `examples/arXiv-2605.26379v1/main.tex` (three-run local clean medians with
auto draft-prepass default and `target/release/texpilot`, measured June 28,
2026), wall-clock times were:

| command | time |
| --- | ---: |
| `pdflatex` one pass | 2.496s |
| `pdflatex` full recipe | 7.624s |
| `pdflatex` converged | 12.657s |
| `latexmk` | 14.203s |
| `texpilot` direct | 6.379s |
| `texpilot --draft-prepass never` | 13.059s |
| `texpilot --draft-prepass always` | 6.289s |
| `texpilot --runner latexmk` | 14.156s |
| `texpilot --once` | 2.661s |
| `texpilot --once --fast` | 1.040s |

On the larger included paper (`examples/arXiv-2511.08544v3/main.tex`, also
three-run local clean medians), native direct scheduling was the larger win:

| command | time |
| --- | ---: |
| `pdflatex` one pass | 3.575s |
| `pdflatex` full recipe | 11.024s |
| `pdflatex` converged | 11.022s |
| `latexmk` | 16.188s |
| `texpilot` direct | 8.320s |
| `texpilot --draft-prepass never` | 12.242s |
| `texpilot --draft-prepass always` | 8.273s |
| `texpilot --runner latexmk` | 16.211s |
| `texpilot --once` | 4.122s |
| `texpilot --once --fast` | 2.214s |

The `texpilot --runner latexmk` rows intentionally track raw `latexmk`: that
mode is a compatibility baseline, not the optimization path. The speedup comes
from the native direct runner owning TeX/BibTeX/rerun scheduling.

The experimental `--engine texpilot-pdftex` path is a separate native-renderer
track rather than a `pdflatex` scheduler optimization. In the latest release
gate on this workspace, three forced full native builds of each bundled large
example completed as single final PDF passes with median wall times of 0.575s
for `arXiv-2605.26379v1` (48 pages) and 0.534s for
`arXiv-2511.08544v3` (50 pages), with zero external `pdflatex`, BibTeX, Biber,
or draft-prepass runs. The native speed/parity harnesses now also require the
caption-placement trace field so page-builder diagnostics are present on the
measured path. The speed gate also reports and can enforce the same
two-column graphic float fallback count and estimated-slot budgets as the
rendered-parity harness, so sub-second results remain tied to shrinking the
native functionality gap rather than hiding it.

Compatibility for this track means accepting the TeX/STY input surface and
producing a near-identical rendered PDF. Legacy intermediates such as `.aux`,
`.toc`, `.lof`, `.lot`, `.out`, `.brf`, and verbose rerun logs are treated as
optional diagnostics or fallback bridges, not as success criteria for the fast
end-to-end path. In the PDF-only hot path, list/float metadata scans and
bibliography-style sidecar discovery are skipped unless the rendered PDF can use
their results.
The native parity harness follows the same rule: warnings and failures are about
the rendered PDF, page flow, and dimensions, not sidecar-file shape.

For the external `pdflatex` path, the remaining final-build cost is mostly
`pdflatex` itself and repeated reruns for citations/references. The preview
modes are intentionally not final-quality builds; they trade stable references,
bibliography, image inclusion, TikZ
externalization, and `minted` syntax highlighting for edit-loop speed.
`--draft-prepass` is different from preview mode: it still finishes with at
least one PDF-producing full-image TeX pass, but can avoid image inclusion and
throwaway PDF writes during earlier passes whose main purpose is to write
auxiliary files for BibTeX/Biber/index tools, back-reference data, and other
rerun-controlled state. Once those no-PDF passes are stable, `texpilot` promotes
the next likely final pass to PDF output so it does not spend an additional
full-layout pass merely to confirm convergence.
For file-change rerun warnings, direct mode also snapshots standard LaTeX
rerun files such as `.aux`, `.out`, `.toc`, `.brf`, `.lof`, and `.lot` around
full-layout no-PDF passes. If those files do not change, the next pass can be
promoted to PDF output earlier; if they are still changing, the more
conservative promotion threshold remains in place.

Direct builds also write a `.texpilot-<job>.state.toml` file in the output
directory using TeX's `.fls` recorder dependencies plus BibTeX `.bib`/`.bst`
inputs. If the same build mode is requested and every recorded input is
unchanged, `texpilot` skips TeX entirely. Dependency fingerprints use metadata
for the common fast path and fall back to content hashes when a file's mtime
changes without a length change, so editor/sync-tool touches of unchanged files
do not force a rebuild. When writing fresh state after a build, unchanged
dependency metadata reuses the prior content hash rather than rereading large
inputs such as images. Use `--force` to bypass this cache.
For TeX-like recorded inputs (`.tex`, `.ltx`, `.sty`, `.cls`, `.def`, `.cfg`,
and `.clo`), direct mode fingerprints only through TeX's own effective input
boundary: root files stop at the first uncommented `\end{document}`, and all
TeX-like inputs stop after the line containing an uncommented `\endinput`. It
also ignores physical trailing ASCII spaces at line ends because TeX's line
reader discards them before tokenization, drops full `%` comment lines, and
ignores text after an unescaped `%` while keeping the `%` marker that suppresses
the physical endline. That means comment-prose edits and added pure comment
lines do not invalidate a settled PDF cache. Runs of ordinary spaces immediately
before an inline `%` comment are canonicalized to one space, so editor padding
before a comment also stays on the no-op path; removing that separation entirely
is still treated as effective TeX input. Files containing explicit `\catcode`
assignments or common verbatim-like environments use conservative full-file
effective hashing for comment and boundary shortcuts, because `%` may not be a
comment character there. Parseable inline literal commands such as `\verb`,
`\lstinline`, `\mintinline`, and `\mint` are handled more precisely: `%`,
`\end{document}`, and `\endinput` inside the inline literal are treated as
literal content, while ordinary comments and document/input boundaries outside
the span still use the fast effective hash. Edits on the `\endinput` line still
invalidate the cache when they change effective line content because TeX
continues processing the rest of that line; only later lines are ignored.
Changing scheduler-only settings such as `--max-runs` or `--draft-prepass` does
not invalidate an already settled final-build cache.
For BibTeX/Biber, MakeIndex-style lookup, and related fingerprinting, direct
mode prepends the document tree to Kpathsea search paths while preserving
existing `BIBINPUTS`, `BSTINPUTS`, `TEXINPUTS`, `INDEXSTYLE`, and
`TEXINDEXSTYLE` values, so projects that rely on shared bibliography or index
style directories keep the same path behavior and cache invalidation remains
tied to the actual external files. The direct build and bibliography cache keys
include hashed Kpathsea-relevant environment values, so changing a shared search
path cannot silently reuse outputs from the previous path.
When source changes but bibliography inputs and the settled citation signature
do not, direct builds reuse the existing `.bbl` and skip BibTeX. Plain BibTeX
database fingerprints are narrowed to cited entries for ordinary finite
citation sets, so appending an uncited `@book` to a `.bib` file can stay in the
same no-op cache path. The fast path stays conservative: `\citation{*}`,
crossref-like entries, generated request inputs, and ambiguous parser cases
fall back to whole-file fingerprints. Included/sub-aux bibliographies are
handled too, such as a chapter-level `\bibliography` inside `\include{...}`.
When a fresh `.fls` recorder file is available, BibTeX aux discovery also
includes `.aux` outputs recorded by the latest TeX pass, which covers
package-style bibliography aux files that are not linked from the root aux tree.
Package-written `logreq` request files are used to honor BibTeX-family command
requests such as `bibtex8`/`bibtexu` plus their options, and generated request
inputs such as `main-blx.bib` are tracked in the bibliography cache. Conversely,
stale `.aux` files that are only read by LaTeX, such as chapters excluded by
`\includeonly`, are not treated as active BibTeX jobs when the latest recorder
file is available. BibLaTeX/Biber builds are cached from the generated `.bcf`
plus referenced bibliography datasource fingerprints; ordinary finite-key Biber
`.bib` datasources use the same cited-entry fingerprint fast path, including
globbed BibTeX datasource matches. `\nocite{*}`, non-BibTeX datatypes, and
ambiguous entries fall back to whole-file tracking. Biber is run from the
document directory with explicit input/output directories, so project-local
`biber.conf`/`.biber.conf` lookup follows the source tree rather than the build
directory. The effective Biber config choice is also fingerprinted, so editing a
project config or adding one after a cached build invalidates the bibliography
cache. In automatic bibliography mode, current BibTeX aux files take precedence
over a leftover `.bcf`, so switching a BibLaTeX project from Biber to BibTeX
does not keep using stale Biber metadata from the output directory. Similarly,
Biber runs are tied to a `.bcf` recorded by the latest TeX pass when a recorder
file is available, so an old `.bcf` cannot keep Biber alive after the current
document stops producing one.
If a current document intentionally produces both BibTeX aux files and a Biber
control file, direct mode runs the two backends together only when their `.bbl`
outputs are disjoint; if both would write the same `.bbl`, auto mode fails
early and asks for an explicit `--bib bibtex` or `--bib biber` policy rather
than racing two tools against the same file.
Biber `.bcf` datasource globs also get a cheap match-set fingerprint, so adding
or removing a matching bibliography file invalidates the cache even before any
matched file is read.
When only bibliography or index-tool inputs change, `texpilot` can reuse the
previous `.aux`/control files, run the auxiliary tool first, and then do the
minimum TeX pass needed to refresh the PDF. If that preflight regenerates
byte-identical TeX-consumed outputs such as `.bbl`, `.ind`, `.gls`, `.acr`, or
`.nls`, `texpilot` updates its cache state and skips TeX entirely. The same
byte-equality check applies after an ordinary source-edit TeX pass: rerunning an
auxiliary tool does not force a follow-up TeX pass unless the tool's output
actually changed.

The direct runner also detects generated index-family files, runs MakeIndex or
`makeglossaries` when their content or non-generated style inputs change, and
reruns TeX until the document settles. Supported derived outputs include
`.idx -> .ind`, glossaries `.glo -> .gls`, acronyms `.acn -> .acr`, and
nomenclature `.nlo -> .nls`. For package-managed indexes such as `imakeidx`,
direct mode reads MakeIndex commands emitted in the TeX log and honors `-s`
style files plus common MakeIndex flags such as `-l`, `-r`, and `-p`, tracking
those styles and options as cache inputs. It also follows package-emitted
`xindy`/`texindy` commands for ordinary `.idx -> .ind` jobs and forwards common
Xindy options such as `-L`, `-C`, `-M`, `-I`, and `-d`. For `glossaries`
documents that select Xindy, `texpilot` delegates the glossary/acronym pass to
`makeglossaries` directly instead of going through `latexmk`. Source edits that
leave those inputs unchanged reuse the existing generated outputs. When a fresh
`.fls` recorder file is available, MakeIndex-family job discovery is based on
that latest TeX run rather than every old `.idx`, `.glo`, or `.xdy` file left in
the output directory, so removing an index or switching a glossary away from
Xindy does not keep stale auxiliary jobs alive.

For projects that use `\include{sections/foo}` with an output directory,
`texpilot` mirrors the needed include subdirectories under the output directory
before launching TeX, avoiding the common `I can't write on file
sections/foo.aux` failure from raw `pdflatex -output-directory`. Multiline
braced `\include` and `\input` forms are followed for source discovery and
include-directory setup; `\subfile`, `\subfileinclude`, and
`\InputIfFileExists` are also followed when they resolve to local source files;
`\IfFileExists` probes are followed only when the probed file looks like TeX
source, so binary asset checks do not get read as documents. Source pre-scans
respect `\includeonly`, so excluded chapters do not trigger draft-prepass
decisions, PGF externalization mode forcing, output-subdirectory setup, or
source-driven EPS/SVG conversion work. Common `import` package forms such as
`\import{dir}{file}`, `\subimport`, `\inputfrom`, and `\includefrom` are
followed too.
Chapter-level bibliographies inside those subdirectory includes are handled
through the same sub-aux BibTeX path. For `pdflatex`, local and
Kpathsea-discovered EPS graphics referenced by `\includegraphics` are converted
with `epstopdf` into the output directory before TeX runs, including multiline
`\graphicspath`, literal `\DeclareGraphicsExtensions{...}` extension order,
multiline `\includegraphics`, and starred `\includegraphics*` forms common in
formatted papers. Import-style source
commands also contribute their local directory as a graphics search path while
their files are scanned, so `\import{sections/}{intro}` can preconvert
`sections/fig.eps` for an imported `\includegraphics{fig}` without dirtying the
source tree. TeX-like source extensions in source-discovery probes are matched
case-insensitively, so explicit files such as `chapter.TEX` still participate in
preflight decisions. Documents using the `svg` package's common `\includesvg{...}`
defaults are handled the same way: direct mode runs Inkscape before TeX and
writes `svg-inkscape/*_svg-tex.pdf` plus `.pdf_tex` under the output directory
for TeX to find with shell escape disabled. `\svgpath`, `\graphicspath`, and
package-wide or local `\svgsetup`/`\setsvg` options such as
`inkscapelatex=false`/`latex=false`/`tex=false`,
`inkscapearea=page|drawing|nocrop|crop`, `inkscapeformat=png`, numeric
`inkscapedpi`/`inkscapedensity`, safe `svgextension`/`extension`/`ext` source
extensions, safe relative `inkscapename` values, and safe command-name
`inkscapeexe` overrides are recognized when they appear before the affected
`\includesvg`; custom `inkscapeopt` and EPS/PS export formats are left to the
TeX package/shell-escape path. The
direct runner also executes Asymptote
`.asy -> .pdf` figure jobs and PythonTeX `.pytxcode -> .pytxmcr` jobs
generated by the latest TeX pass, `gnuplottex` `.gnuplot -> .tex/.pdf/etc.`
plot jobs, PGF/TikZ externalization makefiles from `mode=list and make`
(`.figlist`/`.makefile -> externalized `.pdf` figures), and
`glossaries-extra` record-resource jobs through `bib2gls` (`.aux` resource
records -> `.glstex`), then reruns TeX when those generated outputs change.
For PythonTeX, the generated macro file and, when enabled, the Pygments macro
file are both treated as required outputs; dependencies recorded by
`pytex.add_dependencies()` and `pytex.open()` in PythonTeX's
`pythontex_data.pkl` are fingerprinted after each PythonTeX run, so data-only
edits invalidate the external-tool cache. For gnuplottex, local quoted
data/script references in generated `plot`, `splot`, `replot`, `load`, and
`call` commands are also fingerprinted when they resolve to existing files.
Asymptote jobs fingerprint local `import`, `include`, `access`, and
`from ... import` helper files when they resolve beside the generated `.asy`
file, including the common implicit `.asy` suffix. MetaPost jobs likewise
fingerprint local `input` files when they resolve beside the generated `.mp`
file, including the common implicit `.mp` suffix.
Source-driven EPS/SVG conversion discovery is checked once per direct build
invocation, and the discovered job/output-path list is reused for later
conversion checks, auxiliary-output snapshots, and final build-state input
collection in the same run. The EPS and SVG source scans also share a TeX
source-read cache, so projects using both formats do not reread the include
tree for each conversion family. TeX-generated external tools such as
Asymptote, PythonTeX, gnuplottex, PGF externalization, and `bib2gls` are still
checked after each pass because their request files can appear or change as TeX
writes
auxiliary outputs.
Generated aux, index, rerun-state, and external-tool file extensions are also
classified case-insensitively when direct mode reads the recorder output.
When shell escape is disabled, ordinary `\tikzexternalize` documents that do
not explicitly choose an externalization mode are routed through PGF's
`list and make` mode so the direct runner can build the figure PDFs itself.
Independent
BibTeX jobs from multiple included aux files, independent MakeIndex-family jobs,
independent EPS/SVG conversions, independent Asymptote figures, PythonTeX jobs,
gnuplottex plots, and `bib2gls` resource builds run in parallel where their
dependencies allow it; PGF/TikZ externalized figure makefiles are invoked with
a bounded `make -j` when they contain multiple targets.
Bibliography, index/glossary, and external tool families are also scheduled
concurrently after each TeX pass, so large split documents do not serialize
avoidable auxiliary work.

Rerun detection recognizes common LaTeX/package spellings such as biblatex's
`Please rerun LaTeX`, rerunfilecheck file-change warnings, and hyperref page
label rerun requests, while avoiding permanent unresolved-reference warnings
that do not imply another pass will settle the document.

For nonstandard TeX-generated files, the direct runner also snapshots generated
output content across consecutive passes. If a custom generated file keeps
changing after the log stops asking for a rerun, `texpilot` continues until that
custom output settles, while standard `.aux`/`.toc`/`.out` files stay governed
by LaTeX rerun warnings and the bibliography/index tool caches.

Recently measured warmed edit-loop medians after a fresh final build
(`scripts/benchmark_paper.py --scenario warm-edits --runs 3 --texpilot
target/release/texpilot`):

| paper | runner | no-op | touch main | trailing spaces | full-line comment | trailing comment after `\end{document}` | in-document comment | inline-comment padding | inline-verb comment | unused `.bib` edit | cited `.bib` edit |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `examples/arXiv-2511.08544v3/main.tex` | `latexmk` | 0.069s | 0.068s | 4.025s | 4.021s | 4.017s | 4.046s | 4.037s | 4.019s | 0.159s | 8.207s |
| `examples/arXiv-2511.08544v3/main.tex` | `texpilot` direct | 0.052s | 0.051s | 0.051s | 0.051s | 0.051s | 0.051s | 0.052s | 0.051s | 0.051s | 4.360s |
| `examples/arXiv-2605.26379v1/main.tex` | `latexmk` | 0.059s | 0.058s | 2.796s | 2.835s | 2.784s | 2.793s | 2.811s | 2.794s | 0.149s | 2.871s |
| `examples/arXiv-2605.26379v1/main.tex` | `texpilot` direct | 0.034s | 0.034s | 0.034s | 0.034s | 0.033s | 0.034s | 0.033s | 0.033s | 0.034s | 2.888s |

The trailing-space, full-line-comment, trailing-comment, and in-document comment
cases are intentionally narrow: `texpilot` fingerprints TeX-like source bytes as
TeX sees them, so editor whitespace churn, comment-prose edits, and notebook
scratch text after the document terminator do not invalidate the final PDF
cache. In-document edits that change real TeX input still rebuild, and on these
papers their remaining wall-clock time is essentially the cost of one real
`pdflatex` pass.

## Lint rules

Defaults are configured in `texpilot.toml`.

- `math/inline-dollar`: prefer `\( ... \)` over `$ ... $`.
- `math/display-dollar`: prefer `\[ ... \]` over `$$ ... $$`.
- `math/unclosed`, `math/unclosed-environment`, `math/mixed-delimiters`,
  `math/unmatched-*`: catch delimiter balance errors.
- `math/left-right`: catch unmatched or mismatched scalable delimiters such as
  `\left( ... \right]`.
- `indent/size`: enforce environment and display-math body indentation.
- `indent/tabs`: reject tabs.
- `env/mismatch`, `env/unclosed`, `env/unmatched-end`: check environment stack.
- `math/prime-command`: optional preference for explicit `\prime` style inside
  delimiter math and common math environments such as `equation` and `align`.
- `line/length`: configurable line-length warning.

Use `[lint.rules]` to override individual rule levels without disabling the
scanner. Values are `off`, `warn`, or `error`; aliases `allow`/`ignore` and
`deny` are also accepted. For example:

```toml
[lint.rules]
"math/prime-command" = "warn"
"math/inline-dollar" = "error"
"line/length" = "off"
```

The indentation default is "indent every environment except `document`",
including math environments such as `align`; display math opened with `\[` or
`$$` keeps the opener/closer at the surrounding indentation level and indents
the body one level deeper. Set `indent_display_math = false` to leave display
math body indentation alone, and add environments to
`ignored_indent_environments` when theorem/proof blocks should not move content.
Set `check_environment_stack = false` to suppress only `env/*` structural stack
diagnostics while keeping indentation checks active.
Verbatim-like literal blocks such as `verbatim`, `lstlisting`, `minted`, and
`filecontents` are treated as literal content, so math delimiters and example
`\begin`/`\end` text inside them do not produce structural diagnostics.
Inline literal commands such as `\verb`, `\lstinline`, `\mintinline`, and
`\mint` are masked before comment and math scanning, so `%`, `$`, `\[`, and
example environment text inside those snippets do not confuse later lintable
text on the same line.
When `prefer_prime_command` is enabled, apostrophes inside text-like math
payloads such as `\text{Alice's note}` and `\mbox{Bob's label}` are treated as
text rather than prime shorthand.
Use `% texpilot-ignore-line [rule,...]` or
`% texpilot-ignore-next-line [rule,...]` for local exceptions. Omit the rule
list to suppress all diagnostics on the target line.
