//! Key value syntax parser.  
//! キー値構文パーサー。  
//!
//! # Examples
//!
//! ```
//! // key = val
//! ```

use crate::model::layer225::Keyval;
use crate::parser::phase200::error;
use crate::parser::phase200::error_via;
use crate::parser::phase200::{
    layer210::{KeyP, PResult},
    layer225::{KeyvalP, ValP},
};
use casual_logger::Table as LogTable;
use look_ahead_items::LookAheadItems;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug)]
pub enum State {
    AfterEquals,
    // After key.
    // キーの後。
    BeforeEqual,
    End,
    First,
    Val,
}

impl KeyvalP {
    pub fn new() -> Self {
        KeyvalP {
            key_buffer: None,
            val_buffer: None,
            key_p: Some(KeyP::default()),
            val_p: None,
            state: State::First,
        }
    }

    pub fn flush(&mut self) -> Option<Keyval> {
        let m = if let Some(key) = &self.key_buffer {
            if let Some(val) = &self.val_buffer {
                Some(Keyval::new(&key, &val))
            } else {
                panic!("keyval_p.rs.53.")
            }
        } else {
            panic!("keyval_p.rs.56.")
        };
        self.key_buffer = None;
        self.val_buffer = None;
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
        let chr0 = look_ahead_items.get(0).unwrap();
        match self.state {
            // After `=`.
            State::AfterEquals => {
                self.val_p = Some(ValP::default());
                self.state = State::Val;
            }
            // After key.
            State::BeforeEqual => {
                match chr0 {
                    '\t' | ' ' => {} //Ignored it.
                    // `=`.
                    '=' => {
                        self.state = State::AfterEquals;
                    }
                    _ => return error(&mut self.log(), &look_ahead_items, "keyval.rs.65."),
                }
            }
            State::First => {
                match chr0 {
                    '\t' | ' ' => {} //Ignored it.
                    'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' => {
                        let p = self.key_p.as_mut().unwrap();
                        match p.parse(&look_ahead_items) {
                            PResult::End => {
                                if let Some(child_m) = p.flush() {
                                    self.key_buffer = Some(child_m);
                                    self.key_p = None;
                                    self.state = State::BeforeEqual;
                                } else {
                                    return error(
                                        &mut self.log(),
                                        &look_ahead_items,
                                        "keyval.rs.84.",
                                    );
                                }
                            }
                            PResult::Err(mut table) => {
                                return error_via(
                                    &mut table,
                                    &mut self.log(),
                                    &look_ahead_items,
                                    "keyval.rs.84.",
                                );
                            }
                            PResult::Ongoing => {}
                        }
                    }
                    _ => return error(&mut self.log(), &look_ahead_items, "keyval.rs.65."),
                }
            }
            // After `=`.
            State::Val => {
                let p = self.val_p.as_mut().unwrap();
                match p.parse(look_ahead_items) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.val_buffer = Some(child_m);
                            self.val_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), &look_ahead_items, "keyval.rs.84.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            &look_ahead_items,
                            "keyval.rs.88.",
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            State::End => return error(&mut self.log(), &look_ahead_items, "keyval.rs.93."),
        }
        PResult::Ongoing
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> LogTable {
        let mut t = LogTable::default()
            .str("state", &format!("{:?}", self.state))
            .clone();
        if let Some(m) = &self.key_buffer {
            t.str("key_buffer", &m.to_string());
        }
        if let Some(m) = &self.val_buffer {
            t.str("val_buffer", &m.to_string());
        }
        if let Some(p) = &self.val_p {
            t.sub_t("val_p", &p.log());
        }
        t
    }
}
