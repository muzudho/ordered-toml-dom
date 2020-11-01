//! Whitespace character parser.  
//! 空白文字パーサー。  

use crate::model::{
    layer110::{Character, TokenType},
    layer210::Wschar,
};
use crate::parser::phase200::layer210::{PResult, WscharP};
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
    Wschar,
}

impl WscharP {
    pub fn new() -> Self {
        WscharP {
            buffer: None,
            state: State::First,
        }
    }
    pub fn flush(&mut self) -> Option<Wschar> {
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
            // Space, Horizon tab.
            0x20 | 0x09 => Some(Judge::Wschar),
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
                    self.buffer = Some(Wschar::default());
                }
                let m = self.buffer.as_mut().unwrap();
                let character0 = tokens.current.as_ref().unwrap();
                m.push_token(&Token::from_character(character0, TokenType::Wschar));
                let character1 = tokens.current.as_ref().unwrap();
                if let None = Self::judge(&character1) {
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
