//! Single quoted string model.  
//! 単一引用符文字列モデル。  
//!
//! # Examples
//!
//! ```
//! // 'ハロー'
//! ```

use crate::model::{layer110::token::Token, layer210::SingleQuotedString};
use std::fmt;

impl Default for SingleQuotedString {
    fn default() -> Self {
        SingleQuotedString {
            value: String::new(),
        }
    }
}
impl SingleQuotedString {
    pub fn push_token(&mut self, token: &Token) {
        self.value.push_str(&token.value);
    }
}
impl fmt::Debug for SingleQuotedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}'", self.value)
    }
}
