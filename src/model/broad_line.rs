//! BroadLine model.  
//! 縦幅のある行 モデル。  

use crate::model::{BroadLine, Comment, KeyValue};
use std::fmt;

impl BroadLine {
    pub fn from_comment(m: &Comment) -> Self {
        BroadLine::Comment(m.clone())
    }
    pub fn from_key_value(m: &KeyValue) -> Self {
        BroadLine::KeyValue(m.clone())
    }
}
impl fmt::Debug for BroadLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BroadLine::Comment(m) => write!(f, "{}", format!("{:?}", m)),
            BroadLine::KeyValue(m) => write!(f, "{}", format!("{:?}", m)),
        }
    }
}
