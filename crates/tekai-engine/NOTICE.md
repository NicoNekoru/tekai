# Tekai engine source provenance and licenses

`tekai-engine` contains modified Rust translations and adaptations of pdfTeX,
TeX, e-TeX, and Web2C sources. The combined crate is distributed under
`GPL-2.0-or-later`. Its upstream code was never covered by the repository's
former MIT package label. Earlier MIT grants for independently authored Tekai
material remain in [LICENSE-MIT](../../LICENSE-MIT).

The original copyright, permission, warranty, naming, and non-promotion
notices retain their original wording in
[upstream-notices.txt](licenses/upstream-notices.txt), with trailing spaces
and tabs normalized. The complete
[GNU GPL version 2](licenses/GPL-2.0.txt) and
[GNU LGPL version 2.1](licenses/LGPL-2.1.txt) texts come from the same upstream
snapshot. This notice concerns engine source. Bundled TeX packages, fonts,
runtime data, and Cargo dependencies retain their own licenses.

## Exact upstream snapshot

The import used the [TeX Live source repository](https://github.com/TeX-Live/texlive-source)
at commit [`52dac44dde13a8310bbb8231e81c58be28cebe20`](https://github.com/TeX-Live/texlive-source/commit/52dac44dde13a8310bbb8231e81c58be28cebe20),
committed on 2026-07-03 at 20:01:24 UTC. Its `git-svn-id` records
`svn://tug.org/texlive/trunk/Build/source@79575` and repository UUID
`c570f23f-e606-0410-a88d-b1316a301751`.
This is a development-source snapshot, not an inferred TeX Live release tag.
The snapshot's version headers identify pdfTeX 1.40.29 and e-TeX 2.6.

Tekai commit `6cf41d1c3bc5cd3950a49e8bc609f8538d4180ac`, dated 2026-07-05,
introduced `crates/pdftex-rust`, the historical `scripts/pdftex_port.py`, and
the `third_party/texlive-source` gitlink to that exact upstream commit.
The script configured and built upstream pdfTeX, then used C2Rust with LLVM 16
to translate the generated `pdftexini.c`, `pdftex0.c`, and `pdftex-pool.c`.
The crate's historical README documented that conversion.

Commit `079a34d0889e1ccba17c3e6011aee9e9eefafde6` added the translated
entry wrapper and PDF backend. Commit
`f339df7470076b3a6ab8de1397b58477250d1b6e` added handwritten Rust PDF
inclusion and support adaptations. Both are dated 2026-07-05.
The gitlink stayed at the recorded revision until commit
`3030bde573c5ee18ab863990c0225da0fe9d380f` removed the legacy source checkout
and conversion tooling. Later commits renamed the crate to `tekai-engine`,
changed its state ownership and interfaces, and rewrote support code.
The Rust files are modified derivatives, not unmodified upstream files.
Licensing and provenance notices were restored on 2026-10-13.

## Core source mapping

All upstream paths below are relative to the recorded TeX Live source root.

| Current Rust source | Upstream origin |
| --- | --- |
| `src/generated/pdftex0.rs` | Build-generated `pdftex0.c`, from `texk/web2c/pdftexdir/pdftex.web` and its change files |
| `src/generated/pdftexini.rs` | Build-generated `pdftexini.c`, from the same WEB sources |
| `src/generated/pdftex_pool.rs` | Build-generated `pdftex-pool.c`, from the same WEB string pool |
| `src/generated/pdftexextra.rs` | `texk/web2c/pdftexdir/pdftexextra.c`, its included `texk/web2c/lib/texmfmp.c`, and included engine headers |
| `src/core_types.rs` | Shared TeX record definitions extracted from the translated core modules in Tekai commit `1079306dc975e310b5d15e1f775ed4fc164e72e4` |

The original [pdfTeX license grant](https://github.com/TeX-Live/texlive-source/blob/52dac44dde13a8310bbb8231e81c58be28cebe20/texk/web2c/pdftexdir/README)
and the `pdftex.web` and `pdftex.ch` headers permit GPL version 2 or any later
version. Han The Thanh's core copyright covers 1996-2026.
The core incorporates Donald E. Knuth's TeX and Peter Breitenlohner's e-TeX,
including the NTS team's earlier contributions. The exact upstream notices
retain their master-file and naming terms. Tekai is a modified engine and does
not claim to be Knuth's unmodified TeX or the unmodified e-TeX master source.

The upstream build applies Web2C's `tex.ch`, MLTeX changes, encTeX changes,
pool-in-binary changes, and other engine change files listed in
[`texk/web2c/pdftexdir/am/pdftex.am`](https://github.com/TeX-Live/texlive-source/blob/52dac44dde13a8310bbb8231e81c58be28cebe20/texk/web2c/pdftexdir/am/pdftex.am).
MLTeX's Michael J. Ferguson and Bernd Raichle notices remain in the retained
`tex.ch` preamble. Petr Olsak contributed encTeX. `pdftexextra.c` and
`lib/texmfmp.c` state that they are public domain. Their included pdfTeX
headers carry GPL notices, so that public-domain statement does not describe
every declaration in the resulting Rust module.

SyncTeX change files also contributed node fields and hooks to the translated
core. Tekai's handwritten `src/synctex.rs` provides no-op compatibility calls
instead of the upstream C sidecar writer. Jerome Laurens' original permissive
license, warranty disclaimer, and non-promotion clause remain in the notices
for the eight change files listed by upstream
[`texk/web2c/synctexdir/am/synctex.am`](https://github.com/TeX-Live/texlive-source/blob/52dac44dde13a8310bbb8231e81c58be28cebe20/texk/web2c/synctexdir/am/synctex.am).

## PDF backend source mapping

Each file in `src/generated/backend/` maps to the same basename with `.c`
under `texk/web2c/pdftexdir/`. The complete mapping is retained here to keep
the notices attached to specific Rust files.

| Rust basename | Upstream file |
| --- | --- |
| `avl.rs` | `avl.c` |
| `avlstuff.rs` | `avlstuff.c` |
| `epdf.rs` | `epdf.c` |
| `mapfile.rs` | `mapfile.c` |
| `pkin.rs` | `pkin.c` |
| `subfont.rs` | `subfont.c` |
| `tounicode.rs` | `tounicode.c` |
| `vfpacket.rs` | `vfpacket.c` |
| `writeenc.rs` | `writeenc.c` |
| `writefont.rs` | `writefont.c` |
| `writeimg.rs` | `writeimg.c` |
| `writejbig2.rs` | `writejbig2.c` |
| `writejpg.rs` | `writejpg.c` |
| `writepng.rs` | `writepng.c` |
| `writet1.rs` | `writet1.c` |
| `writet3.rs` | `writet3.c` |
| `writettf.rs` | `writettf.c`, including `macnames.c` |
| `writezip.rs` | `writezip.c` |

These sources carry GPL version 2 or later grants. The exact per-file
copyright years and notices remain in `licenses/upstream-notices.txt`.
The modified libavl implementation and declarations in `avl.c` and `avl.h`
credit Han The Thanh, the Free Software Foundation, and original author
Ben Pfaff. Their grant is also GPL version 2 or later.
The `pkin.c` preamble credits Tomas Rokicki's dvips code, PKtype, and
Piet Tutelaers' redesign.

`src/backend_types.rs` consolidates records previously repeated in generated
modules. Their upstream declarations include `ptexlib.h`, `image.h`,
`writettf.h`, and `avl.h`. Their original notices remain with the source
mapping. The shared records moved in Tekai commit
`238cb16efb9dcd89bda1fbb49f6ebd517140a8e2`.

## Handwritten adaptations and boundary records

`src/pdftoepdf.rs` is a handwritten Rust adaptation of PDF-inclusion logic in
`texk/web2c/pdftexdir/pdftoepdf.cc`, whose original header credits
Han The Thanh, 1996-2024, under GPL version 2 or later.
`src/utils.rs` combines handwritten Rust support with adapted pdfTeX
algorithms from `texk/web2c/pdftexdir/utils.c`. These include PDF escaping,
subset-tag generation, and color-stack behavior. Its upstream notice credits
Han The Thanh, 1996-2017, under GPL version 2 or later.
This attribution does not classify independently rewritten Rust formatting,
regex, parsing, or operating-system helpers as C2Rust translations.

The shared records in `src/kpathsea_types.rs` retain layouts from
`texk/kpathsea/types.h`, `simpletypes.h`, `hash.h`, `str-list.h`,
`str-llist.h`, and `tex-glyph.h`. The original headers credit Karl Berry
and Olaf Weber and permit LGPL version 2.1 or later. Their exact notices
and license text remain here even though Tekai no longer links native
Kpathsea. The `tex-file.h` notice is also retained for the earlier generated
boundary declarations.

The MD5 state layout originated in `texk/web2c/libmd5/md5.h`. Its Aladdin
Enterprises and L. Peter Deutsch notice permits use and modification under
its stated origin, altered-source, and notice-preservation conditions.
That notice accompanies the adapted state declarations. Tekai's Rust MD5
implementation is not the unmodified upstream C library.

## Source availability and redistribution

Keep this notice, the upstream notices, and the applicable license texts with
source distributions. Preserve the dated modification notices in the Rust
files and the original authors' notices when moving or extracting derived
code. Keep upstream `tex.web` and `etex.ch` master files unchanged under their
original names when redistributing those master sources.

The exact upstream snapshot is available through the commit link above.
Tekai's historical conversion script and import README remain available in
the recorded repository commits. A provenance link is not a replacement for
the GPL's source-distribution requirements. Any compiled distribution must
separately satisfy the applicable complete-source requirements, including
the scripts used to control compilation and installation. This notice does
not assert that a binary source bundle has been prepared.
