local M = {}

M.defaults = {
  executable = "tekai",
  main_file = nil,
  config_file = nil,
  lint = {
    on_save = true,
    on_open = true,
  },
  build = {
    on_save = false,
    extra_args = {},
  },
  check = {
    open_on_success = true,
    extra_args = {},
  },
  preview = {
    open_on_start = true,
    final_after_idle_ms = 1500,
    extra_args = {},
    -- nil uses vim.ui.open. A list is executed with the PDF appended, and a
    -- function receives the absolute PDF path.
    viewer = nil,
  },
}

M.options = vim.deepcopy(M.defaults)

function M.setup(options)
  M.options = vim.tbl_deep_extend("force", vim.deepcopy(M.defaults), options or {})
  return M.options
end

return M
