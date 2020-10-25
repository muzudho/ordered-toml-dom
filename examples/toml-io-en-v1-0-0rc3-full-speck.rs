//! An exemplary program.
//! 模範的なプログラム。
//!
//! `cargo run --example example`

extern crate tomboy_toml_dom;

use casual_logger::{Level, Log, Table};
use tomboy_toml_dom::Toml;

/// WIP.  
/// 作業中。  
fn main() {
    // Configuration a log.
    Log::set_file_name("exa-toml-io-en-v1-0-0rc3-full-speck");
    Log::set_level(Level::Debug);
    Log::set_retention_days(-1);
    Log::remove_old_logs();

    // Read a toml.
    // Toml読取。
    let doc = Toml::from_file("./resource/toml-io-en-v1-0-0rc3-full-speck.txt");
    Log::info_t(
        "Read.",
        Table::default()
            .str("Display", &format!("{}", doc))
            .str("Debug", &format!("{:?}", doc)),
    );
}
