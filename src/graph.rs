// graph.rs
use std::collections::HashMap;

pub type Edge = (usize, usize);
pub type ListOfEdges = Vec<Edge>;

#[derive(Debug)]
pub struct Graph {
    pub n: usize,
    pub outedges: Vec<Vec<usize>>,
}

impl Graph {
    pub fn add_directed_edges(&mut self, edges: &ListOfEdges) {
        for (u, v) in edges {
            self.outedges[*u].push(*v);
        }
    }

    pub fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }

    pub fn create_directed(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Graph {
            n,
            outedges: vec![vec![]; n],
        };
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g
    }

    pub fn compute_degree_centrality(&self) -> HashMap<usize, usize> {
        let mut centrality = HashMap::new();
        for (node, edges) in self.outedges.iter().enumerate() {
            centrality.insert(node, edges.len());
        }
        centrality
    }
}
