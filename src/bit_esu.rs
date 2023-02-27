use fixedbitset::FixedBitSet;
use petgraph::{Graph, EdgeType};
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
    let mut all_subgraphs = Vec::new();
    for v in 0..bitgraph.n {
        let mut walker = Walker::new(&bitgraph, v);
        extend_subgraph(&bitgraph, &mut all_subgraphs, &mut walker, k);
        // let sub = initial_subgraph(v, bitgraph.n);
        // let mut ext = initial_extension(&bitgraph, v);
        // let nbh = initial_neighborhood(&ext, v);
        // extend_subgraph(&bitgraph, &mut all_subgraphs, &mut walker, v, k);
        // break;
    }
    // println!("Found {} subgraphs", all_subgraphs.len());
}

fn extend_subgraph(
    bitgraph: &BitGraph,
    all_subgraphs: &mut Vec<FixedBitSet>,
    walker: &mut Walker,
    k: usize) {

    if walker.is_complete(k) {
        println!("Found Subgraph!");
    } else {
        // println!("{:?}", walker);
        while walker.is_searching() {
            walker.step_down();
            // println!("{:?}", walker);
            extend_subgraph(bitgraph, all_subgraphs, walker, k);
            // break;
            // let w = walker.pop_extension();
            // let n_sub = walker.append_subgraph(w);
            // let w_exc = walker.exclusive_neighbors(bitgraph, w);
            // let w_nbh = walker.append_exclusive(&w_exc);
            // let w_ext = walker.overwrite_extension(&w_exc, w);
            // walker.extend_subgraph(bitgraph, all_subgraphs, &n_sub, &mut w_ext, &w_nbh, v, k);
        }
    }

    // if sub.count_ones(..) < k {
    //     while !ext.is_clear() {
    //         let w = pop_extension(ext);
    //         let n_sub = append_subgraph(sub, w);
    //         let w_exc = exclusive_neighbors(bitgraph, nbh, w);
    //         let w_nbh = append_exclusive(nbh, &w_exc);
    //         let mut w_ext = overwrite_extension(&w_exc, ext, v, w);
    //         extend_subgraph(bitgraph, all_subgraphs, &n_sub, &mut w_ext, &w_nbh, v, k);
    //     }
    // } else {
    //     all_subgraphs.push(sub.to_owned());
    // }

}
