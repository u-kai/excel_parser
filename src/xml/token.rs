#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType {
    StartToken,
    EndToken,
    SingleToken,
    Character,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PrevCharState {
    StartTag,
    EndTag,
    Character,
    Slash,
}
type Slash = char;
struct SlashBuffer {
    value: Vec<Slash>,
}
impl SlashBuffer {
    pub fn new() -> Self {
        SlashBuffer { value: Vec::new() }
    }
    pub fn add(&mut self, slash: char) {
        self.value.push(slash)
    }
    pub fn trash_one(&mut self) {
        self.value.pop();
    }
    pub fn pop(&mut self) -> char {
        self.value.pop().unwrap()
    }
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
        let mut slash_buffer = SlashBuffer::new();
        let mut prev_char_state = PrevCharState::Character;
        fn _init_token(token: &mut Token, new_type: TokenType) {
            token.clear_value();
            token.change_type(new_type);
        }
        for c in s.chars() {
            match c {
                '<' => {
                    if !(tmp_token.is_empty_value()) {
                        token_array.push(tmp_token.clone());
                    }
                    tmp_token.init_token(TokenType::StartToken);
                    prev_char_state = PrevCharState::StartTag;
                }
                '/' => {
                    match prev_char_state {
                        PrevCharState::StartTag => tmp_token.change_type(TokenType::EndToken),
                        PrevCharState::Slash => tmp_token.add_char(slash_buffer.pop()),
                        PrevCharState::EndTag => {
                            tmp_token.add_char(c);
                        }
                        PrevCharState::Character => {
                            ();
                        }
                    }
                    prev_char_state = PrevCharState::Slash;
                    slash_buffer.add(c);
                }
                '>' => {
                    if prev_char_state == PrevCharState::Slash {
                        tmp_token.change_type(TokenType::SingleToken);
                        slash_buffer.trash_one();
                    }
                    prev_char_state = PrevCharState::EndTag;
                    token_array.push(tmp_token.clone());
                    tmp_token.init_token(TokenType::Character)
                }
                _ => {
                    if tmp_token.is_add_slash(prev_char_state) {
                        tmp_token.add_char(slash_buffer.pop());
                    } else {
                        slash_buffer.trash_one();
                    }
                    match tmp_token.get_token_type() {
                        TokenType::Character => {
                            if !(c.is_whitespace()) {
                                tmp_token.add_char(c)
                            }
                        }
                        _ => tmp_token.add_char(c),
                    }
                    prev_char_state = PrevCharState::Character;
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
    fn is_add_slash(&self, prev_char_state: PrevCharState) -> bool {
        prev_char_state == PrevCharState::Slash && self.get_token_type() != &TokenType::EndToken
    }
    fn is_empty_value(&self) -> bool {
        self.get_value() == ""
    }
    fn init_token(&mut self, token_type: TokenType) {
        self.clear_value();
        self.change_type(token_type);
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
            TokenType::SingleToken,
            r#"a href="http://localhost:8000""#.to_string(),
        );
        let end_root_token = Token::new_source(TokenType::EndToken, "div".to_string());
        let tobe = vec![start_root_token, char_token, root_token, end_root_token];
        let token_array = Token::create_token_array(source);
        assert_eq!(token_array, tobe);
    }
}
