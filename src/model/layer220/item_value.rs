//! Item value model.  
//! アイテム値モデル。  

use crate::model::layer220::ItemValue;
use std::fmt;

impl fmt::Debug for ItemValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ItemValue::Array(m) => write!(f, "{:?}", m),
            ItemValue::DoubleQuotedString(m) => write!(f, "{:?}", m),
            ItemValue::InlineTable(m) => write!(f, "{:?}", m),
            ItemValue::KeyValue(m) => write!(f, "{:?}", m),
            ItemValue::LiteralValue(m) => write!(f, "{:?}", m),
            ItemValue::SingleQuotedString(m) => write!(f, "{:?}", m),
        }
    }
}
