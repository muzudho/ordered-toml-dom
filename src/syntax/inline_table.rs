use crate::lexical_parser::Token;
use crate::lexical_parser::TokenLine;
use casual_logger::Table;

/*
/// `{ key = value, key = value }`.
#[derive(Debug)]
enum InlineTableSyntaxMachineState {
    AfterLeftCurlyBracket,
}
*/

/// `{ key = value, key = value }`.
pub struct InlineTableSyntaxParser {
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
    pub fn log(&self) -> Table {
        Table::default()
            .str("contents", &format!("{:?}", self.contents))
            .clone()
    }
}
