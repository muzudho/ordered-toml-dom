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
age = 40
int_max = 2147483647
int_min = -2147483648
weight = 93.5

# hexadecimal with prefix `0x`
hex1 = 0xDEADBEEF
hex2 = 0xdeadbeef
hex3 = 0xdead_beef

# octal with prefix `0o`
oct1 = 0o01234567
oct2 = 0o755

# binary with prefix `0b`
bin1 = 0b11010110

# basic string
apple = "pie"
basic_string_empty = ""
basic_string_escape_backslash = "\\"
basic_string_escape_double_quotation = "\""
basic_string_letter = "Hello, world!!"
basic_string_punctuation = "., ={}[]'\"\\!?"
basic_string_tab = "a\tb"

multiline_basic_string_letter = """Hello,
world!!"""
multiline_basic_string_punctuation = """., ={}[]"'""\\
!?"""
multiline_basic_string_trim_start = """\
  The quick brown \
  fox jumps over \
  the lazy dog.\
  """
multiline_basic_string_escape_double_quotation = """
\\
"""
multiline_basic_string_tab = """
a\tb
"""

literal_string_empty = ''
literal_string_letter = 'Hello, world!!'
literal_string_punctuation = '., ={}[]"\!?'

multiline_literal_string_letter = '''Hello,
world!!'''
multiline_literal_string_punctuation = '''., ={}[]'"\
!?'''
multiline_literal_string_first_newline_is_trimmed = '''
The first newline is
trimmed in raw strings.
All other whitespace
is preserved.
'''

adult = true
student = false

dob = 1979-05-27T07:32:00-08:00
```

## TODO

* [ ] Literal
  * [x] Parsing a literal containing dots. Example: `3.14`.  
      ドットを含むリテラル文字列の解析。例： `3.14`。
  * [ ] Literal numbers...
    * [x] `0b` - binary.
    * [x] `0o` - oct.
    * [x] `0x` - hex.
    * [ ] `_` - space.
    * [ ] `nan` - Not a number.
    * [ ] `+nan` - Not a number.
    * [ ] `-nan` - Not a number.
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
    * [ ]  UTC
      * [x] `1979-05-27T07:32:00-08:00`.
