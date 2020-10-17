//! Literal string model.  
//! リテラル文字列モデル。  
//!
//! # Examples
//!
//! ```
//! // work_number
//! ```

use crate::model::{layer110::Token, layer210::LiteralValue};
use std::fmt;

impl Default for LiteralValue {
    fn default() -> Self {
        LiteralValue { tokens: Vec::new() }
    }
}
impl LiteralValue {
    pub fn from_token(token: &Token) -> Self {
        let mut m = LiteralValue::default();
        m.tokens.push(token.clone());
        m
    }

    pub fn push_token(&mut self, token: &Token) {
        self.tokens.push(token.clone());
    }
}
impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&format!("{}", token));
        }
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&format!("{:?}", token));
        }
        write!(f, "{}", buf)
    }
}
