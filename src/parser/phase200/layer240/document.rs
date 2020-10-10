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
                            self.err_table()
                                .str("token_line", &format!("{:?}", token_line))
                                .str("remaining_tokens", &format!("{:?}", remaining_tokens))
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
        t.str("parser", "document_parser.rs/DocumentParser#scan_line")
            .sub_t("line", &self.document_element_p.log_table());
        t
    }
}
