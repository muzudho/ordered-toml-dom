//! Literal string model.  
//! リテラル文字列モデル。  
//!
//! # Examples
//!
//! ```
//! // work_number
//! ```

use crate::model::{layer110::token::Token, layer210::LiteralValue};
use std::fmt;

impl Default for LiteralValue {
    fn default() -> Self {
        LiteralValue {
            value: String::new(),
        }
    }
}
impl LiteralValue {
    pub fn from_token(token: &Token) -> Self {
        LiteralValue {
            value: token.value.to_string(),
        }
    }

    pub fn from_str(text: &str) -> Self {
        LiteralValue {
            value: text.to_string(),
        }
    }

    pub fn push_token(&mut self, token: &Token) {
        self.value.push_str(&token.value);
    }
}
impl fmt::Debug for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
