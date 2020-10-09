//! Array of table model.  
//! テーブルの配列モデル。  

use crate::model::layer30::ArrayOfTable;
use crate::token::Token;
use std::fmt;

impl Default for ArrayOfTable {
    fn default() -> Self {
        ArrayOfTable {
            value: String::new(),
        }
    }
}
impl ArrayOfTable {
    pub fn push_token(&mut self, token: &Token) {
        self.value.push_str(&token.value);
    }
}
impl fmt::Debug for ArrayOfTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}
