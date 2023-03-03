use crate::{bitgraph::BitGraph, walker::Walker};
use petgraph::{EdgeType, Graph};

type CanonCounts = hashbrown::HashMap<Vec<u64>, usize>;
type Memo = hashbrown::HashMap<Vec<u64>, Vec<u64>>;

pub fn enumerate_subgraphs<N, E, Ty>(graph: &Graph<N, E, Ty>, k: usize) -> hashbrown::HashMap<Vec<u64>, usize>
where
    Ty: EdgeType,
{
    let bitgraph = BitGraph::from_graph(graph);
    let mut canon_counts = CanonCounts::with_capacity(bitgraph.n * k);
    let mut memo = Memo::with_capacity(bitgraph.n * k);
    let mut num_subgraphs = 0;
    let mut num_dups = 0;
    (0..bitgraph.n)
        .for_each(|v| {
            let mut walker = Walker::new(&bitgraph, v, k);
            extend_subgraph(
                &mut canon_counts,
                &mut memo,
                &mut num_subgraphs,
                &mut num_dups,
                &mut walker,
            );
        });

    eprintln!(">> Num subgraphs           : {num_subgraphs}");
    eprintln!(">> Unique subgraphs        : {}", canon_counts.len());
    eprintln!(">> Duplicate Subgraphs     : {num_dups}");

    canon_counts
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
