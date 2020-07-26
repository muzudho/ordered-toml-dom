use crate::model::document::DocumentM;
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
            match self.line_parser.parse(token) {
                SyntaxParserResult::End => {
                    if let Some(child_p) = self.line_parser.flush() {
                        dom.push_line(&child_p);
                    } else {
                        return SyntaxParserResult::Err(
                            self.err_table()
                                .str("token_line", &format!("{:?}", token_line))
                                .str(
                                    "rest",
                                    &format!(
                                        "{:?}",
                                        TokenLine::new(&token_line.tokens[i..].to_vec())
                                    ),
                                )
                                .clone(),
                        );
                    }
                }
                SyntaxParserResult::Err(table) => {
                    return SyntaxParserResult::Err(
                        self.err_table()
                            .str("token_line", &format!("{:?}", token_line))
                            .sub_t("error", &table)
                            .clone(),
                    );
                }
                SyntaxParserResult::Ongoing => {}
            }
        }

        SyntaxParserResult::Ongoing
    }

    pub fn err_table(&self) -> Table {
        let mut t = Table::default();
        t.str("parser", "syntax_scanner.rs/LineSyntaxScanner#scan_line")
            .sub_t("line", &self.line_parser.err_table());
        t
    }
}
