//! Inline table model.  
//! インライン・テーブル・モデル。  
//!
//! # Examples
//!
//! ```
//! // { name="a", pass="b", age=3 }
//! ```

use crate::model::layer225::{InlineTable, KeyValue};
use std::fmt;

impl Default for InlineTable {
    fn default() -> Self {
        InlineTable { items: Vec::new() }
    }
}
impl InlineTable {
    pub fn push_keyval(&mut self, m: &KeyValue) {
        self.items.push(m.clone());
    }
    pub fn to_debug_string(&self) -> String {
        format!("{:?}", self)
    }
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}
impl fmt::Display for InlineTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for item in &self.items {
            buf.push_str(&format!("{},", item))
        }
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for InlineTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for item in &self.items {
            buf.push_str(&format!("{:?},", item))
        }
        write!(f, "{{ {} }}", buf)
    }
}
