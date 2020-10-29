//! An exemplary program.
//! 模範的なプログラム。
//!
//! `cargo run --example example`

extern crate tomboy_toml_dom;

use casual_logger::{Level, Log, Table};
use chrono::FixedOffset;
use chrono::NaiveDate;
use chrono::NaiveTime;
use chrono::{
    naive::NaiveDateTime,
    prelude::{DateTime, Utc},
};
use tomboy_toml_dom::Toml;

/// WIP.  
/// 作業中。  
fn main() {
    // Configuration a log.
    Log::set_file_name("exa-toml-io-en-a-quick-tour-of-toml-v1-0-0rc3");
    Log::set_level(Level::Debug);
    Log::set_retention_days(-1);
    Log::remove_old_logs();

    // Read a toml.
    // Toml読取。
    let doc = Toml::from_file("./resource/toml-io-en-a-quick-tour-of-toml-v1-0-0rc3.toml");
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
    assert_eq!(doc.get_i128_by_key_v2("int1"), Ok(Some(99)));
    assert_eq!(doc.get_i128_by_key_v2("int2"), Ok(Some(42)));
    assert_eq!(doc.get_i128_by_key_v2("int3"), Ok(Some(0)));
    assert_eq!(doc.get_i128_by_key_v2("int4"), Ok(Some(-17)));
    assert_eq!(doc.get_i128_by_key_v2("hex1"), Ok(Some(0xDEADBEEF)));
    assert_eq!(doc.get_i128_by_key_v2("hex2"), Ok(Some(0xdeadbeef)));
    assert_eq!(doc.get_i128_by_key_v2("hex3"), Ok(Some(0xdead_beef)));
    assert_eq!(doc.get_i128_by_key_v2("oct1"), Ok(Some(0o01234567)));
    assert_eq!(doc.get_i128_by_key_v2("oct2"), Ok(Some(0o755)));
    assert_eq!(doc.get_i128_by_key_v2("bin1"), Ok(Some(0b11010110)));

    assert_eq!(doc.get_f64_by_key_v2("float1"), Ok(Some(1.0)));
    assert_eq!(doc.get_f64_by_key_v2("float2"), Ok(Some(3.1415)));
    assert_eq!(doc.get_f64_by_key_v2("float3"), Ok(Some(-0.01)));
    assert_eq!(doc.get_f64_by_key_v2("float4"), Ok(Some(5e+22)));
    assert_eq!(doc.get_f64_by_key_v2("float5"), Ok(Some(1e06)));
    assert_eq!(doc.get_f64_by_key_v2("float6"), Ok(Some(-2E-2)));
    assert_eq!(doc.get_f64_by_key_v2("float7"), Ok(Some(6.626e-34)));
    assert_eq!(
        doc.get_f64_by_key_v2("float8"),
        Ok(Some(224_617.445_991_228))
    );
    assert_eq!(doc.get_f64_by_key_v2("infinite1"), Ok(Some(f64::INFINITY)));
    assert_eq!(doc.get_f64_by_key_v2("infinite2"), Ok(Some(f64::INFINITY)));
    assert_eq!(doc.get_f64_by_key_v2("infinite3"), Ok(Some(-f64::INFINITY)));
    assert!(if let Ok(Some(n)) = doc.get_f64_by_key_v2("not1") {
        n.is_nan() && n.is_sign_positive()
    } else {
        false
    });
    assert!(if let Ok(Some(n)) = doc.get_f64_by_key_v2("not2") {
        n.is_nan() && n.is_sign_positive()
    } else {
        false
    });
    assert!(if let Ok(Some(n)) = doc.get_f64_by_key_v2("not3") {
        n.is_nan() && n.is_sign_negative()
    } else {
        false
    });

    assert_eq!(
        doc.get_datetime_utc_by_key("odt1"),
        Some("1979-05-27T07:32:00Z".parse::<DateTime<Utc>>().unwrap())
    );

    assert_eq!(
        doc.get_datetime_fixed_offset_by_key("odt2"),
        Some(
            "1979-05-27T00:32:00-07:00"
                .parse::<DateTime<FixedOffset>>()
                .unwrap()
        )
    );

    assert_eq!(
        doc.get_datetime_fixed_offset_by_key("odt3"),
        Some(
            "1979-05-27T00:32:00.999999-07:00"
                .parse::<DateTime<FixedOffset>>()
                .unwrap()
        )
    );

    // TODO Local datetime
    assert_eq!(
        // "1979-05-27T07:32:00". Toml の独自書式か。該当するフォーマット定義見つからず。
        doc.get_naive_datetime_by_key("ldt1"),
        Some(
            match NaiveDateTime::parse_from_str("1979-05-27T07:32:00", "%Y-%m-%dT%H:%M:%S") {
                Ok(n) => n,
                Err(why) => panic!("{}", why),
            }
        )
    );

    assert_eq!(
        // "1979-05-27T00:32:00.999999".
        doc.get_naive_datetime_by_key("ldt2"),
        Some(
            NaiveDateTime::parse_from_str("1979-05-27T00:32:00.999999", "%Y-%m-%dT%H:%M:%S%.6f")
                .unwrap()
        )
    );

    assert_eq!(
        // "1979-05-27".
        doc.get_naive_date_by_key("ld1"),
        Some(match NaiveDate::parse_from_str("1979-05-27", "%Y-%m-%d") {
            Ok(n) => n,
            Err(why) => panic!("{}", why),
        })
    );

    assert_eq!(
        doc.get_naive_time_by_key("lt1"),
        Some(NaiveTime::parse_from_str("07:32:00", "%H:%M:%S").unwrap())
    );

    assert_eq!(
        doc.get_naive_time_by_key("lt2"),
        Some(NaiveTime::parse_from_str("00:32:00.999999", "%H:%M:%S%.6f").unwrap())
    );
}
