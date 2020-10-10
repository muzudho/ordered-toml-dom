//! Table model.  
//! テーブル・モデル。  

use crate::model::{layer10::token::Token, layer40::Table};
use std::fmt;

impl Default for Table {
    fn default() -> Self {
        Table {
            value: String::new(),
        }
    }
}
impl Table {
    pub fn push_token(&mut self, token: &Token) {
        self.value.push_str(&token.value);
    }
}
impl fmt::Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}
