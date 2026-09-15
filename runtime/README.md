# Tekai's bundled TeX data

This is a selected, pinned set of TeX Live packages, not a complete TeX Live
distribution. Tekai embeds `texmf.tar.gz` and `font-outlines.tar.gz` in its
executable. Cargo builds and
normal CLI runs never download these files or run a TeX installer.

`packages.lock.json` records each package's revision, upstream archive URL,
SHA-512 digest, and license metadata. `bundle-id.txt` is the SHA-256 digest of
the two archives concatenated in that order and identifies the extracted
runtime and build caches. The separate archives keep each file below hosting
size limits; neither is fetched at runtime.
Upstream package files are unmodified. The bundle includes package sources and
textual documentation; PDF manuals and documentation images are omitted.
Tekai generates the combined `pdftex.map`, its Adobe-to-URW metric-name aliases,
the filename index, and the regular BBM bitmap fonts described below. Package
authors retain their copyrights and licenses. These files are not covered by
the root project's MIT license. See the license notices inside each package
and `LICENSE.TeX-Live` for the distribution's copying guidance.

The matching LaTeX format is embedded separately from `formats/pdflatex.fmt`.
Its SHA-256 digest in `formats/format-id.txt` also invalidates build caches.
Additional project files can be provided through explicit `TEXINPUTS`,
`BIBINPUTS`, `BSTINPUTS`, and font search paths. Native lookup supports path
lists and recursive `//` entries. It does not invoke `kpsewhich`, scan system
TeX installations, expand arbitrary Kpathsea configuration, or install missing
packages automatically.

The bundle includes the package and font data used by the two large arXiv
fixtures, including LaTeX's first-aid compatibility layer, PSNFSS/URW outlines,
CM-Super outlines for EC/TC encodings, and the dependencies of `newpx`,
`tcolorbox`, and `mdframed`. It also includes `eso-pic` for ICLR page overlays,
alongside `fancyhdr`, `natbib`, and the Times small-cap, bold, and italic fonts.
The `eso-pic` entry records its later database snapshot separately so adding it
does not change the other pinned package revisions or the LaTeX format.

`bitmap-fonts.tar.gz` holds pregenerated regular BBM fonts and is incorporated
into `texmf.tar.gz` by the generator. These fonts cover the standard design
sizes at the resolutions listed in `font-build.lock.json`. Other BBM families
and bitmap resolutions are not generated on demand. No font is silently
replaced with another family.

## Updating the bundle

The maintainer-only generator requires Python with its standard library and
network access, not TeX Live:

```sh
python3 tools/bundle_tex_runtime.py
```

That command reproduces the bundle from the lock, using verified archives
cached in `target/runtime-downloads`. If an upstream archive has changed, it
fails rather than silently accepting a new version. To intentionally select
new package revisions or change the package list:

```sh
python3 tools/bundle_tex_runtime.py --refresh-lock
```

Review package licenses and the lock diff, retain the verified download cache
for reproducibility if the mirror removes older revisions, then run the
dependency-free CLI tests and the PDF fidelity gate. Commit the lock, archive,
and bundle ID together. Updating packages may also require regenerating the
embedded format with the matching LaTeX kernel:

```sh
cargo build --locked
python3 tools/build_tex_format.py
cargo build --locked
```

That format builder runs Tekai's own engine with an empty `PATH`. It does not
use `fmtutil`, `kpsewhich`, or an installed TeX distribution.

To regenerate the regular BBM bitmap data on macOS:

```sh
python3 tools/build_bitmap_fonts.py
python3 tools/bundle_tex_runtime.py
```

The maintainer-only bitmap generator verifies small, pinned Metafont and
GF-to-PK tool archives from `font-build.lock.json`, uses them in a temporary
directory, and checks every generated font's metrics against the upstream
TFM. It installs nothing and ships only the resulting font data. Use
`--refresh-lock` only when intentionally updating those build tools.

`tools/verify_bundled_papers.py` builds both large fixtures with an empty
`PATH`, then compares every rendered page at 144 DPI against a pinned upstream
pdfTeX executable using the same format, package data, and auxiliary files.
The small reference download and Poppler are maintainer verification tools,
not dependencies of Tekai, Cargo builds, or ordinary tests.

## Auxiliary workflows

Ordinary BibTeX uses Tectonic's reusable BibTeX library linked into Tekai with
the same project and bundled file lookup. It does not invoke Tectonic's CLI,
an installed BibTeX, or a network package provider.

Biber, MakeIndex, graphics converters, and other auxiliary programs are not
included. The default build fails explicitly when it needs an unsupported
workflow. `--external-tools` enables installed compatibility tools, including
system BibTeX. External engines, `--runner latexmk`, and `--shell-escape` are
also explicit departures from the dependency-free default.
