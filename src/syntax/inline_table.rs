//! Syntax parser.
//! 構文パーサー。

use crate::lexical_parser::{Token, TokenType};
use crate::object_model::{inline_table::InlineTableM, key_value::KeyValueM, value::ValueM};
use crate::syntax::key_value::KeyValueP;
use crate::syntax::SyntaxParserResult;
use casual_logger::{Log, Table};

/// `{ key = value, key = value }`.
pub struct InlineTableP {
    state: MachineState,
    product: InlineTableM,
    key_value_syntax_parser: Option<Box<KeyValueP>>,
}
impl Default for InlineTableP {
    fn default() -> Self {
        InlineTableP {
            state: MachineState::AfterLeftCurlyBracket,
            product: InlineTableM::default(),
            key_value_syntax_parser: None,
        }
    }
}
impl InlineTableP {
    pub fn product(&self) -> InlineTableM {
        self.product.clone()
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
                        self.product
                            .items
                            .push(ValueM::KeyValue(KeyValueM::new(&token.value)));
                        self.key_value_syntax_parser = Some(Box::new(KeyValueP::new(&token.value)));
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
            MachineState::AfterKey => {
                match self.key_value_syntax_parser.as_mut().unwrap().parse(token) {
                    SyntaxParserResult::Ok(end_of_syntax) => {
                        if end_of_syntax {
                            self.key_value_syntax_parser = None;
                            self.state = MachineState::AfterValue;
                        }
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
                }
            }
            MachineState::AfterValue => match token.type_ {
                TokenType::WhiteSpace => {} // Ignore it.
                TokenType::Comma => {
                    self.state = MachineState::AfterLeftCurlyBracket;
                }
                TokenType::RightCurlyBracket => {
                    return SyntaxParserResult::Ok(true);
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
        SyntaxParserResult::Ok(false)
    }
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();
        if let Some(key_value_syntax_parser) = &self.key_value_syntax_parser {
            t.sub_t("key_value", &key_value_syntax_parser.log());
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
