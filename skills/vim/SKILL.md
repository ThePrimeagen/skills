# Neovim Lua API Reference

This document contains type stubs and API references for Neovim's Lua API.
Use this as a reference when writing Neovim plugins or configurations in Lua.

---

## api

The following are type stubs for all the functions available on `vim.api.*`. Prefer these functions where possible.

```lua
vim.api = {}
vim.api.nvim__buf_debug_extmarks(buffer, keys, dot) 
vim.api.nvim__buf_stats(buffer) 
vim.api.nvim__complete_set(index, opts) 
vim.api.nvim__get_lib_dir() 
vim.api.nvim__get_runtime(pat, all, opts) 
vim.api.nvim__id(obj) 
vim.api.nvim__id_array(arr) 
vim.api.nvim__id_dict(dct) 
vim.api.nvim__id_float(flt) 
vim.api.nvim__inspect_cell(grid, row, col) 
vim.api.nvim__invalidate_glyph_cache() 
vim.api.nvim__ns_get(ns_id) 
vim.api.nvim__ns_set(ns_id, opts) 
vim.api.nvim__redraw(opts) 
vim.api.nvim__runtime_inspect() 
vim.api.nvim__screenshot(path) 
vim.api.nvim__stats() 
vim.api.nvim__unpack(str) 
vim.api.nvim_buf_add_highlight(buffer, ns_id, hl_group, line, col_start, col_end) 
vim.api.nvim_buf_attach(buffer, send_buffer, opts) 
vim.api.nvim_buf_call(buffer, fun) 
vim.api.nvim_buf_clear_highlight(buffer, ns_id, line_start, line_end) 
vim.api.nvim_buf_clear_namespace(buffer, ns_id, line_start, line_end) 
vim.api.nvim_buf_create_user_command(buffer, name, command, opts) 
vim.api.nvim_buf_del_extmark(buffer, ns_id, id) 
vim.api.nvim_buf_del_keymap(buffer, mode, lhs) 
vim.api.nvim_buf_del_mark(buffer, name) 
vim.api.nvim_buf_del_user_command(buffer, name) 
vim.api.nvim_buf_del_var(buffer, name) 
vim.api.nvim_buf_delete(buffer, opts) 
vim.api.nvim_buf_get_changedtick(buffer) 
vim.api.nvim_buf_get_commands(buffer, opts) 
vim.api.nvim_buf_get_extmark_by_id(buffer, ns_id, id, opts) 
vim.api.nvim_buf_get_extmarks(buffer, ns_id, start, end_, opts) 
vim.api.nvim_buf_get_keymap(buffer, mode) 
vim.api.nvim_buf_get_lines(buffer, start, end_, strict_indexing) 
vim.api.nvim_buf_get_mark(buffer, name) 
vim.api.nvim_buf_get_name(buffer) 
vim.api.nvim_buf_get_number(buffer) 
vim.api.nvim_buf_get_offset(buffer, index) 
vim.api.nvim_buf_get_option(buffer, name) 
vim.api.nvim_buf_get_text(buffer, start_row, start_col, end_row, end_col, opts) 
vim.api.nvim_buf_get_var(buffer, name) 
vim.api.nvim_buf_is_loaded(buffer) 
vim.api.nvim_buf_is_valid(buffer) 
vim.api.nvim_buf_line_count(buffer) 
vim.api.nvim_buf_set_extmark(buffer, ns_id, line, col, opts) 
vim.api.nvim_buf_set_keymap(buffer, mode, lhs, rhs, opts) 
vim.api.nvim_buf_set_lines(buffer, start, end_, strict_indexing, replacement) 
vim.api.nvim_buf_set_mark(buffer, name, line, col, opts) 
vim.api.nvim_buf_set_name(buffer, name) 
vim.api.nvim_buf_set_option(buffer, name, value) 
vim.api.nvim_buf_set_text(buffer, start_row, start_col, end_row, end_col, replacement) 
vim.api.nvim_buf_set_var(buffer, name, value) 
vim.api.nvim_buf_set_virtual_text(buffer, src_id, line, chunks, opts) 
vim.api.nvim_call_dict_function(dict, fn, args) 
vim.api.nvim_call_function(fn, args) 
vim.api.nvim_chan_send(chan, data) 
vim.api.nvim_clear_autocmds(opts) 
vim.api.nvim_cmd(cmd, opts) 
vim.api.nvim_command(command) 
vim.api.nvim_command_output(command) 
vim.api.nvim_create_augroup(name, opts) 
vim.api.nvim_create_autocmd(event, opts) 
vim.api.nvim_create_buf(listed, scratch) 
vim.api.nvim_create_namespace(name) 
vim.api.nvim_create_user_command(name, command, opts) 
vim.api.nvim_del_augroup_by_id(id) 
vim.api.nvim_del_augroup_by_name(name) 
vim.api.nvim_del_autocmd(id) 
vim.api.nvim_del_current_line() 
vim.api.nvim_del_keymap(mode, lhs) 
vim.api.nvim_del_mark(name) 
vim.api.nvim_del_user_command(name) 
vim.api.nvim_del_var(name) 
vim.api.nvim_echo(chunks, history, opts) 
vim.api.nvim_err_write(str) 
vim.api.nvim_err_writeln(str) 
vim.api.nvim_eval(expr) 
vim.api.nvim_eval_statusline(str, opts) 
vim.api.nvim_exec(src, output) 
vim.api.nvim_exec2(src, opts) 
vim.api.nvim_exec_autocmds(event, opts) 
vim.api.nvim_feedkeys(keys, mode, escape_ks) 
vim.api.nvim_get_all_options_info() 
vim.api.nvim_get_autocmds(opts) 
vim.api.nvim_get_chan_info(chan) 
vim.api.nvim_get_color_by_name(name) 
vim.api.nvim_get_color_map() 
vim.api.nvim_get_commands(opts) 
vim.api.nvim_get_context(opts) 
vim.api.nvim_get_current_buf() 
vim.api.nvim_get_current_line() 
vim.api.nvim_get_current_tabpage() 
vim.api.nvim_get_current_win() 
vim.api.nvim_get_hl(ns_id, opts) 
vim.api.nvim_get_hl_by_id(hl_id, rgb) 
vim.api.nvim_get_hl_by_name(name, rgb) 
vim.api.nvim_get_hl_id_by_name(name) 
vim.api.nvim_get_hl_ns(opts) 
vim.api.nvim_get_keymap(mode) 
vim.api.nvim_get_mark(name, opts) 
vim.api.nvim_get_mode() 
vim.api.nvim_get_namespaces() 
vim.api.nvim_get_option(name) 
vim.api.nvim_get_option_info(name) 
vim.api.nvim_get_option_info2(name, opts) 
vim.api.nvim_get_option_value(name, opts) 
vim.api.nvim_get_proc(pid) 
vim.api.nvim_get_proc_children(pid) 
vim.api.nvim_get_runtime_file(name, all) 
vim.api.nvim_get_var(name) 
vim.api.nvim_get_vvar(name) 
vim.api.nvim_input(keys) 
vim.api.nvim_input_mouse(button, action, modifier, grid, row, col) 
vim.api.nvim_list_bufs() 
vim.api.nvim_list_chans() 
vim.api.nvim_list_runtime_paths() 
vim.api.nvim_list_tabpages() 
vim.api.nvim_list_uis() 
vim.api.nvim_list_wins() 
vim.api.nvim_load_context(dict) 
vim.api.nvim_notify(msg, log_level, opts) 
vim.api.nvim_open_term(buffer, opts) 
vim.api.nvim_open_win(buffer, enter, config) 
vim.api.nvim_out_write(str) 
vim.api.nvim_parse_cmd(str, opts) 
vim.api.nvim_parse_expression(expr, flags, highlight) 
vim.api.nvim_paste(data, crlf, phase) 
vim.api.nvim_put(lines, type, after, follow) 
vim.api.nvim_replace_termcodes(str, from_part, do_lt, special) 
vim.api.nvim_select_popupmenu_item(item, insert, finish, opts) 
vim.api.nvim_set_current_buf(buffer) 
vim.api.nvim_set_current_dir(dir) 
vim.api.nvim_set_current_line(line) 
vim.api.nvim_set_current_tabpage(tabpage) 
vim.api.nvim_set_current_win(window) 
vim.api.nvim_set_decoration_provider(ns_id, opts) 
vim.api.nvim_set_hl(ns_id, name, val) 
vim.api.nvim_set_hl_ns(ns_id) 
vim.api.nvim_set_hl_ns_fast(ns_id) 
vim.api.nvim_set_keymap(mode, lhs, rhs, opts) 
vim.api.nvim_set_option(name, value) 
vim.api.nvim_set_option_value(name, value, opts) 
vim.api.nvim_set_var(name, value) 
vim.api.nvim_set_vvar(name, value) 
vim.api.nvim_strwidth(text) 
vim.api.nvim_tabpage_del_var(tabpage, name) 
vim.api.nvim_tabpage_get_number(tabpage) 
vim.api.nvim_tabpage_get_var(tabpage, name) 
vim.api.nvim_tabpage_get_win(tabpage) 
vim.api.nvim_tabpage_is_valid(tabpage) 
vim.api.nvim_tabpage_list_wins(tabpage) 
vim.api.nvim_tabpage_set_var(tabpage, name, value) 
vim.api.nvim_tabpage_set_win(tabpage, win) 
vim.api.nvim_win_call(window, fun) 
vim.api.nvim_win_close(window, force) 
vim.api.nvim_win_del_var(window, name) 
vim.api.nvim_win_get_buf(window) 
vim.api.nvim_win_get_config(window) 
vim.api.nvim_win_get_cursor(window) 
vim.api.nvim_win_get_height(window) 
vim.api.nvim_win_get_number(window) 
vim.api.nvim_win_get_option(window, name) 
vim.api.nvim_win_get_position(window) 
vim.api.nvim_win_get_tabpage(window) 
vim.api.nvim_win_get_var(window, name) 
vim.api.nvim_win_get_width(window) 
vim.api.nvim_win_hide(window) 
vim.api.nvim_win_is_valid(window) 
vim.api.nvim_win_set_buf(window, buffer) 
vim.api.nvim_win_set_config(window, config) 
vim.api.nvim_win_set_cursor(window, pos) 
vim.api.nvim_win_set_height(window, height) 
vim.api.nvim_win_set_hl_ns(window, ns_id) 
vim.api.nvim_win_set_option(window, name, value) 
vim.api.nvim_win_set_var(window, name, value) 
vim.api.nvim_win_set_width(window, width) 
vim.api.nvim_win_text_height(window, opts) 
```

---

## builtin

Various APIs that are provided by Neovim, that are unique to the Lua API.

```lua
---@meta
-- luacheck: no unused args

error('Cannot require a meta file')

--- @brief <pre>help
--- vim.api.{func}({...}) *vim.api*
--- Invokes Nvim |API| function {func} with arguments {...}.
--- Example: call the "nvim_get_current_line()" API function: >lua
--- print(tostring(vim.api.nvim_get_current_line()))
---
--- vim.NIL *vim.NIL*
--- Special value representing NIL in |RPC| and |v:null| in Vimscript
--- conversion, and similar cases. Lua `nil` cannot be used as part of a Lua
--- table representing a Dictionary or Array, because it is treated as
--- missing: `{"foo", nil}` is the same as `{"foo"}`.
---
--- vim.type_idx *vim.type_idx*
--- Type index for use in |lua-special-tbl|. Specifying one of the values from
--- |vim.types| allows typing the empty table (it is unclear whether empty Lua
--- table represents empty list or empty array) and forcing integral numbers
--- to be |Float|. See |lua-special-tbl| for more details.
---
--- vim.val_idx *vim.val_idx*
--- Value index for tables representing |Float|s. A table representing
--- floating-point value 1.0 looks like this: >lua
--- {
--- [vim.type_idx] = vim.types.float,
--- [vim.val_idx] = 1.0,
--- }
--- < See also |vim.type_idx| and |lua-special-tbl|.
---
--- vim.types *vim.types*
--- Table with possible values for |vim.type_idx|. Contains two sets of
--- key-value pairs: first maps possible values for |vim.type_idx| to
--- human-readable strings, second maps human-readable type names to values
--- for |vim.type_idx|. Currently contains pairs for `float`, `array` and
--- `dictionary` types.
---
--- Note: One must expect that values corresponding to `vim.types.float`,
--- `vim.types.array` and `vim.types.dictionary` fall under only two following
--- assumptions:
--- 1. Value may serve both as a key and as a value in a table. Given the
--- properties of Lua tables this basically means “value is not `nil`”.
--- 2. For each value in `vim.types` table `vim.types[vim.types[value]]` is the
--- same as `value`.
--- No other restrictions are put on types, and it is not guaranteed that
--- values corresponding to `vim.types.float`, `vim.types.array` and
--- `vim.types.dictionary` will not change or that `vim.types` table will only
--- contain values for these three types.
---
--- *log_levels* *vim.log.levels*
--- Log levels are one of the values defined in `vim.log.levels`:
---
--- vim.log.levels.DEBUG
--- vim.log.levels.ERROR
--- vim.log.levels.INFO
--- vim.log.levels.TRACE
--- vim.log.levels.WARN
--- vim.log.levels.OFF
---
--- </pre>

---@nodoc
---@class vim.NIL

---@type vim.NIL
---@nodoc
vim.NIL = ...

--- Returns true if the code is executing as part of a "fast" event handler,
--- where most of the API is disabled. These are low-level events (e.g.
--- |lua-loop-callbacks|) which can be invoked whenever Nvim polls for input.
--- When this is `false` most API functions are callable (but may be subject
--- to other restrictions such as |textlock|).
function vim.in_fast_event() end

--- Creates a special empty table (marked with a metatable), which Nvim
--- converts to an empty dictionary when translating Lua values to Vimscript
--- or API types. Nvim by default converts an empty table `{}` without this
--- metatable to an list/array.
---
--- Note: If numeric keys are present in the table, Nvim ignores the metatable
--- marker and converts the dict to a list/array anyway.
--- @return table
function vim.empty_dict() end

--- Sends {event} to {channel} via |RPC| and returns immediately. If {channel}
--- is 0, the event is broadcast to all channels.
---
--- This function also works in a fast callback |lua-loop-callbacks|.
--- @param channel integer
--- @param method string
--- @param ...? any
function vim.rpcnotify(channel, method, ...) end

--- Sends a request to {channel} to invoke {method} via |RPC| and blocks until
--- a response is received.
---
--- Note: NIL values as part of the return value is represented as |vim.NIL|
--- special value
--- @param channel integer
--- @param method string
--- @param ...? any
function vim.rpcrequest(channel, method, ...) end

--- Compares strings case-insensitively.
--- @param a string
--- @param b string
--- @return 0|1|-1
--- if strings are
--- equal, {a} is greater than {b} or {a} is lesser than {b}, respectively.
function vim.stricmp(a, b) end

--- Gets a list of the starting byte positions of each UTF-8 codepoint in the given string.
---
--- Embedded NUL bytes are treated as terminating the string.
--- @param str string
--- @return integer[]
function vim.str_utf_pos(str) end

--- Gets the distance (in bytes) from the starting byte of the codepoint (character) that {index}
--- points to.
---
--- The result can be added to {index} to get the starting byte of a character.
---
--- Examples:
---
--- ```lua
--- -- The character 'æ' is stored as the bytes '\xc3\xa6' (using UTF-8)
---
--- -- Returns 0 because the index is pointing at the first byte of a character
--- vim.str_utf_start('æ', 1)
---
--- -- Returns -1 because the index is pointing at the second byte of a character
--- vim.str_utf_start('æ', 2)
--- ```
---
--- @param str string
--- @param index integer
--- @return integer
function vim.str_utf_start(str, index) end

--- Gets the distance (in bytes) from the last byte of the codepoint (character) that {index} points
--- to.
---
--- Examples:
---
--- ```lua
--- -- The character 'æ' is stored as the bytes '\xc3\xa6' (using UTF-8)
---
--- -- Returns 0 because the index is pointing at the last byte of a character
--- vim.str_utf_end('æ', 2)
---
--- -- Returns 1 because the index is pointing at the penultimate byte of a character
--- vim.str_utf_end('æ', 1)
--- ```
---
--- @param str string
--- @param index integer
--- @return integer
function vim.str_utf_end(str, index) end

--- The result is a String, which is the text {str} converted from
--- encoding {from} to encoding {to}. When the conversion fails `nil` is
--- returned. When some characters could not be converted they
--- are replaced with "?".
--- The encoding names are whatever the iconv() library function
--- can accept, see ":Man 3 iconv".
---
--- @param str string Text to convert
--- @param from string Encoding of {str}
--- @param to string Target encoding
--- @return string? : Converted string if conversion succeeds, `nil` otherwise.
function vim.iconv(str, from, to, opts) end

--- Schedules {fn} to be invoked soon by the main event-loop. Useful
--- to avoid |textlock| or other temporary restrictions.
--- @param fn fun()
function vim.schedule(fn) end

--- Wait for {time} in milliseconds until {callback} returns `true`.
---
--- Executes {callback} immediately and at approximately {interval}
--- milliseconds (default 200). Nvim still processes other events during
--- this time.
---
--- Cannot be called while in an |api-fast| event.
---
--- Examples:
---
--- ```lua
--- ---
--- -- Wait for 100 ms, allowing other events to process
--- vim.wait(100, function() end)
---
--- ---
--- -- Wait for 100 ms or until global variable set.
--- vim.wait(100, function() return vim.g.waiting_for_var end)
---
--- ---
--- -- Wait for 1 second or until global variable set, checking every ~500 ms
--- vim.wait(1000, function() return vim.g.waiting_for_var end, 500)
---
--- ---
--- -- Schedule a function to set a value in 100ms
--- vim.defer_fn(function() vim.g.timer_result = true end, 100)
---
--- -- Would wait ten seconds if results blocked. Actually only waits 100 ms
--- if vim.wait(10000, function() return vim.g.timer_result end) then
--- print('Only waiting a little bit of time!')
--- end
--- ```
---
--- @param time integer Number of milliseconds to wait
--- @param callback? fun(): boolean Optional callback. Waits until {callback} returns true
--- @param interval? integer (Approximate) number of milliseconds to wait between polls
--- @param fast_only? boolean If true, only |api-fast| events will be processed.
--- @return boolean, nil|-1|-2
--- - If {callback} returns `true` during the {time}: `true, nil`
--- - If {callback} never returns `true` during the {time}: `false, -1`
--- - If {callback} is interrupted during the {time}: `false, -2`
--- - If {callback} errors, the error is raised.
function vim.wait(time, callback, interval, fast_only) end

--- Subscribe to |ui-events|, similar to |nvim_ui_attach()| but receive events in a Lua callback.
--- Used to implement screen elements like popupmenu or message handling in Lua.
---
--- {options} is a dict with one or more `ext_…` |ui-option|s set to true to enable events for
--- the respective UI element.
---
--- {callback} receives event name plus additional parameters. See |ui-popupmenu|
--- and the sections below for event format for respective events.
---
--- Callbacks for `msg_show` events are executed in |api-fast| context; showing
--- the message should be scheduled.
---
--- Excessive errors inside the callback will result in forced detachment.
---
--- WARNING: This api is considered experimental. Usability will vary for
--- different screen elements. In particular `ext_messages` behavior is subject
--- to further changes and usability improvements. This is expected to be
--- used to handle messages when setting 'cmdheight' to zero (which is
--- likewise experimental).
---
--- Example (stub for a |ui-popupmenu| implementation):
---
--- ```lua
--- ns = vim.api.nvim_create_namespace('my_fancy_pum')
---
--- vim.ui_attach(ns, {ext_popupmenu=true}, function(event, ...)
--- if event == 'popupmenu_show' then
--- local items, selected, row, col, grid = ...
--- print('display pum ', #items)
--- elseif event == 'popupmenu_select' then
--- local selected = ...
--- print('selected', selected)
--- elseif event == 'popupmenu_hide' then
--- print('FIN')
--- end
--- end)
--- ```
---
--- @since 0
---
--- @param ns integer
--- @param options table<string, any>
--- @param callback fun()
function vim.ui_attach(ns, options, callback) end

--- Detach a callback previously attached with |vim.ui_attach()| for the
--- given namespace {ns}.
--- @param ns integer
function vim.ui_detach(ns) end
```

---

## api_keysets

The following describe various types that are used in Neovim's API.

```lua
--- @meta _
-- THIS FILE IS GENERATED
-- DO NOT EDIT
error('Cannot require a meta file')

--- @class vim.api.keyset.buf_attach
--- @field on_lines? fun(_: "lines", bufnr: integer, changedtick: integer, first: integer, last_old: integer, last_new: integer, byte_count: integer, deleted_codepoints?: integer, deleted_codeunits?: integer): boolean?
--- @field on_bytes? fun(_: "bytes", bufnr: integer, changedtick: integer, start_row: integer, start_col: integer, start_byte: integer, old_end_row: integer, old_end_col: integer, old_end_byte: integer, new_end_row: integer, new_end_col: integer, new_end_byte: integer): boolean?
--- @field on_changedtick? fun(_: "changedtick", bufnr: integer, changedtick: integer)
--- @field on_detach? fun(_: "detach", bufnr: integer)
--- @field on_reload? fun(_: "reload", bufnr: integer)
--- @field utf_sizes? boolean
--- @field preview? boolean

--- @class vim.api.keyset.buf_delete
--- @field force? boolean
--- @field unload? boolean

--- @class vim.api.keyset.clear_autocmds
--- @field buffer? integer
--- @field event? string|string[]
--- @field group? integer|string
--- @field pattern? string|string[]

--- @class vim.api.keyset.cmd
--- @field cmd? string
--- @field range? any[]
--- @field count? integer
--- @field reg? string
--- @field bang? boolean
--- @field args? string[]
--- @field magic? table<string,any>
--- @field mods? table<string,any>
--- @field nargs? integer|string
--- @field addr? string
--- @field nextcmd? string

--- @class vim.api.keyset.cmd_magic
--- @field file? boolean
--- @field bar? boolean

--- @class vim.api.keyset.cmd_mods
--- @field silent? boolean
--- @field emsg_silent? boolean
--- @field unsilent? boolean
--- @field filter? table<string,any>
--- @field sandbox? boolean
--- @field noautocmd? boolean
--- @field browse? boolean
--- @field confirm? boolean
--- @field hide? boolean
--- @field horizontal? boolean
--- @field keepalt? boolean
--- @field keepjumps? boolean
--- @field keepmarks? boolean
--- @field keeppatterns? boolean
--- @field lockmarks? boolean
--- @field noswapfile? boolean
--- @field tab? integer
--- @field verbose? integer
--- @field vertical? boolean
--- @field split? string

--- @class vim.api.keyset.cmd_mods_filter
--- @field pattern? string
--- @field force? boolean

--- @class vim.api.keyset.cmd_opts
--- @field output? boolean

--- @class vim.api.keyset.complete_set
--- @field info? string

--- @class vim.api.keyset.context
--- @field types? string[]

--- @class vim.api.keyset.create_augroup
--- @field clear? boolean

--- @class vim.api.keyset.create_autocmd
--- @field buffer? integer
--- @field callback? string|(fun(args: vim.api.keyset.create_autocmd.callback_args): boolean?)
--- @field command? string
--- @field desc? string
--- @field group? integer|string
--- @field nested? boolean
--- @field once? boolean
--- @field pattern? string|string[]

--- @class vim.api.keyset.echo_opts
--- @field err? boolean
--- @field verbose? boolean

--- @class vim.api.keyset.empty

--- @class vim.api.keyset.eval_statusline
--- @field winid? integer
--- @field maxwidth? integer
--- @field fillchar? string
--- @field highlights? boolean
--- @field use_winbar? boolean
--- @field use_tabline? boolean
--- @field use_statuscol_lnum? integer

--- @class vim.api.keyset.exec_autocmds
--- @field buffer? integer
--- @field group? integer|string
--- @field modeline? boolean
--- @field pattern? string|string[]
--- @field data? any

--- @class vim.api.keyset.exec_opts
--- @field output? boolean

--- @class vim.api.keyset.get_autocmds
--- @field event? string|string[]
--- @field group? integer|string
--- @field pattern? string|string[]
--- @field buffer? integer|integer[]
--- @field id? integer

--- @class vim.api.keyset.get_commands
--- @field builtin? boolean

--- @class vim.api.keyset.get_extmark
--- @field details? boolean
--- @field hl_name? boolean

--- @class vim.api.keyset.get_extmarks
--- @field limit? integer
--- @field details? boolean
--- @field hl_name? boolean
--- @field overlap? boolean
--- @field type? string

--- @class vim.api.keyset.get_highlight
--- @field id? integer
--- @field name? string
--- @field link? boolean
--- @field create? boolean

--- @class vim.api.keyset.get_ns
--- @field winid? integer

--- @class vim.api.keyset.highlight
--- @field bold? boolean
--- @field standout? boolean
--- @field strikethrough? boolean
--- @field underline? boolean
--- @field undercurl? boolean
--- @field underdouble? boolean
--- @field underdotted? boolean
--- @field underdashed? boolean
--- @field italic? boolean
--- @field reverse? boolean
--- @field altfont? boolean
--- @field nocombine? boolean
--- @field default? boolean
--- @field cterm? integer|string
--- @field foreground? integer|string
--- @field fg? integer|string
--- @field background? integer|string
--- @field bg? integer|string
--- @field ctermfg? integer|string
--- @field ctermbg? integer|string
--- @field special? integer|string
--- @field sp? integer|string
--- @field link? integer|string
--- @field global_link? integer|string
--- @field fallback? boolean
--- @field blend? integer
--- @field fg_indexed? boolean
--- @field bg_indexed? boolean
--- @field force? boolean
--- @field url? string

--- @class vim.api.keyset.highlight_cterm
--- @field bold? boolean
--- @field standout? boolean
--- @field strikethrough? boolean
--- @field underline? boolean
--- @field undercurl? boolean
--- @field underdouble? boolean
--- @field underdotted? boolean
--- @field underdashed? boolean
--- @field italic? boolean
--- @field reverse? boolean
--- @field altfont? boolean
--- @field nocombine? boolean

--- @class vim.api.keyset.keymap
--- @field noremap? boolean
--- @field nowait? boolean
--- @field silent? boolean
--- @field script? boolean
--- @field expr? boolean
--- @field unique? boolean
--- @field callback? function
--- @field desc? string
--- @field replace_keycodes? boolean

--- @class vim.api.keyset.ns_opts
--- @field wins? any[]

--- @class vim.api.keyset.open_term
--- @field on_input? fun(_: "input", term: integer, bufnr: integer, data: any)
--- @field force_crlf? boolean

--- @class vim.api.keyset.option
--- @field scope? string
--- @field win? integer
--- @field buf? integer
--- @field filetype? string

--- @class vim.api.keyset.redraw
--- @field flush? boolean
--- @field cursor? boolean
--- @field valid? boolean
--- @field statuscolumn? boolean
--- @field statusline? boolean
--- @field tabline? boolean
--- @field winbar? boolean
--- @field range? any[]
--- @field win? integer
--- @field buf? integer

--- @class vim.api.keyset.runtime
--- @field is_lua? boolean
--- @field do_source? boolean

--- @class vim.api.keyset.set_decoration_provider
--- @field on_start? fun(_: "start", tick: integer): boolean?
--- @field on_buf? fun(_: "buf", bufnr: integer, tick: integer)
--- @field on_win? fun(_: "win", winid: integer, bufnr: integer, toprow: integer, botrow: integer): boolean?
--- @field on_line? fun(_: "line", winid: integer, bufnr: integer, row: integer): boolean?
--- @field on_end? fun(_: "end", tick: integer)
--- @field _on_hl_def? fun(_: "hl_def")
--- @field _on_spell_nav? fun(_: "spell_nav")
--- @field _on_conceal_line? fun(_: "conceal_line")

--- @class vim.api.keyset.set_extmark
--- @field id? integer
--- @field end_line? integer
--- @field end_row? integer
--- @field end_col? integer
--- @field hl_group? any
--- @field virt_text? any[]
--- @field virt_text_pos? string
--- @field virt_text_win_col? integer
--- @field virt_text_hide? boolean
--- @field virt_text_repeat_linebreak? boolean
--- @field hl_eol? boolean
--- @field hl_mode? string
--- @field invalidate? boolean
--- @field ephemeral? boolean
--- @field priority? integer
--- @field right_gravity? boolean
--- @field end_right_gravity? boolean
--- @field virt_lines? any[]
--- @field virt_lines_above? boolean
--- @field virt_lines_leftcol? boolean
--- @field virt_lines_overflow? string
--- @field strict? boolean
--- @field sign_text? string
--- @field sign_hl_group? integer|string
--- @field number_hl_group? integer|string
--- @field line_hl_group? integer|string
--- @field cursorline_hl_group? integer|string
--- @field conceal? string
--- @field conceal_lines? string
--- @field spell? boolean
--- @field ui_watched? boolean
--- @field undo_restore? boolean
--- @field url? string
--- @field scoped? boolean

--- @class vim.api.keyset.user_command
--- @field addr? any
--- @field bang? boolean
--- @field bar? boolean
--- @field complete? any
--- @field count? any
--- @field desc? any
--- @field force? boolean
--- @field keepscript? boolean
--- @field nargs? any
--- @field preview? any
--- @field range? any
--- @field register? boolean

--- @class vim.api.keyset.win_config
--- @field row? number
--- @field col? number
--- @field width? integer
--- @field height? integer
--- @field anchor? 'NW'|'NE'|'SW'|'SE'
--- @field relative? 'cursor'|'editor'|'laststatus'|'mouse'|'tabline'|'win'
--- @field split? 'left'|'right'|'above'|'below'
--- @field win? integer
--- @field bufpos? integer[]
--- @field external? boolean
--- @field focusable? boolean
--- @field mouse? boolean
--- @field vertical? boolean
--- @field zindex? integer
--- @field border? 'none'|'single'|'double'|'rounded'|'solid'|'shadow'|string[]
--- @field title? any
--- @field title_pos? 'center'|'left'|'right'
--- @field footer? any
--- @field footer_pos? 'center'|'left'|'right'
--- @field style? 'minimal'
--- @field noautocmd? boolean
--- @field fixed? boolean
--- @field hide? boolean

--- @class vim.api.keyset.win_text_height
--- @field start_row? integer
--- @field end_row? integer
--- @field start_vcol? integer
--- @field end_vcol? integer

--- @class vim.api.keyset.xdl_diff
--- @field on_hunk? fun(start_a: integer, count_a: integer, start_b: integer, count_b: integer): integer?
--- @field result_type? string
--- @field algorithm? string
--- @field ctxlen? integer
--- @field interhunkctxlen? integer
--- @field linematch? boolean|integer
--- @field ignore_whitespace? boolean
--- @field ignore_whitespace_change? boolean
--- @field ignore_whitespace_change_at_eol? boolean
--- @field ignore_cr_at_eol? boolean
--- @field ignore_blank_lines? boolean
--- @field indent_heuristic? boolean
```

---

## api_keysets_extra

Additional types that are used in Neovim's API.

```lua
--- @meta _
error('Cannot require a meta file')

--- Extra types we can't generate keysets for

--- @class vim.api.keyset.extmark_details
--- @field ns_id integer
--- @field right_gravity boolean
---
--- @field end_row? integer
--- @field end_col? integer
--- @field end_right_gravity? integer
---
--- @field priority? integer
---
--- @field undo_restore? false
--- @field invalidate? true
--- @field invalid? true
---
--- @field hl_group? string
--- @field hl_eol? boolean
---
--- @field conceal? boolean
--- @field spell? boolean
--- @field ui_watched? boolean
--- @field url? string
--- @field hl_mode? string
---
--- @field virt_text? [string, string][]
--- @field virt_text_hide? boolean
--- @field virt_text_repeat_linebreak? boolean
--- @field virt_text_win_col? integer
--- @field virt_text_pos? string
---
--- @field virt_lines? [string, string][][]
--- @field virt_lines_above? boolean
--- @field virt_lines_leftcol? boolean
---
--- @field sign_text? string
--- @field sign_name? string
--- @field sign_hl_group? string
--- @field number_hl_group? string
--- @field line_hl_group? string
--- @field cursorline_hl_group? string

--- @class vim.api.keyset.get_extmark_item_by_id
--- @field [1] integer row
--- @field [2] integer col
--- @field [3] vim.api.keyset.extmark_details?

--- @class vim.api.keyset.get_extmark_item
--- @field [1] integer extmark_id
--- @field [2] integer row
--- @field [3] integer col
--- @field [4] vim.api.keyset.extmark_details?

--- @class vim.api.keyset.get_mark
--- @field [1] integer row
--- @field [2] integer col
--- @field [3] integer buffer
--- @field [4] string buffername

--- @class vim.api.keyset.get_autocmds.ret
--- @field id? integer
--- @field group? integer
--- @field group_name? integer
--- @field desc? string
--- @field event? string
--- @field command? string
--- @field callback? function
--- @field once? boolean
--- @field pattern? string
--- @field buflocal? boolean
--- @field buffer? integer

--- @class vim.api.keyset.create_autocmd.callback_args
--- @field id integer autocommand id
--- @field event string name of the triggered event |autocmd-events|
--- @field group? integer autocommand group id, if any
--- @field match string expanded value of <amatch>
--- @field buf integer expanded value of <abuf>
--- @field file string expanded value of <afile>
--- @field data? any arbitrary data passed from |nvim_exec_autocmds()| *event-data*

--- @class vim.api.keyset.create_user_command.command_args
--- @field name string Command name
---
--- The args passed to the command, if any <args>
--- @field args string
---
--- The args split by unescaped whitespace
--- (when more than one argument is allowed), if any <f-args>
--- @field fargs string[]
---
--- Number of arguments |:command-nargs|
--- @field nargs string
---
--- "true" if the command was executed with a ! modifier <bang>
--- @field bang boolean
---
--- The starting line of the command range <line1>
--- @field line1 integer
---
--- The final line of the command range <line2>
--- @field line2 integer
---
--- The number of items in the command range: 0, 1, or 2 <range>
--- @field range integer
---
--- Any count supplied <count>
--- @field count integer
--- The optional register, if specified <reg>
--- @field reg string
--- Command modifiers, if any <mods>
--- @field mods string
---
--- Command modifiers in a structured format. Has the same structure as the
--- "mods" key of |nvim_parse_cmd()|.
--- @field smods table

--- @class vim.api.keyset.command_info
--- @field name string
--- @field definition string
--- @field script_id integer
--- @field bang boolean
--- @field bar boolean
--- @field register boolean
--- @field keepscript boolean
--- @field preview boolean
--- @field nargs string
--- @field complete? string
--- @field complete_arg? string
--- @field count? string
--- @field range? string
--- @field addr? string

--- @class vim.api.keyset.hl_info.base
--- @field reverse? true
--- @field bold? true
--- @field italic? true
--- @field underline? true
--- @field undercurl? true
--- @field underdouble? true
--- @field underdotted? true
--- @field underdashed? true
--- @field standout? true
--- @field strikethrough? true
--- @field altfont? true
--- @field nocombine? true
--- @field ctermfg? integer
--- @field ctermbg? integer

--- @class vim.api.keyset.hl_info.cterm : vim.api.keyset.hl_info.base
--- @field foreground? integer
--- @field background? integer

--- @class vim.api.keyset.get_hl_info : vim.api.keyset.hl_info.base
--- @field fg? integer
--- @field bg? integer
--- @field sp? integer
--- @field default? true
--- @field link? string
--- @field blend? integer
--- @field cterm? vim.api.keyset.hl_info.cterm

--- @class vim.api.keyset.set_hl_info : vim.api.keyset.hl_info.base
--- @field fg? integer|string
--- @field bg? integer|string
--- @field sp? integer|string
--- @field default? true
--- @field link? string
--- @field blend? integer
--- @field force? true
--- @field cterm? vim.api.keyset.hl_info.cterm

--- @class vim.api.keyset.get_keymap
--- @field abbr? 0|1
--- @field buffer? 0|1
--- @field callback? function
--- @field desc? string
--- @field expr? 0|1
--- @field lhs? string
--- @field lhsraw? string
--- @field lhsrawalt? string
--- @field lnum? integer
--- @field mode? string
--- @field mode_bits? integer
--- @field noremap? 0|1
--- @field nowait? 0|1
--- @field rhs? string
--- @field script? 0|1
--- @field scriptversion? integer
--- @field sid? integer
--- @field silent? 0|1

--- @class vim.api.keyset.get_mode
--- @field blocking boolean
--- @field mode string

--- @class vim.api.keyset.get_option_info
--- @field name string
--- @field shortname string
--- @field scope 'buf'|'win'|'global'
--- @field global_local boolean
--- @field commalist boolean
--- @field flaglist boolean
--- @field was_set boolean
--- @field last_set_sid integer
--- @field last_set_linenr integer
--- @field last_set_chan integer
--- @field type 'string'|'boolean'|'number'
--- @field default string|boolean|integer
--- @field allows_duplicates boolean

--- @class vim.api.keyset.parse_cmd.mods
--- @field filter { force: boolean, pattern: string }
--- @field silent boolean
--- @field emsg_silent boolean
--- @field unsilent boolean
--- @field sandbox boolean
--- @field noautocmd boolean
--- @field tab integer
--- @field verbose integer
--- @field browse boolean
--- @field confirm boolean
--- @field hide boolean
--- @field keepalt boolean
--- @field keepjumps boolean
--- @field keepmarks boolean
--- @field keeppatterns boolean
--- @field lockmarks boolean
--- @field noswapfile boolean
--- @field vertical boolean
--- @field horizontal boolean
--- @field split ''|'botright'|'topleft'|'belowright'|'aboveleft'

--- @class vim.api.keyset.parse_cmd
--- @field addr 'line'|'arg'|'buf'|'load'|'win'|'tab'|'qf'|'none'|'?'
--- @field args string[]
--- @field bang boolean
--- @field cmd string
--- @field magic {bar: boolean, file: boolean}
--- @field mods vim.api.keyset.parse_cmd.mods
--- @field nargs '0'|'1'|'?'|'+'|'*'
--- @field nextcmd string
--- @field range? integer[]
--- @field count? integer
--- @field reg? string
```

---

## builtin_types

Various types used by Neovim's builtin APIs.

```lua
--- @class vim.fn.sign
--- @field group string
--- @field id integer
--- @field lnum integer
--- @field name string
--- @field priority integer

--- @class vim.fn.getbufinfo.dict
--- @field buflisted? 0|1
--- @field bufloaded? 0|1
--- @field bufmodified? 0|1

--- @class vim.fn.getbufinfo.ret.item
--- @field bufnr integer
--- @field changed 0|1
--- @field changedtick integer
--- @field hidden 0|1
--- @field lastused integer
--- @field linecount integer
--- @field listed 0|1
--- @field lnum integer
--- @field loaded 0|1
--- @field name string
--- @field signs vim.fn.sign[]
--- @field variables table<string,any>
--- @field windows integer[]

--- @alias vim.fn.getjumplist.ret [vim.fn.getjumplist.ret.item[], integer]

--- @class vim.fn.getjumplist.ret.item
--- @field bufnr integer
--- @field col integer
--- @field coladd integer
--- @field filename? string
--- @field lnum integer

--- @class vim.fn.getmarklist.ret.item
--- @field mark string
--- @field pos [integer, integer, integer, integer]
--- @field file string

--- @class vim.fn.getmousepos.ret
--- @field screenrow integer
--- @field screencol integer
--- @field winid integer
--- @field winrow integer
--- @field wincol integer
--- @field line integer
--- @field column integer

--- @class vim.fn.getwininfo.ret.item
--- @field botline integer
--- @field bufnr integer
--- @field height integer
--- @field loclist integer
--- @field quickfix integer
--- @field tabnr integer
--- @field terminal integer
--- @field textoff integer
--- @field topline integer
--- @field variables table<string,any>
--- @field width integer
--- @field winbar integer
--- @field wincol integer
--- @field winid integer
--- @field winnr integer
--- @field winrow integer

--- @class vim.quickfix.entry
--- buffer number; must be the number of a valid buffer
--- @field bufnr? integer
---
--- name of a file; only used when "bufnr" is not
--- present or it is invalid.
--- @field filename? string
---
--- name of a module; if given it will be used in
--- quickfix error window instead of the filename.
--- @field module? string
---
--- line number in the file
--- @field lnum? integer
---
--- end of lines, if the item spans multiple lines
--- @field end_lnum? integer
---
--- search pattern used to locate the error
--- @field pattern? string
---
--- column number
--- @field col? integer
---
--- when non-zero: "col" is visual column
--- when zero: "col" is byte index
--- @field vcol? integer
---
--- end column, if the item spans multiple columns
--- @field end_col? integer
---
--- error number
--- @field nr? integer
---
--- description of the error
--- @field text? string
---
--- single-character error type, 'E', 'W', etc.
--- @field type? string
---
--- recognized error message
--- @field valid? boolean
---
--- custom data associated with the item, can be
--- any type.
--- @field user_data? any

--- @class vim.fn.setqflist.what
---
--- quickfix list context. See |quickfix-context|
--- @field context? table
---
--- errorformat to use when parsing text from
--- "lines". If this is not present, then the
--- 'errorformat' option value is used.
--- See |quickfix-parse|
--- @field efm? string
---
--- quickfix list identifier |quickfix-ID|
--- @field id? integer
--- index of the current entry in the quickfix
--- list specified by "id" or "nr". If set to '$',
--- then the last entry in the list is set as the
--- current entry. See |quickfix-index|
--- @field idx? integer
---
--- list of quickfix entries. Same as the {list}
--- argument.
--- @field items? vim.quickfix.entry[]
---
--- use 'errorformat' to parse a list of lines and
--- add the resulting entries to the quickfix list
--- {nr} or {id}. Only a |List| value is supported.
--- See |quickfix-parse|
--- @field lines? string[]
---
--- list number in the quickfix stack; zero
--- means the current quickfix list and "$" means
--- the last quickfix list.
--- @field nr? integer
---
--- function to get the text to display in the
--- quickfix window. The value can be the name of
--- a function or a funcref or a lambda. Refer
--- to |quickfix-window-function| for an explanation
--- of how to write the function and an example.
--- @field quickfixtextfunc? function
---
--- quickfix list title text. See |quickfix-title|
--- @field title? string

--- @class vim.fn.sign_define.dict
--- @field text string
--- @field icon? string
--- @field linehl? string
--- @field numhl? string
--- @field texthl? string
--- @field culhl? string

--- @class vim.fn.sign_getdefined.ret.item
--- @field name string
--- @field text string
--- @field icon? string
--- @field texthl? string
--- @field culhl? string
--- @field numhl? string
--- @field linehl? string

--- @class vim.fn.sign_getplaced.dict
--- @field group? string
--- @field id? integer
--- @field lnum? string|integer

--- @class vim.fn.sign_getplaced.ret.item
--- @field bufnr integer
--- @field signs vim.fn.sign[]

--- @class vim.fn.sign_place.dict
--- @field lnum? integer|string
--- @field priority? integer

--- @class vim.fn.sign_placelist.list.item
--- @field buffer integer|string
--- @field group? string
--- @field id? integer
--- @field lnum? integer|string
--- @field name string
--- @field priority? integer

--- @class vim.fn.sign_unplace.dict
--- @field buffer? integer|string
--- @field id? integer

--- @class vim.fn.sign_unplacelist.list.item
--- @field buffer? integer|string
--- @field group? string
--- @field id? integer

--- @class vim.fn.winrestview.dict
--- @field col? integer
--- @field coladd? integer
--- @field curswant? integer
--- @field leftcol? integer
--- @field lnum? integer
--- @field skipcol? integer
--- @field topfill? integer
--- @field topline? integer

--- @class vim.fn.winsaveview.ret: vim.fn.winrestview.dict
--- @field col integer
--- @field coladd integer
--- @field curswant integer
--- @field leftcol integer
--- @field lnum integer
--- @field skipcol integer
--- @field topfill integer
--- @field topline integer

--- @class vim.fn.getscriptinfo.ret
--- @field autoload false
--- @field functions? string[]
--- @field name string
--- @field sid string
--- @field variables? table<string, any>
--- @field version 1

--- @class vim.fn.undotree.entry
---
--- Undo sequence number. Same as what appears in
--- \|:undolist|.
--- @field seq integer
---
--- Timestamp when the change happened. Use
--- \|strftime()| to convert to something readable.
--- @field time integer
---
--- Only appears in the item that is the last one
--- that was added. This marks the last change
--- and where further changes will be added.
--- @field newhead? integer
---
--- Only appears in the item that is the last one
--- that was undone. This marks the current
--- position in the undo tree, the block that will
--- be used by a redo command. When nothing was
--- undone after the last change this item will
--- not appear anywhere.
--- @field curhead? integer
---
--- Only appears on the last block before a file
--- write. The number is the write count. The
--- first write has number 1, the last one the
--- "save_last" mentioned above.
--- @field save? integer
---
--- Alternate entry. This is again a List of undo
--- blocks. Each item may again have an "alt"
--- item.
--- @field alt? vim.fn.undotree.entry[]

--- @class vim.fn.undotree.ret
---
--- The highest undo sequence number used.
--- @field seq_last integer
---
--- The sequence number of the current position in
--- the undo tree. This differs from "seq_last"
--- when some changes were undone.
--- @field seq_cur integer
---
--- Time last used for |:earlier| and related
--- commands. Use |strftime()| to convert to
--- something readable.
--- @field time_cur integer
---
--- Number of the last file write. Zero when no
--- write yet.
--- @field save_last integer
---
--- Number of the current position in the undo
--- tree.
--- @field save_cur integer
---
--- Non-zero when the last undo block was synced.
--- This happens when waiting from input from the
--- user. See |undo-blocks|.
--- @field synced integer
---
--- A list of dictionaries with information about
--- undo blocks.
--- @field entries vim.fn.undotree.entry[]

--- @class vim.fn.winlayout.leaf
--- @field [1] "leaf" Node type
--- @field [2] integer winid

--- @class vim.fn.winlayout.branch
--- @field [1] "row" | "col" Node type
--- @field [2] (vim.fn.winlayout.leaf|vim.fn.winlayout.branch)[] children

--- @class vim.fn.winlayout.empty

--- @alias vim.fn.winlayout.ret
--- | vim.fn.winlayout.leaf
--- | vim.fn.winlayout.branch
--- | vim.fn.winlayout.empty
```

---

## vimfn

Functions available from Neovim's vimscript APIs. They are available via `vim.fn.*`.

```lua
vim.fn.abs(expr) 
vim.fn.acos(expr) 
vim.fn.add(object, expr) 
vim.fn['and'] = function(expr, expr1) 
vim.fn.api_info() 
vim.fn.append(lnum, text) 
vim.fn.appendbufline(buf, lnum, text) 
vim.fn.argc(winid) 
vim.fn.argidx() 
vim.fn.arglistid(winnr, tabnr) 
vim.fn.argv(nr, winid) 
vim.fn.asin(expr) 
vim.fn.assert_beeps(cmd) 
vim.fn.assert_equal(expected, actual, msg) 
vim.fn.assert_equalfile(fname_one, fname_two) 
vim.fn.assert_exception(error, msg) 
vim.fn.assert_fails(cmd, error, msg, lnum, context) 
vim.fn.assert_false(actual, msg) 
vim.fn.assert_inrange(lower, upper, actual, msg) 
vim.fn.assert_match(pattern, actual, msg) 
vim.fn.assert_nobeep(cmd) 
vim.fn.assert_notequal(expected, actual, msg) 
vim.fn.assert_notmatch(pattern, actual, msg) 
vim.fn.assert_report(msg) 
vim.fn.assert_true(actual, msg) 
vim.fn.atan(expr) 
vim.fn.atan2(expr1, expr2) 
vim.fn.blob2list(blob) 
vim.fn.browse(save, title, initdir, default) 
vim.fn.browsedir(title, initdir) 
vim.fn.bufadd(name) 
vim.fn.bufexists(buf) 
vim.fn.buffer_exists(...) 
vim.fn.buffer_name(...) 
vim.fn.buffer_number(...) 
vim.fn.buflisted(buf) 
vim.fn.bufload(buf) 
vim.fn.bufloaded(buf) 
vim.fn.bufname(buf) 
vim.fn.bufnr(buf, create) 
vim.fn.bufwinid(buf) 
vim.fn.bufwinnr(buf) 
vim.fn.byte2line(byte) 
vim.fn.byteidx(expr, nr, utf16) 
vim.fn.byteidxcomp(expr, nr, utf16) 
vim.fn.call(func, arglist, dict) 
vim.fn.ceil(expr) 
vim.fn.chanclose(id, stream) 
vim.fn.changenr() 
vim.fn.chansend(id, data) 
vim.fn.char2nr(string, utf8) 
vim.fn.charclass(string) 
vim.fn.charcol(expr, winid) 
vim.fn.charidx(string, idx, countcc, utf16) 
vim.fn.chdir(dir) 
vim.fn.cindent(lnum) 
vim.fn.clearmatches(win) 
vim.fn.col(expr, winid) 
vim.fn.complete(startcol, matches) 
vim.fn.complete_add(expr) 
vim.fn.complete_check() 
vim.fn.complete_info(what) 
vim.fn.confirm(msg, choices, default, type) 
vim.fn.copy(expr) 
vim.fn.cos(expr) 
vim.fn.cosh(expr) 
vim.fn.count(comp, expr, ic, start) 
vim.fn.ctxget(index) 
vim.fn.ctxpop() 
vim.fn.ctxpush(types) 
vim.fn.ctxset(context, index) 
vim.fn.ctxsize() 
vim.fn.cursor(lnum, col, off) 
vim.fn.cursor(list) 
vim.fn.debugbreak(pid) 
vim.fn.deepcopy(expr, noref) 
vim.fn.delete(fname, flags) 
vim.fn.deletebufline(buf, first, last) 
vim.fn.dictwatcheradd(dict, pattern, callback) 
vim.fn.dictwatcherdel(dict, pattern, callback) 
vim.fn.did_filetype() 
vim.fn.diff_filler(lnum) 
vim.fn.diff_hlID(lnum, col) 
vim.fn.digraph_get(chars) 
vim.fn.digraph_getlist(listall) 
vim.fn.digraph_set(chars, digraph) 
vim.fn.digraph_setlist(digraphlist) 
vim.fn.empty(expr) 
vim.fn.environ() 
vim.fn.escape(string, chars) 
vim.fn.eval(string) 
vim.fn.eventhandler() 
vim.fn.executable(expr) 
vim.fn.execute(command, silent) 
vim.fn.exepath(expr) 
vim.fn.exists(expr) 
vim.fn.exp(expr) 
vim.fn.expand(string, nosuf, list) 
vim.fn.expand(string, nosuf, list) 
vim.fn.expandcmd(string, options) 
vim.fn.extend(expr1, expr2, expr3) 
vim.fn.extendnew(expr1, expr2, expr3) 
vim.fn.feedkeys(string, mode) 
vim.fn.file_readable(file) 
vim.fn.filecopy(from, to) 
vim.fn.filereadable(file) 
vim.fn.filewritable(file) 
vim.fn.filter(expr1, expr2) 
vim.fn.finddir(name, path, count) 
vim.fn.findfile(name, path, count) 
vim.fn.flatten(list, maxdepth) 
vim.fn.flattennew(list, maxdepth) 
vim.fn.float2nr(expr) 
vim.fn.floor(expr) 
vim.fn.fmod(expr1, expr2) 
vim.fn.fnameescape(string) 
vim.fn.fnamemodify(fname, mods) 
vim.fn.foldclosed(lnum) 
vim.fn.foldclosedend(lnum) 
vim.fn.foldlevel(lnum) 
vim.fn.foldtext() 
vim.fn.foldtextresult(lnum) 
vim.fn.foreach(expr1, expr2) 
vim.fn.fullcommand(name) 
vim.fn.funcref(name, arglist, dict) 
vim.fn['function'] = function(name, arglist, dict) 
vim.fn.garbagecollect(atexit) 
vim.fn.get(list, idx, default) 
vim.fn.get(blob, idx, default) 
vim.fn.get(dict, key, default) 
vim.fn.get(func, what) 
vim.fn.getbufinfo(buf) 
vim.fn.getbufinfo(dict) 
vim.fn.getbufline(buf, lnum, end_) 
vim.fn.getbufoneline(buf, lnum) 
vim.fn.getbufvar(buf, varname, def) 
vim.fn.getcellwidths() 
vim.fn.getchangelist(buf) 
vim.fn.getchar(expr, opts) 
vim.fn.getcharmod() 
vim.fn.getcharpos(expr) 
vim.fn.getcharsearch() 
vim.fn.getcharstr(expr, opts) 
vim.fn.getcmdcomplpat() 
vim.fn.getcmdcompltype() 
vim.fn.getcmdline() 
vim.fn.getcmdpos() 
vim.fn.getcmdprompt() 
vim.fn.getcmdscreenpos() 
vim.fn.getcmdtype() 
vim.fn.getcmdwintype() 
vim.fn.getcompletion(pat, type, filtered) 
vim.fn.getcurpos(winid) 
vim.fn.getcursorcharpos(winid) 
vim.fn.getcwd(winnr, tabnr) 
vim.fn.getenv(name) 
vim.fn.getfontname(name) 
vim.fn.getfperm(fname) 
vim.fn.getfsize(fname) 
vim.fn.getftime(fname) 
vim.fn.getftype(fname) 
vim.fn.getjumplist(winnr, tabnr) 
vim.fn.getline(lnum, end_) 
vim.fn.getline(lnum, end_) 
vim.fn.getloclist(nr, what) 
vim.fn.getmarklist(buf) 
vim.fn.getmatches(win) 
vim.fn.getmousepos() 
vim.fn.getpid() 
vim.fn.getpos(expr) 
vim.fn.getqflist(what) 
vim.fn.getreg(regname, list) 
vim.fn.getreg(regname, list) 
vim.fn.getreginfo(regname) 
vim.fn.getregion(pos1, pos2, opts) 
vim.fn.getregionpos(pos1, pos2, opts) 
vim.fn.getregtype(regname) 
vim.fn.getscriptinfo(opts) 
vim.fn.getstacktrace() 
vim.fn.gettabinfo(tabnr) 
vim.fn.gettabvar(tabnr, varname, def) 
vim.fn.gettabwinvar(tabnr, winnr, varname, def) 
vim.fn.gettagstack(winnr) 
vim.fn.gettext(text) 
vim.fn.getwininfo(winid) 
vim.fn.getwinpos(timeout) 
vim.fn.getwinposx() 
vim.fn.getwinposy() 
vim.fn.getwinvar(winnr, varname, def) 
vim.fn.glob(expr, nosuf, list, alllinks) 
vim.fn.glob2regpat(string) 
vim.fn.globpath(path, expr, nosuf, list, allinks) 
vim.fn.has(feature) 
vim.fn.has_key(dict, key) 
vim.fn.haslocaldir(winnr, tabnr) 
vim.fn.hasmapto(what, mode, abbr) 
vim.fn.highlightID(name) 
vim.fn.highlight_exists(name) 
vim.fn.histadd(history, item) 
vim.fn.histdel(history, item) 
vim.fn.histget(history, index) 
vim.fn.histnr(history) 
vim.fn.hlID(name) 
vim.fn.hlexists(name) 
vim.fn.hostname() 
vim.fn.iconv(string, from, to) 
vim.fn.id(expr) 
vim.fn.indent(lnum) 
vim.fn.index(object, expr, start, ic) 
vim.fn.indexof(object, expr, opts) 
vim.fn.input(prompt, text, completion) 
vim.fn.input(opts) 
vim.fn.inputdialog(...) 
vim.fn.inputlist(textlist) 
vim.fn.inputrestore() 
vim.fn.inputsave() 
vim.fn.inputsecret(prompt, text) 
vim.fn.insert(object, item, idx) 
vim.fn.interrupt() 
vim.fn.invert(expr) 
vim.fn.isabsolutepath(path) 
vim.fn.isdirectory(directory) 
vim.fn.isinf(expr) 
vim.fn.islocked(expr) 
vim.fn.isnan(expr) 
vim.fn.items(dict) 
vim.fn.jobclose(...) 
vim.fn.jobpid(job) 
vim.fn.jobresize(job, width, height) 
vim.fn.jobsend(...) 
vim.fn.jobstart(cmd, opts) 
vim.fn.jobstop(id) 
vim.fn.jobwait(jobs, timeout) 
vim.fn.join(list, sep) 
vim.fn.json_decode(expr) 
vim.fn.json_encode(expr) 
vim.fn.keys(dict) 
vim.fn.keytrans(string) 
vim.fn.last_buffer_nr() 
vim.fn.len(expr) 
vim.fn.libcall(libname, funcname, argument) 
vim.fn.libcallnr(libname, funcname, argument) 
vim.fn.line(expr, winid) 
vim.fn.line2byte(lnum) 
vim.fn.lispindent(lnum) 
vim.fn.list2blob(list) 
vim.fn.list2str(list, utf8) 
vim.fn.localtime() 
vim.fn.log(expr) 
vim.fn.log10(expr) 
vim.fn.map(expr1, expr2) 
vim.fn.maparg(name, mode, abbr, dict) 
vim.fn.maparg(name, mode, abbr, dict) 
vim.fn.mapcheck(name, mode, abbr) 
vim.fn.maplist(abbr) 
vim.fn.mapnew(expr1, expr2) 
vim.fn.mapset(mode, abbr, dict) 
vim.fn.mapset(dict) 
vim.fn.match(expr, pat, start, count) 
vim.fn.matchadd(group, pattern, priority, id, dict) 
vim.fn.matchaddpos(group, pos, priority, id, dict) 
vim.fn.matcharg(nr) 
vim.fn.matchbufline(buf, pat, lnum, end_, dict) 
vim.fn.matchdelete(id, win) 
vim.fn.matchend(expr, pat, start, count) 
vim.fn.matchfuzzy(list, str, dict) 
vim.fn.matchfuzzypos(list, str, dict) 
vim.fn.matchlist(expr, pat, start, count) 
vim.fn.matchstr(expr, pat, start, count) 
vim.fn.matchstrlist(list, pat, dict) 
vim.fn.matchstrpos(expr, pat, start, count) 
vim.fn.max(expr) 
vim.fn.menu_get(path, modes) 
vim.fn.menu_info(name, mode) 
vim.fn.min(expr) 
vim.fn.mkdir(name, flags, prot) 
vim.fn.mode(expr) 
vim.fn.msgpackdump(list, type) 
vim.fn.msgpackparse(data) 
vim.fn.nextnonblank(lnum) 
vim.fn.nr2char(expr, utf8) 
vim.fn['or'] = function(expr, expr1) 
vim.fn.pathshorten(path, len) 
vim.fn.perleval(expr) 
vim.fn.pow(x, y) 
vim.fn.prevnonblank(lnum) 
vim.fn.printf(fmt, expr1) 
vim.fn.prompt_getprompt(buf) 
vim.fn.prompt_setcallback(buf, expr) 
vim.fn.prompt_setinterrupt(buf, expr) 
vim.fn.prompt_setprompt(buf, text) 
vim.fn.pum_getpos() 
vim.fn.pumvisible() 
vim.fn.py3eval(expr) 
vim.fn.pyeval(expr) 
vim.fn.pyxeval(expr) 
vim.fn.rand(expr) 
vim.fn.range(expr, max, stride) 
vim.fn.readblob(fname, offset, size) 
vim.fn.readdir(directory, expr) 
vim.fn.readfile(fname, type, max) 
vim.fn.reduce(object, func, initial) 
vim.fn.reg_executing() 
vim.fn.reg_recorded() 
vim.fn.reg_recording() 
vim.fn.reltime() 
vim.fn.reltime(start) 
vim.fn.reltime(start, end_) 
vim.fn.reltimefloat(time) 
vim.fn.reltimestr(time) 
vim.fn.remove(list, idx) 
vim.fn.remove(list, idx, end_) 
vim.fn.remove(blob, idx) 
vim.fn.remove(blob, idx, end_) 
vim.fn.remove(dict, key) 
vim.fn.rename(from, to) 
vim.fn['repeat'] = function(expr, count) 
vim.fn.resolve(filename) 
vim.fn.reverse(object) 
vim.fn.round(expr) 
vim.fn.rpcnotify(channel, event, ...) 
vim.fn.rpcrequest(channel, method, ...) 
vim.fn.rpcstart(prog, argv) 
vim.fn.rpcstop(...) 
vim.fn.rubyeval(expr) 
vim.fn.screenattr(row, col) 
vim.fn.screenchar(row, col) 
vim.fn.screenchars(row, col) 
vim.fn.screencol() 
vim.fn.screenpos(winid, lnum, col) 
vim.fn.screenrow() 
vim.fn.screenstring(row, col) 
vim.fn.search(pattern, flags, stopline, timeout, skip) 
vim.fn.searchcount(options) 
vim.fn.searchdecl(name, global, thisblock) 
vim.fn.searchpair(start, middle, end_, flags, skip, stopline, timeout) 
vim.fn.searchpairpos(start, middle, end_, flags, skip, stopline, timeout) 
vim.fn.searchpos(pattern, flags, stopline, timeout, skip) 
vim.fn.serverlist() 
vim.fn.serverstart(address) 
vim.fn.serverstop(address) 
vim.fn.setbufline(buf, lnum, text) 
vim.fn.setbufvar(buf, varname, val) 
vim.fn.setcellwidths(list) 
vim.fn.setcharpos(expr, list) 
vim.fn.setcharsearch(dict) 
vim.fn.setcmdline(str, pos) 
vim.fn.setcmdpos(pos) 
vim.fn.setcursorcharpos(lnum, col, off) 
vim.fn.setcursorcharpos(list) 
vim.fn.setenv(name, val) 
vim.fn.setfperm(fname, mode) 
vim.fn.setline(lnum, text) 
vim.fn.setloclist(nr, list, action, what) 
vim.fn.setmatches(list, win) 
vim.fn.setpos(expr, list) 
vim.fn.setqflist(list, action, what) 
vim.fn.setreg(regname, value, options) 
vim.fn.settabvar(tabnr, varname, val) 
vim.fn.settabwinvar(tabnr, winnr, varname, val) 
vim.fn.settagstack(nr, dict, action) 
vim.fn.setwinvar(nr, varname, val) 
vim.fn.sha256(string) 
vim.fn.shellescape(string, special) 
vim.fn.shiftwidth(col) 
vim.fn.sign_define(name, dict) 
vim.fn.sign_define(list) 
vim.fn.sign_getdefined(name) 
vim.fn.sign_getplaced(buf, dict) 
vim.fn.sign_jump(id, group, buf) 
vim.fn.sign_place(id, group, name, buf, dict) 
vim.fn.sign_placelist(list) 
vim.fn.sign_undefine(name) 
vim.fn.sign_undefine(list) 
vim.fn.sign_unplace(group, dict) 
vim.fn.sign_unplacelist(list) 
vim.fn.simplify(filename) 
vim.fn.sin(expr) 
vim.fn.sinh(expr) 
vim.fn.slice(expr, start, end_) 
vim.fn.sockconnect(mode, address, opts) 
vim.fn.sort(list, how, dict) 
vim.fn.soundfold(word) 
vim.fn.spellbadword(sentence) 
vim.fn.spellsuggest(word, max, capital) 
vim.fn.split(string, pattern, keepempty) 
vim.fn.sqrt(expr) 
vim.fn.srand(expr) 
vim.fn.state(what) 
vim.fn.stdioopen(opts) 
vim.fn.stdpath(what) 
vim.fn.stdpath(what) 
vim.fn.stdpath(what) 
vim.fn.str2float(string, quoted) 
vim.fn.str2list(string, utf8) 
vim.fn.str2nr(string, base) 
vim.fn.strcharlen(string) 
vim.fn.strcharpart(src, start, len, skipcc) 
vim.fn.strchars(string, skipcc) 
vim.fn.strdisplaywidth(string, col) 
vim.fn.strftime(format, time) 
vim.fn.strgetchar(str, index) 
vim.fn.stridx(haystack, needle, start) 
vim.fn.string(expr) 
vim.fn.strlen(string) 
vim.fn.strpart(src, start, len, chars) 
vim.fn.strptime(format, timestring) 
vim.fn.strridx(haystack, needle, start) 
vim.fn.strtrans(string) 
vim.fn.strutf16len(string, countcc) 
vim.fn.strwidth(string) 
vim.fn.submatch(nr, list) 
vim.fn.submatch(nr, list) 
vim.fn.substitute(string, pat, sub, flags) 
vim.fn.swapfilelist() 
vim.fn.swapinfo(fname) 
vim.fn.swapname(buf) 
vim.fn.synID(lnum, col, trans) 
vim.fn.synIDattr(synID, what, mode) 
vim.fn.synIDtrans(synID) 
vim.fn.synconcealed(lnum, col) 
vim.fn.synstack(lnum, col) 
vim.fn.system(cmd, input) 
vim.fn.systemlist(cmd, input, keepempty) 
vim.fn.tabpagebuflist(arg) 
vim.fn.tabpagenr(arg) 
vim.fn.tabpagewinnr(tabarg, arg) 
vim.fn.tagfiles() 
vim.fn.taglist(expr, filename) 
vim.fn.tan(expr) 
vim.fn.tanh(expr) 
vim.fn.tempname() 
vim.fn.termopen(cmd, opts) 
vim.fn.timer_info(id) 
vim.fn.timer_pause(timer, paused) 
vim.fn.timer_start(time, callback, options) 
vim.fn.timer_stop(timer) 
vim.fn.timer_stopall() 
vim.fn.tolower(expr) 
vim.fn.toupper(expr) 
vim.fn.tr(src, fromstr, tostr) 
vim.fn.trim(text, mask, dir) 
vim.fn.trunc(expr) 
vim.fn.type(expr) 
vim.fn.undofile(name) 
vim.fn.undotree(buf) 
vim.fn.uniq(list, func, dict) 
vim.fn.utf16idx(string, idx, countcc, charidx) 
vim.fn.values(dict) 
vim.fn.virtcol(expr, list, winid) 
vim.fn.virtcol2col(winid, lnum, col) 
vim.fn.visualmode(expr) 
vim.fn.wait(timeout, condition, interval) 
vim.fn.wildmenumode() 
vim.fn.win_execute(id, command, silent) 
vim.fn.win_findbuf(bufnr) 
vim.fn.win_getid(win, tab) 
vim.fn.win_gettype(nr) 
vim.fn.win_gotoid(expr) 
vim.fn.win_id2tabwin(expr) 
vim.fn.win_id2win(expr) 
vim.fn.win_move_separator(nr, offset) 
vim.fn.win_move_statusline(nr, offset) 
vim.fn.win_screenpos(nr) 
vim.fn.win_splitmove(nr, target, options) 
vim.fn.winbufnr(nr) 
vim.fn.wincol() 
vim.fn.windowsversion() 
vim.fn.winheight(nr) 
vim.fn.winlayout(tabnr) 
vim.fn.winline() 
vim.fn.winnr(arg) 
vim.fn.winrestcmd() 
vim.fn.winrestview(dict) 
vim.fn.winsaveview() 
vim.fn.winwidth(nr) 
vim.fn.wordcount() 
vim.fn.writefile(object, fname, flags) 
vim.fn.xor(expr, expr1) 
```
