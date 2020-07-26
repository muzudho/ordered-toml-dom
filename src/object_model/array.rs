//! Array model.  
//! 配列モデル。  
use crate::object_model::value::ValueM;
use std::fmt;

#[derive(Clone)]
pub struct ArrayM {
    pub items: Vec<ValueM>,
}
impl Default for ArrayM {
    fn default() -> Self {
        ArrayM { items: Vec::new() }
    }
}
impl fmt::Debug for ArrayM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for item in &self.items {
            buf.push_str(&format!("{:?},", item))
        }
        write!(f, "{{ {} }}", buf)
    }
}
