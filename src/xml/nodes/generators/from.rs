use std::usize;

use crate::xml::{
    nodes::{node::XMLNode, node_type::NodeType},
    tokens::{states::TokenType, token::Token, token_array::TokenArray},
};

impl<'a> From<&'a str> for XMLNode<'a> {
    fn from(s: &'a str) -> Self {
        let token_array = TokenArray::new(s);
        XMLNode::from(token_array)
    }
}
impl<'a> From<Token<'a>> for XMLNode<'a> {
    fn from(token: Token<'a>) -> Self {
        token_to_node(token)
    }
}
impl<'a> From<TokenArray<'a>> for XMLNode<'a> {
    fn from(token_array: TokenArray<'a>) -> Self {
        let token_array = token_array.token_array();
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
            single_parent.set_node_type(NodeType::SingleElement);
            return single_parent;
        }
        panic!("not had end tag this stack : {:?}", parent_stack)
    }
}

fn token_to_node<'a>(token: Token<'a>) -> XMLNode<'a> {
    match token.get_token_type() {
        TokenType::StartToken => start_or_single_token_to_node(token),
        TokenType::SingleToken => start_or_single_token_to_node(token),
        TokenType::Character => XMLNode::new(token.get_value(), NodeType::Character),
        _ => panic!("not consider end type"),
    }
}
fn start_or_single_token_to_node<'a>(token: Token<'a>) -> XMLNode<'a> {
    let mut element = Element::new();
    let mut prev_char = StartTokenPrevChar::new();
    let mut start_index = 0;
    let mut node_char_range = start_index..start_index;
    let source = token.get_value();
    let _ = source.chars().enumerate().for_each(|(i, c)| match c {
        ' ' => match prev_char {
            StartTokenPrevChar::NodeChar => {
                // case end of node-char
                node_char_range = start_index..i;
                prev_char.blank()
            }
            StartTokenPrevChar::ElementValue => {
                // blank means split element value
                element.tmp_push(&source.get(start_index..i).unwrap());
                prev_char.value_blank();
            }
            _ => {
                ();
            }
        },
        '"' => match prev_char {
            StartTokenPrevChar::ElementValue => {
                // case element-value derimita
                // and begin element-key
                // so push tmp-value
                // and push values
                element.tmp_push(&source.get(start_index..i).unwrap());
                element.values_push();
                prev_char.blank()
            }

            StartTokenPrevChar::Equal => prev_char.element_value(),
            _ => panic!(r#"error not parse before {} after ""#, c),
        },

        '=' => match prev_char {
            StartTokenPrevChar::ElementKey => {
                //  element.push()
                prev_char.equal();
            }
            StartTokenPrevChar::ElementValue => {
                // element.add_value(c);
            }
            StartTokenPrevChar::Blank => {}
            _ => {
                panic!(r#"not pattern to prev {} and next ="#, c)
            }
        },
        _ => match prev_char {
            StartTokenPrevChar::Blank => {
                // start element-key
                prev_char.element_key();
                start_index = i;
            }
            StartTokenPrevChar::Equal => {
                // start element-value
                prev_char.element_value();
                start_index = i;
            }
            StartTokenPrevChar::ElementValueBlank => {
                // start element-value
                prev_char.element_value();
                start_index = i;
            }
            //in the middle of prev
            _ => (),
        },
    });
    let node_type = match token.get_token_type() {
        TokenType::SingleToken => NodeType::SingleElement,
        TokenType::StartToken => NodeType::Element,
        _ => panic!("not consider end and character type"),
    };
    let mut node = XMLNode::new(&source.get(node_char_range).unwrap(), node_type);
    let mut key_values = element.key_values();
    key_values.iter_mut().for_each(|(key, values)| {
        node.add_element(*key, values.drain(..).collect());
    });
    node
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Element<'a> {
    keys: Vec<&'a str>,
    values: Vec<Vec<&'a str>>,
    tmp_values: Vec<&'a str>,
}
impl<'a> Element<'a> {
    pub fn new() -> Self {
        Element {
            keys: Vec::new(),
            values: Vec::new(),
            tmp_values: Vec::new(),
        }
    }
    pub fn key_push(&mut self, key: &'a str) {
        self.keys.push(key)
    }
    pub fn len(&self) -> usize {
        self.keys.len()
    }
    pub fn key_values(&mut self) -> Vec<(&'a str, Vec<&'a str>)> {
        let mut result = Vec::new();
        let _ = self
            .keys
            .iter()
            .for_each(|key| result.push((*key, self.values.remove(0))));
        result
    }
    pub fn values_push(&mut self) {
        self.values.push(self.tmp_values.drain(..).collect());
    }
    pub fn tmp_push(&mut self, value: &'a str) {
        self.tmp_values.push(value)
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
enum StartTokenPrevChar {
    NodeChar,
    ElementKey,
    ElementValue,
    ElementValueBlank,
    Equal,
    Blank,
}
impl StartTokenPrevChar {
    pub fn new() -> Self {
        StartTokenPrevChar::NodeChar
    }
    pub fn node_char(&mut self) {
        *self = StartTokenPrevChar::NodeChar
    }
    pub fn element_key(&mut self) {
        *self = StartTokenPrevChar::ElementKey
    }
    pub fn element_value(&mut self) {
        *self = StartTokenPrevChar::ElementValue
    }
    pub fn value_blank(&mut self) {
        *self = StartTokenPrevChar::ElementValueBlank
    }
    pub fn equal(&mut self) {
        *self = StartTokenPrevChar::Equal
    }
    pub fn blank(&mut self) {
        *self = StartTokenPrevChar::Blank
    }
}
#[cfg(test)]
mod token_to_node_tests {
    use crate::xml::{
        nodes::{node::XMLNode, node_type::NodeType},
        tokens::{states::TokenType, token::Token},
    };

    use super::token_to_node;

    #[test]
    fn token_to_node_case_element_test() {
        let token = Token::with_type("div", TokenType::StartToken);
        assert_eq!(token_to_node(token), XMLNode::new("div", NodeType::Element));
        let token = Token::with_type(
            r#"div id="kai" class="style style2""#,
            TokenType::StartToken,
        );
        let mut node = XMLNode::new("div", NodeType::Element);
        node.add_element("id", vec!["kai"]);
        node.add_element("class", vec!["style", "style2"]);
        assert_eq!(token_to_node(token), node)
    }
    #[test]
    fn token_to_node_case_single_test() {
        let token = Token::with_type("div", TokenType::SingleToken);
        assert_eq!(
            token_to_node(token),
            XMLNode::new("div", NodeType::SingleElement)
        );
    }
    #[test]
    fn token_to_node_case_charcter_test() {
        let token = Token::with_type("char", TokenType::Character);
        assert_eq!(
            token_to_node(token),
            XMLNode::new("char", NodeType::Character)
        );
    }
}
//#[cfg(test)]
//mod create_node {

//use crate::xml::{
//nodes::{node::XMLNode, node_type::NodeType},
//tokens::token_array::TokenArray,
//};
//#[test]
//fn from_token_array_test() {
//let data = "<div>
//<p>p-data</p>
//div-data
//</div>";
//let token_array = TokenArray::new(data);
//let expect = XMLNode::from(token_array);
//let mut p = XMLNode::new("p", NodeType::Element);
//p.add_text("p-data");
//let mut div = XMLNode::new("div", NodeType::Element);
//div.add_node(p);
//div.add_text("div-data");
//assert_eq!(expect, div);
//let data = "<div><div>div-first
//<p>p-data</p>
//div-data</div>
//</div>";
//let token_array = TokenArray::new(data);
//let expect = XMLNode::from(token_array);
//let mut p = XMLNode::new("p", NodeType::Element);
//p.add_text("p-data");
//let mut div = XMLNode::new("div", NodeType::Element);
//let mut child_div = XMLNode::new("div", NodeType::Element);
//child_div.add_text("div-first");
//child_div.add_node(p);
//child_div.add_text("div-data");
//div.add_node(child_div);
//assert_eq!(expect, div);
//let data = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
//<div><div>div-first
//<p>p-data</p>
//<data/>
//div-data</div>
//</div>"#;
//let expect = XMLNode::from(data);
//let mut root = XMLNode::new("?xml", NodeType::SingleElement);
//root.add_element("standalone", vec![r#"yes"#]);
//root.add_element("encoding", vec![r#"UTF-8"#]);
//root.add_element("version", vec![r#"1.0"#]);
//let mut p = XMLNode::new("p", NodeType::Element);
//p.add_text("p-data");
//let single_data = XMLNode::new("data", NodeType::SingleElement);
//let mut div = XMLNode::new("div", NodeType::Element);
//let mut child_div = XMLNode::new("div", NodeType::Element);
//child_div.add_text("div-first");
//child_div.add_node(p);
//child_div.add_node(single_data);
//child_div.add_text("div-data");
//div.add_node(child_div);
//root.add_node(div);
//assert_eq!(expect, root)
//}
//#[test]
//fn element_test() {
//let data = r#"<div id="1180" name="kai"><div>div-first
//<p>p-data</p>
//<data/>
//div-data</div>
//</div>"#;

//let token_array = TokenArray::new(data);
//let expect = XMLNode::from(token_array);
//let mut p = XMLNode::new("p", NodeType::Element);
//p.add_text("p-data");
//let single_data = XMLNode::new("data", NodeType::SingleElement);
//let mut div = XMLNode::new("div", NodeType::Element);
//div.add_element("name", vec!["kai"]);
//div.add_element("id", vec!["1180"]);
//let mut child_div = XMLNode::new("div", NodeType::Element);
//child_div.add_text("div-first");
//child_div.add_node(p);
//child_div.add_node(single_data);
//child_div.add_text("div-data");
//div.add_node(child_div);
//assert_eq!(expect, div);
//let data = r#"<div id="1180" name="kai" class="style1 style2"><div>div-first
//<p>p-data</p>
//<data/>
//div-data</div>
//</div>"#;
//let token_array = TokenArray::new(data);
//let expect = XMLNode::from(token_array);
//let mut p = XMLNode::new("p", NodeType::Element);
//p.add_text("p-data");
//let single_data = XMLNode::new("data", NodeType::SingleElement);
//let mut div = XMLNode::new("div", NodeType::Element);
//div.add_element("name", vec!["kai"]);
//div.add_element("id", vec!["1180"]);
//div.add_element("class", vec!["style1 style2"]);
//let mut child_div = XMLNode::new("div", NodeType::Element);
//child_div.add_text("div-first");
//child_div.add_node(p);
//child_div.add_node(single_data);
//child_div.add_text("div-data");
//div.add_node(child_div);
//assert_eq!(expect, div)
//}
//}

#[cfg(test)]
mod token_array_test {

    use crate::xml::{
        nodes::{node::XMLNode, node_type::NodeType},
        tokens::token_array::TokenArray,
    };

    #[test]
    fn from_token_array_test() {
        let data = "<div><div>div-first
<p>p-data</p>
div-data</div>
</div>";
        let token_array = TokenArray::new(data);
        let expect = XMLNode::from(token_array);
        let mut p = XMLNode::new("p", NodeType::Element);
        p.add_text("p-data");
        let mut div = XMLNode::new("div", NodeType::Element);
        let mut child_div = XMLNode::new("div", NodeType::Element);
        child_div.add_text("div-first");
        child_div.add_node(p);
        child_div.add_text("div-data");
        div.add_node(child_div);
        assert_eq!(expect, div);
        let data = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                                <div>
                                    <div>
                                        div-first
                                        <p>p-data</p>
                                        <data/>
                                        div-data
                                    </div>
                                </div>"#;
        let expect = XMLNode::from(data);
        let mut root = XMLNode::new("?xml", NodeType::SingleElement);
        root.add_element("version", vec!["1.0"]);
        root.add_element("encoding", vec!["UTF-8"]);
        root.add_element("standalone", vec!["yes"]);
        let mut p = XMLNode::new("p", NodeType::Element);
        p.add_text("p-data");
        let single_data = XMLNode::new("data", NodeType::SingleElement);
        let mut div = XMLNode::new("div", NodeType::Element);
        let mut child_div = XMLNode::new("div", NodeType::Element);
        child_div.add_text("div-first");
        child_div.add_node(p);
        child_div.add_node(single_data);
        child_div.add_text("div-data");
        div.add_node(child_div);
        root.add_node(div);
        assert_eq!(expect, root)
    }
}
