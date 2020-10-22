//! Comment syntax parser.  
//! コメント構文パーサー。  

use crate::model::layer110::TokenType;
use crate::parser::phase200::layer210::DateTime;
use crate::parser::phase200::layer210::{DateTimeP, PResult};
use crate::parser::phase200::LookAheadTokens;
use casual_logger::Table;

impl DateTimeP {
    pub fn new() -> Self {
        DateTimeP { buffer: None }
    }
    pub fn flush(&mut self) -> Option<DateTime> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
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
            TokenType::EndOfLine => return PResult::End,
            _ => {
                if let None = self.buffer {
                    self.buffer = Some(DateTime::default());
                }
                let m = self.buffer.as_mut().unwrap();
                m.push_token(&token0);
            }
        }
        PResult::Ongoing
    }
    /// Log.  
    /// ログ。  
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();
        if let Some(m) = &self.buffer {
            t.str("buffer", &m.to_string());
        }
        t
    }
}
