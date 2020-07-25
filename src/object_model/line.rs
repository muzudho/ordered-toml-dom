use crate::object_model::comment::CommentM;
use crate::object_model::key_value::KeyValueM;
use std::fmt;

#[derive(Clone)]
pub struct LineM {
    pub items: Vec<LineItemModel>,
}
impl Default for LineM {
    fn default() -> Self {
        LineM { items: Vec::new() }
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
