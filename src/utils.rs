use hashbrown::HashSet;
use petgraph::{graph::NodeIndex, EdgeType, Graph};

pub fn pop_extension(ext: &mut HashSet<NodeIndex>) -> NodeIndex {
    let w = ext.iter().next().unwrap().clone();
    ext.remove(&w);
    w
}

pub fn append_subgraph(subgraph: &HashSet<NodeIndex>, w: &NodeIndex) -> HashSet<NodeIndex> {
    let mut new_sg = subgraph.clone();
    new_sg.insert(*w);
    new_sg
}

pub fn append_exclusive(cnh: &HashSet<NodeIndex>, exc: &HashSet<NodeIndex>) -> HashSet<NodeIndex> {
    cnh.union(exc).cloned().collect::<HashSet<NodeIndex>>()
}

pub fn initial_extension<N, E, Ty>(graph: &Graph<N, E, Ty>, v: &NodeIndex) -> HashSet<NodeIndex>
where
    Ty: EdgeType,
{
    graph.neighbors_undirected(*v).filter(|w| w > v).collect()
}

pub fn initial_neighborhood(ext: &HashSet<NodeIndex>, v: &NodeIndex) -> HashSet<NodeIndex> {
    let mut cnh = ext.clone();
    cnh.insert(*v);
    cnh
}

pub fn exclusive_neighborhood<N, E, Ty>(
    graph: &Graph<N, E, Ty>,
    cn: &HashSet<NodeIndex>,
    w: &NodeIndex,
) -> HashSet<NodeIndex>
where
    Ty: EdgeType,
{
    graph
        .neighbors_undirected(*w)
        .filter(|n| !cn.contains(n))
        .collect()
}

pub fn overwrite_extension(
    exc: &HashSet<NodeIndex>,
    ext: &HashSet<NodeIndex>,
    v: &NodeIndex,
    w: &NodeIndex,
) -> HashSet<NodeIndex> {
    ext.union(exc)
        .filter(|u| *u > v)
        .filter(|u| *u != w)
        .cloned()
        .collect::<HashSet<NodeIndex>>()
}

#[cfg(test)]
mod testing {

    #[test]
    fn test_initial_extension() {
        let edges = vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
        let graph = super::Graph::<(), ()>::from_edges(&edges);
        let u = super::NodeIndex::new(0);
        let v = super::NodeIndex::new(1);
        let w = super::NodeIndex::new(2);
        let y = super::NodeIndex::new(3);
        let ext_u = super::initial_extension(&graph, &u);
        let ext_v = super::initial_extension(&graph, &v);
        let ext_w = super::initial_extension(&graph, &w);
        let ext_y = super::initial_extension(&graph, &y);
        assert_eq!(ext_u.len(), 3);
        assert_eq!(ext_v.len(), 2);
        assert_eq!(ext_w.len(), 1);
        assert_eq!(ext_y.len(), 0);
    }

    #[test]
    fn test_exclusive_neighborhood() {
        let edges = vec![
            (0, 1),
            (1, 2),
            (2, 0),
            (3, 0),
            (0, 4),
            (5, 1),
            (1, 6),
            (7, 2),
            (2, 8),
        ];
        let graph = super::Graph::<(), ()>::from_edges(&edges);
        let u = super::NodeIndex::new(0);
        let v = super::NodeIndex::new(1);
        let ext = super::initial_extension(&graph, &u);
        let cnh = super::initial_neighborhood(&ext, &u);
        let exc = super::exclusive_neighborhood(&graph, &cnh, &v);

        assert_eq!(exc.len(), 2);
        for n in exc {
            assert!(!cnh.contains(&n));
        }
    }
}
