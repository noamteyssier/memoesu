use petgraph::{Graph, Directed};
use rayon::prelude::*;
use crate::enumerate::{BitGraph, NautyGraph, EnumResult};

type Counts = ahash::HashMap<Vec<u64>, usize>;
type Memo = flurry::HashMap<Vec<u64>, Vec<u64>>;

pub struct ParEsu {
    motif_size: usize,
    graph: BitGraph,
    counts: Counts,
    memo: Memo,
    total: usize,
}
impl ParEsu {
    pub fn new(motif_size: usize, petgraph: &Graph<(), (), Directed>) -> Self {
        let graph = BitGraph::from_graph(petgraph);
        let counts = Counts::default();
        let memo = Memo::default();
        let total = 0;
        Self {
            motif_size,
            graph,
            counts,
            memo,
            total,
        }
    }

    pub fn enumerate(&mut self) {
        let ext = vec![0; self.graph.n];
        let (counts, total) = (0..self.graph.n)
            .par_bridge()
            .map(|i| {
                let mut ngraph = NautyGraph::new_directed(self.motif_size);
                let mut counts = Counts::default();
                let mut current = vec![0; self.motif_size];
                let mut total = 0;
                self.go(i, 0, 0, &ext, &mut ngraph, &mut counts, &mut current, &mut total);
                (counts, total)
            })
            .reduce(
                || (Counts::default(), 0),
                |(mut counts1, total1), (counts2, total2)| {
                    counts2.into_iter().for_each(|(k, v)| {
                        *counts1.entry(k).or_insert(0) += v;
                    });
                    (counts1, total1 + total2)
                });
        self.counts = counts;
        self.total = total;
    }

    pub fn build_nauty(&self, current: &[usize], ngraph: &mut NautyGraph) {
        current.iter().enumerate().for_each(|(i, &u)| {
            current.iter().enumerate().for_each(|(j, &v)| {
                if self.graph.is_connected_directed(u, v) {
                    ngraph.add_arc(i, j);
                }
            })
        });
    }

    pub fn run_nauty(&self, ngraph: &mut NautyGraph) {
        ngraph.run();
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
    pub fn go(
        &self, 
        n: usize, 
        size: usize, 
        next: usize, 
        ext: &Vec<usize>, 
        ngraph: &mut NautyGraph, 
        counts: &mut Counts,
        current: &mut Vec<usize>,
        total: &mut usize,
        ) {

        current[size] = n;
        let size = size + 1;

        if size == self.motif_size {
            *total += 1;
            self.build_nauty(current, ngraph);

            // Check if the subgraph is isomorphic to a subgraph that has already been enumerated.
            if let Some(label) = self.memo.get(ngraph.graph(), &self.memo.guard()) {
                if let Some(count) = counts.get_mut(label) {
                    *count += 1;
                } else {
                    counts.insert(label.to_vec(), 1);
                }

            // Otherwise run nauty to find the canonical label of the subgraph.
            } else {
                self.run_nauty(ngraph);
                let label = ngraph.canon();
                self.memo.insert(ngraph.graph().to_vec(), label.to_vec(), &self.memo.guard());
                counts.insert(label.to_vec(), 1);
            };

            ngraph.clear_canon();
            ngraph.clear_graph();
        } else {
            let mut next2 = next;

            // Copy the list of nodes in the extension.
            let mut ext2 = ext.clone();

            // Get the neighbors of the last node in the current subgraph
            let neighbors = self.graph.neighbors(current[size - 1]).ones();

            // Iterate over the neighbors of the last node in the current subgraph
            for v in neighbors {

                // If the neighbor is smaller than the first node in the current subgraph, skip it
                if v <= current[0] {
                    continue;
                }
                
                // Iterate over the nodes in the current subgraph
                // and if there are any neighbors, break
                let exclusive = current
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
                self.go(ext2[next2], size, next2, &ext2, ngraph, counts, current, total);
            }
        }
    }

    pub fn result(self) -> EnumResult {
        EnumResult::new(self.counts, self.total)
    }
}

pub fn parallel_enumerate_subgraphs(graph: &Graph<(), (), Directed>, motif_size: usize) -> EnumResult {
    let mut esu = ParEsu::new(motif_size, graph);
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
        let result = parallel_enumerate_subgraphs(&graph, 3);
        assert_eq!(result.total_subgraphs(), 16);
        assert_eq!(result.unique_subgraphs(), 4);
    }

    #[test]
    fn example_s4() {
        let filepath = "example/example.txt";
        let graph = load_numeric_graph(filepath, false).unwrap();
        let result = parallel_enumerate_subgraphs(&graph, 4);
        assert_eq!(result.total_subgraphs(), 24);
        assert_eq!(result.unique_subgraphs(), 8);
    }

    #[test]
    fn yeast_s3() {
        let filepath = "example/yeast.txt";
        let graph = load_numeric_graph(filepath, false).unwrap();
        let result = parallel_enumerate_subgraphs(&graph, 3);
        assert_eq!(result.total_subgraphs(), 13150);
        assert_eq!(result.unique_subgraphs(), 7);
    }

    #[test]
    fn yeast_s4() {
        let filepath = "example/yeast.txt";
        let graph = load_numeric_graph(filepath, false).unwrap();
        let result = parallel_enumerate_subgraphs(&graph, 4);
        assert_eq!(result.total_subgraphs(), 183174);
        assert_eq!(result.unique_subgraphs(), 34);
    }
}
