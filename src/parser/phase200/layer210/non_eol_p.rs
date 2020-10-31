//! Non end-of-line parser.  
//! 非行末パーサー。  

use crate::model::{layer110::TokenType, layer210::NonEol};
use crate::parser::phase200::error;
use crate::parser::phase200::error_via;
use crate::parser::phase200::layer210::NonAsciiP;
use crate::parser::phase200::layer210::{NonEolP, PResult};
use crate::parser::phase200::LookAheadTokens;
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
    pub fn check_starts(token: &Token) -> bool {
        if NonAsciiP::check_starts(token) {
            return true;
        }
        let unicode = token.to_string_chars_nth(0).unwrap() as u32;
        match unicode {
            0x09 | 0x20..=0x7F => true,
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
        match self.state {
            State::End => {}
            State::First => {
                if NonAsciiP::check_starts(token0) {
                    self.non_ascii_p = Some(NonAsciiP::new());
                    self.state = State::NonAscii;
                    return self.parse_non_ascii(tokens);
                } else {
                    let ch = token0.to_string_chars_nth(0).unwrap();
                    let unicode = ch as u32;
                    match unicode {
                        0x09 | 0x20..=0x7F => {
                            if let None = self.buffer {
                                self.buffer = Some(NonEol::default());
                            }
                            let m = self.buffer.as_mut().unwrap();
                            m.push_token(&Token::new(
                                token0.column_number,
                                &ch.to_string(),
                                TokenType::NonEol,
                            ));

                            // TODO 次の文字をチェックすべきか、次のトークンをチェックすべきか？
                            let token1 = tokens.current.as_ref().unwrap();
                            if !Self::check_starts(&token1) {
                                return PResult::End;
                            }
                        }
                        _ => {
                            return error(&mut self.log(), &tokens, "non_eol_p.rs.86.");
                        }
                    }
                }
            }
            State::NonAscii => {
                return self.parse_non_ascii(tokens);
            }
        }
        PResult::Ongoing
    }

    fn parse_non_ascii(&mut self, tokens: &LookAheadTokens) -> PResult {
        let p = self.non_ascii_p.as_mut().unwrap();
        match p.parse(tokens) {
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
                    &tokens,
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
