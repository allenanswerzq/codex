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

    /// Print intermediate representation (IR)
    #[arg(long, default_value_t = false)]
    print_ir: bool,

    /// Print project graph
    #[arg(long, default_value_t = false)]
    print_graph: bool,

    /// Name of the symbol/function to query (enables find_depends mode)
    #[arg(long, value_name = "NAME")]
    query: Option<String>,

    /// Search recursively for transitive dependencies (default: direct dependencies only)
    #[arg(long, default_value_t = false)]
    recursive: bool,
}

pub fn run(cli: Cli) -> Result<()> {
    let Cli {
        files,
        dir,
        lang,
        print_ir,
        print_graph,
        query,
        recursive,
    } = cli;

    let opts = LlmccOptions {
        files,
        dir,
        print_ir,
        print_graph,
        query,
        recursive,
    };

    let output = match lang.as_str() {
        "rust" => run_main::<LangRust>(&opts).map_err(|err| anyhow!("{err}"))?,
        "python" => run_main::<LangPython>(&opts).map_err(|err| anyhow!("{err}"))?,
        _ => return Err(anyhow!("Unknown language: {lang}")),
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
