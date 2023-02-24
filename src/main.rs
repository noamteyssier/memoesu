mod canon;
mod esu;
mod rand_esu;
mod utils;

use std::{fmt::Debug, io::BufRead};

use esu::enumerate_subgraphs;
use rand_esu::random_enumerate_subgraphs;

use graph6_rs::write_graph6;
use hashbrown::{HashMap, HashSet};
use petgraph::{graph::NodeIndex, Directed, Graph};
use rayon::prelude::*;
use graph_canon::CanonLabeling;
use crate::canon::IntoSubgraph;

fn assemble_map<N, E>(
    graph: &Graph<N, E>,
    subgraph_indices: Vec<HashSet<NodeIndex>>,
) -> HashMap<CanonLabeling, usize>
where
    N: Debug + Clone + Send + Sync,
    E: Debug + Clone + Send + Sync,
{
    let subgraphs = subgraph_indices
        .into_par_iter()
        .map(|indices| graph.into_subgraph(&indices))
        .map(|g| CanonLabeling::new(&g))
        .collect::<Vec<_>>();

    let mut sg_map = HashMap::new();
    for subgraph in subgraphs {
        let count = sg_map.entry(subgraph).or_insert(0);
        *count += 1;
    }
    sg_map
}

fn run_enumerated<N, E>(graph: &Graph<N, E>, k: usize) -> HashMap<CanonLabeling, usize>
where
    N: Debug + Clone + Send + Sync,
    E: Debug + Clone + Send + Sync,
{
    eprintln!("Running complete enumerated search...");
    let subgraph_indices = enumerate_subgraphs(graph, k);
    eprintln!("Subgraphs found: {}", subgraph_indices.len());
    assemble_map(graph, subgraph_indices)
}

fn run_random_enumerated<N, E>(
    graph: &Graph<N, E>,
    k: usize,
    p: f64,
    seed: usize,
) -> HashMap<CanonLabeling, usize>
where
    N: Debug + Clone + Send + Sync,
    E: Debug + Clone + Send + Sync,
{
    eprintln!("Running partial enumerated search...");
    let subgraph_indices = random_enumerate_subgraphs(graph, k, p, seed);
    eprintln!("Subgraphs found: {}", subgraph_indices.len());
    assemble_map(graph, subgraph_indices)
}

fn load_graph(filepath: &str) -> Graph<(), (), Directed> {
    let file = std::fs::File::open(filepath).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut edges = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let u = split.next().unwrap().parse::<u32>().unwrap();
        let v = split.next().unwrap().parse::<u32>().unwrap();
        edges.push((u, v));
    }
    Graph::from_edges(&edges)
}

fn main() {
    let k = 4;
    let graph = load_graph("example/yeast.txt");

    let full_sg_map = run_enumerated(&graph, k);
    let partial_sg_map = run_random_enumerated(&graph, k, 0.7, 0);
    println!("{} {}", full_sg_map.len(), partial_sg_map.len());

    let full_sg_total = full_sg_map.values().sum::<usize>();
    let partial_sg_total = partial_sg_map.values().sum::<usize>();

    for subgraph in full_sg_map.keys() {
        let signature = write_graph6(subgraph.flat_adjacency(), k, true);
        let full_count = full_sg_map.get(subgraph).unwrap();
        let partial_count = partial_sg_map.get(subgraph).unwrap_or(&0);
        let full_frequency = *full_count as f64 / full_sg_total as f64;
        let partial_frequency = *partial_count as f64 / partial_sg_total as f64;

        println!(
            "{}\t{}\t{}\t{}\t{}",
            signature, full_count, partial_count, full_frequency, partial_frequency
        );
        // println!("{}", signature);
    }
}
