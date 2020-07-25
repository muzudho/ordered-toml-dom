//! Syntax parser.
//! 構文パーサー。

use crate::lexical_parser::{Token, TokenType};
use casual_logger::Table;

/// `'value'`.
pub struct SingleQuotedStringParser {
    value: String,
}
impl SingleQuotedStringParser {
    pub fn new() -> Self {
        SingleQuotedStringParser {
            value: String::new(),
        }
    }
    /// # Returns
    ///
    /// End of syntax.
    pub fn parse(&mut self, token: &Token) -> bool {
        match token.type_ {
            TokenType::SingleQuotation => {
                // End of syntax.
                // 構文の終わり。
                return true;
            }
            _ => {
                self.value.push_str(&token.value);
            }
        }
        false
    }
    pub fn log(&self) -> Table {
        Table::default().str("value", &self.value).clone()
    }
}
