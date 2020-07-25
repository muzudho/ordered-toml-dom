use crate::object_model::value::Value;
use std::fmt;

#[derive(Clone)]
pub struct KeyValueModel {
    key: String,
    value: Option<Box<Value>>,
}
impl Default for KeyValueModel {
    fn default() -> Self {
        KeyValueModel {
            key: String::new(),
            value: None,
        }
    }
}
impl KeyValueModel {
    pub fn new(key: &str) -> Self {
        KeyValueModel {
            key: key.to_string(),
            value: None,
        }
    }
}
impl fmt::Debug for KeyValueModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}={}",
            self.key,
            if let Some(v) = &self.value {
                format!("{:?}", v).to_string()
            } else {
                "".to_string()
            }
        )
    }
}
