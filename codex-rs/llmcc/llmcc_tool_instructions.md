## `llmcc`

Use the `llmcc` shell command to indexing flies or folders, then extract dependenciy graphs for a symbol.

*** Full --help output
llmcc: llm context compiler

Usage: llmcc [OPTIONS] [FILE]...

Arguments:
  [FILE]...  Files to compile

Options:
  -d, --dir <DIR>     Load all .rs files from a directory (recursive)
      --lang <LANG>   Language to use: 'rust' or 'python' [default: rust]
      --print-ir      Print intermediate representation (IR), internal debugging output
      --print-graph   Print project graph or file structure graph
      --query <NAME>  Name of the symbol/function to query (enables find_depends mode)
      --recursive     Search recursively for transitive dependencies (default: direct dependencies only)
      --dependents    Return blocks that depend on the queried symbol instead of the ones it depends on
  -h, --help          Print help
  -V, --version       Print version


*** Please always use absolute path for --dir

*** Example invocation to query all dpenends code for `Codex`
```
  shell {"command":["llmcc", "--dir codex-rs/core --lang rust --query Codex"]}
```
