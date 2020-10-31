//! Comment syntax parser.  
//! コメント構文パーサー。  

use crate::model::{layer110::TokenType, layer210::Comment};
use crate::parser::phase200::layer210::{CommentP, PResult};
use crate::parser::phase200::LookAheadTokens;
use casual_logger::Table;

impl CommentP {
    pub fn new() -> Self {
        CommentP { buffer: None }
    }
    pub fn flush(&mut self) -> Option<Comment> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    /// # Arguments
    ///
    /// * `tokens` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, tokens: &LookAheadTokens) -> PResult {
        let token0 = tokens.current.as_ref().unwrap();
        match token0.type_ {
            TokenType::Newline => return PResult::End,
            _ => {
                if let None = self.buffer {
                    self.buffer = Some(Comment::default());
                }
                let m = self.buffer.as_mut().unwrap();
                m.push_token(&token0);
            }
        }
        PResult::Ongoing
    }
    /// Log.  
    /// ログ。  
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();
        if let Some(m) = &self.buffer {
            t.str("buffer", &m.to_string());
        }
        t
    }
}
