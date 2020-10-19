//! Litera value parser.  
//! リテラル値パーサー。  

use crate::model::layer110::token::tokens_stringify;
use crate::model::{
    layer110::{Token, TokenType},
    layer210::LiteralValue,
};
use crate::parser::phase200::error;
use crate::parser::phase200::error_via;
use crate::parser::phase200::layer210::HexStringP;
use crate::parser::phase200::layer210::{LiteralValueP, PResult};
use crate::parser::phase200::LookAheadTokens;
use casual_logger::Table as LogTable;
use std::char::from_u32;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug, Clone)]
pub enum State {
    End,
    First,
    /// 0x
    ZeroXPrefix1st,
    ZeroXString,
}

impl Default for LiteralValueP {
    fn default() -> Self {
        LiteralValueP {
            hex_string_p: None,
            buffer: Some(LiteralValue::default()),
            state: State::First,
        }
    }
}
impl LiteralValueP {
    pub fn flush(&mut self) -> Option<LiteralValue> {
        if let Some(literal_value) = &self.buffer {
            let m = Some(literal_value.clone()); // TODO トリム要らないのでは。
            self.buffer = None;
            return m;
        }
        None
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
            State::End => {
                return error(&mut self.log(), &tokens, "literal_value.rs.57.");
            }
            State::First => {
                println!("[trace61 token0.type_={:?}]", &token0.type_);
                let zero_x = match token0.type_ {
                    TokenType::AbChar
                    | TokenType::Colon
                    | TokenType::Dot
                    | TokenType::Hyphen
                    | TokenType::Plus
                    | TokenType::Underscore => {
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&token0);
                        false
                    }
                    TokenType::NumChar => {
                        let length = if let Some(buffer) = &self.buffer {
                            buffer.to_string().len()
                        } else {
                            0
                        };
                        println!("[trace78 length={}]", length);

                        let base_number = if length == 0 {
                            if let Some(ch0) = token0.to_string().chars().nth(0) {
                                println!("[trace82 ch0={}]", ch0);
                                if ch0 == '0' {
                                    // 0x ?
                                    // Look-ahead.
                                    // 先読み。
                                    if let Some(token1) = tokens.one_ahead.as_ref() {
                                        match token1.type_ {
                                            TokenType::AbChar => {
                                                let ch1 = token1.to_string();
                                                if ch1 == "x" {
                                                    16
                                                } else {
                                                    0
                                                }
                                            }
                                            _ => 0,
                                        }
                                    } else {
                                        0
                                    }
                                } else {
                                    0
                                }
                            } else {
                                0
                            }
                        } else {
                            0
                        };
                        println!("[trace111 base_number={}]", base_number);

                        if base_number == 16 {
                            true
                        } else {
                            let m = self.buffer.as_mut().unwrap();
                            m.push_token(&token0);
                            false
                        }
                    }
                    _ => return error(&mut self.log(), &tokens, "literal_value_p.rs.38."),
                };

                // TODO 機能停止中。
                let zero_x = false;

                if zero_x {
                    // `0x` の `0` は無視します。
                    println!("[trace129={}]", token0);
                    self.state = State::ZeroXPrefix1st;
                    PResult::Ongoing
                } else {
                    // Look-ahead.
                    // 先読み。
                    if let Some(token1) = &tokens.one_ahead {
                        match token1.type_ {
                            TokenType::AbChar
                            | TokenType::Colon
                            | TokenType::Dot
                            | TokenType::Hyphen
                            | TokenType::NumChar
                            | TokenType::Plus
                            | TokenType::Underscore => PResult::Ongoing,
                            _ => {
                                self.state = State::End;
                                PResult::End
                            }
                        }
                    } else {
                        self.state = State::End;
                        PResult::End
                    }
                }
            }
            State::ZeroXPrefix1st => {
                // トークンの文字列の先頭が x のケースです。
                // 例えば `0xDEADBEEF` の場合、 `xDEADBEEF` という文字列トークンです。
                println!("[trace160={}]", token0);
                self.hex_string_p = Some(HexStringP::default().clone());
                self.state = State::ZeroXString;
                PResult::Ongoing
            }
            State::ZeroXString => {
                println!("[trace164={}]", token0);
                let p = self.hex_string_p.as_mut().unwrap();
                match p.parse(&tokens) {
                    PResult::End => {
                        // Filled.
                        // 満ちたなら。
                        let numeral_string = tokens_stringify(&p.flush());
                        println!("[trace173={}]", numeral_string);
                        // 数値変換はしない。
                        /*
                        let hex = match u32::from_str_radix(&numeral_string, 16) {
                            Ok(n) => n,
                            Err(why) => panic!("{}", why),
                        };
                        println!("[trace178={}]", hex);
                        println!("[trace180={}]", &hex.to_string());
                        */
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&Token::new(
                            token0.column_number,
                            &numeral_string,
                            TokenType::SPHexString,
                        ));

                        println!("[trace187={}]", &m.to_string());
                        println!("[trace188={:?}]", &m.to_string());

                        self.hex_string_p = None;
                        self.state = State::End;
                        return PResult::End;
                    }
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            &tokens,
                            "literal_value_p.rs.173.",
                        );
                    }
                    PResult::Ongoing => PResult::Ongoing,
                }
            }
        }
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> LogTable {
        let mut t = LogTable::default().clone();
        if let Some(m) = &self.buffer {
            t.str("buffer", &m.to_string());
        }
        t
    }
}
