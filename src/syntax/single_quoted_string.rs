//! Syntax parser.
//! 構文パーサー。

use crate::object_model::single_quoted_string::SingleQuotedStringM;
use crate::syntax::SyntaxParserResult;
use crate::token::{Token, TokenType};
use casual_logger::Table;

/// `'value'`.
#[derive(Clone)]
pub struct SingleQuotedStringP {
    product: SingleQuotedStringM,
}
impl SingleQuotedStringP {
    pub fn product(&self) -> &SingleQuotedStringM {
        &self.product
    }
    pub fn new() -> Self {
        SingleQuotedStringP {
            product: SingleQuotedStringM::default(),
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
                self.product.push_token(&token);
            }
        }
        SyntaxParserResult::Ongoing
    }
    pub fn err_table(&self) -> Table {
        Table::default().str("value", &self.product.value).clone()
    }
}
