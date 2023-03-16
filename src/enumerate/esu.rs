use crate::enumerate::{BitGraph, EnumResult, NautyGraph};
use ahash::HashMap;
use petgraph::{Directed, Graph};

type Counts = HashMap<Vec<u64>, usize>;
type Memo = HashMap<Vec<u64>, Vec<u64>>;

pub struct Esu {
    motif_size: usize,
    current: Vec<usize>,
    graph: BitGraph,
    ngraph: NautyGraph,
    counts: Counts,
    memo: Memo,
    total: usize,
}
impl Esu {
    pub fn new(motif_size: usize, petgraph: &Graph<(), (), Directed>) -> Self {
        let graph = BitGraph::from_graph(petgraph);
        let current = vec![0; motif_size];
        let ngraph = NautyGraph::new_directed(motif_size);
        let counts = Counts::default();
        let memo = Memo::default();
        let total = 0;
        Self {
            motif_size,
            current,
            graph,
            ngraph,
            counts,
            memo,
            total,
        }
    }

    pub fn enumerate(&mut self) {
        let ext = vec![0; self.graph.n];
        (0..self.graph.n).for_each(|i| self.go(i, 0, 0, &ext));
    }

    pub fn build_nauty(&mut self) {
        self.current.iter().enumerate().for_each(|(i, &u)| {
            self.current.iter().enumerate().for_each(|(j, &v)| {
                if self.graph.is_connected_directed(u, v) {
                    self.ngraph.add_arc(i, j);
                }
            })
        });
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
    pub fn go(&mut self, n: usize, size: usize, next: usize, ext: &Vec<usize>) {
        self.current[size] = n;
        let size = size + 1;

        if size == self.motif_size {
            self.total += 1;
            self.build_nauty();

            // Check if the subgraph is isomorphic to a subgraph that has already been enumerated.
            let label = if let Some(label) = self.memo.get(self.ngraph.graph()) {
                label

            // Otherwise run nauty to find the canonical label of the subgraph.
            } else {
                self.run_nauty();
                let label = self.ngraph.canon();
                self.memo
                    .insert(self.ngraph.graph().to_vec(), label.to_vec());
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
            let mut ext2 = ext.clone();

            // Get the neighbors of the last node in the current subgraph
            let neighbors = self.graph.neighbors(self.current[size - 1]).ones();

            // Iterate over the neighbors of the last node in the current subgraph
            for v in neighbors {
                // If the neighbor is smaller than the first node in the current subgraph, skip it
                if v <= self.current[0] {
                    continue;
                }

                // Iterate over the nodes in the current subgraph
                // and if there are any neighbors, break
                let exclusive = self
                    .current
                    .iter()
                    .take(size - 1)
                    .all(|&u| !self.graph.is_connected(v, u));

                // If we are at the last node in the current subgraph, add the neighbor to the extension
                if exclusive {
                    ext2[next2] = v;
                    next2 += 1;
                }
            }

            // Recursively call the function for each node in the extension
            while next2 > 0 {
                next2 -= 1;
                self.go(ext2[next2], size, next2, &ext2);
            }
        }
    }

    pub fn result(self) -> EnumResult {
        EnumResult::new(self.counts, self.total)
    }
}

pub fn enumerate_subgraphs(petgraph: &Graph<(), (), Directed>, motif_size: usize) -> EnumResult {
    let mut esu = Esu::new(motif_size, petgraph);
    esu.enumerate();
    esu.result()
}

#[cfg(test)]
mod testing {

    use super::*;
    use crate::io::load_numeric_graph;

    #[test]
    fn example_s3() {
        let filepath = "example/example.txt";
        let graph = load_numeric_graph(filepath, false).unwrap();
        let result = enumerate_subgraphs(&graph, 3);
        assert_eq!(result.total_subgraphs(), 16);
        assert_eq!(result.unique_subgraphs(), 4);
    }

    #[test]
    fn example_s4() {
        let filepath = "example/example.txt";
        let graph = load_numeric_graph(filepath, false).unwrap();
        let result = enumerate_subgraphs(&graph, 4);
        assert_eq!(result.total_subgraphs(), 24);
        assert_eq!(result.unique_subgraphs(), 8);
    }

    #[test]
    fn yeast_s3() {
        let filepath = "example/yeast.txt";
        let graph = load_numeric_graph(filepath, false).unwrap();
        let result = enumerate_subgraphs(&graph, 3);
        assert_eq!(result.total_subgraphs(), 13150);
        assert_eq!(result.unique_subgraphs(), 7);
    }

    #[test]
    fn yeast_s4() {
        let filepath = "example/yeast.txt";
        let graph = load_numeric_graph(filepath, false).unwrap();
        let result = enumerate_subgraphs(&graph, 4);
        assert_eq!(result.total_subgraphs(), 183174);
        assert_eq!(result.unique_subgraphs(), 34);
    }
}
