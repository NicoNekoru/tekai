# Changelog

## 0.3.4

- Explain lint-blocked checks with counts, the first issue, and an explicit
  notice that compilation did not run. Show warning-only blocks as warnings.
- Open Problems for blocked checks and include every diagnostic and its help
  text in Tekai output, alongside the process exit code.
- Add an explicit one-time warning override without changing project rules or
  persistent settings. Lint errors still block compilation.
- Preserve CLI stdout and stderr when a report cannot be parsed.

## 0.3.3

- Keep the previous PDF visible when a build deletes or truncates its output.
  Load a complete replacement before discarding the previous document.
- Report compiler file and line errors in Problems when the CLI supplies them.
  Keep compiler diagnostics separate from lint diagnostics.
- Explain missing PDFs without exposing a webview resource 404.

## 0.3.2

- Pass each source file's nearest `tekai.toml` to the linter explicitly, rather
  than relying on its working-directory-only default lookup.
- Group workspace lint by config so separate papers keep their own rules.
- Resolve and log the config path explicitly for builds, checks, and previews.
- Refresh open-file diagnostics when TOML settings are saved, and clear stale
  Problems when a rule is disabled. Keep explicit `tekai.configFile` overrides.
- Add regression tests for inherited, nested, changed, deleted, and invalid
  configs, workspace lint, and relative/absolute config overrides.

## 0.3.1

- Check for missing or empty SyncTeX maps before invoking the CLI and explain
  how to rebuild. Keep verbose CLI help in the output log, not notifications.
- Support both compressed and uncompressed SyncTeX maps.
- Add `tekai.workingDirectory` for projects whose output paths are relative to
  the root TeX document rather than the VS Code workspace.
- Test nested-paper builds and missing-map recovery in the extension host.

## 0.3.0

- Add check-before-build with exact source-graph diagnostics and warning gates.
- Generate SyncTeX in all build modes and add forward/inverse source navigation.
- Replace the iframe with a bundled PDF.js custom editor, including text search,
  selection, links, zoom, page controls, and position-preserving live refresh.
- Add project-aware bibliography and label completion, hover, and definitions.
- Include snippets, outline, command shortcuts,
  watch toggle, installation checks, and workspace-trust guards.
- Defer editing providers to LaTeX Workshop when enabled, while retaining Tekai
  compilation, diagnostics, PDF viewing, and SyncTeX. Reuse VS Code's grammars.
- Document and test a Workshop companion setup with a Tekai-only build recipe.
- Test language providers and real builds in an isolated VS Code extension host.

## 0.2.0

- Added diagnostics from the same structured report used by `tekai check`.
- Added exact builds, fast previews, live previews, and root-document discovery.

## 0.1.0

- Initial lint, build, one-shot preview, and live-preview support.
