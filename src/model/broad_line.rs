//! BroadLine model.  
//! 縦幅のある行 モデル。  

use crate::model::{layer1::Comment, ArrayOfTable, BroadLine, KeyValue, Table};
use std::fmt;

impl BroadLine {
    pub fn from_array_of_table(m: &ArrayOfTable) -> Self {
        BroadLine::ArrayOfTable(m.clone())
    }
    pub fn from_comment(m: &Comment) -> Self {
        BroadLine::Comment(m.clone())
    }
    pub fn from_key_value(m: &KeyValue) -> Self {
        BroadLine::KeyValue(m.clone())
    }
    pub fn from_table(m: &Table) -> Self {
        BroadLine::Table(m.clone())
    }
}
impl fmt::Debug for BroadLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BroadLine::ArrayOfTable(m) => write!(f, "{}", format!("{:?}", m)),
            BroadLine::Comment(m) => write!(f, "{}", format!("{:?}", m)),
            BroadLine::EmptyLine => write!(f, ""),
            BroadLine::KeyValue(m) => write!(f, "{}", format!("{:?}", m)),
            BroadLine::Table(m) => write!(f, "{}", format!("{:?}", m)),
        }
    }
}
