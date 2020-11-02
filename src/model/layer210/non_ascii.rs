//! Non ascii model.  
//! 非アスキー・モデル。  

use crate::model::layer210::NonAscii;
use std::fmt;

impl NonAscii {
    pub fn new(chr: char) -> Self {
        NonAscii { character: chr }
    }
    pub fn get_character(&self) -> char {
        self.character.clone()
    }
    pub fn to_debug_string(&self) -> String {
        format!("{:?}", self)
    }
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}
impl fmt::Display for NonAscii {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.character)
    }
}
impl fmt::Debug for NonAscii {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.character)
    }
}
