use crate::model::layer110::CharacterLine;
use std::fmt;

impl CharacterLine {
    pub fn new(row_number: usize) -> Self {
        CharacterLine {
            row_number: row_number,
            characters: Vec::new(),
        }
    }

    /// Remaining characters.
    /// 残りのトークン。
    pub fn remaining_tokens(&self, character_index: usize) -> Self {
        CharacterLine {
            row_number: self.row_number,
            characters: self.characters[character_index..].to_vec(),
        }
    }
}
impl fmt::Display for CharacterLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.characters {
            buf.push_str(&token.to_string());
        }
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for CharacterLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.characters {
            buf.push_str(&token.to_debug_string());
        }
        write!(f, "{}", buf)
    }
}
