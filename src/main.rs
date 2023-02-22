use hashbrown::{HashMap, HashSet};
use nauty_pet::prelude::CanonGraph;
use petgraph::{graph::NodeIndex, prelude::UnGraph, Undirected};
use petgraph_gen::random_gnp_graph;
use rayon::prelude::*;
use std::fmt::Debug;

fn enumerated_search<N, E>(graph: &UnGraph<N, E>, k: usize) -> Vec<HashSet<NodeIndex>>
where
    N: Debug + Sync,
    E: Debug + Sync,
{
    graph
        .node_indices()
        .into_iter()
        .par_bridge()
        .map(|v| {
            let mut all_subgraphs = Vec::new();
            let v_subgraph = HashSet::from_iter(vec![v]);
            let mut v_extension = graph
                .neighbors(v)
                .filter(|&w| w.index() > v.index())
                .collect::<HashSet<_>>();
            let current_neighborhood = graph.neighbors(v).collect::<HashSet<_>>();
            extend_subgraph(
                &mut all_subgraphs,
                graph,
                &v_subgraph,
                &mut v_extension,
                &current_neighborhood,
                v,
                k,
            );
            all_subgraphs
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn extend_subgraph<N: Debug, E: Debug>(
    all_subgraphs: &mut Vec<HashSet<NodeIndex>>,
    graph: &UnGraph<N, E>,
    subgraph: &HashSet<NodeIndex>,
    extension: &mut HashSet<NodeIndex>,
    current_neighborhood: &HashSet<NodeIndex>,
    v: NodeIndex,
    k: usize,
) {
    if subgraph.len() == k {
        all_subgraphs.push(subgraph.clone());
    } else {
        while !extension.is_empty() {
            let w = *extension.iter().next().unwrap();
            extension.remove(&w);

            let e_neighborhood = exclusive_neighborhood(graph, subgraph, current_neighborhood, w);
            let w_subgraph = insert_subgraph(subgraph, w);
            let mut w_extension = modify_extension(extension, &e_neighborhood, w);
            let w_current_neighborhood = insert_neighborhood(current_neighborhood, &e_neighborhood);
            extend_subgraph(
                all_subgraphs,
                graph,
                &w_subgraph,
                &mut w_extension,
                &w_current_neighborhood,
                v,
                k,
            );
        }
    }
}

fn exclusive_neighborhood<N: Debug, E: Debug>(
    graph: &UnGraph<N, E>,
    subgraph: &HashSet<NodeIndex>,
    current_neighborhood: &HashSet<NodeIndex>,
    v: NodeIndex,
) -> HashSet<NodeIndex> {
    graph
        .neighbors(v)
        .filter(|x| x.index() > v.index())
        .filter(|x| !subgraph.contains(x))
        .filter(|x| !current_neighborhood.contains(x))
        .collect()
}

fn insert_subgraph(subgraph: &HashSet<NodeIndex>, w: NodeIndex) -> HashSet<NodeIndex> {
    let mut w_subgraph = subgraph.clone();
    w_subgraph.insert(w);
    w_subgraph
}

fn modify_extension(
    extension: &HashSet<NodeIndex>,
    e_neighborhood: &HashSet<NodeIndex>,
    w: NodeIndex,
) -> HashSet<NodeIndex> {
    extension
        .union(e_neighborhood)
        .filter(|x| **x != w)
        .copied()
        .collect()
}

fn insert_neighborhood(
    current_neighborhood: &HashSet<NodeIndex>,
    e_neighborhood: &HashSet<NodeIndex>,
) -> HashSet<NodeIndex> {
    current_neighborhood
        .union(e_neighborhood)
        .copied()
        .collect()
}

fn select_subgraph(graph: &UnGraph<(), ()>, indices: &HashSet<NodeIndex>) -> UnGraph<(), ()> {
    let mut subgraph = UnGraph::new_undirected();
    let node_map = indices
        .iter()
        .map(|x| (x, subgraph.add_node(())))
        .collect::<HashMap<_, _>>();
    for edge in graph.edge_indices() {
        let (u, v) = graph.edge_endpoints(edge).unwrap();
        if indices.contains(&u) && indices.contains(&v) {
            let u_idx = node_map[&u];
            let v_idx = node_map[&v];
            subgraph.add_edge(u_idx, v_idx, ());
        }
    }
    subgraph
}

fn canonical_form(graph: UnGraph<(), ()>) -> CanonGraph<(), (), Undirected> {
    CanonGraph::from(graph)
}

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
    let graph = random_gnp_graph(&mut rng, 100, 0.3);
    eprintln!("Starting Search...");
    let subgraph_indices = enumerated_search(&graph, 5);
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
