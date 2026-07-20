vim.opt.runtimepath:prepend(vim.fn.getcwd())

local protocol = require("tekai.protocol")
assert(type(require("tekai.health").check) == "function", "health module did not load")

local function equal(actual, expected, label)
  if actual ~= expected then
    error(("%s: expected %s, got %s"):format(label, vim.inspect(expected), vim.inspect(actual)))
  end
end

equal(protocol.magic_root("% !TEX root = ../main.tex\nChapter"), "../main.tex", "plain magic root")
equal(protocol.magic_root("% !TeX root = 'paper.tex'"), "paper.tex", "quoted magic root")
equal(protocol.built_pdf("built /tmp/my paper/build/main.pdf in 12.3ms"), "/tmp/my paper/build/main.pdf", "watch PDF")

local lint, lint_err = protocol.decode_lint(vim.json.encode({
  diagnostics = {
    {
      path = "paper.tex",
      line = 2,
      column = 4,
      severity = "warning",
      rule = "math/inline-dollar",
      message = "prefer paren math",
    },
  },
  error_count = 0,
  warning_count = 1,
}))
assert(lint, lint_err)
equal(lint.diagnostics[1].line, 2, "lint line")

local build, build_err = protocol.decode_build('{"pdf_path":"build/main.pdf","elapsed_ms":8.5,"skipped":false}')
assert(build, build_err)
equal(build.pdf_path, "build/main.pdf", "build PDF")

local check, check_err = protocol.decode_check(vim.json.encode({
  diagnostics = lint.diagnostics,
  error_count = 0,
  warning_count = 1,
  pdf_path = "build/main.pdf",
  elapsed_ms = 9.5,
  skipped = false,
}))
assert(check, check_err)
equal(check.diagnostics[1].rule, "math/inline-dollar", "check diagnostic")
equal(check.pdf_path, "build/main.pdf", "check PDF")

local blocked_check, blocked_check_err = protocol.decode_check(vim.json.encode({
  diagnostics = lint.diagnostics,
  error_count = 1,
  warning_count = 0,
}))
assert(blocked_check, blocked_check_err)
equal(blocked_check.elapsed_ms, nil, "blocked check has no build")

require("tekai").setup({ lint = { on_open = false, on_save = false } })
equal(vim.fn.exists(":TekaiBuild"), 2, "build command")
equal(vim.fn.exists(":TekaiCheck"), 2, "check command")
equal(vim.fn.exists(":TekaiPreview"), 2, "preview command")

local executable = vim.env.TEKAI_TEST_EXECUTABLE
if executable and executable ~= "" then
  local fixture = vim.fs.normalize(vim.fs.joinpath(vim.fn.getcwd(), "..", "..", "examples", "arXiv-2511.08544v3", "content", "abstract.tex"))
  vim.cmd.edit(vim.fn.fnameescape(fixture))
  local tekai = require("tekai").setup({
    executable = executable,
    lint = { on_open = false, on_save = false },
  })
  local discovered, root_err = require("tekai.root").resolve(0)
  assert(discovered, root_err)
  equal(vim.fs.basename(discovered), "main.tex", "discovered root")

  tekai.lint(fixture)
  local completed = vim.wait(10000, function()
    return next(tekai._state.lint_jobs) == nil
  end, 10)
  assert(completed, "timed out waiting for Tekai lint")
  local diagnostics = vim.diagnostic.get(0, { namespace = tekai.namespace })
  assert(#diagnostics > 0, "expected real Tekai diagnostics")
  equal(diagnostics[1].source, "tekai", "diagnostic source")

  local minimal = vim.fs.normalize(vim.fs.joinpath(vim.fn.getcwd(), "..", "..", "examples", "minimal.tex"))
  tekai.build(minimal, { fast = true })
  completed = vim.wait(30000, function()
    return tekai._state.build_job == nil
  end, 10)
  assert(completed, "timed out waiting for Tekai build")
  assert(tekai._state.last_pdf and vim.uv.fs_stat(tekai._state.last_pdf), "expected a built PDF")

  tekai.check(minimal, { open = false })
  completed = vim.wait(30000, function()
    return tekai._state.build_job == nil
  end, 10)
  assert(completed, "timed out waiting for Tekai check")
  assert(tekai._state.last_pdf and vim.uv.fs_stat(tekai._state.last_pdf), "expected a checked PDF")

  tekai._state.last_pdf = nil
  require("tekai.config").options.preview.open_on_start = false
  require("tekai.config").options.preview.final_after_idle_ms = nil
  tekai.preview(minimal)
  completed = vim.wait(30000, function()
    return tekai._state.last_pdf ~= nil
  end, 10)
  assert(completed, "timed out waiting for Tekai live preview")
  tekai.stop(false)
end

print("tekai.nvim tests passed")
