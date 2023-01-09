use crate::Graph::Graph;
use crate::Graph::vertex::Vertex;

pub struct Edge<'a> {
    name: String,
    from: &'a Vertex<'a>,
    to: &'a Vertex<'a>,
}

impl Edge<'_> {
    pub(crate) fn new<'a>(name: String, from: &'a Vertex<'a>, to: &'a Vertex<'a>) -> Edge {
        Edge {
            name,
            from,
            to,
        }
    }
}

pub struct EdgeBuilder<'a> {
    graph: &'a mut Graph<'a>,
    name: String,
}

impl<'a> EdgeBuilder<'a> {
    pub(crate) fn from(self, from: String) -> EdgeToBuilder<'a> {
        let from = self.graph.get_vertex(from);
        EdgeToBuilder {
            builder: &self,
            from,
        }
    }
}

struct EdgeToBuilder<'a> {
    builder: &'a EdgeBuilder<'a>,
    from: &'a Vertex<'a>,
}

impl<'a> EdgeToBuilder<'a> {
    pub(crate) fn to(self, to: String) {
        let EdgeBuilder { graph, name } = self.builder;
        let to = graph.get_vertex(to);
        graph.edges.push(Edge {
            name: name.to_string(),
            from: self.from,
            to
        });
    }
}