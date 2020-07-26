use crate::lexical_parser::TokenLine;
use crate::object_model::document::DocumentM;
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
    pub fn scan_line(&mut self, token_line: &TokenLine, dom: &mut DocumentM) -> SyntaxParserResult {
        for (i, token) in token_line.tokens.iter().enumerate() {
            match self.line_parser.parse(token, dom) {
                SyntaxParserResult::Ok(_) => {}
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

        // TODO 行末が終端子のケース。
        match self.line_parser.eol() {
            SyntaxParserResult::Ok(end_of_syntax) => {
                if end_of_syntax {
                    dom.push_line(&self.line_parser.product())
                }
            }
            SyntaxParserResult::Err(table) => {
                return SyntaxParserResult::Err(
                    Table::default()
                        .str("token_line", &format!("{:?}", token_line))
                        .sub_t("error", &table)
                        .clone(),
                );
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
