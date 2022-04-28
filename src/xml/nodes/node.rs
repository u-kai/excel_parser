use crate::xml::tokens::{states::TokenType, token::Token, token_array::TokenArray};

use super::{
    funcs::from_token::token_to_node,
    parts::{NodeElement, NodeValue},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NodeType {
    Element,
    SingleElement,
    Character,
}
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
    pub fn get_child_charcters(&self) -> Option<Vec<&str>> {
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
    pub fn get_child_charcter(&self, n: usize) -> Option<&str> {
        let maybe_charcters = self.get_child_charcters();
        if maybe_charcters.is_some() {
            return maybe_charcters.unwrap().get(n).map(|c| *c);
        }
        None
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
    pub fn search_nodes(&self, search_value: &str) -> Option<Vec<&XMLNode>> {
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
    // search_child_rec is search all children that one parent has.
    //
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
impl From<&str> for XMLNode {
    fn from(s: &str) -> Self {
        let token_array = TokenArray::new(s);
        XMLNode::from(token_array)
    }
}
impl From<Token> for XMLNode {
    fn from(token: Token) -> Self {
        token_to_node(token)
    }
}
impl From<TokenArray> for XMLNode {
    fn from(token_array: TokenArray) -> Self {
        let token_array = token_array.drain();
        let mut parent_stack = Vec::new();
        for token in token_array {
            match token.get_token_type() {
                TokenType::StartToken => parent_stack.push(XMLNode::from(token)),
                TokenType::Character => parent_stack
                    .last_mut()
                    .unwrap()
                    .add_charcter(token.get_value()),
                TokenType::SingleToken => {
                    let node = XMLNode::from(token);
                    parent_stack.last_mut().unwrap().add_node(node);
                }
                TokenType::EndToken => {
                    let child = parent_stack.pop();
                    match child {
                        Some(node) => {
                            if parent_stack.len() == 0 {
                                return node;
                            }
                            parent_stack.last_mut().unwrap().add_node(node)
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
