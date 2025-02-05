use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Edge {
    label_1: String,
    label_2: String,
    attributes: HashMap<String, String>,
}

impl Edge {
    pub fn new(label_1: &str, label_2: &str) -> Self {
        Self {
            label_1: label_1.to_string(),
            label_2: label_2.to_string(),
            attributes: HashMap::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        for (name, value) in attrs {
            self.attributes.insert(name.to_string(), value.to_string());
        }

        self
    }

    pub fn attr(&self, attribute: &str) -> Option<&str> {
        self.attributes.get(attribute).map(|s| s.as_str())
    }
}
