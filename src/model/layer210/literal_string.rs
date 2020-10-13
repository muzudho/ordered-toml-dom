//! Literal string model.  
//! リテラル文字列モデル。  
//!
//! # Examples
//!
//! ```
//! // work_number
//! ```

use crate::model::{layer110::token::Token, layer210::LiteralString};
use std::fmt;

impl Default for LiteralString {
    fn default() -> Self {
        LiteralString {
            value: String::new(),
        }
    }
}
impl LiteralString {
    pub fn from_token(token: &Token) -> Self {
        LiteralString {
            value: token.value.to_string(),
        }
    }

    pub fn from_str(text: &str) -> Self {
        LiteralString {
            value: text.to_string(),
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
