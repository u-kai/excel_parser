use super::node_element::{ElementsInterface, NodeElement};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NodeValue<'a> {
    value: &'a str,
    element: Option<NodeElement<'a>>,
}

impl<'a> NodeValue<'a> {
    pub fn new(s: &'a str) -> Self {
        NodeValue {
            value: s,
            element: None,
        }
    }
    pub fn get_value(&self) -> &str {
        &self.value
    }
    pub fn is_containe_key_value(&self, key: &str, value: &str) -> bool {
        if self.element.is_some() {
            self.element
                .as_ref()
                .unwrap()
                .is_containe_key_value(key, value)
        } else {
            false
        }
    }
    pub fn to_string(&self) -> String {
        if let Some(element) = &self.element {
            format!("{} {}", self.value, element.to_string())
        } else {
            self.value.to_string()
        }
    }
    pub fn change_value(&mut self, value: &'a str) {
        self.value = value
    }
    pub fn search_all_element(&self, key: &str) -> Option<&Vec<&str>> {
        if self.element.is_some() {
            self.element.as_ref().unwrap().search_all(key)
        } else {
            None
        }
    }
    pub fn search_element(&self, key: &str) -> Option<&str> {
        if self.element.is_some() {
            self.element.as_ref().unwrap().search(key)
        } else {
            None
        }
    }
    pub fn add_element(&mut self, key: &'a str, values: Vec<&'a str>) {
        if self.element.is_some() {
            self.element.as_mut().unwrap().add(key, values)
        } else {
            self.element = Some(NodeElement::new(key, values));
        }
    }
}
impl<'a> Into<String> for NodeValue<'a> {
    fn into(self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod node_value_test {

    use super::NodeValue;

    #[test]
    fn search_element_test() {
        let mut node_value = NodeValue::new("test");
        node_value.add_element("id", vec!["yeah"]);
        assert_eq!(node_value.search_element("id"), Some("yeah"));
        assert_eq!(node_value.search_element("non"), None);
    }
    #[test]
    fn add_element_test() {
        let mut node = NodeValue::new("test");
        node.add_element("class", vec!["big"]);
        node.add_element("class", vec!["big2"]);
        assert_eq!(node.search_element("class"), Some("big"));
        assert_eq!(node.search_all_element("class"), Some(&vec!["big", "big2"]));
        assert_eq!(node.search_element("non"), None);
    }
    #[test]
    fn to_string_test_case_not_has_element() {
        let node = NodeValue::new("test");
        let expect: String = node.to_string();
        assert_eq!(expect, "test".to_string());
        let node = NodeValue::new("test2");
        let expect: String = node.to_string();
        assert_eq!(expect, "test2".to_string())
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
        node.add_element("yek1", vec!["value1 value2"]);
        let expect: String = node.into();
        assert_eq!(
            expect,
            r#"test key1="value1" yek1="value1 value2""#.to_string()
        );
        let mut node = NodeValue::new("test");
        node.add_element("a", vec!["value1"]);
        node.add_element("b", vec!["value2 value3"]);
        let expect: String = node.to_string();
        assert_eq!(expect, r#"test a="value1" b="value2 value3""#.to_string());
    }
}
