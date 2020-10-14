//! Document syntax parser.  
//! ドキュメント構文解析器。  

use crate::model::{
    layer110::token::{Token, TokenLine},
    layer310::Document,
};
use crate::parser::phase200::{
    error_via,
    {layer210::PResult, layer230::DocumentElementP, layer310::DocumentP},
};
use casual_logger::Table;

impl Default for DocumentP {
    fn default() -> Self {
        DocumentP {
            document_element_p: None,
        }
    }
}
impl DocumentP {
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn scan_line(&mut self, token_line: &TokenLine, doc: &mut Document) -> PResult {
        // 先読みトークン。
        let mut look_ahead_token: Option<&Token> = None;
        for (_i, token) in token_line.tokens.iter().enumerate() {
            if let Some(look_ahead_token) = look_ahead_token {
                // The current token is the look-ahead token, and the previous look-ahead token is the current token.
                // 現在のトークンは先読みトークン、前回の先読みトークンは今回のトークンです。
                self.one_delay_loop(Some(token), look_ahead_token, doc);
            }
            look_ahead_token = Some(token);
        }
        // Last 1 token.
        // 最後の１トークン。
        if let Some(look_ahead_token) = look_ahead_token {
            self.one_delay_loop(None, look_ahead_token, doc);
        }

        PResult::Ongoing
    }

    /// One delay loop.  
    /// １つ遅れループ。  
    fn one_delay_loop(
        &mut self,
        look_ahead_token: Option<&Token>,
        token: &Token,
        doc: &mut Document,
    ) -> PResult {
        if let None = self.document_element_p {
            self.document_element_p = Some(DocumentElementP::default());
        }
        let p = self.document_element_p.as_mut().unwrap();
        match p.parse(look_ahead_token, token) {
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
