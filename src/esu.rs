use std::fmt::Debug;
use rayon::prelude::*;
use petgraph::{graph::NodeIndex, Graph, EdgeType};
use hashbrown::HashSet;

pub fn enumerated_search<N, E, Ty>(graph: &Graph<N, E, Ty>, k: usize) -> Vec<HashSet<NodeIndex>>
where
    N: Debug + Sync,
    E: Debug + Sync,
    Ty: EdgeType + Sync,
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

fn extend_subgraph<N, E, Ty>(
    all_subgraphs: &mut Vec<HashSet<NodeIndex>>,
    graph: &Graph<N, E, Ty>,
    subgraph: &HashSet<NodeIndex>,
    extension: &mut HashSet<NodeIndex>,
    current_neighborhood: &HashSet<NodeIndex>,
    v: NodeIndex,
    k: usize,
) 
where
    Ty: EdgeType,
{
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

fn exclusive_neighborhood<N, E, Ty>(
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

#[cfg(test)]
mod testing {


    #[test]
    fn build_undirected_test_graph() {
        let edges = [
            (0, 1),
            (0, 2),
            (1, 2),
            (3, 0),
            (4, 0),
            (5, 1),
            (6, 1),
            (7, 2),
            (8, 2),
        ];


    }
}
