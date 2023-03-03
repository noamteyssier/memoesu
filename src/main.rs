mod bitgraph;
mod cli;
mod esu;
mod io;
mod ngraph;
mod multibitset;
mod parallel_esu;
mod walker;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use esu::enumerate_subgraphs;
use parallel_esu::parallel_enumerate_subgraphs;

fn submodule_enumerate(
        filepath: &str, 
        subgraph_size: usize, 
        output: Option<String>, 
        num_threads: Option<usize>) -> Result<()> {

    // Load the graph.
    let graph = io::load_graph(filepath)?;

    eprintln!("----------------------------------------");
    eprintln!("Log");
    eprintln!("----------------------------------------");
    eprintln!(">> Number of nodes         : {}", graph.node_count());
    eprintln!(">> Number of edges         : {}", graph.edge_count());

    // Enumerate the subgraphs.
    let now = std::time::Instant::now();
    let canon_counts = if let Some(num_threads) = num_threads {
        // Build a thread pool and use it to enumerate the subgraphs.
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()?;

        // Run the enumeration in parallel.
        parallel_enumerate_subgraphs(&graph, subgraph_size)
    } else {
        // Run the enumeration in serial.
        enumerate_subgraphs(&graph, subgraph_size)
    };

    eprintln!(">> Finished enumeration in : {:?}", now.elapsed());

    // Write the results to the output file.
    io::write_counts(&canon_counts, subgraph_size, output)?;

    Ok(())

}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.mode {
        cli::Mode::Enumerate {
            input,
            output,
            subgraph_size,
            threads,
        } => submodule_enumerate(&input, subgraph_size, output, threads),
    }
}
