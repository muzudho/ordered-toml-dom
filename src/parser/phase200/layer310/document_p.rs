//! Document syntax parser.  
//! ドキュメント構文解析器。  

use crate::model::{layer110::TokenLine, layer310::Document};
use crate::parser::phase200::LookAheadTokens;
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
        // * `tokens` - Tokens contains look ahead.
        //             先読みを含むトークン。
        // (Current token, 1 ahead token, 2 ahead token)
        // （現在のトークン, １つ先のトークン，２つ先のトークン）
        let mut tokens = LookAheadTokens::default();
        for (_i, token) in token_line.tokens.iter().enumerate() {
            // Shift.
            // The current token is the look-ahead token, and the previous look-ahead token is the current token.
            // ずらします。
            // 現在のトークンは先読みトークン、前回の先読みトークンは今回のトークンです。
            tokens.push(Some(token.clone()));
            if let Some(_) = tokens.current {
                match self.one_delay_loop(&tokens, doc) {
                    PResult::End => {}
                    PResult::Err(mut table) => {
                        return error_via(&mut table, &mut self.log(), &tokens, "document.rs.43.");
                    }
                    PResult::Ongoing => {}
                }
            }
        }

        // Last 4 token.
        // 最後の４トークン。
        tokens.push(None);
        if let Some(_) = tokens.current {
            self.one_delay_loop(&tokens, doc);
        }

        // Last 3 token.
        // 最後の３トークン。
        tokens.push(None);
        if let Some(_) = tokens.current {
            self.one_delay_loop(&tokens, doc);
        }

        // Last 2 token.
        // 最後の２トークン。
        tokens.push(None);
        if let Some(_) = tokens.current {
            self.one_delay_loop(&tokens, doc);
        }

        // Last 1 token.
        // 最後の１トークン。
        tokens.push(None);
        if let Some(_) = tokens.current {
            self.one_delay_loop(&tokens, doc);
        }

        PResult::Ongoing
    }

    /// One delay loop.  
    /// １つ遅れループ。  
    ///
    /// # Arguments
    ///
    /// * `tokens` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    fn one_delay_loop(&mut self, tokens: &LookAheadTokens, doc: &mut Document) -> PResult {
        if let None = self.document_element_p {
            self.document_element_p = Some(DocumentElementP::default());
        }
        let p = self.document_element_p.as_mut().unwrap();
        match p.parse(&tokens) {
            PResult::End => {
                if let Some(m) = p.flush() {
                    doc.push_element(&m);
                    self.document_element_p = None;
                } else {
                    // TODO 何も取得できないことがある？
                    /*
                    return error(&mut self.log(), &tokens, "document.rs.34.");
                    */
                }
            }
            PResult::Err(mut table) => {
                return error_via(&mut table, &mut self.log(), &tokens, "document.rs.92.");
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
