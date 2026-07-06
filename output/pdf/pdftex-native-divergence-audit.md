# pdfTeX vs native renderer divergence audit

Date: 2026-07-05

Compared PDFs:

- arXiv-2605 native: `/tmp/texpilot-pdf-compare/arxiv-2605/native/main.pdf`
- arXiv-2605 pdfTeX: `/tmp/texpilot-pdf-compare/arxiv-2605/pdftex/main.pdf`
- arXiv-2511 native: `/tmp/texpilot-pdf-compare/arxiv-2511/native/main.pdf`
- arXiv-2511 pdfTeX: `/tmp/texpilot-pdf-compare/arxiv-2511/pdftex/main.pdf`

Rendered audit artifacts:

- arXiv-2605 contact sheets: `tmp/pdfs/audit/arxiv-2605-contact-01-12.jpg`, `13-24.jpg`, `25-36.jpg`, `37-48.jpg`
- arXiv-2511 contact sheets: `tmp/pdfs/audit/arxiv-2511-contact-01-12.jpg`, `13-24.jpg`, `25-36.jpg`, `37-48.jpg`, `49-50.jpg`
- Per-page diffs: `tmp/pdfs/audit/{case}-page-{NN}-diff.png`
- Metrics: `tmp/pdfs/audit/arxiv-2605-metrics.csv`, `tmp/pdfs/audit/arxiv-2511-metrics.csv`, `tmp/pdfs/audit/all-metrics.csv`

The diff images are 96 dpi Poppler rasters with red marking changed pixels. The page notes below are based on the rendered pages/contact sheets plus `pdffonts`, `pdfinfo`, and the native renderer source.

## Why the fonts are different

This is not a subtle PDF backend issue. The native renderer currently bypasses most of pdfTeX's font machinery.

The native renderer hard-codes layout profiles with one text font, one code font, one math font, and one heading font:

- `crates/texpilot-pdftex/src/native.rs:1870-1896` defines `DocumentLayout` font slots.
- `crates/texpilot-pdftex/src/native.rs:1922-1925` uses Nimbus Roman, Courier, Nimbus italic, and Nimbus bold in the default profile.
- `crates/texpilot-pdftex/src/native.rs:1952-1955` uses the same simplified Nimbus/Courier stack for the NeurIPS single-column profile.
- `crates/texpilot-pdftex/src/native.rs:1982-1985` uses Pagella/Heros/Pagella italic/Heros bold for the ICML two-column profile.
- `crates/texpilot-pdftex/src/native.rs:2148-2188` maps those choices to a tiny internal metric enum. Notably, text line breaking always falls back to `TimesRoman` for text in `line_break_metric_for_font`.
- `crates/texpilot-pdftex/src/native.rs:15209-15215` emits only those five font resources, including `Symbol`.
- `crates/texpilot-pdftex/src/native.rs:15453-15481` emits simple Type 1 font dictionaries.
- `crates/texpilot-pdftex/src/native.rs:15487-15511` only knows embedded Type 1 files for a few TeX Gyre fonts. Other fonts fall back to simple base font resources.

pdfTeX does the opposite. It executes the LaTeX font selection stack, reads font metrics and virtual fonts, consults map files and encodings, subsets embedded fonts, and applies package-level font choices. For arXiv-2511, the style explicitly loads `newpxtext,newpxmath` under pdfTeX (`examples/arXiv-2511.08544v3/simpleicml.sty:48-54`) and enables microtype expansion/protrusion (`simpleicml.sty:82-85`). Our native renderer instead uses TeX Gyre Pagella/Heros plus simplified math text.

Observed font tables:

- arXiv-2605 native: only `NimbusRomNo9L-Regu`, `Courier`, `NimbusRomNo9L-ReguItal`, `NimbusRomNo9L-Medi`, `Symbol`, plus pass-through fonts inside imported figures. Core document fonts are mostly unembedded and unsubsetted.
- arXiv-2605 pdfTeX: subsetted embedded Nimbus fonts, CMTT, CMR, CMMI, CMSY, CMEX, MSBM, and figure fonts.
- arXiv-2511 native: `TeXGyrePagellaX-Regular`, `TeXGyreHeros-Regular`, `TeXGyrePagellaX-Italic`, `TeXGyreHeros-Bold`, `Symbol`, many unsubsetted CM-ish math fonts, and figure fonts.
- arXiv-2511 pdfTeX: subsetted `TeXGyrePagellaX-*`, `TeXGyreHeros-*`, `NewPXMI`, `NewPXBMI`, `pxmiaX`, `txsym`, `txexs`, `t1xtt`, and many subsetted CMR/CMMI/CMSY/CMEX/MSBM fonts.

The visual "lighter font" effect is real in raster terms, but the native streams set text color to black. On page 1, native pages have fewer and lighter non-white pixels than pdfTeX. That is from different glyphs, weights, metrics, density, and line/page placement rather than an explicit gray text color.

## Failure-mode key

These failure modes recur throughout the audit.

- F: Font identity, glyph shapes, encodings, subsetting, kerning, ligatures, and font metrics differ.
- L: Paragraph breaking, line breaking, justification, hyphenation, microtype protrusion/expansion, and page building differ.
- M: Math is flattened/normalized instead of built as TeX math lists with stacked fractions, radicals, scripts, delimiters, limits, equation numbers, and alignment boxes.
- G: Graphics/floats/captions differ in page order, size, scale, crop, caption style, or surrounding text flow.
- T: Tables, booktabs rules, arrays, tabular alignment, listings, and code blocks differ.
- B: Theorem boxes, tcolorboxes, shaded/rule boxes, algorithms, and styled framed environments differ.
- R: Bibliography, citations, cross-references, natbib labels, backrefs, hyperlink coloring, and reference page breaks differ.
- H: Headers, footers, navigation bars, page numbers, and running titles differ.
- X: Title, author, abstract, affiliation, first-page notices, and front matter differ.

F and L are present on every page of both documents. H is present on essentially every page where a header/footer/nav element exists. The page tables list the additional local divergences and the most visible manifestations.

## Summary metrics

| Case | Pages | Mean changed pixels | Median changed pixels | Max changed pixels | Mean RMS |
| --- | ---: | ---: | ---: | ---: | ---: |
| arXiv-2605 | 48 | 16.659% | 16.172% | 34.811% | 47.370 |
| arXiv-2511 | 50 | 18.614% | 19.238% | 47.829% | 54.696 |

Worst pages by changed-pixel percentage:

- arXiv-2605: pages 39, 38, 6, 43, 3, 5, 10, 9.
- arXiv-2511: pages 1, 17, 19, 4, 32, 7, 6, 2.

## arXiv-2605 page-by-page

Baseline for every page: F, L, and page-builder drift. The native output is not running the NeurIPS style's full TeX page builder, font stack, math layout, float placement, or bibliography machinery.

| Page | Changed/RMS | Additional divergences |
| ---: | ---: | --- |
| 1 | 16.8% / 50.8 | X, G, R. Title/author/abstract block differs in size and vertical placement; NeurIPS preprint notice/footer and first-page spacing differ; first figure row and caption are scaled and placed differently; citations/backrefs differ. |
| 2 | 19.8% / 55.8 | M, R. Section flow and paragraph breaks differ; inline math/citation density differs; page starts and ends at different source positions. |
| 3 | 24.0% / 47.9 | G, B, M. Top figure/caption placement differs; pdfTeX has styled theorem/statement box and exact math spacing; native uses simplified text/math and different page break. |
| 4 | 15.4% / 46.9 | M, B. Multi-line displayed derivations, proof spacing, equation numbers, and theorem/proof presentation differ. |
| 5 | 22.7% / 54.3 | B, M. Blue theorem/corollary boxes and section boundaries differ; math displays are flattened; vertical box spacing is wrong. |
| 6 | 26.2% / 52.0 | B, M. Multiple theorem-style blue boxes and equations differ; equation alignment and page break are substantially off. |
| 7 | 16.1% / 49.8 | G, T, M. Figure strip, table, captions, and lower equations differ in scale, order, and tabular alignment. |
| 8 | 15.2% / 46.6 | G, T. Figure placement and experimental table differ; rules, columns, and caption spacing are not pdfTeX-equivalent. |
| 9 | 21.0% / 60.1 | G, M, R. Native and pdfTeX are on different float/text material; pdfTeX image grid does not match native page order; section and citation flow diverge. |
| 10 | 21.4% / 60.0 | G, R. Native still shows experiment figures/text while pdfTeX is in acknowledgments/references; page identity has drifted. |
| 11 | 17.6% / 49.4 | R. Bibliography layout, labels, entry wrapping, hyperlink/backref marks, and page breaks differ. |
| 12 | 16.8% / 45.4 | R. Bibliography remains visually different in wrapping, line height, labels, and references carried to/from adjacent pages. |
| 13 | 17.5% / 47.0 | R. Bibliography entry order/line wrapping/page position differ; native is not matching natbib/BibTeX typesetting. |
| 14 | 18.1% / 48.6 | R. Bibliography pages continue to diverge; hyperlink colors/backrefs and entry breaks differ. |
| 15 | 15.1% / 46.8 | R, T. Bibliography tail and transition material differ; native begins/places appendix-like content at a different vertical position. |
| 16 | 7.2% / 21.9 | R, M. pdfTeX is near the bibliography-to-appendix transition; native contains different appendix math/proof material and different page occupancy. |
| 17 | 9.9% / 41.0 | T, M. Appendix overview table in pdfTeX differs from native's simplified list/math rendering; table rules and alignment are wrong. |
| 18 | 16.7% / 47.6 | B, M. Blue theorem box, proof heading, display math, and equation numbers differ; native proof layout is flattened. |
| 19 | 12.1% / 38.7 | M. Dense proof equations and multiline alignments differ; vertical spacing and equation numbering differ. |
| 20 | 11.6% / 40.1 | M. Continued proof derivations differ in math glyphs, alignment, equation breaks, and line spacing. |
| 21 | 12.4% / 38.0 | M, T. Native does not match pdfTeX display math/table-like derivations; section spacing and page end differ. |
| 22 | 16.2% / 48.4 | B, M. Styled theorem/corollary box and proof display math differ; box rule/background geometry differs. |
| 23 | 15.5% / 42.8 | B, M. Theorem box, proof equations, and heading hierarchy differ; page break continues to drift. |
| 24 | 10.5% / 37.6 | M. Large aligned equations and explanatory proof text differ; equation numbering and vertical spacing differ. |
| 25 | 19.7% / 45.5 | B, M. Theorem box and proof math differ; native page contains shifted text and equations relative to pdfTeX. |
| 26 | 17.1% / 49.0 | M, L. Text-heavy appendix page has different line breaks, section positions, equation placement, and page end. |
| 27 | 13.0% / 40.3 | B, M. Blue proof/theorem box and multi-line derivations differ; math atom placement and numbers are not equivalent. |
| 28 | 11.8% / 43.3 | T, M. Table/rule layout and proof equations differ; section heading and page break differ. |
| 29 | 18.1% / 53.1 | M, L. Dense appendix prose and inline math are rewrapped; heading positions and source material boundaries differ. |
| 30 | 20.4% / 54.6 | M, L. Long paragraphs and inline/display math differ; accumulated line-breaking drift is severe. |
| 31 | 17.6% / 48.6 | T, G, R. Top table/figure/section transition differs; table alignment and caption/source positioning differ. |
| 32 | 16.8% / 48.9 | T, M. Experimental-section table/list material and equations differ; native page starts/ends at different source points. |
| 33 | 12.8% / 39.5 | T, M. Architecture/configuration table and enumerated material differ in rules, columns, and equation/text wrapping. |
| 34 | 16.0% / 48.0 | M. Appendix equations and section text differ; alignment and numbering are not pdfTeX-equivalent. |
| 35 | 15.7% / 50.0 | M, L. Proof text and equations are reflowed; vertical spacing and paragraph density differ. |
| 36 | 16.2% / 50.9 | M. Dense displayed math and proof text differ; exact line breaks and equation placement are off. |
| 37 | 16.4% / 46.7 | G, T. Native float placement differs from pdfTeX; figure/caption and lower table/text are on different vertical schedule. |
| 38 | 34.3% / 95.3 | G, T. Major page identity mismatch: native shows heatmap figures while pdfTeX shows a large architecture/experiment table. |
| 39 | 34.8% / 97.2 | G. Major float-page mismatch: native shows scatter/plot panels while pdfTeX shows heatmaps and caption text. |
| 40 | 15.1% / 45.2 | G, T. Native scatter plots differ from pdfTeX's line plots and table; float scheduling is shifted. |
| 41 | 13.6% / 39.7 | G, T. Native table/text material differs from pdfTeX figure/table material; captions and table rules differ. |
| 42 | 13.6% / 41.0 | G, T. Native text/table material differs from pdfTeX scatter/table page; float order remains shifted. |
| 43 | 24.2% / 60.5 | G. Top figure is close in identity but differs in scale/placement; pdfTeX also places additional grid figure/caption not matched by native. |
| 44 | 14.7% / 41.3 | G. Native figure grid is on a different page than pdfTeX's line-plot page; captions and surrounding text differ. |
| 45 | 9.9% / 25.9 | G, L. Native contains text/proof material while pdfTeX is mostly figure panels and caption; page occupancy differs. |
| 46 | 10.1% / 30.3 | G. Both pages contain plot panels, but scaling, order, captions, and vertical placement differ. |
| 47 | 6.2% / 26.4 | G. Sparse float page differs in figure count/placement and caption; native underfills the page differently. |
| 48 | 15.7% / 35.2 | G. Final figure grid is similar in identity but differs in scale, placement, caption wrapping, and page-bottom text. |

## arXiv-2511 page-by-page

Baseline for every page: F and L. The two-column broad geometry is closer than arXiv-2605, but the native renderer still misses `simpleicml` font/math choices, headers/navigation, exact title box/page builder behavior, float placement, boxes, listings, and math layout.

| Page | Changed/RMS | Additional divergences |
| ---: | ---: | --- |
| 1 | 47.8% / 62.1 | X, G, T. Title/author/abstract block, blue intro panel, teaser figures, table, captions, and first-page vertical spacing differ dramatically. |
| 2 | 27.1% / 72.3 | H, M. Two-column introduction line breaks, running header/navigation, citations, and inline math differ; page starts/ends at different source positions. |
| 3 | 23.1% / 63.6 | G, H. Top figure identity is similar but scale/caption/placement differ; definitions and section text reflow. |
| 4 | 31.8% / 75.0 | G, H. Large figure strip is close in identity but boxed/caption geometry and subsequent two-column flow differ. |
| 5 | 25.4% / 72.5 | B, M. Theorem/definition box styling, equations, and two-column text flow differ; page material is shifted. |
| 6 | 28.3% / 67.4 | G, B, M. Figure panels, colored theorem box, proposition/section boundaries, and math displays differ. |
| 7 | 28.3% / 74.0 | G, B, M. Figures and boxed theorem/proof content are scaled and placed differently; math alignment differs. |
| 8 | 24.5% / 65.3 | G, B, M. Figure strip, theorem/lemma boxes, and equations differ; column break and caption flow differ. |
| 9 | 24.9% / 72.1 | G, M. Native and pdfTeX are on different float/text schedule; top figures and theorem text do not align by page. |
| 10 | 23.5% / 63.9 | G, T, B. Native shows a scatter grid and different section material while pdfTeX has code/box/plot material; page identity differs. |
| 11 | 26.7% / 64.5 | T, B, M. Listing/code block, theorem box, equations, and surrounding text differ; native content is shifted. |
| 12 | 26.1% / 68.0 | G, M, B. Figure and equation material are scheduled differently; theorem box and display math are not equivalent. |
| 13 | 19.2% / 61.9 | G, T. Native conclusion/results material differs from pdfTeX plot/table page; table rules and captions differ. |
| 14 | 22.4% / 63.8 | G, T. Native text/references-like material differs from pdfTeX figure/table/results page; float scheduling is off. |
| 15 | 24.6% / 63.2 | G. Native plot page is delayed relative to pdfTeX multi-plot page; captions and column text differ. |
| 16 | 25.6% / 78.3 | G, T. Native figure/text page differs from pdfTeX line plots, table, and image strip; float placement drift is severe. |
| 17 | 35.5% / 76.5 | G. Major page identity mismatch: native plot/experiment page vs pdfTeX image panels and experiment text. |
| 18 | 26.0% / 78.1 | R, G, T. Native table/image-strip page differs from pdfTeX conclusion/references page; bibliography transition shifted. |
| 19 | 34.5% / 76.8 | R, G. Native image panels and text differ from pdfTeX references page; page identity mismatch. |
| 20 | 24.1% / 66.6 | R. Both are in reference-like material, but entry wrapping, labels, spacing, and page boundaries differ. |
| 21 | 25.3% / 66.5 | R. Bibliography continues to differ in two-column line breaks, hyperlink styling, labels, and entry pagination. |
| 22 | 25.7% / 67.6 | R. Bibliography tail remains offset; entry order/wrapping/page end do not match pdfTeX. |
| 23 | 4.5% / 29.1 | R. End-of-bibliography page is closest in changed-pixel percentage, but labels, wrapping, and vertical placement still differ. |
| 24 | 12.0% / 56.2 | X, B, M. Appendix title/ruled title page and first appendix material differ; pdfTeX begins with a styled theorem box and exact math layout. |
| 25 | 21.4% / 60.2 | B, M. Theorem/proof boxes and display equations differ; native flattens multiline math and box rules/backgrounds. |
| 26 | 19.6% / 50.5 | M, B. Proof equations, equation numbering, and theorem box layout differ. |
| 27 | 7.9% / 38.3 | M. Proof alignments and equations differ; native has flattened math and different vertical whitespace. |
| 28 | 8.0% / 37.5 | M. Continued proof derivations differ in stacked fractions, alignment points, equation numbers, and page breaks. |
| 29 | 8.7% / 39.0 | M. Proof equations and displayed derivation trees differ; native text math is not TeX math layout. |
| 30 | 13.2% / 54.1 | B, M. Boxed theorem/proof plus dense equations differ; box geometry and math alignment are wrong. |
| 31 | 9.4% / 42.0 | M, L. Mostly text/proof page, but equation display and paragraph breaks differ. |
| 32 | 30.8% / 47.0 | B, M. Large theorem/box region differs heavily; display math and vertical spacing are not aligned. |
| 33 | 7.6% / 36.8 | M. Equation derivations and proof text differ; equation numbering/placement differs. |
| 34 | 8.0% / 38.3 | M. Long aligned derivations differ in stacked structure and equation labels; native flattening is visible. |
| 35 | 18.9% / 54.8 | B, M. Theorem/proof box and equations differ; text flow and page end differ. |
| 36 | 14.2% / 51.0 | M. Proof display math, alignment, and vertical spacing differ. |
| 37 | 9.4% / 41.1 | M. Continued proof equations differ in alignment, stacked fractions, delimiters, and equation numbers. |
| 38 | 7.7% / 37.5 | M. Proof equations remain non-equivalent; native's flattened forms and page spacing differ. |
| 39 | 6.1% / 32.8 | M. Equation-numbered align environment differs; native has simplified math and different vertical rhythm. |
| 40 | 9.0% / 39.4 | G, M. Native includes a plot/table-like page while pdfTeX continues proof equations; float/page scheduling differs. |
| 41 | 9.7% / 41.2 | G, T, M. Native shows tables/text while pdfTeX has equations and section transitions; table and math layout differ. |
| 42 | 13.1% / 50.6 | G, T. Native text/proof page differs from pdfTeX plot plus tables; float ordering has shifted. |
| 43 | 11.7% / 49.6 | T, G. Native math/text page differs from pdfTeX tables; tabular rules and page identity differ. |
| 44 | 8.1% / 39.5 | M. Both are equation-heavy, but exact alignments, references, equation numbers, and font/math glyphs differ. |
| 45 | 5.9% / 35.2 | M, T. Native is sparse while pdfTeX contains long derivation and a highlighted table/note; page occupancy differs. |
| 46 | 2.0% / 20.5 | M. Closest appendix page; both are sparse, but final equation/note placement and header/footer still differ. |
| 47 | 19.3% / 61.4 | G. Figure grid identity is close, but scale, page placement, captions, header/nav, and font/caption metrics differ. |
| 48 | 13.6% / 45.8 | G. Figure panels are close in content but differ in scale, vertical spacing, captions, and page-bottom placement. |
| 49 | 16.9% / 44.3 | G. Multi-panel plots differ in scale, grid spacing, caption placement, and page-end flow. |
| 50 | 13.3% / 40.8 | G. Final plot pages differ in scale, caption wrapping, and vertical placement; headers and font metrics remain different. |

## Root causes by implementation area

1. Font subsystem: native hard-coded font profiles replace pdfTeX NFSS, TFM/VF/map/encoding/subsetting, package fonts, and microtype font expansion/protrusion.
2. Text/page builder: native uses fixed wrap widths, line heights, and simple placement; pdfTeX uses Knuth-Plass paragraph breaking, glue/shrink/stretch, penalties, inserts, output routines, and class/style dimensions.
3. Math layout: native normalizes math to visible text; pdfTeX builds math lists with atom classes, styles, scripts, radicals, delimiters, alignment, equation numbering, and font-family selection.
4. Float scheduler: native places images/captions in a simplified flow; pdfTeX schedules floats across pages/columns with penalties and constraints.
5. Environment renderer: native approximates `tcolorbox`, theorem styles, tables, listings, algorithms, and section/navigation styles instead of executing their TeX box construction.
6. Bibliography/cross-reference stack: native does not reproduce natbib/BibTeX/backref/hyperref page-break behavior.
7. PDF backend details: native PDF object generation is functional but not pdfTeX-compatible in font resources, encodings, object/version choices, metadata, and exact text drawing sequences.

## Bottom line

The divergence is systemic, not a final one-pixel backend issue. Every page differs because the native renderer currently substitutes its own simplified document model for pdfTeX's font, paragraph, math, box, float, and output-routine machinery. The most direct path to pixel parity is not another round of local layout tuning; it is replacing the simplified native layout engine with real TeX semantics for fonts/TFM/VF/map files, paragraph breaking, math lists, output routines, and float/env box construction, while keeping pdfTeX only as a verifier.
