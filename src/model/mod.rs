pub mod array;
pub mod comment;
pub mod document;
pub mod double_quoted_string;
pub mod element;
pub mod inline_table;
pub mod item_value;
pub mod key_value;
pub mod literal_string;
pub mod single_quoted_string;

#[derive(Clone)]
pub struct Array {
    items: Vec<ItemValue>,
}

#[derive(Clone)]
pub struct Comment {
    value: String,
}

#[derive(Clone)]
pub struct Document {
    pub elements: Vec<Element>,
}

#[derive(Clone)]
pub struct DoubleQuotedString {
    pub value: String,
}

#[derive(Clone)]
pub enum Element {
    Comment(Comment),
    KeyValue(KeyValue),
}

#[derive(Clone)]
pub struct InlineTable {
    items: Vec<ItemValue>,
}

#[derive(Clone)]
pub struct KeyValue {
    pub key: String,
    pub value: Box<ItemValue>,
}

#[derive(Clone)]
pub struct LiteralString {
    value: String,
}

#[derive(Clone)]
pub struct SingleQuotedString {
    pub value: String,
}

#[derive(Clone)]
pub enum ItemValue {
    Array(Array),
    DoubleQuotedString(DoubleQuotedString),
    InlineTable(InlineTable),
    KeyValue(KeyValue),
    LiteralString(LiteralString),
    SingleQuotedString(SingleQuotedString),
}
