//! Document model.  
//! ドキュメント・モデル。  

use crate::model::layer210::LiteralValue;
use crate::model::layer225::RightValue;
use crate::model::layer230::DocumentElement::KeyValue;
use crate::model::{layer230::DocumentElement, layer310::Document};
use chrono::prelude::{DateTime, Utc};
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
                    if m.key.to_string() == key.to_string() {
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
    pub fn get_literal_string_by_key(&self, key: &str) -> Option<&LiteralValue> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let KeyValue(key_value) = doc_elm {
                if key_value.key.to_string() == key.to_string() {
                    if let RightValue::LiteralValue(literal_value) = &*key_value.value {
                        return Some(literal_value);
                    }
                }
            }
        }
        None
    }

    /// Right integer of `left = 123`.  
    /// キー・バリューの右の整数値。  
    pub fn get_i128_by_key(&self, key: &str) -> Option<i128> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let KeyValue(key_value) = doc_elm {
                if key_value.key.to_string() == key.to_string() {
                    if let RightValue::LiteralValue(literal_value) = &*key_value.value {
                        match literal_value.to_string().parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// WIP. まだ `.` をパースできていません。  
    ///
    /// Right integer of `left = 1.2`.  
    /// キー・バリューの右の整数値。  
    pub fn get_f64_by_key(&self, key: &str) -> Option<f64> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let KeyValue(key_value) = doc_elm {
                if key_value.key.to_string() == key.to_string() {
                    if let RightValue::LiteralValue(literal_value) = &*key_value.value {
                        match literal_value.to_string().parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    #[deprecated(
        since = "0.1.10",
        note = "Please use the tomboy_toml_dom::model::layer310::document::get_string_by_key() method instead"
    )]
    pub fn get_str_by_key(&self, _key: &str) -> Option<&str> {
        panic!("Obsoleted. Please use the tomboy_toml_dom::model::layer310::document::get_string_by_key() method instead.")
    }

    /// Right string of `left = "abc"`.  
    /// キー・バリューの右の文字列。  
    pub fn get_string_by_key(&self, key: &str) -> Option<String> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let KeyValue(key_value) = doc_elm {
                if key_value.key.to_string() == key.to_string() {
                    match &*key_value.value {
                        RightValue::BasicString(basic_string) => {
                            return Some(basic_string.to_string());
                        }
                        RightValue::LiteralString(literal_string) => {
                            return Some(literal_string.to_string());
                        }
                        _ => {}
                    }
                }
            }
        }
        None
    }
    /// For this library developer.
    pub fn get_debug_string_by_key(&self, key: &str) -> String {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let KeyValue(key_value) = doc_elm {
                if key_value.key.to_string() == key.to_string() {
                    match &*key_value.value {
                        RightValue::BasicString(basic_string) => {
                            return basic_string.to_debug_string();
                        }
                        RightValue::LiteralString(literal_string) => {
                            return literal_string.to_debug_string();
                        }
                        _ => {}
                    }
                }
            }
        }
        "".to_string()
    }

    /// Right boolean of `left = true`.  
    /// キー・バリューの右の論理値。  
    pub fn get_bool_by_key(&self, key: &str) -> Option<bool> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let KeyValue(key_value) = doc_elm {
                if key_value.key.to_string() == key.to_string() {
                    if let RightValue::LiteralValue(literal_value) = &*key_value.value {
                        match literal_value.to_string().parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// DateTime. UTC.  
    /// 日付と時刻。協定世界時。  
    pub fn get_datetime_utc_by_key(&self, key: &str) -> Option<DateTime<Utc>> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let KeyValue(key_value) = doc_elm {
                if key_value.key.to_string() == key.to_string() {
                    if let RightValue::LiteralValue(literal_value) = &*key_value.value {
                        match format!("{}", literal_value).to_string().parse() {
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
    pub fn to_debug_string(&self) -> String {
        format!("{:?}", self)
    }
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}
impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for elem in &self.elements {
            buf.push_str(&format!("{}", elem));
        }
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for elem in &self.elements {
            buf.push_str(&format!("{:?}", elem));
        }
        write!(f, "{}", buf)
    }
}
