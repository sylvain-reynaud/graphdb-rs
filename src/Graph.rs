mod vertex;
mod edge;

use crate::Graph::vertex::Vertex;
use crate::Graph::edge::{Edge, EdgeBuilder};

struct Graph<'a> {
    name: String,
    vertices: Vec<Vertex<'a>>,
    edges: Vec<Edge<'a>>,
}

impl Graph<'_> {
    fn new(name: String) -> Graph<'static> {
        Graph {
            name,
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn addV(&mut self, name: String) {
        let vertex = Vertex::new(name);
        self.vertices.push(vertex);
    }

    fn addE(&mut self, name: String) -> EdgeBuilder {
        EdgeBuilder {
            graph: self,
            name,
        }
    }

    fn get_vertex(&self, name: String) -> &Vertex {
        for vertex in &self.vertices {
            if vertex.name == name {
                return vertex;
            }
        }
        panic!("Vertex not found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_v_adds_vertex_to_graph() {
        let mut graph = Graph::new(String::from("test"));
        graph.addV(String::from("test"));
        assert_eq!(graph.vertices.len(), 1);
    }

    #[test]
    fn add_2_vertices_to_graph() {
        let mut graph = Graph::new(String::from("test"));
        graph.addV(String::from("test"));
        graph.addV(String::from("test"));
        assert_eq!(graph.vertices.len(), 2);
    }

    #[test]
    fn add_e_to_graph() {
        let mut graph = Graph::new(String::from("test"));
        graph.addV(String::from("target"));
        graph.addV(String::from("source"));
        graph.addE(String::from("test"));
        assert_eq!(graph.edges.len(), 1);
    }

    #[test]
    fn add_e_from_to_chained_adds_edge_to_graph() {
        let mut graph = Graph::new(String::from("test"));
        graph.addV(String::from("target"));
        graph.addV(String::from("source"));
        graph.addE(String::from("edge")).from(String::from("source")).to(String::from("target"));
        assert_eq!(graph.edges.len(), 1);
    }
}