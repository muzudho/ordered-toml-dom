//! WIP.
//! Test config file.
//! 設定ファイルのテスト。
//!
//! `cargo run --example main`

use casual_logger::Log;
use toml_menu::scanner::LineScanner;
fn main() {
    Log::remove_old_logs();
    let _doc = LineScanner::from_file("./casual-logger.type.toml");
    Log::flush();
}
