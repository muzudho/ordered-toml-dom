pub mod token;
pub mod token_line;

/// Token type.  
/// トークンの種類。  
#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    /// Alphabet character. A ～ Z, a ～ z.  1 disit.  
    Alpha,
    /// \  
    Backslash,
    /// }  
    CloseCurlyBracket,
    /// :  
    Colon,
    /// #  
    CommentStartSymbol,
    /// ,  
    Comma,
    /// Numeral character. 0 ～ 9. 1 disit.  
    Digit,
    /// .  
    Dot,
    /// "  
    DoubleQuotation,
    /// =  
    Equals,
    /// \r, \n, or combine.
    Newline,
    /// -  
    Hyphen,
    /// {  
    LeftCurlyBracket,
    /// [  
    LeftSquareBracket,
    /// +  
    Plus,
    /// }  
    RightCurlyBracket,
    /// ]  
    RightSquareBracket,
    /// 0x80 - 0xD7FF.
    NonAscii,
    /// Non end-of-line. 0x09, 0x20 - 0x7F, non-ascii.
    NonEol,
    /// '  
    SingleQuotation,
    /// A ～ Z, a ～ z.  Multiple disits.   
    /// 構文解析の結果。文字列トークン。  
    SPAlphabetString,
    SPDateTimeString,
    /// Syntax parser result.  
    /// Positional numeral system string.  
    /// Binary, Octal, Decimal, Hexadecimal...  
    /// 構文解析の結果。  
    /// 進数文字列。  
    /// `0x01aB23Cd` なら、 `01aB23Cd` の部分。  
    SPPositionalNumeralString,
    /// _  
    Underscore,
    /// Multi-byte character or more.  
    /// 全角文字などいろいろ。  
    Unknown,
    /// Whitespace means tab ('\t' 0x09) or space (' ' 0x20). Multiple digits.  
    /// ホワイトスペースは タブ ('\t', 0x09) と 半角スペース (' ' 0x20) です。 複数桁です。  
    WS,
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
