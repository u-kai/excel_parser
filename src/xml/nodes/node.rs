use crate::xml::tokens::{states::TokenType, token::Token, token_array::TokenArray};

use super::{
    funcs::from_token::token_to_node,
    parts::{NodeElement, NodeValue},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChildrenNode {
    nodes: Option<Box<Vec<XMLNode>>>,
    characters: Option<Vec<String>>,
}
impl ChildrenNode {
    pub fn new() -> Self {
        ChildrenNode {
            nodes: None,
            characters: None,
        }
    }
    pub fn add_charcters(&mut self, s: &str) {
        if self.has_characters() {
            self.characters.as_mut().unwrap().push(s.to_string());
            return;
        }
        self.characters = Some(vec![s.to_string()])
    }
    pub fn add_node(&mut self, node: XMLNode) {
        if self.has_nodes() {
            self.nodes.as_mut().unwrap().push(node);
            return;
        }
        self.nodes = Some(Box::new(vec![node]));
    }
    pub fn init_characters(&mut self) {
        self.characters = None;
    }
    pub fn init_nodes(&mut self) {
        self.nodes = None;
    }
    pub fn get_nodes(&self) -> &Option<Box<Vec<XMLNode>>> {
        &self.nodes
    }
    pub fn get_charcters(&self) -> &Option<Vec<String>> {
        &self.characters
    }
    pub fn get_n_node(&self, n: usize) -> Option<&XMLNode> {
        if self.has_nodes() {
            self.get_nodes().as_ref().unwrap().get(n)
        } else {
            None
        }
    }
    pub fn get_n_charcters(&self, n: usize) -> Option<&String> {
        if self.has_characters() {
            self.get_charcters().as_ref().unwrap().get(n)
        } else {
            None
        }
    }
    fn has_nodes(&self) -> bool {
        self.nodes.is_some()
    }
    fn has_characters(&self) -> bool {
        self.characters.is_some()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct XMLNode {
    value: NodeValue,
    children: ChildrenNode,
}

impl XMLNode {
    pub fn new(s: &str) -> Self {
        XMLNode {
            value: NodeValue::new(s),
            children: ChildrenNode::new(),
        }
    }
    pub fn get_child_nodes(&self) -> &Option<Box<Vec<XMLNode>>> {
        &self.children.get_nodes()
    }
    pub fn new_with_element(s: &str, element: Option<NodeElement>) -> Self {
        if element.is_some() {
            let mut node = XMLNode::new(s);
            node.value.set_element(element.unwrap());
            node
        } else {
            XMLNode::new(s)
        }
    }
    pub fn add_node(&mut self, child: XMLNode) {
        self.children.add_node(child);
    }
    pub fn search_node(&self, search_value: &str) -> Option<&XMLNode> {
        if self.children.has_nodes() {
            return self
                .get_child_nodes()
                .as_ref()
                .unwrap()
                .iter()
                .filter(|child| child.get_value() == search_value)
                .nth(0);
        }
        None
    }
    #[allow(dead_code)]
    pub fn search_nodes(&self, search_value: &str) -> Option<Vec<&XMLNode>> {
        if self.children.has_nodes() {
            return Some(
                self.get_child_nodes()
                    .as_ref()
                    .unwrap()
                    .iter()
                    .filter(|child| child.get_value() == search_value)
                    .collect(),
            );
        }
        None
    }
    #[allow(dead_code)]
    pub fn nth_child_node(&mut self, n: usize) -> Option<&XMLNode> {
        if self.children.has_nodes() {
            let result = self.children.get_n_node(n);
            return result;
        }
        None
    }
    #[allow(dead_code)]
    pub fn element_all(&self, key: &str, value: &str) -> Option<Vec<&XMLNode>> {
        if self.children.has_nodes() {
            let maybe = self
                .get_child_nodes()
                .as_ref()
                .unwrap()
                .iter()
                .filter(|node| node.is_containe_key_value(key, value))
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
                TokenType::Character => {
                    let child = XMLNode::from(token);
                    parent_stack.last_mut().unwrap().add_node(child)
                }
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