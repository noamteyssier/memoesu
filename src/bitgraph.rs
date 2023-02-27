use fixedbitset::FixedBitSet;
use petgraph::{EdgeType, Graph, graph::NodeIndex};


/// A graph represented as a set of adjacency lists
#[derive(Debug)]
pub struct BitGraph {
    /// Adjacency list for undirected (all) edges
    u_adj: Vec<FixedBitSet>,

    /// Adjacency list for directed (outgoing) edges
    d_adj: Vec<FixedBitSet>,

    /// Number of nodes in the graph
    pub n: usize,
}
impl BitGraph {
    pub fn from_graph<N, E, Ty: EdgeType>(graph: &Graph<N, E, Ty>) -> Self {
        let n = graph.node_count();
        let mut u_adj = Vec::with_capacity(n);
        let mut d_adj = Vec::with_capacity(n);
        for v in 0..n {
            let v_index = NodeIndex::new(v);
            u_adj.push(undirected_neighbors(graph, v_index));
            d_adj.push(directed_neighbors(graph, v_index));
        }
        Self { u_adj, d_adj, n }
    }

    pub fn neighbors(&self, v: usize) -> &FixedBitSet {
        &self.u_adj[v]
    }

    pub fn neighbors_directed(&self, v: usize) -> &FixedBitSet {
        &self.d_adj[v]
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

    use petgraph::{Directed, Undirected};
    use super::*;

    fn build_directed_graph() -> Graph<(), (), Directed> {
        let edges = [
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 2),
            (2, 3),
        ];
        Graph::from_edges(&edges)
    }

    fn build_undirected_graph() -> Graph<(), (), Undirected> {
        let edges = [
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 2),
            (2, 3),
        ];
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
