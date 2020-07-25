//! Syntax parser.
//! 構文パーサー。

use crate::lexical_parser::Token;
use crate::lexical_parser::{TokenLine, TokenType};
use crate::syntax::inline_table::InlineTableParser;
use crate::syntax::single_quoted_string::SingleQuotedStringParser;
use crate::syntax::SyntaxParserResult;
use casual_logger::{Log, Table};

/// `key = value`.
pub struct KeyValueParser {
    state: MachineState,
    key: String,
    rest: TokenLine,
    inline_table_parser: Option<InlineTableParser>,
    single_quoted_string_parser: Option<SingleQuotedStringParser>,
}
impl KeyValueParser {
    pub fn new(key: &str) -> Self {
        KeyValueParser {
            state: MachineState::AfterKey,
            key: key.to_string(),
            rest: TokenLine::default(),
            inline_table_parser: None,
            single_quoted_string_parser: None,
        }
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
                        self.inline_table_parser = Some(InlineTableParser::default());
                        self.state = MachineState::AfterLeftCurlyBracket;
                    }
                    TokenType::SingleQuotation => {
                        self.single_quoted_string_parser = Some(SingleQuotedStringParser::new());
                        self.state = MachineState::SingleQuotedString;
                    }
                    _ => {
                        self.rest.tokens.push(token.clone());
                    }
                }
            }
            MachineState::AfterLeftCurlyBracket => {
                if let Some(p) = &mut self.inline_table_parser {
                    if p.parse(token) {
                        self.inline_table_parser = None;
                        self.state = MachineState::End;
                        return SyntaxParserResult::Ok(true);
                    }
                } else {
                    panic!(Log::fatal_t(
                        "",
                        Table::default()
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token))
                    ));
                }
            }
            MachineState::SingleQuotedString => {
                if let Some(p) = &mut self.single_quoted_string_parser {
                    if p.parse(token) {
                        self.single_quoted_string_parser = None;
                        self.state = MachineState::End;
                        return SyntaxParserResult::Ok(true);
                    }
                } else {
                    panic!(Log::fatal_t(
                        "",
                        Table::default()
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token))
                    ));
                }
            }
            MachineState::End => {
                panic!(Log::fatal_t(
                    "",
                    Table::default()
                        .str("state", &format!("{:?}", self.state))
                        .str("token", &format!("{:?}", token))
                ));
            }
        }
        SyntaxParserResult::Ok(false)
    }
    pub fn log(&self) -> Table {
        let mut t = Table::default()
            .str("state", &format!("{:?}", self.state))
            .str("key", &self.key)
            .clone();
        if !self.rest.tokens.is_empty() {
            t.str("rest", &format!("{:?}", self.rest));
        }
        if let Some(inline_table_parser) = &self.inline_table_parser {
            t.sub_t("inline_table", &inline_table_parser.log());
        }
        if let Some(single_quoted_string_parser) = &self.single_quoted_string_parser {
            t.sub_t("single_quoted_string", &single_quoted_string_parser.log());
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
    SingleQuotedString,
    End,
}
