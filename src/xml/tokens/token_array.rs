use super::token::{Token, TokenType};

enum StateMachine {
    CharBlank,
    CharChar,
    StartStart,
    EndChar,
    StartChar,
    StartSlash,
}

pub fn create_token_array<'a>(source: &'a str) -> Vec<Token<'a>> {
    let mut start_index = 0;
    let mut vec = Vec::new();
    let mut state = StateMachine::CharBlank;
    source.bytes().enumerate().for_each(|(i, c)| match state {
        StateMachine::CharBlank => match c {
            60 => {
                state = StateMachine::StartStart;
                start_index = i + 1;
            }
            _ => {
                if !(c.is_ascii_whitespace()) {
                    state = StateMachine::CharChar;
                    start_index = i;
                }
            }
        },
        StateMachine::CharChar => match c {
            60 => {
                vec.push(Token::with_type(
                    source.get(start_index..i).unwrap(),
                    TokenType::Character,
                ));
                state = StateMachine::StartStart;
                start_index = i + 1;
            }
            _ => {
                if c.is_ascii_whitespace() {
                    vec.push(Token::with_type(
                        source.get(start_index..i).unwrap(),
                        TokenType::Character,
                    ));
                    state = StateMachine::CharBlank;
                }
            }
        },
        StateMachine::StartStart => match c {
            47 => {
                state = StateMachine::EndChar;
                start_index += 1;
            }
            _ => {
                if c.is_ascii_whitespace() {
                    return;
                }
                state = StateMachine::StartChar;
            }
        },
        StateMachine::EndChar => match c {
            62 => {
                vec.push(Token::with_type(
                    source.get(start_index..i).expect(
                        format!(
                            "len {} start {} i {}, range {:?}",
                            source.len(),
                            start_index,
                            i,
                            source.get(190..)
                        )
                        .as_str(),
                    ),
                    TokenType::EndToken,
                ));
                state = StateMachine::CharBlank;
            }
            _ => (),
        },
        StateMachine::StartChar => match c {
            47 => {
                state = StateMachine::StartSlash;
            }
            62 => {
                state = StateMachine::CharBlank;
                vec.push(Token::with_type(
                    source.get(start_index..i).unwrap(),
                    TokenType::StartToken,
                ))
            }
            _ => (),
        },
        StateMachine::StartSlash => match c {
            62 => {
                vec.push(Token::with_type(
                    source.get(start_index..i - 1).unwrap(),
                    TokenType::SingleToken,
                ));
                state = StateMachine::CharBlank;
            }
            _ => {
                if !(c.is_ascii_whitespace()) {
                    state = StateMachine::StartChar;
                }
            }
        },
    });

    vec
}

#[cfg(test)]
mod p_token_array_test {
    use crate::xml::tokens::{
        token::{Token, TokenType},
        token_array::create_token_array,
    };

    #[test]
    fn build_test() {
        let source = r#"
                    <div>
                        hello world
                    </div>
                    "#;
        let token_array = create_token_array(source);
        assert_eq!(
            token_array,
            vec![
                Token::with_type("div", TokenType::StartToken),
                Token::with_type("hello", TokenType::Character),
                Token::with_type("world", TokenType::Character),
                Token::with_type("div", TokenType::EndToken),
            ]
        );
        let source = r#"
        <div id="name" class="style style2">
        hello world
        </div>
        "#;
        let token_array = create_token_array(source);
        assert_eq!(
            token_array,
            vec![
                Token::with_type(
                    r#"div id="name" class="style style2""#,
                    TokenType::StartToken
                ),
                Token::with_type("hello", TokenType::Character),
                Token::with_type("world", TokenType::Character),
                Token::with_type("div", TokenType::EndToken),
            ]
        );
        let source = r#"
        <div id="name" class="style style2">
        <data />
        hello world
        <p> p desu </ p>
        </div>
        "#;
        let token_array = create_token_array(source);
        assert_eq!(
            token_array,
            vec![
                Token::with_type(
                    r#"div id="name" class="style style2""#,
                    TokenType::StartToken
                ),
                Token::with_type("data ", TokenType::SingleToken),
                Token::with_type("hello", TokenType::Character),
                Token::with_type("world", TokenType::Character),
                Token::with_type("p", TokenType::StartToken),
                Token::with_type("p", TokenType::Character),
                Token::with_type("desu", TokenType::Character),
                Token::with_type(" p", TokenType::EndToken),
                Token::with_type("div", TokenType::EndToken),
            ]
        );
    }
}

//#[derive(Debug)]
//pub struct TokenArray<'a>(Vec<Token<'a>>);
//impl<'a> TokenArray<'a> {
//pub fn new(source: &'a str) -> Self {
//let token_array = TokenArray::_new();
//token_array._build(source)
//}
//pub fn token_array(self) -> Vec<Token<'a>> {
//self.0
//}
//fn _new() -> Self {
//TokenArray(Vec::new())
//}
//fn _build(mut self, s: &'a str) -> Self {
//let mut start_index = 0;
//let mut token_type = TokenType::new();
//let mut prev_char = PrevChar::new();
//s.chars().enumerate().for_each(|(i, c)| match c {
//// case start-tag
//// next is start token-type element
//'<' => {
//// case start_index == i is init loop 0 == 0
//if start_index != i && prev_char != PrevChar::Blank {
//// push before token
//self.0
//.push(Token::with_type(s.get(start_index..i).unwrap(), token_type))
//}
//start_index = i + 1;
//// もしかしたら次が空白かもしれないのでtoken_typeはまだ設定しない
//prev_char.change_start_tag();
//}
//'/' => {
//match prev_char {
//// case begin end-tag token
//PrevChar::StartTag => {
//token_type.change_end();
//}
//// case in the middle of token
//_ => (),
//}
//prev_char.change_slash();
//}
//'>' => {
//match prev_char {
////case end end-tag token
//PrevChar::Slash => {
//token_type.change_single();
//// i-1 means is except before "/"
//self.0.push(Token::with_type(
//s.get(start_index..(i - 1)).unwrap(),
//token_type,
//))
//}
////case end start-tag or single-tag token
//_ => self.0.push(Token::with_type(
//s.get(start_index..i)
//.expect(format!("{},{:?}", start_index, self.0).as_str()),
//token_type,
//)),
//}
////next is begin something token
//start_index = i + 1;
//prev_char.change_end_tag();
//token_type.change_character();
//}
//_ => {
//if c.is_whitespace() {
//match prev_char {
////case ignore blank
//// so incriment start_index
//PrevChar::Blank => {
//start_index += 1;
//}

////case split character
//PrevChar::Character => {
//// case start_index == i is init loop 0 == 0
//if start_index != i {
//if let Some(str) = s.get(start_index..i) {
//self.0.push(Token::with_type(str, token_type));
//}
//}
//prev_char.change_blank();
//start_index = i + 1
//}
//PrevChar::StartTag => {
//// case in the middle of start-tag
//// so ignore
//}
//PrevChar::EndTag => {
//// case ignore blank
//// so incriment start_index
//start_index += 1;
//}
//PrevChar::Slash => {
////ignore
//}
//}
//return;
//}
//match prev_char {
////case ignore blank
//// so incriment start_index
//PrevChar::Blank => {
//if token_type == TokenType::Character {
//start_index = i
//}
//prev_char.change_character()
//}

////case split character
//PrevChar::Character => {
//// case start_index == i is init loop 0 == 0
//if start_index != i {
//if token_type == TokenType::Character {
//return;
//}
//if token_type == TokenType::EndToken {
//return;
//}
////self.0
////.push(Token::with_type(s.get(start_index..i).unwrap(), token_type));
//}
//prev_char.change_character();
//start_index = i + 1
//}
//PrevChar::StartTag => {
//// case in the middle of start token
//if token_type == TokenType::StartToken {
//return;
//}
//token_type.change_start();
//start_index = i
//}

//PrevChar::EndTag => {
//// case ignore blank
//// so incriment start_index
//prev_char.change_character();
//token_type.change_character();
//start_index = i;
//}
//PrevChar::Slash => {
////ignore
//start_index = i;
//prev_char.change_character();
//}
//}
//}
//});
//self
//}
//}

//#[cfg(test)]
//mod token_array_test {
//use crate::xml::tokens::{states::TokenType, token::Token};

//use super::TokenArray;
//#[test]
//fn build_test() {
//let source = r#"
//<div>
//hello world
//</div>
//"#;
//let token_array = TokenArray::new(source).token_array();
//assert_eq!(
//token_array,
//vec![
//Token::with_type("div", TokenType::StartToken),
//Token::with_type("hello", TokenType::Character),
//Token::with_type("world", TokenType::Character),
//Token::with_type("div", TokenType::EndToken),
//]
//);
//let source = r#"
//<div id="name" class="style style2">
//hello world
//</div>
//"#;
//let token_array = TokenArray::new(source).token_array();
//assert_eq!(
//token_array,
//vec![
//Token::with_type(
//r#"div id="name" class="style style2""#,
//TokenType::StartToken
//),
//Token::with_type("hello", TokenType::Character),
//Token::with_type("world", TokenType::Character),
//Token::with_type("div", TokenType::EndToken),
//]
//);
//let source = r#"
//<div id="name" class="style style2">
//<data />
//hello world
//<p> p desu </    p>
//</div>
//"#;
//let token_array = TokenArray::new(source).token_array();
//assert_eq!(
//token_array,
//vec![
//Token::with_type(
//r#"div id="name" class="style style2""#,
//TokenType::StartToken
//),
//Token::with_type("data ", TokenType::SingleToken),
//Token::with_type("hello", TokenType::Character),
//Token::with_type("world", TokenType::Character),
//Token::with_type("p", TokenType::StartToken),
//Token::with_type("p", TokenType::Character),
//Token::with_type("desu", TokenType::Character),
//Token::with_type("p", TokenType::EndToken),
//Token::with_type("div", TokenType::EndToken),
//]
//);
//}
//}
