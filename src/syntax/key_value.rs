//! Syntax parser.
//! 構文パーサー。

use crate::lexical_parser::{Token, TokenType};
use crate::object_model::{key_value::KeyValueM, literal_string::LiteralStringM, value::ValueM};
use crate::syntax::array::ArrayP;
use crate::syntax::inline_table::InlineTableP;
use crate::syntax::single_quoted_string::SingleQuotedStringP;
use crate::syntax::SyntaxParserResult;
use casual_logger::Table;

/// `key = value`.
pub struct KeyValueP {
    state: MachineState,
    product: KeyValueM,
    inline_table_p: Option<InlineTableP>,
    single_quoted_string_p: Option<SingleQuotedStringP>,
    array_p: Option<ArrayP>,
}
impl KeyValueP {
    pub fn new(key: &Token) -> Self {
        KeyValueP {
            state: MachineState::AfterKey,
            product: KeyValueM::new(key),
            inline_table_p: None,
            single_quoted_string_p: None,
            array_p: None,
        }
    }
    pub fn product(&self) -> KeyValueM {
        self.product.clone()
    }
    pub fn eol(&self) -> SyntaxParserResult {
        SyntaxParserResult::Ok(false)
    }
    /// # Returns
    ///
    /// * `SyntaxParserResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token) -> SyntaxParserResult {
        match self.state {
            MachineState::AfterKey => {
                match token.type_ {
                    TokenType::WhiteSpace => {} //Ignored it.
                    TokenType::Equals => {
                        self.state = MachineState::AfterEquals;
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
            MachineState::AfterEquals => {
                // key_value_syntax.parse(token_line, token);
                match token.type_ {
                    TokenType::WhiteSpace => {} //Ignored it.
                    TokenType::Key => {
                        // TODO true, false
                        self.product
                            .set_value(&ValueM::LiteralString(LiteralStringM::new(&token)));
                        self.state = MachineState::End;
                        return SyntaxParserResult::Ok(true);
                    }
                    TokenType::LeftCurlyBracket => {
                        self.inline_table_p = Some(InlineTableP::default());
                        self.state = MachineState::AfterLeftCurlyBracket;
                    }
                    TokenType::LeftSquareBracket => {
                        self.array_p = Some(ArrayP::default());
                        self.state = MachineState::AfterLeftSquareBracket;
                    }
                    TokenType::SingleQuotation => {
                        self.single_quoted_string_p = Some(SingleQuotedStringP::new());
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
            MachineState::AfterLeftCurlyBracket => {
                let p = self.inline_table_p.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::Ok(end_of_syntax) => {
                        if end_of_syntax {
                            self.product.set_value(&ValueM::InlineTable(p.product()));
                            self.inline_table_p = None;
                            self.state = MachineState::End;
                            return SyntaxParserResult::Ok(true);
                        }
                    }
                    SyntaxParserResult::Err(table) => {
                        return SyntaxParserResult::Err(
                            self.err_table()
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                }
            }
            MachineState::AfterLeftSquareBracket => {
                let p = self.array_p.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::Ok(end_of_syntax) => {
                        if end_of_syntax {
                            self.product.set_value(&ValueM::Array(p.product()));
                            self.array_p = None;
                            self.state = MachineState::End;
                            return SyntaxParserResult::Ok(true);
                        }
                    }
                    SyntaxParserResult::Err(table) => {
                        return SyntaxParserResult::Err(
                            self.err_table()
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                }
            }
            MachineState::SingleQuotedString => {
                let p = self.single_quoted_string_p.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::Ok(end_of_syntax) => {
                        if end_of_syntax {
                            self.product
                                .set_value(&ValueM::SingleQuotedString(p.product()));
                            self.single_quoted_string_p = None;
                            self.state = MachineState::End;
                            return SyntaxParserResult::Ok(true);
                        }
                    }
                    SyntaxParserResult::Err(table) => {
                        return SyntaxParserResult::Err(
                            self.err_table()
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                }
            }
            MachineState::End => {
                return SyntaxParserResult::Err(
                    self.err_table()
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
        }
        SyntaxParserResult::Ok(false)
    }
    pub fn err_table(&self) -> Table {
        let mut t = Table::default()
            .str("parser", "KeyValueP#parse")
            .str("state", &format!("{:?}", self.state))
            .str("product", &format!("{:?}", &self.product))
            .clone();
        if let Some(inline_table_p) = &self.inline_table_p {
            t.sub_t("inline_table", &inline_table_p.err_table());
        }
        if let Some(single_quoted_string_p) = &self.single_quoted_string_p {
            t.sub_t("single_quoted_string", &single_quoted_string_p.err_table());
        }
        t
    }
}

/// `key = right_value`.
#[derive(Debug)]
enum MachineState {
    AfterKey,
    AfterEquals,
    AfterLeftCurlyBracket,
    AfterLeftSquareBracket,
    SingleQuotedString,
    End,
}
