#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType {
    StartToken,
    EndToken,
    Character,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PreviousCharState {
    StartTag,
    EndTag,
    Character,
    Slash,
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
        let mut slash_buffer = Vec::new();
        let mut prev_char_state = PreviousCharState::Character;
        for c in s.chars() {
            match c {
                '<' => {
                    if token_array.len() > 0 && tmp_token.get_value() != "" {
                        token_array.push(tmp_token.clone());
                        tmp_token.clear_value();
                    }
                    prev_char_state = PreviousCharState::StartTag;
                    tmp_token.change_type(TokenType::StartToken);
                }
                '/' => {
                    match prev_char_state {
                        PreviousCharState::StartTag => tmp_token.change_type(TokenType::EndToken),
                        PreviousCharState::Slash => tmp_token.add_char(slash_buffer.pop().unwrap()),
                        PreviousCharState::EndTag => {
                            tmp_token.add_char(c);
                        }
                        PreviousCharState::Character => {
                            ();
                        }
                    }
                    prev_char_state = PreviousCharState::Slash;
                    slash_buffer.push(c);
                }
                '>' => {
                    if prev_char_state == PreviousCharState::Slash {
                        tmp_token.change_type(TokenType::EndToken);
                        slash_buffer.pop();
                    }
                    prev_char_state = PreviousCharState::EndTag;
                    token_array.push(tmp_token.clone());
                    tmp_token.clear_value();
                    tmp_token.change_type(TokenType::Character);
                }
                _ => {
                    if prev_char_state == PreviousCharState::Slash
                        && tmp_token.get_token_type() != &TokenType::EndToken
                    {
                        tmp_token.add_char(slash_buffer.pop().unwrap());
                    }
                    match tmp_token.get_token_type() {
                        TokenType::Character => {
                            if !(c.is_whitespace()) {
                                tmp_token.add_char(c)
                            }
                        }
                        TokenType::StartToken => tmp_token.add_char(c),
                        TokenType::EndToken => tmp_token.add_char(c),
                    }
                    prev_char_state = PreviousCharState::Character;
                }
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
    pub fn get_token_type(&self) -> &TokenType {
        &self.token_type
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
    #[test]
    fn confirm_slash_test() {
        let source = r#"
        <div>
        helloworld
        <a href="http://localhost:8000"/>
        </div>
        "#;
        let start_root_token = Token::new_source(TokenType::StartToken, "div".to_string());
        let char_token = Token::new_source(TokenType::Character, "helloworld".to_string());
        let root_token = Token::new_source(
            TokenType::EndToken,
            r#"a href="http://localhost:8000""#.to_string(),
        );
        let end_root_token = Token::new_source(TokenType::EndToken, "div".to_string());
        let tobe = vec![start_root_token, char_token, root_token, end_root_token];
        let token_array = Token::create_token_array(source);
        assert_eq!(token_array, tobe);
    }
}
