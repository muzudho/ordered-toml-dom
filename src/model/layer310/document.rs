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
    pub fn child(&self, name: &str) -> Option<&DocumentElement> {
        for elem in &self.elements {
            match elem {
                DocumentElement::ArrayOfTable(_) => {
                    // TODO
                }
                DocumentElement::Comment(_) => {}
                DocumentElement::EmptyLine => {}
                DocumentElement::KeyValue(m) => {
                    println!("m.key={}", m.key);
                    if m.key == name {
                        println!("HIT m.key={}", m.key);
                        return Some(elem);
                    }
                }
                DocumentElement::Table(_) => {
                    // TODO
                }
            }
        }
        None
    }
    pub fn push_broad_line(&mut self, m: &DocumentElement) {
        self.elements.push(m.clone());
    }
}
impl fmt::Debug for Document {
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
