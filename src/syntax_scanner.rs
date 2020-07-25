use crate::lexical_parser::TokenLine;
use crate::syntax::line::LineP;
use crate::syntax::SyntaxParserResult;
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
    pub fn scan_line(&mut self, token_line: &TokenLine) -> SyntaxParserResult {
        for (i, token) in token_line.tokens.iter().enumerate() {
            match self.line_parser.parse(token) {
                SyntaxParserResult::Ok(_) => {} // Ignored it.
                SyntaxParserResult::Err(table) => {
                    return SyntaxParserResult::Err(
                        Table::default()
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
        SyntaxParserResult::Ok(false)
    }

    pub fn log(&self) -> Table {
        let mut t = Table::default();
        t.sub_t("line", &self.line_parser.log());
        t
    }
}
