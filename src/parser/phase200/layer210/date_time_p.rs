//! Comment syntax parser.  
//! コメント構文パーサー。  

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
                return error(&mut self.log(), &tokens, "date_time_p.rs.42.");
            }
            State::FirstOfDate => {
                let token0 = tokens.current.as_ref().unwrap();
                match token0.type_ {
                    TokenType::EndOfLine => return PResult::End,
                    _ => {
                        self.buffer.push(token0.clone());
                    }
                }
                PResult::Ongoing
            }
            State::FirstOfTime => {
                let token0 = tokens.current.as_ref().unwrap();
                match token0.type_ {
                    TokenType::EndOfLine => return PResult::End,
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
