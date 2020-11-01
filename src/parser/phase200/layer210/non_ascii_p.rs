//! Non ascii parser.  
//! 非ASCIIパーサー。  

use crate::model::{layer110::TokenType, layer210::NonAscii};
use crate::parser::phase200::layer210::{NonAsciiP, PResult};
use crate::parser::phase200::LookAheadCharacters;
use crate::parser::phase200::Token;

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
    /// * `token` - Token.  
    ///             トークン。  
    /// # Returns
    ///
    /// * `bool` - このパーサーの対象とするトークンになる.  
    ///                             結果。
    pub fn judge(token: &Token) -> Option<Judge> {
        let unicode = token.to_string_chars_nth(0).unwrap() as u32;
        match unicode {
            // non-ascii
            0x80..=0xD7FF | 0xE000..=0x10FFFF => Some(Judge::NonAscii),
            // 0x80..=0xD7FF | 0xE000..=u32::MAX => Judge::NonAscii,
            _ => None,
        }
    }
    /// # Arguments
    ///
    /// * `tokens` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, tokens: &LookAheadCharacters) -> PResult {
        match self.state {
            State::End => {
                return PResult::End;
            }
            State::First => {
                if let None = self.buffer {
                    self.buffer = Some(NonAscii::default());
                }
                let m = self.buffer.as_mut().unwrap();
                let token0 = tokens.current.as_ref().unwrap();
                m.push_token(&Token::from_base(token0, TokenType::NonAscii));
                let token1 = tokens.current.as_ref().unwrap();
                if let None = Self::judge(&token1) {
                    return PResult::End;
                }
            }
        }
        PResult::Ongoing
    }
    /*
    /// Log.
    /// ログ。
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();
        if let Some(m) = &self.buffer {
            t.str("buffer", &m.to_string());
        }
        t
    }
    */
}
