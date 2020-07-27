pub mod array;
pub mod broad_line;
pub mod comment;
pub mod document;
pub mod double_quoted_string;
pub mod inline_table;
pub mod item_value;
pub mod key_value;
pub mod literal_string;
pub mod right_value;
pub mod single_quoted_string;

/// It has multiple item values.  
/// 複数の項目値を持ちます。  
#[derive(Clone)]
pub struct Array {
    items: Vec<ItemValue>,
}

/// Either a Empty-line, Comment, Key Value, Table or a Array-of-table.  
/// 空行、コメント、キー値、テーブル、テーブルの配列のいずれかです。  
#[derive(Clone)]
pub enum BroadLine {
    Comment(Comment),
    EmptyLine,
    KeyValue(KeyValue),
    // TODO Table
    // TODO ArrayOfTable
}

/// It has one string.  
/// １つの文字列を持ちます。  
#[derive(Clone)]
pub struct Comment {
    value: String,
}

/// It has multiple `broad_line`.  
/// 複数の `縦幅を持つ行` を持ちます。  
#[derive(Clone)]
pub struct Document {
    /// Line with height.
    /// 縦幅を持つ行。
    pub broad_lines: Vec<BroadLine>,
}

/// It has one string.  
/// １つの文字列を持ちます。  
#[derive(Clone)]
pub struct DoubleQuotedString {
    pub value: String,
}

/// It has multiple item values.  
/// 複数の項目値を持ちます。  
#[derive(Clone)]
pub struct InlineTable {
    items: Vec<ItemValue>,
}

/// It has a key and a value.  
/// キーと値を持ちます。  
#[derive(Clone)]
pub struct KeyValue {
    pub key: String,
    pub value: Box<RightValue>,
}

/// It has one string.  
/// １つの文字列を持ちます。  
#[derive(Clone)]
pub struct LiteralString {
    value: String,
}

/// The right side of the key value model.  
/// キー値モデルの右辺です。  
#[derive(Clone)]
pub enum RightValue {
    Array(Array),
    DoubleQuotedString(DoubleQuotedString),
    InlineTable(InlineTable),
    // No KeyValue.
    LiteralString(LiteralString),
    SingleQuotedString(SingleQuotedString),
}

/// It has one string.  
/// １つの文字列を持ちます。  
#[derive(Clone)]
pub struct SingleQuotedString {
    pub value: String,
}

/// Array, inline table item.  
/// 配列、インライン・テーブルの項目です。  
#[derive(Clone)]
pub enum ItemValue {
    Array(Array),
    DoubleQuotedString(DoubleQuotedString),
    InlineTable(InlineTable),
    KeyValue(KeyValue),
    LiteralString(LiteralString),
    SingleQuotedString(SingleQuotedString),
}
