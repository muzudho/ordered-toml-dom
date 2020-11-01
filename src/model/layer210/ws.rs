//! Non end-of-line model.  
//! 非行末モデル。  

use crate::model::{layer110::Token, layer210::Ws};
use std::fmt;

impl Default for Ws {
    fn default() -> Self {
        Ws { tokens: Vec::new() }
    }
}
impl Ws {
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
impl fmt::Display for Ws {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&token.to_string());
        }
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for Ws {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&token.to_debug_string());
        }
        write!(f, "{}", buf)
    }
}
