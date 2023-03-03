use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand, Debug)]
pub enum Mode {
    /// Enumerate all subgraphs of a given size in a graph.
    Enumerate {
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
    },

    /// Formats an input graph into a usable format for `memoesu`
    Format {
        /// File path to the input graph (white space separated edgelist)
        #[arg(short, long)]
        input: String,

        /// Output file prefix to write graph and node dictionary with
        #[arg(short, long)]
        output: String,

        /// Filter out loops (i.e. a node connects to itself) [default: false]
        #[arg(short, long)]
        filter_loops: bool,
    }
}
