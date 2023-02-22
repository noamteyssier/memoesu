use std::hash::Hash;

use petgraph::{graph::{NodeIndex, IndexType}, Graph, EdgeType};
use hashbrown::{HashMap, HashSet};
use nauty_pet::prelude::CanonGraph;

pub trait IntoSubgraph<N, E, Ty>
where
    Ty: EdgeType,
{
    fn into_subgraph(&self, indices: &HashSet<NodeIndex>) -> Graph<(), (), Ty>;
}

impl<N, E, Ty> IntoSubgraph<N, E, Ty> for Graph<N, E, Ty>
where
    Ty: EdgeType,
{
    fn into_subgraph(&self, indices: &HashSet<NodeIndex>) -> Graph<(), (), Ty, u32> {
        let node_map = indices
            .iter()
            .enumerate()
            .map(|(idx, x)| (x, idx as u32))
            .collect::<HashMap<_, _>>();


        let edges = self.edge_indices()
            .map(|e| self.edge_endpoints(e).unwrap())
            .filter(|(u, v)| indices.contains(u) && indices.contains(v))
            .map(|(u, v)| (node_map[&u], node_map[&v]))
            .collect::<Vec<(_, _)>>();

        Graph::from_edges(&edges)
    }
}

// impl<N, E, Undirected> IntoSubgraph<N, E, Undirected> for Graph<N, E, Undirected>
// where
//     N: Clone,
//     E: Clone,
// {
//     fn into_subgraph(&self, indices: &HashSet<NodeIndex>) -> Graph<N, E, Undirected> {
//         let mut subgraph = Graph::new();
//         let node_map = indices
//             .iter()
//             .map(|x| (x, subgraph.add_node(self[*x].clone())))
//             .collect::<HashMap<_, _>>();
//         for edge in self.edge_indices() {
//             let (u, v) = self.edge_endpoints(edge).unwrap();
//             if indices.contains(&u) && indices.contains(&v) {
//                 let u_idx = node_map[&u];
//                 let v_idx = node_map[&v];
//                 subgraph.add_edge(u_idx, v_idx, self[edge].clone());
//             }
//         }
//         subgraph
//     }
// }

pub fn canonical_form<N, E, Ty>(graph: Graph<N, E, Ty>) -> CanonGraph<N, E, Ty> 
where
    N: Hash + Ord,
    E: Hash + Ord,
    Ty: EdgeType,
{
    CanonGraph::from(graph)
}
