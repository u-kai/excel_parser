use super::{
    node_type::NodeType,
    node_value::{NodeElement, NodeValue},
};
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct XMLNode {
    value: NodeValue,
    node_type: NodeType,
    children: Option<Box<Vec<XMLNode>>>,
}

impl XMLNode {
    pub fn new(s: &str, node_type: NodeType) -> Self {
        XMLNode {
            value: NodeValue::new(s),
            children: None,
            node_type,
        }
    }
    pub fn new_with_element(s: &str, element: Option<NodeElement>, node_type: NodeType) -> Self {
        if element.is_some() {
            let mut node = XMLNode::new(s, node_type);
            node.value.set_element(element.unwrap());
            node
        } else {
            XMLNode::new(s, node_type)
        }
    }
    pub fn get_child_nodes(&self) -> Option<Vec<&XMLNode>> {
        if self.has_nodes() {
            let nodes = self
                .children
                .as_ref()
                .unwrap()
                .iter()
                .filter(|node| {
                    node.node_type == NodeType::SingleElement || node.node_type == NodeType::Element
                })
                .collect::<Vec<_>>();
            if nodes.len() == 0 {
                return None;
            }
            return Some(nodes);
        }
        None
    }
    #[allow(dead_code)]
    pub fn get_child_charcter(&self, n: usize) -> Option<&str> {
        let maybe_charcters = self.get_all_charcters();
        if maybe_charcters.is_some() {
            return maybe_charcters.unwrap().get(n).map(|c| *c);
        }
        None
    }
    #[allow(dead_code)]
    pub fn get_all_charcters(&self) -> Option<Vec<&str>> {
        if self.has_characters() {
            let chars = self
                .children
                .as_ref()
                .unwrap()
                .iter()
                .filter(|node| node.node_type == NodeType::Character)
                .map(|c_node| c_node.get_value())
                .collect::<Vec<_>>();
            return Some(chars);
        }
        None
    }
    #[allow(dead_code)]
    pub fn search_node(&self, search_value: &str) -> Option<&XMLNode> {
        if self.has_nodes() {
            return self
                .get_child_nodes()
                .as_ref()
                .unwrap()
                .iter()
                .filter(|child| child.get_value() == search_value)
                .collect::<Vec<_>>()
                .get(0)
                .map(|child| **child);
        }
        None
    }
    #[allow(dead_code)]
    pub fn search_all_nodes(&self, search_value: &str) -> Option<Vec<&XMLNode>> {
        if self.has_nodes() {
            return Some(
                self.get_child_nodes()
                    .as_ref()
                    .unwrap()
                    .iter()
                    .filter(|child| child.get_value() == search_value)
                    .map(|node| *node)
                    .collect(),
            );
        }
        None
    }
    #[allow(dead_code)]
    pub fn search_element(&self, key: &str) -> Option<&str> {
        self.value.search_element(key)
    }
    #[allow(dead_code)]
    pub fn nth_child_node(&self, n: usize) -> Option<&XMLNode> {
        if self.has_nodes() {
            return self.get_child_nodes().unwrap().get(n).map(|c| *c);
        }
        None
    }
    pub fn add_node(&mut self, child: XMLNode) {
        if self.has_children() {
            self.children.as_mut().unwrap().push(child);
            return;
        }
        self.children = Some(Box::new(vec![child]));
    }
    pub fn add_charcter(&mut self, s: &str) {
        if self.has_children() {
            self.children
                .as_mut()
                .unwrap()
                .push(XMLNode::new(s, NodeType::Character));
            return;
        }
        self.children = Some(Box::new(vec![XMLNode::new(s, NodeType::Character)]));
    }
    #[allow(dead_code)]
    pub fn add_element(&mut self, key: &str, value: Vec<&str>) {
        self.value.add_element(key, value)
    }
    #[allow(dead_code)]
    pub fn element_all(&self, key: &str, value: &str) -> Option<Vec<&XMLNode>> {
        if self.has_nodes() {
            let maybe = self
                .get_child_nodes()
                .as_ref()
                .unwrap()
                .iter()
                .filter(|node| node.is_containe_key_value(key, value))
                .map(|node| *node)
                .collect::<Vec<_>>();
            if maybe.len() == 0 {
                return None;
            }
            return Some(maybe);
        }
        None
    }
    pub fn is_containe_key_value(&self, key: &str, value: &str) -> bool {
        if let Some(element) = &self.value.get_element() {
            if element.contains_key(key) {
                element[key].contains(&value.to_string())
            } else {
                false
            }
        } else {
            false
        }
    }
    #[allow(dead_code)]
    pub fn serach_child_rec(&self, key: &str, value: &str) -> Option<&XMLNode> {
        match self.get_child_nodes() {
            Some(children) => {
                for child in children.iter() {
                    if child.is_containe_key_value(key, value) {
                        return Some(child);
                    }
                    let result_rec = child.serach_child_rec(key, value);
                    if let Some(node) = result_rec {
                        return Some(node);
                    }
                }
                None
            }
            None => None,
        }
    }
    #[allow(dead_code)]
    pub fn get_value(&self) -> &str {
        &self.value.get_value()
    }
    #[allow(dead_code)]
    pub fn get_node_value(&mut self) -> &mut NodeValue {
        &mut self.value
    }
    #[allow(dead_code)]
    fn has_characters(&self) -> bool {
        if self.has_children() {
            return self
                .children
                .as_ref()
                .unwrap()
                .iter()
                .find(|node| node.node_type == NodeType::Character)
                .is_some();
        }
        false
    }
    fn has_nodes(&self) -> bool {
        if self.has_children() {
            return self
                .children
                .as_ref()
                .unwrap()
                .iter()
                .find(|node| {
                    node.node_type == NodeType::Element || node.node_type == NodeType::SingleElement
                })
                .is_some();
        }
        false
    }
    fn has_children(&self) -> bool {
        self.children.is_some()
    }
}

/// test code

#[cfg(test)]
pub mod xml_node_test {
    use std::collections::HashMap;

    use crate::xml::nodes::node::{NodeType, XMLNode};
    #[test]
    fn get_nth_child_test() {
        let data = r#"<div id="1180" name="kai"><div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
        </div>"#;
        let root_node = XMLNode::from(data);
        let child = root_node.nth_child_node(0).unwrap();
        assert_eq!(
            child,
            &XMLNode::from(
                r#"<div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
            <div/>"#
            )
        );

        let char = child.get_child_charcter(0);
        assert_eq!(char, Some("div-first"));
        let char = child.get_child_charcter(1);
        assert_eq!(char, Some("div-data"));
        let char = child.get_child_charcter(2);
        assert_eq!(char, None);
        let child = root_node.nth_child_node(2);
        assert_eq!(child, None);
    }
    #[test]
    fn search_node_test() {
        let data = r#"<div id="1180" name="kai"><div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
        </div>"#;
        let root_node = XMLNode::from(data);
        let search_node = root_node.search_node("div").unwrap().clone();
        assert_eq!(
            search_node,
            XMLNode::from(
                r#"<div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
            <div/>"#
            )
        );
        let search_node = search_node.search_node("p").unwrap();
        assert_eq!(search_node, &XMLNode::from(r#"<p>p-data</p>"#));
    }
    #[test]

    fn search_nodes_test() {
        let data = r#"<div id="1180" name="kai"><div>div-first
            <p>p-data</p>
            <p>p-data-2</p>
            <data/>
            div-data</div>
        </div>"#;
        let root_node = XMLNode::from(data);
        let search_node = root_node.search_all_nodes("div").unwrap();
        assert_eq!(
            search_node,
            vec![&XMLNode::from(
                r#"<div>div-first
            <p>p-data</p>
            <p>p-data-2</p>
            <data/>
            div-data</div>
            <div/>"#
            )]
        );
        println!(
            "##################{:?}",
            &XMLNode::from(
                r#"<div>div-first
            <p>p-data</p>
            <p>p-data-2</p>
            <data/>
            div-data</div>
            "#
            )
        );
        let search_node = search_node[0];
        let search_node = search_node.search_all_nodes("p").unwrap();
        assert_eq!(
            search_node,
            vec![
                &XMLNode::from(r#"<p>p-data</p>"#),
                &XMLNode::from(r#"<p>p-data-2</p>"#)
            ]
        );
    }
    #[test]
    fn element_all_test() {
        let data = r#"<div id="1180" name="kai">
            <p class="p1">p-data</p>
            <p class="p1">p-data-2</p>
            <data/>
        </div>"#;
        let root_node = XMLNode::from(data);
        assert_eq!(
            root_node.element_all("class", "p1"),
            Some(vec![
                &XMLNode::from(r#"<p class="p1">p-data</p>"#),
                &XMLNode::from(r#"<p class="p1">p-data-2</p>"#)
            ])
        );
    }
    #[test]
    fn add_element_test() {
        let mut node = XMLNode::new("div", NodeType::Element);

        node.add_element("class", vec!["big"]);
        let mut tobe_element = HashMap::new();
        tobe_element.insert("class".to_string(), vec!["big".to_string()]);
        assert_eq!(
            node,
            XMLNode::new_with_element("div", Some(tobe_element), NodeType::Element)
        )
    }
}
