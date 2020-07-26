//! Syntax parser.
//! 構文パーサー。

use crate::lexical_parser::{Token, TokenType};
use crate::object_model::{array::ArrayM, literal_string::LiteralStringM};
use crate::syntax::single_quoted_string::SingleQuotedStringP;
use crate::syntax::SyntaxParserResult;
use casual_logger::Table;

/// `[ 'a', 'b', 'c' ]`.
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
                            .push_literal_string(&LiteralStringM::new(token));
                        self.state = MachineState::AfterItem;
                    }
                    TokenType::SingleQuotation => {
                        self.single_quoted_string_p = Some(Box::new(SingleQuotedStringP::new()));
                        self.state = MachineState::SingleQuotedString;
                    }
                    _ => return SyntaxParserResult::Err(self.err_table(Some(token)).clone()),
                }
            }
            MachineState::SingleQuotedString => {
                let p = self.single_quoted_string_p.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::Ok(end_of_syntax) => {
                        if end_of_syntax {
                            self.product.push_single_quote_string(&p.product());
                            self.single_quoted_string_p = None;
                            self.state = MachineState::AfterSingleQuotedString;
                        }
                    }
                    SyntaxParserResult::Err(table) => {
                        return SyntaxParserResult::Err(
                            self.err_table(Some(token)).sub_t("error", &table).clone(),
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
                _ => return SyntaxParserResult::Err(self.err_table(Some(token)).clone()),
            },
            MachineState::AfterSingleQuotedString => match token.type_ {
                TokenType::WhiteSpace => {} // Ignore it.
                TokenType::Comma => {
                    self.state = MachineState::AfterLeftSquareBracket;
                }
                TokenType::RightSquareBracket => {
                    return SyntaxParserResult::Ok(true);
                }
                _ => return SyntaxParserResult::Err(self.err_table(Some(token)).clone()),
            },
        }
        SyntaxParserResult::Ok(false)
    }
    pub fn err_table(&self, token: Option<&Token>) -> Table {
        let mut t = Table::default().clone();
        t.str("parser", "ArrayP#parse")
            .str("state", &format!("{:?}", self.state));

        if let Some(token) = token {
            t.str("token", &format!("{:?}", token));
        }

        if let Some(p) = &self.single_quoted_string_p {
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
