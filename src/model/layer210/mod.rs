pub mod basic_string;
pub mod comment;
pub mod date_time;
pub mod key;
pub mod literal_string;
pub mod literal_value;
pub mod ws;

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

/// White space.  
/// ホワイト・スペース。  
#[derive(Clone)]
pub struct WS {
    pub tokens: Vec<Token>,
}
