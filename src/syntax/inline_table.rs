use crate::lexical_parser::{Token, TokenLine, TokenType};
// use crate::syntax::key_value::KeyValueSyntaxParser;
use casual_logger::Table;

/// `{ key = value, key = value }`.
#[derive(Debug)]
enum InlineTableSyntaxMachineState {
    AfterLeftCurlyBracket,
}

/// `{ key = value, key = value }`.
pub struct InlineTableSyntaxParser {
    state: InlineTableSyntaxMachineState,
    contents: TokenLine,
    //key_value_syntax_parser: Option<KeyValueSyntaxParser>,
}
impl Default for InlineTableSyntaxParser {
    fn default() -> Self {
        InlineTableSyntaxParser {
            state: InlineTableSyntaxMachineState::AfterLeftCurlyBracket,
            contents: TokenLine::default(),
            //key_value_syntax_parser: None,
        }
    }
}
impl InlineTableSyntaxParser {
    pub fn parse(&mut self, token_line: &TokenLine, token: &Token) {
        match self.state {
            InlineTableSyntaxMachineState::AfterLeftCurlyBracket => {
                match token.type_ {
                    TokenType::WhiteSpace => {} // Ignore it.
                    TokenType::Key => {}
                    _ => {}
                }
                self.contents.tokens.push(token.clone());
            }
        }
    }
    pub fn log(&self) -> Table {
        Table::default()
            .str("contents", &format!("{:?}", self.contents))
            .clone()
    }
}
