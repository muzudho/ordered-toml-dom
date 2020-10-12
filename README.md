# Tomboy toml dom

For those who are struggling with Rust's cool syntax, our goal is to provide a TOML parser that's as easy as pointing to a menu and eating fast food.  
Rustのイケてる構文に難儀している人のために、メニューを指差してファーストフードを食べるぐらい簡単な操作のTOMLパーサーを提供することを目標とします。  

It's a tryal and error process. Specifications will change.  
試行錯誤中です。 仕様はコロコロ変わるでしょう。  

Tomboy is a pun.  
トムボーイ（おてんば娘）は語呂合わせです。  

* References
  * [Developer's blog(開発者ブログ)](https://crieit.net/drafts/5f8094a14a0cf/resume)  
  * [TOML parsing（TOMLの構文解析）](https://crieit.net/posts/TOML-parsing-TOML)

## Run (実行)

Take a look at the repository.  
リポジトリを見てください。  

```shell
cargo run --example comment
cargo run --example cover
cargo run --example example
cargo run --example inline_table
cargo run --example key_value_int
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
    if let Some(age) = doc.get_int128_by_key("age") {
        println!("age = {}", age);
        // age = 40
    }
}
```
