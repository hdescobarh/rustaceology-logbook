#[derive(Debug, PartialEq)]
pub struct Node {}
impl Node {
    pub fn new(name: &str) -> Self {
        todo!();
    }
    pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
        todo!()
    }

    pub fn attr(&self, attribute: &str) -> Option<&str> {
        todo!()
    }
}
