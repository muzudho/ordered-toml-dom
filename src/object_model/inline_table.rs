//! Inline model.  
//! インライン・モデル。  

use crate::object_model::key_value::KeyValueM;
use crate::object_model::value::ValueM;
use std::fmt;

#[derive(Clone)]
pub struct InlineTableM {
    items: Vec<ValueM>,
}
impl Default for InlineTableM {
    fn default() -> Self {
        InlineTableM { items: Vec::new() }
    }
}
impl InlineTableM {
    pub fn push_key_value(&mut self, m: &KeyValueM) {
        self.items.push(ValueM::KeyValue(m.clone()));
    }
}
impl fmt::Debug for InlineTableM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for item in &self.items {
            buf.push_str(&format!("{:?},", item))
        }
        write!(f, "{{ {} }}", buf)
    }
}
