pub mod character;
pub mod character_line;
pub mod token;
pub mod token_line;

/// Character type.  
/// 文字の種類。  
#[derive(Clone, Copy, Debug)]
pub enum CharacterType {
    /// Alphabet character. A ～ Z, a ～ z.  1 disit.  
    Alpha,
    /// \  
    Backslash,
    /// :  
    Colon,
    /// ,  
    Comma,
    /// #  
    CommentStartSymbol,
    /// Numeral character. 0 ～ 9. 1 disit.  
    Digit,
    /// .  
    Dot,
    /// "  
    DoubleQuotation,
    /// =  
    Equals,
    /// \t
    HorizontalTab,
    /// -  
    Hyphen,
    /// {  
    LeftCurlyBracket,
    /// [  
    LeftSquareBracket,
    /// \r, \n, or combine.
    Newline,
    /// +  
    Plus,
    /// }  
    RightCurlyBracket,
    /// ]  
    RightSquareBracket,
    /// '  
    SingleQuotation,
    /// 半角スペース。  
    Space,
    /// _  
    Underscore,
    NonAscii,
}

/// Token type.  
/// トークンの種類。  
#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    /// 0x80 - 0xD7FF.
    NonAscii,
    /// Non end-of-line. 0x09, 0x20 - 0x7F, non-ascii.
    NonEol,
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
    /// Multi-byte character or more.  
    /// 全角文字などいろいろ。  
    Unknown,
    LiteralValue,
    LiteralString,
    Key,
    BasicString,
    Comment,
    DateTime,
    EscapeSequence,
    Table,
    /// White space.
    WSOld,
    Ws,
    Wschar,
}

/// A row of characters.  
/// 一列の字。  
pub struct CharacterLine {
    pub row_number: usize,
    pub characters: Vec<Character>,
}

/// A row of tokens.  
/// 一列のトークン。  
pub struct TokenLine {
    pub row_number: usize,
    pub tokens: Vec<Token>,
}

/// Character.  
/// 字。  
#[derive(Clone)]
pub struct Character {
    pub column_number: usize,
    pub value: char,
    pub type_: CharacterType,
}

/// Token.  
/// 字句。  
#[derive(Clone)]
pub struct Token {
    pub column_number: usize,
    pub value: String,
    pub type_: TokenType,
}
