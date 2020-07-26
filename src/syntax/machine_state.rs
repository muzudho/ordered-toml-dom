/// `[ 'a', 'b', 'c' ]`.
#[derive(Clone, Debug)]
pub enum ArrayState {
    /// [ か , の次。
    AfterDoubleQuotedString,
    AfterLeftSquareBracket,
    AfterSingleQuotedString,
    /// , か ] を待ちます。
    AfterItem,
    DoubleQuotedString,
    End,
    SingleQuotedString,
}

/// `{ key = value, key = value }`.
#[derive(Debug)]
pub enum InlineTableState {
    AfterLeftCurlyBracket,
    KeyValue,
    AfterKeyValue,
}

/// `key = right_value`.
#[derive(Debug)]
pub enum KeyValueState {
    AfterKey,
    AfterEquals,
    AfterLeftCurlyBracket,
    AfterLeftSquareBracket,
    DoubleQuotedString,
    SingleQuotedString,
    End,
}

#[derive(Debug)]
pub enum LineState {
    AfterComment,
    AfterKeyValue,
    /// `# comment`.
    CommentSyntax,
    First,
    /// `key = right_value`.
    KeyValueSyntax,
    Unimplemented,
}
