use super::{
    states::{PrevChar, TokenType},
    token::{self, Token},
};

#[derive(Debug)]
pub struct TokenArray<'a>(Vec<Token<'a>>);
impl<'a> TokenArray<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut token_array = TokenArray::_new();
        token_array._build(source)
    }
    pub fn token_array(self) -> Vec<Token<'a>> {
        self.0
    }
    fn _new() -> Self {
        TokenArray(Vec::new())
    }
    fn _build(mut self, s: &'a str) -> Self {
        let mut start_index = 0;
        let mut token_type = TokenType::Character;
        let mut prev_char = PrevChar::new();
        s.chars().enumerate().for_each(|(i, c)| match c {
            // case start-tag
            // next is start token-type element
            '<' => {
                // case start_index == i is init loop 0 == 0
                if start_index != i {
                    // push before token
                    self.0
                        .push(Token::with_type(s.get(start_index..i).unwrap(), token_type))
                }
                start_index = i + 1;
                token_type.change_start();
                prev_char.change_start_tag();
            }
            '/' => {
                match prev_char {
                    // case begin end-tag token
                    PrevChar::StartTag => {
                        token_type.change_end();
                    }
                    // case in the middle of token
                    _ => (),
                }
                prev_char.change_slash();
            }
            '>' => {
                match prev_char {
                    //case end end-tag token
                    PrevChar::Slash => {
                        token_type.change_single();
                        // i-1 means is except before "/"
                        self.0.push(Token::with_type(
                            s.get(start_index..(i - 1)).unwrap(),
                            token_type,
                        ))
                    }
                    //case end start-tag or single-tag token
                    _ => self
                        .0
                        .push(Token::with_type(s.get(start_index..i).unwrap(), token_type)),
                }
                //next is begin something token
                start_index = i + 1;
                prev_char.change_end_tag();
                token_type.change_character();
            }
            _ => {
                if c.is_whitespace() && token_type == TokenType::Character {
                    //case split character by Blank
                    if prev_char != PrevChar::Blank && i != start_index {
                        self.0
                            .push(Token::with_type(s.get(start_index..i).unwrap(), token_type));
                    }
                    start_index += 1;
                    prev_char.change_blank();
                    return;
                }
                prev_char.change_character();
            }
        });
        self
    }
}

#[cfg(test)]
mod token_array_test {
    use crate::xml::tokens::{states::TokenType, token::Token};

    use super::TokenArray;
    //impl<'a> TokenArray<'a> {
    //pub fn token_array(self) -> Vec<Token<'a>> {
    //self.0
    //}
    //}

    #[test]
    fn build_test() {
        let source = r#"
        <div>
            hello world
        </div>
        "#;
        let token_array = TokenArray::new(source).token_array();
        assert_eq!(
            token_array,
            vec![
                Token::with_type("div", TokenType::StartToken),
                Token::with_type("hello", TokenType::Character),
                Token::with_type("world", TokenType::Character),
                Token::with_type("div", TokenType::EndToken),
            ]
        )
    }
}
