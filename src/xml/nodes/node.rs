use super::{node_type::NodeType, node_value::NodeValue};
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct XMLNode<'a> {
    value: NodeValue<'a>,
    node_type: NodeType,
    children: Option<Box<Vec<XMLNode<'a>>>>,
}

impl<'a> XMLNode<'a> {
    pub fn new(s: &'a str, node_type: NodeType) -> Self {
        XMLNode {
            value: NodeValue::new(s),
            children: None,
            node_type,
        }
    }
    pub fn new_with_element(
        s: &'a str,
        element_key: &'a str,
        element_values: Vec<&'a str>,
        node_type: NodeType,
    ) -> Self {
        let mut value = NodeValue::new(s);
        value.add_element(element_key, element_values);
        XMLNode {
            value,
            node_type,
            children: None,
        }
    }
    pub fn get_node_type(&self) -> NodeType {
        self.node_type.clone()
    }
    pub fn set_node_type(&mut self, node_type: NodeType) {
        self.node_type = node_type
    }
    pub fn get_children(&self) -> Option<Vec<&XMLNode<'a>>> {
        if self.has_children() {
            Some(self.children.as_ref().unwrap().iter().collect())
        } else {
            None
        }
    }
    /// Returns children that source has.
    ///
    ///     let source = r#"
    ///         <div id="1180" name="kai">
    ///             <div>
    ///                 div-first
    ///                 <p>p-data</p>
    ///                 <p>p-data</p>
    ///                 <data/>
    ///                 div-data
    ///              </div>
    ///          </div>"#;
    ///
    ///     let node = XMLNode::from(source);
    ///     assert_eq!(node.get_child_nodes(),XMLNode::from(r#"
    ///             <div>
    ///                 div-first
    ///                 <p>p-data</p>
    ///                 <p>p-data</p>
    ///                 <data/>
    ///                 div-data
    ///              </div>"#)
    ///     );
    ///
    pub fn get_child_nodes(&self) -> Option<Vec<&XMLNode<'a>>> {
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
    pub fn get_child_nodes_mut(&mut self) -> Option<Vec<&mut XMLNode<'a>>> {
        if self.has_nodes() {
            let nodes = self
                .children
                .as_mut()
                .unwrap()
                .iter_mut()
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
    pub fn get_child_text(&self, n: usize) -> Option<&str> {
        let maybe_texts = self.get_all_texts();
        if maybe_texts.is_some() {
            return maybe_texts.unwrap().get(n).map(|c| *c);
        }
        None
    }
    #[allow(dead_code)]
    pub fn get_all_texts(&self) -> Option<Vec<&str>> {
        self.children.as_ref().map(|children| {
            children
                .iter()
                .filter(|node| node.node_type == NodeType::Character)
                .map(|node| node.get_value())
                .collect::<Vec<_>>()
        })
    }

    #[allow(dead_code)]
    pub fn search_node_mut(&mut self, search_value: &str) -> Option<&mut XMLNode<'a>> {
        if self.has_nodes() {
            return self
                .children
                .as_mut()
                .unwrap()
                .iter_mut()
                .filter(|child| child.get_value() == search_value)
                .next();
        }
        None
    }
    #[allow(dead_code)]
    pub fn search_all_nodes_mut(&mut self, search_value: &str) -> Option<Vec<&mut XMLNode<'a>>> {
        if self.has_nodes() {
            Some(
                self.children
                    .as_mut()
                    .unwrap()
                    .iter_mut()
                    .filter(|child| child.get_value() == search_value)
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn search_node(&self, search_value: &str) -> Option<&XMLNode<'a>> {
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
    pub fn add_node(&mut self, child: XMLNode<'a>) {
        if self.has_children() {
            self.children.as_mut().unwrap().push(child);
            return;
        }
        self.children = Some(Box::new(vec![child]));
    }
    pub fn add_text(&mut self, s: &'a str) {
        if self.has_children() {
            s.split(' ').for_each(|s| {
                self.children
                    .as_mut()
                    .unwrap()
                    .push(XMLNode::new(s, NodeType::Character))
            });
            return;
        }
        self.children = Some(Box::new(vec![XMLNode::new(s, NodeType::Character)]));
    }
    pub fn change_child_node(&mut self, new_node: XMLNode<'a>) {
        if self.has_children() {
            for (i, child) in &mut self.children.as_mut().unwrap().iter().enumerate() {
                if child.get_value() == new_node.get_value() {
                    self.children.as_mut().unwrap()[i] = new_node;
                    break;
                }
            }
            return;
        }
        self.add_node(new_node)
    }
    pub fn change_text(&mut self, s: &'a str) {
        fn _push_text_node<'a>(node: &mut XMLNode<'a>, s: &'a str) {
            let _ = s.split(' ').filter(|s| s.len() > 0).for_each(|splited| {
                node.children
                    .as_mut()
                    .unwrap()
                    .push(XMLNode::new(splited, NodeType::Character));
            });
        }
        // todo not use clone
        if self.has_children() {
            let new_child = self
                .children
                .as_ref()
                .unwrap()
                .iter()
                .filter(|node| node.node_type != NodeType::Character)
                .map(|node| node.clone())
                .collect::<Vec<_>>();
            self.children = Some(Box::new(new_child));
            _push_text_node(self, s)
        }
        self.children = Some(Box::new(vec![]));
        _push_text_node(self, s);
    }
    #[allow(dead_code)]
    pub fn add_element(&mut self, key: &'a str, values: Vec<&'a str>) {
        self.value.add_element(key, values)
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
        self.value.is_containe_key_value(key, value)
    }
    #[allow(dead_code)]
    pub fn search_child_by_id_mut(&mut self, key: &str, value: &str) -> Option<&mut XMLNode<'a>> {
        match self.get_child_nodes_mut() {
            Some(mut children) => {
                for child in children.drain(..) {
                    if child.is_containe_key_value(key, value) {
                        return Some(child);
                    }
                    let result_rec = child.search_child_by_id_mut(key, value);
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
    pub fn search_child_by_id(&self, key: &str, value: &str) -> Option<&XMLNode> {
        match self.get_child_nodes() {
            Some(children) => {
                for child in children.iter() {
                    if child.is_containe_key_value(key, value) {
                        return Some(child);
                    }
                    let result_rec = child.search_child_by_id(key, value);
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
    pub fn get_node_value(&self) -> &NodeValue {
        &self.value
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
    pub fn has_children(&self) -> bool {
        self.children.is_some()
    }
    pub fn search_all_child(&self, key: &str, value: &str) -> Vec<&XMLNode> {
        let mut buf = Vec::new();
        if self.is_containe_key_value(key, value) {
            buf.push(self);
        }
        if self.has_nodes() {
            self.get_child_nodes().unwrap().iter().for_each(|child| {
                child
                    .search_all_child(key, value)
                    .iter()
                    .for_each(|v| buf.push(*v));
            });
            buf
        } else {
            buf
        }
    }
}

#[cfg(test)]
pub mod xml_node_test {

    use crate::xml::nodes::node::{NodeType, XMLNode};
    #[test]
    fn get_nth_child_test() {
        let data = r#"
        <div id="1180" name="kai"><div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
        </div>"#;
        let root_node = XMLNode::from(data);
        let child = root_node.nth_child_node(0).unwrap();
        assert_eq!(
            child,
            &XMLNode::from(
                r#"
            <div>div-first
                <p>p-data</p>
                <data/>
                div-data
            </div>"#
            )
        );

        let char = child.get_child_text(0);
        assert_eq!(char, Some("div-first"));
        let char = child.get_child_text(1);
        assert_eq!(char, Some("div-data"));
        let char = child.get_child_text(2);
        assert_eq!(char, None);
        let child = root_node.nth_child_node(2);
        assert_eq!(child, None);
    }
    #[test]
    fn search_node_test() {
        let data = r#"
        <div id="1180" name="kai">
            <div>div-first
                <p>p-data</p>
                <data/>
                div-data
            </div>
        </div>"#;
        let root_node = XMLNode::from(data);
        let search_node = root_node.search_node("div").unwrap().clone();
        assert_eq!(
            search_node,
            XMLNode::from(
                r#"
            <div>div-first
                <p>p-data</p>
                <data/>
                div-data
            </div>"#
            )
        );
        let search_node = search_node.search_node("p").unwrap();
        assert_eq!(search_node, &XMLNode::from(r#"<p>p-data</p>"#));
    }
    #[test]
    fn search_node_mut_test() {
        let data = r#"
        <div id="1180" name="kai">
            <div>div-first
                <p>p-data</p>
                <p>p-data-2</p>
                <data/>
                div-data
            </div>
        </div>"#;
        let mut root_node = XMLNode::from(data);
        let search_node = root_node.search_node_mut("div").unwrap();

        assert_eq!(
            search_node,
            &XMLNode::from(
                r#"
            <div>div-first
                <p>p-data</p>
                <p>p-data-2</p>
                <data/>
                div-data
            </div>"#
            )
        );
    }
    #[test]
    fn search_all_nodes_mut_test() {
        let data = r#"
        <div id="1180" name="kai">
            <div>div-first
                <p>p-data</p>
                <p>p-data-2</p>
                <data/>
                div-data
            </div>
        </div>"#;
        let mut root_node = XMLNode::from(data);
        let search_node = root_node.search_all_nodes_mut("div").unwrap();

        assert_eq!(
            search_node,
            vec![&XMLNode::from(
                r#"
            <div>div-first
                <p>p-data</p>
                <p>p-data-2</p>
                <data/>
                div-data
            </div>"#
            )]
        );
    }
    fn search_child_id_mut_test() {
        let data = r#"
        <div id="1180" name="kai">
            <div>div-first
                <p id="test">p-data</p>
                <p>p-data-2</p>
                <data/>
                div-data
            </div>
        </div>"#;
        let mut root_node = XMLNode::from(data);
        let search_node = root_node.search_child_by_id_mut("id", "test").unwrap();

        assert_eq!(
            search_node,
            &mut XMLNode::from(
                r#"
                <p id="test">p-data</p>
            "#
            )
        );
        let search_node = root_node.search_child_by_id_mut("id", "test").unwrap();
        assert_eq!(
            search_node,
            &mut XMLNode::from(
                r#"
                <p id="test">p-data</p>
            "#
            )
        );
    }
    #[test]

    fn search_nodes_test() {
        let data = r#"
        <div id="1180" name="kai">
            <div>div-first
                <p>p-data</p>
                <p>p-data-2</p>
                <data/>
                div-data
            </div>
        </div>"#;
        let root_node = XMLNode::from(data);
        let search_node = root_node.search_all_nodes("div").unwrap();
        assert_eq!(
            search_node,
            vec![&XMLNode::from(
                r#"
            <div>div-first
                <p>p-data</p>
                <p>p-data-2</p>
                <data/>
                div-data
            </div>"#
            )]
        );
        println!(
            "##################{:?}",
            &XMLNode::from(
                r#"
            <div>div-first
                <p>p-data</p>
                <p>p-data-2</p>
                <data/>
                div-data
            </div>"#
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
        let data = r#"
        <div id="1180" name="kai">
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
        assert_eq!(
            node,
            XMLNode::new_with_element("div", "class", vec!["big"], NodeType::Element)
        )
    }
    #[test]
    fn change_text_test() {
        let mut node = XMLNode::new("div", NodeType::Element);
        node.change_text("hello world");
        let mut tobe_node = XMLNode::new("div", NodeType::Element);
        tobe_node.add_text("hello");
        tobe_node.add_text("world");
        assert_eq!(node, tobe_node);
        node.change_text("hello world rust");
        tobe_node.add_text("rust");
        assert_eq!(node, tobe_node);
    }
    #[test]
    fn search_all_child_test() {
        let data = r#"
        <xml>
        <div id="1180" name="kai" class="blue">
            <div>div-first
                <p class="blue">p-data</p>
                <p>p-data-2</p>
                <data/>
                div-data
            </div>
        </div>"#;
        let node = XMLNode::from(data);
        let buf = node.search_all_child("class", "blue");
        let node1 = r#"
        <div id="1180" name="kai" class="blue">
            <div>div-first
                <p class="blue">p-data</p>
                <p>p-data-2</p>
                <data/>
                div-data
            </div>
        </div>"#;
        let node2 = r#"
                <p class="blue">p-data</p>
                "#;
        let node1 = XMLNode::from(node1);
        let node2 = XMLNode::from(node2);
        let tobe = vec![&node1, &node2];
        println!("{:?}", buf[1]);
        assert_eq!(buf, tobe)
    }
}
