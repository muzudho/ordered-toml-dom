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
    product: InlineTableM,
    key_value_p: Option<Box<KeyValueP>>,
}
impl Default for InlineTableP {
    fn default() -> Self {
        InlineTableP {
            state: MachineState::AfterLeftCurlyBracket,
            product: InlineTableM::default(),
            key_value_p: None,
        }
    }
}
impl InlineTableP {
    pub fn product(&self) -> &InlineTableM {
        &self.product
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
                        self.product.push_key_value(&KeyValueM::new(&token));
                        self.key_value_p = Some(Box::new(KeyValueP::new(&token)));
                        self.state = MachineState::AfterKey;
                    }
                    _ => panic!(Log::fatal_t(
                        "InlineTableP#parse/AfterValue",
                        Table::default()
                            .str("parser", "InlineTableP#parse")
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token))
                    )),
                }
            }
            MachineState::AfterKey => match self.key_value_p.as_mut().unwrap().parse(token) {
                SyntaxParserResult::End => {
                    self.key_value_p = None;
                    self.state = MachineState::AfterValue;
                }
                SyntaxParserResult::Err(table) => {
                    return SyntaxParserResult::Err(
                        Table::default()
                            .str("parser", "InlineTableP#parse")
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token))
                            .sub_t("error", &table)
                            .clone(),
                    )
                }
                SyntaxParserResult::Ongoing => {}
            },
            MachineState::AfterValue => match token.type_ {
                TokenType::WhiteSpace => {} // Ignore it.
                TokenType::Comma => {
                    self.state = MachineState::AfterLeftCurlyBracket;
                }
                TokenType::RightCurlyBracket => {
                    return SyntaxParserResult::End;
                }
                _ => panic!(Log::fatal_t(
                    "InlineTableP#parse/AfterValue",
                    Table::default()
                        .str("parser", "InlineTableP#parse")
                        .str("state", &format!("{:?}", self.state))
                        .str("token", &format!("{:?}", token))
                )),
            },
        }
        SyntaxParserResult::Ongoing
    }
    pub fn err_table(&self) -> Table {
        let mut t = Table::default().clone();
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
    AfterKey,
    AfterValue,
}
