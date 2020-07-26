//! Syntax parser.
//! 構文パーサー。

use crate::object_model::comment::CommentM;
use crate::syntax::SyntaxParserResult;
use crate::token::{Token, TokenType};
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
        match token.type_ {
            TokenType::EndOfLine => return SyntaxParserResult::End,
            _ => {
                self.product.push_token(token);
            }
        }
        SyntaxParserResult::Ongoing
    }
    pub fn err_table(&self) -> Table {
        Table::default()
            .str("Parse", "CommentP")
            .str("product", &self.product.value)
            .clone()
    }
}
