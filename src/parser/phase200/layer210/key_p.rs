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
    /// * `look_ahead_items` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, look_ahead_items: &LookAheadItems<char>) -> PResult {
        let chr0 = look_ahead_items.get(0).unwrap();
        match chr0.type_ {
            'A'..='Z' | 'a'..='z'
            | '0'..='9'
            | '-'
            | '_' => {
                let m = self.buffer.as_mut().unwrap();
                m.push_token(&Token::from_character(chr, TokenType::Key));

                // Look-ahead.
                // 先読み。
                if let Some(token1) = look_ahead_items.one_ahead.as_ref() {
                    match token1.type_ {
                        'A'..='Z' | 'a'..='z'
                        | '0'..='9'
                        | '-'
                        | '_' => PResult::Ongoing,
                        _ => PResult::End,
                    }
                } else {
                    PResult::End
                }
            }
            _ => return error(&mut self.log(), &look_ahead_items, "key.rs.38."),
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
