//! DocumentElement model.  
//! 縦幅のある行 モデル。  

use crate::model::{
    layer210::Comment,
    layer225::KeyValue,
    layer230::{DocumentElement, HeaderOfArrayOfTable, HeaderOfTable},
};
use std::fmt;

impl DocumentElement {
    pub fn from_header_of_array_of_table(m: &HeaderOfArrayOfTable) -> Self {
        DocumentElement::HeaderOfArrayOfTable(m.clone())
    }
    pub fn from_comment(m: &Comment) -> Self {
        DocumentElement::Comment(m.clone())
    }
    pub fn from_key_value(m: &KeyValue) -> Self {
        DocumentElement::KeyValue(m.clone())
    }
    pub fn from_header_of_table(m: &HeaderOfTable) -> Self {
        DocumentElement::HeaderOfTable(m.clone())
    }
    pub fn to_debug_string(&self) -> String {
        format!("{}", self)
    }
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}
impl fmt::Display for DocumentElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DocumentElement::HeaderOfArrayOfTable(m) => write!(f, "{}", m),
            DocumentElement::Comment(m) => write!(f, "{}", m),
            DocumentElement::EmptyLine => write!(f, ""),
            DocumentElement::KeyValue(m) => write!(f, "{}", m),
            DocumentElement::HeaderOfTable(m) => write!(f, "{}", m),
        }
    }
}
impl fmt::Debug for DocumentElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DocumentElement::HeaderOfArrayOfTable(m) => write!(f, "{:?}", m),
            DocumentElement::Comment(m) => write!(f, "{:?}", m),
            DocumentElement::EmptyLine => write!(f, ""),
            DocumentElement::KeyValue(m) => write!(f, "{:?}", m),
            DocumentElement::HeaderOfTable(m) => write!(f, "{:?}", m),
        }
    }
}
