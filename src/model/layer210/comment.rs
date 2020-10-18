//! Comment model.  
//! コメント・モデル。  
//!
//! # Examples
//!
//! ```
//! // # Toml's comment（トムルのコメント）
//! ```

use crate::model::{layer110::Token, layer210::Comment};
use std::fmt;

impl Default for Comment {
    fn default() -> Self {
        Comment { tokens: Vec::new() }
    }
}
impl Comment {
    pub fn push_token(&mut self, token: &Token) {
        self.tokens.push(token.clone());
    }
    pub fn to_debug_string(&self) -> String {
        format!("{:?}", self)
    }
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}
impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&token.to_string());
        }
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&token.to_debug_string());
        }
        write!(f, "{}", buf)
    }
}
