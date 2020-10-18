//! Model of header of table.  
//! テーブル・ヘッダー・モデル。  
//!
//! # Examples
//!
//! ```
//! // [name.name.name]
//! ```

use crate::model::{layer110::Token, layer230::HeaderOfTable};
use std::fmt;

impl Default for HeaderOfTable {
    fn default() -> Self {
        HeaderOfTable { tokens: Vec::new() }
    }
}
impl HeaderOfTable {
    pub fn extend_tokens(&mut self, tokens: &Vec<Token>) {
        self.tokens.extend(*tokens);
    }
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
impl fmt::Display for HeaderOfTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&token.to_string());
        }
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for HeaderOfTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&token.to_debug_string());
        }
        write!(f, "{}", buf)
    }
}
