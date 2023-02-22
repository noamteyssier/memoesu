use petgraph::{graph::NodeIndex, prelude::UnGraph, Undirected};
use hashbrown::{HashMap, HashSet};
use nauty_pet::prelude::CanonGraph;

pub fn select_subgraph(graph: &UnGraph<(), ()>, indices: &HashSet<NodeIndex>) -> UnGraph<(), ()> {
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

pub fn canonical_form(graph: UnGraph<(), ()>) -> CanonGraph<(), (), Undirected> {
    CanonGraph::from(graph)
}
