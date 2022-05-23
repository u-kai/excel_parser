pub struct NodeElement<'a>(Vec<(&'a str, Vec<&'a str>)>);
impl<'a> NodeElement<'a> {
    fn new(key: &'a str, values: Vec<&'a str>) -> Self {
        NodeElement(vec![(key, values)])
    }
}

pub trait ElementsInterface<'a> {
    fn add_element(&mut self, key: &'a str, values: Vec<&'a str>) -> ();
    //fn contains_key(&self, key: &str) -> bool;
    //fn to_string(&self) -> String;
    //fn search_all_element(&self,key:&str)->Option<Vec<&'a str>>;
    //fn search_element(&self,key:&str)->Option<&'a str>;
}

impl<'a> ElementsInterface<'a> for NodeElement<'a> {
    fn add_element(&mut self, key: &'a str, values: Vec<&'a str>) -> () {
        self.0.push((key, values))
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
    fn add_element_test() {
        let mut element = NodeElement::new("test", vec!["value"]);
        element.add_element("test2", vec!["value2"]);
        assert_eq!(
            element.get(),
            vec![("test", vec!["value"]), ("test2", vec!["value2"])]
        )
    }
}
