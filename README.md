# Tomboy toml dom

For those who are struggling with Rust's cool syntax, our goal is to provide a TOML parser that's as easy as pointing to a menu and eating fast food.  
Rustのイケてる構文に難儀している人のために、メニューを指差してファーストフードを食べるぐらい簡単な操作のTOMLパーサーを提供することを目標とします。  

It's a tryal and error process. Specifications will change.  
試行錯誤中です。 仕様はコロコロ変わるでしょう。  

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
cargo run --example key_value_number
cargo run --example key_value_wip
cargo run --example key_value
cargo run --example main
cargo run --example mix_array
cargo run --example table
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

use tomboy_toml_dom::Toml;

fn main() {
    // Read a toml.
    // Toml読取。
    let doc = Toml::from_file("./resource/example.toml");

    // Read a number.
    // 数値読取。
    assert_eq!(doc.get_i128_by_key("age"), Some(40));
    assert_eq!(doc.get_f64_by_key("weight"), Some(93.5));

    // WIP. Read a string.
    // 作業中。 文字列読取。
    assert_eq!(doc.get_str_by_key("apple"), Some("pie"));

    assert_eq!(
        doc.get_str_by_key("basic_string_letter"),
        Some("Hello, world!!")
    );
    assert_eq!(doc.get_str_by_key("basic_string_empty"), Some(""));
    assert_eq!(
        doc.get_str_by_key("basic_string_escape_backslash"),
        Some("\\")
    );
    assert_eq!(
        doc.get_str_by_key("basic_string_escape_double_quotation"),
        Some("\"")
    );
    assert_eq!(
        doc.get_str_by_key("basic_string_punctuation"),
        Some("., ={}[]'\"\\!?")
    );

    assert_eq!(
        doc.get_str_by_key("multiline_basic_string_letter"),
        Some(
            "Hello,
world!!"
        )
    );

    assert_eq!(
        doc.get_str_by_key("multiline_basic_string_punctuation"),
        Some(
            "., ={}[]\"'\"\"\\
!?"
        )
    );
    assert_eq!(
        doc.get_str_by_key("multiline_basic_string_trim_start"),
        Some("The quick brown fox jumps over the lazy dog.")
    );

    assert_eq!(doc.get_str_by_key("literal_string_empty"), Some(""));
    assert_eq!(
        doc.get_str_by_key("literal_string_letter"),
        Some("Hello, world!!")
    );
    assert_eq!(
        doc.get_str_by_key("literal_string_punctuation"),
        Some("., ={}[]\"\\!?")
    );
    assert_eq!(
        doc.get_str_by_key("multiline_literal_string_letter"),
        Some(
            "Hello,
world!!"
        )
    );
    assert_eq!(
        doc.get_str_by_key("multiline_literal_string_punctuation"),
        Some(
            "., ={}[]'\"\\
!?"
        )
    );

    // Read a boolean.
    // 論理値読取。
    assert_eq!(doc.get_bool_by_key("adult"), Some(true));
    assert_eq!(doc.get_bool_by_key("student"), Some(false));
}
```

## TODO

* [ ] Literal
  * [x] Parsing a literal containing dots. Example: `3.14`.  
      ドットを含むリテラル文字列の解析。例： `3.14`。
  * [ ] Numbers...
* [ ] String
  * [x] `"abc"` - Basic string.
    * [x] Plain.
    * [x] `\` Escape.
  * [ ] `"""abc"""` - Multi-line basic string.
    * [x] Plain.
    * [x] Ending backslash to automatically trim.
  * [ ] `'abc'` - Literal string.
    * [x] Plain.
  * [ ] `'''abc'''` - multi-line literal string.
    * [x] Plain.
