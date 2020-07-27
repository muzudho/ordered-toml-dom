//! Document model.  
//! ドキュメント・モデル。  

use crate::model::{BroadLine, Document};
use std::fmt;

impl Default for Document {
    fn default() -> Self {
        Document {
            broad_lines: Vec::new(),
        }
    }
}
impl Document {
    pub fn child(&self, name: &str) -> Option<&BroadLine> {
        for elem in &self.broad_lines {
            match elem {
                BroadLine::Comment(_) => {}
                BroadLine::KeyValue(m) => {
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
    pub fn push_line(&mut self, m: &BroadLine) {
        self.broad_lines.push(m.clone());
    }
}
impl fmt::Debug for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for item in &self.broad_lines {
            buf.push_str(&format!(
                "{:?}
",
                item
            ));
        }
        write!(f, "{}", buf)
    }
}
