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
    Log::set_file_name("exa-comment");
    Log::remove_old_logs();

    // Read a Toml file.
    let toml_file = "./resource/comment.toml";
    let doc = Toml::from_file(toml_file);
    Log::info_toml_document(toml_file, &doc);

    // TODO コメントはどうやって Get する？

    Log::flush();
}
