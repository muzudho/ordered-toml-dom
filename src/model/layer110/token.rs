//! Token.  
//! 字句。  

use std::fmt;

/// A row of tokens.  
/// 一列のトークン。  
pub struct TokenLine {
    pub row_number: usize,
    pub tokens: Vec<Token>,
}
impl TokenLine {
    pub fn new(row_number: usize) -> Self {
        TokenLine {
            row_number: row_number,
            tokens: Vec::new(),
        }
    }

    /// Remaining tokens.
    /// 残りのトークン。
    pub fn remaining_tokens(&self, token_number: usize) -> Self {
        TokenLine {
            row_number: self.row_number,
            tokens: self.tokens[token_number..].to_vec(),
        }
    }
}
impl fmt::Debug for TokenLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&format!("{:?}", token));
        }
        write!(f, "{}", buf)
    }
}

/// Token type.  
/// トークンの種類。  
#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    /// }  
    CloseCurlyBracket,
    /// ,  
    Comma,
    /// .  
    Dot,
    /// "  
    DoubleQuotation,
    EndOfLine,
    Equals,
    /// キーに使える文字で構成した単語。ドットは含まない。
    KeyWithoutDot,
    /// Multi-byte character or more.  
    /// 全角文字などいろいろ。  
    Otherwise,
    /// {  
    LeftCurlyBracket,
    /// [  
    LeftSquareBracket,
    /// }  
    RightCurlyBracket,
    /// ]  
    RightSquareBracket,
    /// #  
    Sharp,
    /// '  
    SingleQuotation,
    /// Whitespace means tab ('\t' 0x09) or space (' ' 0x20).  
    /// ホワイトスペースは タブ ('\t', 0x09) と 半角スペース (' ' 0x20) です。  
    WhiteSpace,
}

/// Token.  
/// 字句。  
#[derive(Clone)]
pub struct Token {
    pub column_number: usize,
    pub value: String,
    pub type_: TokenType,
}
impl Token {
    pub fn new(column_number: usize, value: &str, type_: TokenType) -> Self {
        Token {
            column_number: column_number,
            value: value.to_string(),
            type_: type_,
        }
    }
}
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}[{:?}]", self.value, self.type_)
    }
}
