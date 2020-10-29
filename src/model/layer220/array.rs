//! Array model.  
//! 配列モデル。  
//!
//! # Examples
//!
//! ```
//! // [ 1, 2, 3 ]
//! ```
use num_traits::Num;

use crate::model::{
    layer210::{BasicString, LiteralString, LiteralValue},
    layer220::{Array, ItemValue},
};
use std::fmt;

impl Default for Array {
    fn default() -> Self {
        Array { items: Vec::new() }
    }
}
impl Array {
    pub fn push_literal_string(&mut self, m: &LiteralValue) {
        self.items.push(ItemValue::LiteralValue(m.clone()));
    }
    pub fn push_single_quote_string(&mut self, m: &LiteralString) {
        self.items.push(ItemValue::LiteralString(m.clone()));
    }
    pub fn push_double_quote_string(&mut self, m: &BasicString) {
        self.items.push(ItemValue::BasicString(m.clone()));
    }
    pub fn push_array(&mut self, m: &Array) {
        self.items.push(ItemValue::Array(m.clone()));
    }
    pub fn to_debug_string(&self) -> String {
        format!("{:?}", self)
    }
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
    pub fn to_string_vector(&self) -> Vec<String> {
        let mut vec = Vec::<String>::new();
        for item in &self.items {
            vec.push(item.to_string());
        }
        vec
    }
    pub fn to_int_vector<T: Num + std::str::FromStr>(&self) -> Result<Vec<T>, String>
    where
        <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        let mut vec = Vec::<T>::new();
        for item in &self.items {
            let num = match item.to_string().parse() {
                Ok(n) => n,
                Err(why) => return Err(format!("{}", why)),
            };
            vec.push(num);
        }
        Ok(vec)
    }
}
impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for item in &self.items {
            buf.push_str(&format!("{},", item))
        }
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for item in &self.items {
            buf.push_str(&format!("{:?},", item))
        }
        write!(f, "[ {} ]", buf)
    }
}
