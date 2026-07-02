# texpilot-pdftex

This crate is the experimental pdfTeX-rewrite track for `texpilot`.

It is intentionally separate from the current CLI crate. Today `texpilot`
optimizes the build graph around TeX: it chooses passes, prepares preview modes,
runs legacy helper tools when needed, tracks inputs, and reports timing.
External `pdflatex` still owns the expensive engine work:

- category-code-sensitive tokenization;
- macro expansion, grouping, assignments, registers, and primitives;
- package execution and file I/O;
- paragraph breaking, math layout, page building, and output routines;
- fonts, encodings, image inclusion, PDF primitives, and PDF object writing;
- diagnostics, optional interop artifacts, and shell-escape semantics.

That boundary is the reason scheduler work can beat `latexmk` but cannot make a
clean final-quality build sub-second once one ordinary pdfTeX pass already takes
multiple seconds.

## Target

The target is not a preview renderer and not a LaTeX parser. The target is a
high-performance TeX/STY-to-PDF engine backend that can eventually replace
external `pdflatex` for final builds. Compatibility is judged by the generated
PDF: for the same TeX/STY inputs, the native backend should produce a
near-identical rendered document quickly. Legacy sidecars such as `.aux`,
`.fls`, `.log`, `.bbl`, `.idx`, and SyncTeX are optional adapters or debugging
outputs, not the hot-path correctness model.

Development should stay ruthless about that boundary. A compatibility feature
belongs on the native hot path only when it improves rendered-PDF parity or
unblocks TeX/STY semantics that do. Recreating page chrome, rerun logs, aux-file
shape, BibTeX/Biber choreography, or other process artifacts is not useful by
itself if the pixels do not get closer.

For the bundled large-paper benchmark, the first meaningful gate is:

1. produce near-identical rendered PDF output compared with external pdfTeX;
2. beat one external PDF-producing pdfTeX pass on clean final builds;
3. then push both large examples below one second end-to-end.

## Why a rewrite can win

The external-engine path pays for work that a native backend can avoid or
collapse:

- process startup and distribution/package reparsing;
- repeated full TeX passes for aux convergence;
- disk round-trips for control files that can be modeled in memory;
- external bibliography and index runs for common paper patterns;
- serial asset loading and PDF object generation;
- opaque engine state that `texpilot` cannot inspect or reuse.

The planned backend attacks those directly with memory-mapped format snapshots,
preindexed distribution files, in-memory document fixed-point solving, native
bibliography/index models, a native layout/PDF backend, and parallel asset/PDF
pipelines.

## Non-solutions

These are useful in other parts of the project but do not satisfy the clean
full-build target by themselves:

- faster source parsing without TeX expansion;
- preamble caching alone;
- pass scheduling alone;
- preview-only rendering;
- page reuse that only helps warm incremental edits;
- exact `.aux`/`.log`/`.fls`/BibTeX/Biber sidecar compatibility;
- silently switching users to an incompatible LaTeX subset.

## Phases

The code exposes the current boundary and roadmap as Rust data so later
integration work can test against it:

- `p0`: rendered-PDF equivalence harness;
- `p1`: expansion core;
- `p2`: snapshot and file-system layer;
- `p3`: in-memory document convergence;
- `p4`: layout and PDF backend;
- `p5`: sub-second candidate.

The default `texpilot` runner still uses external TeX engines. This crate is
where the replacement engine can grow until its rendered PDFs are equivalent
enough and fast enough to be wired into `BuildOptions` as a default-capable
backend.

## Current executable subset

The crate is now wired into the main CLI as:

```sh
cargo run -- build path/to/main.tex --engine texpilot-pdftex
```

The native backend currently handles a deliberately pragmatic final-build
subset:

- article-style `\begin{document}` / `\end{document}`;
- recursive local and `TEXINPUTS`-resolved `\input{...}` / `\include{...}`
  expansion with `.tex` extension probing, `\IfFileExists`/
  `\InputIfFileExists` branch expansion, `\jobname` file-name expansion, plus
  `\includeonly{...}` filtering;
- plain paragraphs and line wrapping;
- `\title`, `\author`, `\date`, `\maketitle`, starred/numbered sectioning,
  `\AtBeginDocument`/`\AtEndDocument` hooks, visible run-in `\paragraph`
  heading rendering, and NeurIPS-style paragraph before-skip accounting;
- focused `\newcommand`/`\renewcommand`/`\providecommand`,
  `\DeclareRobustCommand`, `\DeclareMathOperator`, `\def`, `\gdef`,
  `\edef`/`\xdef`, and `\protected@edef` macro expansion, including positional
  arguments for common paper macros;
- simple inline formatting, transparent color/style text wrappers with visible
  `\textsc` small-caps text, plus permissive inline math segment/control-word
  fallback and common math wrapper/operator/script normalization in rendered
  equations, including compact fraction/root/accent atoms, layout/style
  declaration cleanup, prose/caption math-boundary cleanup, relation/operator
  spacing, labeled arrow normalization, derivative/gradient and set-operator
  cleanup, transpose cleanup, under/over brace annotation cleanup, native PDF
  Symbol-font rendering for common Greek, relation/operator, arrow,
  perpendicular, partial, gradient, square-root, and plus-minus glyphs,
  minus-plus ASCII fallback, raw `$$...$$` display blocks, and `\[...\]`
  display-delimiter cleanup on the text-cleaning path;
- package declarations as no-ops by default, with local `.sty` discovery,
  recursive local package dependency tracking, optional legacy `.fls` export, and
  native-safe adapters for selected document-command packages such as
  `simpleicml` and `sectionnav`;
- bibliography loading with numeric `\cite`/`\citep`/`\parencite` rendering,
  textual numeric `\citet`/`\textcite` author labels, `plainnat` author-year
  `\cite`/`\citep`/`\citet` labels, natbib-style optional citation pre/post
  notes, `\bibliography` and `\addbibresource` discovery, basic BibTeX
  brace/accent/text-command cleanup, manual `thebibliography`/`\bibitem`
  parsing, and a generated references section;
- section `\label{...}`, `\ref{...}`, and `\pageref{...}` resolution without
  aux-file reruns, plus one-pass `\tableofcontents` rendering with
  `\addcontentsline` TOC entries and appendix section numbering;
- native `\footnote`, `\thanks`, `\footnotemark`, and `\footnotetext` markers
  using compact numeric inline marks with rendered notes;
- numbered `equation`, `align`, and `multline` environments with labels, plus
  lightweight raw `$$...$$` display-math blocks;
- approximate table/list/algorithm/listing/minipage wrappers,
  itemize/enumerate/description markers, optional numbered captions,
  `\makecell`/`\thead` text-cell unwrapping, source-derived
  `\lstlistingname` for listing captions and single listing references,
  single `\cref` prefixes for section/figure/table/theorem/listing labels, plus
  local style `labelsep=period` caption punctuation and conservative wrapping
  for extreme caption overflows,
  native `lstlisting`, grouped two-column `lstfloat` blocks, local and
  `TEXINPUTS`-resolved `\lstinputlisting`, verbatim-like, minted, and
  `\inputminted` code rendering, `\captionof`,
  one-pass `\listoffigures`/
  `\listoftables` rendering with native `.lof`/`.lot`, and ignored
  `\captionsetup`;
- common hyperref text commands (`\href`, `\url`, `\hyperref`, `\autoref`),
  `\phantomsection`, native PDF outlines and `.out` bookmark output from
  `\pdfbookmark`, section, and subsection anchors, plus page-backref `.brf`
  records;
- common `\pdfinfo{...}` and `\hypersetup{pdf...=...}` metadata as native PDF
  `/Info` entries, plus safe no-op `\pdfcatalog`, `\pdfnames`, `\pdftrailer`,
  `\pdfmapfile`, and `\pdfmapline` declarations;
- theorem-like tcolorbox/amsthm headings with native counters and labels, plus
  colon-style titled graphical theorem boxes for image-heavy definition panels,
  including theorem boxes nested inside wide figure floats;
- JPEG/PNG `\includegraphics` as native PDF image XObjects, including
  local and `TEXINPUTS`-resolved `\graphicspath` and
  `\DeclareGraphicsExtensions` lookup, common `graphicx` sizing options
  (`width`, `height`, `scale`, `keepaspectratio`, `angle`, `\linewidth`,
  `\textwidth`, and `\columnwidth`) plus PDF-visible `trim`/`viewport` boxes
  with `clip`, and PNG alpha soft masks;
- first-page PDF `\includegraphics` as Form XObjects with imported resources;
- PDF `\includegraphics[page=N]{...}` selection for one-based page imports;
- other resolved graphics as tracked placeholders;
- style-aware built-in PDF base-font selection for the current NeurIPS-like
  Times and `simpleicml` Palatino/sans-heading profiles;
- calibrated NeurIPS-style `\LARGE` title sizing, baseline, and centering;
- native NeurIPS-style `\And` author-grid title blocks with preserved author,
  affiliation, and email rows;
- calibrated native page-number rendering for the current conference-style
  page profiles, with title-page numbers suppressed where those styles do so;
- calibrated native `simpleicml`/`sectionnav` two-line running headers on
  non-title pages;
- centered native title, author, and abstract-line rendering, including
  full-width ICML-style title panels with shaded abstract boxes and native
  teaser image/table rows;
- PDF text objects isolate fill color so decorative colored boxes and headers do
  not leak graphics state into subsequent body text;
- caption-sized figure, table, and listing captions, including denser full-width
  minipage figure captions, single-column graphic figure row packing, and
  teaser-style minipage figure rows grouped as native float blocks with
  column-top placement, plus deferred placement for leading wide top floats in
  two-column layouts;
- page-builder controls for strict `\clearpage`/layout switches and soft
  near-bottom `\newpage` hints that release queued top floats only when doing so
  preserves rendered flow, plus conservative positive `\vspace` materialization
  for common TeX length units and standalone `\smallskip`/`\medskip`/
  `\bigskip` skip glue;
- nested inline layout-scaffold cleanup for teaser-style minipage/tabular
  content that reaches text-cleaning paths;
- native `.pdf`, `.log`, `.fls`, trace output, minimal `.synctex.gz`, `.toc`,
  `.lof`, `.lot`, `.out`, `.brf`, explicit local `\newwrite`/`\openout`/
  `\write`/`\closeout` sidecar outputs, simple local and `TEXINPUTS`-resolved
  `\newread`/`\openin`/`\read`/`\ifeof`/`\closein` input streams,
  basic `.idx` output and native `\makeindex`/`\index`/`\printindex`
  rendering,
  local and `TEXINPUTS`-resolved class/package/style traversal for native
  adapters and package-option detection, including simple
  `\PassOptionsToPackage{...}{...}` propagation and
  `\RequirePackageWithOptions{...}` package loads, and page-aware `.aux` files
  with supported label, table-of-contents, citation, `\bibstyle`, local and
  `BIBINPUTS`-resolved `\bibdata` records, plus local and
  `BSTINPUTS`-resolved bibliography style inputs from `\bibliographystyle`.

Rendered PDF parity is the compatibility test. Those sidecar outputs are
transitional interop/debug artifacts, not the contract: the native document
state that drives PDF generation is the correctness path, and exporting legacy
files should remain optional unless a caller explicitly asks for them.

Anything outside that subset returns an unsupported reason and the main
`texpilot` direct runner falls back to external `pdflatex`. Runtime modes whose
semantics are not implemented natively yet, including shell escape, also use
that fallback path instead of being silently ignored.

On the bundled large-paper examples, the native path now expands the source
tree and writes final PDFs as one native pass without invoking external
`pdflatex`, BibTeX, or draft-prepass convergence. This is a fast functional
renderer moving toward near-identical PDF output: rich layout constructs are
currently represented approximately.

The main CLI uses the PDF-only artifact policy for supported native documents:
it writes the PDF plus `*.texpilot-pdftex.trace` and skips `.aux`, `.fls`,
`.log`, and other legacy sidecars unless a caller explicitly uses the lower
level legacy-artifact mode for diagnostics or interoperability tests. This keeps
the hot path focused on TeX/STY-to-PDF functionality instead of recreating
outdated rerun machinery. The same in-memory layout pass now feeds page-count
selection, PDF page-stream generation, SyncTeX boxes, and layout trace metrics,
so the measured output-routine state is the state that writes the PDF. Explicit
`\openout`/`\write` streams are stripped, not materialized, on the PDF-only hot
path; list/float sidecar scans and bibliography-style metadata discovery are
also skipped unless they can affect rendered PDF output. Legacy-artifact mode
remains available when sidecar inspection is useful.
The native trace also records caption pages from the rendered layout placements,
giving float/page-builder work a direct native signal instead of relying only on
post-hoc PDF text extraction. It now also records
`layout_two_column_graphic_float_fallbacks` and
`layout_two_column_wide_graphic_float_fallbacks`, counting graphic float bodies
that are visible TeX/STY input but still fall through the approximate
two-column path instead of entering the native grouped-float/output-routine
model. For those skipped bodies, the trace also records shadow native sizing:
`layout_two_column_graphic_float_fallback_estimated_native_slots`,
`layout_two_column_wide_graphic_float_fallback_estimated_native_slots`, and
per-fallback `native_rows`, `native_image_slots`, `native_caption_slots`, and
`native_slots`. These diagnostics do not change the rendered document; they
size the exact output-routine work that must be implemented before enabling the
float bodies on the hot path.

The p5 clean-build timing gate is encoded as:

```sh
scripts/native_pdftex_gate.py
scripts/native_pdftex_gate.py --max-two-column-graphic-float-fallbacks 11 --max-two-column-wide-graphic-float-fallbacks 6 --max-two-column-graphic-float-fallback-native-slots 258 --max-two-column-wide-graphic-float-fallback-native-slots 132
```

It runs the two bundled large examples through `--engine texpilot-pdftex`,
requires the native trace path plus caption-placement diagnostics rather than
fallback, and fails if either median full-build wall time exceeds one second by
default. The current release gate on this workspace passes with medians of
0.575s for `arXiv-2605.26379v1` and 0.534s for `arXiv-2511.08544v3` across
three forced clean native builds per paper.
It also reports the two-column graphic float fallback counters and estimated
native-slot debt, and can enforce the same native-coverage budgets as the
rendered-parity harness. That keeps the sub-second gate honest: a build is not
considered healthier merely because it is fast while more TeX/STY float
functionality has slipped back to the approximate path.

The native hot path resolves project-local and explicit `TEXINPUTS`/
`BIBINPUTS`/`BSTINPUTS` search roots in process before falling back to
`kpsewhich`. Standard distribution class traversal is deliberately not on the
default parser path: local and search-path classes still contribute adapters
and package options, but asking the TeX installation about `article.cls` is
legacy discovery work unless it changes the rendered PDF.

The rendered-PDF equivalence harness is encoded as:

```sh
scripts/native_pdftex_parity.py --smoke
scripts/native_pdftex_parity.py examples/arXiv-2605.26379v1/main.tex
scripts/native_pdftex_parity.py --top-pages 3 --write-comparison-pages
scripts/native_pdftex_parity.py --caption-drift 8
scripts/native_pdftex_parity.py --fail-on-warn --require-page-count-match
scripts/native_pdftex_parity.py --caption-drift 8 --max-caption-drift-sum 40
scripts/native_pdftex_parity.py --max-two-column-graphic-float-fallbacks 0 --max-two-column-wide-graphic-float-fallbacks 0 --max-two-column-graphic-float-fallback-native-slots 0 --max-two-column-wide-graphic-float-fallback-native-slots 0
```

It builds an external `pdflatex` baseline, builds this native backend without
fallback, rasterizes both PDFs, and reports exact page hashes plus RMSE and
different-pixel metrics. Use `--strict` or explicit thresholds when turning
those measurements into a failing near-identical-PDF gate. Use
`--fail-on-warn` to promote warning-class rendered-output regressions, such as
page-count or page-dimension drift, to failures while the RMSE thresholds are
still being calibrated.
Use `--top-pages N` to print the worst rendered pages by RMSE, and
`--write-comparison-pages` to write side-by-side baseline/native PPMs under the
case workdir. Use `--caption-drift N` to print the figure/table/listing captions
with the largest native-vs-baseline page delta. The caption diagnostic resolves
`\input`/`\include` from the TeX root,
uses source caption text to filter baseline `pdftotext` matches, and uses native
`layout_caption` trace entries for the native side when available, so prose
references like "Table 4. While ..." are not mistaken for captions. That
diagnostic is intentionally about rendered document flow: it helps separate
"this page has different pixels" from "this float or table landed six pages
early."
The same diagnostic now reports aggregate caption-flow metrics (`count`,
`sum_abs`, `mean_abs`, and `max_abs`) and can gate them with
`--max-caption-drift-sum`, `--max-caption-drift-mean`, and
`--max-caption-drift-page`. These are page-flow gates, not legacy auxiliary-file
checks.
The harness also reads native coverage counters from the trace and can gate
two-column graphic float bypasses with
`--max-two-column-graphic-float-fallbacks` and
`--max-two-column-wide-graphic-float-fallbacks`, plus the corresponding
estimated native-slot debt with
`--max-two-column-graphic-float-fallback-native-slots` and
`--max-two-column-wide-graphic-float-fallback-native-slots`. These limits track
remaining output-routine functionality that still affects rendered PDF parity;
they are not a promise to recreate legacy float sidecars or logs.

That rendered output is the compatibility target. `.aux`, `.toc`, `.lof`,
`.lot`, `.out`, `.brf`, BibTeX/Biber choreography, and verbose rerun logs are
only useful when they help explain or bridge missing TeX/STY semantics; they
are not required artifacts for the high-performance replacement path.
The harness therefore gates page flow, dimensions, and rendered-pixel distance;
it deliberately does not gate legacy sidecar shape.

Current rendered-parity evidence for the large examples is page-count and
dimension agreement, not near-identical output yet. At the harness default of
96 DPI, the current native renderer measures:

| source | pages | mean RMSE | max page RMSE | max diff ratio |
| --- | ---: | ---: | ---: | ---: |
| `examples/arXiv-2605.26379v1/main.tex` | 48/48 | 53.139 | 65.208 | 0.2788 |
| `examples/arXiv-2511.08544v3/main.tex` | 50/50 | 53.503 | 77.314 | 0.5779 |

With caption-flow diagnostics enabled, the same run reports aggregate absolute
caption drift of 35 pages across 23 matched captions for `arXiv-2605.26379v1`
and 37 pages across 21 matched captions for `arXiv-2511.08544v3`; both have a
largest single-caption drift of 5 pages.
The corresponding native trace surface shows 0 two-column graphic-float
fallbacks for `arXiv-2605.26379v1`, but 11 for `arXiv-2511.08544v3`, including
6 starred/wide graphic floats. Their shadow native footprint is 258 estimated
slots total, including 132 estimated slots from starred/wide floats. That makes
the ICML-style output-routine work a measurable TeX/STY-to-PDF functionality
gap rather than a legacy sidecar issue.

The main remaining replacement gaps are therefore visual-fidelity gaps rather
than scheduler gaps: real TeX paragraph breaking, page-builder/output-routine
behavior, Computer Modern/math font embedding, math layout, float placement,
and package-level layout semantics must replace the current approximate line
model before `--strict` rendered parity can pass.

Recent float experiments reinforce that rendered parity is the only useful
compatibility test here. Treating more float bodies as native blocks sounds
closer to TeX, but naive single-column top placement moved a large appendix
figure too early and changed the worst-page RMSE; broad tabular-float block
placement changed the large NeurIPS-style paper from 48/48 pages to 48/49 pages.
Those are regressions, not progress. The accepted path is narrower: once the
active layout has switched to `\onecolumn`, scoped starred `table*` bodies with
native-parsable `tabular` or `\resizebox` payloads enter the wide top-float
path, preserving the current large-paper page counts and rendered metrics. The
next float target is therefore not more command-by-command legacy emulation,
but an actual deferred float-page/output-routine model that can hold oversized
tables and figures until the rendered baseline would place them.

The caption-flow diagnostic currently shows that the large NeurIPS-style paper's
appendix floats are still early, but less severely after bibliography placement
was moved to the source command location: Table 4 is now 5 pages early, Figures
7 and 9 are 4 pages early, Table 6 and Figure 10 are 3 pages early, Tables 5 and
7 are 2 pages early, and Figure 4 is 2 pages late.
The ICML-style paper now uses a denser `simpleicml` two-column row model and a
separate physical one-column row model after `\onecolumn`; that keeps 50/50
pages while bringing the main experimental floats and tables from 5-7 pages late
down to roughly one page late, and the early two-column `figure*` after the
ICML title block now lands on the baseline page instead of 2 pages early.
One-column graphic figures now enter the native
top-float path, including starred `figure*` graphics once `\onecolumn` is active,
and scoped one-column starred `table*` bodies now enter the same top-float path
when their table payload can be represented natively. Oversized top floats start
at a page top instead of mid-page, and overflow
placements reserve page count without re-rendering the same oversized visual
object on every overflow page. This is still not true TeX clipping or float-page
splitting, but it removes a duplicated-figure artifact, improves the
NeurIPS-style mean RMSE measurement, and moves several ICML appendix floats
closer to the baseline. The remaining ICML flow error is concentrated in the
appendix: Figure 17 still lands 5 pages early, Figures 18, 19, and 20 land 3
pages early, Figure 15 lands 2 pages early, Table 4 lands 3 pages late, and
nearby appendix tables are 2 pages late. The native page
builder now carries layout with each placement and supports explicit output
controls for `\clearpage`, `\onecolumn`, and `\twocolumn`, plus a soft
near-bottom `\newpage` hint for the approximate line model, so appendix content
can switch geometry without changing the global document layout. Native
bibliography rendering is likewise placed at `\bibliography`,
`\printbibliography`, or `thebibliography` in the source stream, with denser
bibliography wrapping to match conference-style reference typography. Earlier
isolated experiments were worse: literal hard `\newpage` handling changed the
NeurIPS-style example from 48/48 to 48/49 pages, while the accepted soft
near-bottom handling preserves the current 48/48 and 50/50 rendered-page
counts; simple `\clearpage` handling changed a large example to 50/51 pages, a
narrower
one-column-graphics-only experiment made the ICML-style paper 50/53 pages,
promoting two-column `figure*` graphics directly to wide top floats made it
50/54 pages, and compacting table rows dropped the ICML-style paper to 50/49
pages. Deferring one-column wide top floats behind following text preserved
page counts and slightly lowered ICML mean RMSE, but worsened appendix caption
flow, moving Figure 17 from 5 to 6 pages early and Figures 18-20 from 3 to 5
pages early. The next target is therefore the real deferred
float-page/output-routine itself, especially queue release around appendix
transitions, oversized tables, and real float clipping/splitting, not more
command-by-command legacy emulation.

Recent display-math experiments landed the same lesson in smaller form:
lightweight raw `$$...$$` display rows preserve the current page-count parity,
but promoting raw `\[...\]` displays to standalone native rows makes the
ICML-style paper grow to 51-55 native pages depending on the glue model. The
accepted `\[...\]` support therefore remains delimiter cleanup through the text
path until the page builder can model real TeX display glue. This improves
TeX/STY syntax coverage without pretending the approximate line model is a real
TeX display-math/page-builder implementation.

The main CLI treats this as a native-first backend: supported documents return
from the single-pass Rust renderer, while unsupported documents keep the
`*.texpilot-pdftex.trace` reason and then re-enter the ordinary `pdflatex`
direct runner. That fallback is a development bridge for uncovered TeX/STY
semantics, not a compatibility design goal.

The crate also exposes an executable p1 expansion-core slice through
`ExpansionEngine`: streaming tokenization with mutable catcodes, `\def`/`\gdef`,
`\edef`/`\xdef`, `\protected@edef`, `\csname`/`\endcsname`, `\expandafter`,
`\futurelet`, `\aftergroup`, `\relax`, focused
`\newcommand`/`\renewcommand`/`\providecommand` and `\DeclareRobustCommand`
macro definitions, protected `\DeclareRobustCommand` aliases, `\let` aliases,
positional argument substitution, optional first-argument defaults,
`\protected` definition prefixes, local brace/`\begingroup` scopes, global
assignment forms such as `\global\def`, and token-level conditionals such as
`\iftrue`, `\iffalse`, `\ifx`, `\ifdefined`, `\ifcsname`, `\if`, `\ifcat`,
`\unless`, `\ifnum`, `\ifodd`, `\ifcase`, and `\newif`, raw and aliased count registers
with `\count<number>`, `\countdef`, `\newcount`, assignment, `\advance`,
`\number`, `\numexpr`, and `\the`, common pdfTeX count primitives such as
`\pdfoutput`, `\pdfminorversion`, `\pdfcompresslevel`, and
`\pdfobjcompresslevel`, plus native-disabled `\pdfshellescape`, dimension
registers with `\dimen<number>`,
`\dimendef`, `\newdimen`, `\ifdim`, `\dimexpr`, common TeX units including
separated `true` units such as `297 true mm`, and pdfTeX page dimension
primitives such as `\pdfpagewidth`, `\pdfpageheight`, `\pdfhorigin`, and
`\pdfvorigin`,
skip/glue registers with `\skip<number>`, `\skipdef`, and `\newskip`,
token-list registers with `\toks<number>`, `\toksdef`, and `\newtoks`,
pdfTeX token primitives such as `\pdfpageattr` and `\pdfpageresources`,
expandable primitives such as `\romannumeral`, `\string`, `\meaning`,
`\detokenize`, `\unexpanded`, `\expanded`, `\noexpand`, `\jobname`,
`\pdfprimitive`, `\ifpdfprimitive`, `\pdfcreationdate`, `\pdffilesize`,
`\pdffilemoddate`, `\pdffiledump`, `\pdfstrcmp`,
`\pdfescapehex`, `\pdfunescapehex`, `\pdfescapestring`, `\pdfescapename`,
`\pdfmdfivesum`, and `\pdfmdfivesum file {...}`,
pdfTeX version constants such as `\pdftexversion` and `\pdftexrevision`,
constants from `\chardef`/`\mathchardef`, plus focused `\catcode`/`\makeatletter` assignments.
The native renderer now uses that engine as a file-aware pre-expansion pass
before document modeling, and injects a native-safe `pdftexcmds` adapter for
installed-package wrappers around supported pdfTeX primitives. This is the
first step from string-level document handling toward a TeX-shaped expansion
pipeline.
