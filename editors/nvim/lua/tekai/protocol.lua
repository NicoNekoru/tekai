local M = {}

function M.magic_root(source)
  local count = 0
  for line in (source .. "\n"):gmatch("(.-)\r?\n") do
    count = count + 1
    if count > 50 then
      break
    end
    local root = line:match("^%s*%%%s*!%s*[Tt][Ee][Xx]%s+[Rr][Oo][Oo][Tt]%s*=%s*(.-)%s*$")
    if root and root ~= "" then
      local quote, value = root:match("^([\"'])(.*)%1$")
      return quote and value or root
    end
  end
  return nil
end

function M.decode_lint(stdout)
  local ok, report = pcall(vim.json.decode, stdout)
  if not ok or type(report) ~= "table" or type(report.diagnostics) ~= "table" then
    return nil, "Tekai returned an invalid lint report"
  end
  for _, diagnostic in ipairs(report.diagnostics) do
    if
      type(diagnostic) ~= "table"
      or type(diagnostic.path) ~= "string"
      or type(diagnostic.line) ~= "number"
      or type(diagnostic.column) ~= "number"
      or (diagnostic.severity ~= "warning" and diagnostic.severity ~= "error")
      or type(diagnostic.rule) ~= "string"
      or type(diagnostic.message) ~= "string"
    then
      return nil, "Tekai returned an invalid diagnostic"
    end
  end
  return report
end

function M.decode_build(stdout)
  local ok, report = pcall(vim.json.decode, stdout)
  if not ok or type(report) ~= "table" or type(report.elapsed_ms) ~= "number" then
    return nil, "Tekai returned an invalid build report"
  end
  return report
end

function M.decode_check(stdout)
  local report, err = M.decode_lint(stdout)
  if not report then
    return nil, err:gsub("lint report", "check report")
  end
  if report.elapsed_ms ~= nil and type(report.elapsed_ms) ~= "number" then
    return nil, "Tekai returned an invalid check build report"
  end
  if report.pdf_path ~= nil and type(report.pdf_path) ~= "string" then
    return nil, "Tekai returned an invalid check PDF path"
  end
  return report
end

function M.built_pdf(line)
  local pdf = line:match("[Bb][Uu][Ii][Ll][Tt]%s+(.+%.pdf)%s+in%s+[%d%.]+[%aµ]+")
    or line:match("[Cc][Aa][Cc][Hh][Ee][Dd]%s+(.+%.pdf)%s+in%s+[%d%.]+[%aµ]+")
  return pdf and vim.trim(pdf) or nil
end

return M
