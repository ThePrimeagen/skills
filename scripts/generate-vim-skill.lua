-- Script to generate VIM.md skill file for LLMs
-- Run with: nvim --headless -l ~/.behaviors/scripts/generate-vim-skill.lua

local remove_all_comments = function(text)
  local lines = vim.split(text, "\n")

  local result = {}
  for _, line in ipairs(lines) do
    line = vim.trim(line)
    if not vim.startswith(line, "--") and not vim.startswith(line, "error") and line ~= "" then
      table.insert(result, line)
    end
  end

  return table.concat(result, "\n")
end

local remove_function_and_end = function(text)
  local lines = vim.split(text, "\n")

  local result = {}
  for _, line in ipairs(lines) do
    line = line:gsub("^function ", "")
    line = line:gsub("end$", "")
    table.insert(result, line)
  end

  return table.concat(result, "\n")
end

local metas = {
  {
    name = "api.lua",
    description = "The following are type stubs for all the functions available on `vim.api.*`. "
      .. "Prefer these functions where possible.",
    process = { remove_all_comments, remove_function_and_end },
  },
  {
    name = "builtin.lua",
    description = "Various APIs that are provided by Neovim, that are unique to the Lua API.",
  },
  {
    name = "api_keysets.lua",
    description = "The following describe various types that are used in Neovim's API.",
  },
  {
    name = "api_keysets_extra.lua",
    description = "Additional types that are used in Neovim's API.",
  },
  {
    name = "builtin_types.lua",
    description = "Various types used by Neovim's builtin APIs.",
  },
  {
    name = "vimfn.lua",
    description = "Functions available from Neovim's vimscript APIs. They are available via `vim.fn.*`.",
    process = { remove_all_comments, remove_function_and_end },
  },
}

local read = {}

-- Add header
table.insert(read, "# Neovim Lua API Reference")
table.insert(read, "")
table.insert(read, "This document contains type stubs and API references for Neovim's Lua API.")
table.insert(read, "Use this as a reference when writing Neovim plugins or configurations in Lua.")
table.insert(read, "")

for _, meta in ipairs(metas) do
  local file = vim.api.nvim_get_runtime_file("lua/vim/_meta/" .. meta.name, false)[1]
  if file then
    local lines = vim.fn.readfile(file)
    for i, line in ipairs(lines) do
      lines[i] = line:gsub("^%s+", ""):gsub("%s+$", ""):gsub("%s+", " ")
    end

    local text = table.concat(lines, "\n")
    if meta.process then
      for _, fn in ipairs(meta.process) do
        text = fn(text)
      end
    end

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
    io.stderr:write("Warning: Could not find " .. meta.name .. "\n")
  end
end

local result = table.concat(read, "\n")

-- Get output path from args or use default
local output_path = arg[1] or (os.getenv("HOME") .. "/.behaviors/vim.md")

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
