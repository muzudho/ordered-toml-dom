//! Test.
//! テスト。
//!
//! `cargo run --example comment`

extern crate tomboy_toml_dom;

mod modules;

use crate::modules::log_ext::LogExt;
use casual_logger::Log;
use tomboy_toml_dom::Toml;

fn main() {
    // Configuration a log.
    Log::set_file_name("comment");
    Log::remove_old_logs();
    Log::println("Start.");

    // Read a Toml file.
    let doc = Toml::from_file("./resource/comment.toml");
    Log::info_toml_document(&doc);

    // TODO コメントはどうやって Get する？

    Log::flush();
    Log::println("Finished.");
}
