//! `Graph Processing` is a pure rust implementation of graph processing and layout algorithms
//
// inspired by  <https://github.com/nrc/r4cppp/blob/master/graphs/>
//
// References: <https://stackoverflow.com/a/2157012/3342767>

use std::cell::RefCell;
use std::rc::Rc;

/// `Graph` is a node-centric representation of a graph
///
/// It contains all nodes in a graph
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Graph {
    /// `nodes` is a list of nodes in the graph
    nodes: Vec<Rc<RefCell<Node>>>,
}

/// `Edge` represents an edge of a graph. It may be directed or undirected.
///
/// Even though the two nodes in the graph are labeled `source`/`destination`, as long as the
/// `directed` field is set false, they will be treated identically.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Edge {
    /// implementation specific ID
    pub id: String,
    /// destination node of edge
    pub destination: Rc<RefCell<Node>>,
    /// if graph is directed or undirected
    pub directed: bool,
    /// `weight` of edge. Optional
    pub weight: Option<u64>,
}

/// `Node` is a node/vertex in a graph
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Node {
    /// `id` implementation specific ID
    pub id: String,
    /// X position
    pub x: Option<u64>,
    /// Y position
    pub y: Option<u64>,
    /// Z position
    pub z: Option<u64>,
    /// list of associated edges
    pub edges: Vec<Edge>,
}

impl Graph {
    /// create an empty graph
    pub fn init() -> Self {
        Self { nodes: Vec::new() }
    }
    /// insert a node into a graph
    pub fn insert_node(&mut self, node: Rc<RefCell<Node>>) {
        self.nodes.push(node);
    }

    pub fn depth_first_search(&self) {
        // the entries in visited_list are the indexes of nodes in self.nodes
        let mut visited_list = Vec::new();
        if let Some(node) = self.nodes.first() {
            self.depth_first_search_inner(node.clone(), &mut visited_list)
        }
    }

    fn depth_first_search_inner(
        &self,
        node: Rc<RefCell<Node>>,
        visited_list: &mut Vec<Rc<RefCell<Node>>>,
    ) {
        visited_list.push(node.clone());
        for edge in &node.borrow().edges {
            if !visited_list.contains(&edge.destination) {
                self.depth_first_search_inner(edge.destination.clone(), visited_list);
            }
        }
    }
}
impl Node {
    /// create an empty node
    pub fn new() -> Self {
        Self {
            id: String::new(),
            x: None,
            y: None,
            z: None,
            edges: Vec::new(),
        }
    }

    /// add an edge to a node
    pub fn add_edge(
        &mut self,
        node: Rc<RefCell<Node>>,
        id: &str,
        directed: bool,
        weight: Option<u64>,
    ) {
        self.edges.push(Edge {
            id: id.to_string(),
            destination: node,
            directed: directed,
            weight: weight,
        })
    }
    /// outputs the degree of the node
    pub fn degree(&self) -> usize {
        self.edges.len()
    }
}
