pub mod graph {
    pub mod graph_items;
    use std::collections::HashMap;

    use graph_items::edge::Edge;
    use graph_items::node::Node;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            for node in nodes {
                self.nodes.push(node.clone());
            }
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            for edge in edges {
                self.edges.push(edge.clone());
            }
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (attribute, value) in attrs {
                self.attrs.insert(attribute.to_string(), value.to_string());
            }
            self
        }

        pub fn node(&self, label: &str) -> Option<&Node> {
            self.nodes.iter().find(|&node| node.label() == label)
        }
    }
}
