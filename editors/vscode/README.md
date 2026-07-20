# Tekai for VS Code

First-party VS Code integration for the `tekai` LaTeX build system.

## Features

- Tekai lint results in the Problems panel on save;
- exact builds and low-latency one-shot previews;
- live `tekai watch --preview` sessions;
- a refreshing PDF preview tab or the system PDF viewer;
- root discovery from `tekai.mainFile`, `% !TEX root = ...`, or a document root;
- workspace-relative `tekai.toml` selection and extra build arguments.

## Install from this checkout

```sh
cd editors/vscode
npm install
npm run compile
npx @vscode/vsce package
code --install-extension tekai-0.2.0.vsix
```

Install the CLI first (`brew install NicoNekoru/tap/tekai`) or set
`tekai.executable` to an absolute development build.

## Commands

Open the Command Palette and run `Tekai: Lint Current File`, `Tekai: Build
PDF`, `Tekai: Build and Open Fast Preview`, or `Tekai: Start Live Preview`.
`Tekai: Stop Live Preview` terminates the watcher. The eye and tools buttons in
the editor title provide the common preview and build actions.

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

Use `tekai.preview.viewer = "external"` if the platform PDF renderer works
better than VS Code's embedded browser renderer.

## Development

```sh
npm run check
npm test
```
