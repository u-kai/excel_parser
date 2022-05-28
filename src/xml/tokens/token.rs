type TokenValue<'a> = &'a str;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    value: TokenValue<'a>,
    token_type: TokenType,
}
impl<'a> Token<'a> {
    pub fn with_type(s: &'a str, token_type: TokenType) -> Self {
        Token {
            value: s,
            token_type,
        }
    }
    pub fn get_token_type(&self) -> TokenType {
        self.token_type
    }
    pub fn get_value(&self) -> TokenValue<'a> {
        &self.value
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType {
    StartToken,
    EndToken,
    SingleToken,
    Character,
}
