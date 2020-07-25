//! Syntax parser.
//! 構文パーサー。

use crate::lexical_parser::{Token, TokenLine, TokenType};
use crate::object_model::{key_value::KeyValueM, value::ValueM};
use crate::syntax::array::ArrayP;
use crate::syntax::inline_table::InlineTableP;
use crate::syntax::single_quoted_string::SingleQuotedStringP;
use crate::syntax::SyntaxParserResult;
use casual_logger::Table;

/// `key = value`.
pub struct KeyValueP {
    state: MachineState,
    product: KeyValueM,
    rest: TokenLine,
    inline_table_p: Option<InlineTableP>,
    single_quoted_string_p: Option<SingleQuotedStringP>,
    array_p: Option<ArrayP>,
}
impl KeyValueP {
    pub fn new(key: &str) -> Self {
        KeyValueP {
            state: MachineState::AfterKey,
            product: KeyValueM::new(key),
            rest: TokenLine::default(),
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
                            Table::default()
                                .str("parser", "KeyValueP#parse")
                                .str("state", &format!("{:?}", self.state))
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
                        self.rest.tokens.push(token.clone());
                    }
                }
            }
            MachineState::AfterLeftCurlyBracket => {
                if let Some(p) = &mut self.inline_table_p {
                    match p.parse(token) {
                        SyntaxParserResult::Ok(end_of_syntax) => {
                            if end_of_syntax {
                                self.product.value =
                                    Some(Box::new(ValueM::InlineTable(p.product())));
                                self.inline_table_p = None;
                                self.state = MachineState::End;
                                return SyntaxParserResult::Ok(true);
                            }
                        }
                        SyntaxParserResult::Err(table) => {
                            return SyntaxParserResult::Err(
                                Table::default()
                                    .str("parser", "KeyValueP#parse")
                                    .str("state", &format!("{:?}", self.state))
                                    .str("token", &format!("{:?}", token))
                                    .sub_t("error", &table)
                                    .clone(),
                            )
                        }
                    }
                } else {
                    return SyntaxParserResult::Err(
                        Table::default()
                            .str("parser", "KeyValueP#parse")
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token))
                            .clone(),
                    );
                }
            }
            MachineState::AfterLeftSquareBracket => {
                if let Some(p) = &mut self.array_p {
                    match p.parse(token) {
                        SyntaxParserResult::Ok(end_of_syntax) => {
                            if end_of_syntax {
                                self.product.value = Some(Box::new(ValueM::Array(p.product())));
                                self.array_p = None;
                                self.state = MachineState::End;
                                return SyntaxParserResult::Ok(true);
                            }
                        }
                        SyntaxParserResult::Err(table) => {
                            return SyntaxParserResult::Err(
                                Table::default()
                                    .str("parser", "KeyValueP#parse")
                                    .str("state", &format!("{:?}", self.state))
                                    .str("token", &format!("{:?}", token))
                                    .sub_t("error", &table)
                                    .clone(),
                            )
                        }
                    }
                } else {
                    return SyntaxParserResult::Err(
                        Table::default()
                            .str("parser", "KeyValueP#parse")
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token))
                            .clone(),
                    );
                }
            }
            MachineState::SingleQuotedString => {
                if let Some(p) = &mut self.single_quoted_string_p {
                    match p.parse(token) {
                        SyntaxParserResult::Ok(end_of_syntax) => {
                            if end_of_syntax {
                                self.product.value =
                                    Some(Box::new(ValueM::SingleQuotedString(p.product())));
                                self.single_quoted_string_p = None;
                                self.state = MachineState::End;
                                return SyntaxParserResult::Ok(true);
                            }
                        }
                        SyntaxParserResult::Err(table) => {
                            return SyntaxParserResult::Err(
                                Table::default()
                                    .str("parser", "KeyValueP#parse")
                                    .str("state", &format!("{:?}", self.state))
                                    .str("token", &format!("{:?}", token))
                                    .sub_t("error", &table)
                                    .clone(),
                            )
                        }
                    }
                } else {
                    return SyntaxParserResult::Err(
                        Table::default()
                            .str("parser", "KeyValueP#parse")
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token))
                            .clone(),
                    );
                }
            }
            MachineState::End => {
                return SyntaxParserResult::Err(
                    Table::default()
                        .str("parser", "KeyValueP#parse")
                        .str("state", &format!("{:?}", self.state))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
        }
        SyntaxParserResult::Ok(false)
    }
    pub fn log(&self) -> Table {
        let mut t = Table::default()
            .str("state", &format!("{:?}", self.state))
            .str("key", &format!("{:?}", &self.product))
            .clone();
        if !self.rest.tokens.is_empty() {
            t.str("rest", &format!("{:?}", self.rest));
        }
        if let Some(inline_table_p) = &self.inline_table_p {
            t.sub_t("inline_table", &inline_table_p.log());
        }
        if let Some(single_quoted_string_p) = &self.single_quoted_string_p {
            t.sub_t("single_quoted_string", &single_quoted_string_p.log());
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
