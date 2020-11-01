//! Character.  
//! 字。  

use crate::model::layer110::{Character, CharacterType};
use std::fmt;

impl Character {
    pub fn new(column_number: usize, value: &str, type_: CharacterType) -> Self {
        Character {
            column_number: column_number,
            value: value.to_string(),
            type_: type_,
        }
    }
    pub fn from_base(token: &Character, type_: CharacterType) -> Self {
        Character {
            column_number: token.column_number,
            value: token.value.to_string(),
            type_: type_,
        }
    }
    pub fn to_string_chars_nth(&self, nth: usize) -> Option<char> {
        self.to_string().chars().nth(nth)
    }
    pub fn to_debug_string(&self) -> String {
        format!("{:?}", self)
    }
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}
impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl fmt::Debug for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{} {} {:?}]",
            self.column_number, self.value, self.type_
        )
    }
}

pub fn tokens_stringify(tokens: &Vec<Character>) -> String {
    let mut buf = String::new();
    for token in tokens {
        buf.push_str(&token.to_string());
    }
    buf
}
pub fn tokens_stringify_debug(tokens: &Vec<Character>) -> String {
    let mut buf = String::new();
    for token in tokens {
        buf.push_str(&token.to_debug_string());
    }
    buf
}
