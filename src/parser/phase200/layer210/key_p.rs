//! Key parser.  
//! キー・パーサー。  

use crate::model::{
    layer110::{Token, TokenType},
    layer210::Key,
};
use crate::parser::phase200::{
    error,
    layer210::{KeyP, PResult},
};
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
        if let Some(buffer) = &self.buffer {
            let m = Some(Key::from_str(buffer.value.trim_end())); // TODO トリム要らないのでは。
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
    pub fn parse(&mut self, tokens: (Option<&Token>, Option<&Token>, Option<&Token>)) -> PResult {
        let token0 = tokens.0.unwrap();
        match token0.type_ {
            TokenType::Alphabet
            | TokenType::Numeral
            | TokenType::Hyphen
            | TokenType::Underscore => {
                let m = self.buffer.as_mut().unwrap();
                m.push_token(&token0);
                return PResult::End;
            }
            _ => return error(&mut self.log(), tokens, "key.rs.38."),
        }
        // PResult::Ongoing
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> LogTable {
        let t = LogTable::default()
            .str("buffer", &format!("{:?}", &self.buffer))
            .clone();
        t
    }
}
