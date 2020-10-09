//! Array model.  
//! 配列モデル。  
use crate::model::{
    layer1::{LiteralString, SingleQuotedString},
    Array, DoubleQuotedString, ItemValue,
};
use std::fmt;

impl Default for Array {
    fn default() -> Self {
        Array { items: Vec::new() }
    }
}
impl Array {
    pub fn push_literal_string(&mut self, m: &LiteralString) {
        self.items.push(ItemValue::LiteralString(m.clone()));
    }
    pub fn push_single_quote_string(&mut self, m: &SingleQuotedString) {
        self.items.push(ItemValue::SingleQuotedString(m.clone()));
    }
    pub fn push_double_quote_string(&mut self, m: &DoubleQuotedString) {
        self.items.push(ItemValue::DoubleQuotedString(m.clone()));
    }
}
impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for item in &self.items {
            buf.push_str(&format!("{:?},", item))
        }
        write!(f, "[ {} ]", buf)
    }
}
