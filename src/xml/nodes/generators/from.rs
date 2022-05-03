use std::collections::HashMap;

use crate::xml::{
    nodes::{node::XMLNode, node_type::NodeType},
    tokens::{states::TokenType, token::Token, token_array::TokenArray},
};

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
                    parent_stack.last_mut().unwrap().add_text(token.get_value())
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
            let mut single_parent = parent_stack.pop().unwrap();
            single_parent.node_type = NodeType::SingleElement;
            return single_parent;
        }
        panic!("not had end tag this stack : {:?}", parent_stack)
    }
}

fn token_to_node(token: Token) -> XMLNode {
    match token.get_token_type() {
        TokenType::StartToken => start_or_single_token_to_node(token),
        TokenType::SingleToken => start_or_single_token_to_node(token),
        TokenType::Character => XMLNode::new(token.get_value(), NodeType::Character),
        _ => panic!("not consider end type"),
    }
}
fn start_or_single_token_to_node(token: Token) -> XMLNode {
    let mut prev_char = StartTokenPrevChar::NodeChar;
    let mut node_value = String::new();
    let mut element = Element::new();

    for c in token.get_value().chars() {
        match c {
            ' ' => match prev_char {
                StartTokenPrevChar::NodeChar => prev_char = StartTokenPrevChar::Blank,
                StartTokenPrevChar::ElementValue => {
                    element.add_value_buffer();
                }
                _ => {
                    ();
                }
            },
            '"' => match prev_char {
                StartTokenPrevChar::ElementValue => {
                    element.add_element();
                    prev_char = StartTokenPrevChar::ElementKey
                }
                StartTokenPrevChar::Equal => prev_char = StartTokenPrevChar::ElementValue,
                _ => panic!(r#"error not parse before {} after ""#, c),
            },

            '=' => match prev_char {
                StartTokenPrevChar::ElementKey => prev_char = StartTokenPrevChar::Equal,
                StartTokenPrevChar::ElementValue => {
                    element.add_value(c);
                }
                StartTokenPrevChar::Blank => {}
                _ => {
                    panic!(r#"not pattern to prev {} and next ="#, c)
                }
            },
            _ => match prev_char {
                StartTokenPrevChar::NodeChar => {
                    node_value.push(c);
                }
                StartTokenPrevChar::Blank => {
                    prev_char = StartTokenPrevChar::ElementKey;
                    element.add_key(c);
                }

                StartTokenPrevChar::ElementKey => {
                    element.add_key(c);
                }
                StartTokenPrevChar::ElementValue => {
                    element.add_value(c);
                }
                StartTokenPrevChar::Equal => {
                    prev_char = StartTokenPrevChar::ElementValue;
                    element.add_value(c);
                }
            },
        }
    }
    let node_type = match token.get_token_type() {
        TokenType::SingleToken => NodeType::SingleElement,
        TokenType::StartToken => NodeType::Element,
        _ => panic!("not consider end and character type"),
    };
    XMLNode::new_with_element(&node_value, element.get_element(), node_type)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum StartTokenPrevChar {
    NodeChar,
    ElementKey,
    ElementValue,
    Equal,
    Blank,
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Element {
    key: String,
    value: String,
    value_buffer: Vec<String>,
    hash_map: HashMap<String, Vec<String>>,
}
impl Element {
    pub fn new() -> Self {
        Element {
            key: String::new(),
            value: String::new(),
            value_buffer: Vec::new(),
            hash_map: HashMap::new(),
        }
    }
    pub fn get_element(self) -> Option<HashMap<String, Vec<String>>> {
        if self.hash_map.is_empty() {
            None
        } else {
            Some(self.hash_map)
        }
    }
    pub fn add_element(&mut self) {
        self.add_value_buffer();
        self.hash_map.insert(
            self.key.drain(..).collect(),
            self.value_buffer.drain(..).collect(),
        );
    }
    pub fn add_value_buffer(&mut self) {
        self.value_buffer.push(self.value.drain(..).collect());
    }
    pub fn add_value(&mut self, c: char) {
        self.value.push(c)
    }
    pub fn add_key(&mut self, c: char) {
        self.key.push(c)
    }
}