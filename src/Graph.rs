struct Graph {
    name: String,
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

struct Property {
    name: String,
    value: String,
}

struct Vertex {
    name: String,
    edges: Vec<Edge>,
    properties: Vec<Property>,
}

trait VecExt {
    fn has(&self, property_name: &str, property_value: &str) -> Vec<&Vertex>;
}

impl VecExt for Vec<Vertex> {
    fn has(&self, property_name: &str, property_value: &str) -> Vec<&Vertex> {
        let mut matching_vertices = Vec::new();
        for vertex in self.iter() {
            for property in vertex.properties.iter() {
                if property.name == property_name && property.value == property_value {
                    matching_vertices.push(vertex);
                }
            }
        }
        matching_vertices
    }
}


struct Edge {
    name: String,
    from: String,
    to: String,
}

impl Graph {
    fn new(name: String) -> Graph {
        Graph {
            name,
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn addV(&mut self, name: String) {
        self.vertices.push(Vertex {
            name,
            edges: Vec::new(),
            properties: Vec::new(),
        });
    }

    fn addV_with_properties(&mut self, name: String, properties: Vec<Property>) {
        self.vertices.push(Vertex {
            name,
            edges: Vec::new(),
            properties,
        });
    }

    fn addE(&mut self, name: String) -> EdgeBuilder {
        EdgeBuilder {
            graph: self,
            name,
        }
    }

    fn V(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    fn E(&self) -> &Vec<Edge> {
        &self.edges
    }
}

struct EdgeBuilder<'a> {
    graph: &'a mut Graph,
    name: String,
}

impl<'a> EdgeBuilder<'a> {
    fn from(self, from: String) -> EdgeToBuilder<'a> {
        EdgeToBuilder {
            builder: self,
            from,
        }
    }
}

struct EdgeToBuilder<'a> {
    builder: EdgeBuilder<'a>,
    from: String,
}

impl<'a> EdgeToBuilder<'a> {
    fn to(self, to: String) {
        let EdgeBuilder { graph, name } = self.builder;
        graph.edges.push(Edge { name, from: self.from, to });
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
        assert_eq!(graph.edges.len(), 0);
    }

    #[test]
    fn add_e_from_to_chained_adds_edge_to_graph() {
        let mut graph = Graph::new(String::from("test"));
        graph.addV(String::from("target"));
        graph.addV(String::from("source"));
        graph.addE(String::from("edge")).from(String::from("source")).to(String::from("target"));
        assert_eq!(graph.edges.len(), 1);
    }

    #[test]
    fn v_returns_all_vertices() {
        let mut graph = Graph::new(String::from("test"));
        graph.addV(String::from("target"));
        graph.addV(String::from("source"));
        assert_eq!(graph.V().len(), 2);
    }

    #[test]
    fn e_returns_all_edges() {
        let mut graph = Graph::new(String::from("test"));
        graph.addV(String::from("target"));
        graph.addV(String::from("source"));
        graph.addE(String::from("edge")).from(String::from("source")).to(String::from("target"));
        assert_eq!(graph.E().len(), 1);
    }

    #[test]
    fn v_has_returns_matching_properties() {
        let mut graph = Graph::new(String::from("test"));
        graph.addV_with_properties(String::from("target"), vec![Property { name: String::from("name"), value: String::from("target") }]);
        graph.addV_with_properties(String::from("source"), vec![Property { name: String::from("name"), value: String::from("target") }]);
        graph.addV(String::from("source"));
        graph.addE(String::from("edge")).from(String::from("source")).to(String::from("target"));
        assert_eq!(graph.V().has("name", "target").len(), 2);
        assert_eq!(graph.V().has("name", "no one").len(), 0);
    }
}