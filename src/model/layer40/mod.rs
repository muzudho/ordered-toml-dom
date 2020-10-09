pub mod document;

use crate::model::{layer20::KeyValue, layer30::BroadLine};

/// It has multiple `broad_line`.  
/// 複数の `縦幅を持つ行` を持ちます。  
#[derive(Clone)]
pub struct Document {
    /// Line with height.
    /// 縦幅を持つ行。
    pub broad_lines: Vec<BroadLine>,
}
