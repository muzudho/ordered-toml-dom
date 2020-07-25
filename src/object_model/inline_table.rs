use crate::object_model::key_value::KeyValueModel;

pub struct InlineTableModel {
    pub items: Vec<InlineTableItemModel>,
}
impl Default for InlineTableModel {
    fn default() -> Self {
        InlineTableModel { items: Vec::new() }
    }
}

pub enum InlineTableItemModel {
    KeyValue(KeyValueModel),
}
