mod canon;
mod esu;

use canon::canonical_form;
use esu::enumerated_search;

use hashbrown::HashMap;
use petgraph_gen::random_gnp_graph;
use rayon::prelude::*;

use crate::canon::select_subgraph;



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
    // let edges = (0..1000).map(|x| (x, x + 1)).collect::<Vec<_>>();
    // let graph = UnGraph::<(), ()>::from_edges(&edges);
    let mut rng = rand::thread_rng();
    let graph = random_gnp_graph(&mut rng, 10, 0.5);
    eprintln!("Starting Search...");
    let subgraph_indices = enumerated_search(&graph, 3);
    eprintln!("Calculating Canonical Forms...");
    let subgraphs = subgraph_indices
        .into_par_iter()
        .map(|indices| select_subgraph(&graph, &indices))
        .map(canonical_form)
        .collect::<Vec<_>>();

    eprintln!("Counting Canonical Forms...");
    let mut sg_map = HashMap::new();
    for subgraph in subgraphs {
        let count = sg_map.entry(subgraph).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", sg_map);
}
