//! Element model.  
//! 要素モデル。  

use crate::model::comment::CommentM;
use crate::model::key_value::KeyValueM;
use std::fmt;

#[derive(Clone)]
pub enum ElementM {
    Comment(CommentM),
    KeyValue(KeyValueM),
}
impl ElementM {
    pub fn from_comment(m: &CommentM) -> Self {
        ElementM::Comment(m.clone())
    }
    pub fn from_key_value(m: &KeyValueM) -> Self {
        ElementM::KeyValue(m.clone())
    }
}
impl fmt::Debug for ElementM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ElementM::Comment(m) => write!(f, "{}", format!("{:?}", m)),
            ElementM::KeyValue(m) => write!(f, "{}", format!("{:?}", m)),
        }
    }
}
