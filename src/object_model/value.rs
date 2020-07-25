//! Value model.  
//! 値モデル。  

use crate::object_model::single_quoted_string::SingleQuotedStringM;
use crate::object_model::{array::ArrayM, inline_table::InlineTableM, key_value::KeyValueM};
use std::fmt;

#[derive(Clone)]
pub enum ValueM {
    Array(ArrayM),
    InlineTable(InlineTableM),
    KeyValue(KeyValueM),
    SingleQuotedString(SingleQuotedStringM),
    String(String),
}
impl fmt::Debug for ValueM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueM::Array(m) => write!(f, "{:?}", m),
            ValueM::InlineTable(m) => write!(f, "{:?}", m),
            ValueM::KeyValue(m) => write!(f, "{:?}", m),
            ValueM::SingleQuotedString(m) => write!(f, "{:?}", m),
            ValueM::String(s) => write!(f, "{}", s),
        }
    }
}
