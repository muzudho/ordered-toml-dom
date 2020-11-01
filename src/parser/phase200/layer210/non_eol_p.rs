//! Non end-of-line parser.  
//! 非行末パーサー。  

use crate::model::{layer110::Character, layer210::NonEol};
use crate::parser::phase200::layer210::NonAsciiP;
use crate::parser::phase200::layer210::NonEolP;

pub enum Judge {
    HorizontalTab(Character),
    Ascii(Character),
    NonAscii(Character),
}

impl NonEolP {
    /// # Arguments
    ///
    /// * `token` - Token.  
    ///             トークン。  
    /// # Returns
    ///
    /// * `bool` - このパーサーの対象とするトークンになる.  
    ///                             結果。
    pub fn judge(character: &Character) -> Option<Judge> {
        if let Some(_judge) = NonAsciiP::judge(character) {
            return Some(Judge::NonAscii(character.clone()));
        }
        let unicode = character.to_char() as u32;
        match unicode {
            0x09 => Some(Judge::HorizontalTab(character.clone())),
            0x20..=0x7F => Some(Judge::Ascii(character.clone())),
            _ => None,
        }
    }
    /// # Arguments
    ///
    /// * `characters` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn product(judge: &Judge) -> NonEol {
        match judge {
            Judge::HorizontalTab(horizontalTab) => {
                return NonEol::new(horizontalTab);
            }
            Judge::Ascii(ascii) => {
                return NonEol::new(ascii);
            }
            Judge::NonAscii(non_ascii) => {
                return NonEol::new(non_ascii);
            }
        }
    }
}
