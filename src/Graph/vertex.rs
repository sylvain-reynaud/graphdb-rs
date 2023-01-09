use crate::Graph::edge::Edge;

pub struct Vertex<'a> {
    pub name: String,
    edges: Vec<Edge<'a>>,
}

impl Vertex<'_> {
    pub(crate) fn new(name: String) -> Vertex<'static> {
        Vertex {
            name,
            edges: Vec::new(),
        }
    }
}
