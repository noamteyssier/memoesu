use std::marker::PhantomData;

use crate::enumerate::{BitGraph, EnumResult, NautyGraph};
use ahash::HashMap;
use petgraph::{EdgeType, Graph};

type Counts = HashMap<Vec<u64>, usize>;
type Memo = HashMap<Vec<u64>, Vec<u64>>;

pub struct Esu<Ty: EdgeType> {
    motif_size: usize,
    current: Vec<usize>,
    graph: BitGraph,
    ngraph: NautyGraph,
    counts: Counts,
    memo: Memo,
    total: usize,
    is_directed: bool,
    phantom: PhantomData<Ty>,
}

impl<Ty: EdgeType> Esu<Ty> {
    pub fn new(motif_size: usize, petgraph: &Graph<(), (), Ty>) -> Self {
        let is_directed = petgraph.is_directed();
        let graph = BitGraph::from_graph(petgraph);
        let current = vec![0; motif_size];
        let ngraph = NautyGraph::new(motif_size, is_directed);
        let counts = Counts::default();
        let memo = Memo::default();
        let total = 0;
        let phantom = PhantomData;
        Self {
            motif_size,
            current,
            graph,
            ngraph,
            counts,
            memo,
            total,
            is_directed,
            phantom,
        }
    }

    pub fn enumerate(&mut self) {
        let ext = vec![0; self.graph.n];
        (0..self.graph.n).for_each(|i| self.go(i, 0, 0, &ext));
    }

    pub fn build_nauty(&mut self) {
        if self.is_directed {
            self.build_nauty_dir();
        } else {
            self.build_nauty_undir();
        }
    }

    pub fn build_nauty_dir(&mut self) {
        self.current.iter().enumerate().for_each(|(i, &u)| {
            self.current.iter().enumerate().for_each(|(j, &v)| {
                if self.graph.is_connected_directed(u, v) {
                    self.ngraph.add_arc(i, j);
                }
            })
        });
    }

    pub fn build_nauty_undir(&mut self) {
        self.current.iter().enumerate().for_each(|(i, &u)| {
            self.current
                .iter()
                .enumerate()
                .skip(i + 1)
                .for_each(|(j, &v)| {
                    if self.graph.is_connected(u, v) {
                        self.ngraph.add_arc(i, j);
                        self.ngraph.add_arc(j, i);
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

pub fn enumerate_subgraphs<Ty: EdgeType>(
    petgraph: &Graph<(), (), Ty>,
    motif_size: usize,
) -> EnumResult {
    let mut esu = Esu::new(motif_size, petgraph);
    esu.enumerate();
    esu.result()
}

#[cfg(test)]
mod testing {

    use super::*;
    use crate::io::load_numeric_graph;
    use petgraph::{Directed, Undirected};

    #[test]
    fn dir_example_s3() {
        let filepath = "example/example.txt";
        let graph = load_numeric_graph::<Directed>(filepath, false).unwrap();
        let result = enumerate_subgraphs(&graph, 3);
        assert_eq!(result.total_subgraphs(), 16);
        assert_eq!(result.unique_subgraphs(), 4);

        // &BP_    1
        // &BC_    3
        // &B?o    3
        // &BCO    9
        result.counts().values().for_each(|&count| {
            let cond = count == 1 || count == 3 || count == 9;
            assert!(cond);
        });
    }

    #[test]
    fn undir_example_s3() {
        let filepath = "example/example.txt";
        let graph = load_numeric_graph::<Undirected>(filepath, false).unwrap();
        let result = enumerate_subgraphs(&graph, 3);
        assert_eq!(result.total_subgraphs(), 16);
        assert_eq!(result.unique_subgraphs(), 2);

        // Bw      1
        // BW      15
        result.counts().values().for_each(|&count| {
            let cond = count == 1 || count == 15;
            assert!(cond);
        });
    }

    #[test]
    fn dir_example_s4() {
        let filepath = "example/example.txt";
        let graph = load_numeric_graph::<Directed>(filepath, false).unwrap();
        let result = enumerate_subgraphs(&graph, 4);
        assert_eq!(result.total_subgraphs(), 24);
        assert_eq!(result.unique_subgraphs(), 8);

        // &C?g_   3
        // &COg_   3
        // &C?gO   3
        // &C?`o   3
        // &C?gG   3
        // &C?@o   3
        // &C?Cg   3
        // &C?Go   3
        result.counts().values().for_each(|&count| {
            let cond = count == 3;
            assert!(cond);
        });
    }

    #[test]
    fn undir_example_s4() {
        let filepath = "example/example.txt";
        let graph = load_numeric_graph::<Undirected>(filepath, false).unwrap();
        let result = enumerate_subgraphs(&graph, 4);
        assert_eq!(result.total_subgraphs(), 24);
        assert_eq!(result.unique_subgraphs(), 3);

        // CN      6
        // CF      6
        // CR      12
        result.counts().values().for_each(|&count| {
            let cond = count == 6
                || count == 12;
            assert!(cond);
        });
    }

    #[test]
    fn dir_yeast_s3() {
        let filepath = "example/yeast.txt";
        let graph = load_numeric_graph::<Directed>(filepath, false).unwrap();
        let result = enumerate_subgraphs(&graph, 3);
        assert_eq!(result.total_subgraphs(), 13150);
        assert_eq!(result.unique_subgraphs(), 7);

        // &BPo    1
        // &BSo    1
        // &B@o    18
        // &BCo    70
        // &BCO    293
        // &BC_    889
        // &B?o    11878
        result.counts().values().for_each(|&count| {
            let cond = count == 1
                || count == 18
                || count == 70
                || count == 293
                || count == 889
                || count == 11878;
            assert!(cond);
        });
    }

    #[test]
    fn undir_yeast_s3() {
        let filepath = "example/yeast.txt";
        let graph = load_numeric_graph::<Undirected>(filepath, false).unwrap();
        let result = enumerate_subgraphs(&graph, 3);
        assert_eq!(result.total_subgraphs(), 13150);
        assert_eq!(result.unique_subgraphs(), 2);

        // Bw      72
        // BW      13078
        result.counts().values().for_each(|&count| {
            let cond = count == 72
                || count == 13078;
            assert!(cond);
        });
    }

    #[test]
    fn dir_yeast_s4() {
        let filepath = "example/yeast.txt";
        let graph = load_numeric_graph::<Directed>(filepath, false).unwrap();
        let result = enumerate_subgraphs(&graph, 4);
        assert_eq!(result.total_subgraphs(), 183174);
        assert_eq!(result.unique_subgraphs(), 34);

        // &CAKw   1
        // &C?do   1
        // &C?`w   1
        // &COkw   1
        // &C?kW   1
        // &C?[w   1
        // &CAKg   1
        // &CAGw   3
        // &CAGW   4
        // &CAKo   6
        // &C?cw   9
        // &C?go   10
        // &CA@o   11
        // &C?hW   16
        // &C?Wo   16
        // &CACw   17
        // &C?kw   17
        // &CACo   17
        // &C?HW   32
        // &CACW   55
        // &C?gO   92
        // &CAGo   102
        // &C?@w   121
        // &C?gG   125
        // &C?Kw   157
        // &C?Kg   286
        // &C?g_   400
        // &CAG_   989
        // &C?Gw   1125
        // &C?@o   1460
        // &C?Ko   1843
        // &C?Cg   4498
        // &C?Go   22995
        // &C??w   148761
        result.counts().values().for_each(|&c| {
            let cond = c == 1
                || c == 3
                || c == 4
                || c == 6
                || c == 9
                || c == 10
                || c == 11
                || c == 16
                || c == 17
                || c == 32
                || c == 55
                || c == 92
                || c == 102
                || c == 121
                || c == 125
                || c == 157
                || c == 286
                || c == 400
                || c == 989
                || c == 1125
                || c == 1460
                || c == 1843
                || c == 4498
                || c == 22995
                || c == 148761;
            assert!(cond);
        })
    }

    #[test]
    fn undir_yeast_s4() {
        let filepath = "example/yeast.txt";
        let graph = load_numeric_graph::<Undirected>(filepath, false).unwrap();
        let result = enumerate_subgraphs(&graph, 4);
        assert_eq!(result.total_subgraphs(), 183174);
        assert_eq!(result.unique_subgraphs(), 6);

        // C~      2
        // C^      202
        // CN      1624
        // Cr      1857
        // CR      28033
        // CF      151456
        result.counts().values().for_each(|&c| {
            let cond = c == 2
                || c == 202
                || c == 1624
                || c == 1857
                || c == 28033
                || c == 151456;
            assert!(cond);
        })
    }
}
