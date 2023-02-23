use hashbrown::HashSet;
use petgraph::{graph::NodeIndex, EdgeType, Graph};

pub fn exclusive_neighborhood<N, E, Ty>(
    graph: &Graph<N, E, Ty>,
    subgraph: &HashSet<NodeIndex>,
    current_neighborhood: &HashSet<NodeIndex>,
    v: NodeIndex,
) -> HashSet<NodeIndex>
where
    Ty: EdgeType,
{
    graph
        .neighbors(v)
        .filter(|x| x.index() > v.index())
        .filter(|x| !subgraph.contains(x))
        .filter(|x| !current_neighborhood.contains(x))
        .collect()
}

pub fn insert_subgraph(subgraph: &HashSet<NodeIndex>, w: NodeIndex) -> HashSet<NodeIndex> {
    let mut w_subgraph = subgraph.clone();
    w_subgraph.insert(w);
    w_subgraph
}

pub fn modify_extension(
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

pub fn insert_neighborhood(
    current_neighborhood: &HashSet<NodeIndex>,
    e_neighborhood: &HashSet<NodeIndex>,
) -> HashSet<NodeIndex> {
    current_neighborhood
        .union(e_neighborhood)
        .copied()
        .collect()
}
