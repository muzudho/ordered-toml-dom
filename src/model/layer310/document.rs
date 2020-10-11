//! Document model.  
//! ドキュメント・モデル。  

use crate::model::{layer230::DocumentElement, layer310::Document};
use std::fmt;

impl Default for Document {
    fn default() -> Self {
        Document {
            elements: Vec::new(),
        }
    }
}
impl Document {
    /// WIP.
    pub fn get_element_by_name(&self, name: &str) -> Option<&DocumentElement> {
        for elem in &self.elements {
            match elem {
                DocumentElement::HeaderOfArrayOfTable(_) => {
                    // TODO
                }
                DocumentElement::Comment(_) => {}
                DocumentElement::EmptyLine => {}
                DocumentElement::KeyValue(m) => {
                    // println!("m.key={}", m.key); // In development.
                    if m.key == name {
                        // println!("HIT m.key={}", m.key);// In development.
                        return Some(elem);
                    }
                }
                DocumentElement::HeaderOfTable(_) => {
                    // TODO
                }
            }
        }
        None
    }
    pub fn push_element(&mut self, m: &DocumentElement) {
        self.elements.push(m.clone());
    }
}
impl fmt::Debug for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for elem in &self.elements {
            buf.push_str(&format!(
                "{:?}
",
                elem
            ));
        }
        write!(f, "{}", buf)
    }
}
