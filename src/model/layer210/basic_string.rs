//! Double quoted string model.  
//! 二重引用符文字列モデル。  
//!
//! # Examples
//!
//! ```
//! // "ハロー"
//! ```

use crate::model::{layer110::Token, layer210::BasicString};
use std::fmt;

impl Default for BasicString {
    fn default() -> Self {
        BasicString { tokens: Vec::new() }
    }
}
impl BasicString {
    pub fn push_token(&mut self, token: &Token) {
        self.tokens.push(token.clone());
    }
}
impl fmt::Display for BasicString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&format!("{}", token.value));
        }
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for BasicString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&format!("{:?}", token.value));
        }
        write!(f, "{}", buf)
    }
}
