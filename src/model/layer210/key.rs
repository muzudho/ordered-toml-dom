//! Literal string model.  
//! リテラル文字列モデル。  
//!
//! # Examples
//!
//! ```
//! // work_number
//! ```

use crate::model::{layer110::Token, layer210::Key};
use std::fmt;

impl Default for Key {
    fn default() -> Self {
        Key {
            value: String::new(),
        }
    }
}
impl Key {
    pub fn from_token(token: &Token) -> Self {
        Key {
            value: token.value.to_string(),
        }
    }

    pub fn from_str(text: &str) -> Self {
        Key {
            value: text.to_string(),
        }
    }

    pub fn push_token(&mut self, token: &Token) {
        self.value.push_str(&token.value);
    }
}
impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl fmt::Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}
