//! Value model.  
//! 値モデル。  

use crate::model::literal_string::LiteralStringM;
use crate::model::single_quoted_string::SingleQuotedStringM;
use crate::model::{array::ArrayM, inline_table::InlineTableM, key_value::KeyValueM};
use std::fmt;

#[derive(Clone)]
pub enum ValueM {
    Array(ArrayM),
    InlineTable(InlineTableM),
    KeyValue(KeyValueM),
    LiteralString(LiteralStringM),
    SingleQuotedString(SingleQuotedStringM),
    String(String),
}
impl fmt::Debug for ValueM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueM::Array(m) => write!(f, "{:?}", m),
            ValueM::InlineTable(m) => write!(f, "{:?}", m),
            ValueM::KeyValue(m) => write!(f, "{:?}", m),
            ValueM::LiteralString(m) => write!(f, "{:?}", m),
            ValueM::SingleQuotedString(m) => write!(f, "{:?}", m),
            ValueM::String(s) => write!(f, "{}", s),
        }
    }
}
