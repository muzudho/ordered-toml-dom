//! Table syntax parser.  
//! テーブル構文パーサー。  

use crate::model::layer30::Table as TableM;
use crate::parser::syntax::layer10::{PResult, TableP};
use crate::token::{Token, TokenType};
use casual_logger::Table as LogTable;

impl TableP {
    pub fn flush(&mut self) -> Option<TableM> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    pub fn new() -> Self {
        TableP {
            buffer: Some(TableM::default()),
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
    pub fn log_table(&self) -> LogTable {
        let mut t = LogTable::default().clone();
        if let Some(m) = &self.buffer {
            t.str("value", &format!("{:?}", m));
        }
        t
    }
}
