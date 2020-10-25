//! Right value model.  
//! 右値モデル。  

use crate::model::layer225::Val;
use std::fmt;

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Val::Array(m) => write!(f, "{}", m),
            Val::BasicString(m) => write!(f, "{}", m),
            Val::InlineTable(m) => write!(f, "{}", m),
            // No Keyval.
            Val::LiteralValue(m) => write!(f, "{}", m),
            Val::LiteralString(m) => write!(f, "{}", m),
        }
    }
}
impl fmt::Debug for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Val::Array(m) => write!(f, "{:?}", m),
            Val::BasicString(m) => write!(f, "{:?}", m),
            Val::InlineTable(m) => write!(f, "{:?}", m),
            // No Keyval.
            Val::LiteralValue(m) => write!(f, "{:?}", m),
            Val::LiteralString(m) => write!(f, "{:?}", m),
        }
    }
}
