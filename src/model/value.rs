//! Value model.  
//! 値モデル。  

use crate::model::ValueM;
use std::fmt;

impl fmt::Debug for ValueM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueM::Array(m) => write!(f, "{:?}", m),
            ValueM::InlineTable(m) => write!(f, "{:?}", m),
            ValueM::KeyValue(m) => write!(f, "{:?}", m),
            ValueM::LiteralString(m) => write!(f, "{:?}", m),
            ValueM::SingleQuotedString(m) => write!(f, "{:?}", m),
        }
    }
}
