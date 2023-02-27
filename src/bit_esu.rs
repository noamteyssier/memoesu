use fixedbitset::FixedBitSet;
use petgraph::{Graph, EdgeType};
use crate::bitgraph::BitGraph;

#[inline(always)]
fn initial_subgraph(v: usize, n: usize) -> FixedBitSet {
    let mut subgraph = FixedBitSet::with_capacity(n);
    subgraph.insert(v);
    subgraph
}

#[inline(always)]
fn initial_extension(bitgraph: &BitGraph, v: usize) -> FixedBitSet {
    let mut extension = bitgraph.neighbors(v).clone();
    extension.set_range(0..v, false);
    extension
}

#[inline(always)]
fn initial_neighborhood(ext: &FixedBitSet, v: usize) -> FixedBitSet {
    let mut neighborhood = ext.clone();
    neighborhood.insert(v);
    neighborhood
}

#[inline(always)]
fn pop_extension(ext: &mut FixedBitSet) -> usize {
    let w = ext.ones().next().unwrap();
    ext.set(w, false);
    w
}

#[inline(always)]
fn append_subgraph(sub: &FixedBitSet, w: usize) -> FixedBitSet {
    let mut new_sub = sub.clone();
    new_sub.insert(w);
    new_sub
}

#[inline(always)]
fn exclusive_neighbors(bitgraph: &BitGraph, nbh: &FixedBitSet, w: usize) -> FixedBitSet {
    let mut exc = bitgraph.neighbors(w).clone();
    exc.difference_with(nbh);
    exc
}

#[inline(always)]
fn append_exclusive(nbh: &FixedBitSet, exc: &FixedBitSet) -> FixedBitSet {
    let mut new_nbh = nbh.clone();
    new_nbh.union_with(exc);
    new_nbh
}

#[inline(always)]
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
    let mut all_subgraphs = Vec::new();
    for v in 0..bitgraph.n {
        let sub = initial_subgraph(v, bitgraph.n);
        let mut ext = initial_extension(&bitgraph, v);
        let nbh = initial_neighborhood(&ext, v);
        extend_subgraph(&bitgraph, &mut all_subgraphs, &sub, &mut ext, &nbh, v, k);
    }
    println!("Found {} subgraphs", all_subgraphs.len());
}

fn extend_subgraph(
    bitgraph: &BitGraph,
    all_subgraphs: &mut Vec<FixedBitSet>,
    sub: &FixedBitSet,
    ext: &mut FixedBitSet,
    nbh: &FixedBitSet,
    v: usize,
    k: usize) {

    if sub.count_ones(..) < k {
        while !ext.is_clear() {
            let w = pop_extension(ext);
            let n_sub = append_subgraph(sub, w);
            let w_exc = exclusive_neighbors(bitgraph, nbh, w);
            let w_nbh = append_exclusive(nbh, &w_exc);
            let mut w_ext = overwrite_extension(&w_exc, ext, v, w);
            extend_subgraph(bitgraph, all_subgraphs, &n_sub, &mut w_ext, &w_nbh, v, k);
        }
    } else {
        all_subgraphs.push(sub.to_owned());
    }

}
