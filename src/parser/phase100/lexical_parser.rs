//! Combines only consecutive whitespace into one.  
//! 連続する空白のみを1つに結合します。  
use crate::model::layer110::{Character, CharacterLine, CharacterType};
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
    product: CharacterLine,
    buffer_character_column_number: usize,
    buffer_character_type: Option<CharacterType>,
    buffer_string: String,
}
impl LexicalParser {
    pub fn new(row_number: usize) -> Self {
        LexicalParser {
            state: State::First,
            product: CharacterLine::new(row_number),
            buffer_character_column_number: 0,
            buffer_character_type: None,
            buffer_string: String::new(),
        }
    }
    /// Flush.
    fn flush(&mut self) {
        if !self.buffer_string.is_empty() {
            self.product.characters.push(Character::new(
                self.buffer_character_column_number,
                &self.buffer_string,
                self.buffer_character_type.unwrap(),
            ));
            self.buffer_string.clear();
        }
    }
    pub fn product(&self) -> &CharacterLine {
        &self.product
    }
    pub fn parse_line(&mut self, line: &str) {
        let ch_vec: Vec<char> = line.chars().collect();
        self.buffer_character_column_number = 0;
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
        self.product.characters.push(Character::new(
            ch_vec.len(),
            "
",
            CharacterType::Newline,
        ));
    }
    fn one_delay_loop(&mut self, i: usize, chars: (Option<&char>, Option<&char>)) {
        let ch0 = chars.0.unwrap();
        match self.state {
            /*
            // `\` の次に連なる文字列は、先頭1文字でトークンを切ります。
            State::EscapeSequenceCharacter => {
                // print!("[trace101 AlbetChar={:?}]", ch0);
                self.buffer_character_column_number = i;
                self.buffer_character_type = TokenType::Alpha;
                self.buffer_string.push(*ch0);
                self.flush();
                self.state = State::First;
            }
            */
            State::First => {
                self.buffer_character_column_number = i;
                self.buffer_string.push(*ch0);
                match ch0 {
                    // A ～ Z, a ～ z.
                    'A'..='Z' | 'a'..='z' => {
                        // print!("[trace105 albet={:?}]", ch0);
                        self.buffer_character_type = Some(CharacterType::Alpha);
                        self.flush();
                    }
                    // \
                    '\\' => {
                        // print!("[trace104 bs={:?}]", ch0);
                        self.buffer_character_type = Some(CharacterType::Backslash);
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
                        self.buffer_character_type = Some(CharacterType::Colon);
                        self.flush();
                    }
                    // ,
                    ',' => {
                        self.buffer_character_type = Some(CharacterType::Comma);
                        self.flush();
                    }
                    // #
                    '#' => {
                        self.buffer_character_type = Some(CharacterType::CommentStartSymbol);
                        self.flush();
                    }
                    '0'..='9' => {
                        self.buffer_character_type = Some(CharacterType::Digit);
                        self.flush();
                    }
                    // .
                    '.' => {
                        self.buffer_character_type = Some(CharacterType::Dot);
                        self.flush();
                    }
                    // "
                    '"' => {
                        self.buffer_character_type = Some(CharacterType::DoubleQuotation);
                        self.flush();
                    }
                    // =
                    '=' => {
                        self.buffer_character_type = Some(CharacterType::Equals);
                        self.flush();
                    }
                    // Horizontal tab.
                    '\t' => {
                        self.buffer_character_type = Some(CharacterType::HorizontalTab);
                        self.flush();
                    }
                    // -
                    '-' => {
                        self.buffer_character_type = Some(CharacterType::Hyphen);
                        self.flush();
                    }
                    // {
                    '{' => {
                        self.buffer_character_type = Some(CharacterType::LeftCurlyBracket);
                        self.flush();
                    }
                    // [
                    '[' => {
                        self.buffer_character_type = Some(CharacterType::LeftSquareBracket);
                        self.flush();
                    }
                    // +
                    '+' => {
                        self.buffer_character_type = Some(CharacterType::Plus);
                        self.flush();
                    }
                    // }
                    '}' => {
                        self.buffer_character_type = Some(CharacterType::RightCurlyBracket);
                        self.flush();
                    }
                    // ]
                    ']' => {
                        self.buffer_character_type = Some(CharacterType::RightSquareBracket);
                        self.flush();
                    }
                    // '
                    '\'' => {
                        self.buffer_character_type = Some(CharacterType::SingleQuotation);
                        self.flush();
                    }
                    // Space.
                    ' ' => {
                        self.buffer_character_type = Some(CharacterType::Space);
                        self.flush();
                    }
                    // _
                    '_' => {
                        self.buffer_character_type = Some(CharacterType::Underscore);
                        self.flush();
                    }
                    _ => {
                        self.buffer_character_type = Some(CharacterType::NonAscii);
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
