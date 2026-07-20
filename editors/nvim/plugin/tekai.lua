if vim.g.loaded_tekai_nvim then
  return
end
vim.g.loaded_tekai_nvim = true

require("tekai").setup()
