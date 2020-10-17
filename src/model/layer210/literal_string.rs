//! Single quoted string model.  
//! 単一引用符文字列モデル。  
//!
//! # Examples
//!
//! ```
//! // 'ハロー'
//! ```

use crate::model::{layer110::Token, layer210::LiteralString};
use std::fmt;

impl Default for LiteralString {
    fn default() -> Self {
        LiteralString { tokens: Vec::new() }
    }
}
impl LiteralString {
    pub fn push_token(&mut self, token: &Token) {
        self.tokens.push(token.clone());
    }
}
impl fmt::Display for LiteralString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&format!("{}", token.value));
        }
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for LiteralString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&format!("{:?}", token.value));
        }
        write!(f, "{}", buf)
    }
}
