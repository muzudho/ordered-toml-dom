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
cargo run --example cover
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

use tomboy_toml_dom::model::layer310::Document;
use tomboy_toml_dom::Toml;

fn main() {
    // Read a toml.
    // Toml読取。
    let toml_file = "./resource/example.toml";
    let doc = Toml::from_file(toml_file);

    // Read a number.
    // 数値読取。
    test_age(&doc);
    test_weight(&doc);

    // WIP. Read a string.
    // 作業中。 文字列読取。
    test_apple(&doc);
    test_basic_string_empty(&doc);
    test_basic_string_escape_backslash(&doc);
    test_basic_string_escape_double_quotation(&doc);
    test_basic_string_punctuation(&doc);
    test_multiline_basic_string_letter(&doc);
    test_multiline_basic_string_punctuation(&doc);
    test_multiline_basic_string_trim_start(&doc);

    // Read a boolean.
    // 論理値読取。
    test_boolean_true(&doc);
    test_boolean_false(&doc);
}

fn test_age(doc: &Document) {
    if let Some(age) = doc.get_i128_by_key("age") {
        println!("age = {}", age);
        // age = 40
    }
}
fn test_weight(doc: &Document) {
    if let Some(age) = doc.get_f64_by_key("weight") {
        println!("weight = {}", age);
        // weight = 93.5
    }
}
fn test_apple(doc: &Document) {
    // "pie"
    if let Some(apple) = doc.get_str_by_key("apple") {
        println!("apple = {}", apple);
        // apple = pie
    }
}
fn test_basic_string_empty(doc: &Document) {
    // ""
    if let Some(basic_string_empty) = doc.get_str_by_key("basic_string_empty") {
        println!("basic_string_empty = {}", basic_string_empty);
        // basic_string_empty =
    }
}
fn test_basic_string_escape_backslash(doc: &Document) {
    // "\\"
    if let Some(basic_string_escape_backslash) =
        doc.get_str_by_key("basic_string_escape_backslash")
    {
        println!(
            "basic_string_escape_backslash = {}",
            basic_string_escape_backslash
        );
        // basic_string_escape_backslash = \
    }
}
fn test_basic_string_escape_double_quotation(doc: &Document) {
    // "\""
    if let Some(basic_string_escape_double_quotation) =
        doc.get_str_by_key("basic_string_escape_double_quotation")
    {
        println!(
            "basic_string_escape_double_quotation = {}",
            basic_string_escape_double_quotation
        );
        // basic_string_escape_double_quotation = \
    }
}
fn test_basic_string_punctuation(doc: &Document) {
    // "., ={}[]'\"\\!?"
    if let Some(basic_string_punctuation) = doc.get_str_by_key("basic_string_punctuation") {
        println!("basic_string_punctuation = {}", basic_string_punctuation);
        // basic_string_punctuation = ., ={}[]'"\!?
    }
}
fn test_multiline_basic_string_letter(doc: &Document) {
    // """Hello,
    // world!!"""
    if let Some(multiline_basic_string_letter) =
        doc.get_str_by_key("multiline_basic_string_letter")
    {
        println!(
            "multiline_basic_string_letter = {}",
            multiline_basic_string_letter
        );
        // multiline_basic_string_letter = Hello,
        // world!!
    }
}
fn test_multiline_basic_string_punctuation(doc: &Document) {
    // """., ={}[]"'""\\
    // !?"""
    if let Some(multiline_basic_string_punctuation) =
        doc.get_str_by_key("multiline_basic_string_punctuation")
    {
        println!(
            "multiline_basic_string_punctuation = {}",
            multiline_basic_string_punctuation
        );
        // multiline_basic_string_punctuation = ., ={}[]"'""\
        // !?
    }
}
fn test_multiline_basic_string_trim_start(doc: &Document) {
    // """\
    //   The quick brown \
    //   fox jumps over \
    //   the lazy dog.\
    //   """
    if let Some(multiline_basic_string_trim_start) =
        doc.get_str_by_key("multiline_basic_string_trim_start")
    {
        println!(
            "multiline_basic_string_trim_start = {}",
            multiline_basic_string_trim_start
        );
        // multiline_basic_string_trim_start =
    }
}
fn test_boolean_true(doc: &Document) {
    if let Some(adult) = doc.get_bool_by_key("adult") {
        println!("adult = {}", adult);
        // adult = true
    }
}
fn test_boolean_false(doc: &Document) {
    if let Some(student) = doc.get_bool_by_key("student") {
        println!("student = {}", student);
        // student = false
    }
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
    * [ ] Plain.
  * [ ] `'''abc'''` - multi-line literal string.
    * [ ] Plain.
