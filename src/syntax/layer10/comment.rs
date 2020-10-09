//! Comment syntax parser.  
//! コメント構文パーサー。  

use crate::model::layer10::Comment;
use crate::syntax::{layer10::CommentP, SyntaxParserResult};
use crate::token::{Token, TokenType};
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
    /// * `SyntaxParserResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token) -> SyntaxParserResult {
        match token.type_ {
            TokenType::EndOfLine => return SyntaxParserResult::End,
            _ => {
                if let None = self.buffer {
                    self.buffer = Some(Comment::default());
                }
                let m = self.buffer.as_mut().unwrap();
                m.push_token(token);
            }
        }
        SyntaxParserResult::Ongoing
    }
    pub fn log_table(&self) -> Table {
        let mut t = Table::default().str("Parse", "CommentP").clone();
        if let Some(m) = &self.buffer {
            t.str("buffer", &format!("{:?}", m));
        }
        t
    }
}