use std::fmt;

pub struct KeyValueModel {
    key: String,
    value: String,
}
impl Default for KeyValueModel {
    fn default() -> Self {
        KeyValueModel {
            key: String::new(),
            value: String::new(),
        }
    }
}
impl KeyValueModel {
    pub fn new(key: &str) -> Self {
        KeyValueModel {
            key: key.to_string(),
            value: String::new(),
        }
    }
}
impl fmt::Debug for KeyValueModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}
