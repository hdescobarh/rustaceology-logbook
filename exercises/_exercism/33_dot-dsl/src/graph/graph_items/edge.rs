use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Edge {
    attributes: HashMap<String, String>,
}

impl Edge {
    pub fn new(node_1: &str, node_2: &str) -> Self {
        todo!();
    }

    pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
        todo!()
    }

    pub fn attr(&self, attribute: &str) -> Option<&str> {
        todo!()
    }
}
