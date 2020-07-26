//! Syntax parser.
//! 構文パーサー。

use crate::lexical_parser::Token;
use crate::object_model::comment::CommentM;
use crate::syntax::SyntaxParserResult;
use casual_logger::Table;

/// `# comment`.
pub struct CommentP {
    product: CommentM,
}
impl CommentP {
    pub fn new() -> Self {
        CommentP {
            product: CommentM::default(),
        }
    }
    pub fn product(&self) -> &CommentM {
        &self.product
    }
    /// # Returns
    ///
    /// * `SyntaxParserResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token) -> SyntaxParserResult {
        self.product.push_token(token);
        SyntaxParserResult::Ok(false)
    }
    pub fn eol(&self) -> SyntaxParserResult {
        SyntaxParserResult::Ok(true)
    }
    pub fn err_table(&self) -> Table {
        Table::default().str("product", &self.product.value).clone()
    }
}
