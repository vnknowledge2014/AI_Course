use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod condb;
mod dream;
mod pageindex;

/// Rusty-Context: A Self-Evolving Agentic Context Management Ecosystem
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Index a document or directory (Incremental & AST-Aware)
    Index {
        #[arg(short, long)]
        path: PathBuf,
    },
    /// Search the memory (STM + LTM)
    Search {
        #[arg(short, long)]
        query: String,
    },
    /// Run the Dreaming process to consolidate STM into LTM
    Dream,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Index { path } => {
            println!("Indexing path: {:?}", path);
            // TODO: Call pageindex::index_document()
        }
        Commands::Search { query } => {
            println!("Searching for: {}", query);
            // TODO: Call condb::search()
        }
        Commands::Dream => {
            println!("Initiating Dream sequence (Consolidating STM -> LTM)...");
            // TODO: Call dream::run_dream_cycle()
        }
    }

    Ok(())
}
