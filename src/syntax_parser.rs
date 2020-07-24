use crate::lexical_parser::Token;
use crate::lexical_parser::{TokenLine, TokenType};
use casual_logger::{Log, Table};

#[derive(Debug)]
enum LineSyntaxParserMachineState {
    First,
    /// `key = right_value`.
    KeyPairSyntax,
    Unimplemented,
}

pub struct LineSyntaxScanner {}
impl Default for LineSyntaxScanner {
    fn default() -> Self {
        LineSyntaxScanner {}
    }
}
impl LineSyntaxScanner {
    pub fn scan_line(&mut self, token_line: &TokenLine) {
        let mut line_syntax_parser = LineSyntaxParser::default();

        for token in &token_line.tokens {
            line_syntax_parser.parse_token(token_line, token);
        }
        // End of line.

        if let Some(key_value_syntax) = &line_syntax_parser.key_value_syntax {
            Log::info_t(
                "LineSyntaxParser#parse_line/AfterEndOfLine",
                Table::default().str("key", &key_value_syntax.key).str(
                    "right_value",
                    &format!("{:?}", key_value_syntax.right_value_buf),
                ),
            );
        }
    }
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
    pub fn parse_token(&mut self, token_line: &TokenLine, token: &Token) {
        let mut state: LineSyntaxParserMachineState = LineSyntaxParserMachineState::First;

        match state {
            LineSyntaxParserMachineState::First => match token.type_ {
                TokenType::Key => {
                    Log::info_t(
                        "",
                        Table::default()
                            .str("token_line", &format!("{:?}", token_line))
                            .str("state", &format!("{:?}", state))
                            .str("token", &format!("{:?}", token)),
                    );
                    self.key_value_syntax = Some(KeyValueSyntaxParser::new(&token.value));
                    state = LineSyntaxParserMachineState::KeyPairSyntax;
                }
                _ => {
                    state = LineSyntaxParserMachineState::Unimplemented;
                }
            },
            LineSyntaxParserMachineState::KeyPairSyntax => {
                if let Some(key_value_syntax) = &mut self.key_value_syntax {
                    key_value_syntax.parse(token_line, token);
                } else {
                    panic!(Log::fatal_t(
                        "",
                        Table::default()
                            .str("token_line", &format!("{:?}", token_line))
                            .str("state", &format!("{:?}", state))
                            .str("token", &format!("{:?}", token))
                    ));
                }
            }
            LineSyntaxParserMachineState::Unimplemented => {
                Log::info_t(
                    "",
                    Table::default()
                        .str("token_line", &format!("{:?}", token_line))
                        .str("state", &format!("{:?}", state))
                        .str("token", &format!("{:?}", token)),
                );
            }
        }
    }
}

/// `key = right_value`.
#[derive(Debug)]
enum KeyValueSyntaxMachineState {
    AfterKey,
    AfterEquals,
    // AfterLeftCurlyBracket,
}

/// `key = value`.
pub struct KeyValueSyntaxParser {
    key_pair_state: KeyValueSyntaxMachineState,
    key: String,
    right_value_buf: TokenLine,
}
impl KeyValueSyntaxParser {
    pub fn new(key: &str) -> Self {
        KeyValueSyntaxParser {
            key_pair_state: KeyValueSyntaxMachineState::AfterKey,
            key: key.to_string(),
            right_value_buf: TokenLine::default(),
        }
    }
    pub fn parse(&mut self, token_line: &TokenLine, token: &Token) {
        match self.key_pair_state {
            KeyValueSyntaxMachineState::AfterKey => {
                match token.type_ {
                    TokenType::WhiteSpace => {} //Ignored it.
                    TokenType::Equals => {
                        self.key_pair_state = KeyValueSyntaxMachineState::AfterEquals;
                    }
                    _ => panic!(Log::fatal_t(
                        "",
                        Table::default()
                            .str("token_line", &format!("{:?}", token_line))
                            .str("state", &format!("{:?}", self.key_pair_state))
                            .str("key_pair_state", &format!("{:?}", self.key_pair_state))
                            .str("token", &format!("{:?}", token))
                    )),
                }
            }
            KeyValueSyntaxMachineState::AfterEquals => {
                // key_value_syntax.parse(token_line, token);
                match token.type_ {
                    TokenType::LeftCurlyBracket => {}
                    _ => {
                        self.right_value_buf.tokens.push(token.clone());
                    }
                }
            }
        }
    }
}

/*
/// `{ key = value, key = value }`.
#[derive(Debug)]
enum InlineTableSyntaxMachineState {
    AfterLeftCurlyBracket,
}
*/

/// `{ key = value, key = value }`.
pub struct InlineTableSyntaxParser {}
impl Default for InlineTableSyntaxParser {
    fn default() -> Self {
        InlineTableSyntaxParser {}
    }
}
impl InlineTableSyntaxParser {
    pub fn parse(&mut self, token_line: &TokenLine, token: &Token) {}
}
