mod bitgraph;
mod esu;
mod io;
mod ngraph;
mod parallel_esu;
mod walker;

use anyhow::Result;
use clap::Parser;
use esu::enumerate_subgraphs;
use parallel_esu::parallel_enumerate_subgraphs;

#[derive(Parser, Debug)]
pub struct Cli {
    /// File path to the input graph (white space separated edgelist)
    #[arg(short, long)]
    input: String,

    /// Output file path to write results to (default: stdout)
    #[arg(short, long)]
    output: Option<String>,

    /// Number of subgraphs to find in the input graph
    #[arg(short, long)]
    subgraph_size: usize,

    /// Number of threads to use (default: 1)
    #[arg(short, long)]
    threads: Option<usize>,
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
    let canon_counts = if let Some(num_threads) = cli.threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()?;
        parallel_enumerate_subgraphs(&graph, cli.subgraph_size)
    } else {
        enumerate_subgraphs(&graph, cli.subgraph_size) 
    };
    eprintln!(">> Finished enumeration in : {:?}", now.elapsed());

    // Write the results to the output file.
    io::write_counts(&canon_counts, cli.subgraph_size, cli.output)?;
    Ok(())
}
