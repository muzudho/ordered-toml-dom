//! Non ascii parser.  
//! 非ASCIIパーサー。  

use crate::model::{
    layer110::{Character, TokenType},
    layer210::NonAscii,
};
use crate::parser::phase200::error;
use crate::parser::phase200::layer210::{NonAsciiP, PResult};
use crate::parser::phase200::LookAheadCharacters;
use crate::parser::phase200::Token;
use casual_logger::Table;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug, Clone)]
pub enum State {
    End,
    First,
}
#[derive(Debug, Clone)]
pub enum Judge {
    NonAscii,
}

impl NonAsciiP {
    pub fn new() -> Self {
        NonAsciiP {
            buffer: None,
            state: State::First,
        }
    }
    pub fn flush(&mut self) -> Option<NonAscii> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
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
            0x80..=0xD7FF | 0xE000..=0x10FFFF => Some(Judge::NonAscii),
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
    pub fn parse(&mut self, characters: &LookAheadCharacters) -> PResult {
        match self.state {
            State::End => {
                return PResult::End;
            }
            State::First => {
                if let None = self.buffer {}
                let m = self.buffer.as_mut().unwrap();
                let character0 = characters.current.as_ref().unwrap();
                if let Some(judge) = Self::judge(&character0) {
                    match judge {
                        Judge::NonAscii => {
                            self.buffer = Some(NonAscii::new(character0));
                            // Forward.
                            return PResult::End;
                        }
                    }
                } else {
                    return error(&mut self.log(), &characters, "non_eol_p.rs.90.");
                }
            }
        }
    }

    /// Log.
    /// ログ。
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();
        if let Some(m) = &self.buffer {
            t.str("buffer", &m.to_string());
        }
        t
    }
}
