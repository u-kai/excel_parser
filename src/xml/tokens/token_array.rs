use super::{
    states::{PrevChar, TokenType},
    token::{self, Token},
};

#[derive(Debug)]
pub struct TokenArray<'a>(Vec<Token<'a>>);
impl<'a> TokenArray<'a> {
    pub fn new(source: &'a str) -> Self {
        let token_array = TokenArray::_new();
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
        let mut token_type = TokenType::new();
        let mut prev_char = PrevChar::new();
        s.chars().enumerate().for_each(|(i, c)| match c {
            // case start-tag
            // next is start token-type element
            '<' => {
                // case start_index == i is init loop 0 == 0
                if start_index != i && prev_char != PrevChar::Blank {
                    // push before token
                    self.0
                        .push(Token::with_type(s.get(start_index..i).unwrap(), token_type))
                }
                start_index = i + 1;
                // もしかしたら次が空白かもしれないのでtoken_typeはまだ設定しない
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
                if c.is_whitespace() {
                    match prev_char {
                        //case ignore blank
                        // so incriment start_index
                        PrevChar::Blank => {
                            start_index += 1;
                        }

                        //case split character
                        PrevChar::Character => {
                            // case start_index == i is init loop 0 == 0
                            if start_index != i {
                                self.0.push(Token::with_type(
                                    s.get(start_index..i).unwrap(),
                                    token_type,
                                ));
                            }
                            prev_char.change_blank();
                            start_index = i + 1
                        }
                        PrevChar::StartTag => {
                            // case in the middle of start-tag
                            // so ignore
                        }
                        PrevChar::EndTag => {
                            // case ignore blank
                            // so incriment start_index
                            start_index += 1;
                        }
                        PrevChar::Slash => {
                            //ignore
                        }
                    }
                    return;
                }
                match prev_char {
                    //case ignore blank
                    // so incriment start_index
                    PrevChar::Blank => {
                        if token_type == TokenType::Character {
                            start_index = i
                        }
                        prev_char.change_character()
                    }

                    //case split character
                    PrevChar::Character => {
                        // case start_index == i is init loop 0 == 0
                        if start_index != i {
                            if token_type == TokenType::Character {
                                return;
                            }
                            if token_type == TokenType::EndToken {
                                return;
                            }
                            //self.0
                            //.push(Token::with_type(s.get(start_index..i).unwrap(), token_type));
                        }
                        prev_char.change_character();
                        start_index = i + 1
                    }
                    PrevChar::StartTag => {
                        // case in the middle of start token
                        if token_type == TokenType::StartToken {
                            return;
                        }
                        token_type.change_start();
                        start_index = i
                    }

                    PrevChar::EndTag => {
                        // case ignore blank
                        // so incriment start_index
                        prev_char.change_character();
                        token_type.change_character();
                        start_index = i;
                    }
                    PrevChar::Slash => {
                        //ignore
                        start_index = i;
                        println!("{:?}", token_type);
                        prev_char.change_character();
                    }
                }
                ////case split character by Blank
                //if prev_char != PrevChar::Blank && i == start_index {
                //self.0
                //.push(Token::with_type(s.get(start_index..i).unwrap(), token_type));
                //}
                //start_index = i + 1;
                //} else {
                //start_index += 1;
                //}
            }
        });
        self
    }
}

#[cfg(test)]
mod token_array_test {
    use crate::xml::tokens::{states::TokenType, token::Token};

    use super::TokenArray;
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
