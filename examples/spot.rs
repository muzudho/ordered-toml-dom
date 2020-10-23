//! Spot test.
//! 単発テスト。
//!
//! `cargo run --example spot`

extern crate tomboy_toml_dom;

use chrono::FixedOffset;
use chrono::NaiveTime;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use tomboy_toml_dom::Toml;

fn main() {
    // Read a toml.
    // Toml読取。
    let doc = Toml::from_file("./resource/spot.toml");
    println!("display=|{}|", doc);
    println!("debug=|{:?}|", doc);

    assert_eq!(
        // "1979-05-27".
        doc.get_naive_date_by_key("ld1"),
        Some(match NaiveDate::parse_from_str("1979-05-27", "%Y-%m-%d") {
            Ok(n) => n,
            Err(why) => panic!("{}", why),
        })
    );

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
        doc.get_datetime_utc_by_key("odt1"),
        Some("1979-05-27T07:32:00Z".parse::<DateTime<Utc>>().unwrap())
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

    assert_eq!(
        doc.get_naive_time_by_key("lt1"),
        Some(NaiveTime::parse_from_str("07:32:00", "%H:%M:%S").unwrap())
    );
}
