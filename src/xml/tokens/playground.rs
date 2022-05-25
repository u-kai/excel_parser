use super::{states::TokenType, token::Token};
enum StateMachine {
    CharBlank,
    CharChar,
    StartStart,
    EndChar,
    StartChar,
    StartSlash,
}

fn create_token_array<'a>(source: &'a str) -> Vec<Token<'a>> {
    let mut start_index = 0;
    let mut vec = Vec::new();
    let mut state = StateMachine::CharBlank;
    source.chars().enumerate().for_each(|(i, c)| match state {
        StateMachine::CharBlank => match c {
            '<' => {
                state = StateMachine::StartStart;
                start_index = i + 1;
            }
            _ => {
                if !(c.is_whitespace()) {
                    state = StateMachine::CharChar;
                    start_index = i;
                }
            }
        },
        StateMachine::CharChar => match c {
            '<' => {
                vec.push(Token::with_type(
                    source.get(start_index..i).unwrap(),
                    TokenType::Character,
                ));
                start_index = i + 1;
            }
            _ => {
                if c.is_whitespace() {
                    vec.push(Token::with_type(
                        source.get(start_index..i).unwrap(),
                        TokenType::Character,
                    ));
                    state = StateMachine::CharBlank;
                }
            }
        },
        StateMachine::StartStart => match c {
            '/' => {
                state = StateMachine::EndChar;
                start_index += 1;
            }
            _ => {
                if c.is_whitespace() {
                    return;
                }
                state = StateMachine::StartChar;
            }
        },
        StateMachine::EndChar => match c {
            '>' => {
                vec.push(Token::with_type(
                    source.get(start_index..i).unwrap(),
                    TokenType::EndToken,
                ));
                state = StateMachine::CharBlank;
            }
            _ => (),
        },
        StateMachine::StartChar => match c {
            '/' => {
                state = StateMachine::StartSlash;
            }
            '>' => {
                state = StateMachine::CharBlank;
                vec.push(Token::with_type(
                    source.get(start_index..i).unwrap(),
                    TokenType::StartToken,
                ))
            }
            _ => (),
        },
        StateMachine::StartSlash => match c {
            '>' => {
                vec.push(Token::with_type(
                    source.get(start_index..i - 1).unwrap(),
                    TokenType::SingleToken,
                ));
                state = StateMachine::CharBlank;
            }
            _ => (),
        },
    });

    vec
}

#[cfg(test)]
mod p_token_array_test {
    use crate::xml::tokens::{playground::create_token_array, states::TokenType, token::Token};

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
