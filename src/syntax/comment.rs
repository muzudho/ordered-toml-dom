//! Syntax parser.
//! 構文パーサー。

use crate::lexical_parser::Token;
use casual_logger::Table;

/// `# comment`.
pub struct CommentParser {
    value: String,
}
impl CommentParser {
    pub fn new() -> Self {
        CommentParser {
            value: String::new(),
        }
    }
    pub fn parse(&mut self, token: &Token) {
        self.value.push_str(&token.value);
    }
    pub fn log(&self) -> Table {
        Table::default().str("value", &self.value).clone()
    }
}
