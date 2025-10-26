use anyhow::Result;
use anyhow::anyhow;
use clap::Parser;
use llmcc::LlmccOptions;
use llmcc::run_main;
use llmcc_python::LangPython;
use llmcc_rust::LangRust;
use std::process;

#[derive(Parser, Debug)]
#[command(name = "llmcc")]
#[command(about = "llmcc: llm context compiler")]
#[command(version)]
pub struct Cli {
    /// Files to compile
    #[arg(value_name = "FILE", required_unless_present = "dir")]
    files: Vec<String>,

    /// Load all .rs files from a directory (recursive)
    #[arg(short, long, value_name = "DIR")]
    dir: Option<String>,

    /// Language to use: 'rust' or 'python'
    #[arg(long, value_name = "LANG", default_value = "rust")]
    lang: String,

    /// Print intermediate representation (IR), internal debugging output
    #[arg(long, default_value_t = false)]
    print_ir: bool,

    /// Print basic block graph
    #[arg(long, default_value_t = false)]
    print_block: bool,

    /// Print a project level graph focused on class relationships, good for understanding high-level design architecture
    #[arg(long, default_value_t = false)]
    project_graph: bool,

    /// Use page rank algorithm to filter the most important nodes in the project graph
    #[arg(long, default_value_t = false)]
    pagerank: bool,

    /// Top k nodes to select using PageRank algorithm
    #[arg(long, value_name = "K", requires = "pagerank")]
    top_k: Option<usize>,

    /// PageRank direction: 'depends-on' to rank depended-upon nodes, 'depended-by' to rank orchestrators (default: depended-by)
    #[arg(
        long,
        value_name = "DIR",
        requires = "pagerank",
        default_value = "depended-by"
    )]
    pagerank_direction: String,

    /// Name of the symbol/function to query (enables find_depends mode)
    #[arg(long, value_name = "NAME")]
    query: Option<String>,

    /// Search recursively for transitive dependencies (default: direct dependencies only)
    #[arg(long, default_value_t = false)]
    recursive: bool,

    /// Return blocks that depend on the queried symbol instead of the ones it depends on
    #[arg(long, default_value_t = false, conflicts_with = "recursive")]
    dependents: bool,
}

pub fn run(args: Cli) -> Result<()> {
    let opts = LlmccOptions {
        files: args.files,
        dir: args.dir,
        print_ir: args.print_ir,
        print_block: args.print_block,
        project_graph: args.project_graph,
        pagerank: args.pagerank,
        top_k: args.top_k,
        pagerank_direction: args.pagerank_direction,
        query: args.query,
        recursive: args.recursive,
        dependents: args.dependents,
    };

    let output = match args.lang.as_str() {
        "rust" => run_main::<LangRust>(&opts).map_err(|err| anyhow!("{err}"))?,
        "python" => run_main::<LangPython>(&opts).map_err(|err| anyhow!("{err}"))?,
        _ => return Err(anyhow!("Unknown language: {0}", args.lang)),
    };

    if let Some(output) = output {
        println!("{output}");
    }

    Ok(())
}

pub fn main() {
    let cli = Cli::parse();
    if let Err(err) = run(cli) {
        eprintln!("{err}");
        process::exit(1);
    }
}
