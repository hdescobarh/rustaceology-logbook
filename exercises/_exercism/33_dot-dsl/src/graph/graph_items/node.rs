use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    label: String,
    attributes: HashMap<String, String>,
}
impl Node {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            attributes: HashMap::new(),
        }
    }
    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        for (attribute, value) in attrs {
            self.attributes
                .insert(attribute.to_string(), value.to_string());
        }
        self
    }

    pub fn attr(&self, attribute: &str) -> Option<&str> {
        self.attributes.get(attribute).map(|v| v.as_str())
    }

    pub fn label(&self) -> &str {
        self.attributes.get("label").unwrap_or(&self.label)
    }
}
