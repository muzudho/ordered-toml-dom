//! Value model.  
//! 値モデル。  

use crate::object_model::{inline_table::InlineTableM, key_value::KeyValueM};
use std::fmt;

#[derive(Clone)]
pub enum ValueM {
    String(String),
    KeyValue(KeyValueM),
    InlineTable(InlineTableM),
}
impl fmt::Debug for ValueM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueM::String(s) => write!(f, "{}", s),
            ValueM::KeyValue(m) => write!(f, "{:?}", m),
            ValueM::InlineTable(m) => write!(f, "{:?}", m),
        }
    }
}
