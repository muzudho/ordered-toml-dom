//! Key parser.  
//! キー・パーサー。  

use crate::model::{layer110::TokenType, layer210::Key};
use crate::parser::phase200::error;
use crate::parser::phase200::layer210::{KeyP, PResult};
use crate::parser::phase200::LookAheadTokens;
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
    /// * `tokens` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, tokens: &LookAheadTokens) -> PResult {
        let token0 = tokens.current.as_ref().unwrap();
        match token0.type_ {
            TokenType::AlphabetCharacter
            | TokenType::NumeralString
            | TokenType::Hyphen
            | TokenType::Underscore => {
                let m = self.buffer.as_mut().unwrap();
                m.push_token(&token0);

                // Look-ahead.
                // 先読み。
                if let Some(token1) = tokens.one_ahead.as_ref() {
                    match token1.type_ {
                        TokenType::AlphabetCharacter
                        | TokenType::NumeralString
                        | TokenType::Hyphen
                        | TokenType::Underscore => PResult::Ongoing,
                        _ => PResult::End,
                    }
                } else {
                    PResult::End
                }
            }
            _ => return error(&mut self.log(), &tokens, "key.rs.38."),
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
