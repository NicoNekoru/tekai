# Usage and configuration

This is the user-facing reference for `tekai`. Run `tekai <command>
--help` for the exact flags supported by the installed binary.
Every subcommand also supports the equivalent `tekai help <command>` form.

## Running the CLI

Install the current release from the public Homebrew tap:

```sh
brew tap NicoNekoru/tap
brew trust --formula NicoNekoru/tap/tekai
brew install tekai
tekai --version
```

Once the tap and formula are trusted, use the short name for installs and
upgrades: `brew install tekai` and `brew upgrade tekai`. On Homebrew versions
without `brew trust`, use `brew install NicoNekoru/tap/tekai` for the first
installation. The existing `NicoNekoru/tap` repository remains the package
source; no dedicated `NicoNekoru/tekai` tap is needed.

The 0.4.0 release supports macOS. Linux is not currently a supported target.

For development from a checkout:

```sh
cargo build --release --locked
target/release/tekai --help
```

Use `cargo run -- <command>` while developing the CLI itself.

The default `tekai-engine` path needs no TeX Live or MacTeX installation.
The executable embeds a pinned set of packages, fonts, maps, the LaTeX format,
and a BibTeX implementation. It extracts its bundled data into the engine
cache on first use, without network access or a helper program. File lookup
uses project files, explicit search paths, and the bundle, never `kpsewhich`
or automatic discovery of system TeX trees. See the
[package manifest](../runtime/packages.lock.json) for the bundled set.

This is not every CTAN package or every TeX auxiliary program. Supply extra
package files through `TEXINPUTS`. Unsupported auxiliary workflows fail by
default; `--external-tools` explicitly enables installed compatibility tools.

## Commands

| Command | Behavior |
| --- | --- |
| `init [PATH]` | Create a complete default config at `PATH` (default `tekai.toml`); use `--force` to replace one. |
| `build MAIN` | Compile a root TeX document. |
| `check MAIN` | Lint `MAIN` and its referenced TeX source graph, then build if lint passes; add `--fix` to apply safe fixes first. |
| `watch MAIN` | Watch relevant source/dependency files and rebuild. |
| `lint [PATH ...]` | Lint files or directories; defaults to the current directory. |
| `format [PATH ...]` | Apply safe lint fixes without compiling. Add `--check` to report needed changes without writing. Defaults to the current directory. |
| `clean` | Safely remove the configured output directory. |

`build`, `check`, and `watch` share the build flags. `check`, `watch`, `lint`,
and `format` also accept `--allow-warnings` or `--fail-on-warnings`. Warnings
fail by default. `--allow-warnings` is the convenient interactive setting.

## Final builds

The default command is the exact, converged build path:

```sh
tekai build paper/main.tex
```

Its effective defaults are:

- engine: `tekai-engine`;
- runner: `direct`;
- bibliography policy: `auto`;
- output directory: `build`;
- draft-prepass policy: `auto`;
- maximum TeX runs: `8`.

The direct runner stops only when TeX and supported auxiliary outputs settle.
If `--max-runs` is exhausted, it returns an error and does not publish a
successful cache state.

Useful final-build flags:

```sh
# Bypass settled-input caches.
tekai build paper/main.tex --force

# Enable SyncTeX or shell escape when the document requires it.
tekai build paper/main.tex --synctex
tekai build paper/main.tex --shell-escape

# Cache a compatible mylatexformat preamble dump.
tekai build paper/main.tex --precompile-preamble

# Select bibliography handling explicitly.
tekai build paper/main.tex --bib bibtex
tekai build paper/main.tex --bib biber --external-tools
tekai build paper/main.tex --bib none

# Choose a single-file output job name.
tekai build paper/main.tex --job-name camera-ready
```

`--job-name` must be one filename component because PDF, auxiliary, and cache
files share that key.

## Preview builds

Preview flags trade completeness or visible fidelity for latency:

```sh
# One TeX pass; no bibliography or reference convergence.
tekai build paper/main.tex --once

# Replace expensive graphics/external content with placeholders.
tekai build paper/main.tex --fast

# Fastest standalone preview.
tekai build paper/main.tex --once --fast
```

`--no-images` is an alias for `--fast`. Preview mode can replace graphics,
included PDFs, SVGs, animation frames, attachments, media, externalized TikZ,
minted/inputminted content, and similar expensive imports. Do not use preview
output as a final artifact.

`--draft-prepass auto` is different: it accelerates intermediate convergence
passes while still producing an exact final PDF. Use `always` or `never` only
when explicitly controlling that scheduler policy.

## Watch and live preview

Ordinary watch rebuilds the configured final mode:

```sh
tekai watch paper/main.tex --allow-warnings
```

The low-latency edit loop is:

```sh
tekai watch paper/main.tex --preview --allow-warnings
```

`--preview` performs an initial whole-document fast build, prewarms a focused
hot-preview document, and then compiles a small source slice for ordinary body
edits. Root-preamble, package/class, bibliography, image, mixed, and other
structural changes fall back to a whole-document preview. The focused preview
PDF is intentionally not the final document.

To get both immediate feedback and an exact settled artifact:

```sh
tekai watch paper/main.tex \
  --preview \
  --final-after-idle-ms 1500 \
  --allow-warnings
```

The final build runs after the relevant file stream has been quiet for the
configured interval. Use `--root DIR` when the watched tree is not the root
document's parent. Use `--no-lint` only when another tool already owns linting.

Watch mode follows source-scanned and recorder-discovered dependencies,
including dependencies outside the project root. It ignores the configured
output directory, `.git`, `target`, and `.tekai` trees.

## Engines and runners

| Selection | Execution and fidelity |
| --- | --- |
| `--engine tekai-engine --runner direct` | Self-contained Tekai engine and scheduler. This is the default exact path. |
| `--engine tekai-engine --runner latexmk` | Installed `latexmk` and system pdfLaTeX. Useful as a baseline. |
| `--engine xe-latex` | Installed XeLaTeX; direct scheduling or `latexmk` as selected. |
| `--engine lua-latex` | Installed LuaLaTeX; direct scheduling or `latexmk` as selected. |
| `--engine tectonic` | Installed Tectonic. |
| `--engine tekai-pdftex` | Experimental approximate native renderer. Unsupported documents fall back to exact pdfTeX. |
| `--engine tekai-pdftex-certified` | Native diagnostic run followed by an exact pdfTeX final artifact. |

The experimental native renderer does not yet claim general pixel parity. See
the [divergence audit](../output/pdf/pdftex-native-divergence-audit.md).

## Configuration

`build`, `check`, and `watch` find the nearest `tekai.toml` at or above the root
document's directory. This also applies when the document is passed from a
parent directory, so `check --fix` uses the same lint policy as `check`. `lint`,
`format`, and `clean` use `./tekai.toml` by default. Directory formatting uses
that one lint config for every selected file, including files in subdirectories.
Pass `--config paper/tekai.toml` when formatting a paper from its parent directory.
All commands except `init` accept `--config PATH`. An explicit path takes
precedence. Explicit CLI build flags override configuration. Omitted flags
retain configured values.

Initialize a documented config containing every effective default with:

```sh
tekai init
# Or choose a path; existing files are preserved unless --force is explicit.
tekai init config/tekai.toml
```

```toml
[build]
engine = "tekai-engine"
runner = "direct"
bib = "auto"
out_dir = "build"
job_name = "paper"
fast = false
draft_prepass = "auto"
once = false
max_runs = 8
force = false
precompile_preamble = false
synctex = false
shell_escape = false
external_tools = false
quiet = false
print_command = false

[build.env]
TEXINPUTS = "tex//:"
BIBINPUTS = "bib//:"
BSTINPUTS = "bst//:"
INDEXSTYLE = "styles//:"

[lint]
indent_size = 2
indent_style = "spaces" # or "tabs"
indent_environments = true
indent_display_math = true
ignored_indent_environments = ["document"]
prefer_paren_inline_math = true
prefer_bracket_display_math = true
prefer_prime_command = false
check_environment_stack = true
max_line_length = 120
# prose_wrap = "unwrapped" # or "hardwrap"; omitted is neutral

[lint.rules]
"math/inline-dollar" = "error"
"math/prime-command" = "warn"
"line/length" = "off"
```

The exact engine is named `tekai-engine`. Other accepted pairs are
`xelatex`/`xe-latex` and `lualatex`/`lua-latex`. `bibliography` is retained as
an alias for `bib`; do not set both. `no_images` is retained as an alias for
`fast`; if both are present they must agree.

`[build.env]` is applied before engine, watcher, and auxiliary-tool work. It is
the right place for checked-in Kpathsea roots such as `TEXINPUTS`, `BIBINPUTS`,
`BSTINPUTS`, `INDEXSTYLE`, and `TEXINDEXSTYLE`.

## Cache and output behavior

Direct builds write artifacts under `out_dir`, including a
`.tekai-<job>.state.toml` dependency state. If mode, output, environment,
and effective inputs are unchanged, the next build skips TeX.

The cache is TeX-aware:

- metadata is the common fast path;
- unchanged content survives harmless mtime-only touches;
- ordinary TeX comment text, trailing physical spaces, and content after
  effective `\end{document}`/`\endinput` boundaries can remain cache hits;
- catcode-sensitive or verbatim-like inputs use conservative fingerprints;
- bibliography and auxiliary-tool inputs are tracked separately.

Use `--force` to bypass the settled cache. Use `clean --dry-run` before removal
when checking which directory a config selects:

```sh
tekai clean --dry-run
tekai clean
```

`clean` refuses empty paths, files, symlinks, the current directory, and its
ancestors.

Global reusable caches default to the platform cache directory under `tekai`.
Advanced users can override individual roots with `TEKAI_FORMAT_CACHE`,
`TEKAI_AUX_CACHE`, `TEKAI_BIBTEX_CACHE`, and `TEKAI_ENGINE_CACHE`.

## JSON output

```sh
tekai build paper/main.tex --report-json
tekai check paper/main.tex --report-json --allow-warnings
tekai lint paper --report-json --allow-warnings
tekai format paper --report-json --allow-warnings
tekai format paper --check --report-json
tekai clean --dry-run --report-json
```

Build reports include cache status, PDF path, total/draft/final/PDF-producing
TeX runs, per-pass timing and rerun reasons, bibliography/index/external runs,
and preflight/preamble-format usage. `check --report-json` always includes the
lint diagnostics and counts that gated the build. When lint passes, the same
object is augmented with the normal build-report fields; when lint blocks the
build, it exits with status 1 and omits those build fields.

Format reports include `check`, `fixes_applied`, `files_changed`,
`fixes_available`, and `files_would_change`, along with the same `diagnostics`,
`error_count`, and `warning_count` fields as lint reports. Normal formatting
reports applied edits and changed paths, with zero available fixes and an empty
would-change list. Check mode reports available edits and would-change paths,
with zero applied fixes and an empty changed-file list. Diagnostics describe
the updated sources in normal mode and the unchanged sources in check mode.
JSON is the only stdout output in either mode, including when lint policy or
needed formatting changes return exit code 1. File and config errors go to
stderr and may prevent a JSON report.

## Formatting

```sh
# Format all supported sources below the current directory.
tekai format

# Format a directory using its project rules, without building a PDF.
tekai format paper --config paper/tekai.toml

# Format selected files only.
tekai format main.tex chapters/intro.tex

# Read-only CI gate for required edits and lint diagnostics.
tekai format paper --check --config paper/tekai.toml

# Apply safe fixes while accepting any remaining lint warnings.
tekai format paper --allow-warnings
```

`format` uses the existing lint fixer. It repairs dollar-math delimiters,
indentation style, and environment/display-math indentation. It preserves
valid braced continuation indentation, Unicode text, line endings, and the
presence or absence of a final newline. Disabled rules, suppression comments,
and verbatim content are respected. Prose wrapping, long lines, prime notation,
and structurally ambiguous math are reported without automatic rewrites.
Formatting never invokes TeX or other build tools.

File selection matches `lint`. It scans `.tex`, `.ltx`, and `.cls` extensions
case-insensitively and recursively visits directory arguments. It skips
`.git`, `target`, `build`, `.latexmk`, and `.tekai` directories, and leaves
`.sty` and other file types alone. Overlapping targets are processed once.
An explicitly named supported file is processed even inside an otherwise
ignored directory. A file argument formats that file only, without following
`\input` or `\include`. Use a directory to include its chapters, or use
`check MAIN --fix` to fix only a root document's referenced source graph.

| Option | Behavior |
| --- | --- |
| `--config PATH` | Use this lint config instead of `./tekai.toml`. |
| `--check` | Compute safe fixes and list files that would change, without writing. |
| `--report-json` | Emit the format report and lint diagnostics as JSON. |
| `-q`, `--quiet` | Suppress text summaries and diagnostics. JSON reports and file/config errors are still emitted. |
| `--allow-warnings` | Accept remaining lint warnings. Errors still fail. |
| `--fail-on-warnings` | Fail on lint warnings, which is already the default. Conflicts with `--allow-warnings`. |

After applying safe fixes, normal mode lints the updated sources. It exits
with status 0 when those sources pass lint policy, even if files changed.
Remaining errors or disallowed warnings return status 1. Fixes stay applied
when other diagnostics remain. Files are written in place, so an error on a
later file can leave earlier files formatted.

`--check` returns status 1 if any file would change, regardless of
`--allow-warnings`. It also applies lint policy to the unchanged sources.
It returns status 0 only when no fixes are needed and diagnostics pass that
policy. Invalid arguments return status 2. No formatting-specific TOML section
is needed, since both formatting modes use `[lint]` and `[lint.rules]`.

## Linting

```sh
tekai lint paper --allow-warnings
tekai check paper/main.tex --allow-warnings
tekai check paper/main.tex --fix
tekai check paper/main.tex --fix --allow-warnings
```

`lint` is read-only and scans `.tex`, `.ltx`, and `.cls` files. Package `.sty`
files remain build and watch dependencies but are not lint targets.

`check MAIN` does not sweep `MAIN`'s parent directory. It always lints the
explicit root, follows the TeX sources referenced by that document, and ignores
unreferenced sibling files. Project build environment such as `TEXINPUTS` is
applied before resolving this source graph.

`check --fix` rewrites deterministic, safe fixes, lints the updated sources,
and builds only when the remaining diagnostics pass. It currently fixes
dollar-math delimiters, indentation style, and environment/display-math
indentation. It does not rewrite prose, long lines,
prime notation, or structurally ambiguous math. Suppression comments and disabled
rules are respected. If non-fixable warnings remain, pass `--allow-warnings` to
continue to the build. Use `format` for the same repairs without a build.

Rule identifiers currently include:

- `math/inline-dollar`, `math/display-dollar`, `math/mixed-delimiters`,
  `math/nested`, `math/prime-command`, `math/left-right`, `math/unclosed`,
  `math/unmatched-paren`, `math/unmatched-bracket`,
  `math/unclosed-environment`, and `math/unmatched-environment`;
- `env/mismatch`, `env/unclosed`, and `env/unmatched-end`;
- `indent/size`, `indent/spaces`, `indent/tabs`, `line/length`, and
  `prose/wrap`.

Set `indent_style = "spaces"` (the default) to use `indent_size` spaces per
environment level, or set `indent_style = "tabs"` to require one tab per level.
In tab mode, `indent_size` is the visual width used when `format` or
`check --fix` converts existing space indentation.

Multiline braced arguments such as `\hypersetup{...}` may use up to one extra
indentation level per open brace. Continuation indentation is optional, so
unindented macro bodies remain valid. Leading closing braces remove their
continuation levels. Comments, escaped braces, and verbatim content do not affect
brace depth. Environment and display-math indentation still apply.

Set `prose_wrap = "hardwrap"` to require prose source lines to stay within
`max_line_length`. Set it to `"unwrapped"` to require one physical source line
per prose paragraph; prose is then exempt from `line/length`. If `prose_wrap`
is omitted, the linter preserves the previous neutral behavior and only applies
the general `line/length` rule. The prose scanner is deliberately conservative:
it ignores command-only lines, environment boundaries, display math, comments,
and verbatim content. Neither prose mode is auto-fixable. `format` and
`check --fix` report violations without reflowing TeX source.

Set a rule to `off`, `warn`, or `error` under `[lint.rules]`. Suppress a specific
source line when needed:

```tex
Text using legacy $x$ syntax. % tekai-ignore-line math/inline-dollar

% tekai-ignore-next-line line/length
This intentionally long generated line is accepted here.
```

Omit rule names after the suppression directive to suppress all diagnostics on
the target line.

## External tools

Ordinary BibTeX runs inside Tekai and uses bundled or project bibliography
styles. It does not launch `bibtex`. PythonTeX cache metadata is decoded as
data in Rust, without launching Python or executing pickle reducers.

Biber, BibTeX variants, index/glossary tools, SVG/EPS converters, Asymptote,
MetaPost, Gnuplot, PythonTeX, and PGF externalization remain optional
compatibility workflows. They require `--external-tools` or
`[build].external_tools = true` before Tekai can launch an installed program.
That opt-in also selects system BibTeX instead of the built-in implementation.
Without it, Tekai reports an unsupported workflow rather than relying on a
program in `PATH` or substituting different output. Shell escape remains a
separate opt-in and can also run external commands.

## Exit status and troubleshooting

`tekai` returns zero only when the requested operation completes under the
selected policy. Lint warnings fail by default, unsettled builds fail after
`--max-runs`, and missing external programs fail when the document needs them.

```sh
tekai build paper/main.tex --print-command --force
tekai build paper/main.tex --report-json > build-report.json
tekai build paper/main.tex --runner latexmk
```

These commands expose executed tools, record scheduler and cache details, and
check the compatibility baseline. Use `--shell-escape` only for trusted input
because it permits TeX packages to run external commands.
