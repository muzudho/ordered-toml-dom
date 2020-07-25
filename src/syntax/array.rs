//! Syntax parser.
//! 構文パーサー。

use crate::lexical_parser::{Token, TokenLine, TokenType};
use crate::object_model::array::{ArrayItem, ArrayModel};
use crate::syntax::single_quoted_string::SingleQuotedStringParser;
use crate::syntax::SyntaxParserResult;
use casual_logger::{Log, Table};

/// `[ 'a', 'b', 'c' ]`.
pub struct ArrayParser {
    state: MachineState,
    product: ArrayModel,
    rest: TokenLine,
    single_quoted_string_parser: Option<Box<SingleQuotedStringParser>>,
}
impl Default for ArrayParser {
    fn default() -> Self {
        ArrayParser {
            state: MachineState::AfterLeftSquareBracket,
            product: ArrayModel::default(),
            rest: TokenLine::default(),
            single_quoted_string_parser: None,
        }
    }
}
impl ArrayParser {
    /// # Returns
    ///
    /// * `SyntaxParserResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token) -> SyntaxParserResult {
        match self.state {
            MachineState::AfterLeftSquareBracket => {
                match token.type_ {
                    TokenType::WhiteSpace => {} // Ignore it.
                    TokenType::SingleQuotation => {
                        self.single_quoted_string_parser =
                            Some(Box::new(SingleQuotedStringParser::new()));
                        self.state = MachineState::SingleQuotedString;
                    }
                    _ => {
                        self.rest.tokens.push(token.clone());
                    }
                }
            }
            MachineState::SingleQuotedString => {
                let p = self.single_quoted_string_parser.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::Ok(end_of_syntax) => {
                        if end_of_syntax {
                            self.product
                                .items
                                .push(ArrayItem::String(p.value.to_string()));
                            self.single_quoted_string_parser = None;
                            self.state = MachineState::AfterSingleQuotedString;
                        }
                    }
                    SyntaxParserResult::Err(table) => {
                        return SyntaxParserResult::Err(
                            Table::default()
                                .str("parser", "ArrayParser#parse")
                                .str("state", &format!("{:?}", self.state))
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                }
            }
            MachineState::AfterSingleQuotedString => match token.type_ {
                TokenType::WhiteSpace => {} // Ignore it.
                TokenType::Comma => {
                    self.state = MachineState::AfterLeftSquareBracket;
                }
                TokenType::RightSquareBracket => {
                    return SyntaxParserResult::Ok(true);
                }
                _ => panic!(Log::fatal_t(
                    "ArrayParser#parse/AfterValue",
                    Table::default()
                        .str("parser", "ArrayParser#parse")
                        .str("state", &format!("{:?}", self.state))
                        .str("token", &format!("{:?}", token))
                )),
            },
        }
        SyntaxParserResult::Ok(false)
    }
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();
        if !self.rest.tokens.is_empty() {
            t.str("rest", &format!("{:?}", self.rest));
        }
        if let Some(p) = &self.single_quoted_string_parser {
            t.sub_t("single_quoted_string", &p.log());
        }
        t
    }
}

/// `[ 'a', 'b', 'c' ]`.
#[derive(Debug)]
enum MachineState {
    AfterLeftSquareBracket,
    SingleQuotedString,
    AfterSingleQuotedString,
}
