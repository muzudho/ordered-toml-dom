use crate::lexical_parser::TokenLine;
use crate::syntax::line::LineSyntaxParser;
use casual_logger::Table;

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
            self.line_syntax_parser.parse(token);
        }
    }

    pub fn log(&self) -> Table {
        let mut t = Table::default();
        t.sub_t("line", &self.line_syntax_parser.log());
        t
    }
}
