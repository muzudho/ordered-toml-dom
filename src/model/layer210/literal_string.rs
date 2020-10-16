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
        LiteralString {
            value: String::new(),
        }
    }
}
impl LiteralString {
    pub fn push_token(&mut self, token: &Token) {
        self.value.push_str(&token.value);
    }
}
impl fmt::Debug for LiteralString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}'", self.value)
    }
}
