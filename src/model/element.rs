//! Element model.  
//! 要素モデル。  

use crate::model::{Comment, Element, KeyValue};
use std::fmt;

impl Element {
    pub fn from_comment(m: &Comment) -> Self {
        Element::Comment(m.clone())
    }
    pub fn from_key_value(m: &KeyValue) -> Self {
        Element::KeyValue(m.clone())
    }
}
impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Element::Comment(m) => write!(f, "{}", format!("{:?}", m)),
            Element::KeyValue(m) => write!(f, "{}", format!("{:?}", m)),
        }
    }
}
