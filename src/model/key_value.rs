//! Key value model.  
//! キー値モデル。  

use crate::model::{KeyValue, Value};
use crate::token::Token;
use std::fmt;

impl KeyValue {
    pub fn new(token: &Token, m: &Value) -> Self {
        KeyValue {
            key: token.value.to_string(),
            value: Box::new(m.clone()),
        }
    }
}
impl fmt::Debug for KeyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}={:?}", self.key, self.value)
    }
}
