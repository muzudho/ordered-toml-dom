//! Syntax parser.
//! 構文パーサー。

use crate::object_model::{inline_table::InlineTableM, key_value::KeyValueM};
use crate::syntax::key_value::KeyValueP;
use crate::syntax::SyntaxParserResult;
use crate::token::{Token, TokenType};
use casual_logger::{Log, Table};

/// `{ key = value, key = value }`.
pub struct InlineTableP {
    state: MachineState,
    buffer: Option<InlineTableM>,
    key_value_p: Option<Box<KeyValueP>>,
}
impl Default for InlineTableP {
    fn default() -> Self {
        InlineTableP {
            state: MachineState::AfterLeftCurlyBracket,
            buffer: Some(InlineTableM::default()),
            key_value_p: None,
        }
    }
}
impl InlineTableP {
    pub fn flush(&mut self) -> Option<InlineTableM> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    /// # Returns
    ///
    /// * `SyntaxParserResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token) -> SyntaxParserResult {
        match self.state {
            MachineState::AfterLeftCurlyBracket => {
                match token.type_ {
                    TokenType::WhiteSpace => {} // Ignore it.
                    TokenType::Key => {
                        self.key_value_p = Some(Box::new(KeyValueP::new(&token)));
                        self.state = MachineState::KeyValue;
                    }
                    _ => panic!(Log::fatal_t(
                        "InlineTableP#parse/AfterValue",
                        self.err_table()
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token))
                    )),
                }
            }
            MachineState::KeyValue => {
                let p = self.key_value_p.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::End => {
                        if let Some(child_m) = p.flush() {
                            let m = self.buffer.as_mut().unwrap();
                            m.push_key_value(&child_m);
                            self.key_value_p = None;
                            self.state = MachineState::AfterKeyValue;
                        } else {
                            return SyntaxParserResult::Err(
                                self.err_table()
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    }
                    SyntaxParserResult::Err(table) => {
                        return SyntaxParserResult::Err(
                            self.err_table()
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                    SyntaxParserResult::Ongoing => {}
                }
            }
            MachineState::AfterKeyValue => match token.type_ {
                TokenType::WhiteSpace => {} // Ignore it.
                TokenType::Comma => {
                    self.state = MachineState::AfterLeftCurlyBracket;
                }
                TokenType::RightCurlyBracket => {
                    return SyntaxParserResult::End;
                }
                _ => panic!(Log::fatal_t(
                    "InlineTableP#parse/AfterValue",
                    self.err_table().str("token", &format!("{:?}", token))
                )),
            },
        }
        SyntaxParserResult::Ongoing
    }
    pub fn err_table(&self) -> Table {
        let mut t = Table::default()
            .str("parser", "InlineTableP#parse")
            .str("state", &format!("{:?}", self.state))
            .clone();
        if let Some(key_value_p) = &self.key_value_p {
            t.sub_t("key_value", &key_value_p.err_table());
        }
        t
    }
}

/// `{ key = value, key = value }`.
#[derive(Debug)]
enum MachineState {
    AfterLeftCurlyBracket,
    KeyValue,
    AfterKeyValue,
}
