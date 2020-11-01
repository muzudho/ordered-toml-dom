//! Non end-of-line model.  
//! 非行末モデル。  

use crate::model::layer210::Character;
use crate::model::layer210::NonEol;
use std::fmt;

impl NonEol {
    pub fn new(character: &Character) -> Self {
        NonEol {
            character: character.clone(),
        }
    }
    pub fn get_character(&self) -> Character {
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
