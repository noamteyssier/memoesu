mod cli;
mod enumerate;
mod io;
mod switching;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use enumerate::{enumerate_subgraphs, parallel_enumerate_subgraphs};
use io::FormatGraph;

/// Enumerate the subgraphs of a given size in a graph.
fn submodule_enumerate(
    filepath: &str,
    subgraph_size: usize,
    output: Option<String>,
    num_threads: Option<usize>,
    include_loops: bool,
) -> Result<()> {
    // Load the graph.
    let graph = io::load_numeric_graph(filepath, include_loops)?;

    eprintln!("----------------------------------------");
    eprintln!("Log");
    eprintln!("----------------------------------------");
    eprintln!(">> Number of nodes         : {}", graph.node_count());
    eprintln!(">> Number of edges         : {}", graph.edge_count());
    eprintln!(">> Including loops         : {include_loops}");

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

fn submodule_format(input: &str, prefix: &str, filter_loops: bool) -> Result<()> {
    let network_path = format!("{prefix}.network.tsv");
    let dict_path = format!("{prefix}.dictionary.tsv");

    // Load the graph.
    let format_graph = FormatGraph::from_filepath(input, filter_loops)?;

    eprintln!(">> Reading graph from {}", input);
    eprintln!(">> Found {} nodes", format_graph.node_count());
    eprintln!(">> Found {} edges", format_graph.edge_count());
    if filter_loops {
        eprintln!(">> Filtered out {} loops", format_graph.loops_removed());
    }
    eprintln!(">> Writing graph to {}", network_path);
    eprintln!(">> Writing node dictionary to {}", dict_path);

    // Write the graph and node dictionary to the output files.
    format_graph.write_graph(&network_path)?;
    format_graph.write_node_dict(&dict_path)?;

    Ok(())
}

fn submodule_switch(
    filepath: &str,
    output: Option<String>,
    q: usize,
    seed: Option<u8>,
) -> Result<()> {
    // Load the graph.
    let graph = io::load_numeric_graph(filepath, false)?;

    // Set the seed if not provided
    let seed = seed.unwrap_or_else(rand::random);

    eprintln!("----------------------------------------");
    eprintln!("Log");
    eprintln!("----------------------------------------");
    eprintln!(">> Number of nodes         : {}", graph.node_count());
    eprintln!(">> Number of edges         : {}", graph.edge_count());
    eprintln!(">> Using random seed       : {}", seed);

    // Switch the graph.
    let now = std::time::Instant::now();
    let switched_graph = switching::switching(&graph, q, seed);
    eprintln!(">> Finished switching in   : {:?}", now.elapsed());

    // Validate the switched graph.
    assert_eq!(graph.node_count(), switched_graph.node_count());
    assert_eq!(graph.edge_count(), switched_graph.edge_count());

    // Write the results to the output file.
    io::write_graph(&switched_graph, output)?;

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
            include_loops,
        } => submodule_enumerate(&input, subgraph_size, output, threads, include_loops),
        cli::Mode::Format {
            input,
            output,
            filter_loops,
        } => submodule_format(&input, &output, filter_loops),
        cli::Mode::Switch {
            input,
            output,
            q,
            seed,
        } => submodule_switch(&input, output, q, seed),
    }
}
