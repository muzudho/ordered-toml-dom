//! Syntax scanner.  
//! 構文走査器。  

use crate::model::layer40::Document;
use crate::parser::syntax::{DocumentElementP, ResultSP};
use crate::token::TokenLine;
use casual_logger::Table;

/// Syntax scanner.  
/// 構文走査器。  
pub struct SyntaxScanner {
    pub broad_line_p: DocumentElementP,
}
impl Default for SyntaxScanner {
    fn default() -> Self {
        SyntaxScanner {
            broad_line_p: DocumentElementP::default(),
        }
    }
}
impl SyntaxScanner {
    /// # Returns
    ///
    /// * `ResultSP` - Result.  
    ///                             結果。
    pub fn scan_line(&mut self, token_line: &TokenLine, dom: &mut Document) -> ResultSP {
        for (i, token) in token_line.tokens.iter().enumerate() {
            match self.broad_line_p.parse(token) {
                ResultSP::End => {
                    if let Some(child_m) = self.broad_line_p.flush() {
                        dom.push_broad_line(&child_m);
                    } else {
                        let mut err_token_line = TokenLine::new(token_line.row_number);
                        err_token_line.tokens = token_line.tokens[i..].to_vec();
                        return ResultSP::Err(
                            self.err_table()
                                .str("token_line", &format!("{:?}", token_line))
                                .str("rest", &format!("{:?}", err_token_line))
                                .clone(),
                        );
                    }
                }
                ResultSP::Err(table) => {
                    return ResultSP::Err(
                        self.err_table()
                            .str("token_line", &format!("{:?}", token_line))
                            .sub_t("error", &table)
                            .clone(),
                    );
                }
                ResultSP::Ongoing => {}
            }
        }

        ResultSP::Ongoing
    }

    pub fn err_table(&self) -> Table {
        let mut t = Table::default();
        t.str("parser", "syntax_scanner.rs/SyntaxScanner#scan_line")
            .sub_t("line", &self.broad_line_p.log_table());
        t
    }
}
