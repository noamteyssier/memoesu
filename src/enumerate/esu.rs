use super::{BitGraph, EnumResult, Walker};
use petgraph::{EdgeType, Graph};

type CanonCounts = hashbrown::HashMap<Vec<u64>, usize>;
// type Memo = fxhash::FxHashMap<Vec<u64>, Vec<u64>>;
type Memo = hashbrown::HashMap<Vec<u64>, Vec<u64>>;

pub fn enumerate_subgraphs<N, E, Ty>(graph: &Graph<N, E, Ty>, k: usize) -> EnumResult
where
    Ty: EdgeType,
{
    let bitgraph = BitGraph::from_graph(graph);
    let mut canon_counts = CanonCounts::with_capacity(bitgraph.n * k);
    let mut memo = Memo::with_capacity(bitgraph.n * k);
    let mut num_subgraphs = 0;
    let mut num_dups = 0;
    (0..bitgraph.n).for_each(|v| {
        let mut walker = Walker::new(&bitgraph, v, k);
        extend_subgraph(
            &mut canon_counts,
            &mut memo,
            &mut num_subgraphs,
            &mut num_dups,
            &mut walker,
        );
    });
    EnumResult::new(canon_counts, num_subgraphs, num_dups)
}

fn extend_subgraph(
    canon_counts: &mut CanonCounts,
    memo: &mut Memo,
    num_subgraphs: &mut usize,
    num_dups: &mut usize,
    walker: &mut Walker,
) {
    while !walker.is_finished() {
        if walker.is_descending() {
            if walker.has_extension() {
                walker.descend();
            } else {
                walker.ascend();
            }
        } else {
            walker.fill_nauty();
            if let Some(label) = memo.get(walker.nauty_graph()) {
                *canon_counts.entry(label.clone()).or_insert(0) += 1;
                *num_dups += 1;
            } else {
                let label = walker.run_nauty();
                memo.insert(walker.nauty_graph().to_vec(), label.clone());
                *canon_counts.entry(label.clone()).or_insert(0) += 1;
            }
            *num_subgraphs += 1;
            walker.clear_nauty();
            walker.ascend();
        }
    }
}

#[cfg(test)]
mod testing {

    use super::*;
    use crate::io::load_numeric_graph;

    #[test]
    fn example_s3() {
        let filepath = "example/example.txt";
        let graph = load_numeric_graph(filepath, false).unwrap();
        let result = enumerate_subgraphs(&graph, 3);
        assert_eq!(result.total_subgraphs(), 16);
        assert_eq!(result.unique_subgraphs(), 4);
    }

    #[test]
    fn example_s4() {
        let filepath = "example/example.txt";
        let graph = load_numeric_graph(filepath, false).unwrap();
        let result = enumerate_subgraphs(&graph, 4);
        assert_eq!(result.total_subgraphs(), 24);
        assert_eq!(result.unique_subgraphs(), 8);
    }

    #[test]
    fn yeast_s3() {
        let filepath = "example/yeast.txt";
        let graph = load_numeric_graph(filepath, false).unwrap();
        let result = enumerate_subgraphs(&graph, 3);
        assert_eq!(result.total_subgraphs(), 13150);
        assert_eq!(result.unique_subgraphs(), 7);
    }

    #[test]
    fn yeast_s4() {
        let filepath = "example/yeast.txt";
        let graph = load_numeric_graph(filepath, false).unwrap();
        let result = enumerate_subgraphs(&graph, 4);
        assert_eq!(result.total_subgraphs(), 183174);
        assert_eq!(result.unique_subgraphs(), 34);
    }
}
