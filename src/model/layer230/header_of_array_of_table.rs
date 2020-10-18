//! Model of header of array of table.  
//! テーブルの配列のヘッダー・モデル。  
//!
//! # Examples
//!
//! ```
//! // [[name.name.name]]
//! ```

use crate::model::{layer110::Token, layer230::HeaderOfArrayOfTable};
use std::fmt;

impl Default for HeaderOfArrayOfTable {
    fn default() -> Self {
        HeaderOfArrayOfTable { tokens: Vec::new() }
    }
}
impl HeaderOfArrayOfTable {
    pub fn push_token(&mut self, token: &Token) {
        self.tokens.push(token.clone());
    }
}
impl fmt::Display for HeaderOfArrayOfTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&format!("{}", token.value));
        }
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for HeaderOfArrayOfTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&format!("{:?}", token.value));
        }
        write!(f, "{}", buf)
    }
}
