//! Non end-of-line parser.  
//! 非行末パーサー。  

use crate::model::{
    layer110::{Character, TokenType},
    layer210::Ws,
};
use crate::parser::phase200::error_via;
use crate::parser::phase200::layer210::WscharP;
use crate::parser::phase200::layer210::{PResult, WsP};
use crate::parser::phase200::LookAheadCharacters;
use crate::parser::phase200::Token;
use casual_logger::Table;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug, Clone)]
pub enum State {
    End,
    First,
    Wschar,
}

pub enum Judge {
    Wschar,
    HorizontalTabAndAscii,
}

impl Default for WsP {
    fn default() -> Self {
        WsP {
            ws: Ws::default(),
            state: State::First,
            wschar_p: None,
        }
    }
}
impl WsP {
    pub fn get_ws(&mut self) -> Ws {
        self.ws.clone()
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
        if let Some(_judge) = WscharP::judge(character) {
            return Some(Judge::Wschar);
        }
        let unicode = character.to_char() as u32;
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
                let character0 = characters.current.as_ref().unwrap();
                self.ws
                    .push_token(&Token::from_character(character0, TokenType::Ws));

                // TODO 次の文字をチェックすべきか、次のトークンをチェックすべきか？
                let character1 = characters.current.as_ref().unwrap();
                if let None = Self::judge(&character1) {
                    return PResult::End;
                }
            }
            State::Wschar => {
                return self.parse_wschar(characters);
            }
        }
        PResult::Ongoing
    }

    fn parse_wschar(&mut self, characters: &LookAheadCharacters) -> PResult {
        if let None = self.wschar_p {
            self.wschar_p = Some(WscharP::new());
        }
        let p = self.wschar_p.as_mut().unwrap();
        match p.parse(characters) {
            PResult::End => {
                self.ws.extend_tokens(&p.flush().unwrap().tokens);
                self.wschar_p = None;
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
        t.str("ws", &self.ws.to_string());
        t
    }
}
