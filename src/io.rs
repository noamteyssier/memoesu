use anyhow::{bail, Result};
use bitvec::{prelude::Msb0, view::BitView};
use graph6_rs::write_graph6;
use hashbrown::{HashMap, HashSet};
use petgraph::{Directed, Graph};
use std::{
    fs::File,
    io::{stdout, BufRead, BufReader, BufWriter, Write},
};

use crate::enrichment::EnrichResult;

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
        let mut reader = File::open(filepath).map(BufReader::new)?;
        Ok(Self::from_buffer(&mut reader, filter_loops))
    }

    /// Reads a graph from a buffer.
    pub fn from_buffer<B: BufRead>(buffer: &mut B, filter_loops: bool) -> Self {
        let mut map = HashMap::new();
        let mut edges = HashSet::new();
        let mut num_filtered = 0;

        for line in buffer.lines() {
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
        Self::new(graph, map, num_filtered)
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
    let mut reader = File::open(filepath).map(BufReader::new)?;
    load_numeric_graph_from_buffer(&mut reader, include_loops)
}

/// Load a graph from a buffer
///
/// Expects a 1-Indexed numeric white-space delimited edgelist.
pub fn load_numeric_graph_from_buffer<B: BufRead>(buffer: &mut B, include_loops: bool) -> Result<Graph<(), (), Directed>> {
    let mut edges = Vec::new();
    for line in buffer.lines() {
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
    canon_counts: &ahash::HashMap<Vec<u64>, usize>,
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
    canon_counts: &ahash::HashMap<Vec<u64>, usize>,
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

pub fn write_stats(results: &EnrichResult, k: usize, output: Option<String>) -> Result<()> {
    if let Some(output) = output {
        let mut buffer = File::create(&output).map(BufWriter::new)?;
        eprintln!(">> Writing results to      : {}", &output);
        write_stats_to_buffer(&mut buffer, results, k)
    } else {
        let mut buffer = BufWriter::new(stdout().lock());
        write_stats_to_buffer(&mut buffer, results, k)
    }
}

fn write_stats_to_buffer<W: Write>(
    buffer: &mut BufWriter<W>,
    results: &EnrichResult,
    k: usize,
) -> Result<()> {
    writeln!(buffer, "canon\tabundance\tmean\tstd\tzscore")?;
    for idx in 0..results.len() {
        let subgraph = &results.subgraphs[idx];
        let adj = graph_to_flat_adj(subgraph, k);
        let canon = write_graph6(adj, k, true);
        let abundance = &results.abundances[idx];
        let mean = &results.mean_random_frequency[idx];
        let std = &results.std_random_frequency[idx];
        let zscore = &results.zscores[idx];
        writeln!(buffer, "{canon}\t{abundance}\t{mean}\t{std}\t{zscore}")?;
    }
    Ok(())
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
        writeln!(buffer, "{}\t{}", u.index() + 1, v.index() + 1)?;
    }
    Ok(())
}

#[cfg(test)]
mod testing {
    use std::io::Cursor;


    #[test]
    fn read_graph() {
        let filepath = "example/example.txt";
        let graph = super::load_numeric_graph(filepath, false).unwrap();
        assert_eq!(graph.node_count(), 9);
        assert_eq!(graph.edge_count(), 9);
        assert!(graph.contains_edge(0.into(), 1.into()));
        assert!(graph.contains_edge(1.into(), 2.into()));
        assert!(graph.contains_edge(2.into(), 0.into()));
        assert!(graph.contains_edge(3.into(), 0.into()));
        assert!(graph.contains_edge(0.into(), 4.into()));
        assert!(graph.contains_edge(5.into(), 1.into()));
        assert!(graph.contains_edge(1.into(), 6.into()));
        assert!(graph.contains_edge(7.into(), 2.into()));
        assert!(graph.contains_edge(2.into(), 8.into()));
    }

    #[test]
    fn read_zero_index() {
        let internal = "0\t1\n1\t2\n2\t0\n";
        let mut buffer = Cursor::new(internal);
        let graph = super::load_numeric_graph_from_buffer(&mut buffer, false);
        assert!(graph.is_err());
    }

    #[test]
    fn read_one_index() {
        let internal = "1\t2\n2\t3\n3\t1\n";
        let mut buffer = Cursor::new(internal);
        let graph = super::load_numeric_graph_from_buffer(&mut buffer, false);
        assert!(graph.is_ok());
    }

    #[test]
    fn read_with_loops() {
        // 1 -> 2
        // 2 -> 3
        // 3 -> 1
        // 1 -> 1
        let internal = "1\t2\n2\t3\n3\t1\n1\t1";
        let mut buffer = Cursor::new(internal);
        let graph = super::load_numeric_graph_from_buffer(&mut buffer, true).unwrap();
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 4);
        assert!(graph.contains_edge(0.into(), 0.into()));
    }

    #[test]
    fn read_without_loops() {
        // 1 -> 2
        // 2 -> 3
        // 3 -> 1
        // 1 -> 1
        let internal = "1\t2\n2\t3\n3\t1\n1\t1";
        let mut buffer = Cursor::new(internal);
        let graph = super::load_numeric_graph_from_buffer(&mut buffer, false).unwrap();
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 3);
        assert!(!graph.contains_edge(0.into(), 0.into()));
    }

}
