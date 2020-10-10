//! Syntax scanner.  
//! 構文走査器。  

use crate::model::{layer110::token::TokenLine, layer310::Document};
use crate::parser::{layer210::PResult, layer230::DocumentElementP};
use casual_logger::Table;

/// Syntax scanner.  
/// 構文走査器。  
pub struct SyntaxScanner {
    pub document_element_p: DocumentElementP,
}
impl Default for SyntaxScanner {
    fn default() -> Self {
        SyntaxScanner {
            document_element_p: DocumentElementP::default(),
        }
    }
}
impl SyntaxScanner {
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn scan_line(&mut self, token_line: &TokenLine, dom: &mut Document) -> PResult {
        for (i, token) in token_line.tokens.iter().enumerate() {
            match self.document_element_p.parse(token) {
                PResult::End => {
                    if let Some(child_m) = self.document_element_p.flush() {
                        dom.push_broad_line(&child_m);
                    } else {
                        let mut err_token_line = TokenLine::new(token_line.row_number);
                        err_token_line.tokens = token_line.tokens[i..].to_vec();
                        return PResult::Err(
                            self.err_table()
                                .str("token_line", &format!("{:?}", token_line))
                                .str("rest", &format!("{:?}", err_token_line))
                                .clone(),
                        );
                    }
                }
                PResult::Err(table) => {
                    return PResult::Err(
                        self.err_table()
                            .str("token_line", &format!("{:?}", token_line))
                            .sub_t("error", &table)
                            .clone(),
                    );
                }
                PResult::Ongoing => {}
            }
        }

        PResult::Ongoing
    }

    pub fn err_table(&self) -> Table {
        let mut t = Table::default();
        t.str("parser", "syntax_scanner.rs/SyntaxScanner#scan_line")
            .sub_t("line", &self.document_element_p.log_table());
        t
    }
}
