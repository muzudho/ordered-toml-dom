//! Array of ArrayOfTable syntax parser.  
//! テーブルの配列構文パーサー。  

use crate::model::layer30::ArrayOfTable as ArrayOfTableM;
use crate::syntax::{ArrayOfTableP, SyntaxParserResult};
use crate::token::{Token, TokenType};
use casual_logger::Table as LogTable;

impl ArrayOfTableP {
    pub fn flush(&mut self) -> Option<ArrayOfTableM> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    pub fn new() -> Self {
        ArrayOfTableP {
            buffer: Some(ArrayOfTableM::default()),
        }
    }
    /// # Returns
    ///
    /// * `SyntaxParserResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token) -> SyntaxParserResult {
        match token.type_ {
            TokenType::DoubleQuotation => {
                // End of syntax.
                // 構文の終わり。
                return SyntaxParserResult::End;
            }
            _ => {
                let m = self.buffer.as_mut().unwrap();
                m.push_token(&token);
            }
        }
        SyntaxParserResult::Ongoing
    }
    pub fn log_table(&self) -> LogTable {
        let mut t = LogTable::default().clone();
        if let Some(m) = &self.buffer {
            t.str("value", &format!("{:?}", m));
        }
        t
    }
}
