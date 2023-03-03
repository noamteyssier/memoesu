use crate::{bitgraph::BitGraph, walker::Walker};
use petgraph::{EdgeType, Graph};
use rayon::prelude::*;

type CanonCounts = hashbrown::HashMap<Vec<u64>, usize>;
type Memo = flurry::HashMap<Vec<u64>, Vec<u64>>;

pub fn parallel_enumerate_subgraphs<N, E, Ty>(graph: &Graph<N, E, Ty>, k: usize) -> hashbrown::HashMap<Vec<u64>, usize>
where
    Ty: EdgeType,
{
    let bitgraph = BitGraph::from_graph(graph);
    let memo = Memo::with_capacity(bitgraph.n * k);
    let (canon_counts, num_subgraphs, num_dups) = (0..bitgraph.n)
        .into_iter()
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

    eprintln!(">> Num subgraphs           : {num_subgraphs}");
    eprintln!(">> Unique subgraphs        : {}", canon_counts.len());
    eprintln!(">> Duplicate Subgraphs     : {num_dups}");

    canon_counts
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

