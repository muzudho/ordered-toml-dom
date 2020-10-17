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
        Key { tokens: Vec::new() }
    }
}
impl Key {
    pub fn from_token(token: &Token) -> Self {
        let mut m = Key::default();
        m.push_token(token);
        m
    }

    pub fn push_token(&mut self, token: &Token) {
        self.tokens.push(token.clone());
    }
}
impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&format!("{}", token.value));
        }
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&format!("{:?}", token.value));
        }
        write!(f, "{}", buf)
    }
}
