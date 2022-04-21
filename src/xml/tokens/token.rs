use super::states::{PrevChar, TokenType};

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
    pub fn drain(&mut self) -> Token {
        Token {
            token_type: self.token_type,
            value: self.value.drain(..).collect(),
        }
    }
    pub fn get_value(&self) -> &TokenValue {
        &self.value
    }
    pub fn get_token_type(&self) -> &TokenType {
        &self.token_type
    }
    pub fn add_char(&mut self, c: char) {
        self.value.push(c)
    }
    pub fn change_single(&mut self) {
        self.change_type(TokenType::SingleToken)
    }
    pub fn change_start(&mut self) {
        self.change_type(TokenType::StartToken)
    }
    pub fn change_end(&mut self) {
        self.change_type(TokenType::EndToken)
    }
    pub fn change_character(&mut self) {
        self.change_type(TokenType::Character)
    }
    pub fn is_add_slash(&self, prev_char_state: PrevChar) -> bool {
        prev_char_state == PrevChar::Slash && self.get_token_type() != &TokenType::EndToken
    }
    pub fn is_empty_value(&self) -> bool {
        self.get_value() == ""
    }
    fn change_type(&mut self, token_type: TokenType) {
        self.token_type = token_type
    }
}

#[cfg(test)]
mod token_test {

    use crate::xml::tokens::{token::TokenValue, token_array::TokenArray};

    use super::{Token, TokenType};

    #[test]
    fn create_token_array_test() {
        impl Token {
            pub fn new_source(token_type: TokenType, value: TokenValue) -> Self {
                Token { token_type, value }
            }
        }
        let data = "<div><p>p-data</p>div-data</div>";
        let token_array = TokenArray::new(data);
        let div_start = Token::new_source(TokenType::StartToken, "div".to_string());
        let p_start = Token::new_source(TokenType::StartToken, "p".to_string());
        let p_char = Token::new_source(TokenType::Character, "p-data".to_string());
        let p_end = Token::new_source(TokenType::EndToken, "p".to_string());
        let div_char = Token::new_source(TokenType::Character, "div-data".to_string());
        let div_end = Token::new_source(TokenType::EndToken, "div".to_string());
        let test_array = vec![div_start, p_start, p_char, p_end, div_char, div_end];
        assert_eq!(token_array.drain(), test_array);
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
            TokenType::SingleToken,
            r#"a href="http://localhost:8000""#.to_string(),
        );
        let end_root_token = Token::new_source(TokenType::EndToken, "div".to_string());
        let tobe = vec![start_root_token, char_token, root_token, end_root_token];
        let token_array = TokenArray::new(source);
        assert_eq!(token_array.drain(), tobe);
    }
}
