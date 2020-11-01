//! Non end-of-line parser.  
//! 非行末パーサー。  

use crate::model::{layer110::Character, layer210::NonEol};
use crate::parser::phase200::layer210::NonAsciiP;
use crate::parser::phase200::layer210::NonEolP;

pub enum Judge {
    HorizontalTab(NonEol),
    Ascii(NonEol),
    NonAscii(NonEol),
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
            return Some(Judge::NonAscii(NonEol::new(character)));
        }
        let unicode = character.to_char() as u32;
        match unicode {
            0x09 => Some(Judge::HorizontalTab(NonEol::new(character))),
            0x20..=0x7F => Some(Judge::Ascii(NonEol::new(character))),
            _ => None,
        }
    }
}
