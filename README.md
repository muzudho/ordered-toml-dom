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

use tomboy_toml_dom::Toml;

fn main() {
    // Read a toml.
    // Toml読取。
    let toml_file = "./resource/example.toml";
    let doc = Toml::from_file(toml_file);

    // Read a number.
    // 数値読取。
    if let Some(age) = doc.get_i128_by_key("age") {
        println!("age = {}", age);
        // age = 40
    }
    if let Some(age) = doc.get_f64_by_key("weight") {
        println!("weight = {}", age);
        // weight = 93.5
    }

    // WIP. Read a string.
    // 作業中。 文字列読取。
    //
    // "pie"
    if let Some(apple) = doc.get_str_by_key("apple") {
        println!("apple = {}", apple);
        // apple = pie
    }
    // ""
    if let Some(double_quoted_empty) = doc.get_str_by_key("double_quoted_empty") {
        println!("double_quoted_empty = {}", double_quoted_empty);
        // double_quoted_empty =
    }
    // "\\"
    if let Some(double_quoted_escape_backslash) =
        doc.get_str_by_key("double_quoted_escape_backslash")
    {
        println!(
            "double_quoted_escape_backslash = {}",
            double_quoted_escape_backslash
        );
        // double_quoted_escape_backslash = \
    }
    // "\""
    if let Some(double_quoted_escape_double_quotation) =
        doc.get_str_by_key("double_quoted_escape_double_quotation")
    {
        println!(
            "double_quoted_escape_double_quotation = {}",
            double_quoted_escape_double_quotation
        );
        // double_quoted_escape_double_quotation = \
    }
    // "., ={}[]'\"\\!?"
    if let Some(double_quoted_punctuation) = doc.get_str_by_key("double_quoted_punctuation") {
        println!("double_quoted_punctuation = {}", double_quoted_punctuation);
        // double_quoted_punctuation = ., ={}[]'"\!?
    }
    // """Hello,
    // world!!"""
    if let Some(triple_double_quoted_letter) = doc.get_str_by_key("triple_double_quoted_letter") {
        println!(
            "triple_double_quoted_letter = {}",
            triple_double_quoted_letter
        );
        // triple_double_quoted_letter = Hello,
        // world!!
    }

    // Read a boolean.
    // 論理値読取。
    if let Some(adult) = doc.get_bool_by_key("adult") {
        println!("adult = {}", adult);
        // adult = true
    }
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
  * [x] `"abc"` - Basic strings.
    * [x] Plain.
    * [x] `\` Escape.
  * [ ] `"""abc"""` - Multi-line basic strings.
    * [x] Plain.
    * [ ] Ending backslash to automatically trim.
  * [ ] `'abc'` - Literal strings.
    * [ ] Plain.
  * [ ] `'''abc'''` - multi-line literal strings.
    * [ ] Plain.
