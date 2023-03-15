use crate::enumerate::BitGraph;


pub struct Esu {
    motif_size: usize,
    graph_size: usize,
    next: usize,
    current: Vec<usize>,
    pub ext: Vec<usize>,
    graph: BitGraph,
}
impl Esu {
    pub fn new(motif_size: usize, graph: BitGraph) -> Self {
        // let current = Vec::with_capacity(motif_size);
        // let ext = Vec::with_capacity(graph.n);
        let current = vec![0; motif_size];
        let ext = vec![0; graph.n];
        let next = 0;
        Self {
            motif_size,
            graph_size: graph.n,
            next,
            current,
            ext,
            graph,
        }
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

        // self.current.push(n);
        self.current[size] = n;
        let size = size + 1;

        if size == self.motif_size {
            *total += 1;
            // println!("{:?}", self.current);
            // self.current.remove(*size - 1);
            // unimplemented!();
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
}
