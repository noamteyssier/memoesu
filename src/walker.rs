use std::{fmt::Debug, ops::Range};

use fixedbitset::FixedBitSet;
use crate::bitgraph::BitGraph;

pub struct FlatBitSet {
    data: FixedBitSet,
    n: usize,
    k: usize,
}
impl Debug for FlatBitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        string.push_str("FlatBitSet {\n");
        for i in 0..self.k {
            string.push_str(&format!("  {}: {:?}\n", i, self.get_ones_at_row(i).collect::<Vec<_>>()))
        }
        string.push_str("}");
        write!(f, "{}", string)
    }
}
impl FlatBitSet {
    pub fn new(n: usize, k: usize) -> Self {
        let data = FixedBitSet::with_capacity(n * k);
        Self { data, n, k }
    }

    /// Performs an inplace union of set at depth `u` with set at depth `v`.
    pub fn union(&mut self, u: usize, v: usize) {
        let u_offset = u * self.n;
        let v_offset = v * self.n;
        for i in 0..self.n {
            self.data.set(
                u_offset + i, 
                self.data[u_offset + i] || self.data[v_offset + i]
            );
        }
    }

    /// Performs an inplace union of set at depth `u` with an external `FixedBitSet`.
    pub fn union_with(&mut self, u: usize, other: &FixedBitSet) {
        let offset = u * self.n;
        for i in 0..self.n {
            self.data.set(
                offset + i, 
                self.data[offset + i] || other[i]
            );
        }
    }

    pub fn union_with_other(&mut self, u: usize, other: &Self, v: usize) {
        let u_offset = u * self.n;
        let v_offset = v * self.n;
        for i in 0..self.n {
            self.data.set(
                u_offset + i, 
                self.data[u_offset + i] || other.data[v_offset + i]
            );
        }
    }

    /// Performs an inplace intersection of set at depth `u` with set at depth `v`.
    pub fn intersection(&mut self, u: usize, v: usize) {
        let u_offset = u * self.n;
        let v_offset = v * self.n;
        for i in 0..self.n {
            self.data.set(
                u_offset + i, 
                self.data[u_offset + i] && self.data[v_offset + i]
            );
        }
    }

    /// Performs an inplace difference of set at depth `u` with set at depth `v`.
    pub fn difference(&mut self, u: usize, v: usize) {
        let u_offset = u * self.n;
        let v_offset = v * self.n;
        for i in 0..self.n {
            self.data.set(
                u_offset + i, 
                self.data[u_offset + i] && !self.data[v_offset + i]
            );
        }
    }

    pub fn difference_with_other(&mut self, u: usize, other: &Self, v: usize) {
        let u_offset = u * self.n;
        let v_offset = v * self.n;
        for i in 0..self.n {
            self.data.set(
                u_offset + i, 
                self.data[u_offset + i] && !other.data[v_offset + i]
            );
        }
    }

    /// Clears the set at depth `u`.
    pub fn clear(&mut self, u: usize) {
        let u_offset = u * self.n;
        self.data.set_range(u_offset..u_offset + self.n, false);
    }

    /// Sets a bit at depth `depth` and index `v`.
    pub fn set(&mut self, depth: usize, v: usize, enabled: bool) {
        let offset = depth * self.n;
        self.data.set(offset + v, enabled);
    }

    pub fn get(&self, depth: usize, v: usize) -> bool {
        let offset = depth * self.n;
        self.data[offset + v]
    }

    pub fn get_ones_at_row(&self, depth: usize) -> impl Iterator<Item = usize> + '_ {
        let bounds_low = depth * self.n;
        let bounds_high = bounds_low + self.n;
        self.data
            .ones()
            .filter(move |&x| x >= bounds_low && x < bounds_high)
            .map(move |x| x - bounds_low)
    }

    pub fn set_range(&mut self, depth: usize, range: Range<usize>, enabled: bool) {
        let offset = depth * self.n;
        self.data.set_range(offset + range.start..offset + range.end, enabled);
    }
}

#[derive(Debug)]
pub struct Walker<'a> {
    /// A reference to the underlying bitgraph.
    bitgraph: &'a BitGraph,

    /// The subgraph of the current walk.
    sub: FixedBitSet,

    /// The extension of the current walk.
    ext: FlatBitSet,

    /// The neighborhood of the current walk.
    nbh: FlatBitSet,

    /// The exclusive neighborhood of the current walk.
    exc: FlatBitSet,

    /// The root of the walk.
    root: usize,

    /// The current head of the walk.
    head: usize,

    /// The parent nodes of the current head.
    parent: Vec<usize>,

    /// The number of vertices in the graph.
    n: usize,

    /// The maximum size of the subgraph.
    k: usize, 

    /// The current depth of the walk.
    pub depth: usize,
}
impl<'a> Walker<'a> {
    pub fn new(bitgraph: &'a BitGraph, root: usize, k: usize) -> Self {
        let head = root;
        let mut parent = vec![0; k];
        parent[0] = root;
        let n = bitgraph.n;
        let depth = 0;

        // Initialize the subgraph, extension, and neighborhood.
        let mut sub = Self::init_subgraph(root, n);
        let mut ext = FlatBitSet::new(n, k);
        let mut nbh = FlatBitSet::new(n, k);
        let exc = FlatBitSet::new(n, k);

        // Insert the root into the subgraph
        sub.set(root, true);

        // Insert the roots neighbors into the extension
        ext.union_with(0, &bitgraph.neighbors(root));
        ext.set_range(0, 0..root+1, false);
        nbh.union_with(0, &bitgraph.neighbors(root));
        nbh.set(0, root, true);

        Self {
            bitgraph,
            sub,
            ext,
            nbh,
            exc,
            root,
            head,
            parent,
            n,
            k,
            depth,
        }
    }

    fn init_subgraph(root: usize, n: usize) -> FixedBitSet {
        let mut sub = FixedBitSet::with_capacity(n);
        sub.insert(root);
        sub
    }

    pub fn descend(&mut self) {
        self.depth += 1;

        // update the parent node
        self.parent[self.depth] = self.head;

        // draw a new head from the extension
        self.head = self.ext
            .get_ones_at_row(self.depth - 1)
            .next().unwrap();

        // insert the head into the subgraph
        self.sub.set(self.head, true);

        // create the new extension at the depth
        // then remove the head from the extension
        self.ext.union(self.depth, self.depth - 1);
        self.ext.set(self.depth, self.head, false);

        // create the new neighborhood at the depth
        self.nbh.union(self.depth, self.depth - 1);

        // create the new exclusive neighborhood at the depth
        self.exc.union_with(self.depth, self.bitgraph.neighbors(self.head));
        self.exc.difference_with_other(self.depth, &self.nbh, self.depth - 1);
        self.exc.set_range(self.depth, 0..self.root+1, false);
        
        // add the exclusive neighborhood to the extension and neighborhood
        self.ext.union_with_other(self.depth, &self.exc, self.depth);
        self.nbh.union_with_other(self.depth, &self.exc, self.depth);

        // println!(">> Depth: {}", self.depth);
        // println!("\n\n>> Descent to Depth: {}", self.depth);
        // println!("Sub:\n{:?}", self.sub.ones().collect::<Vec<_>>());
        // println!("Ext:\n{:?}", self.ext);
        // println!("Nbh:\n{:?}", self.nbh);
        // println!("Exc:\n{:?}", self.exc);
    }

    pub fn ascend(&mut self) {

        // remove the head from the subgraph
        self.sub.set(self.head, false);

        // remove the head from the extension a level above
        self.ext.set(self.depth - 1, self.head, false);

        // sets the head to the parent
        // println!("Setting head to parent: {} => {}", self.head, self.parent[self.depth]);
        self.head = self.parent[self.depth];
        
        // clear the extension at the depth
        self.ext.clear(self.depth);

        // clear the neighborhood at the depth
        self.nbh.clear(self.depth);
        
        // clear the exclusive neighborhood at the depth
        self.exc.clear(self.depth);

        // decrement the depth
        self.depth -= 1;

        // println!(">> Depth: {}", self.depth);
        // println!("\n\n>> Ascend to Depth: {}", self.depth);
        // println!("Sub:\n{:?}", self.sub.ones().collect::<Vec<_>>());
        // println!("Ext:\n{:?}", self.ext);
        // println!("Nbh:\n{:?}", self.nbh);
        // println!("Exc:\n{:?}", self.exc);
    }

    pub fn is_descending(&self) -> bool {
        self.depth < self.k - 1
    }

    pub fn has_extension(&self) -> bool {
        self.ext.get_ones_at_row(self.depth).count() > 0
    }

    /// monitors the initial extension to determine completeness
    pub fn is_finished(&self) -> bool {
        self.ext.get_ones_at_row(0).count() == 0
    }

    pub fn subgraph(&self) -> Vec<usize> {
        self.sub.ones().collect()
    }
}
