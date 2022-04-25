use super::{
    states::{PrevChar, PrevCharcter, TokenType},
    token::Token,
};

#[derive(Debug)]
pub struct TokenArray {
    value: Vec<Token>,
    tmp_token: Token,
    prev_char: PrevCharcter,
}
impl TokenArray {
    pub fn new(s: &str) -> Self {
        let token_array = TokenArray::_new();
        token_array._build(s)
    }
    pub fn drain(self) -> Vec<Token> {
        self.value
    }
    fn _new() -> Self {
        TokenArray {
            value: Vec::new(),
            tmp_token: Token::new(),
            prev_char: PrevCharcter::new(),
        }
    }
    fn _build(mut self, s: &str) -> Self {
        s.chars().for_each(|c| match c {
            '<' => {
                if !(self.tmp_token.is_empty_value()) {
                    self.value.push(self.tmp_token.drain());
                }
                self.tmp_token.change_start();
                self.prev_char.change_start_tag();
            }
            '/' => {
                match self.prev_char.get_prev_char() {
                    PrevChar::StartTag => self.tmp_token.change_end(),
                    PrevChar::Slash => self.tmp_token.add_char('/'),
                    PrevChar::EndTag => self.tmp_token.add_char(c),
                    PrevChar::Character => {}
                }
                self.prev_char.change_slash();
            }
            '>' => {
                if self.prev_char.get_prev_char() == PrevChar::Slash {
                    self.tmp_token.change_single();
                }
                self.prev_char.change_end_tag();
                self.value.push(self.tmp_token.drain());
                self.tmp_token.change_character()
            }
            _ => {
                if self.tmp_token.is_add_slash(self.prev_char.get_prev_char()) {
                    self.tmp_token.add_char('/');
                }
                match self.tmp_token.get_token_type() {
                    TokenType::Character => {
                        if !(c.is_whitespace()) {
                            self.tmp_token.add_char(c)
                        }
                    }
                    _ => self.tmp_token.add_char(c),
                }
                self.prev_char.change_character()
            }
        });
        self
    }
}

#[cfg(test)]
mod create_node {
    use std::collections::HashMap;

    use crate::xml::nodes::node::NodeType;
    use crate::xml::{nodes::node::XMLNode, tokens::token_array::TokenArray};
    #[test]
    fn from_token_array_test() {
        let data = "<div>
            <p>p-data</p>
            div-data
        </div>";
        let token_array = TokenArray::new(data);
        let expect = XMLNode::from(token_array);
        let mut p = XMLNode::new("p", NodeType::Element);
        p.add_charcter("p-data");
        let mut div = XMLNode::new("div", NodeType::Element);
        div.add_node(p);
        div.add_charcter("div-data");
        assert_eq!(expect, div);
        let data = "<div><div>div-first
            <p>p-data</p>
            div-data</div>
        </div>";
        let token_array = TokenArray::new(data);
        let expect = XMLNode::from(token_array);
        let mut p = XMLNode::new("p", NodeType::Element);
        p.add_charcter("p-data");
        let mut div = XMLNode::new("div", NodeType::Element);
        let mut child_div = XMLNode::new("div", NodeType::Element);
        child_div.add_charcter("div-first");
        child_div.add_node(p);
        child_div.add_charcter("div-data");
        div.add_node(child_div);
        assert_eq!(expect, div);
        let data = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
        <div><div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
        </div>"#;
        let expect = XMLNode::from(data);
        let mut root = XMLNode::new("?xml", NodeType::Element);
        let mut root_element = HashMap::new();
        root_element.insert("standalone".to_string(), vec![r#"yes"#.to_string()]);
        root_element.insert("encoding".to_string(), vec![r#"UTF-8"#.to_string()]);
        root_element.insert("version".to_string(), vec![r#"1.0"#.to_string()]);

        root.get_node_value().set_element(root_element);
        let mut p = XMLNode::new("p", NodeType::Element);
        p.add_charcter("p-data");
        let single_data = XMLNode::new("data", NodeType::SingleElement);
        let mut div = XMLNode::new("div", NodeType::Element);
        let mut child_div = XMLNode::new("div", NodeType::Element);
        child_div.add_charcter("div-first");
        child_div.add_node(p);
        child_div.add_node(single_data);
        child_div.add_charcter("div-data");
        div.add_node(child_div);
        root.add_node(div);
        assert_eq!(expect, root)
    }
    #[test]
    fn element_test() {
        let data = r#"<div id="1180" name="kai"><div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
        </div>"#;

        let token_array = TokenArray::new(data);
        let expect = XMLNode::from(token_array);
        let mut p = XMLNode::new("p", NodeType::Element);
        p.add_charcter("p-data");
        let single_data = XMLNode::new("data", NodeType::Element);
        let mut div = XMLNode::new("div", NodeType::Element);
        let mut element = HashMap::new();

        element.insert("name".to_string(), vec![r#"kai"#.to_string()]);
        element.insert("id".to_string(), vec![r#"1180"#.to_string()]);
        div.get_node_value().set_element(element);
        let mut child_div = XMLNode::new("div", NodeType::Element);
        child_div.add_charcter("div-first");
        child_div.add_node(p);
        child_div.add_node(single_data);
        child_div.add_charcter("div-data");
        div.add_node(child_div);
        assert_eq!(expect, div);
        let data = r#"<div id="1180" name="kai" class="style1 style2"><div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
        </div>"#;
        let token_array = TokenArray::new(data);
        let expect = XMLNode::from(token_array);
        let mut p = XMLNode::new("p", NodeType::Element);
        p.add_charcter("p-data");
        let single_data = XMLNode::new("data", NodeType::SingleElement);
        let mut div = XMLNode::new("div", NodeType::Element);
        let mut element = HashMap::new();

        element.insert("name".to_string(), vec![r#"kai"#.to_string()]);
        element.insert("id".to_string(), vec![r#"1180"#.to_string()]);
        element.insert(
            "class".to_string(),
            vec!["style1".to_string(), "style2".to_string()],
        );
        div.get_node_value().set_element(element);
        let mut child_div = XMLNode::new("div", NodeType::Element);
        child_div.add_charcter("div-first");
        child_div.add_node(p);
        child_div.add_node(single_data);
        child_div.add_charcter("div-data");
        div.add_node(child_div);
        assert_eq!(expect, div)
    }
}
