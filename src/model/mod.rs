pub mod array;
pub mod comment;
pub mod document;
pub mod element;
pub mod inline_table;
pub mod key_value;
pub mod literal_string;
pub mod single_quoted_string;
pub mod value;

#[derive(Clone)]
pub struct Array {
    items: Vec<Value>,
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
pub enum Element {
    Comment(Comment),
    KeyValue(KeyValue),
}

#[derive(Clone)]
pub struct InlineTable {
    items: Vec<Value>,
}

#[derive(Clone)]
pub struct KeyValue {
    pub key: String,
    pub value: Box<Value>,
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
pub enum Value {
    Array(Array),
    InlineTable(InlineTable),
    KeyValue(KeyValue),
    LiteralString(LiteralString),
    SingleQuotedString(SingleQuotedString),
}
