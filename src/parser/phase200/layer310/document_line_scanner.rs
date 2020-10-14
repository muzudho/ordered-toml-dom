//! Document syntax parser.  
//! ドキュメント構文解析器。  

use crate::model::{layer110::token::TokenLine, layer310::Document};
use crate::parser::phase200::{
    error_via,
    {layer210::PResult, layer230::DocumentElementP, layer310::DocumentLineScanner},
};
use casual_logger::Table;

impl Default for DocumentLineScanner {
    fn default() -> Self {
        DocumentLineScanner {
            document_element_p: None,
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
            if let None = self.document_element_p {
                self.document_element_p = Some(DocumentElementP::default());
            }
            let p = self.document_element_p.as_mut().unwrap();
            match p.parse(token) {
                PResult::End => {
                    if let Some(m) = p.flush() {
                        doc.push_element(&m);
                        self.document_element_p = None;
                    } else {
                        // TODO 何も取得できないことがある？
                        /*
                        return error(&mut self.log(), token, "document.rs.34.");
                        */
                    }
                }
                PResult::Err(mut table) => {
                    return error_via(&mut table, &mut self.log(), token, "document.rs.43.");
                }
                PResult::Ongoing => {}
            }
        }

        PResult::Ongoing
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> Table {
        let mut t = Table::default();
        if let Some(p) = &self.document_element_p {
            t.sub_t("document_element_p", &p.log());
        }
        t
    }
}
