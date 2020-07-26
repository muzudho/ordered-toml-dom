//! Element model.  
//! 要素モデル。  

use crate::model::comment::CommentM;
use crate::model::key_value::KeyValueM;
use std::fmt;

#[derive(Clone)]
pub struct ElementM {
    items: Vec<LineItemModel>,
}
impl Default for ElementM {
    fn default() -> Self {
        ElementM { items: Vec::new() }
    }
}
impl ElementM {
    pub fn push_comment(&mut self, m: &CommentM) {
        self.items.push(LineItemModel::Comment(m.clone()));
    }
    pub fn push_key_value(&mut self, m: &KeyValueM) {
        self.items.push(LineItemModel::KeyValue(m.clone()));
    }
}
impl fmt::Debug for ElementM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for item in &self.items {
            buf.push_str(&format!("{:?}", item));
        }
        write!(f, "{}", buf)
    }
}

#[derive(Clone)]
pub enum LineItemModel {
    Comment(CommentM),
    KeyValue(KeyValueM),
}
impl fmt::Debug for LineItemModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LineItemModel::Comment(m) => write!(f, "{}", format!("{:?}", m)),
            LineItemModel::KeyValue(m) => write!(f, "{}", format!("{:?}", m)),
        }
    }
}
