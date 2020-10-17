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
        HeaderOfTable {
            value: String::new(),
        }
    }
}
impl HeaderOfTable {
    pub fn push_token(&mut self, token: &Token) {
        self.value.push_str(&token.value);
    }
}
impl fmt::Display for HeaderOfTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl fmt::Debug for HeaderOfTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}]", self.value)
    }
}
