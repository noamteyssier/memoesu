mod bitgraph;
mod esu;
mod ngraph;
mod walker;

use std::io::BufRead;
use petgraph::{Graph, Directed};


fn load_graph(filepath: &str) -> Graph<(), (), Directed> {
    let file = std::fs::File::open(filepath).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut edges = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let u = split.next().unwrap().parse::<u32>().unwrap();
        let v = split.next().unwrap().parse::<u32>().unwrap();
        edges.push((u, v));
    }
    Graph::from_edges(&edges)
}

fn main() {
    let k = 5;
    let graph = load_graph("example/yeast.txt");
    esu::enumerate_subgraphs(&graph, k);
}
