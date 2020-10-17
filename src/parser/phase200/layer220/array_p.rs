//! Array syntax parser.  
//! 配列構文パーサー。  
//!
//! # Examples
//!
//! ```
//! // [ 1, 2, 3 ]
//! ```

use crate::model::{
    layer110::{Token, TokenType},
    layer210::LiteralValue,
    layer220::Array,
};
use crate::parser::phase200::{
    error, error_via,
    layer210::{BasicStringP, LiteralStringP, PResult},
    layer220::ArrayP,
};
use casual_logger::Table;

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
    /// * `tokens` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///               結果。
    pub fn parse(&mut self, tokens: (Option<&Token>, Option<&Token>, Option<&Token>)) -> PResult {
        let token0 = tokens.0.unwrap();
        match self.state {
            // After `]`.
            State::AfterArray => {
                match token0.type_ {
                    TokenType::WhiteSpace => {} // Ignore it.
                    // ,
                    TokenType::Comma => {
                        self.state = State::AfterCommaBehindArray;
                    }
                    // ]
                    TokenType::RightSquareBracket => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => return error(&mut self.log(), tokens, "array.rs.93."),
                }
            }
            // After `[],`.
            State::AfterCommaBehindArray => {
                match token0.type_ {
                    // [
                    TokenType::LeftSquareBracket => {
                        self.array_p = Some(Box::new(ArrayP::default()));
                        self.state = State::Array;
                    }
                    TokenType::WhiteSpace => {} // Ignore it.
                    // ]
                    TokenType::RightSquareBracket => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => return error(&mut self.log(), tokens, "array.rs.130."),
                }
            }
            // ", ` の次。
            State::AfterCommaBefindString => {
                match token0.type_ {
                    // "
                    TokenType::DoubleQuotation => {
                        self.basic_string_p = Some(Box::new(BasicStringP::new()));
                        self.state = State::DoubleQuotedString;
                    }
                    // '
                    TokenType::SingleQuotation => {
                        self.literal_string_p = Some(Box::new(LiteralStringP::new()));
                        self.state = State::LiteralString;
                    }
                    TokenType::WhiteSpace => {} // Ignore it.
                    // ]
                    TokenType::RightSquareBracket => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => return error(&mut self.log(), tokens, "array.rs.176."),
                }
            }
            // After `literal,`.
            State::AfterCommaBehindLiteralValue => {
                match token0.type_ {
                    TokenType::AlphabetString
                    | TokenType::NumeralString
                    | TokenType::Hyphen
                    | TokenType::Underscore => {
                        // TODO 数字なら正しいが、リテラル文字列だと間違い。キー・バリューかもしれない。
                        if let None = self.buffer {
                            self.buffer = Some(Array::default());
                        }
                        let m = self.buffer.as_mut().unwrap();
                        m.push_literal_string(&LiteralValue::from_token(token0));
                        self.state = State::LiteralValue;
                    }
                    TokenType::WhiteSpace => {} // Ignore it.
                    // `]`.
                    TokenType::RightSquareBracket => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => return error(&mut self.log(), tokens, "array.rs.218."),
                }
            }
            // After " or '.
            State::AfterString => {
                match token0.type_ {
                    TokenType::WhiteSpace => {} // Ignore it.
                    TokenType::Comma => {
                        self.state = State::AfterCommaBefindString;
                    }
                    TokenType::RightSquareBracket => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => return error(&mut self.log(), tokens, "array.rs.245."),
                }
            }
            // `[array]`.
            State::Array => {
                let p = self.array_p.as_mut().unwrap();
                match p.parse(tokens) {
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
                        return error_via(&mut table, &mut self.log(), tokens, "array.rs.283.");
                    }
                    PResult::Ongoing => {}
                }
            }
            // After `[`.
            State::First => {
                match token0.type_ {
                    // "
                    TokenType::DoubleQuotation => {
                        self.basic_string_p = Some(Box::new(BasicStringP::new()));
                        self.state = State::DoubleQuotedString;
                    }
                    TokenType::AlphabetString
                    | TokenType::NumeralString
                    | TokenType::Hyphen
                    | TokenType::Underscore => {
                        // TODO 数字なら正しいが、リテラル文字列だと間違い。キー・バリューかもしれない。
                        if let None = self.buffer {
                            self.buffer = Some(Array::default());
                        }
                        let m = self.buffer.as_mut().unwrap();
                        m.push_literal_string(&LiteralValue::from_token(token0));
                        self.state = State::LiteralValue;
                    }
                    // `[`. Recursive.
                    TokenType::LeftSquareBracket => {
                        self.array_p = Some(Box::new(ArrayP::default()));
                        self.state = State::Array;
                    }
                    // `]`. Empty array.
                    TokenType::RightSquareBracket => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    // '
                    TokenType::SingleQuotation => {
                        self.literal_string_p = Some(Box::new(LiteralStringP::new()));
                        self.state = State::LiteralString;
                    }
                    TokenType::WhiteSpace => {} // Ignore it.
                    _ => return error(&mut self.log(), tokens, "array.rs.358."),
                }
            }
            State::LiteralValue => match token0.type_ {
                TokenType::Comma => {
                    self.state = State::AfterCommaBehindLiteralValue;
                }
                TokenType::RightSquareBracket => {
                    self.state = State::End;
                    return PResult::End;
                }
                _ => return error(&mut self.log(), tokens, "array.rs.383."),
            },
            // "dog".
            State::DoubleQuotedString => {
                let p = self.basic_string_p.as_mut().unwrap();
                match p.parse(tokens) {
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
                            return error(&mut self.log(), tokens, "array.rs.439.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(&mut table, &mut self.log(), tokens, "array.rs.448.");
                    }
                    PResult::Ongoing => {}
                }
            }
            State::End => {
                return error(&mut self.log(), tokens, "array.rs.466.");
            }
            // `'C:\temp'`.
            State::LiteralString => {
                let p = self.literal_string_p.as_mut().unwrap();
                match p.parse(tokens) {
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
                            return error(&mut self.log(), tokens, "array.rs.493.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(&mut table, &mut self.log(), tokens, "array.rs.502.");
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
