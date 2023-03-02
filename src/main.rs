mod bitgraph;
mod esu;
mod io;
mod ngraph;
mod walker;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    /// File path to the input graph (white space separated edgelist)
    #[arg(short, long)]
    input: String,

    /// Output file path to write results to (default: stdout)
    #[arg(short, long)]
    output: Option<String>,

    /// Number of subgraphs to find in the input graph
    #[arg(short = 'k', long)]
    subgraph_size: usize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let graph = io::load_graph(&cli.input)?;
    let canon_counts = esu::enumerate_subgraphs(&graph, cli.subgraph_size);
    io::write_counts(canon_counts, cli.subgraph_size, cli.output)?;
    Ok(())
}
