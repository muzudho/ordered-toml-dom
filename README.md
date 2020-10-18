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
}
```

## TODO

* [ ] Literal
  * [x] Parsing a literal containing dots. Example: `3.14`.  
      ドットを含むリテラル文字列の解析。例： `3.14`。
  * [ ] Numbers...
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
    * [x] `\r` caridge return.
    * [x] `\n` line feed.
    * [x] `\t` tab.
    * [x] `\\` tab.
    * [x] `\"` tab.
    * [x] `\u0000` Unicode.
    * [ ] `\U00000000` Unicode.
* [ ] DateTime
    * [ ]  UTC
      * [x] `1979-05-27T07:32:00-08:00`.
