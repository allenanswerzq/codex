## `llmcc`

Use the `llmcc` shell command to indexing flies or folders, then extract dependenciy graphs for a symbol.

*** help info
llmcc [OPTIONS] < --file <FILE>...|--dir <DIR>... >
Input (required, one of):

-f, --file <FILE>... — Individual files to compile (repeatable)
-d, --dir <DIR>... — Directories to scan recursively (repeatable)
Language (optional):

--lang <LANG> — Language: 'rust' or 'python' [default: rust]
Analysis (optional):

--design-graph — Generate high-level design graph
--pagerank --top-k <K> — Rank by importance (PageRank) and limit to top K
--query <NAME> — Symbol/function to analyze
--depends — Show what the symbol depends on
--dependents — Show what depends on the symbol
--recursive — Include transitive dependencies (vs. direct only)
Output format (optional):

--summary — Show file paths and line ranges (vs. full code texts)
--print-ir — Internal: print intermediate representation
--print-block — Internal: print basic block graph
Examples:

# Design graph with PageRank ranking
llmcc --dir crates --lang rust --design-graph --pagerank --top-k 100

# Dependencies and dependents of a symbol
llmcc --dir crates --lang rust --query CompileCtxt --depends
llmcc --dir crates --lang rust --query CompileCtxt --dependents --recursive

# Cross-directory analysis
llmcc --dir crates/llmcc-core/src --dir crates/llmcc-rust/src --lang rust --design-graph --pagerank --top-k 25

# Multiple files
llmcc --file crates/llmcc/src/main.rs --file crates/llmcc/src/lib.rs --lang rust --query run_main


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