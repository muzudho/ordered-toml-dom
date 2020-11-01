//! Non ascii model.  
//! 非アスキー・モデル。  

use crate::model::layer110::Character;
use crate::model::layer210::NonAscii;
use std::fmt;

impl NonAscii {
    pub fn new(character: &Character) -> Self {
        NonAscii {
            character: character.clone(),
        }
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
