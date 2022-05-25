use crate::xml::{
    nodes::{node::XMLNode, node_type::NodeType},
    tokens::token::{Token, TokenType},
};

impl<'a> From<Token<'a>> for XMLNode<'a> {
    fn from(token: Token<'a>) -> Self {
        token_to_node(token)
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

            StartTokenPrevChar::Equal => {
                prev_char.element_value();
                start_index = i + 1
            }
            _ => panic!(r#"error not parse before {} after ""#, c),
        },

        '=' => match prev_char {
            StartTokenPrevChar::ElementKey => {
                //  element.push()
                element.key_push(source.get(start_index..i).unwrap());
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
                // split
                //println!("tmp_push3 {}", &source.get(start_index..i).unwrap());
                //element.tmp_push(source.get(start_index..i).unwrap());
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
    if start_index == 0 {
        node_char_range = 0..(source.len())
    }
    if prev_char == StartTokenPrevChar::ElementKey {
        element.key_push(source.get(start_index..source.len()).unwrap());
        element.empty_push();
    }
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
    pub fn empty_push(&mut self) {
        self.values.push(vec![]);
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
        tokens::token::{Token, TokenType},
    };

    use super::token_to_node;
    #[test]
    fn token_to_node_case_only_element_key_test() {
        let token = Token::with_type(
            r#"div id="kai" class="style style2" only"#,
            TokenType::StartToken,
        );
        let mut node = XMLNode::new("div", NodeType::Element);
        node.add_element("id", vec!["kai"]);
        node.add_element("class", vec!["style", "style2"]);
        node.add_element("only", vec![]);
        assert_eq!(token_to_node(token), node)
    }
    #[test]
    fn token_to_node_case_element_test() {
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
