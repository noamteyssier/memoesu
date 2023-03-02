use anyhow::Result;
use bitvec::{prelude::Msb0, view::BitView};
use graph6_rs::write_graph6;
use hashbrown::HashMap;
use petgraph::{Directed, Graph};
use std::{
    fs::File,
    io::{stdout, BufRead, BufReader, BufWriter, Write},
};

/// Load a graph from a file
pub fn load_graph(filepath: &str) -> Result<Graph<(), (), Directed>> {
    let reader = File::open(filepath).map(BufReader::new)?;
    let mut edges = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let u = split.next().unwrap().parse::<u32>().unwrap();
        let v = split.next().unwrap().parse::<u32>().unwrap();
        edges.push((u, v));
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
        let mut buffer = File::create(output).map(BufWriter::new)?;
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
