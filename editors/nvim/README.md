# tekai.nvim

Neovim diagnostics, builds, and live PDF previews for the `tekai` LaTeX build
system. Neovim 0.10 or newer is required.

## Install

With `lazy.nvim`:

```lua
{
  "NicoNekoru/tekai",
  dir = "/path/to/tekai/editors/nvim", -- while developing from this checkout
  config = function()
    require("tekai").setup({
      main_file = "paper/main.tex", -- optional
      preview = {
        final_after_idle_ms = 1500,
        -- viewer = { "sioyek", "--reuse-window" },
      },
    })
  end,
}
```

For a direct local install, add `editors/nvim` to `runtimepath`. The plugin
loads with useful defaults even when `setup()` is omitted. Install the CLI first
or set `executable` to an absolute development build.

## Commands

| Command | Action |
| --- | --- |
| `:TekaiLint [file]` | Populate `vim.diagnostic` from Tekai JSON output. |
| `:TekaiLintWorkspace [dir]` | Lint a whole project. |
| `:TekaiBuild [main]` | Make an exact build. |
| `:TekaiCheck [main]` | Check the root source graph, replace diagnostics with that exact result, build, and open the PDF. |
| `:TekaiFastPreview [main]` | Make a one-pass fast build and open it. |
| `:TekaiPreview [main]` | Start `tekai watch --preview` and open the first PDF. |
| `:TekaiStop` | Stop live preview. |
| `:TekaiOpen` | Open the last built PDF. |
| `:TekaiLog` | Open the captured Tekai process log. |

By default, lint runs when TeX files open and save. Root discovery checks
`main_file`, `% !TEX root = ...`, the current document, and the nearest
`main.tex`. Disable automation or choose a viewer as needed:

```lua
require("tekai").setup({
  executable = "tekai",
  config_file = "tekai.toml",
  lint = { on_open = true, on_save = true },
  build = { on_save = false, extra_args = {} },
  check = { open_on_success = true, extra_args = {} },
  preview = {
    open_on_start = true,
    final_after_idle_ms = 1500, -- nil disables exact idle builds
    extra_args = {},
    viewer = nil, -- vim.ui.open; or { "zathura" }; or function(pdf) ... end
  },
})
```

Run `:checkhealth tekai` to verify that the CLI is available.

## Development

```sh
cd editors/nvim
nvim --headless -u NONE -l tests/run.lua
```
