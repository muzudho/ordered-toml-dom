//! Key value model.  
//! キー値モデル。  

use crate::model::{KeyValueM, ValueM};
use crate::token::Token;
use std::fmt;

impl KeyValueM {
    pub fn new(token: &Token, m: &ValueM) -> Self {
        KeyValueM {
            key: token.value.to_string(),
            value: Box::new(m.clone()),
        }
    }
}
impl fmt::Debug for KeyValueM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}={:?}", self.key, self.value)
    }
}
