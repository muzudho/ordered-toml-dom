use crate::token::Token;
use std::fmt;

#[derive(Clone)]
pub struct LiteralStringM {
    value: String,
}
impl Default for LiteralStringM {
    fn default() -> Self {
        LiteralStringM {
            value: String::new(),
        }
    }
}
impl LiteralStringM {
    pub fn new(token: &Token) -> Self {
        LiteralStringM {
            value: token.value.to_string(),
        }
    }
    pub fn push_token(&mut self, token: &Token) {
        self.value.push_str(&token.value);
    }
}
impl fmt::Debug for LiteralStringM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
