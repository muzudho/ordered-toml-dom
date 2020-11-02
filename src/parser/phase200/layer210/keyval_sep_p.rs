//! Non end-of-line parser.  
//! 非行末パーサー。  

use crate::model::{layer110::TokenType, layer210::Ws};
use crate::parser::phase200::error;
use crate::parser::phase200::error_via;
use crate::parser::phase200::layer210::WsP;
use crate::parser::phase200::layer210::WscharP;
use crate::parser::phase200::layer210::{KeyvalSepP, PResult};
use crate::parser::phase200::Token;
use casual_logger::Table;
use look_ahead_items::LookAheadItems;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug, Clone)]
pub enum State {
    End,
    First,
    Ws1,
    Equals,
    Ws2,
}

pub enum Judge {
    Ws,
    Equals,
}

impl Default for KeyvalSepP {
    fn default() -> Self {
        KeyvalSepP {
            state: State::First,
            ws1: Ws::default(),
            ws2: Ws::default(),
        }
    }
}
impl KeyvalSepP {
    pub fn is_end(&mut self) -> bool {
        match self.state {
            State::End => true,
            _ => false,
        }
    }
    /// # Arguments
    ///
    /// * `token` - Token.  
    ///             トークン。  
    /// # Returns
    ///
    /// * `bool` - このパーサーの対象とするトークンになる.  
    ///                             結果。
    pub fn judge(chr: char) -> Option<Judge> {
        if let Some(_judge) = WsP::judge(chr) {
            return Some(Judge::Ws);
        }
        let unicode = chr as u32;
        match unicode {
            0x3D => Some(Judge::Equals),
            _ => None,
        }
    }
    /// # Arguments
    ///
    /// * `look_ahead_items` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, look_ahead_items: &LookAheadItems<char>) -> PResult {
        match self.state {
            State::First => {
                let chr0 = look_ahead_items.get(0).unwrap();
                if let Some(judge) = Self::judge(*chr0) {
                    match judge {
                        Judge::Ws => {
                            return self.parse_ws1(look_ahead_items);
                        }
                        Judge::Equals => {
                            self.state = State::Ws2;
                        }
                    }
                } else {
                    return error(&mut self.log(), &look_ahead_items, "keyval_sep_p.rs.108.");
                }
            }
            State::Ws1 => {
                return self.parse_ws1(look_ahead_items);
            }
            State::Equals => {
                return self.parse_equals(look_ahead_items);
            }
            State::Ws2 => {
                return self.parse_ws2(look_ahead_items);
            }
            State::End => {
                return PResult::End;
            }
        }
        PResult::Ongoing
    }

    fn parse_ws1(&mut self, look_ahead_items: &LookAheadItems<char>) -> PResult {
        let chr0 = look_ahead_items.get(0).unwrap();
        self.ws1
            .push_token(&Token::from_character(*chr0, TokenType::Ws));

        // 次のトークンを調べます。
        let chr1 = look_ahead_items.get(1).unwrap();
        if let Some(judge) = Self::judge(*chr1) {
            match judge {
                Judge::Ws => self.state = State::Ws1,
                Judge::Equals => self.state = State::Equals,
            }
        } else {
            return error(&mut self.log(), &look_ahead_items, "keyval_sep_p.rs.87.");
        }
        PResult::Ongoing
    }

    fn parse_equals(&mut self, look_ahead_items: &LookAheadItems<char>) -> PResult {
        // 次の文字を調べます。
        let chr1 = look_ahead_items.get(1).unwrap();
        if let Some(judge) = Self::judge(*chr1) {
            match judge {
                Judge::Ws => {
                    return self.parse_ws2(look_ahead_items);
                }
                Judge::Equals => {
                    return error(&mut self.log(), &look_ahead_items, "keyval_sep_p.rs.87.");
                }
            }
        } else {
            return error(&mut self.log(), &look_ahead_items, "keyval_sep_p.rs.93.");
        }
    }

    fn parse_ws2(&mut self, look_ahead_items: &LookAheadItems<char>) -> PResult {
        let chr0 = look_ahead_items.get(0).unwrap();
        self.ws2
            .push_token(&Token::from_character(*chr0, TokenType::Ws));

        // 次のトークンを調べます。
        let chr1 = look_ahead_items.get(1).unwrap();
        if let Some(judge) = Self::judge(*chr1) {
            match judge {
                Judge::Ws => self.state = State::Ws2,
                Judge::Equals => {
                    return error(&mut self.log(), &look_ahead_items, "keyval_sep_p.rs.147.");
                }
            }
            PResult::Ongoing
        } else {
            PResult::End
        }
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();
        t.str("ws1", &self.ws1.to_string());
        t.str("ws2", &self.ws2.to_string());
        t
    }
}
