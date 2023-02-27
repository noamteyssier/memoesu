use std::fmt::Debug;

use fixedbitset::FixedBitSet;
use crate::bitgraph::BitGraph;

#[derive(Clone)]
pub struct Walker<'a> {
    /// A reference to the underlying bitgraph.
    bitgraph: &'a BitGraph,

    /// The subgraph of the current walk.
    sub: FixedBitSet,

    /// The extension of the current walk.
    ext: FixedBitSet,

    /// The neighborhood of the current walk.
    nbh: FixedBitSet,

    /// The current exclusive neighborhood of the walk.
    exc: FixedBitSet,

    /// The root vertex of the current walk.
    root: usize,

    /// The exploratory vertex of the current walk.
    w: usize,

    /// The size of the current subgraph in the walk.
    n_sub: usize,

    /// The size of the current extension in the walk.
    n_ext: usize,
}
impl<'a> Debug for Walker<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Walker")
            .field("root", &self.root)
            .field("vertex", &self.w)
            .field("n_sub", &self.n_sub)
            .field("n_ext", &self.n_ext)
            .field("sub", &bitset_as_string(&self.sub))
            .field("ext", &bitset_as_string(&self.ext))
            .field("exc", &bitset_as_string(&self.exc))
            .field("nbh", &bitset_as_string(&self.nbh))
            .finish()
    }
}
impl<'a> Walker<'a> {
    pub fn new(bitgraph: &'a BitGraph, root: usize) -> Self {
        let sub = initial_subgraph(root, bitgraph.n);
        let ext = initial_extension(bitgraph, root);
        let nbh = initial_neighborhood(&ext, root);
        let exc = initial_exclusive(bitgraph.n);
        let n_sub = sub.count_ones(..);
        let n_ext = ext.count_ones(..);
        Self {
            bitgraph,
            sub,
            ext,
            nbh,
            exc,
            root,
            w: root,
            n_sub,
            n_ext,
        }
    }

    pub fn is_complete(&self, k: usize) -> bool {
        self.n_sub >= k
    }

    pub fn is_searching(&self) -> bool {
        self.n_ext != 0
    }

    pub fn step_down(&mut self) {
        self.pop_extension();
        self.append_subgraph(self.w);
        self.exclusive_neighbors(self.w);
        self.expand_neighborhood();
        self.expand_extension();
    }

    fn pop_extension(&mut self) {
        self.w = self.ext.ones().next().unwrap();
        self.ext.set(self.w, false);
        self.n_ext -= 1;
    }

    fn append_subgraph(&mut self, w: usize) {
        self.sub.set(w, true);
        self.n_sub += 1;
    }

    fn exclusive_neighbors(&mut self, w: usize) {
        self.exc.clear();
        self.exc.union_with(self.bitgraph.neighbors(w));
        self.exc.difference_with(&self.nbh);
    }
    
    fn expand_neighborhood(&mut self) {
        self.nbh.union_with(&self.exc);
    }

    fn expand_extension(&mut self) {
        self.ext.union_with(&self.exc);
        self.ext.set(self.w, false);
        self.ext.set_range(0..self.root, false);
        self.n_ext += self.exc.count_ones(..);
    }
}

fn bitset_as_string(bitset: &FixedBitSet) -> String {
    let mut string = String::new();
    string.push_str("{ ");
    for (idx, i) in bitset.ones().enumerate() {
        if idx > 0 {
            string.push_str(", ");
        }
        string.push_str(&format!("{}", i));
    }
    string.push_str(" }");
    string
}

// #[inline(always)]
fn initial_subgraph(v: usize, n: usize) -> FixedBitSet {
    let mut subgraph = FixedBitSet::with_capacity(n);
    subgraph.insert(v);
    subgraph
}

// #[inline(always)]
fn initial_extension(bitgraph: &BitGraph, v: usize) -> FixedBitSet {
    let mut extension = bitgraph.neighbors(v).clone();
    extension.set_range(0..v, false);
    extension
}

fn initial_exclusive(n: usize) -> FixedBitSet {
    let mut exclusive = FixedBitSet::with_capacity(n);
    exclusive
}

// #[inline(always)]
fn initial_neighborhood(ext: &FixedBitSet, v: usize) -> FixedBitSet {
    let mut neighborhood = ext.clone();
    neighborhood.insert(v);
    neighborhood
}
