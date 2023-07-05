use anyhow::{bail, Result};
use bitvec::{prelude::Msb0, view::BitView};
use graph6_rs::write_graph6;
use hashbrown::{HashMap, HashSet};
use petgraph::{Directed, EdgeType, Graph};
use std::{
    fs::File,
    io::{stdout, BufRead, BufReader, BufWriter, Write},
};

use crate::{
    enrichment::EnrichResult,
    enumerate::{Counts, Groups, Label},
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
pub fn load_numeric_graph<Ty: EdgeType>(
    filepath: &str,
    include_loops: bool,
) -> Result<Graph<(), (), Ty>> {
    let mut reader = File::open(filepath).map(BufReader::new)?;
    load_numeric_graph_from_buffer(&mut reader, include_loops)
}

/// Load a graph from a buffer
///
/// Expects a 1-Indexed numeric white-space delimited edgelist.
pub fn load_numeric_graph_from_buffer<B: BufRead, Ty: EdgeType>(
    buffer: &mut B,
    include_loops: bool,
) -> Result<Graph<(), (), Ty>> {
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
    canon_counts: &Counts,
    k: usize,
    output: Option<String>,
    is_directed: bool,
) -> Result<()> {
    if let Some(output) = output {
        let mut buffer = File::create(&output).map(BufWriter::new)?;
        eprintln!(">> Writing results to      : {}", &output);
        write_counts_to_buffer(&mut buffer, canon_counts, k, is_directed)
    } else {
        let mut buffer = BufWriter::new(stdout().lock());
        write_counts_to_buffer(&mut buffer, canon_counts, k, is_directed)
    }
}

/// Write the counts of each subgraph to a buffer
fn write_counts_to_buffer<W: Write>(
    buffer: &mut BufWriter<W>,
    canon_counts: &Counts,
    k: usize,
    is_directed: bool,
) -> Result<()> {
    // Sort by count
    let mut sorted_counts: Vec<(&Label, &usize)> = canon_counts.iter().collect();
    sorted_counts.sort_by(|a, b| a.1.cmp(b.1));

    // Write to buffer
    for (label, count) in sorted_counts {
        let adj = graph_to_flat_adj(label, k);
        let canon = write_graph6(adj, k, is_directed);
        writeln!(buffer, "{canon}\t{count}")?;
    }
    Ok(())
}

/// Write the groups of each node to a file or stdout
pub fn write_groups(
    groups: &Groups,
    k: usize,
    output: Option<String>,
    is_directed: bool,
    no_header: bool,
) -> Result<()> {
    if let Some(output) = output {
        let mut buffer = File::create(&output).map(BufWriter::new)?;
        eprintln!(">> Writing results to      : {}", &output);
        write_groups_to_buffer(&mut buffer, groups, k, is_directed, no_header)
    } else {
        let mut buffer = BufWriter::new(stdout().lock());
        write_groups_to_buffer(&mut buffer, groups, k, is_directed, no_header)
    }
}

/// Write the counts of each subgraph to a buffer
fn write_groups_to_buffer<W: Write>(
    buffer: &mut BufWriter<W>,
    groups: &Groups,
    k: usize,
    is_directed: bool,
    no_header: bool,
) -> Result<()> {
    if !no_header {
        writeln!(buffer, "node_idx\tcanon\tlabel\torbit\tabundance")?;
    }
    for (node_idx, group_info) in groups.iter() {
        for ((label, node_label, orbit), abundance) in group_info.iter() {
            let adj = graph_to_flat_adj(label, k);
            let canon = write_graph6(adj, k, is_directed);
            let adj_node_idx = node_idx + 1;
            writeln!(
                buffer,
                "{adj_node_idx}\t{canon}\t{node_label}\t{orbit}\t{abundance}"
            )?;
        }
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
    use petgraph::Undirected;

    use crate::enumerate::group_subgraphs;

    use super::*;
    use std::io::Cursor;

    #[test]
    fn read_graph_directed() {
        let filepath = "example/example.txt";
        let graph = load_numeric_graph::<Directed>(filepath, false).unwrap();
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
    fn read_graph_undirected() {
        let filepath = "example/example.txt";
        let graph = load_numeric_graph::<Undirected>(filepath, false).unwrap();
        assert_eq!(graph.node_count(), 9);
        assert_eq!(graph.edge_count(), 9);
        assert!(graph.contains_edge(0.into(), 1.into()));
        assert!(graph.contains_edge(1.into(), 0.into()));
        assert!(graph.contains_edge(1.into(), 2.into()));
        assert!(graph.contains_edge(2.into(), 1.into()));
        assert!(graph.contains_edge(2.into(), 0.into()));
        assert!(graph.contains_edge(0.into(), 2.into()));
        assert!(graph.contains_edge(3.into(), 0.into()));
        assert!(graph.contains_edge(0.into(), 3.into()));
        assert!(graph.contains_edge(0.into(), 4.into()));
        assert!(graph.contains_edge(4.into(), 0.into()));
        assert!(graph.contains_edge(5.into(), 1.into()));
        assert!(graph.contains_edge(1.into(), 5.into()));
        assert!(graph.contains_edge(1.into(), 6.into()));
        assert!(graph.contains_edge(6.into(), 1.into()));
        assert!(graph.contains_edge(7.into(), 2.into()));
        assert!(graph.contains_edge(2.into(), 7.into()));
        assert!(graph.contains_edge(2.into(), 8.into()));
        assert!(graph.contains_edge(8.into(), 2.into()));
    }

    #[test]
    fn read_zero_index() {
        let internal = "0\t1\n1\t2\n2\t0\n";
        let mut buffer = Cursor::new(internal);
        let graph = load_numeric_graph_from_buffer::<Cursor<&str>, Directed>(&mut buffer, false);
        assert!(graph.is_err());
    }

    #[test]
    fn read_one_index() {
        let internal = "1\t2\n2\t3\n3\t1\n";
        let mut buffer = Cursor::new(internal);
        let graph = load_numeric_graph_from_buffer::<Cursor<&str>, Directed>(&mut buffer, false);
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
        let graph =
            load_numeric_graph_from_buffer::<Cursor<&str>, Directed>(&mut buffer, true).unwrap();
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
        let graph =
            load_numeric_graph_from_buffer::<Cursor<&str>, Directed>(&mut buffer, false).unwrap();
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 3);
        assert!(!graph.contains_edge(0.into(), 0.into()));
    }

    #[test]
    fn test_groups_io() {
        // 2 -> 1
        // 3 -> 1
        // 4 -> 1
        let internal = "2\t1\n3\t1\n4\t1";
        let mut buffer = Cursor::new(internal);
        let graph = load_numeric_graph_from_buffer::<Cursor<&str>, Directed>(&mut buffer, false).unwrap();
        let results = group_subgraphs(&graph, 3);
        let output = Cursor::new(Vec::new());
        let mut output_buffer = BufWriter::new(output);
        write_groups_to_buffer(&mut output_buffer, results.groups(), 3, true, true).unwrap();
        let string_buffer = std::str::from_utf8(output_buffer.buffer()).unwrap();
        println!("{}", string_buffer);
        assert!(string_buffer.contains("4\t&BC_\t1\t1\t2"));
        assert!(string_buffer.contains("1\t&BC_\t0\t0\t3"));
        assert!(string_buffer.contains("3\t&BC_\t2\t1\t1"));
        assert!(string_buffer.contains("3\t&BC_\t1\t1\t1"));
        assert!(string_buffer.contains("2\t&BC_\t2\t1\t2"));
        assert!(string_buffer.chars().filter(|c| c == &'\n').count() == 5);
    }

    #[test]
    fn test_groups_io_with_header() {
        // 2 -> 1
        // 3 -> 1
        // 4 -> 1
        let internal = "2\t1\n3\t1\n4\t1";
        let mut buffer = Cursor::new(internal);
        let graph = load_numeric_graph_from_buffer::<Cursor<&str>, Directed>(&mut buffer, false).unwrap();
        let results = group_subgraphs(&graph, 3);
        let output = Cursor::new(Vec::new());
        let mut output_buffer = BufWriter::new(output);
        write_groups_to_buffer(&mut output_buffer, results.groups(), 3, true, false).unwrap();
        let string_buffer = std::str::from_utf8(output_buffer.buffer()).unwrap();
        println!("{}", string_buffer);
        assert!(string_buffer.contains("node_idx\tcanon\tlabel\torbit\tabundance\n"));
        assert!(string_buffer.contains("4\t&BC_\t1\t1\t2"));
        assert!(string_buffer.contains("1\t&BC_\t0\t0\t3"));
        assert!(string_buffer.contains("3\t&BC_\t2\t1\t1"));
        assert!(string_buffer.contains("3\t&BC_\t1\t1\t1"));
        assert!(string_buffer.contains("2\t&BC_\t2\t1\t2"));
        assert!(string_buffer.chars().filter(|c| c == &'\n').count() == 6);
    }
}
