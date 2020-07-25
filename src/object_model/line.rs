use crate::object_model::key_value::KeyValueModel;
use std::fmt;

pub struct LineModel {
    pub items: Vec<LineModelItem>,
}
impl Default for LineModel {
    fn default() -> Self {
        LineModel { items: Vec::new() }
    }
}
impl fmt::Debug for LineModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for item in &self.items {
            buf.push_str(&format!("{:?}", item));
        }
        write!(f, "{}", buf)
    }
}

pub enum LineModelItem {
    KeyValue(KeyValueModel),
}
impl fmt::Debug for LineModelItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LineModelItem::KeyValue(m) => write!(f, "{}", format!("{:?}", m)),
        }
    }
}
