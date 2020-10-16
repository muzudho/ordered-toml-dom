pub mod basic_strings;
pub mod comment;
pub mod literal_value;
pub mod single_quoted_string;

/// It has one string.  
/// １つの文字列を持ちます。  
#[derive(Clone)]
pub struct Comment {
    pub value: String,
}

/// It has one string.  
/// １つの文字列を持ちます。  
#[derive(Clone)]
pub struct DoubleQuotedString {
    pub value: String,
}

/// It has one string.  
/// １つの文字列を持ちます。  
#[derive(Clone)]
pub struct LiteralValue {
    pub value: String,
}

/// It has one string.  
/// １つの文字列を持ちます。  
#[derive(Clone)]
pub struct LiteralString {
    pub value: String,
}
