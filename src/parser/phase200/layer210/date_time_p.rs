//! Comment syntax parser.  
//! コメント構文パーサー。  

use crate::model::layer110::token::tokens_stringify;
use crate::model::layer110::TokenType;
use crate::parser::phase200::Token;
use crate::parser::phase200::{
    error,
    layer210::{DateTime, DateTimeP, PResult},
    LookAheadTokens,
};
use casual_logger::Table;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug, Clone)]
pub enum State {
    AfterSecond,
    End,
    FirstOfDate,
    FirstOfTime,
    LongitudeZero,
    Point,
    OffsetSign,
}

impl DateTimeP {
    pub fn new(state: State) -> Self {
        DateTimeP {
            buffer: Vec::new(),
            state: state,
        }
    }
    pub fn flush(&mut self) -> Vec<Token> {
        let m = self.buffer.clone();
        self.buffer.clear();
        println!("[trace36.flush={}]", tokens_stringify(&m));
        m
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
        match self.state {
            State::AfterSecond => PResult::Ongoing,
            State::End => {
                return error(&mut self.log(), &tokens, "date_time_p.rs.50.");
            }
            State::FirstOfDate => {
                let token0 = tokens.current.as_ref().unwrap();
                let token1 = tokens.one_ahead.as_ref().unwrap();
                match token0.type_ {
                    TokenType::EndOfLine => {
                        println!("[trace59.]");
                        return PResult::End;
                    }
                    TokenType::AbChar => match token0.to_string().as_str() {
                        "T" => {
                            println!("[trace64.]");
                            self.buffer.push(token0.clone());
                            self.state = State::End;
                        }
                        _ => {
                            println!("[trace69.]");
                            return error(&mut self.log(), &tokens, "date_time_p.rs.63.");
                        }
                    },
                    TokenType::Hyphen | TokenType::NumChar => {
                        self.buffer.push(token0.clone());
                        match token1.type_ {
                            TokenType::AbChar => match token1.to_string().as_str() {
                                "T" => {
                                    println!(
                                        "[trace78={}|{}]",
                                        token0.to_string().as_str(),
                                        token1.to_string().as_str()
                                    );
                                    self.state = State::FirstOfTime;
                                }
                                _ => {
                                    println!(
                                        "[trace81={}|{}]",
                                        token0.to_string().as_str(),
                                        token1.to_string().as_str()
                                    );
                                    return error(&mut self.log(), &tokens, "date_time_p.rs.72.");
                                }
                            },
                            TokenType::Hyphen | TokenType::NumChar => {
                                println!("[trace86={}]", token0.to_string().as_str());
                            }
                            _ => {
                                println!("[trace89={}]", token0.to_string().as_str());
                                return PResult::End;
                            }
                        }
                    }
                    _ => {
                        println!("[trace95.]");
                        return error(&mut self.log(), &tokens, "date_time_p.rs.82.");
                    }
                }
                PResult::Ongoing
            }
            State::FirstOfTime => {
                let token0 = tokens.current.as_ref().unwrap();
                let token1 = tokens.one_ahead.as_ref().unwrap();
                match token0.type_ {
                    TokenType::EndOfLine => {
                        println!("[trace114.]");
                        return PResult::End;
                    }
                    TokenType::Colon | TokenType::NumChar => {
                        self.buffer.push(token0.clone());
                        match token1.type_ {
                            TokenType::AbChar => match token1.to_string().as_str() {
                                "Z" => {
                                    println!(
                                        "[trace124={}|{}]",
                                        token0.to_string().as_str(),
                                        token1.to_string().as_str()
                                    );
                                    self.state = State::LongitudeZero;
                                }
                                _ => {
                                    println!(
                                        "[trace132={}|{}]",
                                        token0.to_string().as_str(),
                                        token1.to_string().as_str()
                                    );
                                    return error(&mut self.log(), &tokens, "date_time_p.rs.72.");
                                }
                            },
                            TokenType::Dot => {
                                println!(
                                    "[trace141={}|{}]",
                                    token0.to_string().as_str(),
                                    token1.to_string().as_str()
                                );
                                self.state = State::Point;
                            }
                            TokenType::Plus | TokenType::Hyphen => {
                                println!(
                                    "[trace149={}|{}]",
                                    token0.to_string().as_str(),
                                    token1.to_string().as_str()
                                );
                                self.state = State::OffsetSign;
                            }
                            TokenType::Colon | TokenType::NumChar => {
                                println!(
                                    "[trace156={}|{}]",
                                    token0.to_string().as_str(),
                                    token1.to_string().as_str()
                                );
                            }
                            _ => {
                                println!(
                                    "[trace164={}|{}]",
                                    token0.to_string().as_str(),
                                    token1.to_string().as_str()
                                );
                                return PResult::End;
                            }
                        }
                    }
                    _ => {
                        self.buffer.push(token0.clone());
                    }
                }
                PResult::Ongoing
            }
            State::LongitudeZero => PResult::Ongoing,
            State::OffsetSign => PResult::Ongoing,
            State::Point => PResult::Ongoing,
        }
    }
    /// Log.  
    /// ログ。  
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();

        let mut buf = String::new();
        for token in &self.buffer {
            buf.push_str(&token.to_string());
        }

        t.str("value", &buf);
        t
    }
}
