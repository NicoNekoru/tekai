# Tekai for VS Code

LaTeX diagnostics, builds, live preview, and PDF navigation powered by Tekai.
Use it alone or alongside LaTeX Workshop's editing tools. No latexmk or language
server is required.

## Features

- Lint diagnostics on open/save and an explicit check-before-build command.
- Exact builds, fast one-shot builds, and live previews with exact idle builds.
- Bundled PDF.js preview with selectable text, links, search, zoom, page controls,
  automatic refresh, and retained scroll position. No CDN or local HTTP server.
- Forward SyncTeX from the cursor and inverse SyncTeX by double-clicking the PDF.
- Project-aware citation completion for BibTeX and BibLaTeX. Search by key,
  author, year, or title; accepting a result inserts only the citation key.
- Label completion, citation/reference hover and Go to Definition, section
  outline, paired-environment snippets, and common LaTeX snippets. Syntax
  highlighting comes from VS Code's built-in LaTeX and BibTeX grammars.
- Root discovery from `tekai.mainFile`, `% !TEX root`, the current document,
  the nearest `main.tex`, or an unambiguous workspace root.

The citation index follows `\input`, `\include`, `\subfile`, `\bibliography`,
and BibLaTeX resource declarations. It includes unsaved TeX and bibliography
buffers, handles nested BibTeX fields and string concatenation, and never mixes
in unrelated workspace bibliographies. Inline `\bibitem` entries also work.
This is a static source index, not a TeX macro interpreter; macro-generated paths
and bibliography keys require literal declarations to be indexed.

`tekai.languageFeatures` defaults to `auto`. If LaTeX Workshop is enabled, Tekai
leaves all completions, snippets, hover, definitions, and outlines to Workshop.
Without Workshop, Tekai supplies its own. Set `native` to force Tekai's providers
or `off` to leave editing to another extension. Builds, diagnostics, PDF viewing,
and SyncTeX work in every mode.

## Install from this checkout

```sh
cd editors/vscode
npm ci
npm run package
code --install-extension tekai-0.3.4.vsix --force
```

Install the CLI first (`brew install NicoNekoru/tap/tekai`) or set
`tekai.executable` to an absolute development build. SyncTeX navigation also needs
the `synctex` command-line utility, available with TeX Live or as a standalone
build from [the SyncTeX project](https://github.com/jlaurens/synctex).
Set `tekai.synctex.executable` if it is not on VS Code's PATH. On macOS,
`/Library/TeX/texbin/synctex` is detected automatically. PDF viewing and
completion work without this utility.

Set `workbench.editorAssociations["*.pdf"]` to `"tekai.pdfPreview"` if you want
Tekai to open PDFs by default. Texpresso is not needed for this workflow.

## Using LaTeX Workshop alongside Tekai

Workshop supplies more extensive package-aware command completion, editing
tools, and bibliography support. Tekai still owns checks, live compilation,
diagnostics, and source/PDF navigation. This is similar to using VimTeX with the
Neovim integration. Keep the native Tekai PDF viewer: it uses the PDF path
reported by the compiler, including custom output directories, without needing
to keep a second extension's path settings in sync.

Disable Workshop's automatic compiler and linters, and replace its recipes so
even an explicit Workshop build uses Tekai:

```json
{
  "tekai.languageFeatures": "auto",
  "latex-workshop.latex.autoBuild.run": "never",
  "latex-workshop.linting.chktex.enabled": false,
  "latex-workshop.linting.lacheck.enabled": false,
  "latex-workshop.latex.autoClean.run": "never",
  "latex-workshop.latex.autoBuild.cleanAndRetry.enabled": false,
  "latex-workshop.latex.build.enableMagicComments": false,
  "latex-workshop.latex.build.fromFolder": ".",
  "latex-workshop.latex.recipe.default": "first",
  "latex-workshop.latex.recipes": [{ "name": "Tekai", "tools": ["tekai"] }],
  "latex-workshop.latex.tools": [{
    "name": "tekai",
    "command": "tekai",
    "args": ["build", "%DOC_EXT%", "--synctex"]
  }],
  "workbench.editorAssociations": { "*.pdf": "tekai.pdfPreview" }
}
```

Use an absolute executable path if needed. Disabling Workshop magic comments
prevents `% !TEX program` from selecting another compiler, but also disables
Workshop's `% !TEX root` handling. Tekai's commands still honor root comments;
use them for multi-root projects, custom `tekai.mainFile`/`configFile`, and strict
checks. The fallback Workshop recipe discovers its own root and does not read
Tekai extension settings. Both discover the nearest `tekai.toml` above the root
document. Relative output paths are resolved from the process working directory.

For a nested paper that normally builds from its own directory, set these in
the workspace settings and match Workshop's output path to `[build].out_dir`:

```json
{
  "tekai.workingDirectory": "document",
  "latex-workshop.latex.build.fromFolder": "",
  "latex-workshop.latex.outDir": "%DIR%/output/pdf"
}
```

The default `tekai.workingDirectory` is `workspace` for compatibility. This
setting affects builds, checks, and live preview, not root/config-file paths.

Use Tekai's build, preview, and SyncTeX buttons. If both extensions compete for
shortcuts, add these to your user `keybindings.json` on macOS, replacing `cmd`
with `ctrl` on Windows/Linux:

```json
[
  { "key": "cmd+alt+b", "command": "tekai.build", "when": "editorLangId == latex || editorLangId == tex" },
  { "key": "cmd+alt+c", "command": "tekai.check", "when": "editorLangId == latex || editorLangId == tex" },
  { "key": "cmd+alt+j", "command": "tekai.forwardSync", "when": "editorLangId == latex || editorLangId == tex" },
  { "key": "cmd+alt+w", "command": "tekai.toggleWatch", "when": "editorLangId == latex || editorLangId == tex" },
  { "key": "cmd+alt+v", "command": "tekai.openPdf", "when": "editorLangId == latex || editorLangId == tex" }
]
```

## Commands

### Project configuration

Without a `tekai.configFile` override, linting searches upward from each source
file for the nearest `tekai.toml`. Workspace lint groups files by their config;
one paper's rules do not spill into another. Builds, checks, fast preview, and
watch use the nearest config above the root TeX document. Each command passes
the selected path explicitly as `--config`, visible along with its working
directory in Tekai output. Nearest configs replace parent configs, not merge.

`tekai.configFile` takes precedence and accepts an absolute path or a path
relative to the containing workspace. Configs are read afresh on each command.
Saving a TOML file refreshes lint diagnostics for open TeX files unless lint is
manual. A running watch process must be restarted to pick up config edits.
Invalid configs report an error instead of silently falling back to defaults.

Compiler/linter settings come from TOML; VS Code still controls editor behavior
and explicit command flags. In particular, `tekai.synctex.enabled` adds
`--synctex`, fast preview adds `--once --fast`, and `*.extraArgs` take precedence
over TOML. These are deliberate command overrides; other build and lint policy
stays in TOML.

### Available commands

| Command | macOS shortcut | Windows/Linux shortcut |
| --- | --- | --- |
| `Tekai: Build PDF` | Cmd+Alt+B | Ctrl+Alt+B |
| `Tekai: Check, Build, and Open PDF` | Cmd+Alt+C | Ctrl+Alt+C |
| `Tekai: SyncTeX: Show Cursor in PDF` | Cmd+Alt+J | Ctrl+Alt+J |
| `Tekai: Toggle Live Preview` | Cmd+Alt+W | Ctrl+Alt+W |

The Command Palette also includes lint-file/workspace, fast preview, start/stop
watching, open last PDF, show output, and check installation. Build, live preview,
and forward-sync buttons appear in the TeX editor title.

Explicit builds save dirty TeX/BibTeX sources in the current workspace. Checks
replace Tekai's diagnostics with the exact source graph that gated the build.
Warnings block checks by default. A blocked check opens Problems and reports
the counts, first issue, and the fact that compilation did not run. Tekai output
lists all diagnostics with their source locations and help text.
Choose **Build with Warnings Once** in the warning notification or run
`Tekai: Check and Build with Warnings Once` to allow warnings for that build.
The next ordinary check remains strict, and lint errors always block checks.
For a persistent override, set `tekai.check.extraArgs: ["--allow-warnings"]`.
Live preview permits warnings.
All build modes emit SyncTeX maps unless `tekai.synctex.enabled` is false.
An older PDF may have no map. Open its root TeX file and run `Tekai: Build PDF`,
then navigate in the PDF opened by that build. Forward sync rebuilds a missing
map automatically; inverse sync cannot safely infer the source of an arbitrary
PDF and instead explains how to rebuild. Compressed and plain SyncTeX maps work.

Forward sync opens the embedded preview even when the normal viewer is external.
Double-click a PDF location to focus its source line. Cmd/Ctrl+F searches PDF
text; Enter and Shift+Enter move between matches. PDFs refresh after rebuilds
without resetting zoom and scroll. Remembered root/PDF pairs survive window
reloads and respect custom output paths returned by Tekai.

Failed reloads keep the previous PDF visible, with a notice that it is stale.
The viewer reads a complete snapshot before replacing it, so a failed build
cannot break later page loads in the previous PDF.

Compiler errors appear in Problems with their source file and line when using
the CLI built from this checkout. Its quiet/JSON failure output includes the
first TeX error and log path. Older CLI builds only provide a generic exit code.
Successful builds clear compiler errors without clearing unrelated lint results.

For multi-file documents, either set:

```json
{
  "tekai.mainFile": "paper/main.tex"
}
```

or put this near the top of an included TeX file:

```tex
% !TEX root = ../main.tex
```

Use `tekai.preview.viewer = "external"` to open normal builds in the system
viewer. Embedded SyncTeX navigation still uses the Tekai tab. Restricted Mode
permits completion and PDF viewing but never launches compiler or SyncTeX
processes.

## Source formatting

For safe lint fixes without a build, save your buffers and run
`tekai format FILE --config path/to/tekai.toml` in a terminal.
Use `--check` for a read-only check. This CLI command is separate from VS Code's
Format Document action and does not add a formatting provider. See
[the formatting reference](../../docs/usage.md#formatting) for supported fixes
and exit policy.

## Development

```sh
npm run check
npm test
npm run test:integration
```

The integration runner uses an isolated temporary workspace and VS Code profile.
Set `VSCODE_EXECUTABLE` to an existing VS Code executable to avoid downloading
a test editor. Set both `TEKAI_EXECUTABLE` and `SYNCTEX_EXECUTABLE` to absolute
paths to also test strict check diagnostics, a real build, a rendered PDF,
source/PDF round trips, and watch startup/shutdown.
Set `WORKSHOP_EXTENSION_PATH` to an installed Workshop extension directory for a
second run with both extensions enabled. It checks that providers do not
duplicate each other and that Workshop's Tekai recipe builds with SyncTeX.
