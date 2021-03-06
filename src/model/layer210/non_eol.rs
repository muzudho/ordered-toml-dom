//! Non end-of-line model.  
//! 非行末モデル。  

use crate::model::layer210::NonEol;
use std::fmt;

impl NonEol {
    pub fn new(chr: char) -> Self {
        NonEol { character: chr }
    }
    pub fn get_character(&self) -> char {
        self.character.clone()
    }
    pub fn to_debug_string(&self) -> String {
        format!("{:?}", self.character)
    }
    pub fn to_string(&self) -> String {
        format!("{}", self.character)
    }
}
impl fmt::Display for NonEol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.character)
    }
}
impl fmt::Debug for NonEol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.character)
    }
}
