//! Comment syntax parser.  
//! コメント構文パーサー。  

use crate::model::{
    layer110::{Token, TokenType},
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
    /// # Arguments
    ///
    /// * `tokens` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, tokens: (Option<&Token>, Option<&Token>, Option<&Token>)) -> PResult {
        let token0 = tokens.0.unwrap();
        match token0.type_ {
            TokenType::EndOfLine => return PResult::End,
            _ => {
                if let None = self.buffer {
                    self.buffer = Some(Comment::default());
                }
                let m = self.buffer.as_mut().unwrap();
                m.push_token(token0);
            }
        }
        PResult::Ongoing
    }
    /// Log.  
    /// ログ。  
    pub fn log(&self) -> Table {
        let mut t = Table::default().str("Parse", "CommentP").clone();
        if let Some(m) = &self.buffer {
            t.str("buffer", &format!("{}", m));
        }
        t
    }
}
