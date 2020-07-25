use crate::object_model::line::LineModel;
use std::fmt;

pub struct DocumentModel {
    pub items: Vec<LineModel>,
}
impl DocumentModel {}
impl fmt::Debug for DocumentModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for item in &self.items {
            buf.push_str(&format!("{:?}", item));
        }
        write!(f, "{}", buf)
    }
}
