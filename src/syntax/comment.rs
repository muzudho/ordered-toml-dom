//! Syntax parser.
//! 構文パーサー。

use crate::lexical_parser::Token;
use casual_logger::Table;

/// `# comment`.
pub struct CommentP {
    product: String,
}
impl CommentP {
    pub fn new() -> Self {
        CommentP {
            product: String::new(),
        }
    }
    pub fn product(&self) -> String {
        self.product.clone()
    }
    pub fn parse(&mut self, token: &Token) {
        self.product.push_str(&token.value);
    }
    pub fn log(&self) -> Table {
        Table::default().str("product", &self.product).clone()
    }
}
