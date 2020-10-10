//! Single quoted string syntax parser.  
//! 単一引用符文字列構文パーサー。  

use crate::model::{
    layer10::token::{Token, TokenType},
    layer20::SingleQuotedString,
};
use crate::parser::syntax::layer20::{PResult, SingleQuotedStringP};
use casual_logger::Table;

impl SingleQuotedStringP {
    pub fn flush(&mut self) -> Option<SingleQuotedString> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    pub fn new() -> Self {
        SingleQuotedStringP {
            buffer: Some(SingleQuotedString::default()),
        }
    }
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token) -> PResult {
        match token.type_ {
            // `'`
            TokenType::SingleQuotation => {
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
