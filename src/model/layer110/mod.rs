pub mod token;
pub mod token_line;

/// Token type.  
/// トークンの種類。  
#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    /// A ～ Z, a ～ z.  Multiple ditis.  
    AlphabetString,
    /// \  
    Backslash,
    /// }  
    CloseCurlyBracket,
    /// :  
    Colon,
    /// ,  
    Comma,
    /// .  
    Dot,
    /// "  
    DoubleQuotation,
    EndOfLine,
    /// =  
    Equals,
    /// -  
    Hyphen,
    /// {  
    LeftCurlyBracket,
    /// [  
    LeftSquareBracket,
    /// 0 ～ 9. Multiple ditis.  
    NumeralString,
    /// +  
    Plus,
    /// }  
    RightCurlyBracket,
    /// ]  
    RightSquareBracket,
    /// #  
    Sharp,
    /// '  
    SingleQuotation,
    /// _  
    Underscore,
    /// Multi-byte character or more.  
    /// 全角文字などいろいろ。  
    Unknown,
    /// Whitespace means tab ('\t' 0x09) or space (' ' 0x20).  
    /// ホワイトスペースは タブ ('\t', 0x09) と 半角スペース (' ' 0x20) です。  
    WhiteSpace,
}

/// A row of tokens.  
/// 一列のトークン。  
pub struct TokenLine {
    pub row_number: usize,
    pub tokens: Vec<Token>,
}

/// Token.  
/// 字句。  
#[derive(Clone)]
pub struct Token {
    pub column_number: usize,
    pub value: String,
    pub type_: TokenType,
}
