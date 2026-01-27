# Neovim Treesitter API Reference

This document contains type stubs and API references for Neovim's treesitter Lua API.
Use this as a reference when working with treesitter in Neovim Lua.

---

## tsnode

TSNode methods - represents a specific element in a parsed syntax tree. Use these methods to navigate and inspect nodes.

```lua
function TSNode:parent() end
function TSNode:next_sibling() end
function TSNode:prev_sibling() end
function TSNode:next_named_sibling() end
function TSNode:prev_named_sibling() end
function TSNode:iter_children() end
function TSNode:field(name) end
function TSNode:child_count() end
function TSNode:child(index) end
function TSNode:named_child_count() end
function TSNode:named_children() end
function TSNode:__has_ancestor(node_types) end
function TSNode:named_child(index) end
function TSNode:child_with_descendant(descendant) end
function TSNode:start() end
function TSNode:end_() end
function TSNode:range(include_bytes) end
function TSNode:type() end
function TSNode:symbol() end
function TSNode:named() end
function TSNode:missing() end
function TSNode:extra() end
function TSNode:has_changes() end
function TSNode:has_error() end
function TSNode:sexpr() end
function TSNode:id() end
function TSNode:tree() end
function TSNode:descendant_for_range(start_row, start_col, end_row, end_col) end
function TSNode:named_descendant_for_range(start_row, start_col, end_row, end_col) end
function TSNode:equal(node) end
function TSNode:byte_length() end
```

---

## tstree

TSTree methods - represents the parsed contents of a buffer.

```lua
function TSTree:root() end
function TSTree:edit(start_byte, end_byte_old, end_byte_new, start_row, start_col, end_row_old, end_col_old, end_row_new, end_col_new) end
function TSTree:copy() end
function TSTree:included_ranges(include_bytes) end
function TSTree:included_ranges(include_bytes) end
```

---

## tsquery

TSQuery methods - for working with treesitter queries.

```lua
function TSQuery:inspect() end
function TSQuery:disable_capture(capture_name) end
function TSQuery:disable_pattern(pattern_index) end
```

---

## misc

Various treesitter types including TSParser, TSQueryMatch, TSQueryCursor, and TSLangInfo.

```lua
vim._ts_inspect_language = function(lang) end
vim._ts_get_language_version = function() end
vim._ts_add_language_from_object = function(path, lang, symbol_name) end
vim._ts_add_language_from_wasm = function(path, lang) end
vim._ts_get_minimum_language_version = function() end
vim._ts_parse_query = function(lang, query) end
vim._create_ts_parser = function(lang) end
function TSQueryMatch:info() end
function TSQueryCursor:next_capture() end
function TSQueryCursor:next_match() end
function vim._create_ts_querycursor(node, query, start, stop, opts) end
```
