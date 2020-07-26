//! Array model.  
//! 配列モデル。  
use crate::model::literal_string::LiteralStringM;
use crate::model::single_quoted_string::SingleQuotedStringM;
use crate::model::value::ValueM;
use std::fmt;

#[derive(Clone)]
pub struct ArrayM {
    items: Vec<ValueM>,
}
impl Default for ArrayM {
    fn default() -> Self {
        ArrayM { items: Vec::new() }
    }
}
impl ArrayM {
    pub fn push_literal_string(&mut self, m: &LiteralStringM) {
        self.items.push(ValueM::LiteralString(m.clone()));
    }
    pub fn push_single_quote_string(&mut self, m: &SingleQuotedStringM) {
        self.items.push(ValueM::SingleQuotedString(m.clone()));
    }
}
impl fmt::Debug for ArrayM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for item in &self.items {
            buf.push_str(&format!("{:?},", item))
        }
        write!(f, "[ {} ]", buf)
    }
}
