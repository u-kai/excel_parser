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
    pub fn search_element_all(&self, key: &str) -> Option<Vec<&str>> {
        let d = self
            .element
            .iter()
            .filter(|element| element.contains_key(key))
            .map(|element| &element[key])
            .nth(0);
        if d.is_some() {
            return Some(d.unwrap().iter().map(|s| s.as_str()).collect());
        }
        None
    }
    pub fn search_element(&self, key: &str) -> Option<&str> {
        if self.search_element_all(key).is_some() {
            return Some(self.search_element_all(key).unwrap()[0]);
        }
        None
    }
    pub fn set_element(&mut self, element: NodeElement) {
        self.element = Some(element)
    }
}

pub type NodeElement = HashMap<String, Vec<String>>;

#[cfg(test)]
mod node_value_test {
    use std::collections::HashMap;

    use super::NodeValue;

    #[test]
    fn search_element_test() {
        let mut node_value = NodeValue::new("test");
        let mut hash = HashMap::new();
        hash.insert("id".to_string(), vec!["yeah".to_string()]);
        node_value.set_element(hash);
        assert_eq!(node_value.search_element("id"), Some("yeah"));
        assert_eq!(node_value.search_element("non"), None);
    }
}
