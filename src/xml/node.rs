use std::collections::HashMap;

use self::from_token::token_to_node;

use super::token::{PrevChar, Token, TokenType};
type NodeElement = HashMap<String, Vec<String>>;
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
pub struct TokenArray {
    value: Vec<Token>,
    tmp_token: Token,
    prev_char: PrevCharcter,
}
struct PrevCharcter {
    value: PrevChar,
}
impl PrevCharcter {
    pub fn new() -> Self {
        PrevCharcter {
            value: PrevChar::Character,
        }
    }
    pub fn get_prev_char(&self) -> PrevChar {
        self.value
    }
    pub fn change_character(&mut self) {
        self.value = PrevChar::Character;
    }
    pub fn change_start_tag(&mut self) {
        self.value = PrevChar::StartTag;
    }
    pub fn change_end_tag(&mut self) {
        self.value = PrevChar::EndTag;
    }
    pub fn change_slash(&mut self) {
        self.value = PrevChar::Slash;
    }
}
impl TokenArray {
    pub fn new(s: &str) -> Self {
        let token_array = TokenArray::_new();
        token_array._build(s)
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
                    PrevChar::EndTag => {
                        self.tmp_token.add_char(c);
                    }
                    PrevChar::Character => {
                        ();
                    }
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
    pub fn drain(self) -> Vec<Token> {
        self.value
    }
}
impl From<&str> for XMLNode {
    fn from(s: &str) -> Self {
        let token_array = TokenArray::new(s);
        XMLNode::from(token_array.drain())
    }
}
impl From<Token> for XMLNode {
    fn from(token: Token) -> Self {
        token_to_node(token)
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

mod from_token {
    use super::{NodeValue, XMLNode};
    use crate::xml::token::{Token, TokenType};
    use std::collections::HashMap;

    pub fn token_to_node(token: Token) -> XMLNode {
        match token.get_token_type() {
            TokenType::Character => XMLNode::new(token.get_value()),
            TokenType::StartToken => start_or_single_token_to_node(token),
            TokenType::SingleToken => start_or_single_token_to_node(token),
            TokenType::EndToken => XMLNode {
                value: NodeValue {
                    value: token.get_value().to_string(),
                    element: None,
                },
                children: None,
            },
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
        XMLNode {
            value: {
                NodeValue {
                    value: node_value,
                    element: element.get_element(),
                }
            },
            children: None,
        }
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
}

#[cfg(test)]
mod xml_node_test {
    use std::collections::HashMap;

    use crate::xml::node::TokenArray;

    use super::XMLNode;
    #[test]
    fn from_token_array_test() {
        let data = "<div>
            <p>p-data</p>
            div-data
        </div>";
        let mut token_array = TokenArray::new(data);
        let expect = XMLNode::from(token_array.drain());
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
        let mut token_array = TokenArray::new(data);
        let expect = XMLNode::from(token_array.drain());
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
        root_element.insert("standalone".to_string(), vec![r#"yes"#.to_string()]);
        root_element.insert("encoding".to_string(), vec![r#"UTF-8"#.to_string()]);
        root_element.insert("version".to_string(), vec![r#"1.0"#.to_string()]);
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

        let mut token_array = TokenArray::new(data);
        let expect = XMLNode::from(token_array.drain());
        let p_child = XMLNode::new("p-data");
        let mut p = XMLNode::new("p");
        p.add_child(p_child);
        let div_child = XMLNode::new("div-data");
        let single_data = XMLNode::new("data");
        let mut div = XMLNode::new("div");
        let mut element = HashMap::new();

        element.insert("name".to_string(), vec![r#"kai"#.to_string()]);
        element.insert("id".to_string(), vec![r#"1180"#.to_string()]);
        div.value.element = Some(element);
        let mut child_div = XMLNode::new("div");
        let child_div_child = XMLNode::new("div-first");
        child_div.add_child(child_div_child);
        child_div.add_child(p);
        child_div.add_child(single_data);
        child_div.add_child(div_child);
        div.add_child(child_div);
        assert_eq!(expect, div);
        let data = r#"<div id="1180" name="kai" class="style1 style2"><div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
        </div>"#;
        let mut token_array = TokenArray::new(data);
        let expect = XMLNode::from(token_array.drain());
        let p_child = XMLNode::new("p-data");
        let mut p = XMLNode::new("p");
        p.add_child(p_child);
        let div_child = XMLNode::new("div-data");
        let single_data = XMLNode::new("data");
        let mut div = XMLNode::new("div");
        let mut element = HashMap::new();

        element.insert("name".to_string(), vec![r#"kai"#.to_string()]);
        element.insert("id".to_string(), vec![r#"1180"#.to_string()]);
        element.insert(
            "class".to_string(),
            vec!["style1".to_string(), "style2".to_string()],
        );
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
    #[test]

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
            <p>p-data-2</p>
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
