//! Array model.  
//! 配列モデル。  

pub struct ArrayM {
    pub items: Vec<ArrayItem>,
}
impl Default for ArrayM {
    fn default() -> Self {
        ArrayM { items: Vec::new() }
    }
}

pub enum ArrayItem {
    String(String),
}
