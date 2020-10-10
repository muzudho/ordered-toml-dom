//! Comment model.  
//! コメント・モデル。  
//!
//! # Examples
//!
//! ```
//! // # Toml's comment（トムルのコメント）
//! ```

use crate::model::{layer10::Comment, layer5::token::Token};
use std::fmt;

impl Default for Comment {
    fn default() -> Self {
        Comment {
            value: String::new(),
        }
    }
}
impl Comment {
    pub fn push_token(&mut self, token: &Token) {
        self.value.push_str(&token.value);
    }
}
impl fmt::Debug for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.value)
    }
}
