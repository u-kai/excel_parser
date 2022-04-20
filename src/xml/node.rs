use std::collections::HashMap;

use super::token::{Token, TokenType};

type NodeElement = HashMap<String, String>;
#[derive(Debug, PartialEq, Eq, Clone)]
struct NodeValue {
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
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct XMLNode {
    value: NodeValue,
    children: Option<Box<Vec<XMLNode>>>,
}

impl XMLNode {
    pub fn new(s: &str) -> Self {
        XMLNode {
            value: NodeValue::new(s),
            children: None,
        }
    }
    pub fn add_child(&mut self, child: XMLNode) {
        if self.has_children() {
            self.children.as_mut().unwrap().push(child);
            return;
        }
        self.children = Some(Box::new(vec![child]));
    }
    pub fn search_node(&self, search_value: &str) -> Option<&XMLNode> {
        if self.has_children() {
            return self
                .children
                .as_ref()
                .unwrap()
                .iter()
                .filter(|child| child.get_value() == search_value)
                .nth(0);
        }
        None
    }
    pub fn search_nodes(&self, search_value: &str) -> Option<Vec<&XMLNode>> {
        if self.has_children() {
            return Some(
                self.children
                    .as_ref()
                    .unwrap()
                    .iter()
                    .filter(|child| child.get_value() == search_value)
                    .collect(),
            );
        }
        None
    }
    pub fn nth_child(&mut self, n: usize) -> Option<XMLNode> {
        if self.has_children() {
            let result = Some(self.children.as_mut().unwrap().remove(n));
            if self.children.as_ref().unwrap().len() == 0 {
                self.children = None;
            }
            return result;
        }
        None
    }
    pub fn get_value(&self) -> &str {
        &self.value.value
    }
    fn has_children(&self) -> bool {
        self.children.is_some()
    }
}

impl From<&str> for XMLNode {
    fn from(s: &str) -> Self {
        let token_array = Token::create_token_array(s);
        XMLNode::from(token_array)
    }
}
impl From<Token> for XMLNode {
    fn from(token: Token) -> Self {
        fn _token_to_node(token: Token) -> XMLNode {
            let mut split_value = token
                .get_value()
                .split(' ')
                .filter(|s| s.len() != 0)
                .collect::<Vec<_>>();
            let value = split_value.remove(0);
            if split_value.len() == 0 {
                return XMLNode::new(value);
            }
            let mut element: HashMap<String, String> = HashMap::new();
            for s in split_value.iter() {
                let kv = s.split('=').collect::<Vec<_>>();
                element.insert(kv[0].to_string(), kv.get(1).unwrap_or(&"").to_string());
            }
            XMLNode {
                value: NodeValue {
                    value: value.to_string(),
                    element: Some(element),
                },
                children: None,
            }
        }
        match token.get_token_type() {
            TokenType::Character => XMLNode::new(token.get_value()),
            _ => _token_to_node(token),
        }
    }
}
impl From<Vec<Token>> for XMLNode {
    fn from(token_array: Vec<Token>) -> Self {
        let mut parent_stack = Vec::new();
        for token in token_array {
            match token.get_token_type() {
                TokenType::StartToken => parent_stack.push(XMLNode::from(token)),
                TokenType::Character => {
                    let child = XMLNode::from(token);
                    parent_stack.last_mut().unwrap().add_child(child)
                }
                TokenType::SingleToken => {
                    let node = XMLNode::from(token);
                    parent_stack.last_mut().unwrap().add_child(node);
                }
                TokenType::EndToken => {
                    let child = parent_stack.pop();
                    match child {
                        Some(c) => {
                            if parent_stack.len() == 0 {
                                return c;
                            }
                            parent_stack.last_mut().unwrap().add_child(c)
                        }
                        None => panic!("error: this case is not parse"),
                    }
                }
            }
        }
        // case exist declear line
        if parent_stack.len() == 1 {
            return parent_stack.pop().unwrap();
        }
        panic!("not had end tag this stack : {:?}", parent_stack)
    }
}

#[cfg(test)]
mod xml_node_test {
    use std::collections::HashMap;

    use super::{Token, XMLNode};
    #[test]
    fn from_token_array_test() {
        let data = "<div>
            <p>p-data</p>
            div-data
        </div>";
        let token_array = Token::create_token_array(data);
        let expect = XMLNode::from(token_array);
        let p_child = XMLNode::new("p-data");
        let mut p = XMLNode::new("p");
        p.add_child(p_child);
        let div_child = XMLNode::new("div-data");
        let mut div = XMLNode::new("div");
        div.add_child(p);
        div.add_child(div_child);
        assert_eq!(expect, div);
        let data = "<div><div>div-first
            <p>p-data</p>
            div-data</div>
        </div>";
        let token_array = Token::create_token_array(data);
        let expect = XMLNode::from(token_array);
        let p_child = XMLNode::new("p-data");
        let mut p = XMLNode::new("p");
        p.add_child(p_child);
        let div_child = XMLNode::new("div-data");
        let mut div = XMLNode::new("div");
        let mut child_div = XMLNode::new("div");
        let child_div_child = XMLNode::new("div-first");
        child_div.add_child(child_div_child);
        child_div.add_child(p);
        child_div.add_child(div_child);
        div.add_child(child_div);
        assert_eq!(expect, div);
        let data = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
        <div><div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
        </div>"#;
        let expect = XMLNode::from(data);
        let mut root = XMLNode::new("?xml");
        let mut root_element = HashMap::new();
        root_element.insert("standalone".to_string(), r#""yes"?"#.to_string());
        root_element.insert("encoding".to_string(), r#""UTF-8""#.to_string());
        root_element.insert("version".to_string(), r#""1.0""#.to_string());
        root.value.element = Some(root_element);
        let p_child = XMLNode::new("p-data");
        let mut p = XMLNode::new("p");
        p.add_child(p_child);
        let div_child = XMLNode::new("div-data");
        let single_data = XMLNode::new("data");
        let mut div = XMLNode::new("div");
        let mut child_div = XMLNode::new("div");
        let child_div_child = XMLNode::new("div-first");
        child_div.add_child(child_div_child);
        child_div.add_child(p);
        child_div.add_child(single_data);
        child_div.add_child(div_child);
        div.add_child(child_div);
        root.add_child(div);
        assert_eq!(expect, root)
    }
    #[test]
    fn element_test() {
        let data = r#"<div id="1180" name="kai"><div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
        </div>"#;
        let token_array = Token::create_token_array(data);
        let expect = XMLNode::from(token_array);
        let p_child = XMLNode::new("p-data");
        let mut p = XMLNode::new("p");
        p.add_child(p_child);
        let div_child = XMLNode::new("div-data");
        let single_data = XMLNode::new("data");
        let mut div = XMLNode::new("div");
        let mut element = HashMap::new();

        element.insert("name".to_string(), r#""kai""#.to_string());
        element.insert("id".to_string(), r#""1180""#.to_string());
        div.value.element = Some(element);
        let mut child_div = XMLNode::new("div");
        let child_div_child = XMLNode::new("div-first");
        child_div.add_child(child_div_child);
        child_div.add_child(p);
        child_div.add_child(single_data);
        child_div.add_child(div_child);
        div.add_child(child_div);
        assert_eq!(expect, div)
    }
    #[test]
    fn get_nth_child_test() {
        let data = r#"<div id="1180" name="kai"><div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
        </div>"#;
        let mut root_node = XMLNode::from(data);
        let child = root_node.nth_child(0).unwrap();
        assert_eq!(
            child,
            XMLNode::from(
                r#"<div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
            <div/>"#
            )
        );
        let child = root_node.nth_child(0);
        assert_eq!(child, None);
        let child = root_node.nth_child(0);
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
    fn search_nodes_test() {
        let data = r#"<div id="1180" name="kai"><div>div-first
            <p>p-data</p>
            <p>p-data-2</p>
            <data/>
            div-data</div>
        </div>"#;
        let root_node = XMLNode::from(data);
        let search_node = root_node.search_nodes("div").unwrap();
        assert_eq!(
            search_node,
            vec![&XMLNode::from(
                r#"<div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
            <div/>"#
            )]
        );
        let search_node = search_node[0];
        let search_node = search_node.search_nodes("p").unwrap();
        assert_eq!(
            search_node,
            vec![
                &XMLNode::from(r#"<p>p-data</p>"#),
                &XMLNode::from(r#"<p>p-data-2</p>"#)
            ]
        );
    }
}
