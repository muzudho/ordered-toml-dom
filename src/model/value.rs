//! Value model.  
//! 値モデル。  

use crate::model::Value;
use std::fmt;

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Array(m) => write!(f, "{:?}", m),
            Value::InlineTable(m) => write!(f, "{:?}", m),
            Value::KeyValue(m) => write!(f, "{:?}", m),
            Value::LiteralString(m) => write!(f, "{:?}", m),
            Value::SingleQuotedString(m) => write!(f, "{:?}", m),
        }
    }
}
