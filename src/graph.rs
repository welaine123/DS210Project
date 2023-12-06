// graph.rs

// use crate::graph::ListOfEdges;



#[derive(Debug)]
pub struct Graph {
    n: usize,
    outedges: Vec<Vec<usize>>,
}

pub type ListOfEdges = Vec<(usize, usize)>;

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

    pub fn create_undirected(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Self::create_directed(n, edges);
        g.add_directed_edges(&reverse_edges(edges));
        g.sort_graph_lists();
        g
    }
}

fn reverse_edges(edges: &ListOfEdges) -> ListOfEdges {
    edges.iter().map(|&(u, v)| (v, u)).collect()
}
