//! Document syntax parser.  
//! ドキュメント構文解析器。  

use crate::model::{layer110::token::TokenLine, layer310::Document};
use crate::parser::phase200::{layer210::PResult, layer230::DocumentElementP};
use casual_logger::Table;

/// Document syntax parser.  
/// ドキュメント構文解析器。  
pub struct DocumentParser {
    pub document_element_p: DocumentElementP,
}
impl Default for DocumentParser {
    fn default() -> Self {
        DocumentParser {
            document_element_p: DocumentElementP::default(),
        }
    }
}
impl DocumentParser {
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn scan_line(&mut self, token_line: &TokenLine, doc: &mut Document) -> PResult {
        for (i, token) in token_line.tokens.iter().enumerate() {
            match self.document_element_p.parse(token) {
                PResult::End => {
                    if let Some(m) = self.document_element_p.flush() {
                        doc.push_element(&m);
                    } else {
                        let remaining_tokens = token_line.remaining_tokens(i);
                        return PResult::Err(
                            self.log_snapshot()
                                .str("place_of_occurrence", "document.rs.34.")
                                .str("token_line", &format!("{:?}", token_line))
                                .str("remaining_tokens", &format!("{:?}", remaining_tokens))
                                .clone(),
                        );
                    }
                }
                PResult::Err(mut table) => {
                    return PResult::Err(
                        table
                            .sub_t(
                                "snapshot",
                                self.log_snapshot()
                                    .str("place_of_occurrence", "document.rs.43.")
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
        t.str("parser", "document_parser.rs")
            .sub_t("line", &self.document_element_p.log_table("no-data"));
        t
    }
}
