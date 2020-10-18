use crate::model::layer110::TokenLine;
use std::fmt;

impl TokenLine {
    pub fn new(row_number: usize) -> Self {
        TokenLine {
            row_number: row_number,
            tokens: Vec::new(),
        }
    }

    /// Remaining tokens.
    /// 残りのトークン。
    pub fn remaining_tokens(&self, token_index: usize) -> Self {
        TokenLine {
            row_number: self.row_number,
            tokens: self.tokens[token_index..].to_vec(),
        }
    }
}
impl fmt::Display for TokenLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&token.to_string());
        }
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for TokenLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&token.to_debug_string());
        }
        write!(f, "{}", buf)
    }
}
