# Editor integrations

Tekai includes independently packageable integrations for VS Code and Neovim.
They invoke the installed `tekai` executable rather than embedding a second
compiler or linter, so editor, terminal, and CI behavior all use the same
`tekai.toml` policy.

| Editor | Package | Diagnostics | Preview |
| --- | --- | --- | --- |
| VS Code | [`editors/vscode`](../editors/vscode/README.md) | Problems panel on save or command | Refreshing editor tab or system viewer |
| Neovim | [`editors/nvim`](../editors/nvim/README.md) | `vim.diagnostic` on open/save or command | `vim.ui.open` or configured viewer |

## Root documents

Both integrations resolve a multi-file document in this order:

1. the editor-specific `mainFile`/`main_file` setting;
2. a magic comment in the current file;
3. the current file when it contains a document preamble;
4. a discoverable `main.tex` or unambiguous document root.

The shared magic-comment form is:

```tex
% !TEX root = ../main.tex
```

Set the root explicitly when a workspace contains several independent papers.

## CLI contract

Lint clients run:

```sh
tekai lint FILE --report-json --allow-warnings
```

Check commands use `tekai check MAIN --report-json`; its diagnostic array is the
exact source graph and policy that gated the accompanying build. Editors should
replace their Tekai diagnostic namespace from that array so annotations cannot
drift from the explicit check result.

Build clients consume `pdf_path` from `tekai build MAIN --report-json`. Live
preview uses the CLI's own watcher and dependency graph:

```sh
tekai watch MAIN --preview --allow-warnings --final-after-idle-ms 1500
```

The editor only owns process lifecycle, diagnostics presentation, root
selection, and PDF opening. Tekai continues to own build modes, caching,
dependencies, auxiliary tools, and lint policy.
