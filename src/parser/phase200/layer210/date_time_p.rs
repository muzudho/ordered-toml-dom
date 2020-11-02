//! Comment syntax parser.  
//! コメント構文パーサー。  

use crate::model::layer110::TokenType;
use crate::parser::phase200::Token;
use crate::parser::phase200::{
    error,
    layer210::{DateTimeP, PResult},
};
use casual_logger::Table;
use look_ahead_items::LookAheadItems;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug, Clone)]
pub enum State {
    End,
    FirstOfDate,
    FirstOfTime,
    LongitudeZero,
    FractionalSeconds,
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
        // println!("[trace36.flush={}]", tokens_stringify(&m));
        m
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
            State::End => {
                return error(&mut self.log(), &look_ahead_items, "date_time_p.rs.50.");
            }
            State::FirstOfDate => {
                let chr0 = look_ahead_items.get(0).unwrap();
                let chr1 = look_ahead_items.get(1).unwrap();
                match chr0 {
                    '\r' | '\t' => {
                        // println!("[trace59.]");
                        return PResult::End;
                    }
                    'A'..='Z' | 'a'..='z' => match chr0.to_string().as_str() {
                        "T" => {
                            // println!("[trace64.]");
                            self.buffer
                                .push(Token::from_character(*chr0, TokenType::DateTime));
                            self.state = State::End;
                        }
                        _ => {
                            // println!("[trace69.]");
                            return error(&mut self.log(), &look_ahead_items, "date_time_p.rs.63.");
                        }
                    },
                    '-' | '0'..='9' => {
                        self.buffer
                            .push(Token::from_character(*chr0, TokenType::DateTime));
                        match chr1 {
                            'A'..='Z' | 'a'..='z' => match chr1.to_string().as_str() {
                                "T" => {
                                    /*
                                    println!(
                                        // "[trace78={}|{}]",
                                        chr0.to_string().as_str(),
                                        chr1.to_string().as_str()
                                    );
                                    */
                                    self.state = State::FirstOfTime;
                                }
                                _ => {
                                    /*
                                    println!(
                                        // "[trace81={}|{}]",
                                        chr0.to_string().as_str(),
                                        chr1.to_string().as_str()
                                    );
                                    */
                                    return error(
                                        &mut self.log(),
                                        &look_ahead_items,
                                        "date_time_p.rs.72.",
                                    );
                                }
                            },
                            '-' | '0'..='9' => {
                                // println!("[trace86={}]", chr0.to_string().as_str());
                            }
                            _ => {
                                // println!("[trace89={}]", chr0.to_string().as_str());
                                return PResult::End;
                            }
                        }
                    }
                    _ => {
                        // println!("[trace95.]");
                        return error(&mut self.log(), &look_ahead_items, "date_time_p.rs.82.");
                    }
                }
                PResult::Ongoing
            }
            State::FirstOfTime => {
                let chr0 = look_ahead_items.get(0).unwrap();
                let chr1 = look_ahead_items.get(1).unwrap();
                match chr0 {
                    '\r' | '\t' => {
                        // println!("[trace114.]");
                        return PResult::End;
                    }
                    ':' | '0'..='9' => {
                        self.buffer
                            .push(Token::from_character(chr0, TokenType::DateTime));
                        match chr1 {
                            'A'..='Z' | 'a'..='z' => match chr1.to_string().as_str() {
                                "Z" => {
                                    /*
                                    println!(
                                        // "[trace124={}|{}]",
                                        chr0.to_string().as_str(),
                                        chr1.to_string().as_str()
                                    );
                                    */
                                    self.state = State::LongitudeZero;
                                }
                                _ => {
                                    /*
                                    println!(
                                        // "[trace132={}|{}]",
                                        chr0.to_string().as_str(),
                                        chr1.to_string().as_str()
                                    );
                                    */
                                    return error(
                                        &mut self.log(),
                                        &look_ahead_items,
                                        "date_time_p.rs.72.",
                                    );
                                }
                            },
                            '.' => {
                                /*
                                println!(
                                    // "[trace141={}|{}]",
                                    chr0.to_string().as_str(),
                                    chr1.to_string().as_str()
                                );
                                */
                                self.state = State::FractionalSeconds;
                            }
                            '+' | '-' => {
                                /*
                                println!(
                                    // "[trace149={}|{}]",
                                    chr0.to_string().as_str(),
                                    chr1.to_string().as_str()
                                );
                                */
                                self.state = State::OffsetSign;
                            }
                            ':' | '0'..='9' => {
                                /*
                                println!(
                                    // "[trace156={}|{}]",
                                    chr0.to_string().as_str(),
                                    chr1.to_string().as_str()
                                );
                                */
                            }
                            _ => {
                                /*
                                println!(
                                    // "[trace164={}|{}]",
                                    chr0.to_string().as_str(),
                                    chr1.to_string().as_str()
                                );
                                */
                                return PResult::End;
                            }
                        }
                    }
                    _ => {
                        self.buffer
                            .push(Token::from_character(*chr0, TokenType::DateTime));
                    }
                }
                PResult::Ongoing
            }
            State::LongitudeZero => {
                let chr0 = look_ahead_items.get(0).unwrap();
                self.buffer
                    .push(Token::from_character(*chr0, TokenType::DateTime));
                self.state = State::End;
                PResult::End
            }
            State::OffsetSign => {
                let chr0 = look_ahead_items.get(0).unwrap();
                let chr1 = look_ahead_items.get(1).unwrap();
                match chr0 {
                    ':' | '-' | '0'..='9' | '+' => {
                        self.buffer
                            .push(Token::from_character(*chr0, TokenType::DateTime));
                        match chr1 {
                            ':' | '0'..='9' => {
                                /*
                                println!(
                                    // "[trace193={}|{}]",
                                    chr0.to_string().as_str(),
                                    chr1.to_string().as_str()
                                );
                                */
                            }
                            _ => {
                                /*
                                println!(
                                    // "[trace200={}|{}]",
                                    chr0.to_string().as_str(),
                                    chr1.to_string().as_str()
                                );
                                */
                                return PResult::End;
                            }
                        }
                    }
                    _ => {
                        /*
                        println!(
                            // "[trace210={}|{}]",
                            chr0.to_string().as_str(),
                            chr1.to_string().as_str()
                        );
                        */
                        return error(&mut self.log(), &look_ahead_items, "date_time_p.rs.244.");
                    }
                }
                PResult::Ongoing
            }
            State::FractionalSeconds => {
                let chr0 = look_ahead_items.get(0).unwrap();
                let chr1 = look_ahead_items.get(1).unwrap();
                match chr0 {
                    '.' | '0'..='9' => {
                        self.buffer
                            .push(Token::from_character(*chr0, TokenType::DateTime));
                        match chr1 {
                            '-' | '+' => {
                                // - or +.
                                /*
                                println!(
                                    // "[trace229={}|{}]",
                                    chr0.to_string().as_str(),
                                    chr1.to_string().as_str()
                                );
                                */
                                self.state = State::OffsetSign;
                            }
                            '.' | '0'..='9' => {
                                /*
                                println!(
                                    // "[trace237={}|{}]",
                                    chr0.to_string().as_str(),
                                    chr1.to_string().as_str()
                                );
                                */
                            }
                            _ => {
                                /*
                                println!(
                                    // "[trace244={}|{}]",
                                    chr0.to_string().as_str(),
                                    chr1.to_string().as_str()
                                );
                                */
                                return PResult::End;
                            }
                        }
                    }
                    _ => {
                        /*
                        println!(
                            // "[trace219={}|{}]",
                            chr0.to_string().as_str(),
                            chr1.to_string().as_str()
                        );
                        */
                        return error(&mut self.log(), &look_ahead_items, "date_time_p.rs.244.");
                    }
                }
                PResult::Ongoing
            }
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
