//! Key value model.  
//! キー値モデル。  

use crate::model::{
    layer110::token::Token,
    layer225::{KeyValue, RightValue},
};
use std::fmt;

impl KeyValue {
    pub fn new(token: &Token, value: &RightValue) -> Self {
        KeyValue {
            key: token.value.to_string(),
            value: Box::new(value.clone()),
        }
    }
}
impl fmt::Debug for KeyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}={:?}", self.key, self.value)
    }
}
