pub mod edge;

use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use itertools::Itertools;
use crate::graph::edge::Edge;

pub type GraphIndex = usize;
#[derive(Debug, Clone, PartialEq)]
pub struct Graph<T> {
    vertices: HashMap<GraphIndex, T>,
    edges: HashSet<Edge>,
    next_valid_vertex_index: usize
}

#[allow(dead_code)]
impl<T> Graph<T> {
    pub fn new() -> Graph<T> {
        Self {
            vertices: Default::default(),
            edges: Default::default(),
            next_valid_vertex_index: Default::default()
        }
    }

    pub fn insert(&mut self, to_insert: T) -> GraphIndex {
        let number = self.next_valid_vertex_index;
        self.vertices.insert(number, to_insert);
        self.next_valid_vertex_index += 1;
        return number;
    }

    pub fn remove_at(&mut self, index: GraphIndex) -> bool {
        let removed = self.vertices.remove(&index);
        return match removed {
            None => false,
            Some(_) => {
                // Keep only edges which to not point to or come from the vertex at index
                self
                    .edges
                    .retain(|x| !(x.from == index || x.to == index));
                true
            }
        }
    }

    pub fn connect(&mut self, from: GraphIndex, to: GraphIndex) -> bool {
        if !self.vertices.contains_key(&from) || !self.vertices.contains_key(&to) {
            return false;
        }

        self.edges.insert(Edge {
            from,
            to,
        })
    }

    pub fn disconnect(&mut self, from: GraphIndex, to: GraphIndex) -> bool {
        self.edges.remove(&Edge {
            from,
            to,
        })
    }

    pub fn get_outbound_edges(&self, from: GraphIndex) -> Vec<Edge> {
        self.edges
            .iter()
            .filter_map(|edge| {
                if edge.from == from {
                    Some(*edge)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_inbound_edges(&self, to: GraphIndex) -> Vec<Edge> {
        self.edges
            .iter()
            .filter_map(|edge| {
                if edge.to == to {
                    Some(*edge)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_vertex_edges(&self, on: GraphIndex) -> Vec<Edge> {
        self.edges
            .iter()
            .filter_map(|edge| {
                if edge.to == on || edge.from == on {
                    Some(*edge)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn edges(&self) -> &HashSet<Edge> {
        &self.edges
    }

    pub fn edges_mut(&mut self) -> &mut HashSet<Edge> {
        &mut self.edges
    }

    pub fn vertices(&self) -> impl Iterator<Item=&T> {
        self
            .vertices
            .iter()
            .map(|(_, v)| v)
    }

    pub fn indices(&self) -> impl Iterator<Item=GraphIndex> + '_ {
        self
            .vertices
            .keys()
            .map(|x| *x)
    }

    pub fn get(&self, index: GraphIndex) -> Option<&T> {
        self.vertices.get(&index)
    }

    pub fn set(&mut self, index: GraphIndex, to: T) -> bool {
        let value = self.vertices.get_mut(&index);
        return match value {
            None => false,
            Some(value) => {
                *value = to;
                true
            }
        }
    }
}
impl<T: Display> Graph<T> {
    #[allow(dead_code)]
    pub fn as_mermaid(&self) -> String {
        let mut text = String::from("graph LR\n");
        for (index, vertex) in self.vertices.iter() {
            let escaped_vertex_text = vertex.to_string().replace("\"", "&quot;");
            text.push_str(format!("GraphIndex{}[{}]\n", index, escaped_vertex_text).as_str())
        }

        for edge in &self.edges {
            text.push_str(format!("GraphIndex{} --> GraphIndex{}\n", edge.from, edge.to).as_str())
        }

        text
    }


    #[allow(dead_code)]
    pub fn as_mermaid_debug(&self, iteration: usize) -> String {
        let mut text = String::new();
        for (index, vertex) in self.vertices.iter() {
            let escaped_vertex_text = vertex.to_string().replace("\"", "&quot;");
            text.push_str(format!("Graph{}Index{}[{}]\n", iteration, index, escaped_vertex_text).as_str())
        }

        for edge in &self.edges {
            text.push_str(format!("Graph{}Index{} --> Graph{}Index{}\n", iteration, edge.from, iteration, edge.to).as_str())
        }

        text
    }
}

impl<T> From<(Vec<T>, Vec<(GraphIndex, GraphIndex)>)> for Graph<T> {
    fn from((vertices, edges): (Vec<T>, Vec<(usize, usize)>)) -> Self {
        let vertices = HashMap::from_iter(vertices.into_iter().enumerate());
        let count = vertices.len();
        Self {
            vertices,
            edges: HashSet::from_iter(edges.into_iter().map(|(from, to)| Edge {from, to})),
            next_valid_vertex_index: count
        }
    }
}

// exists to do method overloading
pub trait GraphAppend<T> {
    fn append(&mut self, other: T) -> HashMap<GraphIndex, GraphIndex>;
}

impl<T> GraphAppend<Self> for Graph<T> {
    fn append(&mut self, mut other: Self) -> HashMap<GraphIndex, GraphIndex> {

        let indices = other
            .indices()
            .sorted()
            .collect::<Vec<GraphIndex>>();



        // copy edge information
        let mut edges = vec![];
        for other_index in &indices {
            edges.append(&mut other.get_vertex_edges(*other_index))
        }

        // insert all vertices from other
        let mut mapping = HashMap::new();
        for other_index in &indices {
            let value = other.vertices.remove(other_index).unwrap();
            mapping.insert(*other_index, self.insert(value));
        }

        // make all connections
        for edge in edges {
            let from = mapping[&edge.from];
            let to = mapping[&edge.to];
            self.connect(from, to);
        }

        mapping
    }
}