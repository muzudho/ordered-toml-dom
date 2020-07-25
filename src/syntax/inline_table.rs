use crate::lexical_parser::{Token, TokenLine, TokenType};
use crate::syntax::key_value::KeyValueSyntaxParser;
use casual_logger::Table;

/// `{ key = value, key = value }`.
#[derive(Debug)]
enum InlineTableSyntaxMachineState {
    AfterLeftCurlyBracket,
    AfterKey,
}

/// `{ key = value, key = value }`.
pub struct InlineTableSyntaxParser {
    state: InlineTableSyntaxMachineState,
    contents: TokenLine,
    key_value_syntax_parser: Option<Box<KeyValueSyntaxParser>>,
}
impl Default for InlineTableSyntaxParser {
    fn default() -> Self {
        InlineTableSyntaxParser {
            state: InlineTableSyntaxMachineState::AfterLeftCurlyBracket,
            contents: TokenLine::default(),
            key_value_syntax_parser: None,
        }
    }
}
impl InlineTableSyntaxParser {
    pub fn parse(&mut self, token: &Token) {
        match self.state {
            InlineTableSyntaxMachineState::AfterLeftCurlyBracket => {
                match token.type_ {
                    TokenType::WhiteSpace => {} // Ignore it.
                    TokenType::Key => {
                        self.key_value_syntax_parser =
                            Some(Box::new(KeyValueSyntaxParser::new(&token.value)));
                        self.state = InlineTableSyntaxMachineState::AfterKey;
                    }
                    _ => {
                        self.contents.tokens.push(token.clone());
                    }
                }
            }
            InlineTableSyntaxMachineState::AfterKey => {
                self.contents.tokens.push(token.clone());
                self.key_value_syntax_parser.as_mut().unwrap().parse(token);
            }
        }
    }
    pub fn log(&self) -> Table {
        let mut t = Table::default()
            .str("contents", &format!("{:?}", self.contents))
            .clone();
        if let Some(key_value_syntax_parser) = &self.key_value_syntax_parser {
            t.sub_t("key_value", &key_value_syntax_parser.log());
        }
        t
    }
}
