//! Document syntax parser.  
//! ドキュメント構文解析器。  

use crate::model::{layer110::token::TokenLine, layer310::Document};
use crate::parser::phase200::{layer210::PResult, layer230::DocumentElementP};
use crate::util::random_name;
use casual_logger::Table;

/// Document syntax parser.  
/// ドキュメント構文解析器。  
pub struct DocumentLineScanner {
    pub document_element_p: DocumentElementP,
}
impl Default for DocumentLineScanner {
    fn default() -> Self {
        DocumentLineScanner {
            document_element_p: DocumentElementP::default(),
        }
    }
}
impl DocumentLineScanner {
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn scan_line(&mut self, token_line: &TokenLine, doc: &mut Document) -> PResult {
        for (_i, token) in token_line.tokens.iter().enumerate() {
            match self.document_element_p.parse(token) {
                PResult::End => {
                    if let Some(m) = self.document_element_p.flush() {
                        doc.push_element(&m);
                    } else {
                        // TODO 何も取得できないことがある？
                        /*
                        return PResult::Err(
                            self.log_snapshot()
                                .str("place_of_occurrence", "document.rs.34.")
                                .str("token_index", &format!("{:?}", i))
                                .str("token", &format!("{:?}", token))
                                .str("token_line", &format!("{:?}", token_line))
                                .str("remaining_tokens", &format!("{:?}", token_line.remaining_tokens(i)))
                                .clone(),
                        );
                        */
                    }
                }
                PResult::Err(mut table) => {
                    return PResult::Err(
                        table
                            .sub_t(
                                &random_name(),
                                self.log_snapshot()
                                    .str("via", "document.rs.43.")
                                    .str("token_line", &format!("{:?}", token_line)),
                            )
                            .clone(),
                    );
                }
                PResult::Ongoing => {}
            }
        }

        PResult::Ongoing
    }

    pub fn log_snapshot(&self) -> Table {
        let mut t = Table::default();
        t.sub_t(
            "document_element_p",
            &self.document_element_p.log_snapshot(),
        );
        t
    }
}
