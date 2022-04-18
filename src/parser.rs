#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    StartToken,
    EndToken,
    Character,
}
type TokenValue = String;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    token_type: TokenType,
    value: TokenValue,
}
impl Token {
    pub fn new() -> Self {
        Token {
            value: String::new(),
            token_type: TokenType::Character,
        }
    }
    pub fn new_source(token_type: TokenType, value: TokenValue) -> Self {
        Token { token_type, value }
    }
    pub fn create_token_array(s: &str) -> Vec<Token> {
        let mut token_array = Vec::new();
        let mut tmp_token = Token::new();
        for c in s.chars().filter(|c| !c.is_whitespace()) {
            match c {
                '<' => {
                    if token_array.len() > 0 && tmp_token.get_value() != "" {
                        token_array.push(tmp_token.clone());
                        tmp_token.clear_value();
                    }
                    tmp_token.change_type(TokenType::StartToken);
                }
                '/' => {
                    tmp_token.change_type(TokenType::EndToken);
                }
                '>' => {
                    token_array.push(tmp_token.clone());
                    tmp_token.clear_value();
                    tmp_token.change_type(TokenType::Character);
                }
                _ => tmp_token.add_char(c),
            }
        }
        token_array
    }
    pub fn add_char(&mut self, c: char) {
        self.value = format!("{}{}", self.value, c)
    }
    pub fn change_type(&mut self, token_type: TokenType) {
        self.token_type = token_type
    }
    pub fn get_value(&self) -> &TokenValue {
        &self.value
    }
    pub fn clear_value(&mut self) {
        self.value = "".to_string();
    }
}

#[cfg(test)]
mod token_test {
    use super::{Token, TokenType};

    #[test]
    fn create_token_array_test() {
        let data = "<div><p>p-data</p>div-data</div>";
        let token_array = Token::create_token_array(data);
        let div_start = Token::new_source(TokenType::StartToken, "div".to_string());
        let p_start = Token::new_source(TokenType::StartToken, "p".to_string());
        let p_char = Token::new_source(TokenType::Character, "p-data".to_string());
        let p_end = Token::new_source(TokenType::EndToken, "p".to_string());
        let div_char = Token::new_source(TokenType::Character, "div-data".to_string());
        let div_end = Token::new_source(TokenType::EndToken, "div".to_string());
        let test_array = vec![div_start, p_start, p_char, p_end, div_char, div_end];
        assert_eq!(token_array, test_array);
        let data = "<div>
            <p>p-data</p>
            div-data
        </div>";
        let token_array = Token::create_token_array(data);
        assert_eq!(token_array, test_array);
    }
}
type NodeValue = String;
#[derive(Debug, PartialEq, Eq)]
pub struct XMLNode {
    value: NodeValue,
    children: Box<Vec<XMLNode>>,
}

impl XMLNode {
    pub fn add_child(&mut self, child: XMLNode) {
        self.children.push(child)
    }
}

impl From<&str> for XMLNode {
    fn from(s: &str) -> Self {
        XMLNode {
            value: s.to_string(),
            children: Box::new(Vec::new()),
        }
    }
}
impl From<Token> for XMLNode {
    fn from(token: Token) -> Self {
        XMLNode {
            value: token.get_value().clone(),
            children: Box::new(Vec::new()),
        }
    }
}
impl From<Vec<Token>> for XMLNode {
    fn from(mut token_array: Vec<Token>) -> Self {
        let root = XMLNode::from(token_array.remove(0));
        let mut parent_stack = vec![root];
        for token in token_array {
            match token.token_type {
                TokenType::StartToken => parent_stack.push(XMLNode::from(token)),
                TokenType::Character => {
                    let child = XMLNode::from(token);
                    parent_stack.last_mut().unwrap().add_child(child)
                }
                TokenType::EndToken => {
                    let end_node = XMLNode::from(token);
                    if end_node.value != parent_stack.last().unwrap().value {
                        continue;
                    }
                    let child = parent_stack.pop();
                    match child {
                        Some(c) => {
                            if parent_stack.len() == 0 {
                                return c;
                            }
                            parent_stack.last_mut().unwrap().add_child(c)
                        }
                        None => panic!("error not parent_stack"),
                    }
                }
            }
        }
        panic!("error not end tag")
    }
}

#[cfg(test)]
mod xml_node_test {
    use super::{Token, XMLNode};
    #[test]
    fn from_token_array_test() {
        let data = "<div>
            <p>p-data</p>
            div-data
        </div>";
        let token_array = Token::create_token_array(data);
        let expect = XMLNode::from(token_array);
        let p_child = XMLNode::from("p-data");
        let mut p = XMLNode::from("p");
        p.add_child(p_child);
        let div_child = XMLNode::from("div-data");
        let mut div = XMLNode::from("div");
        div.add_child(p);
        div.add_child(div_child);
        assert_eq!(expect, div);
        let data = "<div><div>div-first
            <p>p-data</p>
            div-data</div>
        </div>";
        let token_array = Token::create_token_array(data);
        let expect = XMLNode::from(token_array);
        let p_child = XMLNode::from("p-data");
        let mut p = XMLNode::from("p");
        p.add_child(p_child);
        let div_child = XMLNode::from("div-data");
        let mut div = XMLNode::from("div");
        let mut child_div = XMLNode::from("div");
        let child_div_child = XMLNode::from("div-first");
        child_div.add_child(child_div_child);
        child_div.add_child(p);
        child_div.add_child(div_child);
        div.add_child(child_div);
        assert_eq!(expect, div);
        let data = "<div><div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
        </div>";
        let token_array = Token::create_token_array(data);
        let expect = XMLNode::from(token_array);
        let p_child = XMLNode::from("p-data");
        let mut p = XMLNode::from("p");
        p.add_child(p_child);
        let div_child = XMLNode::from("div-data");
        let mut div = XMLNode::from("div");
        let mut child_div = XMLNode::from("div");
        let child_div_child = XMLNode::from("div-first");
        child_div.add_child(child_div_child);
        child_div.add_child(p);
        child_div.add_child(div_child);
        div.add_child(child_div);
        assert_eq!(expect, div)
    }
}
