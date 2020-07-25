use crate::lexical_parser::Token;
use casual_logger::Table;

/// `# comment`.
pub struct CommentSyntaxParser {
    value: String,
}
impl CommentSyntaxParser {
    pub fn new() -> Self {
        CommentSyntaxParser {
            value: String::new(),
        }
    }
    pub fn parse(&mut self, token: &Token) {
        self.value.push_str(&token.value);
    }
    pub fn log(&self) -> Table {
        Table::default().str("value", &self.value).clone()
    }
}
