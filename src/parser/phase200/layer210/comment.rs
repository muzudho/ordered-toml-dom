//! Comment syntax parser.  
//! コメント構文パーサー。  

use crate::model::{
    layer110::token::{Token, TokenType},
    layer210::Comment,
};
use crate::parser::phase200::layer210::{CommentP, PResult};
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
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, look_ahead_token: Option<&Token>, token: &Token) -> PResult {
        match token.type_ {
            TokenType::EndOfLine => return PResult::End,
            _ => {
                if let None = self.buffer {
                    self.buffer = Some(Comment::default());
                }
                let m = self.buffer.as_mut().unwrap();
                m.push_token(token);
            }
        }
        PResult::Ongoing
    }
    /// Log.  
    /// ログ。  
    pub fn log(&self) -> Table {
        let mut t = Table::default().str("Parse", "CommentP").clone();
        if let Some(m) = &self.buffer {
            t.str("buffer", &format!("{:?}", m));
        }
        t
    }
}
