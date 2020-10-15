//! Key value syntax parser.  
//! キー値構文パーサー。  
//!
//! # Examples
//!
//! ```
//! // key = right_value
//! ```

use crate::model::{
    layer110::token::{Token, TokenType},
    layer225::KeyValue,
};
use crate::parser::phase200::{
    error, error_via,
    layer210::PResult,
    layer225::{KeyValueP, RightValueP},
};
use casual_logger::Table as LogTable;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug)]
pub enum State {
    // After key.
    // キーの後。
    First,
    AfterEquals,
    RightValue,
    End,
}

impl KeyValueP {
    pub fn new(key: &Token) -> Self {
        KeyValueP {
            buffer: None,
            key: key.clone(),
            right_value_p: None,
            state: State::First,
        }
    }

    pub fn flush(&mut self) -> Option<KeyValue> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }

    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, look_ahead_token: Option<&Token>, token: &Token) -> PResult {
        match self.state {
            // After key.
            State::First => {
                println!("test.e1.");
                match token.type_ {
                    TokenType::WhiteSpace => {} //Ignored it.
                    // `=`.
                    TokenType::Equals => {
                        self.state = State::AfterEquals;
                    }
                    _ => return error(&mut self.log(), token, "key_value.rs.65."),
                }
            }
            // After `=`.
            State::AfterEquals => {
                println!("test.e2.");
                self.right_value_p = Some(RightValueP::default());
                self.state = State::RightValue;
            }
            // After `=`.
            State::RightValue => {
                println!("test.right-value.");
                let p = self.right_value_p.as_mut().unwrap();
                match p.parse(look_ahead_token, token) {
                    PResult::End => {
                        println!("test.e4.");
                        if let Some(child_m) = p.flush() {
                            println!("test.e5.");
                            self.buffer = Some(KeyValue::new(&self.key, &child_m));
                            self.right_value_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), token, "key_value.rs.84.");
                        }
                    }
                    PResult::Err(mut table) => {
                        println!("test.e6.error. token=|{:?}|", token);
                        return error_via(&mut table, &mut self.log(), token, "key_value.rs.88.");
                    }
                    PResult::Ongoing => {
                        println!("test.right-value.ongoing.");
                    }
                }
            }
            State::End => return error(&mut self.log(), token, "key_value.rs.93."),
        }
        PResult::Ongoing
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> LogTable {
        let mut t = LogTable::default()
            .str("buffer", &format!("{:?}", &self.buffer))
            .str("state", &format!("{:?}", self.state))
            .clone();
        if let Some(right_value_p) = &self.right_value_p {
            t.sub_t("right_value_p", &right_value_p.log());
        }
        t
    }
}
