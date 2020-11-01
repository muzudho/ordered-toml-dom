pub mod basic_string;
pub mod comment;
pub mod date_time;
pub mod key;
pub mod literal_string;
pub mod literal_value;
pub mod non_ascii;
pub mod non_eol;
pub mod ws;
pub mod wschar;

use crate::model::layer110::Character;
use crate::model::layer110::Token;

/// Comment.  
/// コメント。  
#[derive(Clone)]
pub struct Comment {
    pub tokens: Vec<Token>,
}

/// Date time.  
/// 年月日日付。  
#[derive(Clone)]
pub struct DateTime {
    pub tokens: Vec<Token>,
}

/// It has one string.  
/// １つの文字列を持ちます。  
#[derive(Clone)]
pub struct BasicString {
    pub tokens: Vec<Token>,
}

/// Key.  
/// キー。  
#[derive(Clone)]
pub struct Key {
    pub tokens: Vec<Token>,
}

/// It has one string.  
/// １つの文字列を持ちます。  
#[derive(Clone)]
pub struct LiteralString {
    pub tokens: Vec<Token>,
}

/// It has one string.  
/// １つの文字列を持ちます。  
#[derive(Clone)]
pub struct LiteralValue {
    pub tokens: Vec<Token>,
}

/// NonAscii.  
/// 非ASCII。  
#[derive(Clone)]
pub struct NonAscii {
    pub character: Character,
}

/// Non end-of-line.  
/// 非行末。  
#[derive(Clone)]
pub struct NonEol {
    pub tokens: Vec<Token>,
}

/// Whitespace.  
/// 空白。  
#[derive(Clone)]
pub struct Ws {
    pub tokens: Vec<Token>,
}

/// Whitespace character.  
/// 空白文字。  
#[derive(Clone)]
pub struct Wschar {
    pub tokens: Vec<Token>,
}
