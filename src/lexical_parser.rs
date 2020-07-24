//! 単語単位に分けます。
use crate::RE_KEY;
use std::fmt;

#[derive(Debug)]
enum LineMachineState {
    Key,
    /// Whitespace means tab (0x09 '\t') or space (0x20 ' ').
    WhiteSpace,
}

/// WIP.
/// Line parser.
/// 行パーサー。
pub struct LineParser {
    state: Option<LineMachineState>,
    tokens: Vec<Token>,
    buf_token_type: TokenType,
    buf: String,
}
impl Default for LineParser {
    fn default() -> Self {
        LineParser {
            state: None,
            tokens: Vec::new(),
            buf_token_type: TokenType::WhiteSpace,
            buf: String::new(),
        }
    }
}
impl LineParser {
    pub fn parse_line(&mut self, line: &str) {
        println!("parse_line=|{}|", line);

        let ch_vec: Vec<char> = line.chars().collect();
        for ch in ch_vec {
            println!("parse_line/ch=|{}|", ch);
            if let Some(state) = &self.state {
                println!("parse_line/{:?}/ch=|{}|", state, ch);
                match state {
                    LineMachineState::WhiteSpace => {
                        // 最初に出てくる文字まで飛ばします。
                        match ch {
                            '\t' | ' ' => {
                                self.buf.push(ch);
                                println!("{:?}=|{}|", state, ch);
                            }
                            _ => {
                                self.flush();
                                self.initial(ch);
                            }
                        }
                    }
                    LineMachineState::Key => {
                        let matched = match RE_KEY.lock() {
                            Ok(re_key) => re_key.is_match(&ch.to_string()),
                            Err(why) => {
                                panic!("{}", why);
                            }
                        };
                        if matched {
                            // A key.
                            self.buf.push(ch);
                            println!("{:?}=|{}|", state, ch);
                        } else {
                            self.flush();
                            self.initial(ch);
                        }
                    }
                }
            } else {
                println!("parse_line/None/ch=|{}|", ch);
                self.flush();
                self.initial(ch);
            }
            println!("End of parse_line/ch=|{}|", ch);
        }
        println!("End of line.");
        self.flush();
    }
    /// Flush.
    fn flush(&mut self) {
        if !self.buf.is_empty() {
            self.tokens.push(Token::new(&self.buf, self.buf_token_type));
            println!("Flush=|{}|", self.buf);
            self.buf.clear();
            self.state = None;
        }
    }
    /// 最初の文字
    fn initial(&mut self, ch: char) {
        self.buf.push(ch);
        println!("Begin of initial=|{}|", ch);
        match ch {
            '\t' | ' ' => {
                self.buf_token_type = TokenType::WhiteSpace;
                println!("initial/{:?}=|{}|", self.buf_token_type, ch);
                self.state = Some(LineMachineState::WhiteSpace);
            }
            ',' => {
                self.buf_token_type = TokenType::Comma;
                println!("initial/{:?}=|{}|", self.buf_token_type, ch);
            }
            '=' => {
                self.buf_token_type = TokenType::Equals;
                println!("initial/{:?}=|{}|", self.buf_token_type, ch);
            }
            '{' => {
                self.buf_token_type = TokenType::LeftCurlyBracket;
                println!("initial/{:?}=|{}|", self.buf_token_type, ch);
            }
            '}' => {
                self.buf_token_type = TokenType::RightCurlyBracket;
                println!("initial/{:?}=|{}|", self.buf_token_type, ch);
            }
            '\'' => {
                self.buf_token_type = TokenType::SingleQuotation;
                println!("initial/{:?}=|{}|", self.buf_token_type, ch);
            }
            _ => {
                let matched = match RE_KEY.lock() {
                    Ok(re_key) => re_key.is_match(&ch.to_string()),
                    Err(why) => panic!("{}", why),
                };
                if matched {
                    // A key.
                    self.buf_token_type = TokenType::Key;
                    println!("initial/{:?}=|{}|", self.buf_token_type, ch);
                    self.state = Some(LineMachineState::Key);
                } else {
                    self.buf_token_type = TokenType::Unimplemented;
                    println!("initial/{:?}=|{}|", self.buf_token_type, ch);
                }
            }
        }
        println!("End of initial=|{}|", ch);
    }
}
impl fmt::Debug for LineParser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&format!("{:?}", token));
        }
        write!(f, "{}", buf)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    /// }
    CloseCurlyBracket,
    /// ,
    Comma,
    Equals,
    Key,
    /// {
    LeftCurlyBracket,
    /// }
    RightCurlyBracket,
    /// '
    SingleQuotation,
    Unimplemented,
    /// Whitespace means tab (0x09 '\t') or space (0x20 ' ').
    WhiteSpace,
}

pub struct Token {
    value: String,
    type_: TokenType,
}
impl Token {
    pub fn new(value: &str, type_: TokenType) -> Self {
        Token {
            value: value.to_string(),
            type_: type_,
        }
    }
}
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "|{}|{:?}", self.value, self.type_)
    }
}
