use crate::xml::tokens::{states::TokenType, token::Token, token_array::TokenArray};

use super::{
    funcs::from_token::token_to_node,
    parts::{NodeElement, NodeValue},
};

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
    pub fn get_child(&self) -> &Option<Box<Vec<XMLNode>>> {
        &self.children
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn element_all(&self, key: &str, value: &str) -> Option<Vec<&XMLNode>> {
        if self.has_children() {
            let maybe = self
                .children
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
        match self.get_child() {
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
                        Some(node) => {
                            if parent_stack.len() == 0 {
                                return node;
                            }
                            parent_stack.last_mut().unwrap().add_child(node)
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
