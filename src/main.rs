mod cli;
mod enrichment;
mod enumerate;
mod io;
mod switching;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use enrichment::enrichment;
use enumerate::{enumerate_subgraphs, parallel_enumerate_subgraphs};
use io::FormatGraph;
use petgraph::{Directed, EdgeType, Undirected};

use crate::enumerate::group_subgraphs;

/// Enumerate the subgraphs of a given size in a graph.
fn submodule_enumerate<Ty: EdgeType + Sync>(
    filepath: &str,
    subgraph_size: usize,
    output: Option<String>,
    num_threads: Option<usize>,
    include_loops: bool,
    is_directed: bool,
) -> Result<()> {
    // Load the graph.
    let graph = io::load_numeric_graph::<Ty>(filepath, include_loops)?;

    eprintln!("----------------------------------------");
    eprintln!("Log");
    eprintln!("----------------------------------------");
    eprintln!(">> Number of nodes         : {}", graph.node_count());
    eprintln!(">> Number of edges         : {}", graph.edge_count());
    eprintln!(">> Including loops         : {include_loops}");
    eprintln!(
        ">> Graph edge type         : {}",
        if is_directed {
            "directed"
        } else {
            "undirected"
        }
    );

    // Enumerate the subgraphs.
    let now = std::time::Instant::now();

    let results = match num_threads {
        Some(1) | None => enumerate_subgraphs(&graph, subgraph_size),
        Some(num_threads) => {
            // Build a thread pool and use it to enumerate the subgraphs.
            rayon::ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build_global()?;

            // Run the enumeration in parallel.
            parallel_enumerate_subgraphs(&graph, subgraph_size)
        }
    };

    eprintln!(">> Total subgraphs         : {}", results.total_subgraphs());
    eprintln!(
        ">> Unique subgraphs        : {}",
        results.unique_subgraphs()
    );
    // eprintln!(">> Duplicate calculations  : {}", results.num_duplicates());
    eprintln!(">> Finished enumeration in : {:?}", now.elapsed());
    eprintln!("----------------------------------------");

    // Write the results to the output file.
    io::write_counts(results.counts(), subgraph_size, output, is_directed)?;

    Ok(())
}

/// Enumerate the subgraphs of a given size in a graph.
fn submodule_groups<Ty: EdgeType + Sync>(
    filepath: &str,
    subgraph_size: usize,
    output: Option<String>,
    include_loops: bool,
    is_directed: bool,
    no_header: bool,
) -> Result<()> {
    // Load the graph.
    let graph = io::load_numeric_graph::<Ty>(filepath, include_loops)?;

    eprintln!("----------------------------------------");
    eprintln!("Log");
    eprintln!("----------------------------------------");
    eprintln!(">> Number of nodes         : {}", graph.node_count());
    eprintln!(">> Number of edges         : {}", graph.edge_count());
    eprintln!(">> Including loops         : {include_loops}");
    eprintln!(
        ">> Graph edge type         : {}",
        if is_directed {
            "directed"
        } else {
            "undirected"
        }
    );

    // Enumerate the subgraphs.
    let now = std::time::Instant::now();
    let results = group_subgraphs(&graph, subgraph_size);

    eprintln!(">> Total subgraphs         : {}", results.total_subgraphs());
    eprintln!(
        ">> Unique subgraphs        : {}",
        results.unique_subgraphs()
    );
    eprintln!(">> Finished enumeration in : {:?}", now.elapsed());
    eprintln!("----------------------------------------");

    // Write the results to the output file.
    io::write_groups(results.groups(), subgraph_size, output, is_directed, no_header)?;

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
    seed: Option<usize>,
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

fn submodule_enrichment(
    filepath: &str,
    subgraph_size: usize,
    output: Option<String>,
    _num_threads: Option<usize>,
    random_graphs: usize,
    q: usize,
    seed: Option<usize>,
) -> Result<()> {
    let graph = io::load_numeric_graph(filepath, false)?;
    let results = enrichment(&graph, subgraph_size, random_graphs, q, seed);
    io::write_stats(&results, subgraph_size, output)?;
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
            undirected,
        } => {
            if undirected {
                submodule_enumerate::<Undirected>(
                    &input,
                    subgraph_size,
                    output,
                    threads,
                    include_loops,
                    false,
                )
            } else {
                submodule_enumerate::<Directed>(
                    &input,
                    subgraph_size,
                    output,
                    threads,
                    include_loops,
                    true,
                )
            }
        }
        cli::Mode::Groups {
            input,
            output,
            subgraph_size,
            include_loops,
            undirected,
            no_header,
        } => {
            if undirected {
                submodule_groups::<Undirected>(&input, subgraph_size, output, include_loops, false, no_header)
            } else {
                submodule_groups::<Directed>(&input, subgraph_size, output, include_loops, true, no_header)
            }
        }
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
        cli::Mode::Enrich {
            input,
            output,
            subgraph_size,
            threads,
            random_graphs,
            q,
            seed,
        } => submodule_enrichment(
            &input,
            subgraph_size,
            output,
            threads,
            random_graphs,
            q,
            seed,
        ),
    }
}
