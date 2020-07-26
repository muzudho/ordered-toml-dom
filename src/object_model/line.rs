use crate::object_model::comment::CommentM;
use crate::object_model::key_value::KeyValueM;
use std::fmt;

#[derive(Clone)]
pub struct LineM {
    items: Vec<LineItemModel>,
}
impl Default for LineM {
    fn default() -> Self {
        LineM { items: Vec::new() }
    }
}
impl LineM {
    pub fn push_comment(&mut self, m: &CommentM) {
        self.items.push(LineItemModel::Comment(m.clone()));
    }
    pub fn push_key_value(&mut self, m: &KeyValueM) {
        self.items.push(LineItemModel::KeyValue(m.clone()));
    }
}
impl fmt::Debug for LineM {
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
