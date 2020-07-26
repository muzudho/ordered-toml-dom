use crate::object_model::document::DocumentM;
use crate::syntax::line::LineP;
use crate::syntax::SyntaxParserResult;
use crate::token::TokenLine;
use casual_logger::Table;

pub struct LineSyntaxScanner {
    pub line_parser: LineP,
}
impl Default for LineSyntaxScanner {
    fn default() -> Self {
        LineSyntaxScanner {
            line_parser: LineP::default(),
        }
    }
}
impl LineSyntaxScanner {
    /// # Returns
    ///
    /// * `SyntaxParserResult` - Result.  
    ///                             結果。
    pub fn scan_line(&mut self, token_line: &TokenLine, dom: &mut DocumentM) -> SyntaxParserResult {
        for (i, token) in token_line.tokens.iter().enumerate() {
            match self.line_parser.parse(token, dom) {
                SyntaxParserResult::End | SyntaxParserResult::Ongoing => {}
                SyntaxParserResult::Err(table) => {
                    return SyntaxParserResult::Err(
                        Table::default()
                            .str("parser", "syntax_scanner.rs/LineSyntaxScanner#scan_line")
                            .str("token_line", &format!("{:?}", token_line))
                            .str(
                                "rest",
                                &format!("{:?}", TokenLine::new(&token_line.tokens[i..].to_vec())),
                            )
                            .sub_t("error", &table)
                            .clone(),
                    );
                }
            }
        }

        SyntaxParserResult::Ongoing
    }

    pub fn log(&self) -> Table {
        let mut t = Table::default();
        t.sub_t("line", &self.line_parser.err_table());
        t
    }
}
