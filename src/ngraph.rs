use bitvec::prelude::*;
use nauty_Traces_sys::{
    bit, densenauty, empty_graph, optionblk, set, statsblk, ADDONEARC, GRAPHROW, SETBT, SETWD,
    SETWORDSNEEDED,
};
use std::os::raw::c_int;

/// A dense graph representation for use with nauty.
#[derive(Debug)]
pub struct NautyGraph {
    /// The binary representation of the graph.
    pub graph: Vec<u64>,

    /// The binary representation of the canonical labeling.
    pub canon: Vec<u64>,

    /// The number of nodes in the graph.
    pub n: usize,

    /// The number of edges in the graph.
    pub e: usize,

    /// The number of words in the graph representation.
    pub m: usize,

    /// The nodes labeling / coloring of the graph.
    pub nodes: Nodes,

    /// The options for the nauty run.
    pub opts: optionblk,

    /// The statistics of the nauty run.
    pub stats: statsblk,
}
impl NautyGraph {
    pub fn new_directed(n: usize) -> Self {
        let m = SETWORDSNEEDED(n);
        let graph = empty_graph(m, n);
        let canon = empty_graph(m, n);
        let opts = opts_default_dir();
        let stats = statsblk::default();
        Self {
            graph,
            canon,
            n,
            m,
            opts,
            stats,
            e: 0,
            nodes: Nodes::new(n),
        }
    }
    pub fn new_undirected(n: usize) -> Self {
        let m = SETWORDSNEEDED(n);
        let graph = empty_graph(m, n);
        let canon = empty_graph(m, n);
        let opts = opts_default_undir();
        let stats = statsblk::default();
        Self {
            graph,
            canon,
            n,
            m,
            opts,
            stats,
            e: 0,
            nodes: Nodes::new(n),
        }
    }
    pub fn add_arc(&mut self, u: usize, v: usize) {
        ADDONEARC(&mut self.graph, u, v, self.m);
        self.e += 1;
    }

    pub fn rm_arc(&mut self, u: usize, v: usize) {
        DELONEARC(&mut self.graph, u, v, self.m);
        self.e -= 1;
    }

    pub fn run(&mut self) {
        unsafe {
            densenauty(
                self.graph.as_mut_ptr(),
                self.nodes.lab.as_mut_ptr(),
                self.nodes.ptn.as_mut_ptr(),
                self.nodes.orbits.as_mut_ptr(),
                &mut self.opts,
                &mut self.stats,
                self.m as c_int,
                self.n as c_int,
                self.canon.as_mut_ptr(),
            )
        }
    }

    pub fn clear_canon(&mut self) {
        self.canon[..].iter_mut().for_each(|x| *x = 0);
    }

    pub fn clear_graph(&mut self) {
        self.graph[..].iter_mut().for_each(|x| *x = 0);
    }

    pub fn canon(&self) -> &[u64] {
        &self.canon
    }

    pub fn pprint_graph(&self) -> Vec<u8> {
        let mut bit_vector = Vec::with_capacity(self.n * self.n);
        for num in self.graph.iter() {
            let bv = num.view_bits::<Msb0>();
            for b in bv.iter().take(self.n) {
                if *b {
                    bit_vector.push(1);
                } else {
                    bit_vector.push(0);
                }
            }
        }
        bit_vector
    }
}

#[derive(Debug)]
pub struct Nodes {
    /// The node labeling.
    pub lab: Vec<c_int>,

    /// The node partition.
    pub ptn: Vec<c_int>,

    /// The node orbits.
    pub orbits: Vec<c_int>,
}
impl Nodes {
    pub fn new(n: usize) -> Self {
        Self {
            lab: (0..n as c_int).collect(),
            ptn: vec![0; n],
            orbits: vec![0; n],
        }
    }
}

#[allow(non_snake_case)]
fn DELONEARC(g: &mut [u64], v: usize, w: usize, m: usize) {
    DELELEMENT(GRAPHROW(g, v, m), w);
}

#[allow(non_snake_case)]
fn DELONEEDGE(g: &mut [u64], v: usize, w: usize, m: usize) {
    DELONEARC(g, v, w, m);
    DELONEARC(g, w, v, m);
}

#[allow(non_snake_case)]
fn DELELEMENT(setadd: &mut [set], pos: usize) {
    setadd[SETWD(pos)] &= !bit[SETBT(pos)]
}

fn opts_default_undir() -> optionblk {
    optionblk {
        getcanon: 1,
        ..Default::default()
    }
}

fn opts_default_dir() -> optionblk {
    optionblk {
        getcanon: 1,
        digraph: 1,
        ..Default::default()
    }
}
