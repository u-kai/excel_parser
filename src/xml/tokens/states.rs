#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType {
    StartToken,
    EndToken,
    SingleToken,
    Character,
}
impl TokenType {
    pub fn new() -> Self {
        TokenType::Character
    }
    pub fn change_start(&mut self) {
        *self = TokenType::StartToken
    }
    pub fn change_end(&mut self) {
        *self = TokenType::EndToken
    }
    pub fn change_single(&mut self) {
        *self = TokenType::SingleToken
    }
    pub fn change_character(&mut self) {
        *self = TokenType::Character
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PrevChar {
    StartTag,
    EndTag,
    Character,
    Slash,
    Blank,
}
impl PrevChar {
    pub fn new() -> Self {
        PrevChar::Character
    }
    pub fn change_character(&mut self) {
        *self = PrevChar::Character;
    }
    pub fn change_start_tag(&mut self) {
        *self = PrevChar::StartTag;
    }
    pub fn change_end_tag(&mut self) {
        *self = PrevChar::EndTag;
    }
    pub fn change_slash(&mut self) {
        *self = PrevChar::Slash;
    }
    pub fn change_blank(&mut self) {
        *self = PrevChar::Blank;
    }
}
