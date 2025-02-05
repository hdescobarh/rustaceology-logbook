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
            todo!("Construct a new Graph struct.");
        }

        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            todo!()
        }

        pub fn with_edges(self, edges: &[Edge]) -> Self {
            todo!()
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            todo!()
        }

        pub fn node(&self, node: &str) -> Option<&Node> {
            todo!()
        }
    }
}
