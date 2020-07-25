//! Syntax parser.
//! 構文パーサー。

use crate::lexical_parser::Token;
use crate::syntax::SyntaxParserResult;
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
    /// # Returns
    ///
    /// * `SyntaxParserResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token) -> SyntaxParserResult {
        self.product.push_str(&token.value);
        SyntaxParserResult::Ok(false)
    }
    pub fn eol(&self) -> SyntaxParserResult {
        SyntaxParserResult::Ok(true)
    }
    pub fn log(&self) -> Table {
        Table::default().str("product", &self.product).clone()
    }
}
