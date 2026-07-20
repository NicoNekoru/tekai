local config = require("tekai.config")
local protocol = require("tekai.protocol")

local M = {}

local function is_absolute(filename)
  return filename:sub(1, 1) == "/" or filename:match("^%a:[/\\]") ~= nil
end

local function absolute(filename, base)
  if is_absolute(filename) then
    return vim.fs.normalize(filename)
  end
  return vim.fs.normalize(vim.fs.joinpath(base, filename))
end

local function readable(filename)
  local stat = vim.uv.fs_stat(filename)
  return stat ~= nil and stat.type == "file"
end

local function read_file(filename, limit)
  local file = io.open(filename, "rb")
  if not file then
    return nil
  end
  local source = file:read(limit or "*a")
  file:close()
  return source
end

local function looks_like_root(source)
  return source
    and source:find("\\documentclass", 1, true)
    and source:find("\\begin{document}", 1, true)
end

function M.project_dir(filename)
  local normalized = vim.fs.normalize(filename)
  local stat = vim.uv.fs_stat(normalized)
  local start = stat and stat.type == "directory" and normalized or vim.fs.dirname(normalized)
  return vim.fs.root(start, { "tekai.toml", ".git" }) or start
end

function M.command_cwd(main)
  return M.project_dir(main)
end

local function configured_main(bufnr)
  local value = config.options.main_file
  if type(value) == "function" then
    value = value(bufnr)
  end
  if type(value) ~= "string" or value == "" then
    return nil
  end
  local current = vim.api.nvim_buf_get_name(bufnr)
  local base = current ~= "" and M.project_dir(current) or vim.uv.cwd()
  return absolute(value, base)
end

function M.resolve(bufnr, override)
  bufnr = bufnr or 0
  local current = vim.api.nvim_buf_get_name(bufnr)
  local directory = current ~= "" and vim.fs.dirname(current) or vim.uv.cwd()

  if override and override ~= "" then
    local candidate = absolute(override, vim.uv.cwd())
    return readable(candidate) and candidate or nil, readable(candidate) and nil or ("root file does not exist: " .. candidate)
  end

  local explicit = configured_main(bufnr)
  if explicit then
    return readable(explicit) and explicit or nil, readable(explicit) and nil or ("configured root file does not exist: " .. explicit)
  end

  if current ~= "" then
    local lines = vim.api.nvim_buf_get_lines(bufnr, 0, math.min(50, vim.api.nvim_buf_line_count(bufnr)), false)
    local magic = protocol.magic_root(table.concat(lines, "\n"))
    if magic then
      local candidate = absolute(magic, directory)
      return readable(candidate) and candidate or nil, readable(candidate) and nil or ("magic root file does not exist: " .. candidate)
    end

    local source = table.concat(vim.api.nvim_buf_get_lines(bufnr, 0, -1, false), "\n")
    if looks_like_root(source) then
      return vim.fs.normalize(current)
    end
  end

  local nearest_main = vim.fs.find("main.tex", { path = directory, upward = true, type = "file" })[1]
  if nearest_main then
    return vim.fs.normalize(nearest_main)
  end

  local project = current ~= "" and M.project_dir(current) or vim.uv.cwd()
  local files = vim.fs.find(function(name)
    local lower = name:lower()
    return lower:sub(-4) == ".tex" or lower:sub(-4) == ".ltx"
  end, { path = project, type = "file", limit = 200 })
  local candidates = {}
  for _, filename in ipairs(files) do
    if not filename:find("/build/", 1, true) and not filename:find("/.git/", 1, true) then
      if looks_like_root(read_file(filename, 256 * 1024)) then
        table.insert(candidates, vim.fs.normalize(filename))
      end
    end
  end
  if #candidates == 1 then
    return candidates[1]
  end
  if #candidates > 1 then
    return nil, "multiple root documents found; set require('tekai').setup({ main_file = '...' })"
  end
  return nil, "could not determine the root TeX file; set main_file or add a % !TEX root comment"
end

return M
