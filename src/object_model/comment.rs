use crate::lexical_parser::Token;
use std::fmt;

#[derive(Clone)]
pub struct CommentM {
    pub value: String,
}
impl Default for CommentM {
    fn default() -> Self {
        CommentM {
            value: String::new(),
        }
    }
}
impl CommentM {
    pub fn push_token(&mut self, token: &Token) {
        self.value.push_str(&token.value);
    }
}
impl fmt::Debug for CommentM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.value)
    }
}
