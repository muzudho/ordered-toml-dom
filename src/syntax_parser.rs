use crate::lexical_parser::Token;
use crate::lexical_parser::{TokenLine, TokenType};
use casual_logger::{Log, Table};

#[derive(Debug)]
enum MachineState {
    First,
    /// `key = right_value`.
    KeyPairSyntax,
    Unimplemented,
}
/// `key = right_value`.
#[derive(Debug)]
enum KeyPairSyntaxMachineState {
    AfterKey,
    AfterEquals,
}

pub struct LineSyntaxParser {
    key_value_syntax: Option<KeyValueSyntaxParser>,
}
impl Default for LineSyntaxParser {
    fn default() -> Self {
        LineSyntaxParser {
            key_value_syntax: None,
        }
    }
}
impl LineSyntaxParser {
    pub fn parse_line(&mut self, token_line: &TokenLine) {
        let mut state: MachineState = MachineState::First;
        let mut key_pair_state: KeyPairSyntaxMachineState = KeyPairSyntaxMachineState::AfterKey;

        for token in &token_line.tokens {
            match state {
                MachineState::First => match token.type_ {
                    TokenType::Key => {
                        Log::info_t(
                            "",
                            Table::default()
                                .str("token_line", &format!("{:?}", token_line))
                                .str("state", &format!("{:?}", state))
                                .str("key_pair_state", &format!("{:?}", key_pair_state))
                                .str("token", &format!("{:?}", token)),
                        );
                        self.key_value_syntax = Some(KeyValueSyntaxParser::new(&token.value));
                        state = MachineState::KeyPairSyntax;
                    }
                    _ => {
                        state = MachineState::Unimplemented;
                    }
                },
                MachineState::KeyPairSyntax => match key_pair_state {
                    KeyPairSyntaxMachineState::AfterKey => {
                        match token.type_ {
                            TokenType::WhiteSpace => {} //Ignored it.
                            TokenType::Equals => {
                                key_pair_state = KeyPairSyntaxMachineState::AfterEquals;
                            }
                            _ => panic!(Log::fatal_t(
                                "",
                                Table::default()
                                    .str("token_line", &format!("{:?}", token_line))
                                    .str("state", &format!("{:?}", state))
                                    .str("key_pair_state", &format!("{:?}", key_pair_state))
                                    .str("token", &format!("{:?}", token))
                            )),
                        }
                    }
                    KeyPairSyntaxMachineState::AfterEquals => {
                        if let Some(key_value_syntax) = &mut self.key_value_syntax {
                            key_value_syntax.parse(token);
                        } else {
                            panic!(Log::fatal_t(
                                "",
                                Table::default()
                                    .str("token_line", &format!("{:?}", token_line))
                                    .str("state", &format!("{:?}", state))
                                    .str("key_pair_state", &format!("{:?}", key_pair_state))
                                    .str("token", &format!("{:?}", token))
                            ));
                        }
                    }
                },
                MachineState::Unimplemented => {
                    Log::info_t(
                        "",
                        Table::default()
                            .str("token_line", &format!("{:?}", token_line))
                            .str("state", &format!("{:?}", state))
                            .str("key_pair_state", &format!("{:?}", key_pair_state))
                            .str("token", &format!("{:?}", token)),
                    );
                }
            }
        }
        // End of line.

        match state {
            MachineState::KeyPairSyntax => {
                if let Some(key_value_syntax) = &self.key_value_syntax {
                    Log::info_t(
                        "LineSyntaxParser#parse_line/AfterEndOfLine",
                        Table::default().str("key", &key_value_syntax.key).str(
                            "right_value",
                            &format!("{:?}", key_value_syntax.right_value_buf),
                        ),
                    );
                } else {
                    panic!(Log::fatal_t(
                        "",
                        Table::default()
                            .str("token_line", &format!("{:?}", token_line))
                            .str("state", &format!("{:?}", state))
                            .str("key_pair_state", &format!("{:?}", key_pair_state))
                    ));
                }
            }
            _ => {}
        }
    }
}

/// `key = value`.
pub struct KeyValueSyntaxParser {
    key: String,
    right_value_buf: TokenLine,
}
impl KeyValueSyntaxParser {
    pub fn new(key: &str) -> Self {
        KeyValueSyntaxParser {
            key: key.to_string(),
            right_value_buf: TokenLine::default(),
        }
    }
    pub fn parse(&mut self, token: &Token) {
        self.right_value_buf.tokens.push(token.clone());
    }
}
