pub mod from_token {
    use std::collections::HashMap;

    use crate::xml::{
        nodes::node::XMLNode,
        tokens::{states::TokenType, token::Token},
    };

    pub fn token_to_node(token: Token) -> XMLNode {
        match token.get_token_type() {
            TokenType::StartToken => start_or_single_token_to_node(token),
            TokenType::SingleToken => start_or_single_token_to_node(token),
            _ => XMLNode::new(token.get_value()),
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
        XMLNode::new_with_element(&node_value, element.get_element())
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
