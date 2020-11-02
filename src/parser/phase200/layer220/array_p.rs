//! Array syntax parser.  
//! 配列構文パーサー。  
//!
//! # Examples
//!
//! ```
//! // [ 1, 2, 3 ]
//! ```

use crate::model::{layer110::CharacterType, layer210::LiteralValue, layer220::Array};
use crate::parser::phase200::error;
use crate::parser::phase200::error_via;
use crate::parser::phase200::{
    layer210::{BasicStringP, LiteralStringP, PResult},
    layer220::ArrayP,
};
use casual_logger::Table;
use look_ahead_items::LookAheadItems;

/// Array syntax machine state.  
/// 配列構文状態遷移。  
///
/// Example: `[ 'a', 'b', 'c' ]`.  
#[derive(Clone, Debug)]
pub enum State {
    /// After `[array]`.
    AfterArray,
    /// After `[],`.
    AfterCommaBehindArray,
    /// After `[ "a",`.
    AfterCommaBefindString,
    /// After `[ true,`.
    AfterCommaBehindLiteralValue,
    /// After " or '.
    AfterString,
    /// After `[`.
    First,
    /// `[ true` , か ] を待ちます。
    LiteralValue,
    /// After `[`.
    Array,
    DoubleQuotedString,
    End,
    LiteralString,
}

impl Default for ArrayP {
    fn default() -> Self {
        ArrayP {
            buffer: None,
            array_p: None,
            basic_string_p: None,
            literal_string_p: None,
            state: State::First,
        }
    }
}
impl ArrayP {
    pub fn flush(&mut self) -> Option<Array> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    /// # Arguments
    ///
    /// * `look_ahead_items` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///               結果。
    pub fn parse(&mut self, look_ahead_items: &LookAheadItems<char>) -> PResult {
        let chr0 = look_ahead_items.get(0).unwrap();
        match self.state {
            // After `]`.
            State::AfterArray => {
                match chr0.type_ {
                    '\t' | ' ' => {} // Ignore it.
                    // ,
                    ',' => {
                        self.state = State::AfterCommaBehindArray;
                    }
                    // ]
                    ']' => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => return error(&mut self.log(), &look_ahead_items, "array.rs.93."),
                }
            }
            // After `[],`.
            State::AfterCommaBehindArray => {
                match chr0 {
                    // [
                    '[' => {
                        self.array_p = Some(Box::new(ArrayP::default()));
                        self.state = State::Array;
                    }
                    '\t' | ' ' => {} // Ignore it.
                    // ]
                    ']' => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => return error(&mut self.log(), &look_ahead_items, "array.rs.130."),
                }
            }
            // ", ` の次。
            State::AfterCommaBefindString => {
                match chr0 {
                    // "
                    '"' => {
                        self.basic_string_p = Some(Box::new(BasicStringP::new()));
                        self.state = State::DoubleQuotedString;
                    }
                    // '
                    '\'' => {
                        self.literal_string_p = Some(Box::new(LiteralStringP::new()));
                        self.state = State::LiteralString;
                    }
                    '\t' | ' ' => {} // Ignore it.
                    // ]
                    ']' => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => return error(&mut self.log(), &look_ahead_items, "array.rs.176."),
                }
            }
            // After `literal,`.
            State::AfterCommaBehindLiteralValue => {
                match chr0 {
                    'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' => {
                        // TODO 数字なら正しいが、リテラル文字列だと間違い。キー・バリューかもしれない。
                        if let None = self.buffer {
                            self.buffer = Some(Array::default());
                        }
                        let m = self.buffer.as_mut().unwrap();
                        m.push_literal_string(&LiteralValue::from_character(chr0));
                        self.state = State::LiteralValue;
                    }
                    '\t' | ' ' => {} // Ignore it.
                    // `]`.
                    ']' => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => return error(&mut self.log(), &look_ahead_items, "array.rs.218."),
                }
            }
            // After " or '.
            State::AfterString => {
                match chr0 {
                    '\t' | ' ' => {} // Ignore it.
                    ',' => {
                        self.state = State::AfterCommaBefindString;
                    }
                    ']' => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => return error(&mut self.log(), &look_ahead_items, "array.rs.245."),
                }
            }
            // `[array]`.
            State::Array => {
                let p = self.array_p.as_mut().unwrap();
                match p.parse(look_ahead_items) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            if let None = self.buffer {
                                self.buffer = Some(Array::default());
                            }
                            let m = self.buffer.as_mut().unwrap();
                            m.push_array(&child_m);
                        } else {
                            // Empty array.
                        }
                        self.array_p = None;
                        self.state = State::AfterArray;
                    }
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            &look_ahead_items,
                            "array.rs.283.",
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            // After `[`.
            State::First => {
                match chr0 {
                    // "
                    '"' => {
                        self.basic_string_p = Some(Box::new(BasicStringP::new()));
                        self.state = State::DoubleQuotedString;
                    }
                    'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' => {
                        // TODO 数字なら正しいが、リテラル文字列だと間違い。キー・バリューかもしれない。
                        if let None = self.buffer {
                            self.buffer = Some(Array::default());
                        }
                        let m = self.buffer.as_mut().unwrap();
                        m.push_literal_string(&LiteralValue::from_character(chr0));
                        self.state = State::LiteralValue;
                    }
                    // `[`. Recursive.
                    '[' => {
                        self.array_p = Some(Box::new(ArrayP::default()));
                        self.state = State::Array;
                    }
                    // `]`. Empty array.
                    ']' => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    // '
                    '\'' => {
                        self.literal_string_p = Some(Box::new(LiteralStringP::new()));
                        self.state = State::LiteralString;
                    }
                    '\t' | ' ' => {} // Ignore it.
                    _ => return error(&mut self.log(), &look_ahead_items, "array.rs.358."),
                }
            }
            State::LiteralValue => match chr0 {
                ',' => {
                    self.state = State::AfterCommaBehindLiteralValue;
                }
                ']' => {
                    self.state = State::End;
                    return PResult::End;
                }
                _ => return error(&mut self.log(), &look_ahead_items, "array.rs.383."),
            },
            // "dog".
            State::DoubleQuotedString => {
                let p = self.basic_string_p.as_mut().unwrap();
                match p.parse(&look_ahead_items) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            if let None = self.buffer {
                                self.buffer = Some(Array::default());
                            }
                            let m = self.buffer.as_mut().unwrap();
                            m.push_double_quote_string(&child_m);
                            self.basic_string_p = None;
                            self.state = State::AfterString;
                        } else {
                            return error(&mut self.log(), &look_ahead_items, "array.rs.439.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            &look_ahead_items,
                            "array.rs.448.",
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            State::End => {
                return error(&mut self.log(), &look_ahead_items, "array.rs.466.");
            }
            // `'C:\temp'`.
            State::LiteralString => {
                let p = self.literal_string_p.as_mut().unwrap();
                match p.parse(&look_ahead_items) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            if let None = self.buffer {
                                self.buffer = Some(Array::default());
                            }
                            let m = self.buffer.as_mut().unwrap();
                            m.push_single_quote_string(&child_m);
                            self.literal_string_p = None;
                            self.state = State::AfterString;
                        } else {
                            return error(&mut self.log(), &look_ahead_items, "array.rs.493.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            &look_ahead_items,
                            "array.rs.502.",
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
        }
        PResult::Ongoing
    }
    /// Log.  
    /// ログ。  
    pub fn log(&self) -> Table {
        let mut t = Table::default()
            .str("parser", "ArrayP#parse")
            .str("state", &format!("{:?}", self.state))
            .clone();

        if let Some(p) = &self.basic_string_p {
            t.sub_t("basic_string_p", &p.log());
        }
        if let Some(p) = &self.literal_string_p {
            t.sub_t("literal_string_p", &p.log());
        }
        if let Some(p) = &self.array_p {
            t.sub_t("array_p", &p.log());
        }

        t
    }
}
