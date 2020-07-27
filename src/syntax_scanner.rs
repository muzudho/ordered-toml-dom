//! Syntax scanner.  
//! 構文走査器。  

use crate::model::Document;
use crate::syntax::{BroadLineP, SyntaxParserResult};
use crate::token::TokenLine;
use casual_logger::Table;

/// Syntax scanner.  
/// 構文走査器。  
pub struct SyntaxScanner {
    pub broad_line_p: BroadLineP,
}
impl Default for SyntaxScanner {
    fn default() -> Self {
        SyntaxScanner {
            broad_line_p: BroadLineP::default(),
        }
    }
}
impl SyntaxScanner {
    /// # Returns
    ///
    /// * `SyntaxParserResult` - Result.  
    ///                             結果。
    pub fn scan_line(&mut self, token_line: &TokenLine, dom: &mut Document) -> SyntaxParserResult {
        for (i, token) in token_line.tokens.iter().enumerate() {
            match self.broad_line_p.parse(token) {
                SyntaxParserResult::End => {
                    if let Some(child_m) = self.broad_line_p.flush() {
                        dom.push_broad_line(&child_m);
                    } else {
                        return SyntaxParserResult::Err(
                            self.err_table()
                                .str("token_line", &format!("{:?}", token_line))
                                .str(
                                    "rest",
                                    &format!(
                                        "{:?}",
                                        TokenLine::new(&token_line.tokens[i..].to_vec())
                                    ),
                                )
                                .clone(),
                        );
                    }
                }
                SyntaxParserResult::Err(table) => {
                    return SyntaxParserResult::Err(
                        self.err_table()
                            .str("token_line", &format!("{:?}", token_line))
                            .sub_t("error", &table)
                            .clone(),
                    );
                }
                SyntaxParserResult::Ongoing => {}
            }
        }

        SyntaxParserResult::Ongoing
    }

    pub fn err_table(&self) -> Table {
        let mut t = Table::default();
        t.str("parser", "syntax_scanner.rs/SyntaxScanner#scan_line")
            .sub_t("line", &self.broad_line_p.err_table());
        t
    }
}
