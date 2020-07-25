//! Key value model.  
//! キー値モデル。  

use crate::object_model::value::ValueM;
use std::fmt;

#[derive(Clone)]
pub struct KeyValueM {
    key: String,
    pub value: Option<Box<ValueM>>,
}
impl Default for KeyValueM {
    fn default() -> Self {
        KeyValueM {
            key: String::new(),
            value: None,
        }
    }
}
impl KeyValueM {
    pub fn new(key: &str) -> Self {
        KeyValueM {
            key: key.to_string(),
            value: None,
        }
    }
}
impl fmt::Debug for KeyValueM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}=|{}|",
            self.key,
            if let Some(v) = &self.value {
                format!("{:?}", v).to_string()
            } else {
                "".to_string()
            }
        )
    }
}
