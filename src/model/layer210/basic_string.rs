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
        BasicString {
            value: String::new(),
        }
    }
}
impl BasicString {
    pub fn push_token(&mut self, token: &Token) {
        self.value.push_str(&token.value);
    }
}
impl fmt::Display for BasicString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}
impl fmt::Debug for BasicString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{:?}\"", self.value)
    }
}
