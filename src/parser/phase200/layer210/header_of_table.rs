//! Table syntax parser.  
//! テーブル構文パーサー。  

use crate::model::{
    layer110::token::{Token, TokenType},
    layer230::HeaderOfTable,
};
use crate::parser::phase200::layer210::{HeaderPOfTable, PResult};
// use casual_logger::Table;

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
    pub fn parse(&mut self, look_ahead_token: Option<&Token>, token: &Token) -> PResult {
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

    /* TODO
    /// Log.
    /// ログ。
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();
        if let Some(m) = &self.buffer {
            t.str("value", &format!("{:?}", m));
        }
        t
    }
    */
}
