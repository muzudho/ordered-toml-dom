pub struct ArrayModel {
    pub items: Vec<ArrayItem>,
}
impl Default for ArrayModel {
    fn default() -> Self {
        ArrayModel { items: Vec::new() }
    }
}

pub enum ArrayItem {
    String(String),
}
