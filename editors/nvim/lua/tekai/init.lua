local config = require("tekai.config")
local protocol = require("tekai.protocol")
local root = require("tekai.root")

local M = {}

M.namespace = vim.api.nvim_create_namespace("tekai")
M._state = {
  lint_jobs = {},
  build_job = nil,
  watch_job = nil,
  last_pdf = nil,
  log = {},
}

local function notify(message, level)
  vim.notify(message, level or vim.log.levels.INFO, { title = "Tekai" })
end

local function append_log(text)
  if not text or text == "" then
    return
  end
  for line in (text .. "\n"):gmatch("(.-)\r?\n") do
    if line ~= "" then
      table.insert(M._state.log, line)
    end
  end
  while #M._state.log > 2000 do
    table.remove(M._state.log, 1)
  end
end

local function is_absolute(filename)
  return filename:sub(1, 1) == "/" or filename:match("^%a:[/\\]") ~= nil
end

local function absolute(filename, cwd)
  if is_absolute(filename) then
    return vim.fs.normalize(filename)
  end
  return vim.fs.normalize(vim.fs.joinpath(cwd, filename))
end

local function config_args(cwd)
  local filename = config.options.config_file
  if type(filename) ~= "string" or filename == "" then
    return {}
  end
  return { "--config", absolute(filename, cwd) }
end

local function extend(target, values)
  for _, value in ipairs(values or {}) do
    table.insert(target, tostring(value))
  end
  return target
end

local function command(executable, args)
  local result = { executable }
  return extend(result, args)
end

local function log_command(args)
  local quoted = {}
  for _, arg in ipairs(args) do
    table.insert(quoted, vim.fn.shellescape(arg))
  end
  append_log("$ " .. table.concat(quoted, " "))
end

local function resolve_main(override)
  local main, err = root.resolve(0, override)
  if not main then
    notify(err, vim.log.levels.ERROR)
  end
  return main
end

local function diagnostic_for(item)
  local message = item.message
  if type(item.help) == "string" and item.help ~= "" then
    message = message .. "\n" .. item.help
  end
  local line = math.max(0, item.line - 1)
  local column = math.max(0, item.column - 1)
  return {
    lnum = line,
    col = column,
    end_lnum = line,
    end_col = column + 1,
    severity = item.severity == "error" and vim.diagnostic.severity.ERROR or vim.diagnostic.severity.WARN,
    source = "tekai",
    code = item.rule,
    message = message,
  }
end

local function apply_diagnostics(report, cwd, reset_all)
  if reset_all then
    vim.diagnostic.reset(M.namespace)
  end
  local grouped = {}
  for _, item in ipairs(report.diagnostics) do
    local filename = absolute(item.path, cwd)
    grouped[filename] = grouped[filename] or {}
    table.insert(grouped[filename], diagnostic_for(item))
  end
  for filename, diagnostics in pairs(grouped) do
    local bufnr = vim.fn.bufadd(filename)
    vim.diagnostic.set(M.namespace, bufnr, diagnostics, {})
  end
end

function M.lint(filename, options)
  options = options or {}
  local target = filename
  if not target or target == "" then
    target = vim.api.nvim_buf_get_name(0)
  end
  if target == "" then
    notify("current buffer has no file", vim.log.levels.ERROR)
    return
  end
  target = absolute(target, vim.uv.cwd())
  local cwd = root.project_dir(target)
  local args = { "lint", target, "--report-json", "--allow-warnings" }
  extend(args, config_args(cwd))
  local full_command = command(config.options.executable, args)
  log_command(full_command)

  local previous = M._state.lint_jobs[target]
  if previous then
    previous:kill(15)
  end
  local target_buf = vim.fn.bufnr(target)
  if target_buf >= 0 then
    vim.diagnostic.reset(M.namespace, target_buf)
  end
  local job
  job = vim.system(full_command, { cwd = cwd, text = true }, function(result)
    vim.schedule(function()
      if M._state.lint_jobs[target] ~= job then
        return
      end
      M._state.lint_jobs[target] = nil
      append_log(result.stderr)
      local report, err = protocol.decode_lint(result.stdout or "")
      if not report then
        if result.code ~= 143 then
          notify(err .. (result.stderr ~= "" and (": " .. vim.trim(result.stderr)) or ""), vim.log.levels.ERROR)
        end
        return
      end
      apply_diagnostics(report, cwd, options.workspace == true)
      if options.notify then
        notify(("lint: %d error(s), %d warning(s)"):format(report.error_count or 0, report.warning_count or 0))
      end
    end)
  end)
  M._state.lint_jobs[target] = job
  return job
end

function M.lint_workspace(directory)
  local cwd = directory and absolute(directory, vim.uv.cwd()) or vim.uv.cwd()
  M.lint(cwd, { workspace = true, notify = true })
end

function M.build(main_override, options)
  options = options or {}
  local main = resolve_main(main_override)
  if not main then
    return
  end
  local cwd = root.command_cwd(main)
  local args = { "build", main, "--report-json" }
  extend(args, config_args(cwd))
  if options.fast then
    extend(args, { "--once", "--fast" })
    extend(args, config.options.preview.extra_args)
  else
    extend(args, config.options.build.extra_args)
  end
  local full_command = command(config.options.executable, args)
  log_command(full_command)
  if M._state.build_job then
    M._state.build_job:kill(15)
  end
  local job
  job = vim.system(full_command, { cwd = cwd, text = true }, function(result)
    vim.schedule(function()
      if M._state.build_job ~= job then
        return
      end
      M._state.build_job = nil
      append_log(result.stderr)
      if result.code ~= 0 then
        notify(("build failed (exit %s): %s"):format(result.code, vim.trim(result.stderr or "")), vim.log.levels.ERROR)
        return
      end
      local report, err = protocol.decode_build(result.stdout or "")
      if not report then
        notify(err, vim.log.levels.ERROR)
        return
      end
      if type(report.pdf_path) == "string" then
        M._state.last_pdf = absolute(report.pdf_path, cwd)
      end
      notify(("%s in %d ms"):format(report.skipped and "cached" or "built", math.floor(report.elapsed_ms + 0.5)))
      if options.open and M._state.last_pdf then
        M.open_pdf(M._state.last_pdf)
      end
    end)
  end)
  M._state.build_job = job
  return job
end

function M.check(main_override, options)
  options = options or {}
  local main = resolve_main(main_override)
  if not main then
    return
  end
  local cwd = root.command_cwd(main)
  local args = { "check", main, "--report-json", "--allow-warnings" }
  extend(args, config_args(cwd))
  extend(args, config.options.check.extra_args)
  local full_command = command(config.options.executable, args)
  log_command(full_command)
  if M._state.build_job then
    M._state.build_job:kill(15)
  end
  local job
  job = vim.system(full_command, { cwd = cwd, text = true }, function(result)
    vim.schedule(function()
      if M._state.build_job ~= job then
        return
      end
      M._state.build_job = nil
      append_log(result.stderr)
      local report, err = protocol.decode_check(result.stdout or "")
      if not report then
        notify(err .. (result.stderr ~= "" and (": " .. vim.trim(result.stderr)) or ""), vim.log.levels.ERROR)
        return
      end
      apply_diagnostics(report, cwd, true)
      if result.code ~= 0 then
        notify(
          ("check blocked: %d error(s), %d warning(s)"):format(
            report.error_count or 0,
            report.warning_count or 0
          ),
          vim.log.levels.ERROR
        )
        return
      end
      if type(report.elapsed_ms) ~= "number" then
        notify("Tekai check succeeded without a build report", vim.log.levels.ERROR)
        return
      end
      if type(report.pdf_path) == "string" then
        M._state.last_pdf = absolute(report.pdf_path, cwd)
      end
      notify(
        ("checked and %s in %d ms (%d warning(s))"):format(
          report.skipped and "cached" or "built",
          math.floor(report.elapsed_ms + 0.5),
          report.warning_count or 0
        )
      )
      if options.open ~= false and config.options.check.open_on_success and M._state.last_pdf then
        M.open_pdf(M._state.last_pdf)
      end
    end)
  end)
  M._state.build_job = job
  return job
end

function M.open_pdf(filename)
  local pdf = filename or M._state.last_pdf
  if not pdf then
    notify("no Tekai PDF has been built in this session", vim.log.levels.ERROR)
    return
  end
  local viewer = config.options.preview.viewer
  if type(viewer) == "function" then
    local ok, err = pcall(viewer, pdf)
    if not ok then
      notify(tostring(err), vim.log.levels.ERROR)
    end
  elseif type(viewer) == "table" and #viewer > 0 then
    local args = vim.deepcopy(viewer)
    table.insert(args, pdf)
    vim.fn.jobstart(args, { detach = true })
  else
    local _, err = vim.ui.open(pdf)
    if err then
      notify(tostring(err), vim.log.levels.ERROR)
    end
  end
end

function M.preview(main_override)
  local main = resolve_main(main_override)
  if not main then
    return
  end
  M.stop(false)
  local cwd = root.command_cwd(main)
  local args = { "watch", main, "--preview", "--allow-warnings" }
  extend(args, config_args(cwd))
  local idle = config.options.preview.final_after_idle_ms
  if idle ~= nil then
    extend(args, { "--final-after-idle-ms", tostring(idle) })
  end
  extend(args, config.options.preview.extra_args)
  local full_command = command(config.options.executable, args)
  log_command(full_command)
  local line_buffer = ""
  local opened = false

  local function consume(data)
    if not data then
      return
    end
    append_log(data)
    line_buffer = line_buffer .. data
    local lines = {}
    while true do
      local newline = line_buffer:find("\n", 1, true)
      if not newline then
        break
      end
      table.insert(lines, (line_buffer:sub(1, newline - 1):gsub("\r$", "")))
      line_buffer = line_buffer:sub(newline + 1)
    end
    for _, line in ipairs(lines) do
      local reported = protocol.built_pdf(line)
      if reported then
        local pdf = absolute(reported, cwd)
        vim.schedule(function()
          M._state.last_pdf = pdf
          if not opened and config.options.preview.open_on_start then
            opened = true
            M.open_pdf(pdf)
          end
        end)
      end
    end
  end

  local job
  job = vim.system(full_command, {
    cwd = cwd,
    text = true,
    stdout = function(_, data)
      if data then
        append_log(data)
      end
    end,
    stderr = function(_, data)
      consume(data)
    end,
  }, function(result)
    vim.schedule(function()
      if M._state.watch_job ~= job then
        return
      end
      M._state.watch_job = nil
      if result.code ~= 0 then
        notify(("live preview stopped (exit %s)"):format(result.code), vim.log.levels.ERROR)
      end
    end)
  end)
  M._state.watch_job = job
  notify("live preview started")
  return job
end

function M.stop(user_initiated)
  if not M._state.watch_job then
    return
  end
  M._state.watch_job:kill(15)
  M._state.watch_job = nil
  if user_initiated ~= false then
    notify("live preview stopped")
  end
end

function M.show_log()
  vim.cmd("botright 12new")
  local bufnr = vim.api.nvim_get_current_buf()
  vim.bo[bufnr].buftype = "nofile"
  vim.bo[bufnr].bufhidden = "wipe"
  vim.bo[bufnr].swapfile = false
  vim.bo[bufnr].filetype = "tekai-log"
  vim.api.nvim_buf_set_name(bufnr, "Tekai Log")
  vim.api.nvim_buf_set_lines(bufnr, 0, -1, false, M._state.log)
  vim.bo[bufnr].modifiable = false
  vim.api.nvim_win_set_cursor(0, { math.max(1, #M._state.log), 0 })
end

local function create_commands()
  local command_options = { nargs = "?", complete = "file", force = true }
  vim.api.nvim_create_user_command("TekaiLint", function(opts)
    M.lint(opts.args ~= "" and opts.args or nil, { notify = true })
  end, command_options)
  vim.api.nvim_create_user_command("TekaiLintWorkspace", function(opts)
    M.lint_workspace(opts.args ~= "" and opts.args or nil)
  end, command_options)
  vim.api.nvim_create_user_command("TekaiBuild", function(opts)
    M.build(opts.args ~= "" and opts.args or nil)
  end, command_options)
  vim.api.nvim_create_user_command("TekaiCheck", function(opts)
    M.check(opts.args ~= "" and opts.args or nil)
  end, command_options)
  vim.api.nvim_create_user_command("TekaiFastPreview", function(opts)
    M.build(opts.args ~= "" and opts.args or nil, { fast = true, open = true })
  end, command_options)
  vim.api.nvim_create_user_command("TekaiPreview", function(opts)
    M.preview(opts.args ~= "" and opts.args or nil)
  end, command_options)
  vim.api.nvim_create_user_command("TekaiStop", function()
    M.stop(true)
  end, { force = true })
  vim.api.nvim_create_user_command("TekaiOpen", function()
    M.open_pdf()
  end, { force = true })
  vim.api.nvim_create_user_command("TekaiLog", function()
    M.show_log()
  end, { force = true })
end

local function create_autocmds()
  local group = vim.api.nvim_create_augroup("tekai.nvim", { clear = true })
  vim.api.nvim_create_autocmd("BufReadPost", {
    group = group,
    pattern = { "*.tex", "*.ltx", "*.cls" },
    callback = function(event)
      if config.options.lint.on_open then
        M.lint(vim.api.nvim_buf_get_name(event.buf))
      end
    end,
  })
  vim.api.nvim_create_autocmd("BufWritePost", {
    group = group,
    pattern = { "*.tex", "*.ltx", "*.cls" },
    callback = function(event)
      if config.options.lint.on_save then
        M.lint(vim.api.nvim_buf_get_name(event.buf))
      end
      if config.options.build.on_save then
        M.build()
      end
    end,
  })
  vim.api.nvim_create_autocmd("VimLeavePre", {
    group = group,
    callback = function()
      M.stop(false)
    end,
  })
end

function M.setup(options)
  config.setup(options)
  create_commands()
  create_autocmds()
  return M
end

return M
