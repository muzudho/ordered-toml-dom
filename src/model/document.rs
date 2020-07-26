//! Document model.  
//! ドキュメント・モデル。  

use crate::model::{Document, Element};
use std::fmt;

impl Default for Document {
    fn default() -> Self {
        Document {
            elements: Vec::new(),
        }
    }
}
impl Document {
    pub fn child(&self, name: &str) -> Option<&Element> {
        for elem in &self.elements {
            match elem {
                Element::Comment(_) => {}
                Element::KeyValue(m) => {
                    println!("m.key={}", m.key);
                    if m.key == name {
                        println!("HIT m.key={}", m.key);
                        return Some(elem);
                    }
                }
            }
        }
        None
    }
    pub fn push_line(&mut self, m: &Element) {
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
