-- Script to generate vim.lsp.md skill file for LLMs
-- Run with: nvim --headless -l ~/.behaviors/scripts/generate-vim-lsp-skill.lua

local remove_comments = function(text)
  local lines = vim.split(text, "\n")

  local result = {}
  for _, line in ipairs(lines) do
    local trimmed = vim.trim(line)

    -- Remove ALL comments (anything starting with --)
    -- Also remove error() calls, empty lines, local X = {} declarations
    local is_comment = vim.startswith(trimmed, "--")
    local is_error_call = vim.startswith(trimmed, "error")
    local is_local_table = trimmed:match("^local %w+ = {}")

    local should_remove = is_comment or is_error_call or is_local_table or trimmed == ""

    if not should_remove then
      table.insert(result, trimmed)
    end
  end

  return table.concat(result, "\n")
end

-- LSP meta files that contain type stubs
local metas = {
  {
    name = "protocol.lua",
    path = "lua/vim/lsp/_meta/protocol.lua",
    description = "LSP protocol types and structures used by Neovim's LSP client.",
  },
}

local read = {}

-- Add header
table.insert(read, "# Neovim LSP API Reference")
table.insert(read, "")
table.insert(read, "This document contains type stubs and API references for Neovim's LSP Lua API.")
table.insert(read, "Use this as a reference when working with LSP in Neovim Lua.")
table.insert(read, "")

-- Process meta files (type stubs)
for _, meta in ipairs(metas) do
  local file = vim.api.nvim_get_runtime_file(meta.path, false)[1]
  if file then
    local lines = vim.fn.readfile(file)
    local text = table.concat(lines, "\n")
    text = remove_comments(text)

    table.insert(read, "---")
    table.insert(read, "")
    table.insert(read, "## " .. meta.name:gsub("%.lua$", ""))
    table.insert(read, "")
    table.insert(read, meta.description)
    table.insert(read, "")
    table.insert(read, "```lua")
    table.insert(read, text)
    table.insert(read, "```")
    table.insert(read, "")
  else
    io.stderr:write("Warning: Could not find " .. meta.path .. "\n")
  end
end

local result = table.concat(read, "\n")

-- Get output path from args or use default
local output_path = arg[1] or (os.getenv("HOME") .. "/.behaviors/vim.lsp.md")

-- Ensure directory exists
local dir = output_path:match("(.*/)")
if dir then
  os.execute("mkdir -p " .. dir)
end

-- Write to file
local file = io.open(output_path, "w")
if file then
  file:write(result)
  file:close()
  print("Generated: " .. output_path)
else
  io.stderr:write("Error: Could not write to " .. output_path .. "\n")
  os.exit(1)
end
