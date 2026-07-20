local M = {}

function M.check()
  vim.health.start("tekai.nvim")
  local options = require("tekai.config").options
  if vim.fn.executable(options.executable) == 1 then
    vim.health.ok(("found executable: %s"):format(options.executable))
    local result = vim.system({ options.executable, "--version" }, { text = true }):wait(3000)
    if result.code == 0 then
      vim.health.info(vim.trim(result.stdout))
    else
      vim.health.warn("could not read the Tekai version")
    end
  else
    vim.health.error(("executable not found: %s"):format(options.executable), {
      "Install Tekai or set require('tekai').setup({ executable = '/path/to/tekai' })",
    })
  end
end

return M
