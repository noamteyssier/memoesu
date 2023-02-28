use fixedbitset::FixedBitSet;
use petgraph::{Graph, EdgeType};
use rayon::prelude::*;
use crate::{bitgraph::BitGraph, walker::Walker};


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
fn overwrite_extension(
    exc: &FixedBitSet,
    ext: &FixedBitSet,
    v: usize,
    w: usize,
    ) -> FixedBitSet {
    let mut new_ext = ext.clone();
    new_ext.union_with(exc);
    new_ext.set(w, false);
    new_ext.set_range(..v, false);
    new_ext
}


pub fn enumerate_subgraphs<N, E, Ty>(
    graph: &Graph<N, E, Ty>, 
    k: usize
) where
    Ty: EdgeType,
{
    let bitgraph = BitGraph::from_graph(graph);
    // let mut num_subgraphs = 0;
    let num_subgraphs = (0..bitgraph.n)
        .into_par_iter()
        .map(|v| {
            let mut num_subgraphs = 0;
            let mut walker = Walker::new(&bitgraph, v, k);
            extend_subgraph(&mut num_subgraphs, &mut walker);
            num_subgraphs
        })
        .reduce(|| 0, |a, b| a + b);
    // for v in (0..bitgraph.n).into_par_iter() {
    //     let mut walker = Walker::new(&bitgraph, v, k);
    //     extend_subgraph(&mut num_subgraphs, &mut walker);
    //     // break;
    // }
    println!("Found {} subgraphs", num_subgraphs);
}

fn extend_subgraph(
    num_subgraphs: &mut usize,
    walker: &mut Walker) 
{
    while !walker.is_finished() {
        if walker.is_descending() {
            if walker.has_extension() {
                walker.descend();
            } else {
                walker.ascend();
            }
        } else {
            // walker.debug_subgraph();
            *num_subgraphs += 1;
            walker.ascend();
        }
    }
}
