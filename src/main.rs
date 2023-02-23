mod canon;
mod esu;
mod rand_esu;
mod utils;

use std::fmt::Debug;

use canon::canonical_form;
use esu::enumerated_search;
use nauty_pet::prelude::CanonGraph;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use rand_esu::random_enumerated_search;

use hashbrown::{HashMap, HashSet};
use petgraph::{Graph, Directed, visit::GetAdjacencyMatrix, graph::NodeIndex};
use petgraph_gen::random_gnp_graph;
use rayon::prelude::*;
use graph6_rs::write_graph6;

use graph_canon::CanonLabeling;

use crate::canon::IntoSubgraph;

fn assemble_map<N, E>(graph: &Graph<N, E>, subgraph_indices: Vec<HashSet<NodeIndex>>) -> HashMap<CanonLabeling, usize> 
where
    N: Debug + Clone + Send + Sync,
    E: Debug + Clone + Send + Sync,
{
    // let subgraphs = subgraph_indices
    //     .into_par_iter()
    //     .map(|indices| graph.into_subgraph(&indices))
    //     .map(canonical_form)
    //     .collect::<Vec<_>>();
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
    let subgraph_indices = enumerated_search(&graph, k);
    eprintln!("Subgraphs found: {}", subgraph_indices.len());
    assemble_map(graph, subgraph_indices)
}

fn run_random_enumerated<N, E>(graph: &Graph<N, E>, k: usize, p: f64, seed: usize) -> HashMap<CanonLabeling, usize> 
where
    N: Debug + Clone + Send + Sync,
    E: Debug + Clone + Send + Sync,
{
    eprintln!("Running partial enumerated search...");
    let subgraph_indices = random_enumerated_search(&graph, k, p, seed);
    eprintln!("Subgraphs found: {}", subgraph_indices.len());
    assemble_map(graph, subgraph_indices)
}

fn main() {
    let k = 3;
    let mut rng = ChaChaRng::seed_from_u64(0);
    let graph: Graph<(), (), Directed> = random_gnp_graph(&mut rng, 100, 0.5);
    
    let full_sg_map = run_enumerated(&graph, k);
    let partial_sg_map = run_random_enumerated(&graph, k, 0.5, 0);

    println!("{} {}", full_sg_map.len(), partial_sg_map.len());

    let full_sg_total = full_sg_map.values().sum::<usize>();
    let partial_sg_total = partial_sg_map.values().sum::<usize>();

    for subgraph in full_sg_map.keys() {
        let signature = write_graph6(subgraph.flat_adjacency(), k, true);
        let full_count = full_sg_map.get(subgraph).unwrap();
        let partial_count = partial_sg_map.get(subgraph).unwrap_or(&0);
        let full_frequency = *full_count as f64 / full_sg_total as f64;
        let partial_frequency = *partial_count as f64 / partial_sg_total as f64;

        println!("{}\t{}\t{}\t{}\t{}", signature, full_count, partial_count, full_frequency, partial_frequency);
        // println!("{}", signature);
    }

}
