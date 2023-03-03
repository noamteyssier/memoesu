use anyhow::Result;
use bitvec::{prelude::Msb0, view::BitView};
use graph6_rs::write_graph6;
use hashbrown::HashMap;
use petgraph::{Directed, Graph};
use std::{
    fs::File,
    io::{stdout, BufRead, BufReader, BufWriter, Write},
};

pub struct FormatGraph {
    graph: Graph<(), (), Directed>,
    node_dict: HashMap<String, u32>,
}
impl FormatGraph {
    pub fn new(graph: Graph<(), (), Directed>, node_dict: HashMap<String, u32>) -> Self {
        Self { graph, node_dict }
    }
    
    /// Reads a graph from a file path.
    pub fn from_filepath(filepath: &str) -> Result<Self> {
        let reader = File::open(filepath).map(BufReader::new)?;
        let mut map = HashMap::new();
        let mut edges: Vec<(u32, u32)> = Vec::new();

        for line in reader.lines() {

            let line = line.unwrap();
            let mut split = line.split_whitespace();

            let u = split.next().unwrap();
            let v = split.next().unwrap();

            // Add the nodes to the node dictionary if they are not already present.
            if !map.contains_key(u) {
                map.insert(u.to_string(), map.len() as u32);
            }
            if !map.contains_key(v) {
                map.insert(v.to_string(), map.len() as u32);
            }

            edges.push((map[u], map[v]));
        }

        let graph = Graph::from_edges(&edges);
        Ok(Self::new(graph, map))
    }

    pub fn write_graph(&self, output: &str) -> Result<()> {
        let mut buffer = File::create(output).map(BufWriter::new)?;
        for edge_idx in self.graph.edge_indices() {
            let (u, v) = self.graph.edge_endpoints(edge_idx).unwrap();
            writeln!(buffer, "{} {}", u.index() + 1, v.index() + 1)?;
        }
        Ok(())
    }

    pub fn write_node_dict(&self, output: &str) -> Result<()> {
        let mut buffer = File::create(output).map(BufWriter::new)?;
        for (node, idx) in self.node_dict.iter() {
            writeln!(buffer, "{} {}", node, idx + 1)?;
        }
        Ok(())
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
}

/// Load a graph from a file
///
/// Expects a 1-Indexed numeric white-space delimited edgelist.
pub fn load_numeric_graph(filepath: &str) -> Result<Graph<(), (), Directed>> {
    let reader = File::open(filepath).map(BufReader::new)?;
    let mut edges = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let u = split.next().unwrap().parse::<u32>()?;
        let v = split.next().unwrap().parse::<u32>()?;
        if u == v {
            eprintln!("Skipping self loop: {}", u);
        } else {
            edges.push((u, v));
        }
    }
    Ok(Graph::from_edges(&edges))
}

/// Write the counts of each subgraph to a file or stdout
pub fn write_counts(
    canon_counts: &HashMap<Vec<u64>, usize>,
    k: usize,
    output: Option<String>,
) -> Result<()> {
    if let Some(output) = output {
        let mut buffer = File::create(&output).map(BufWriter::new)?;
        eprintln!(">> Writing results to      : {}", &output);
        write_counts_to_buffer(&mut buffer, canon_counts, k)
    } else {
        let mut buffer = BufWriter::new(stdout().lock());
        write_counts_to_buffer(&mut buffer, canon_counts, k)
    }
}

/// Write the counts of each subgraph to a buffer
fn write_counts_to_buffer<W: Write>(
    buffer: &mut BufWriter<W>,
    canon_counts: &HashMap<Vec<u64>, usize>,
    k: usize,
) -> Result<()> {
    // Sort by count
    let mut sorted_counts: Vec<(&Vec<u64>, &usize)> = canon_counts.iter().collect();
    sorted_counts.sort_by(|a, b| a.1.cmp(b.1));

    // Write to buffer
    for (label, count) in sorted_counts {
        let adj = graph_to_flat_adj(label, k);
        let canon = write_graph6(adj, k, true);
        writeln!(buffer, "{canon}\t{count}")?;
    }
    Ok(())
}

/// Convert a nauty graph to a flat adjacency matrix
fn graph_to_flat_adj(graph: &[u64], n: usize) -> Vec<usize> {
    let mut adj = Vec::with_capacity(n * n);
    for num in graph.iter() {
        let bv = num.view_bits::<Msb0>();
        for b in bv.iter().take(graph.len()) {
            if *b {
                adj.push(1);
            } else {
                adj.push(0);
            }
        }
    }
    adj
}
