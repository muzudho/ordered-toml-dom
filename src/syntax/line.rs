use crate::lexical_parser::Token;
use crate::lexical_parser::{TokenLine, TokenType};
use crate::syntax::comment::CommentSyntaxParser;
use crate::syntax::key_value::KeyValueSyntaxParser;
use casual_logger::{Log, Table};

#[derive(Debug)]
enum LineSyntaxParserMachineState {
    /// `# comment`.
    CommentSyntax,
    First,
    /// `key = right_value`.
    KeyPairSyntax,
    Unimplemented,
}

pub struct LineSyntaxParser {
    state: LineSyntaxParserMachineState,
    comment_syntax: Option<CommentSyntaxParser>,
    key_value_syntax: Option<KeyValueSyntaxParser>,
}
impl Default for LineSyntaxParser {
    fn default() -> Self {
        LineSyntaxParser {
            state: LineSyntaxParserMachineState::First,
            comment_syntax: None,
            key_value_syntax: None,
        }
    }
}
impl LineSyntaxParser {
    pub fn parse(&mut self, token_line: &TokenLine, token: &Token) {
        match self.state {
            LineSyntaxParserMachineState::CommentSyntax => {
                self.comment_syntax
                    .as_mut()
                    .unwrap()
                    .parse(token_line, token);
            }
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
                TokenType::Sharp => {
                    self.comment_syntax = Some(CommentSyntaxParser::new());
                    self.state = LineSyntaxParserMachineState::CommentSyntax;
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
    pub fn log(&self) -> Table {
        let mut t = Table::default()
            .str("state", &format!("{:?}", self.state))
            .clone();
        if let Some(comment_syntax) = &self.comment_syntax {
            t.sub_t("comment", &comment_syntax.log());
        }
        if let Some(key_value_syntax) = &self.key_value_syntax {
            t.sub_t("key_value", &key_value_syntax.log());
        }
        t
    }
}
