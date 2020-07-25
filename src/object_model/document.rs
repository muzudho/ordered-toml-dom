//! Document model.  
//! ドキュメント・モデル。  

use crate::object_model::line::LineM;
use std::fmt;

pub struct DocumentM {
    items: Vec<LineM>,
}
impl Default for DocumentM {
    fn default() -> Self {
        DocumentM { items: Vec::new() }
    }
}
impl DocumentM {
    pub fn push(&mut self, item: &LineM) {
        self.items.push(item.clone());
    }
}
impl fmt::Debug for DocumentM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for item in &self.items {
            buf.push_str(&format!(
                "{:?}
",
                item
            ));
        }
        write!(f, "{}", buf)
    }
}
