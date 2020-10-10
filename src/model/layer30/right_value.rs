//! Right value model.  
//! 右値モデル。  

use crate::model::layer30::RightValue;
use std::fmt;

impl fmt::Debug for RightValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RightValue::Array(m) => write!(f, "{:?}", m),
            RightValue::DoubleQuotedString(m) => write!(f, "{:?}", m),
            RightValue::InlineTable(m) => write!(f, "{:?}", m),
            // No KeyValue.
            RightValue::LiteralString(m) => write!(f, "{:?}", m),
            RightValue::SingleQuotedString(m) => write!(f, "{:?}", m),
        }
    }
}
