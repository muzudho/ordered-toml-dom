//! Edge case.
//! エッジケースをここに書く予定。

extern crate tomboy_toml_dom;

use tomboy_toml_dom::Toml;

fn main() {
    // Read a toml.
    // Toml読取。
    let doc = Toml::from_file("./resource/cover.toml");

    // Read a array.
    // 配列読取。
    assert_eq!(
        doc.get_string_array_by_key("string_array"),
        Ok(Some(vec![
            "a".to_string(),
            "b".to_string(),
            "\"c\"".to_string()
        ]))
    );
}
