use petgraph::{graph::NodeIndex, Graph, EdgeType};
use hashbrown::{HashMap, HashSet};

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
