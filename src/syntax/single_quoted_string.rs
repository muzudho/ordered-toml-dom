//! Single quoted string syntax parser.  
//! 単一引用符文字列構文パーサー。  

use crate::model::layer1::SingleQuotedString;
use crate::syntax::{SingleQuotedStringP, SyntaxParserResult};
use crate::token::{Token, TokenType};
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
    /// * `SyntaxParserResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token) -> SyntaxParserResult {
        match token.type_ {
            TokenType::SingleQuotation => {
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
    pub fn log_table(&self) -> Table {
        let mut t = Table::default().clone();
        if let Some(m) = &self.buffer {
            t.str("value", &format!("{:?}", m));
        }
        t
    }
}
