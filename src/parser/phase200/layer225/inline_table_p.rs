//! Inline table syntax parser.  
//! インライン・テーブル構文パーサー。  

use crate::model::{
    layer110::{Token, TokenType},
    layer225::InlineTable,
};
use crate::parser::phase200::error2;
use crate::parser::phase200::error_via2;
use crate::parser::phase200::LookAheadTokens;
use crate::parser::phase200::{
    layer210::PResult,
    layer225::{InlineTableP, KeyValueP},
};
use casual_logger::Table as LogTable;

/// Inline table syntax machine state.  
/// インライン・テーブル構文状態遷移。  
///
/// Example: `{ key = value, key = value }`.  
#[derive(Debug)]
pub enum State {
    // First. After `{`.
    First,
    KeyValue,
    AfterKeyValue,
}

impl Default for InlineTableP {
    fn default() -> Self {
        InlineTableP {
            state: State::First,
            buffer: Some(InlineTable::default()),
            key_value_p: None,
        }
    }
}
impl InlineTableP {
    pub fn flush(&mut self) -> Option<InlineTable> {
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
    pub fn parse(
        &mut self,
        tokens_old: (Option<&Token>, Option<&Token>, Option<&Token>),
    ) -> PResult {
        let tokens = LookAheadTokens::from_old(tokens_old);
        let token0 = tokens.current.as_ref().unwrap();
        match self.state {
            // After `{`.
            State::First => {
                match token0.type_ {
                    TokenType::WhiteSpaceString => {} // Ignore it.
                    // `apple.banana`
                    TokenType::AlphabetCharacter
                    | TokenType::AlphabetString
                    | TokenType::NumeralString
                    | TokenType::Hyphen
                    | TokenType::Underscore => {
                        self.key_value_p = Some(Box::new(KeyValueP::new()));
                        self.state = State::KeyValue;
                        match self.key_value_p.as_mut().unwrap().parse(tokens_old) {
                            PResult::End => {
                                // 1トークンでは終わらないから。
                                return error2(&mut self.log(), &tokens, "inline_table.rs.64.");
                            }
                            PResult::Err(mut table) => {
                                return error_via2(
                                    &mut table,
                                    &mut self.log(),
                                    &tokens,
                                    "inline_table.rs.71.",
                                )
                            }
                            PResult::Ongoing => {}
                        }
                    }
                    TokenType::RightCurlyBracket => {
                        // Empty inline-table.
                        return PResult::End;
                    }
                    _ => return error2(&mut self.log(), &tokens, "inline_table.rs.63."),
                }
            }
            // `apple.banana`.
            State::KeyValue => {
                let p = self.key_value_p.as_mut().unwrap();
                match p.parse(tokens_old) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer.as_mut().unwrap().push_key_value(&child_m);
                            self.key_value_p = None;
                            self.state = State::AfterKeyValue;
                        } else {
                            return error2(&mut self.log(), &tokens, "inline_table.rs.76.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via2(
                            &mut table,
                            &mut self.log(),
                            &tokens,
                            "inline_table.rs.80.",
                        )
                    }
                    PResult::Ongoing => {}
                }
            }
            // After `banana = 3`.
            State::AfterKeyValue => match token0.type_ {
                TokenType::WhiteSpaceString => {} // Ignore it.
                // `,`
                TokenType::Comma => {
                    self.state = State::First;
                }
                // `}`
                TokenType::RightCurlyBracket => {
                    return PResult::End;
                }
                _ => return error2(&mut self.log(), &tokens, "inline_table.rs.96."),
            },
        }
        PResult::Ongoing
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> LogTable {
        let mut t = LogTable::default()
            .str("parser", "InlineTableP#parse")
            .str("state", &format!("{:?}", self.state))
            .clone();
        if let Some(p) = &self.key_value_p {
            t.sub_t("key_value", &p.log());
        }
        t
    }
}
