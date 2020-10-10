//! Array of table model.  
//! テーブルの配列モデル。  

use crate::model::{layer110::token::Token, layer230::HeaderOfArrayOfTable};
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
impl fmt::Debug for HeaderOfArrayOfTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}
