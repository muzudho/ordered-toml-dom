//! Literal string model.  
//! リテラル文字列モデル。  
//!
//! # Examples
//!
//! ```
//! // work_number
//! ```

use crate::model::{
    layer110::{Character, Token, TokenType},
    layer210::LiteralValue,
};
use std::fmt;

impl Default for LiteralValue {
    fn default() -> Self {
        LiteralValue { tokens: Vec::new() }
    }
}
impl LiteralValue {
    pub fn from_character(character: &Character) -> Self {
        let mut m = LiteralValue::default();
        m.tokens.push(Token::new(
            character.column_number,
            &character.value,
            TokenType::LiteralValue,
        ));
        m
    }
    pub fn from_token(token: &Token) -> Self {
        let mut m = LiteralValue::default();
        m.tokens.push(token.clone());
        m
    }

    pub fn extend_tokens(&mut self, tokens: &Vec<Token>) {
        self.tokens.extend(tokens.clone());
    }
    pub fn push_token(&mut self, token: &Token) {
        self.tokens.push(token.clone());
    }
    pub fn to_debug_string(&self) -> String {
        format!("{:?}", self)
    }
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}
impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&token.to_string());
        }
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&token.to_debug_string());
        }
        write!(f, "{}", buf)
    }
}
