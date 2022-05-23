pub struct NodeElement<'a>(Vec<(&'a str, Vec<&'a str>)>);
impl<'a> NodeElement<'a> {
    fn new(key: &'a str, values: Vec<&'a str>) -> Self {
        NodeElement(vec![(key, values)])
    }
}
fn taple_to_string(taple: &(&str, Vec<&str>)) -> String {
    if taple.1.len() == 0 {
        taple.0.to_string()
    } else {
        format!(r#"{}="{}""#, taple.0, taple.1.join(" "))
    }
}

pub trait ElementsInterface<'a> {
    fn add_element(&mut self, key: &'a str, values: Vec<&'a str>) -> ();
    fn contains_key(&self, key: &str) -> bool;
    fn to_string(&self) -> String;
    fn search_all_element(&self, key: &str) -> Option<&Vec<&'a str>>;
    //fn search_element(&self,key:&str)->Option<&'a str>;
}

impl<'a> ElementsInterface<'a> for NodeElement<'a> {
    fn add_element(&mut self, key: &'a str, values: Vec<&'a str>) -> () {
        if self.contains_key(key) {
            self.0
                .iter_mut()
                .filter(|(e_key, _)| *e_key == key)
                .for_each(|(_key, mut_values)| {
                    values.iter().for_each(|value| mut_values.push(*value))
                });
            return;
        }
        self.0.push((key, values))
    }
    fn to_string(&self) -> String {
        let mut with_last_empty = self.0.iter().fold("".to_string(), |acc, cur| {
            format!("{}{} ", acc, taple_to_string(cur))
        });
        with_last_empty.pop();
        with_last_empty
    }
    fn contains_key(&self, key: &str) -> bool {
        self.0.iter().any(|(e_key, _values)| key == *e_key)
    }
    fn search_all_element(&self, key: &str) -> Option<&Vec<&'a str>> {
        self.0
            .iter()
            .find(|(e_key, _values)| *e_key == key)
            .map(|(_key, values)| values)
    }
}

#[cfg(test)]
mod node_element_tests {
    use crate::xml::nodes::node_element::ElementsInterface;
    impl<'a> NodeElement<'a> {
        pub fn get(&self) -> Vec<(&'a str, Vec<&'a str>)> {
            self.0.clone()
        }
    }
    use super::NodeElement;
    #[test]
    fn search_all_element_test() {
        let mut element = NodeElement::new("test", vec!["value"]);
        element.add_element("test2", vec!["value2", "value3"]);
        assert_eq!(element.search_all_element("test"), Some(&vec!["value"]));
        assert_eq!(
            element.search_all_element("test2"),
            Some(&vec!["value2", "value3"])
        );
        assert_eq!(element.search_all_element("test3"), None);
    }
    #[test]
    fn containes_key_test() {
        let mut element = NodeElement::new("test", vec!["value"]);
        element.add_element("test2", vec!["value2", "value3"]);
        assert_eq!(element.contains_key("test"), true);
        assert_eq!(element.contains_key("test2"), true);
        assert_eq!(element.contains_key("test3"), false);
    }
    #[test]
    fn to_string_test() {
        let mut element = NodeElement::new("test", vec!["value"]);
        element.add_element("test2", vec!["value2", "value3"]);
        assert_eq!(
            element.to_string(),
            r#"test="value" test2="value2 value3""#.to_string()
        );
        element.add_element("test3", vec![]);
        assert_eq!(
            element.to_string(),
            r#"test="value" test2="value2 value3" test3"#.to_string()
        );
    }
    #[test]
    fn add_element_test() {
        let mut element = NodeElement::new("test", vec!["value"]);
        element.add_element("test2", vec!["value2"]);
        assert_eq!(
            element.get(),
            vec![("test", vec!["value"]), ("test2", vec!["value2"])]
        );
    }
    #[test]
    fn add_element_case_add_same_key() {
        let mut element = NodeElement::new("test", vec!["value"]);
        element.add_element("test2", vec!["value2"]);
        assert_eq!(
            element.get(),
            vec![("test", vec!["value"]), ("test2", vec!["value2"])]
        );
        element.add_element("test2", vec!["value3"]);
        assert_eq!(
            element.get(),
            vec![("test", vec!["value"]), ("test2", vec!["value2", "value3"])]
        );
    }
}
