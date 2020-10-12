use casual_logger::{Level, Log, Table};
use tomboy_toml_dom::model::layer310::Document;

pub trait LogExt {
    fn println(s: &str);
    fn println_t(s: &str, t: &mut Table);
    fn info_toml_document(doc: &Document);
}

impl LogExt for Log {
    /// Info level logging and add print to stdout.
    fn println(s: &str) {
        if Log::enabled(Level::Info) {
            println!("{}", s);
        }
        Log::infoln(s);
    }

    /// Info level logging and add print to stdout.
    fn println_t(s: &str, t: &mut Table) {
        if Log::enabled(Level::Info) {
            println!("{}", s);
        }
        Log::infoln_t(s, t);
    }

    fn info_toml_document(doc: &Document) {
        Log::info_t(
            "Product.",
            Table::default()
                .uint("DocumentElementCount", doc.elements.len() as u128)
                .str("OutputDocument", &format!("{:?}", doc)),
        );
    }
}
