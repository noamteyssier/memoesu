mod canon;
mod esu;

use canon::canonical_form;
use esu::enumerated_search;

use hashbrown::{HashSet, HashMap};
use petgraph::{Undirected, Graph, Directed, visit::GetAdjacencyMatrix};
use petgraph_gen::random_gnp_graph;
use rayon::prelude::*;
use graph6_rs::write_graph6;

use crate::canon::IntoSubgraph;
use petgraph::graph::NodeIndex;


fn main() {
    // let edges = [
    //     (0, 1),
    //     (0, 2),
    //     (1, 2),
    //     (3, 0),
    //     (4, 0),
    //     (5, 1),
    //     (6, 1),
    //     (7, 2),
    //     (8, 2),
    // ];
    // let graph = Graph::<(), (), Undirected>::from_edges(&edges);
    // let edges = (0..1000).map(|x| (x, x + 1)).collect::<Vec<_>>();
    // let graph = UnGraph::<(), ()>::from_edges(&edges);
    
    let k = 3;
    let mut rng = rand::thread_rng();
    // let graph: Graph<(), (), Undirected> = random_gnp_graph(&mut rng, 100, 0.2);
    let graph: Graph<(), (), Directed> = random_gnp_graph(&mut rng, 100, 0.2);
    eprintln!("Starting Search...");
    let subgraph_indices = enumerated_search(&graph, k);
    eprintln!("Calculating Canonical Forms...");
    let subgraphs = subgraph_indices
        .into_par_iter()
        .map(|indices| graph.into_subgraph(&indices))
        .map(canonical_form)
        .collect::<Vec<_>>();

    eprintln!("Counting Canonical Forms...");
    let mut sg_map = HashMap::new();
    for subgraph in subgraphs {
        let count = sg_map.entry(subgraph).or_insert(0);
        *count += 1;
    }

    for subgraph in sg_map.keys() {
        let matrix = subgraph.adjacency_matrix();
        let bv = (0..matrix.len())
            .map(|x| {
                if matrix.contains(x) {
                    1
                } else {
                    0
                }
            })
            .collect::<Vec<usize>>();
        let repr = write_graph6(bv, k, true);
        println!("{}", repr);
        // break;
    }
}
