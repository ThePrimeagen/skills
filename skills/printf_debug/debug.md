You are to debug this function, but you can only use printf strategy

please create a useful set of prints that will give the state of this function such that someone observing it can accurately understand whats happening

Do not make any changes to the code, keep the code, but add printf debugging.

If you truly believe you see a bug, add a comment explaining that you think this is where the bug is

<Vim>
If the file is in lua/<proj name>/*.lua then you need to use Vimism for printing.
Use: vim.inspect on anything that is not a string or number, including nil.
</Vim>

<TypeScript>
For typescript projects, there is often a logger.ts somewhere within that project,
make sure you use #debug print from the logger if there is one.  Do research for logging before creating the print statements.

if there is no logger, use console.log
</TypeScript>
