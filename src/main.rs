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

    // Load the graph.
    let graph = io::load_graph(&cli.input)?;

    eprintln!("----------------------------------------");
    eprintln!("Log");
    eprintln!("----------------------------------------");
    eprintln!(">> Number of nodes         : {}", graph.node_count());
    eprintln!(">> Number of edges         : {}", graph.edge_count());

    // Enumerate the subgraphs.
    let now = std::time::Instant::now();
    let canon_counts = esu::enumerate_subgraphs(&graph, cli.subgraph_size);
    eprintln!(">> Finished enumeration in : {:?}", now.elapsed());

    // Write the results to the output file.
    io::write_counts(&canon_counts, cli.subgraph_size, cli.output)?;
    Ok(())
}
