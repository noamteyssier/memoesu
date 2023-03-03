use anyhow::{bail, Result};
use bitvec::{prelude::Msb0, view::BitView};
use graph6_rs::write_graph6;
use hashbrown::{HashMap, HashSet};
use petgraph::{Directed, Graph};
use std::{
    fs::File,
    io::{stdout, BufRead, BufReader, BufWriter, Write},
};

pub struct FormatGraph {
    graph: Graph<(), (), Directed>,
    node_dict: HashMap<String, u32>,
    num_filtered: usize,
}
impl FormatGraph {
    pub fn new(
        graph: Graph<(), (), Directed>,
        node_dict: HashMap<String, u32>,
        num_filtered: usize,
    ) -> Self {
        Self {
            graph,
            node_dict,
            num_filtered,
        }
    }

    /// Reads a graph from a file path.
    pub fn from_filepath(filepath: &str, filter_loops: bool) -> Result<Self> {
        let reader = File::open(filepath).map(BufReader::new)?;
        let mut map = HashMap::new();
        let mut edges = HashSet::new();
        let mut num_filtered = 0;

        for line in reader.lines() {
            let line = line.unwrap();
            let mut split = line.split_whitespace();

            let u = split.next().unwrap();
            let v = split.next().unwrap();

            if filter_loops && u == v {
                num_filtered += 1;
                continue;
            }

            // Add the nodes to the node dictionary if they are not already present.
            if !map.contains_key(u) {
                map.insert(u.to_string(), map.len() as u32);
            }
            if !map.contains_key(v) {
                map.insert(v.to_string(), map.len() as u32);
            }

            edges.insert((map[u], map[v]));
        }

        let graph = Graph::from_edges(&edges);
        Ok(Self::new(graph, map, num_filtered))
    }

    pub fn write_graph(&self, output: &str) -> Result<()> {
        let mut buffer = File::create(output).map(BufWriter::new)?;
        for edge_idx in self.graph.edge_indices() {
            let (u, v) = self.graph.edge_endpoints(edge_idx).unwrap();
            writeln!(buffer, "{}\t{}", u.index() + 1, v.index() + 1)?;
        }
        Ok(())
    }

    pub fn write_node_dict(&self, output: &str) -> Result<()> {
        let mut buffer = File::create(output).map(BufWriter::new)?;
        for (node, idx) in self.node_dict.iter() {
            writeln!(buffer, "{}\t{}", node, idx + 1)?;
        }
        Ok(())
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn loops_removed(&self) -> usize {
        self.num_filtered
    }
}

/// Load a graph from a file
///
/// Expects a 1-Indexed numeric white-space delimited edgelist.
pub fn load_numeric_graph(filepath: &str, include_loops: bool) -> Result<Graph<(), (), Directed>> {
    let reader = File::open(filepath).map(BufReader::new)?;
    let mut edges = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let u = split.next().unwrap().parse::<u32>()?;
        let v = split.next().unwrap().parse::<u32>()?;
        if u == 0 || v == 0 {
            bail!("ERROR: Found a node index: 0; Please use 1-indexed node indices.");
        }
        if !include_loops && u == v {
            continue;
        } else {
            edges.push((u - 1, v - 1));
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

/// Write a graph to a file
pub fn write_graph(graph: &Graph<(), (), Directed>, output: Option<String>) -> Result<()> {
    if let Some(filepath) = output {
        let mut buffer = File::create(filepath).map(BufWriter::new)?;
        write_graph_to_buffer(&mut buffer, graph)
    } else {
        let mut buffer = BufWriter::new(stdout().lock());
        write_graph_to_buffer(&mut buffer, graph)
    }
}

pub fn write_graph_to_buffer<W: Write>(
    buffer: &mut BufWriter<W>,
    graph: &Graph<(), (), Directed>,
) -> Result<()> {
    for edge_idx in graph.edge_indices() {
        let (u, v) = graph.edge_endpoints(edge_idx).unwrap();
        writeln!(buffer, "{}\t{}", u.index(), v.index())?;
    }
    Ok(())
}
