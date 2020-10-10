//! Double quoted string syntax parser.  
//! 二重引用符文字列構文パーサー。  

use crate::model::{
    layer110::token::{Token, TokenType},
    layer210::DoubleQuotedString,
};
use crate::parser::syntax::layer210::{DoubleQuotedStringP, PResult};
use casual_logger::Table;

impl DoubleQuotedStringP {
    pub fn flush(&mut self) -> Option<DoubleQuotedString> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    pub fn new() -> Self {
        DoubleQuotedStringP {
            buffer: Some(DoubleQuotedString::default()),
        }
    }
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token) -> PResult {
        match token.type_ {
            // `"`
            TokenType::DoubleQuotation => {
                // End of syntax.
                // 構文の終わり。
                return PResult::End;
            }
            _ => {
                let m = self.buffer.as_mut().unwrap();
                m.push_token(&token);
            }
        }
        PResult::Ongoing
    }
    pub fn log_table(&self) -> Table {
        let mut t = Table::default().clone();
        if let Some(m) = &self.buffer {
            t.str("value", &format!("{:?}", m));
        }
        t
    }
}
