//! Syntax parser.
//! 構文パーサー。

use crate::lexical_parser::{Token, TokenType};
use crate::object_model::{array::ArrayM, literal_string::LiteralStringM, value::ValueM};
use crate::syntax::single_quoted_string::SingleQuotedStringP;
use crate::syntax::SyntaxParserResult;
use casual_logger::{Log, Table};

/// `[ 'a', 'b', 'c' ]`.
pub struct ArrayP {
    state: MachineState,
    product: ArrayM,
    single_quoted_string_parser: Option<Box<SingleQuotedStringP>>,
}
impl Default for ArrayP {
    fn default() -> Self {
        ArrayP {
            state: MachineState::AfterLeftSquareBracket,
            product: ArrayM::default(),
            single_quoted_string_parser: None,
        }
    }
}
impl ArrayP {
    pub fn product(&self) -> ArrayM {
        self.product.clone()
    }
    /// # Returns
    ///
    /// * `SyntaxParserResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token) -> SyntaxParserResult {
        match self.state {
            MachineState::AfterLeftSquareBracket => {
                match token.type_ {
                    TokenType::WhiteSpace => {} // Ignore it.
                    TokenType::Key => {
                        // TODO 数字なら正しいが、リテラル文字列だと間違い。
                        self.product()
                            .items
                            .push(ValueM::LiteralString(LiteralStringM::new(token)));
                        self.state = MachineState::AfterItem;
                    }
                    TokenType::SingleQuotation => {
                        self.single_quoted_string_parser =
                            Some(Box::new(SingleQuotedStringP::new()));
                        self.state = MachineState::SingleQuotedString;
                    }
                    _ => {
                        return SyntaxParserResult::Err(
                            Table::default()
                                .str("parser", "ArrayP#parse")
                                .str("state", &format!("{:?}", self.state))
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        )
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
                                .push(ValueM::SingleQuotedString(p.product()));
                            self.single_quoted_string_parser = None;
                            self.state = MachineState::AfterSingleQuotedString;
                        }
                    }
                    SyntaxParserResult::Err(table) => {
                        return SyntaxParserResult::Err(
                            Table::default()
                                .str("parser", "ArrayP#parse")
                                .str("state", &format!("{:?}", self.state))
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                }
            }
            MachineState::AfterItem => match token.type_ {
                TokenType::Comma => {
                    self.state = MachineState::AfterLeftSquareBracket;
                }
                TokenType::RightSquareBracket => {
                    return SyntaxParserResult::Ok(true);
                }
                _ => {
                    return SyntaxParserResult::Err(
                        Table::default()
                            .str("parser", "ArrayP#parse")
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token))
                            .clone(),
                    )
                }
            },
            MachineState::AfterSingleQuotedString => match token.type_ {
                TokenType::WhiteSpace => {} // Ignore it.
                TokenType::Comma => {
                    self.state = MachineState::AfterLeftSquareBracket;
                }
                TokenType::RightSquareBracket => {
                    return SyntaxParserResult::Ok(true);
                }
                _ => panic!(Log::fatal_t(
                    "ArrayP#parse/AfterValue",
                    Table::default()
                        .str("parser", "ArrayP#parse")
                        .str("state", &format!("{:?}", self.state))
                        .str("token", &format!("{:?}", token))
                )),
            },
        }
        SyntaxParserResult::Ok(false)
    }
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();
        if let Some(p) = &self.single_quoted_string_parser {
            t.sub_t("single_quoted_string", &p.log());
        }
        t
    }
}

/// `[ 'a', 'b', 'c' ]`.
#[derive(Debug)]
enum MachineState {
    /// [ か , の次。
    AfterLeftSquareBracket,
    SingleQuotedString,
    AfterSingleQuotedString,
    /// , か ] を待ちます。
    AfterItem,
}
