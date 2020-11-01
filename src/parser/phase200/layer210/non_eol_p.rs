//! Non end-of-line parser.  
//! 非行末パーサー。  

use crate::model::{
    layer110::{Character, TokenType},
    layer210::NonEol,
};
use crate::parser::phase200::error_via;
use crate::parser::phase200::layer210::{non_ascii_p::Judge as NonAsciiPJudge, NonAsciiP};
use crate::parser::phase200::layer210::{NonEolP, PResult};
use crate::parser::phase200::LookAheadCharacters;
use crate::parser::phase200::Token;
use casual_logger::Table;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug, Clone)]
pub enum State {
    End,
    First,
    NonAscii,
}

pub enum Judge {
    NonAscii,
    HorizontalTabAndAscii,
}

impl Default for NonEolP {
    fn default() -> Self {
        NonEolP {
            buffer: None,
            state: State::First,
            non_ascii_p: None,
        }
    }
}
impl NonEolP {
    pub fn flush(&mut self) -> Option<NonEol> {
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
    pub fn judge(character: &Character) -> Option<Judge> {
        if let Some(_judge) = NonAsciiP::judge(character) {
            return Some(Judge::NonAscii);
        }
        let unicode = character.to_string_chars_nth(0).unwrap() as u32;
        match unicode {
            0x09 | 0x20..=0x7F => Some(Judge::HorizontalTabAndAscii),
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
                // Horizon tab and Ascii code.
                if let None = self.buffer {
                    self.buffer = Some(NonEol::default());
                }
                let m = self.buffer.as_mut().unwrap();
                let character0 = characters.current.as_ref().unwrap();
                m.push_token(&Token::from_character(character0, TokenType::NonEol));

                // TODO 次の文字をチェックすべきか、次のトークンをチェックすべきか？
                let character1 = characters.current.as_ref().unwrap();
                if let None = Self::judge(&character1) {
                    return PResult::End;
                }
            }
            State::NonAscii => {
                return self.parse_non_ascii(characters);
            }
        }
        PResult::Ongoing
    }

    fn parse_non_ascii(&mut self, characters: &LookAheadCharacters) -> PResult {
        if let None = self.non_ascii_p {
            self.non_ascii_p = Some(NonAsciiP::new());
        }
        let p = self.non_ascii_p.as_mut().unwrap();
        match p.parse(characters) {
            PResult::End => {
                if let None = self.buffer {
                    self.buffer = Some(NonEol::default());
                }
                let m = self.buffer.as_mut().unwrap();
                m.extend_tokens(&p.flush().unwrap().tokens);
                self.non_ascii_p = None;
                self.state = State::End;
                return PResult::End;
            }
            PResult::Err(mut table) => {
                return error_via(
                    &mut table,
                    &mut self.log(),
                    &characters,
                    "literal_value_p.rs.90.",
                );
            }
            PResult::Ongoing => {
                return PResult::Ongoing;
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
