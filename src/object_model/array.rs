//! Array model.  
//! 配列モデル。  
use std::fmt;

#[derive(Clone)]
pub struct ArrayM {
    pub items: Vec<ArrayItem>,
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

#[derive(Clone)]
pub enum ArrayItem {
    String(String),
}
impl fmt::Debug for ArrayItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArrayItem::String(s) => write!(f, "{}", s),
        }
    }
}
