//! Syntax parser.
//! 構文パーサー。

use crate::lexical_parser::{Token, TokenLine, TokenType};
use crate::syntax::key_value::KeyValueParser;
use casual_logger::{Log, Table};

/// `{ key = value, key = value }`.
pub struct InlineTableParser {
    state: MachineState,
    rest: TokenLine,
    key_value_syntax_parser: Option<Box<KeyValueParser>>,
}
impl Default for InlineTableParser {
    fn default() -> Self {
        InlineTableParser {
            state: MachineState::AfterLeftCurlyBracket,
            rest: TokenLine::default(),
            key_value_syntax_parser: None,
        }
    }
}
impl InlineTableParser {
    /// # Returns
    ///
    /// End of syntax.
    pub fn parse(&mut self, token: &Token) -> bool {
        match self.state {
            MachineState::AfterLeftCurlyBracket => {
                match token.type_ {
                    TokenType::WhiteSpace => {} // Ignore it.
                    TokenType::Key => {
                        self.key_value_syntax_parser =
                            Some(Box::new(KeyValueParser::new(&token.value)));
                        self.state = MachineState::AfterKey;
                    }
                    _ => {
                        self.rest.tokens.push(token.clone());
                    }
                }
            }
            MachineState::AfterKey => {
                if self.key_value_syntax_parser.as_mut().unwrap().parse(token) {
                    self.key_value_syntax_parser = None;
                    self.state = MachineState::AfterValue;
                }
            }
            MachineState::AfterValue => match token.type_ {
                TokenType::Comma => {
                    self.state = MachineState::AfterLeftCurlyBracket;
                }
                TokenType::RightCurlyBracket => {
                    return true;
                }
                _ => panic!(Log::fatal_t(
                    "InlineTableParser#parse/AfterValue",
                    Table::default()
                        .str("state", &format!("{:?}", self.state))
                        .str("token", &format!("{:?}", token))
                )),
            },
        }
        false
    }
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();
        if !self.rest.tokens.is_empty() {
            t.str("rest", &format!("{:?}", self.rest));
        }
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
