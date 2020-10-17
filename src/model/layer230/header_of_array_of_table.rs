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
        HeaderOfArrayOfTable {
            value: String::new(),
        }
    }
}
impl HeaderOfArrayOfTable {
    pub fn push_token(&mut self, token: &Token) {
        self.value.push_str(&token.value);
    }
}
impl fmt::Display for HeaderOfArrayOfTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[[{}]]", self.value)
    }
}
impl fmt::Debug for HeaderOfArrayOfTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[[{:?}]]", self.value)
    }
}
