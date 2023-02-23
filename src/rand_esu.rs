use crate::utils::{
    exclusive_neighborhood, insert_neighborhood, insert_subgraph, modify_extension,
};
use hashbrown::HashSet;
use petgraph::{graph::NodeIndex, EdgeType, Graph};
use rand::Rng;
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use rayon::prelude::*;
use std::fmt::Debug;

pub fn random_enumerated_search<N, E, Ty>(
    graph: &Graph<N, E, Ty>,
    k: usize,
    p: f64,
    seed: usize,
) -> Vec<HashSet<NodeIndex>>
where
    N: Debug + Sync,
    E: Debug + Sync,
    Ty: EdgeType + Sync,
{
    graph
        .node_indices()
        .into_iter()
        .enumerate()
        .par_bridge()
        .map(|(idx, v)| {
            let mut rng = ChaCha8Rng::seed_from_u64(seed as u64 + idx as u64);
            let mut all_subgraphs = Vec::new();
            let v_subgraph = HashSet::from_iter(vec![v]);
            let mut v_extension = graph
                .neighbors(v)
                .filter(|&w| w.index() > v.index())
                .collect::<HashSet<_>>();
            let current_neighborhood = graph.neighbors(v).collect::<HashSet<_>>();

            if rng.gen::<f64>() < p {
                random_extend_subgraph(
                    &mut all_subgraphs,
                    graph,
                    &v_subgraph,
                    &mut v_extension,
                    &current_neighborhood,
                    v,
                    k,
                    p,
                    &mut rng,
                );
            }
            all_subgraphs
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn random_extend_subgraph<N, E, Ty, R>(
    all_subgraphs: &mut Vec<HashSet<NodeIndex>>,
    graph: &Graph<N, E, Ty>,
    subgraph: &HashSet<NodeIndex>,
    extension: &mut HashSet<NodeIndex>,
    current_neighborhood: &HashSet<NodeIndex>,
    v: NodeIndex,
    k: usize,
    p: f64,
    rng: &mut R,
) where
    Ty: EdgeType,
    R: Rng + Sync,
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

            if rng.gen::<f64>() < p {
                random_extend_subgraph(
                    all_subgraphs,
                    graph,
                    &w_subgraph,
                    &mut w_extension,
                    &w_current_neighborhood,
                    v,
                    k,
                    p,
                    rng,
                );
            }
        }
    }
}

#[cfg(test)]
mod testing {

    #[test]
    fn build_undirected_test_graph() {
        let _edges = [
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
