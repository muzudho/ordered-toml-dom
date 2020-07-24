use crate::lexical_parser::Token;
use crate::lexical_parser::{TokenLine, TokenType};
use casual_logger::{Log, Table};
use std::fmt;

pub struct LineSyntaxScanner {
    line_syntax_parser: LineSyntaxParser,
}
impl Default for LineSyntaxScanner {
    fn default() -> Self {
        LineSyntaxScanner {
            line_syntax_parser: LineSyntaxParser::default(),
        }
    }
}
impl LineSyntaxScanner {
    pub fn scan_line(&mut self, token_line: &TokenLine) {
        for token in &token_line.tokens {
            self.line_syntax_parser.parse(token_line, token);
        }
        // End of line.

        if let Some(key_value_syntax) = &self.line_syntax_parser.key_value_syntax {
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
impl fmt::Debug for LineSyntaxScanner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "contents=|{:?}|", self.line_syntax_parser)
    }
}

#[derive(Debug)]
enum LineSyntaxParserMachineState {
    First,
    /// `key = right_value`.
    KeyPairSyntax,
    Unimplemented,
}

struct LineSyntaxParser {
    state: LineSyntaxParserMachineState,
    key_value_syntax: Option<KeyValueSyntaxParser>,
}
impl Default for LineSyntaxParser {
    fn default() -> Self {
        LineSyntaxParser {
            state: LineSyntaxParserMachineState::First,
            key_value_syntax: None,
        }
    }
}
impl LineSyntaxParser {
    pub fn parse(&mut self, token_line: &TokenLine, token: &Token) {
        match self.state {
            LineSyntaxParserMachineState::First => match token.type_ {
                TokenType::Key => {
                    Log::info_t(
                        "LineSyntaxParser#parse",
                        Table::default()
                            .str("token_line", &format!("{:?}", token_line))
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token)),
                    );
                    self.key_value_syntax = Some(KeyValueSyntaxParser::new(&token.value));
                    self.state = LineSyntaxParserMachineState::KeyPairSyntax;
                }
                _ => {
                    self.state = LineSyntaxParserMachineState::Unimplemented;
                }
            },
            LineSyntaxParserMachineState::KeyPairSyntax => {
                if let Some(key_value_syntax) = &mut self.key_value_syntax {
                    key_value_syntax.parse(token_line, token);
                } else {
                    panic!(Log::fatal_t(
                        "LineSyntaxParser#parse",
                        Table::default()
                            .str("token_line", &format!("{:?}", token_line))
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token))
                    ));
                }
            }
            LineSyntaxParserMachineState::Unimplemented => {
                Log::info_t(
                    "LineSyntaxParser#parse",
                    Table::default()
                        .str("token_line", &format!("{:?}", token_line))
                        .str("state", &format!("{:?}", self.state))
                        .str("token", &format!("{:?}", token)),
                );
            }
        }
    }
}
impl fmt::Debug for LineSyntaxParser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "state=|{:?}|
{}",
            self.state,
            if let Some(key_value_syntax) = &self.key_value_syntax {
                format!(
                    "{:?}
",
                    key_value_syntax
                )
            } else {
                "".to_string()
            }
        )
    }
}

/// `key = right_value`.
#[derive(Debug)]
enum KeyValueSyntaxMachineState {
    AfterKey,
    AfterEquals,
    AfterLeftCurlyBracket,
}

/// `key = value`.
struct KeyValueSyntaxParser {
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
    pub fn parse(&mut self, token_line: &TokenLine, token: &Token) {
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
                            .str("token_line", &format!("{:?}", token_line))
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
                    p.parse(token_line, token);
                } else {
                    panic!(Log::fatal_t(
                        "",
                        Table::default()
                            .str("state", &format!("{:?}", self.state))
                            .str("token_line", &format!("{:?}", token_line))
                            .str("token", &format!("{:?}", token))
                    ));
                }
            }
        }
    }
}
impl fmt::Debug for KeyValueSyntaxParser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "state=|{:?}|
key=|{}|
right_value_buf=|{:?}|
{}",
            self.state,
            self.key,
            self.right_value_buf,
            if let Some(inline_table_syntax_parser) = &self.inline_table_syntax_parser {
                format!(
                    "{:?}
",
                    inline_table_syntax_parser
                )
            } else {
                "".to_string()
            }
        )
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
struct InlineTableSyntaxParser {
    contents: TokenLine,
}
impl Default for InlineTableSyntaxParser {
    fn default() -> Self {
        InlineTableSyntaxParser {
            contents: TokenLine::default(),
        }
    }
}
impl InlineTableSyntaxParser {
    pub fn parse(&mut self, token_line: &TokenLine, token: &Token) {
        self.contents.tokens.push(token.clone());
    }
}
impl fmt::Debug for InlineTableSyntaxParser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "contents=|{:?}|", self.contents)
    }
}
