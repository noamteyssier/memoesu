mod bitgraph;
mod bit_esu;
mod esu;
mod ngraph;
mod rand_esu;
mod subgraph;
mod utils;
mod walker;

use std::io::BufRead;

use esu::enumerate_subgraphs;
use rand_esu::random_enumerate_subgraphs;
use subgraph::build_subgraph;

use graph6_rs::write_graph6;
use graph_canon::CanonLabeling;
use hashbrown::{HashMap, HashSet};
use petgraph::{graph::NodeIndex, visit::GetAdjacencyMatrix, Directed, EdgeType, Graph};
use rayon::prelude::*;

fn assemble_map<N, E, Ty>(
    graph: &Graph<N, E, Ty>,
    subgraph_indices: Vec<HashSet<NodeIndex>>,
) -> HashMap<CanonLabeling, usize>
where
    N: Send + Sync,
    E: Send + Sync,
    Ty: EdgeType + Send + Sync,
{
    let adj = graph.adjacency_matrix();
    let subgraphs = subgraph_indices
        .into_par_iter()
        // .into_iter()
        // .map(|indices| build_subgraph::<N, E, Ty>(&adj, graph.node_count(), &indices))
        // .map(|g| CanonLabeling::new(&g))
        .collect::<Vec<_>>();

    let mut sg_map = HashMap::new();
    for subgraph in subgraphs {
        // let count = sg_map.entry(subgraph).or_insert(0);
        // *count += 1;
    }
    sg_map
}

fn run_enumerated<N, E>(graph: &Graph<N, E>, k: usize) -> HashMap<CanonLabeling, usize>
where
    N: Send + Sync,
    E: Send + Sync,
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
    N: Send + Sync,
    E: Send + Sync,
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
    let k = 5;
    let graph = load_graph("example/yeast.txt");
    // let graph = load_graph("example/example.txt");
    bit_esu::enumerate_subgraphs(&graph, k);

    // let full_sg_map = run_enumerated(&graph, k);
    // println!("{}", full_sg_map.len());
    // let partial_sg_map = run_random_enumerated(&graph, k, 0.7, 0);
    // println!("{} {}", full_sg_map.len(), partial_sg_map.len());

    // let full_sg_total = full_sg_map.values().sum::<usize>();
    // let partial_sg_total = partial_sg_map.values().sum::<usize>();

    // for subgraph in full_sg_map.keys() {
    //     let signature = write_graph6(subgraph.flat_adjacency(), k, true);
    //     let full_count = full_sg_map.get(subgraph).unwrap();
    //     let partial_count = partial_sg_map.get(subgraph).unwrap_or(&0);
    //     let full_frequency = *full_count as f64 / full_sg_total as f64;
    //     let partial_frequency = *partial_count as f64 / partial_sg_total as f64;

    //     println!(
    //         "{}\t{}\t{}\t{}\t{}",
    //         signature, full_count, partial_count, full_frequency, partial_frequency
    //     );
    //     // println!("{}", signature);
    // }
}
