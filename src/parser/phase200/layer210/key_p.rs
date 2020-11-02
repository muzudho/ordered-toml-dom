//! Key parser.  
//! キー・パーサー。  

use crate::model::{
    layer110::{CharacterType, Token, TokenType},
    layer210::Key,
};
use crate::parser::phase200::error;
use crate::parser::phase200::layer210::{KeyP, PResult};
use crate::parser::phase200::LookAheadItems<char>;
use casual_logger::Table as LogTable;

impl Default for KeyP {
    fn default() -> Self {
        KeyP {
            buffer: Some(Key::default()),
        }
    }
}
impl KeyP {
    pub fn flush(&mut self) -> Option<Key> {
        if let Some(key) = &self.buffer {
            let m = Some(key.clone());
            self.buffer = None;
            return m;
        }
        None
    }
    /// # Arguments
    ///
    /// * `characters` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, characters: &LookAheadItems<char>) -> PResult {
        let chr0 = characters.current.as_ref().unwrap();
        match chr0.type_ {
            CharacterType::Alpha
            | CharacterType::Digit
            | CharacterType::Hyphen
            | CharacterType::Underscore => {
                let m = self.buffer.as_mut().unwrap();
                m.push_token(&Token::from_character(&chr0, TokenType::Key));

                // Look-ahead.
                // 先読み。
                if let Some(token1) = characters.one_ahead.as_ref() {
                    match token1.type_ {
                        CharacterType::Alpha
                        | CharacterType::Digit
                        | CharacterType::Hyphen
                        | CharacterType::Underscore => PResult::Ongoing,
                        _ => PResult::End,
                    }
                } else {
                    PResult::End
                }
            }
            _ => return error(&mut self.log(), &characters, "key.rs.38."),
        }
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> LogTable {
        let mut t = LogTable::default().clone();
        if let Some(key) = &self.buffer {
            t.str("buffer", &key.to_string());
        }
        t
    }
}
