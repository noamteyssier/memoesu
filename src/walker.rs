use std::fmt::Debug;

use crate::{bitgraph::BitGraph, ngraph::NautyGraph};
use multibitset::MultiBitSet;

#[derive(Debug)]
pub struct Walker<'a> {
    /// A reference to the underlying bitgraph.
    bitgraph: &'a BitGraph,

    /// The subgraph of the current walk.
    subgraph: Vec<usize>,

    /// The extension of the current walk.
    extension: MultiBitSet,

    /// The neighborhood of the current walk.
    neighborhood: MultiBitSet,

    /// The exclusive neighborhood of the current walk.
    exclusive: MultiBitSet,

    /// The root of the walk.
    root: usize,

    /// The current head of the walk.
    head: usize,

    /// The parent nodes of the current head.
    parent: Vec<usize>,

    /// The maximum size of the subgraph.
    k: usize,

    /// The current depth of the walk.
    pub depth: usize,

    /// The underlying nauty graph.
    nauty_graph: NautyGraph,
}
impl<'a> Walker<'a> {
    pub fn new(bitgraph: &'a BitGraph, root: usize, k: usize) -> Self {
        // Initialize the number of nodes.
        let n = bitgraph.n;

        // Initialize the head node as root.
        let head = root;

        // Initialize the parent vector.
        let mut parent = vec![0; k];
        parent[0] = root;

        // Initialize the depth.
        let depth = 0;

        // Initialize the underlying nauty graph.
        let nauty_graph = if bitgraph.is_directed {
            NautyGraph::new_directed(k)
        } else {
            NautyGraph::new_undirected(k)
        };

        // Initialize the subgraph, extension, and neighborhood.
        let subgraph = Self::init_subgraph(root, n);
        let mut extension = MultiBitSet::new(n, k);
        let mut neighborhood = MultiBitSet::new(n, k);
        let exclusive = MultiBitSet::new(n, k);

        // Insert the roots neighbors into the extension
        extension.inplace_external_union(0, bitgraph.neighbors(root));
        extension.set_range(0, 0..root+1, false);

        neighborhood.inplace_external_union(0, bitgraph.neighbors(root));
        neighborhood.set(0, root, true);

        Self {
            bitgraph,
            subgraph,
            extension,
            neighborhood,
            exclusive,
            root,
            head,
            parent,
            k,
            depth,
            nauty_graph,
        }
    }

    fn init_subgraph(root: usize, n: usize) -> Vec<usize> {
        let mut subgraph = Vec::with_capacity(n);
        subgraph.push(root);
        subgraph
    }

    pub fn descend(&mut self) {
        self.depth += 1;

        // update the parent node
        self.parent[self.depth] = self.head;

        // draw a new head from the extension
        self.head = self
            .extension
            .get_row(self.depth - 1)
            .ones()
            .next()
            .unwrap();

        // insert the head into the subgraph
        self.subgraph.push(self.head);

        // create the new extension at the depth
        // then remove the head from the extension
        self.extension.inplace_union(self.depth, self.depth - 1);
        self.extension.set(self.depth, self.head, false);

        // create the new neighborhood at the depth
        self.neighborhood.inplace_union(self.depth, self.depth - 1);

        // create the new exclusive neighborhood at the depth
        self.exclusive
            .inplace_external_union(self.depth, self.bitgraph.neighbors(self.head));
        self.exclusive
            .difference_with(&self.neighborhood, self.depth, self.depth - 1);
        self.exclusive.set_range(self.depth, 0..self.root+1, false);

        // add the exclusive neighborhood to the extension and neighborhood
        self.extension
            .union_with(&self.exclusive, self.depth, self.depth);
        self.neighborhood
            .union_with(&self.exclusive, self.depth, self.depth);

        // self.debug(true);
    }

    pub fn ascend(&mut self) {
        // remove the head from the subgraph
        self.subgraph.remove(self.depth);

        // remove the head from the extension a level above
        self.extension.set(self.depth - 1, self.head, false);

        // sets the head to the parent
        self.head = self.parent[self.depth];

        // clear the extension at the depth
        self.extension.clear(self.depth);

        // clear the neighborhood at the depth
        self.neighborhood.clear(self.depth);

        // clear the exclusive neighborhood at the depth
        self.exclusive.clear(self.depth);

        // decrement the depth
        self.depth -= 1;

        // self.debug(false);
    }

    #[allow(dead_code)]
    fn debug(&self, descend: bool) {
        if descend {
            println!("\n\n>> Descent to Depth: {}", self.depth);
        } else {
            println!("\n\n>> Ascend to Depth: {}", self.depth);
        }
        println!("Sub:\n{:?}", self.subgraph);
        println!("Ext:\n{}", self.extension.pprint());
        println!("Nbh:\n{}", self.neighborhood.pprint());
        println!("Exc:\n{}", self.exclusive.pprint());
    }

    #[allow(dead_code)]
    pub fn debug_subgraph(&self) {
        println!("Subgraph: {:?}", self.subgraph());
        println!("NAUTY   : {:?}", self.nauty_graph.pprint_graph());
    }

    pub fn is_descending(&self) -> bool {
        self.depth < self.k - 1
    }

    /// checks if the current depth has an extension
    pub fn has_extension(&self) -> bool {
        self.extension.get_row(self.depth).ones().next().is_some()
    }

    /// monitors the initial extension to determine completeness
    pub fn is_finished(&self) -> bool {
        self.extension.get_row(0).ones().next().is_none()
    }

    pub fn subgraph(&self) -> &[usize] {
        &self.subgraph
    }

    pub fn nauty_graph(&self) -> &[u64] {
        self.nauty_graph.graph()
    }

    pub fn fill_nauty(&mut self) {
        // Fill the nauty graph with the subgraph
        for i in 0..=self.depth {
            for j in 0..=self.depth {
                if self
                    .bitgraph
                    .neighbors_directed(self.subgraph[i])
                    .contains(self.subgraph[j])
                {
                    self.nauty_graph.add_arc(i, j);
                }
            }
        }
    }

    pub fn run_nauty(&mut self) -> Vec<u64> {
        self.nauty_graph.run();
        self.nauty_graph.canon().to_owned()
    }

    pub fn clear_nauty(&mut self) {
        self.nauty_graph.clear_canon();
        self.nauty_graph.clear_graph();
    }
}
