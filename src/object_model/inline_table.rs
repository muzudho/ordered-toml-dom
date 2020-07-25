use crate::object_model::value::Value;

#[derive(Clone)]
pub struct InlineTableModel {
    pub items: Vec<Value>,
}
impl Default for InlineTableModel {
    fn default() -> Self {
        InlineTableModel { items: Vec::new() }
    }
}
