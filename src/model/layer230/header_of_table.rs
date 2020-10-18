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
    pub fn push_token(&mut self, token: &Token) {
        self.tokens.push(token.clone());
    }
}
impl fmt::Display for HeaderOfTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&format!("{}", token.value));
        }
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for HeaderOfTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&format!("{:?}", token.value));
        }
        write!(f, "{}", buf)
    }
}
