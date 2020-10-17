//! Divide into words.  
//! 単語に分けます。  
use crate::model::layer110::{Token, TokenLine, TokenType};
use crate::RE_KEY;
use casual_logger::Log;
use std::fmt;

#[derive(Debug)]
enum LineMachineState {
    Key,
    /// Whitespace means tab ('\t' 0x09) or space (' ' 0x20).  
    /// ホワイトスペースは タブ ('\t', 0x09) と 半角スペース (' ' 0x20) です。  
    WhiteSpace,
}

/// Lexical parser.  
/// 字句解析器。  
pub struct LexicalParser {
    state: Option<LineMachineState>,
    product: TokenLine,
    buf_token_type: TokenType,
    buf: String,
}
impl LexicalParser {
    pub fn new(row_number: usize) -> Self {
        LexicalParser {
            state: None,
            product: TokenLine::new(row_number),
            buf_token_type: TokenType::WhiteSpace,
            buf: String::new(),
        }
    }
    pub fn product(&self) -> &TokenLine {
        &self.product
    }
    pub fn parse_line(&mut self, line: &str) {
        let ch_vec: Vec<char> = line.chars().collect();
        let mut initial_column_number = 0;
        for (i, ch) in ch_vec.iter().enumerate() {
            let column_number = i + 1;
            if let Some(state) = &self.state {
                match state {
                    LineMachineState::WhiteSpace => {
                        // 最初に出てくる文字まで飛ばします。
                        match ch {
                            '\t' | ' ' => {
                                self.buf.push(*ch);
                            }
                            _ => {
                                self.flush(initial_column_number);
                                self.initial(*ch);
                                initial_column_number = column_number;
                            }
                        }
                    }
                    LineMachineState::Key => {
                        let matched = match RE_KEY.lock() {
                            Ok(re_key) => re_key.is_match(&ch.to_string()),
                            Err(why) => panic!(Log::fatal(&format!("{}", why))),
                        };
                        if matched {
                            // A key.
                            self.buf.push(*ch);
                        } else {
                            self.flush(initial_column_number);
                            self.initial(*ch);
                            initial_column_number = column_number;
                        }
                    }
                }
            } else {
                self.flush(initial_column_number);
                self.initial(*ch);
                initial_column_number = column_number;
            }
        }
        // Log::info("End of line.");
        self.flush(initial_column_number);
        self.product.tokens.push(Token::new(
            ch_vec.len(),
            "
",
            TokenType::EndOfLine,
        ));
    }
    /// Flush.
    fn flush(&mut self, column_number: usize) {
        if !self.buf.is_empty() {
            self.product
                .tokens
                .push(Token::new(column_number, &self.buf, self.buf_token_type));
            // Log::info_t("Flush", Table::default().str("buf", &self.buf));
            self.buf.clear();
            self.state = None;
        }
    }
    /// Character at first.  
    /// 最初の文字。  
    fn initial(&mut self, ch: char) {
        self.buf.push(ch);
        match ch {
            // A ～ Z, a ～ z.
            'A'..='Z' | 'a'..='z' => {
                self.buf_token_type = TokenType::Alphabet;
                self.state = Some(LineMachineState::Key);
            }
            // \
            '\\' => {
                self.buf_token_type = TokenType::Backslash;
            }
            // :
            ':' => {
                self.buf_token_type = TokenType::Colon;
            }
            // ,
            ',' => {
                self.buf_token_type = TokenType::Comma;
            }
            // .
            '.' => {
                self.buf_token_type = TokenType::Dot;
            }
            // "
            '"' => {
                self.buf_token_type = TokenType::DoubleQuotation;
            }
            // =
            '=' => {
                self.buf_token_type = TokenType::Equals;
            }
            // -
            '-' => {
                self.buf_token_type = TokenType::Hyphen;
            }
            // {
            '{' => {
                self.buf_token_type = TokenType::LeftCurlyBracket;
            }
            // [
            '[' => {
                self.buf_token_type = TokenType::LeftSquareBracket;
            }
            '0'..='9' => {
                self.buf_token_type = TokenType::Numeral;
            }
            // +
            '+' => {
                self.buf_token_type = TokenType::Plus;
            }
            // }
            '}' => {
                self.buf_token_type = TokenType::RightCurlyBracket;
            }
            // ]
            ']' => {
                self.buf_token_type = TokenType::RightSquareBracket;
            }
            // #
            '#' => {
                self.buf_token_type = TokenType::Sharp;
            }
            // '
            '\'' => {
                self.buf_token_type = TokenType::SingleQuotation;
            }
            // _
            '_' => {
                self.buf_token_type = TokenType::Underscore;
            }
            // Whitespace.
            '\t' | ' ' => {
                self.buf_token_type = TokenType::WhiteSpace;
                self.state = Some(LineMachineState::WhiteSpace);
            }
            _ => {
                self.buf_token_type = TokenType::OtherwiseExceptNumeralHyphenUnderscore;
            }
        }
    }
}
impl fmt::Debug for LexicalParser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.product)
    }
}
