use crate::model::SingleQuotedStringM;
use crate::token::Token;
use std::fmt;

impl Default for SingleQuotedStringM {
    fn default() -> Self {
        SingleQuotedStringM {
            value: String::new(),
        }
    }
}
impl SingleQuotedStringM {
    pub fn push_token(&mut self, token: &Token) {
        self.value.push_str(&token.value);
    }
}
impl fmt::Debug for SingleQuotedStringM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}'", self.value)
    }
}
