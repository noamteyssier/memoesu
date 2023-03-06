use super::{BitGraph, EnumResult, Walker};
use petgraph::{EdgeType, Graph};
use rayon::prelude::*;

type CanonCounts = hashbrown::HashMap<Vec<u64>, usize>;
type Memo = flurry::HashMap<Vec<u64>, Vec<u64>>;

pub fn parallel_enumerate_subgraphs<N, E, Ty>(graph: &Graph<N, E, Ty>, k: usize) -> EnumResult
where
    Ty: EdgeType,
{
    let bitgraph = BitGraph::from_graph(graph);
    let memo = Memo::with_capacity(bitgraph.n * k);
    let (canon_counts, num_subgraphs, num_dups) = (0..bitgraph.n)
        .par_bridge()
        .map(|v| {
            let mut walker = Walker::new(&bitgraph, v, k);
            let mut canon_counts = CanonCounts::with_capacity(walker.bitgraph.n * walker.k);
            let mut num_subgraphs = 0;
            let mut num_dups = 0;
            parallel_extend_subgraph(
                &mut canon_counts,
                &memo,
                &mut num_subgraphs,
                &mut num_dups,
                &mut walker,
            );
            (canon_counts, num_subgraphs, num_dups)
        })
        .reduce(
            || (CanonCounts::with_capacity(bitgraph.n * k), 0, 0),
            |mut acc, x| {
                for (k, v) in x.0 {
                    *acc.0.entry(k).or_insert(0) += v;
                }
                acc.1 += x.1;
                acc.2 += x.2;
                acc
            },
        );
    EnumResult::new(canon_counts, num_subgraphs, num_dups)
}

fn parallel_extend_subgraph(
    canon_counts: &mut CanonCounts,
    memo: &Memo,
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
            if let Some(label) = memo.get(walker.nauty_graph(), &memo.guard()) {
                *canon_counts.entry(label.clone()).or_insert(0) += 1;
                *num_dups += 1;
            } else {
                let label = walker.run_nauty();
                memo.insert(walker.nauty_graph().to_vec(), label.clone(), &memo.guard());
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
        let result = parallel_enumerate_subgraphs(&graph, 3);
        assert_eq!(result.total_subgraphs(), 16);
        assert_eq!(result.unique_subgraphs(), 4);
    }

    #[test]
    fn example_s4() {
        let filepath = "example/example.txt";
        let graph = load_numeric_graph(filepath, false).unwrap();
        let result = parallel_enumerate_subgraphs(&graph, 4);
        assert_eq!(result.total_subgraphs(), 24);
        assert_eq!(result.unique_subgraphs(), 8);
    }

    #[test]
    fn yeast_s3() {
        let filepath = "example/yeast.txt";
        let graph = load_numeric_graph(filepath, false).unwrap();
        let result = parallel_enumerate_subgraphs(&graph, 3);
        assert_eq!(result.total_subgraphs(), 13150);
        assert_eq!(result.unique_subgraphs(), 7);
    }

    #[test]
    fn yeast_s4() {
        let filepath = "example/yeast.txt";
        let graph = load_numeric_graph(filepath, false).unwrap();
        let result = parallel_enumerate_subgraphs(&graph, 4);
        assert_eq!(result.total_subgraphs(), 183174);
        assert_eq!(result.unique_subgraphs(), 34);
    }
}

