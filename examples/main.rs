//! WIP.
//! Test config file.
//! 設定ファイルのテスト。
//!
//! `cargo run --example main`

use toml_menu::scanner::LineScanner;

fn main() {
    let _doc = LineScanner::from_file("./casual-logger.type.toml");
}
