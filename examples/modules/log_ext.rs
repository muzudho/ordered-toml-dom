use casual_logger::{Level, Log, Table};
use tomboy_toml_dom::model::layer310::TomlDocument;

pub trait LogExt {
    fn println(s: &str);
    fn println_t(s: &str, t: &mut Table);
    fn info_toml_document(toml_file: &str, doc: &TomlDocument);
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

    fn info_toml_document(toml_file: &str, doc: &TomlDocument) {
        Log::info_t(
            "Read",
            Table::default()
                .str("File", &format!("{}", toml_file))
                .uint("ExpressionCount", doc.elements.len() as u128)
                .str("OutputDocument", &format!("{}", doc)),
        );
    }
}
