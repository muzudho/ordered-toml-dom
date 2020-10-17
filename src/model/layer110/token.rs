//! Token.  
//! 字句。  

use crate::model::layer110::{Token, TokenType};
use std::fmt;

impl Token {
    pub fn new(column_number: usize, value: &str, type_: TokenType) -> Self {
        Token {
            column_number: column_number,
            value: value.to_string(),
            type_: type_,
        }
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}[{:?}]", self.value, self.type_)
    }
}
