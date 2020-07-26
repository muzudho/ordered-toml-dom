use std::fmt;

pub struct TokenLine {
    pub tokens: Vec<Token>,
}
impl Default for TokenLine {
    fn default() -> Self {
        TokenLine { tokens: Vec::new() }
    }
}
impl TokenLine {
    pub fn new(tokens: &Vec<Token>) -> Self {
        TokenLine {
            tokens: tokens.to_vec(),
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
    Key,
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

#[derive(Clone)]
pub struct Token {
    pub value: String,
    pub type_: TokenType,
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
        write!(f, "{}[{:?}]", self.value, self.type_)
    }
}
