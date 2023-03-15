use fixedbitset::FixedBitSet;
use petgraph::{graph::NodeIndex, EdgeType, Graph};

/// A graph represented as a set of adjacency lists
#[derive(Debug)]
pub struct BitGraph {
    /// Adjacency list for undirected (all) edges
    u_adj: Vec<FixedBitSet>,

    /// Adjacency list for directed (outgoing) edges
    d_adj: Vec<FixedBitSet>,

    /// Number of directed (outgoing) neighbors,
    d_num: Vec<usize>,

    /// Number of nodes in the graph
    pub n: usize,

    /// Is directed
    pub is_directed: bool,
}
impl BitGraph {
    pub fn from_graph<N, E, Ty: EdgeType>(graph: &Graph<N, E, Ty>) -> Self {
        let n = graph.node_count();
        let mut u_adj = Vec::with_capacity(n);
        let mut d_adj = Vec::with_capacity(n);
        let mut d_num = Vec::with_capacity(n);
        for v in 0..n {
            let v_index = NodeIndex::new(v);
            let u_neighbors = undirected_neighbors(graph, v_index);
            let d_neighbors = directed_neighbors(graph, v_index);
            u_adj.push(u_neighbors);
            d_num.push(d_neighbors.count_ones(..));
            d_adj.push(d_neighbors);
        }
        Self {
            u_adj,
            d_adj,
            d_num,
            n,
            is_directed: Ty::is_directed(),
        }
    }

    pub fn neighbors(&self, v: usize) -> &FixedBitSet {
        &self.u_adj[v]
    }

    #[allow(dead_code)]
    pub fn neighbors_directed(&self, v: usize) -> &FixedBitSet {
        &self.d_adj[v]
    }

    pub fn neighbors_directed_unchecked(&self, v: usize) -> &FixedBitSet {
        unsafe { self.d_adj.get_unchecked(v) }
    }

    pub fn num_neighbors_directed(&self, v: usize) -> usize {
        self.d_num[v]
    }

    pub fn is_connected(&self, u: usize, v: usize) -> bool {
        self.u_adj[u].contains(v)
    }

    pub fn is_connected_directed(&self, u: usize, v: usize) -> bool {
        self.d_adj[u].contains(v)
    }
}

fn undirected_neighbors<N, E, Ty: EdgeType>(graph: &Graph<N, E, Ty>, v: NodeIndex) -> FixedBitSet {
    let mut bv = FixedBitSet::with_capacity(graph.node_count());
    for neighbor in graph.neighbors_undirected(v) {
        bv.insert(neighbor.index());
    }
    bv
}

fn directed_neighbors<N, E, Ty: EdgeType>(graph: &Graph<N, E, Ty>, v: NodeIndex) -> FixedBitSet {
    let mut bv = FixedBitSet::with_capacity(graph.node_count());
    for neighbor in graph.neighbors(v) {
        bv.insert(neighbor.index());
    }
    bv
}

#[cfg(test)]
mod testing {

    use super::*;
    use petgraph::{Directed, Undirected};

    fn build_directed_graph() -> Graph<(), (), Directed> {
        let edges = [(0, 1), (0, 2), (0, 3), (1, 2), (2, 3)];
        Graph::from_edges(&edges)
    }

    fn build_undirected_graph() -> Graph<(), (), Undirected> {
        let edges = [(0, 1), (0, 2), (0, 3), (1, 2), (2, 3)];
        Graph::from_edges(&edges)
    }

    #[test]
    fn test_undirected_neighbors() {
        let graph = build_undirected_graph();
        let v = NodeIndex::new(1);
        let neighbors = undirected_neighbors(&graph, v);
        assert_eq!(neighbors.len(), 4);
        assert_eq!(neighbors.count_ones(..), 2);
        assert!(neighbors.contains(0));
        assert!(neighbors.contains(2));
    }

    #[test]
    fn test_directed_neighbors() {
        let graph = build_directed_graph();
        let v = NodeIndex::new(1);
        let neighbors = directed_neighbors(&graph, v);
        assert_eq!(neighbors.len(), 4);
        assert_eq!(neighbors.count_ones(..), 1);
        assert!(!neighbors.contains(0));
        assert!(neighbors.contains(2));
    }

    #[test]
    fn test_bitgraph_directed() {
        let graph = build_directed_graph();
        let bitgraph = BitGraph::from_graph(&graph);
        assert_eq!(bitgraph.n, 4);
        assert_eq!(bitgraph.u_adj.len(), 4);
        assert_eq!(bitgraph.d_adj.len(), 4);
        assert_ne!(bitgraph.neighbors(1), bitgraph.neighbors_directed(1));
    }

    #[test]
    fn test_bitgraph_undirected() {
        let graph = build_undirected_graph();
        let bitgraph = BitGraph::from_graph(&graph);
        assert_eq!(bitgraph.n, 4);
        assert_eq!(bitgraph.u_adj.len(), 4);
        assert_eq!(bitgraph.d_adj.len(), 4);
        assert_eq!(bitgraph.neighbors(1), bitgraph.neighbors_directed(1));
    }
}
