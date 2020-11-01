//! Non ascii parser.  
//! 非ASCIIパーサー。  

use crate::model::{layer110::Character, layer210::NonAscii};
use crate::parser::phase200::layer210::NonAsciiP;

#[derive(Debug, Clone)]
pub enum Judge {
    NonAscii(NonAscii),
}

impl NonAsciiP {
    /// # Arguments
    ///
    /// * `character` - Token.  
    ///             トークン。  
    /// # Returns
    ///
    /// * `bool` - このパーサーの対象とするトークンになる.  
    ///                             結果。
    pub fn judge(character: &Character) -> Option<Judge> {
        let unicode = character.to_char() as u32;
        match unicode {
            // non-ascii
            0x80..=0xD7FF | 0xE000..=0x10FFFF => Some(Judge::NonAscii(NonAscii::new(character))),
            // 0x80..=0xD7FF | 0xE000..=u32::MAX => Judge::NonAscii,
            _ => None,
        }
    }
}
