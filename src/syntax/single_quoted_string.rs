//! Syntax parser.
//! 構文パーサー。

use crate::model::SingleQuotedStringM;
use crate::syntax::SyntaxParserResult;
use crate::token::{Token, TokenType};
use casual_logger::Table;

/// `'value'`.
#[derive(Clone)]
pub struct SingleQuotedStringP {
    buffer: Option<SingleQuotedStringM>,
}
impl SingleQuotedStringP {
    pub fn flush(&mut self) -> Option<SingleQuotedStringM> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    pub fn new() -> Self {
        SingleQuotedStringP {
            buffer: Some(SingleQuotedStringM::default()),
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
    pub fn err_table(&self) -> Table {
        let mut t = Table::default().clone();
        if let Some(m) = &self.buffer {
            t.str("value", &format!("{:?}", m));
        }
        t
    }
}
