//! Combines only consecutive whitespace into one.  
//! 連続する空白のみを1つに結合します。  
use crate::model::layer110::{Token, TokenLine, TokenType};
use std::fmt;

#[derive(Debug)]
enum State {
    // EscapeSequenceCharacter,
    First,
}

/// Lexical parser.  
/// 字句解析器。  
pub struct LexicalParser {
    state: State,
    product: TokenLine,
    buffer_string_token_column_number: usize,
    buffer_string_token_type: TokenType,
    buffer_string: String,
}
impl LexicalParser {
    pub fn new(row_number: usize) -> Self {
        LexicalParser {
            state: State::First,
            product: TokenLine::new(row_number),
            buffer_string_token_column_number: 0,
            buffer_string_token_type: TokenType::Unknown,
            buffer_string: String::new(),
        }
    }
    /// Flush.
    fn flush(&mut self) {
        if !self.buffer_string.is_empty() {
            self.product.tokens.push(Token::new(
                self.buffer_string_token_column_number,
                &self.buffer_string,
                self.buffer_string_token_type,
            ));
            self.buffer_string.clear();
        }
    }
    pub fn product(&self) -> &TokenLine {
        &self.product
    }
    pub fn parse_line(&mut self, line: &str) {
        let ch_vec: Vec<char> = line.chars().collect();
        self.buffer_string_token_column_number = 0;
        let mut j = 0;
        let mut chars: (Option<&char>, Option<&char>) = (None, None);
        for (i, ch) in ch_vec.iter().enumerate() {
            // Shift.
            // The current char is the look-ahead char, and the previous look-ahead char is the current char.
            // ずらします。
            // 現在の文字は先読み文字、前回の先読み文字は今回の文字です。
            chars.0 = chars.1;
            chars.1 = Some(ch);
            if let Some(_) = chars.0 {
                self.one_delay_loop(i - 1, chars);
            }
            j = i;
        }

        // Last 1 char.
        // 最後の１文字。
        chars.0 = chars.1;
        chars.1 = None;
        if let Some(_) = chars.0 {
            self.one_delay_loop(j, chars);
        }

        // Log::info("End of line.");
        self.flush();
        // Append an end of line.
        // 行末を追加します。
        self.product.tokens.push(Token::new(
            ch_vec.len(),
            "
",
            TokenType::Newline,
        ));
    }
    fn one_delay_loop(&mut self, i: usize, chars: (Option<&char>, Option<&char>)) {
        let ch0 = chars.0.unwrap();
        match self.state {
            /*
            // `\` の次に連なる文字列は、先頭1文字でトークンを切ります。
            State::EscapeSequenceCharacter => {
                // print!("[trace101 AlbetChar={:?}]", ch0);
                self.buffer_string_token_column_number = i;
                self.buffer_string_token_type = TokenType::Alpha;
                self.buffer_string.push(*ch0);
                self.flush();
                self.state = State::First;
            }
            */
            State::First => {
                self.buffer_string_token_column_number = i;
                self.buffer_string.push(*ch0);
                match ch0 {
                    // A ～ Z, a ～ z.
                    'A'..='Z' | 'a'..='z' => {
                        // print!("[trace105 albet={:?}]", ch0);
                        self.buffer_string_token_type = TokenType::Alpha;
                        self.flush();
                    }
                    // \
                    '\\' => {
                        // print!("[trace104 bs={:?}]", ch0);
                        self.buffer_string_token_type = TokenType::Backslash;
                        self.flush();
                        /*
                        if let Some(ch1) = chars.1 {
                            match ch1 {
                                'A'..='Z' | 'a'..='z' => {
                                    // print!("[trace103 ahead={:?}]", ch1);
                                    self.state = State::EscapeSequenceCharacter;
                                }
                                _ => {
                                    // print!("[trace102 ahead={:?}]", ch1);
                                }
                            }
                        }
                        */
                    }
                    // :
                    ':' => {
                        self.buffer_string_token_type = TokenType::Colon;
                        self.flush();
                    }
                    // ,
                    ',' => {
                        self.buffer_string_token_type = TokenType::Comma;
                        self.flush();
                    }
                    // .
                    '.' => {
                        self.buffer_string_token_type = TokenType::Dot;
                        self.flush();
                    }
                    // "
                    '"' => {
                        self.buffer_string_token_type = TokenType::DoubleQuotation;
                        self.flush();
                    }
                    // =
                    '=' => {
                        self.buffer_string_token_type = TokenType::Equals;
                        self.flush();
                    }
                    // -
                    '-' => {
                        self.buffer_string_token_type = TokenType::Hyphen;
                        self.flush();
                    }
                    // {
                    '{' => {
                        self.buffer_string_token_type = TokenType::LeftCurlyBracket;
                        self.flush();
                    }
                    // [
                    '[' => {
                        self.buffer_string_token_type = TokenType::LeftSquareBracket;
                        self.flush();
                    }
                    '0'..='9' => {
                        self.buffer_string_token_type = TokenType::Digit;
                        self.flush();
                    }
                    // +
                    '+' => {
                        self.buffer_string_token_type = TokenType::Plus;
                        self.flush();
                    }
                    // }
                    '}' => {
                        self.buffer_string_token_type = TokenType::RightCurlyBracket;
                        self.flush();
                    }
                    // ]
                    ']' => {
                        self.buffer_string_token_type = TokenType::RightSquareBracket;
                        self.flush();
                    }
                    // #
                    '#' => {
                        self.buffer_string_token_type = TokenType::CommentStartSymbol;
                        self.flush();
                    }
                    // '
                    '\'' => {
                        self.buffer_string_token_type = TokenType::SingleQuotation;
                        self.flush();
                    }
                    // _
                    '_' => {
                        self.buffer_string_token_type = TokenType::Underscore;
                        self.flush();
                    }
                    // Whitespace char.
                    '\t' | ' ' => {
                        self.buffer_string_token_type = TokenType::Wschar;
                        self.flush();
                    }
                    _ => {
                        self.buffer_string_token_type = TokenType::Unknown;
                        self.flush();
                    }
                }
            }
        }
    }
}
impl fmt::Display for LexicalParser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.product)
    }
}
impl fmt::Debug for LexicalParser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.product)
    }
}
