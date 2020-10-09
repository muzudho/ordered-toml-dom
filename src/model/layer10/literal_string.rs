//! Literal string model.  
//! リテラル文字列モデル。  
//!
//! # Examples
//!
//! ```
//! // work_number
//! ```

use crate::model::layer10::LiteralString;
use crate::token::Token;
use std::fmt;

impl Default for LiteralString {
    fn default() -> Self {
        LiteralString {
            value: String::new(),
        }
    }
}
impl LiteralString {
    pub fn new(token: &Token) -> Self {
        LiteralString {
            value: token.value.to_string(),
        }
    }
    pub fn push_token(&mut self, token: &Token) {
        self.value.push_str(&token.value);
    }
}
impl fmt::Debug for LiteralString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
