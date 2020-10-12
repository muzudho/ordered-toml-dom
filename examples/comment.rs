//! Test.
//! テスト。
//!
//! `cargo run --example comment`
extern crate tomboy_toml_dom;

mod modules;

use crate::modules::log_ext::LogExt;
use casual_logger::{Log, Table};
use tomboy_toml_dom::Toml;

fn main() {
    Log::println("Start.");
    Log::set_file_name("comment");
    Log::remove_old_logs();

    // Read a file.
    let doc = Toml::from_file("./resource/comment.toml");
    Log::info_t(
        "Product.",
        Table::default()
            .uint("DocumentElementCount", doc.elements.len() as u128)
            .str("OutputDocument", &format!("{:?}", doc)),
    );

    // TODO コメントはどうやって Get する？

    Log::flush();
    Log::println("Finished.");
}
