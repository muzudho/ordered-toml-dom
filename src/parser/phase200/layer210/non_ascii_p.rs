//! Non ascii parser.  
//! 非ASCIIパーサー。  

use crate::model::{layer110::TokenType, layer210::NonAscii};
use crate::parser::phase200::error;
use crate::parser::phase200::layer210::{NonAsciiP, PResult};
use crate::parser::phase200::LookAheadTokens;
use crate::parser::phase200::Token;
use casual_logger::Table;

impl NonAsciiP {
    pub fn new() -> Self {
        NonAsciiP { buffer: None }
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
    pub fn check_non_ascii(token: &Token) -> bool {
        let unicode = token.to_string_chars_nth(0).unwrap() as u32;
        match unicode {
            // non-ascii
            0x80..=0xD7FF | 0xE000..=0x10FFFF => true,
            _ => false,
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
    pub fn parse(&mut self, tokens: &LookAheadTokens) -> PResult {
        let token0 = tokens.current.as_ref().unwrap();
        if Self::check_non_ascii(token0) {
            if let None = self.buffer {
                self.buffer = Some(NonAscii::default());
            }
            let m = self.buffer.as_mut().unwrap();
            m.push_token(&Token::from_base(token0, TokenType::NonAscii));

            let token1 = tokens.current.as_ref().unwrap();
            if !Self::check_non_ascii(&token1) {
                return PResult::End;
            }
        } else {
            return PResult::End;
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
