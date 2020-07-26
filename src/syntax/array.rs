//! Syntax parser.
//! 構文パーサー。

use crate::object_model::{array::ArrayM, literal_string::LiteralStringM};
use crate::syntax::single_quoted_string::SingleQuotedStringP;
use crate::syntax::SyntaxParserResult;
use crate::token::{Token, TokenType};
use casual_logger::{Log, Table};

/// `[ 'a', 'b', 'c' ]`.
#[derive(Clone)]
pub struct ArrayP {
    state: MachineState,
    product: ArrayM,
    single_quoted_string_p: Option<Box<SingleQuotedStringP>>,
}
impl Default for ArrayP {
    fn default() -> Self {
        ArrayP {
            state: MachineState::AfterLeftSquareBracket,
            product: ArrayM::default(),
            single_quoted_string_p: None,
        }
    }
}
impl ArrayP {
    pub fn product(&self) -> &ArrayM {
        &self.product
    }
    /// # Returns
    ///
    /// * `SyntaxParserResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token) -> SyntaxParserResult {
        match self.state {
            MachineState::AfterLeftSquareBracket => {
                match token.type_ {
                    TokenType::WhiteSpace => {
                        Log::trace_t(
                            "ArrayP#parse/After[/WhiteSpace",
                            Table::default().str("token", &format!("{:?}", token)),
                        );
                    } // Ignore it.
                    TokenType::Key => {
                        // TODO 数字なら正しいが、リテラル文字列だと間違い。
                        self.product
                            .push_literal_string(&LiteralStringM::new(token));
                        self.state = MachineState::AfterItem;
                        Log::trace_t(
                            "ArrayP#parse/After[/Key",
                            Table::default()
                                .str("token", &format!("{:?}", token))
                                .str("product", &format!("{:?}", self.product())),
                        );
                    }
                    TokenType::SingleQuotation => {
                        Log::trace_t(
                            "ArrayP#parse/After[/'",
                            Table::default().str("token", &format!("{:?}", token)),
                        );
                        self.single_quoted_string_p = Some(Box::new(SingleQuotedStringP::new()));
                        self.state = MachineState::SingleQuotedString;
                    }
                    _ => {
                        return SyntaxParserResult::Err(
                            self.err_table()
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        )
                    }
                }
            }
            MachineState::SingleQuotedString => {
                Log::trace_t(
                    "ArrayP#parse/'value'",
                    Table::default().str("token", &format!("{:?}", token)),
                );
                let p = self.single_quoted_string_p.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::End => {
                        self.product.push_single_quote_string(&p.product());
                        self.single_quoted_string_p = None;
                        self.state = MachineState::AfterSingleQuotedString;
                    }
                    SyntaxParserResult::Err(table) => {
                        return SyntaxParserResult::Err(
                            self.err_table()
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                    SyntaxParserResult::Ongoing => {}
                }
            }
            MachineState::AfterItem => {
                Log::trace_t(
                    "ArrayP#parse/After*",
                    Table::default().str("token", &format!("{:?}", token)),
                );
                match token.type_ {
                    TokenType::Comma => {
                        self.state = MachineState::AfterLeftSquareBracket;
                    }
                    TokenType::RightSquareBracket => {
                        self.state = MachineState::End;
                        return SyntaxParserResult::End;
                    }
                    _ => {
                        return SyntaxParserResult::Err(
                            self.err_table()
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        )
                    }
                }
            }
            MachineState::AfterSingleQuotedString => {
                Log::trace_t(
                    "ArrayP#parse/After'value'",
                    Table::default().str("token", &format!("{:?}", token)),
                );
                match token.type_ {
                    TokenType::WhiteSpace => {} // Ignore it.
                    TokenType::Comma => {
                        self.state = MachineState::AfterLeftSquareBracket;
                    }
                    TokenType::RightSquareBracket => {
                        self.state = MachineState::End;
                        return SyntaxParserResult::End;
                    }
                    _ => {
                        return SyntaxParserResult::Err(
                            self.err_table()
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        )
                    }
                }
            }
            MachineState::End => {
                Log::trace_t(
                    "ArrayP#parse/End",
                    Table::default().str("token", &format!("{:?}", token)),
                );
                return SyntaxParserResult::Err(
                    self.err_table()
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
        }
        SyntaxParserResult::Ongoing
    }
    pub fn err_table(&self) -> Table {
        let mut t = Table::default().clone();
        t.str("parser", "ArrayP#parse")
            .str("state", &format!("{:?}", self.state));

        if let Some(p) = &self.single_quoted_string_p {
            t.sub_t("single_quoted_string", &p.err_table());
        }
        t
    }
}

/// `[ 'a', 'b', 'c' ]`.
#[derive(Clone, Debug)]
enum MachineState {
    /// [ か , の次。
    AfterLeftSquareBracket,
    AfterSingleQuotedString,
    /// , か ] を待ちます。
    AfterItem,
    End,
    SingleQuotedString,
}
