//! Non ascii parser.  
//! 非ASCIIパーサー。  

use crate::model::{layer110::Character, layer210::NonAscii};
use crate::parser::phase200::layer210::NonAsciiP;
use casual_logger::Table;

#[derive(Debug, Clone)]
pub enum Judge {
    NonAscii(Character),
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
            0x80..=0xD7FF | 0xE000..=0x10FFFF => Some(Judge::NonAscii(character.clone())),
            // 0x80..=0xD7FF | 0xE000..=u32::MAX => Judge::NonAscii,
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
    pub fn product(&mut self, judge: &Judge) -> NonAscii {
        match judge {
            Judge::NonAscii(character) => {
                return NonAscii::new(character);
            }
        }
    }

    /// Log.
    /// ログ。
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();
        t
    }
}
