# Tomboy toml dom

For those who are struggling with Rust's cool syntax, our goal is to provide a TOML parser that's as easy as pointing to a menu and eating fast food.  
Rustのイケてる構文に難儀している人のために、メニューを指差してファーストフードを食べるぐらい簡単な操作のTOMLパーサーを提供することを目標とします。  

Unstable version. It's a tryal and error process. Specifications will change.  
不安定版。 試行錯誤中です。 仕様はコロコロ変わるでしょう。  

Tomboy is a pun.  
トムボーイ（おてんば娘）は語呂合わせです。  

References:  

* [Developer's blog(開発者ブログ)](https://crieit.net/drafts/5f8094a14a0cf)
* [TOML parsing（TOMLの構文解析）](https://crieit.net/posts/TOML-parsing-TOML)

## Run (実行)

Take a look at the repository.  
リポジトリを見てください。  

```shell
cargo run --example comment
cargo run --example example
cargo run --example inline_table
cargo run --example main
cargo run --example mix_array
cargo run --example spot
cargo run --example table
cargo run --example toml-io-en-a-quick-tour-of-toml-v100rc3
```

## Specification (仕様)

The specifications will gradually solidify.  
仕様は少しずつ固めていきます。  

You can think that you can't do anything that isn't written here.  
ここに書かれていないことは何もできないと思ってもらって構いません。  

```rust
//! An exemplary program.
//! 模範的なプログラム。
//!
//! `cargo run --example example`

extern crate tomboy_toml_dom;

use chrono::prelude::{DateTime, Utc};
use chrono::FixedOffset;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use tomboy_toml_dom::Toml;

fn main() {
    // Read a toml.
    // Toml読取。
    let doc = Toml::from_file("./resource/example.toml");

    // Read a number.
    // 数値読取。
    assert_eq!(doc.get_i128_by_key("age"), Some(40));
    assert_eq!(doc.get_i128_by_key("int_max"), Some(2147483647));
    assert_eq!(doc.get_i128_by_key("int_min"), Some(-2147483648));
    assert_eq!(doc.get_f64_by_key("weight"), Some(93.5));
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

    // WIP. Read a string.
    // 作業中。 文字列読取。
    assert_eq!(doc.get_string_by_key("apple"), Some("pie".to_string()));

    assert_eq!(
        doc.get_string_by_key("basic_string_letter"),
        Some("Hello, world!!".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("basic_string_empty"),
        Some("".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("basic_string_escape_backslash"),
        Some("\\".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("basic_string_escape_double_quotation"),
        Some("\"".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("basic_string_punctuation"),
        Some("., ={}[]'\"\\!?".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("basic_string_tab"),
        Some("a\tb".to_string())
    );

    assert_eq!(
        doc.get_string_by_key("multiline_basic_string_letter"),
        Some(
            "Hello,
world!!"
                .to_string()
        )
    );

    assert_eq!(
        doc.get_string_by_key("multiline_basic_string_punctuation"),
        Some(
            "., ={}[]\"'\"\"\\
!?"
            .to_string()
        )
    );
    assert_eq!(
        doc.get_string_by_key("multiline_basic_string_trim_start"),
        Some("The quick brown fox jumps over the lazy dog.".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("multiline_basic_string_escape_double_quotation"),
        Some(
            "
\\
"
            .to_string()
        )
    );
    /*
    // Fixed.
    println!(
        "debug|multiline_basic_string_tab|{}",
        doc.get_debug_string_by_key("multiline_basic_string_tab")
    );
    */
    assert_eq!(
        doc.get_string_by_key("multiline_basic_string_tab"),
        Some(
            "
a\tb
"
            .to_string()
        )
    );

    assert_eq!(
        doc.get_string_by_key("literal_string_empty"),
        Some("".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("literal_string_letter"),
        Some("Hello, world!!".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("literal_string_punctuation"),
        Some("., ={}[]\"\\!?".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("multiline_literal_string_letter"),
        Some(
            "Hello,
world!!"
                .to_string()
        )
    );
    assert_eq!(
        doc.get_string_by_key("multiline_literal_string_punctuation"),
        Some(
            "., ={}[]'\"\\
!?"
            .to_string()
        )
    );
    assert_eq!(
        doc.get_string_by_key("multiline_literal_string_first_newline_is_trimmed"),
        Some(
            "The first newline is
trimmed in raw strings.
All other whitespace
is preserved.
"
            .to_string()
        )
    );

    // Read a boolean.
    // 論理値読取。
    assert_eq!(doc.get_bool_by_key("adult"), Some(true));
    assert_eq!(doc.get_bool_by_key("student"), Some(false));

    // DateTime.
    // 日付と時刻。
    assert_eq!(
        doc.get_datetime_utc_by_key("dob"),
        Some(
            "1979-05-27T07:32:00-08:00"
                .parse::<DateTime<Utc>>()
                .unwrap()
        )
    );

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
```

## TODO

* [ ] Literal
  * [ ] Literal numbers...
    * [ ] integer
      * [x] `0b` - binary.
      * [x] `0o` - octal.
      * [x] `0x` - hexadecimal.
      * [x] `_` - separators.
    * [ ] float
      * [x] `.` - point. Example: `3.14`.
      * [x] `_` - separators.
      * [x] `inf` - Positive infinity.
      * [x] `+inf` - Positive infinity.
      * [x] `-inf` - Negative infinity.
      * [x] `nan` - Not a number. Positive.
      * [x] `+nan` - Not a number. Positive.
      * [x] `-nan` - Not a number. Negative.
* [ ] String (Not str)
  * [x] `"abc"` - Basic string.
    * [x] Plain.
    * [ ] Escape sequence.
  * [ ] `"""abc"""` - Multi-line basic string.
    * [x] Plain.
    * [ ] Escape sequence.
    * [x] Ending backslash to automatically trim.
  * [ ] `'abc'` - Literal string.
    * [x] Plain.
  * [ ] `'''abc'''` - multi-line literal string.
    * [x] Plain.
    * [x] The first newline is trimmed in raw string.
  * [ ] Escape sequence.
    * [x] `\r` - caridge return.
    * [x] `\n` - line feed.
    * [x] `\t` - tab.
    * [x] `\\` - backslash.
    * [x] `\"` - double quotation.
    * [x] `\u0000` - Unicode.
    * [ ] `\U00000000` - Unicode.
* [ ] DateTime
  * [x] `1979-05-27` - Local date. (Naive date)
  * [x] `1979-05-27T07:32:00` - Local datetime. (Naive datetime)
  * [x] `1979-05-27T07:32:00Z` - UTC datetime. (Datetime Utc)
  * [x] `1979-05-27T00:32:00.999999` - Local datetime. (Naive datetime)
  * [x] `1979-05-27T00:32:00-07:00` - UTC datetime. (Datetime fixed offset)
  * [x] `1979-05-27T00:32:00.999999-07:00` - UTC datetime. (Datetime fixed offset)
  * [x] `07:32:00` - Local time. (Naive time)
  * [x] `00:32:00.999999` - Local time. (Naive time)

