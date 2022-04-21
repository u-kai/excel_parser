use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NodeValue {
    value: String,
    element: Option<NodeElement>,
}

impl NodeValue {
    pub fn new(s: &str) -> Self {
        NodeValue {
            value: s.to_string(),
            element: None,
        }
    }
    pub fn get_value(&self) -> &str {
        &self.value
    }
    pub fn get_element(&self) -> &Option<NodeElement> {
        &self.element
    }
    pub fn set_element(&mut self, element: NodeElement) {
        self.element = Some(element)
    }
}

pub type NodeElement = HashMap<String, Vec<String>>;
