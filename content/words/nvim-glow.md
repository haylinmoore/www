---
title: nvim keybind for rendering markdown in glow
description: slowly trying to learn nvim
date: 2024-02-18
tags:
  - vim
---

Some lua I wrote to let me render markdown in [glow](https://github.com/charmbracelet/glow).
It was my first time experimenting with hooking into the neovim APIs
and I'm pretty happy with how it turned out.
Hopefully it will be useful to someone else as guidance,
though I'm sure there are better ways to do this.

```lua
local function render_markdown_with_glow()
  local tempfile = vim.fn.tempname() .. ".md"
  vim.cmd("write! " .. tempfile)

  vim.cmd("enew")
  local bufnr = vim.api.nvim_get_current_buf()

  local command = "terminal glow -p " .. tempfile

  vim.cmd(command)

  vim.cmd("startinsert!")

  vim.api.nvim_create_autocmd("TermClose", {
    buffer = bufnr,
    callback = function()
      vim.loop.fs_unlink(tempfile)
      pcall(vim.api.nvim_buf_delete, bufnr, { force = true })
    end,
  })
end

vim.api.nvim_create_autocmd("FileType", {
  pattern = "markdown",
  callback = function()
    vim.keymap.set(
      "n",
      "<leader>md",
      render_markdown_with_glow,
      { silent = true, buffer = true, desc = "render markdown with glow" }
    )
  end,
})
```
