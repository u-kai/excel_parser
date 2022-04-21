#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType {
    StartToken,
    EndToken,
    SingleToken,
    Character,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PrevChar {
    StartTag,
    EndTag,
    Character,
    Slash,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PrevCharcter {
    value: PrevChar,
}
impl PrevCharcter {
    pub fn new() -> Self {
        PrevCharcter {
            value: PrevChar::Character,
        }
    }
    pub fn get_prev_char(&self) -> PrevChar {
        self.value
    }
    pub fn change_character(&mut self) {
        self.value = PrevChar::Character;
    }
    pub fn change_start_tag(&mut self) {
        self.value = PrevChar::StartTag;
    }
    pub fn change_end_tag(&mut self) {
        self.value = PrevChar::EndTag;
    }
    pub fn change_slash(&mut self) {
        self.value = PrevChar::Slash;
    }
}
