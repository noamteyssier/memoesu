use hashbrown::HashMap;
use crate::enumerate::{BitGraph, NautyGraph};

pub struct Esu {
    motif_size: usize,
    current: Vec<usize>,
    pub ext: Vec<usize>,
    graph: BitGraph,
    ngraph: NautyGraph,
    counts: HashMap<Vec<u64>, usize>,
    memo: HashMap<Vec<u64>, Vec<u64>>,
}
impl Esu {
    pub fn new(motif_size: usize, graph: BitGraph) -> Self {
        let current = vec![0; motif_size];
        let ext = vec![0; graph.n];
        let ngraph = NautyGraph::new_directed(motif_size);
        let counts = HashMap::with_capacity(graph.n * motif_size);
        let memo = HashMap::with_capacity(graph.n * motif_size);
        Self {
            motif_size,
            current,
            ext,
            graph,
            ngraph,
            counts,
            memo,
        }
    }

    pub fn build_nauty(&mut self) {
        for i in 0..self.motif_size {
            for j in 0..self.motif_size {
                if i == j {
                    continue;
                }
                if self.graph.is_connected_directed(self.current[i], self.current[j]) {
                    self.ngraph.add_arc(i, j);
                }
            }
        }
    }

    pub fn run_nauty(&mut self) {
        self.ngraph.run();
    }

    /// The main function for the enumeration.
    ///
    /// This function is called recursively to enumerate all subgraphs of the
    /// given size.
    ///
    /// # Arguments
    /// * `n` - The current node.
    /// * `size` - The current size of the subgraph.
    /// * `next` - The next node to be added to the subgraph.
    /// * `ext` - The extension of the subgraph.
    pub fn go(&mut self, n: usize, size: usize, next: usize, ext: &Vec<usize>, total: &mut usize) {

        self.current[size] = n;
        let size = size + 1;

        if size == self.motif_size {
            *total += 1;
            self.build_nauty();

            // Check if the subgraph is isomorphic to a subgraph that has already been enumerated.
            let label = if let Some(label) = self.memo.get(self.ngraph.graph()) {
                label

            // Otherwise run nauty to find the canonical label of the subgraph.
            } else {
                self.run_nauty();
                let label = self.ngraph.canon();
                self.memo.insert(self.ngraph.graph().to_vec(), label.to_vec());
                self.ngraph.clear_canon();
                self.memo.get(self.ngraph.graph()).unwrap()
            };

            // Increment the count of the subgraph with the given label.
            if let Some(count) = self.counts.get_mut(label) {
                *count += 1;
            } else {
                self.counts.insert(label.to_vec(), 1);
            }

            self.ngraph.clear_graph();
        } else {
            let mut next2 = next;

            // Copy the list of nodes in the extension.
            // let mut ext2 = ext.clone();
            let mut ext2 = vec![0; ext.len()];
            for i in 0..next {
                ext2[i] = ext[i];
            }

            // Get the neighbors of the last node in the current subgraph
            let neighbors = self.graph.neighbors(self.current[size - 1]).ones();

            // Iterate over the neighbors of the last node in the current subgraph
            for v in neighbors {

                // If the neighbor is smaller than the first node in the current subgraph, skip it
                if v <= self.current[0] {
                    continue;
                }
                
                // If the neighbor is connected to any of the nodes in the current subgraph, break
                let mut j = 0;
                while j < size - 1 {
                    if self.graph.is_connected(v, self.current[j]) {
                        break;
                    }
                    j += 1;
                }

                // If we are at the last node in the current subgraph, add the neighbor to the extension
                if j == size - 1 {
                    ext2[next2] = v;
                    next2 += 1;
                }
            }

            // Recursively call the function for each node in the extension
            while next2 > 0 {
                next2 -= 1;
                self.go(ext2[next2], size, next2, &ext2, total);
            }
        }
    }

    pub fn counts(&self) -> &HashMap<Vec<u64>, usize> {
        &self.counts
    }
}
