//! Document syntax parser.  
//! ドキュメント構文解析器。  

use crate::model::layer310::TomlDocument;
use crate::parser::phase200::{
    error_via,
    {layer210::PResult, layer230::ExpressionP, layer310::DocumentP},
};
use casual_logger::Table;
use look_ahead_items::ItemsBuilder;

impl Default for DocumentP {
    fn default() -> Self {
        DocumentP { expression_p: None }
    }
}
impl DocumentP {
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn scan_line(&mut self, char_vec: &Vec<char>, doc: &mut TomlDocument) -> PResult {
        let items = ItemsBuilder::default()
            .set_look_ahead_size(4)
            .read(char_vec)
            .build();

        // * `tokens` - Tokens contains look ahead.
        //             先読みを含むトークン。
        // (Current character, 1 ahead character, 2 ahead character)
        // （現在のトークン, １つ先のトークン，２つ先のトークン）
        for look_ahead_characters in items {
            if let None = self.expression_p {
                self.expression_p = Some(ExpressionP::default());
            }
            let p = self.expression_p.as_mut().unwrap();
            match p.parse(&look_ahead_characters) {
                PResult::End => {
                    if let Some(m) = p.flush() {
                        doc.push_element(&m);
                        self.expression_p = None;
                    } else {
                        // TODO 何も取得できないことがある？
                        /*
                        return error(&mut self.log(), &tokens, "document.rs.34.");
                        */
                    }
                }
                PResult::Err(mut table) => {
                    return error_via(
                        &mut table,
                        &mut self.log(),
                        &look_ahead_characters,
                        "document.rs.92.",
                    );
                }
                PResult::Ongoing => {}
            }
        }

        PResult::Ongoing
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> Table {
        let mut t = Table::default();
        if let Some(p) = &self.expression_p {
            t.sub_t("expression_p", &p.log());
        }
        t
    }
}
