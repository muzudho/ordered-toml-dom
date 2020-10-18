//! Key value model.  
//! キー値モデル。  

use crate::model::{
    layer210::Key,
    layer225::{KeyValue, RightValue},
};
use std::fmt;

impl KeyValue {
    pub fn new(key: &Key, value: &RightValue) -> Self {
        KeyValue {
            key: Box::new(key.clone()),
            value: Box::new(value.clone()),
        }
    }
    pub fn to_debug_string(&self) -> String {
        format!("{}", self)
    }
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}
impl fmt::Display for KeyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}
impl fmt::Debug for KeyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}={:?}", self.key, self.value)
    }
}
