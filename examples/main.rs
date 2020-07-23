//! WIP.
//! Test config file.
//! 設定ファイルのテスト。
//!
//! `cargo run --example main`

use toml_menu::parser::Document;

fn main() {
    let doc = Document::from_file("./casual-logger.toml");
}
