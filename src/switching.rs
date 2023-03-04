use hashbrown::HashMap;
use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    Directed, Graph,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

// type NodeMap = HashMap<NodeIndex, HashMap<NodeIndex, bool>>;
struct NodeMap {
    map: HashMap<NodeIndex, HashMap<NodeIndex, bool>>,
}
impl NodeMap {
    fn contains_edge(&self, x: NodeIndex, y: NodeIndex) -> bool {
        if let Some(xmap) = self.map.get(&x) {
            xmap.contains_key(&y)
        } else {
            false
        }
    }

    fn remove_edge(&mut self, x: NodeIndex, y: NodeIndex) {
        if let Some(xmap) = self.map.get_mut(&x) {
            xmap.remove(&y);
        }
    }
}

/// Creates a random graph that preserves node degrees using the switch model.
///
/// This is done by randomly selecting two edges and switching their endpoints.
/// The switch is not performed if the resulting graph would have a self-loop
/// or if duplicate edges are created.
///
/// More about this model can be found in:
/// 1. On the uniform generation of random graphs with prescribed degree sequences, https://arxiv.org/abs/cond-mat/0312028
/// 2. Kavosh: a new algorithm for finding network motifs, https://bmcbioinformatics.biomedcentral.com/articles/10.1186/1471-2105-10-318
///
/// # Arguments
/// * `graph` - The graph to create a random graph from.
/// * `q` - The number of operations to perform (total = q * num_edges).
/// * `seed` - The seed for the random number generator.
pub fn switching(graph: &Graph<(), (), Directed>, q: usize, seed: u8) -> Graph<(), (), Directed> {
    let mut rgraph = graph.clone();
    let mut node_map = build_map(&rgraph);
    let mut rng = ChaChaRng::from_seed([seed; 32]);
    let mut num_switches = 0;
    let num_operations = graph.edge_count() * q;

    while num_switches < num_operations {
        // Pick two random edges.
        let (idx, jdx) = sample_edges(&mut rng, graph.edge_count());

        // Get the nodes of the edges.
        let (x1, x2) = rgraph.edge_endpoints(idx).unwrap();
        let (y1, y2) = rgraph.edge_endpoints(jdx).unwrap();

        // Check if the switch is valid and continue if not.
        if is_invalid_switch(&node_map, x1, x2, y1, y2) {
            continue;
        }

        // Perform the switch.
        perform_switch(&mut rgraph, &mut node_map, idx, jdx, x1, x2, y1, y2);
        num_switches += 1;
    }
    rgraph
}

fn build_map(graph: &Graph<(), (), Directed>) -> NodeMap {
    let mut map = HashMap::with_capacity(graph.node_count());
    for edge in graph.edge_indices() {
        let (x, y) = graph.edge_endpoints(edge).unwrap();
        let xmap = map.entry(x).or_insert_with(HashMap::new);
        xmap.insert(y, true);
    }
    NodeMap { map }
}

// Check if there already exists an edge from x1 => y2 or from y1 => x2.
// Check if this switch would create a loop.
// If so, we cannot perform the switch.
fn is_invalid_switch(
    node_map: &NodeMap,
    x1: NodeIndex,
    x2: NodeIndex,
    y1: NodeIndex,
    y2: NodeIndex,
) -> bool {
    would_duplicate(node_map, x1, x2, y1, y2) || would_loop(x1, x2, y1, y2)
}

/// Checks if the switch would create duplicate edges.
fn would_duplicate(
    node_map: &NodeMap,
    x1: NodeIndex,
    x2: NodeIndex,
    y1: NodeIndex,
    y2: NodeIndex,
) -> bool {
    node_map.contains_edge(x1, y2) || node_map.contains_edge(y1, x2)
}

/// Checks if the switch would create a loop.
fn would_loop(x1: NodeIndex, x2: NodeIndex, y1: NodeIndex, y2: NodeIndex) -> bool {
    x1 == y2 || y1 == x2 || x2 == y2
}

/// Samples two distinct random edges from a graph.
fn sample_edges<R: Rng>(rng: &mut R, num_edges: usize) -> (EdgeIndex, EdgeIndex) {
    loop {
        let u = rng.gen_range(0..num_edges);
        let v = rng.gen_range(0..num_edges);
        if u != v {
            return (EdgeIndex::new(u), EdgeIndex::new(v));
        }
    }
}

/// Performs a switch on a graph.
///
/// This is done by removing two edges and adding two new edges.
/// The two edges are removed in the reverse order of their indices
/// to avoid invalidating the indices.
fn perform_switch(
    graph: &mut Graph<(), (), Directed>,
    node_map: &mut NodeMap,
    idx: EdgeIndex,
    jdx: EdgeIndex,
    x1: NodeIndex,
    x2: NodeIndex,
    y1: NodeIndex,
    y2: NodeIndex,
) {
    // Remove the two edges in the reverse order of their indices.
    if idx < jdx {
        graph.remove_edge(jdx);
        graph.remove_edge(idx);
    } else {
        graph.remove_edge(idx);
        graph.remove_edge(jdx);
    }

    // Remove the two edges from the node map.
    node_map.remove_edge(x1, x2);
    node_map.remove_edge(y1, y2);

    // Add the two new edges.
    graph.add_edge(x1, y2, ());
    graph.add_edge(y1, x2, ());
}
