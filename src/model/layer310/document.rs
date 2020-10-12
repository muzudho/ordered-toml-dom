//! Document model.  
//! ドキュメント・モデル。  

use crate::model::layer210::LiteralString;
use crate::model::layer220::RightValue;
use crate::model::layer230::DocumentElement::KeyValue;
use crate::model::{layer230::DocumentElement, layer310::Document};
use std::fmt;

impl Default for Document {
    fn default() -> Self {
        Document {
            elements: Vec::new(),
        }
    }
}
impl Document {
    /// Right of `left = right`.  
    /// キー・バリューの右値。  
    pub fn get_right_value_by_key(&self, key: &str) -> Option<&DocumentElement> {
        for elem in &self.elements {
            match elem {
                DocumentElement::HeaderOfArrayOfTable(_) => {
                    // TODO
                }
                DocumentElement::Comment(_) => {}
                DocumentElement::EmptyLine => {}
                DocumentElement::KeyValue(m) => {
                    // println!("m.key={}", m.key); // In development.
                    if m.key == key {
                        // println!("HIT m.key={}", m.key);// In development.
                        return Some(elem);
                    }
                }
                DocumentElement::HeaderOfTable(_) => {
                    // TODO
                }
            }
        }
        None
    }

    /// Right of `left = right`.  
    /// キー・バリューの右値。  
    pub fn get_literal_string_by_key(&self, key: &str) -> Option<&LiteralString> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let KeyValue(key_value) = doc_elm {
                if key_value.key == key {
                    if let RightValue::LiteralString(literal_string) = &*key_value.value {
                        return Some(literal_string);
                    }
                }
            }
        }
        None
    }

    /// Right integer of `left = 123`.  
    /// キー・バリューの右の整数値。  
    pub fn get_int128_by_key(&self, key: &str) -> Option<i128> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let KeyValue(key_value) = doc_elm {
                if key_value.key == key {
                    if let RightValue::LiteralString(literal_string) = &*key_value.value {
                        match literal_string.value.parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// Right string of `left = "abc"`.  
    /// キー・バリューの右の文字列。  
    pub fn get_str_by_key(&self, key: &str) -> Option<&str> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let KeyValue(key_value) = doc_elm {
                if key_value.key == key {
                    match &*key_value.value {
                        RightValue::DoubleQuotedString(literal_string) => {
                            return Some(&literal_string.value);
                        }
                        RightValue::SingleQuotedString(literal_string) => {
                            return Some(&literal_string.value);
                        }
                        _ => {}
                    }
                }
            }
        }
        None
    }

    /// Right boolean of `left = true`.  
    /// キー・バリューの右の論理値。  
    pub fn get_bool_by_key(&self, key: &str) -> Option<bool> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let KeyValue(key_value) = doc_elm {
                if key_value.key == key {
                    if let RightValue::LiteralString(literal_string) = &*key_value.value {
                        match literal_string.value.parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    pub fn push_element(&mut self, m: &DocumentElement) {
        self.elements.push(m.clone());
    }
}
impl fmt::Debug for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for elem in &self.elements {
            buf.push_str(&format!(
                "{:?}
",
                elem
            ));
        }
        write!(f, "{}", buf)
    }
}
