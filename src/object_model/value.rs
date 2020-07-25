use crate::object_model::key_value::KeyValueModel;
use std::fmt;

#[derive(Clone)]
pub enum Value {
    String(String),
    KeyValue(KeyValueModel),
}
impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::KeyValue(m) => write!(f, "{:?}", m),
        }
    }
}
