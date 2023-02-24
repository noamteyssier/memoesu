use fixedbitset::FixedBitSet;
use hashbrown::HashSet;
use petgraph::{graph::NodeIndex, EdgeType, Graph};

/// Populates a graph with nodes, returning a vector of the original indices
fn populate_nodes<Ty: EdgeType>(
    graph: &mut Graph<(), (), Ty>,
    indices: &HashSet<NodeIndex>,
) -> Vec<usize> {
    indices
        .iter()
        .map(|idx| {
            graph.add_node(());
            idx.index()
        })
        .collect()
}

fn populate_edges<Ty: EdgeType>(
    graph: &mut Graph<(), (), Ty>,
    adj: &FixedBitSet,
    n: usize,
    indices: &[usize],
) {
    for (idx, g_idx) in indices.iter().enumerate() {
        for (jdx, g_jdx) in indices.iter().enumerate() {
            if adj.contains((g_idx * n + g_jdx) as usize) {
                graph.add_edge(NodeIndex::new(idx), NodeIndex::new(jdx), ());
            }
        }
    }
}

pub fn build_subgraph<N, E, Ty: EdgeType>(
    adj: &FixedBitSet,
    n: usize,
    indices: &HashSet<NodeIndex>,
) -> Graph<(), (), Ty> {
    let mut subgraph = Graph::with_capacity(indices.len(), 0);
    let indices = populate_nodes(&mut subgraph, indices);
    populate_edges(&mut subgraph, adj, n, &indices);
    subgraph
}
