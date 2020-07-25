use crate::lexical_parser::Token;
use crate::lexical_parser::{TokenLine, TokenType};
use crate::syntax::inline_table::InlineTableSyntaxParser;
use casual_logger::{Log, Table};

/// `key = right_value`.
#[derive(Debug)]
enum KeyValueSyntaxMachineState {
    AfterKey,
    AfterEquals,
    AfterLeftCurlyBracket,
}

/// `key = value`.
pub struct KeyValueSyntaxParser {
    state: KeyValueSyntaxMachineState,
    key: String,
    right_value_buf: TokenLine,
    inline_table_syntax_parser: Option<InlineTableSyntaxParser>,
}
impl KeyValueSyntaxParser {
    pub fn new(key: &str) -> Self {
        KeyValueSyntaxParser {
            state: KeyValueSyntaxMachineState::AfterKey,
            key: key.to_string(),
            right_value_buf: TokenLine::default(),
            inline_table_syntax_parser: None,
        }
    }
    pub fn parse(&mut self, token: &Token) {
        match self.state {
            KeyValueSyntaxMachineState::AfterKey => {
                match token.type_ {
                    TokenType::WhiteSpace => {} //Ignored it.
                    TokenType::Equals => {
                        self.state = KeyValueSyntaxMachineState::AfterEquals;
                    }
                    _ => panic!(Log::fatal_t(
                        "",
                        Table::default()
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token))
                    )),
                }
            }
            KeyValueSyntaxMachineState::AfterEquals => {
                // key_value_syntax.parse(token_line, token);
                match token.type_ {
                    TokenType::LeftCurlyBracket => {
                        self.inline_table_syntax_parser = Some(InlineTableSyntaxParser::default());
                        self.state = KeyValueSyntaxMachineState::AfterLeftCurlyBracket;
                    }
                    _ => {
                        self.right_value_buf.tokens.push(token.clone());
                    }
                }
            }
            KeyValueSyntaxMachineState::AfterLeftCurlyBracket => {
                if let Some(p) = &mut self.inline_table_syntax_parser {
                    p.parse(token);
                } else {
                    panic!(Log::fatal_t(
                        "",
                        Table::default()
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token))
                    ));
                }
            }
        }
    }
    pub fn log(&self) -> Table {
        let mut t = Table::default()
            .str("state", &format!("{:?}", self.state))
            .str("key", &self.key)
            .str("right_value_buf", &format!("{:?}", self.right_value_buf))
            .clone();
        if let Some(inline_table_syntax_parser) = &self.inline_table_syntax_parser {
            t.sub_t("inline_table", &inline_table_syntax_parser.log());
        }
        t
    }
}
