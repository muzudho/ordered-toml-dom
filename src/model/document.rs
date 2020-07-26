//! Document model.  
//! ドキュメント・モデル。  

use crate::model::element::ElementM;
use std::fmt;

pub struct DocumentM {
    pub elements: Vec<ElementM>,
}
impl Default for DocumentM {
    fn default() -> Self {
        DocumentM {
            elements: Vec::new(),
        }
    }
}
impl DocumentM {
    pub fn push_line(&mut self, m: &ElementM) {
        self.elements.push(m.clone());
    }
}
impl fmt::Debug for DocumentM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for item in &self.elements {
            buf.push_str(&format!(
                "{:?}
",
                item
            ));
        }
        write!(f, "{}", buf)
    }
}
