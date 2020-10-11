//! Array of HeaderOfArrayOfTable syntax parser.  
//! テーブルの配列構文パーサー。  

use crate::model::{
    layer110::token::{Token, TokenType},
    layer230::HeaderOfArrayOfTable,
};
use crate::parser::phase200::layer210::{HeaderPOfArrayOfTable, PResult};
use casual_logger::Table as LogTable;

impl HeaderPOfArrayOfTable {
    pub fn flush(&mut self) -> Option<HeaderOfArrayOfTable> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    pub fn new() -> Self {
        HeaderPOfArrayOfTable {
            buffer: Some(HeaderOfArrayOfTable::default()),
        }
    }
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token) -> PResult {
        match token.type_ {
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
    pub fn log_table(&self, code_location: &str) -> LogTable {
        let mut t = LogTable::default()
            .str("code_location", code_location)
            .clone();
        if let Some(m) = &self.buffer {
            t.str("value", &format!("{:?}", m));
        }
        t
    }
}
