## `llmcc`

Use the `llmcc` shell command to indexing flies or folders, then extract dependenciy graphs for a symbol.

*** Full --help output
llmcc: llm context compiler

Usage: llmcc [OPTIONS] [FILE]...

Arguments:
  [FILE]...  Files to compile

Options:
  -d, --dir <DIR>                 Load all .rs files from a directory (recursive)
      --lang <LANG>               Language to use: 'rust' or 'python' [default: rust]
      --print-ir                  Print intermediate representation (IR), internal debugging output
      --print-block               Print basic block graph
      --project-graph             Print a project level graph focused on class relationships for dir, good for understanding high-level design architecture
      --pagerank                  Use page rank algorithm to filter the most important nodes in the project graph, this is **important** to balance speed
      --top-k <K>                 Top k nodes to select using PageRank algorithm
      --pagerank-direction <DIR>  PageRank direction: 'depends-on' to rank depended-upon nodes, 'depended-by' to rank orchestrators (default: depended-by) [default: depended-by]
      --query <NAME>              Name of the symbol/function to query (enables find_depends mode)
      --recursive                 Search recursively for transitive dependencies (default: direct dependencies only)
      --dependents                Return blocks that depend on the queried symbol instead of the ones it depends on
  -h, --help                      Print help
  -V, --version                   Print version


*** Please always use absolute path for --dir

*** Examples:
1. invocation to use pagerank algorithm to output a project level graph
```
  shell {"command":["llmcc", "--dir ../codex/codex-rs --project-graph --pagerank --top-k 100"]}
```

2. invocation to query all dpenends code for `Codex`
```
  shell {"command":["llmcc", "--dir codex-rs/core --lang rust --query Codex"]}
```
