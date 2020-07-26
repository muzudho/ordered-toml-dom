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
pub struct ArrayM {
    items: Vec<ValueM>,
}

#[derive(Clone)]
pub struct CommentM {
    value: String,
}

#[derive(Clone)]
pub struct DocumentM {
    pub elements: Vec<ElementM>,
}

#[derive(Clone)]
pub enum ElementM {
    Comment(CommentM),
    KeyValue(KeyValueM),
}

#[derive(Clone)]
pub struct InlineTableM {
    items: Vec<ValueM>,
}

#[derive(Clone)]
pub struct KeyValueM {
    pub key: String,
    pub value: Box<ValueM>,
}

#[derive(Clone)]
pub struct LiteralStringM {
    value: String,
}

#[derive(Clone)]
pub struct SingleQuotedStringM {
    pub value: String,
}

#[derive(Clone)]
pub enum ValueM {
    Array(ArrayM),
    InlineTable(InlineTableM),
    KeyValue(KeyValueM),
    LiteralString(LiteralStringM),
    SingleQuotedString(SingleQuotedStringM),
}
