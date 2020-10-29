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
    /* TODO
    assert_eq!(
        doc.get_string_array_by_key("int_array"),
        Ok(Some(vec![
            "-1".to_string(),
            "0".to_string(),
            "1".to_string()
        ]))
    );
    assert!(match doc.get_int_array_by_key::<i128>("int_array") {
        Ok(_) => true,
        Err(why) => panic!("{}", why),
    });
    assert_eq!(
        doc.get_int_array_by_key("int_array"),
        Ok(Some(vec![-1, 0, 1]))
    );
    */
    assert_eq!(
        doc.get_string_array_by_key("string_array"),
        Ok(Some(vec![
            "a".to_string(),
            "b".to_string(),
            "\"c\"".to_string()
        ]))
    );
}
