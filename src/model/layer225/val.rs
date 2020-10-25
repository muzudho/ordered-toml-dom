//! Right value model.  
//! 右値モデル。  

use crate::model::layer225::RightValue;
use std::fmt;

impl fmt::Display for RightValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RightValue::Array(m) => write!(f, "{}", m),
            RightValue::BasicString(m) => write!(f, "{}", m),
            RightValue::InlineTable(m) => write!(f, "{}", m),
            // No Keyval.
            RightValue::LiteralValue(m) => write!(f, "{}", m),
            RightValue::LiteralString(m) => write!(f, "{}", m),
        }
    }
}
impl fmt::Debug for RightValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RightValue::Array(m) => write!(f, "{:?}", m),
            RightValue::BasicString(m) => write!(f, "{:?}", m),
            RightValue::InlineTable(m) => write!(f, "{:?}", m),
            // No Keyval.
            RightValue::LiteralValue(m) => write!(f, "{:?}", m),
            RightValue::LiteralString(m) => write!(f, "{:?}", m),
        }
    }
}
