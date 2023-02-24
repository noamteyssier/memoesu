use crate::utils::{
    append_exclusive, append_subgraph, exclusive_neighborhood, initial_extension,
    initial_neighborhood, overwrite_extension, pop_extension,
};
use hashbrown::HashSet;
use petgraph::{graph::NodeIndex, EdgeType, Graph};
use rayon::prelude::*;

pub fn enumerate_subgraphs<N, E, Ty>(graph: &Graph<N, E, Ty>, k: usize) -> Vec<HashSet<NodeIndex>>
where
    N: Sync,
    E: Sync,
    Ty: EdgeType + Sync,
{
    let all_subgraphs = graph
        .node_indices()
        .into_iter()
        .par_bridge()
        .map(|v| {
            let mut all_subgraphs = Vec::new();
            let mut subgraph = HashSet::new();
            subgraph.insert(v);
            let mut ext = initial_extension(graph, &v);
            let cnh = initial_neighborhood(&ext, &v);
            extend_subgraph(
                graph,
                &mut all_subgraphs,
                &mut subgraph,
                &mut ext,
                &cnh,
                &v,
                k,
            );
            all_subgraphs
        })
        .flatten()
        .collect::<Vec<HashSet<NodeIndex>>>();
    // println!("Subgraphs found: {}", all_subgraphs.len());
    all_subgraphs
}

fn extend_subgraph<N, E, Ty>(
    graph: &Graph<N, E, Ty>,
    all_subgraphs: &mut Vec<HashSet<NodeIndex>>,
    subgraph: &HashSet<NodeIndex>,
    ext: &mut HashSet<NodeIndex>,
    cnh: &HashSet<NodeIndex>,
    v: &NodeIndex,
    k: usize,
) where
    Ty: EdgeType,
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

            extend_subgraph(graph, all_subgraphs, &new_sg, &mut new_ext, &new_cnh, v, k);
        }
    } else {
        all_subgraphs.push(subgraph.clone());
    }
}

#[cfg(test)]
mod testing {
    use petgraph::{Directed, Graph, Undirected};

    #[test]
    fn test_undirected_graph() {
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
        let graph = Graph::<(), (), Undirected>::from_edges(&edges);
        let subgraphs = super::enumerate_subgraphs(&graph, 3);
        assert_eq!(subgraphs.len(), 16);
    }

    #[test]
    fn test_directed_graph() {
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
        let graph = Graph::<(), (), Directed>::from_edges(&edges);
        let subgraphs = super::enumerate_subgraphs(&graph, 3);
        assert_eq!(subgraphs.len(), 16);
    }
}
