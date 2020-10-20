//! An exemplary program.
//! 模範的なプログラム。
//!
//! `cargo run --example example`

extern crate tomboy_toml_dom;

use casual_logger::{Level, Log, Table};
use chrono::prelude::{DateTime, Utc};
use tomboy_toml_dom::Toml;

/// WIP.  
/// 作業中。  
fn main() {
    // Configuration a log.
    Log::set_file_name("exa-toml-io-en-a-quick-tour-of-toml-v100rc3");
    Log::set_level(Level::Debug);
    Log::set_retention_days(-1);
    Log::remove_old_logs();

    // Read a toml.
    // Toml読取。
    let doc = Toml::from_file("./resource/toml-io-en-a-quick-tour-of-toml-v100rc3.toml");
    Log::info_t(
        "Read.",
        Table::default()
            .str("Display", &format!("{}", doc))
            .str("Debug", &format!("{:?}", doc)),
    );

    assert_eq!(
        doc.get_string_by_key("str1"),
        Some("I'm a string.".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("str2"),
        Some("You can \"quote\" me.".to_string())
    );

    // Fixed. `\u0000` Unicode.
    assert_eq!(
        doc.get_string_by_key("str3"),
        Some("Name\tJos\u{00E9}\nLoc\tSF.".to_string())
    );

    assert_eq!(
        doc.get_string_by_key("str4"),
        Some(
            "
Roses are red
Violets are blue"
                .to_string()
        )
    );
    assert_eq!(
        doc.get_string_by_key("str5"),
        Some("The quick brown fox jumps over the lazy dog.".to_string())
    );

    assert_eq!(
        doc.get_string_by_key("path"),
        Some("C:\\Users\\nodejs\\templates".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("path2"),
        Some("\\\\User\\admin$\\system32".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("quoted"),
        Some("Tom \"Dubs\" Preston-Werner".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("regex"),
        Some("<\\i\\c*\\s*>".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("re"),
        Some("\\d{2} apps is t[wo]o many".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("lines"),
        Some(
            "The first newline is
trimmed in raw strings.
All other whitespace
is preserved.
"
            .to_string()
        )
    );
    assert_eq!(doc.get_i128_by_key("int1"), Some(99));
    assert_eq!(doc.get_i128_by_key("int2"), Some(42));
    assert_eq!(doc.get_i128_by_key("int3"), Some(0));
    assert_eq!(doc.get_i128_by_key("int4"), Some(-17));
    assert_eq!(doc.get_i128_by_key("hex1"), Some(0xDEADBEEF));
    assert_eq!(doc.get_i128_by_key("hex2"), Some(0xdeadbeef));
    assert_eq!(doc.get_i128_by_key("hex3"), Some(0xdead_beef));
    assert_eq!(doc.get_i128_by_key("oct1"), Some(0o01234567));
    assert_eq!(doc.get_i128_by_key("oct2"), Some(0o755));
    assert_eq!(doc.get_i128_by_key("bin1"), Some(0b11010110));

    assert_eq!(doc.get_f64_by_key("float1"), Some(1.0));
    assert_eq!(doc.get_f64_by_key("float2"), Some(3.1415));
    assert_eq!(doc.get_f64_by_key("float3"), Some(-0.01));
    assert_eq!(doc.get_f64_by_key("float4"), Some(5e+22));
    assert_eq!(doc.get_f64_by_key("float5"), Some(1e06));
    assert_eq!(doc.get_f64_by_key("float6"), Some(-2E-2));
    assert_eq!(doc.get_f64_by_key("float7"), Some(6.626e-34));
    assert_eq!(doc.get_f64_by_key("float8"), Some(224_617.445_991_228));
    assert_eq!(doc.get_f64_by_key("infinite1"), Some(f64::INFINITY));
    assert_eq!(doc.get_f64_by_key("infinite2"), Some(f64::INFINITY));
    assert_eq!(doc.get_f64_by_key("infinite3"), Some(-f64::INFINITY));
    assert!(if let Some(n) = doc.get_f64_by_key("not1") {
        n.is_nan() && n.is_sign_positive()
    } else {
        false
    });
    assert!(if let Some(n) = doc.get_f64_by_key("not2") {
        n.is_nan() && n.is_sign_positive()
    } else {
        false
    });
    assert!(if let Some(n) = doc.get_f64_by_key("not3") {
        n.is_nan() && n.is_sign_negative()
    } else {
        false
    });

    assert_eq!(
        doc.get_datetime_utc_by_key("odt1"),
        Some("1979-05-27T07:32:00Z".parse::<DateTime<Utc>>().unwrap())
    );
    assert_eq!(
        doc.get_datetime_utc_by_key("odt2"),
        Some(
            "1979-05-27T00:32:00-07:00"
                .parse::<DateTime<Utc>>()
                .unwrap()
        )
    );
    assert_eq!(
        doc.get_datetime_utc_by_key("odt3"),
        Some(
            "1979-05-27T00:32:00.999999-07:00"
                .parse::<DateTime<Utc>>()
                .unwrap()
        )
    );
    // TODO Local datetime
    /*
    assert_eq!(
        doc.get_datetime_utc_by_key("ldt1"),
        Some("1979-05-27T07:32:00".parse::<DateTime<Utc>>().unwrap())
    );
    assert_eq!(
        doc.get_datetime_utc_by_key("ldt2"),
        Some(
            "1979-05-27T00:32:00.999999"
                .parse::<DateTime<Utc>>()
                .unwrap()
        )
    );
    assert_eq!(
        doc.get_datetime_utc_by_key("ld1"),
        Some("1979-05-27".parse::<DateTime<Utc>>().unwrap())
    );
    assert_eq!(
        doc.get_datetime_utc_by_key("lt1"),
        Some("07:32:00".parse::<DateTime<Utc>>().unwrap())
    );
    assert_eq!(
        doc.get_datetime_utc_by_key("lt2"),
        Some("00:32:00.999999".parse::<DateTime<Utc>>().unwrap())
    );
    */
}
