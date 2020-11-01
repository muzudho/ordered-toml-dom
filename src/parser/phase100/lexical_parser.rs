//! Combines only consecutive whitespace into one.  
//! 連続する空白のみを1つに結合します。  

use crate::model::layer110::{Character, CharacterLine, CharacterType};
use crate::parser::phase100::LexicalParser;
use std::fmt;

#[derive(Debug)]
pub enum State {
    // EscapeSequenceCharacter,
    First,
}

impl LexicalParser {
    pub fn new(row_number: usize) -> Self {
        LexicalParser {
            state: State::First,
            product: CharacterLine::new(row_number),
            buffer_character_column_number: 0,
            buffer_character_type: None,
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

        // Append an end of line.
        // 行末を追加します。
        // TODO とりあえず 改行は \n を入れておく。Windows と Linux で違うが。
        self.product
            .characters
            .push(Character::new(ch_vec.len(), '\n', CharacterType::Newline));
    }
    fn one_delay_loop(&mut self, i: usize, chars: (Option<&char>, Option<&char>)) {
        let ch0 = chars.0.unwrap();
        match self.state {
            State::First => {
                self.buffer_character_column_number = i;
                self.product.characters.push(Character::new(
                    self.buffer_character_column_number,
                    *ch0,
                    self.buffer_character_type.unwrap(),
                ));
                self.buffer_character_type = match ch0 {
                    // A ～ Z, a ～ z.
                    'A'..='Z' | 'a'..='z' => Some(CharacterType::Alpha),
                    // \
                    '\\' => Some(CharacterType::Backslash),
                    // :
                    ':' => Some(CharacterType::Colon),
                    // ,
                    ',' => Some(CharacterType::Comma),
                    // #
                    '#' => Some(CharacterType::CommentStartSymbol),
                    '0'..='9' => Some(CharacterType::Digit),
                    // .
                    '.' => Some(CharacterType::Dot),
                    // "
                    '"' => Some(CharacterType::DoubleQuotation),
                    // =
                    '=' => Some(CharacterType::Equals),
                    // Horizontal tab.
                    '\t' => Some(CharacterType::HorizontalTab),
                    // -
                    '-' => Some(CharacterType::Hyphen),
                    // {
                    '{' => Some(CharacterType::LeftCurlyBracket),
                    // [
                    '[' => Some(CharacterType::LeftSquareBracket),
                    // +
                    '+' => Some(CharacterType::Plus),
                    // }
                    '}' => Some(CharacterType::RightCurlyBracket),
                    // ]
                    ']' => Some(CharacterType::RightSquareBracket),
                    // '
                    '\'' => Some(CharacterType::SingleQuotation),
                    // Space.
                    ' ' => Some(CharacterType::Space),
                    // _
                    '_' => Some(CharacterType::Underscore),
                    _ => Some(CharacterType::NonAscii),
                };
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
