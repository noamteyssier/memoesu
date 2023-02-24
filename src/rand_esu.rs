use crate::utils::{
    append_exclusive, append_subgraph, exclusive_neighborhood, initial_extension,
    initial_neighborhood, overwrite_extension, pop_extension,
};
use hashbrown::HashSet;
use petgraph::{graph::NodeIndex, EdgeType, Graph};
use rand::Rng;
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use rayon::prelude::*;

pub fn random_enumerate_subgraphs<N, E, Ty>(
    graph: &Graph<N, E, Ty>,
    k: usize,
    p: f64,
    seed: usize,
) -> Vec<HashSet<NodeIndex>>
where
    N: Sync,
    E: Sync,
    Ty: EdgeType + Sync,
{
    graph
        .node_indices()
        .into_iter()
        .par_bridge()
        .map(|v| {
            let mut rng = ChaCha8Rng::seed_from_u64(seed as u64 + v.index() as u64);
            let mut all_subgraphs = Vec::new();
            let mut subgraph = HashSet::new();
            subgraph.insert(v);
            let mut ext = initial_extension(graph, &v);
            let cnh = initial_neighborhood(&ext, &v);
            if rng.gen::<f64>() < p {
                random_extend_subgraph(
                    graph,
                    &mut all_subgraphs,
                    &subgraph,
                    &mut ext,
                    &cnh,
                    &v,
                    k,
                    p,
                    &mut rng,
                );
            }
            all_subgraphs
        })
        .flatten()
        .collect::<Vec<HashSet<NodeIndex>>>()
}

fn random_extend_subgraph<N, E, Ty, R>(
    graph: &Graph<N, E, Ty>,
    all_subgraphs: &mut Vec<HashSet<NodeIndex>>,
    subgraph: &HashSet<NodeIndex>,
    ext: &mut HashSet<NodeIndex>,
    cnh: &HashSet<NodeIndex>,
    v: &NodeIndex,
    k: usize,
    p: f64,
    rng: &mut R,
) where
    Ty: EdgeType,
    R: Rng + Sync,
{
    if subgraph.len() < k {
        while !ext.is_empty() {
            let w = pop_extension(ext);
            let new_sg = append_subgraph(subgraph, &w);
            let exc = exclusive_neighborhood(graph, cnh, &w);
            let new_cnh = append_exclusive(cnh, &exc);
            let mut new_ext = overwrite_extension(&exc, ext, v, &w);

            // let mut tmp_sg = new_sg.iter().map(|i| i.index()).collect::<Vec<_>>();
            // let mut tmp_exc = exc.iter().map(|i| i.index()).collect::<Vec<_>>();
            // let mut tmp_cnh = new_cnh.iter().map(|i| i.index()).collect::<Vec<_>>();
            // let mut tmp_ext = new_ext.iter().map(|i| i.index()).collect::<Vec<_>>();
            // tmp_sg.sort();
            // tmp_exc.sort();
            // tmp_cnh.sort();
            // tmp_ext.sort();

            // println!(">> {:?}", tmp_sg);
            // println!("\t  W -> {}", w.index() + 1);
            // println!("\tEXC -> {:?}", tmp_exc);
            // println!("\tCNH -> {:?}", tmp_cnh);
            // println!("\tEXT -> {:?}", tmp_ext);

            if rng.gen::<f64>() < p {
                random_extend_subgraph(
                    graph,
                    all_subgraphs,
                    &new_sg,
                    &mut new_ext,
                    &new_cnh,
                    &w,
                    k,
                    p,
                    rng,
                );
            }
        }
    } else {
        all_subgraphs.push(subgraph.clone());
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
