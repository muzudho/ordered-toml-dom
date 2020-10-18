pub mod basic_string;
pub mod comment;
pub mod escape_sequence;
pub mod key;
pub mod literal_string;
pub mod literal_value;

use crate::model::layer110::Token;

/// It has one string.  
/// １つの文字列を持ちます。  
#[derive(Clone)]
pub struct Comment {
    pub tokens: Vec<Token>,
}

/// It has one string.  
/// １つの文字列を持ちます。  
#[derive(Clone)]
pub struct BasicString {
    pub tokens: Vec<Token>,
}

/// It has one string.  
/// １つの文字列を持ちます。  
#[derive(Clone)]
pub struct EscapeSequence {
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
