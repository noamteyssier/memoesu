mod bitgraph;
mod cli;
mod esu;
mod io;
mod multibitset;
mod ngraph;
mod parallel_esu;
mod walker;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use esu::enumerate_subgraphs;
use io::FormatGraph;
use parallel_esu::parallel_enumerate_subgraphs;

/// Enumerate the subgraphs of a given size in a graph.
fn submodule_enumerate(
    filepath: &str,
    subgraph_size: usize,
    output: Option<String>,
    num_threads: Option<usize>,
) -> Result<()> {
    // Load the graph.
    let graph = io::load_numeric_graph(filepath)?;

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

fn submodule_format(input: &str, prefix: &str) -> Result<()> {
    let network_path = format!("{prefix}.network.tsv");
    let dict_path = format!("{prefix}.dictionary.tsv");

    // Load the graph.
    let format_graph = FormatGraph::from_filepath(input)?;

    eprintln!(">> Reading graph from {}", input);
    eprintln!(">> Found {} nodes", format_graph.node_count());
    eprintln!(">> Found {} edges", format_graph.edge_count());
    eprintln!(">> Writing graph to {}", network_path);
    eprintln!(">> Writing node dictionary to {}", dict_path);

    // Write the graph and node dictionary to the output files.
    format_graph.write_graph(&network_path)?;
    format_graph.write_node_dict(&dict_path)?;

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
        cli::Mode::Format { 
            input, 
            output, 
        } => submodule_format(&input, &output),
    }
}
