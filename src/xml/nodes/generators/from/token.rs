use crate::xml::{
    nodes::{node::XMLNode, node_type::NodeType},
    tokens::token::{Token, TokenType},
};

impl<'a> From<Token<'a>> for XMLNode<'a> {
    fn from(token: Token<'a>) -> Self {
        token_to_node(token)
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Element<'a> {
    keys: Vec<&'a str>,
    values: Vec<Vec<&'a str>>,
    tmvalues: Vec<&'a str>,
}
impl<'a> Element<'a> {
    pub fn new() -> Self {
        Element {
            keys: Vec::new(),
            values: Vec::new(),
            tmvalues: Vec::new(),
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
            .enumerate()
            .for_each(|(_, key)| result.push((*key, self.values.remove(0))));
        result
    }
    pub fn values_push(&mut self) {
        self.values.push(self.tmvalues.drain(..).collect());
    }
    pub fn tmpush(&mut self, value: &'a str) {
        self.tmvalues.push(value)
    }
    pub fn empty_push(&mut self) {
        self.values.push(vec![]);
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
enum StateMachine {
    ValueBlank,
    ValueChar,
    EleKeyBlank,
    EleKeyChar,
    EleValBlank,
    EleValStart,
    EleValChar,
    EleValSplit,
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
    let mut start_index = 0;
    let mut node_char_range = start_index..start_index;
    let mut state = StateMachine::ValueBlank;
    let source = token.get_value();
    source.bytes().enumerate().for_each(|(i, c)| match state {
        StateMachine::ValueBlank => {
            if c.is_ascii_whitespace() {
                start_index += 1;
                return;
            }
            state = StateMachine::ValueChar
        }
        StateMachine::ValueChar => {
            if c.is_ascii_whitespace() {
                node_char_range = start_index..i;
                state = StateMachine::EleKeyBlank
            }
        }
        StateMachine::EleKeyBlank => {
            if !(c.is_ascii_whitespace()) {
                start_index = i;
                state = StateMachine::EleKeyChar;
            }
        }
        StateMachine::EleKeyChar => {
            if c.is_ascii_whitespace() {
                element.key_push(source.get(start_index..i).unwrap());
                state = StateMachine::EleKeyBlank;
                return;
            }
            if c == 61 {
                element.key_push(source.get(start_index..i).unwrap());
                state = StateMachine::EleValBlank;
            }
        }
        StateMachine::EleValBlank => {
            if c == 34 {
                start_index = i + 1;
                state = StateMachine::EleValStart;
            }
        }
        StateMachine::EleValStart => {
            if !(c.is_ascii_whitespace()) {
                start_index = i;
                state = StateMachine::EleValChar;
            }
        }
        StateMachine::EleValChar => {
            if c == 34 {
                element.tmpush(source.get(start_index..i).unwrap());
                element.values_push();
                state = StateMachine::EleKeyBlank;
                return;
            }
            if c.is_ascii_whitespace() {
                element.tmpush(source.get(start_index..i).unwrap());
                state = StateMachine::EleValSplit;
            }
        }
        StateMachine::EleValSplit => {
            if c == 34 {
                element.values_push();
                state = StateMachine::EleKeyBlank;
            }
            if !(c.is_ascii_whitespace()) {
                start_index = i;
                state = StateMachine::EleValChar;
                return;
            }
        }
    });
    let node_type = match token.get_token_type() {
        TokenType::SingleToken => NodeType::SingleElement,
        TokenType::StartToken => NodeType::Element,
        _ => panic!("not consider end and character type"),
    };
    if start_index == 0 {
        node_char_range = 0..(source.len())
    }
    if state == StateMachine::EleKeyChar {
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
    #[test]
    fn token_to_node_case_workbook_test() {
        let token = Token::with_type(
            r#"workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="x15 xr xr6 xr10 xr2" xmlns:x15="http://schemas.microsoft.com/office/spreadsheetml/2010/11/main" xmlns:xr="http://schemas.microsoft.com/office/spreadsheetml/2014/revision" xmlns:xr6="http://schemas.microsoft.com/office/spreadsheetml/2016/revision6" xmlns:xr10="http://schemas.microsoft.com/office/spreadsheetml/2016/revision10" xmlns:xr2="http://schemas.microsoft.com/office/spreadsheetml/2015/revision2""#,
            TokenType::StartToken,
        );
        let mut node = XMLNode::new("workbook", NodeType::Element);
        node.add_element(
            "xmlns",
            vec!["http://schemas.openxmlformats.org/spreadsheetml/2006/main"],
        );
        node.add_element(
            "xmlns:r",
            vec!["http://schemas.openxmlformats.org/officeDocument/2006/relationships"],
        );
        node.add_element(
            "xmlns:mc",
            vec!["http://schemas.openxmlformats.org/markup-compatibility/2006"],
        );
        node.add_element("mc:Ignorable", vec!["x15", "xr", "xr6", "xr10", "xr2"]);
        node.add_element(
            "xmlns:x15",
            vec!["http://schemas.microsoft.com/office/spreadsheetml/2010/11/main"],
        );
        node.add_element(
            "xmlns:xr",
            vec!["http://schemas.microsoft.com/office/spreadsheetml/2014/revision"],
        );
        node.add_element(
            "xmlns:xr6",
            vec!["http://schemas.microsoft.com/office/spreadsheetml/2016/revision6"],
        );
        node.add_element(
            "xmlns:xr10",
            vec!["http://schemas.microsoft.com/office/spreadsheetml/2016/revision10"],
        );
        node.add_element(
            "xmlns:xr2",
            vec!["http://schemas.microsoft.com/office/spreadsheetml/2015/revision2"],
        );

        assert_eq!(token_to_node(token), node);
    }
}
