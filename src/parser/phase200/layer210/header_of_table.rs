//! Table syntax parser.  
//! テーブル構文パーサー。  

use crate::model::{
    layer110::token::{Token, TokenType},
    layer230::HeaderOfTable,
};
use crate::parser::phase200::layer210::{HeaderPOfTable, PResult};
use casual_logger::Table as LogTable;

impl HeaderPOfTable {
    pub fn flush(&mut self) -> Option<HeaderOfTable> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    pub fn new() -> Self {
        HeaderPOfTable {
            buffer: Some(HeaderOfTable::default()),
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
    pub fn log_table(&self, place_of_occurrence: &str) -> LogTable {
        let mut t = LogTable::default()
            .str("place_of_occurrence", place_of_occurrence)
            .clone();
        if let Some(m) = &self.buffer {
            t.str("value", &format!("{:?}", m));
        }
        t
    }
}
