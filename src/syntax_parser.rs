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

pub struct SyntaxParser {}
impl Default for SyntaxParser {
    fn default() -> Self {
        SyntaxParser {}
    }
}
impl SyntaxParser {
    pub fn parse_line(&mut self, token_line: &TokenLine) {
        let mut state: MachineState = MachineState::First;
        let mut key_pair_state: KeyPairSyntaxMachineState = KeyPairSyntaxMachineState::AfterKey;
        let mut right_value_buf: TokenLine = TokenLine::default();

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
                        right_value_buf.tokens.push(token.clone());
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
                Log::info_t(
                    "SyntaxParser#parse_line",
                    Table::default().str("right_value_buf", &format!("{:?}", right_value_buf)),
                );
            }
            _ => {}
        }
    }
}
