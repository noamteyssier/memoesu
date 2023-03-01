use crate::{bitgraph::BitGraph, walker::Walker};
use fixedbitset::FixedBitSet;
use hashbrown::{HashMap, HashSet};
use petgraph::{EdgeType, Graph};
use rayon::prelude::*;

// #[inline(always)]
fn append_subgraph(sub: &FixedBitSet, w: usize) -> FixedBitSet {
    let mut new_sub = sub.clone();
    new_sub.insert(w);
    new_sub
}

// #[inline(always)]
fn exclusive_neighbors(bitgraph: &BitGraph, nbh: &FixedBitSet, w: usize) -> FixedBitSet {
    let mut exc = bitgraph.neighbors(w).clone();
    exc.difference_with(nbh);
    exc
}

// #[inline(always)]
fn append_exclusive(nbh: &FixedBitSet, exc: &FixedBitSet) -> FixedBitSet {
    let mut new_nbh = nbh.clone();
    new_nbh.union_with(exc);
    new_nbh
}

// #[inline(always)]
fn overwrite_extension(exc: &FixedBitSet, ext: &FixedBitSet, v: usize, w: usize) -> FixedBitSet {
    let mut new_ext = ext.clone();
    new_ext.union_with(exc);
    new_ext.set(w, false);
    new_ext.set_range(..v, false);
    new_ext
}

pub fn enumerate_subgraphs<N, E, Ty>(graph: &Graph<N, E, Ty>, k: usize)
where
    Ty: EdgeType,
{
    let bitgraph = BitGraph::from_graph(graph);
    let mut canon_counts = HashMap::new();
    let mut num_subgraphs = 0;
    (0..bitgraph.n)
        // .into_par_iter()
        // .take(1)
        .for_each(|v| {
            // let mut num_subgraphs = 0;
            let mut walker = Walker::new(&bitgraph, v, k);
            extend_subgraph(&mut canon_counts, &mut num_subgraphs, &mut walker);
        });
    // .sum::<usize>();
    // for v in (0..bitgraph.n).into_par_iter() {
    //     let mut walker = Walker::new(&bitgraph, v, k);
    //     extend_subgraph(&mut num_subgraphs, &mut walker);
    //     // break;
    // }
    println!("Found {} subgraphs", num_subgraphs);
    println!("Found {} unique subgraphs", canon_counts.len());
}

fn extend_subgraph(
    canon_counts: &mut HashMap<Vec<u64>, usize>,
    num_subgraphs: &mut usize,
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
            let label = walker.run_nauty();
            // walker.debug_subgraph();
            *canon_counts.entry(label).or_insert(0) += 1;
            *num_subgraphs += 1;
            walker.clear_nauty();
            walker.ascend();
        }
    }
}
