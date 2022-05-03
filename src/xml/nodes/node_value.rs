use std::{ascii::AsciiExt, collections::HashMap, net::ToSocketAddrs};

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
    pub fn change_value(&mut self, value: &str) {
        self.value = value.to_string()
    }
    pub fn search_all_element(&self, key: &str) -> Option<Vec<&str>> {
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
        if self.search_all_element(key).is_some() {
            return Some(self.search_all_element(key).unwrap()[0]);
        }
        None
    }
    pub fn set_element(&mut self, element: NodeElement) {
        self.element = Some(element)
    }
    pub fn add_element(&mut self, key: &str, value: Vec<&str>) {
        if self.element.is_some() {
            if self.element.as_ref().unwrap().contains_key(key) {
                value.iter().map(|s| s.to_string()).for_each(|s| {
                    self.element.as_mut().unwrap().get_mut(key).unwrap().push(s);
                });
                return;
            }
            self.element.as_mut().unwrap().insert(
                key.to_string(),
                value.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            );
        } else {
            let mut element = HashMap::new();
            element.insert(
                key.to_string(),
                value.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            );
            self.set_element(element)
        }
    }
}
impl Into<String> for NodeValue {
    fn into(self) -> String {
        if let Some(element) = self.element {
            element.iter().for_each(|s| println!("{:?}", s));
            "".to_string()
        } else {
            self.value
        }
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
    #[test]
    fn add_element_test() {
        let mut node = NodeValue::new("test");
        let mut hash_map = HashMap::new();
        hash_map.insert("class".to_string(), vec!["big".to_string()]);
        node.add_element("class", vec!["big"]);
        node.add_element("class", vec!["big2"]);
        assert_eq!(node.search_element("class"), Some("big"));
        assert_eq!(node.search_all_element("class"), Some(vec!["big", "big2"]));
        assert_eq!(node.search_element("non"), None);
    }
    #[test]
    fn into_test_case_not_has_element() {
        let node = NodeValue::new("test");
        let expect: String = node.into();
        assert_eq!(expect, "test".to_string());
        let node = NodeValue::new("test2");
        let expect: String = node.into();
        assert_eq!(expect, "test2".to_string())
    }
    #[test]
    fn into_test_case_has_element() {
        let mut node = NodeValue::new("test");
        node.add_element("key1", vec!["value1"]);
        let expect: String = node.into();
        assert_eq!(expect, r#"test key1="value1""#.to_string());
        let mut node = NodeValue::new("test");
        node.add_element("key1", vec!["value1"]);
        node.add_element("key1", vec!["value1 value2"]);
        let expect: String = node.into();
        assert_eq!(expect, r#"test key1="value1 value2""#.to_string());
    }
}
