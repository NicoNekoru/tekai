# Usage and configuration

This is the user-facing reference for `tekai`. Run `tekai <command>
--help` for the exact flags supported by the installed binary.
Every subcommand also supports the equivalent `tekai help <command>` form.

## Running the CLI

Install the current release from the public Homebrew tap:

```sh
brew install NicoNekoru/tap/tekai
tekai --version
```

The 0.2.0 release supports macOS. Linux is not currently a supported target.

For development from a checkout:

```sh
cargo build --release --locked
target/release/tekai --help
```

Use `cargo run -- <command>` while developing the CLI itself.

`tekai` requires TeX Live or MacTeX for LaTeX packages, fonts, formats, and
filename databases. Document-specific workflows also require their own tools,
such as Biber, MakeIndex, Inkscape, or PythonTeX. The Homebrew formula installs
the CLI but does not choose a TeX distribution for you.

## Commands

| Command | Behavior |
| --- | --- |
| `init [PATH]` | Create a complete default config at `PATH` (default `tekai.toml`); use `--force` to replace one. |
| `build MAIN` | Compile a root TeX document. |
| `check MAIN` | Lint `MAIN` and its referenced TeX source graph, then build if lint passes; add `--fix` to apply safe fixes first. |
| `watch MAIN` | Watch relevant source/dependency files and rebuild. |
| `lint [PATH ...]` | Lint files or directories; defaults to the current directory. |
| `clean` | Safely remove the configured output directory. |

`build`, `check`, and `watch` share the build flags. `check`, `watch`, and
`lint` also accept `--allow-warnings` or `--fail-on-warnings`. Warnings fail by
default; `--allow-warnings` is the convenient interactive setting.

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
tekai build paper/main.tex --bib biber
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

`build`, `check`, and `watch` load `tekai.toml` from the current directory by
default. All commands accept `--config PATH`. Explicit CLI build flags override
configuration; omitted flags retain configured values.

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
tekai clean --dry-run --report-json
```

Build reports include cache status, PDF path, total/draft/final/PDF-producing
TeX runs, per-pass timing and rerun reasons, bibliography/index/external runs,
and preflight/preamble-format usage. `check --report-json` always includes the
lint diagnostics and counts that gated the build. When lint passes, the same
object is augmented with the normal build-report fields; when lint blocks the
build, it exits with status 1 and omits those build fields.

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

`check --fix` follows the Ruff-style check/fix loop: it rewrites deterministic,
safe fixes, lints the updated sources, and builds only when the remaining
diagnostics pass. It currently fixes dollar-math delimiters, indentation style,
and environment/display-math indentation. It does not rewrite prose, long lines,
prime notation, or structurally ambiguous math. Suppression comments and disabled
rules are respected; if non-fixable warnings remain, pass `--allow-warnings` to
continue to the build.

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
In tab mode, `indent_size` is the visual width used when `check --fix` converts
existing space indentation.

Set `prose_wrap = "hardwrap"` to require prose source lines to stay within
`max_line_length`. Set it to `"unwrapped"` to require one physical source line
per prose paragraph; prose is then exempt from `line/length`. If `prose_wrap`
is omitted, the linter preserves the previous neutral behavior and only applies
the general `line/length` rule. The prose scanner is deliberately conservative:
it ignores command-only lines, environment boundaries, display math, comments,
and verbatim content. Neither prose mode is auto-fixable: `check --fix` reports
violations without reflowing TeX source.

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

The direct runner detects and schedules common BibTeX/Biber, index/glossary,
SVG/EPS, Asymptote, MetaPost, Gnuplot, PythonTeX, minted, and PGF-externalization
workflows. Each workflow still requires its corresponding executable and TeX
package installation. Tests skip optional integrations when their program is
not available; real builds report the missing requirement.

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
