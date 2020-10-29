//! Document model.  
//! ドキュメント・モデル。  

use crate::model::layer210::LiteralValue;
use crate::model::layer225::Val;
use crate::model::layer230::Expression::Keyval;
use crate::model::{layer230::Expression, layer310::TomlDocument};
// use crate::util::type_of;
use chrono::prelude::{DateTime, Local, Utc};
use chrono::FixedOffset;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use num_traits::Num;
use std::fmt;

impl Default for TomlDocument {
    fn default() -> Self {
        TomlDocument {
            elements: Vec::new(),
        }
    }
}
impl TomlDocument {
    /// Right integer of `left = 123`.  
    /// キー・バリューの右の整数値。  
    ///
    /// from_str_radix() が使えるように num_traits::Num トレイトを付けます。
    /// s.parse() が使えるように std::str::FromStr トレイトを付けます。
    /// why を文字列表示できるように、 std::fmt::Display トレイトを付けます。
    ///
    /// 返り値を Result に変えたい。
    pub fn get_string_array_by_key(&self, key: &str) -> Result<Option<Vec<String>>, String> {
        if let Some(val) = self.get_val_by_key(key) {
            if let Keyval(_ws1, keyval, _ws2, _comment) = val {
                if keyval.key.to_string() == key.to_string() {
                    if let Val::Array(array) = &*keyval.val {
                        return Ok(Some(array.to_string_vector()));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Right of `left = right`.  
    /// キー・バリューの右値。  
    pub fn get_val_by_key(&self, key: &str) -> Option<&Expression> {
        // println!("[trace22]");
        for elem in &self.elements {
            match elem {
                Expression::HeaderOfArrayOfTable(_) => {
                    // TODO
                    // println!("[trace27]");
                }
                Expression::EmptyLine(_ws, _comment) => {
                    // println!("[trace33]");
                }
                Expression::Keyval(_ws1, keyval, _ws2, _comment) => {
                    if keyval.key.to_string() == key.to_string() {
                        /*
                        println!(
                            // "[trace36 Hit m.key={} key={}]",
                            m.key.to_string(),
                            key.to_string()
                        ); // In development.
                        */
                        return Some(elem);
                    } else {
                        /*
                        println!(
                            // "[trace44 Not found m.key={} key={}]",
                            m.key.to_string(),
                            key.to_string()
                        ); // In development.
                        */
                    }
                }
                Expression::HeaderOfTable(_) => {
                    // TODO
                    // println!("[trace45]");
                }
            }
        }
        None
    }

    /// Right of `left = right`.  
    /// キー・バリューの右値。  
    pub fn get_literal_string_by_key(&self, key: &str) -> Option<&LiteralValue> {
        if let Some(doc_elm) = self.get_val_by_key(key) {
            if let Keyval(_ws1, keyval, _ws2, _comment) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let Val::LiteralValue(literal_value) = &*keyval.val {
                        return Some(literal_value);
                    }
                }
            }
        }
        None
    }

    /// Contains key.  
    /// キーを含むか？  
    pub fn contains_key(&self, key: &str) -> bool {
        if let Some(doc_elm) = self.get_val_by_key(key) {
            if let Keyval(_ws1, keyval, _ws2, _comment) = doc_elm {
                return keyval.key.to_string() == key.to_string();
            }
        }
        false
    }

    /*
    /// TODO float にしか使えないので結局意味がない。 f64::NAN の取り扱いを条件分岐で外せないか？
    pub fn get_number_by_key<T: std::str::FromStr + num_traits::Num>(&self, key: &str) -> Option<T>
    where
        <T as num_traits::Num>::FromStrRadixErr: std::fmt::Display,
        T: num_traits::float::FloatCore,
    {
        if type_of(key).starts_with("&f") {
            return self.get_float_by_key(key);
        } else {
            return self.get_int_by_key(key);
        }
    }
    */

    /// Right integer of `left = 123`.  
    /// キー・バリューの右の整数値。  
    #[deprecated(
        since = "0.1.20",
        note = "Please change to the tomboy_toml_dom::model::layer310::toml_document::get_i128_by_key_v2() method instead"
    )]
    pub fn get_i128_by_key(&self, key: &str) -> Option<i128> {
        return self.get_int_by_key(key);
    }
    pub fn get_i128_by_key_v2(&self, key: &str) -> Result<Option<i128>, String> {
        return self.get_int_by_key_v2(key);
    }

    /// Right integer of `left = 123`.  
    /// キー・バリューの右の整数値。  
    #[deprecated(
        since = "0.1.20",
        note = "Please change to the tomboy_toml_dom::model::layer310::toml_document::get_isize_by_key_v2() method instead"
    )]
    pub fn get_isize_by_key(&self, key: &str) -> Option<isize> {
        return self.get_int_by_key(key);
    }
    pub fn get_isize_by_key_v2(&self, key: &str) -> Result<Option<isize>, String> {
        return self.get_int_by_key_v2(key);
    }

    /// Right integer of `left = 123`.  
    /// キー・バリューの右の整数値。  
    #[deprecated(
        since = "0.1.20",
        note = "Please change to the tomboy_toml_dom::model::layer310::toml_document::get_u128_by_key_v2() method instead"
    )]
    pub fn get_u128_by_key(&self, key: &str) -> Option<u128> {
        return self.get_int_by_key(key);
    }
    pub fn get_u128_by_key_v2(&self, key: &str) -> Result<Option<u128>, String> {
        return self.get_int_by_key_v2(key);
    }

    /// Right integer of `left = 123`.  
    /// キー・バリューの右の整数値。  
    #[deprecated(
        since = "0.1.20",
        note = "Please change to the tomboy_toml_dom::model::layer310::toml_document::get_usize_by_key_v2() method instead"
    )]
    pub fn get_usize_by_key(&self, key: &str) -> Option<usize> {
        return self.get_int_by_key(key);
    }
    pub fn get_usize_by_key_v2(&self, key: &str) -> Result<Option<usize>, String> {
        return self.get_int_by_key_v2(key);
    }

    /// WIP. まだ `.` をパースできていません。  
    ///
    /// Right integer of `left = 1.2`.  
    /// キー・バリューの右の整数値。  
    #[deprecated(
        since = "0.1.20",
        note = "Please change to the tomboy_toml_dom::model::layer310::toml_document::get_f64_by_key_v2() method instead"
    )]
    pub fn get_f64_by_key(&self, key: &str) -> Option<f64> {
        return self.get_float_by_key(key);
    }
    pub fn get_f64_by_key_v2(&self, key: &str) -> Result<Option<f64>, String> {
        return self.get_float_by_key_v2(key);
    }

    /// Right integer of `left = 123`.  
    /// キー・バリューの右の整数値。  
    ///
    /// from_str_radix() が使えるように num_traits::Num トレイトを付けます。
    /// s.parse() が使えるように std::str::FromStr トレイトを付けます。
    /// why を文字列表示できるように、 std::fmt::Display トレイトを付けます。
    ///
    /// 返り値を Result に変えたい。
    #[deprecated(
        since = "0.1.20",
        note = "Please change to the tomboy_toml_dom::model::layer310::toml_document::get_int_by_key_v2() method instead"
    )]
    pub fn get_int_by_key<T: Num + std::str::FromStr>(&self, key: &str) -> Option<T>
    where
        <T as num_traits::Num>::FromStrRadixErr: std::fmt::Display,
    {
        if let Some(doc_elm) = self.get_val_by_key(key) {
            if let Keyval(_ws1, keyval, _ws2, _comment) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let Val::LiteralValue(literal_value) = &*keyval.val {
                        // アンダースコアは除去しないと変換できない。
                        let s = literal_value.to_string().replace("_", "");

                        // 10進数ではないかも知れない。
                        let base_number = if s.starts_with("0b") {
                            2
                        } else if s.starts_with("0o") {
                            8
                        } else if s.starts_with("0x") {
                            16
                        } else {
                            10
                        };

                        if 10 != base_number {
                            // 頭の `0x` は除去しないと変換できない。
                            let s2 = &s[2..];
                            match T::from_str_radix(s2, base_number) {
                                Ok(n) => return Some(n),
                                Err(why) => panic!("{}", why),
                            };
                        }

                        match s.parse() {
                            Ok(n) => return Some(n),
                            Err(_why) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// Right integer of `left = 123`.  
    /// キー・バリューの右の整数値。  
    ///
    /// from_str_radix() が使えるように num_traits::Num トレイトを付けます。
    /// s.parse() が使えるように std::str::FromStr トレイトを付けます。
    /// why を文字列表示できるように、 std::fmt::Display トレイトを付けます。
    ///
    /// 返り値を Result に変えたい。
    pub fn get_int_by_key_v2<T: Num + std::str::FromStr>(
        &self,
        key: &str,
    ) -> Result<Option<T>, String>
    where
        <T as num_traits::Num>::FromStrRadixErr: std::fmt::Display,
        <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        if let Some(doc_elm) = self.get_val_by_key(key) {
            if let Keyval(_ws1, keyval, _ws2, _comment) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let Val::LiteralValue(literal_value) = &*keyval.val {
                        // アンダースコアは除去しないと変換できない。
                        let s = literal_value.to_string().replace("_", "");

                        // 10進数ではないかも知れない。
                        let base_number = if s.starts_with("0b") {
                            2
                        } else if s.starts_with("0o") {
                            8
                        } else if s.starts_with("0x") {
                            16
                        } else {
                            10
                        };

                        if 10 != base_number {
                            // 頭の `0x` は除去しないと変換できない。
                            let s2 = &s[2..];
                            match T::from_str_radix(s2, base_number) {
                                Ok(n) => return Ok(Some(n)),
                                Err(why) => return Err(format!("{}", why)),
                            };
                        }

                        match s.parse() {
                            Ok(n) => return Ok(Some(n)),
                            Err(why) => return Err(format!("{}", why)),
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// WIP. まだ `.` をパースできていません。  
    ///
    /// Right integer of `left = 1.2`.  
    /// キー・バリューの右の整数値。  
    #[deprecated(
        since = "0.1.20",
        note = "Please change to the tomboy_toml_dom::model::layer310::toml_document::get_float_by_key_v2() method instead"
    )]
    pub fn get_float_by_key<T: num_traits::float::FloatCore + Num + std::str::FromStr>(
        &self,
        key: &str,
    ) -> Option<T> {
        if let Some(doc_elm) = self.get_val_by_key(key) {
            if let Keyval(_ws1, keyval, _ws2, _comment) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let Val::LiteralValue(literal_value) = &*keyval.val {
                        // アンダースコアは除去しないと変換できない。
                        let s = literal_value.to_string().replace("_", "");

                        match s.as_str() {
                            "nan" => {
                                return Some(T::nan());
                            }
                            "+nan" => {
                                return Some(T::nan());
                            }
                            "-nan" => {
                                return Some(-T::nan());
                            }
                            _ => {}
                        }

                        match s.parse() {
                            Ok(n) => return Some(n),
                            Err(_why) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// WIP. まだ `.` をパースできていません。  
    ///
    /// Right integer of `left = 1.2`.  
    /// キー・バリューの右の整数値。  
    pub fn get_float_by_key_v2<T: num_traits::float::FloatCore + Num + std::str::FromStr>(
        &self,
        key: &str,
    ) -> Result<Option<T>, String>
    where
        <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        if let Some(doc_elm) = self.get_val_by_key(key) {
            if let Keyval(_ws1, keyval, _ws2, _comment) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let Val::LiteralValue(literal_value) = &*keyval.val {
                        // アンダースコアは除去しないと変換できない。
                        let s = literal_value.to_string().replace("_", "");

                        match s.as_str() {
                            "nan" => {
                                return Ok(Some(T::nan()));
                            }
                            "+nan" => {
                                return Ok(Some(T::nan()));
                            }
                            "-nan" => {
                                return Ok(Some(-T::nan()));
                            }
                            _ => {}
                        }

                        match s.parse() {
                            Ok(n) => return Ok(Some(n)),
                            Err(why) => return Err(format!("{}", why)),
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    #[deprecated(
        since = "0.1.10",
        note = "Please use the tomboy_toml_dom::model::layer310::toml_document::get_string_by_key() method instead"
    )]
    pub fn get_str_by_key(&self, _key: &str) -> Option<&str> {
        panic!("Obsoleted. Please use the tomboy_toml_dom::model::layer310::document::get_string_by_key() method instead.")
    }

    /// Right string of `left = "abc"`.  
    /// キー・バリューの右の文字列。  
    pub fn get_string_by_key(&self, key: &str) -> Option<String> {
        if let Some(doc_elm) = self.get_val_by_key(key) {
            if let Keyval(_ws1, keyval, _ws2, _comment) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    match &*keyval.val {
                        Val::BasicString(basic_string) => {
                            return Some(basic_string.to_string());
                        }
                        Val::LiteralString(literal_string) => {
                            return Some(literal_string.to_string());
                        }
                        _ => {}
                    }
                }
            }
        }
        None
    }
    /// For this library developer.
    pub fn get_debug_string_by_key(&self, key: &str) -> String {
        if let Some(doc_elm) = self.get_val_by_key(key) {
            if let Keyval(_ws1, keyval, _ws2, _comment) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    match &*keyval.val {
                        Val::BasicString(basic_string) => {
                            return basic_string.to_debug_string();
                        }
                        Val::LiteralString(literal_string) => {
                            return literal_string.to_debug_string();
                        }
                        _ => {}
                    }
                }
            }
        }
        "".to_string()
    }

    /// Right boolean of `left = true`.  
    /// キー・バリューの右の論理値。  
    pub fn get_bool_by_key(&self, key: &str) -> Option<bool> {
        if let Some(doc_elm) = self.get_val_by_key(key) {
            if let Keyval(_ws1, keyval, _ws2, _comment) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let Val::LiteralValue(literal_value) = &*keyval.val {
                        match literal_value.to_string().parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// DateTime. UTC.  
    /// 日付と時刻。協定世界時。  
    pub fn get_datetime_utc_by_key(&self, key: &str) -> Option<DateTime<Utc>> {
        if let Some(doc_elm) = self.get_val_by_key(key) {
            if let Keyval(_ws1, keyval, _ws2, _comment) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let Val::LiteralValue(literal_value) = &*keyval.val {
                        match format!("{}", literal_value).to_string().parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// DateTime. Local.  
    /// 日付と時刻。ローカル時。  
    pub fn get_datetime_local_by_key(&self, key: &str) -> Option<DateTime<Local>> {
        if let Some(doc_elm) = self.get_val_by_key(key) {
            if let Keyval(_ws1, keyval, _ws2, _comment) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let Val::LiteralValue(literal_value) = &*keyval.val {
                        match format!("{}", literal_value).to_string().parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// DateTime. Fixed offset.  
    /// 日付と時刻。オフセット。  
    pub fn get_datetime_fixed_offset_by_key(&self, key: &str) -> Option<DateTime<FixedOffset>> {
        if let Some(doc_elm) = self.get_val_by_key(key) {
            if let Keyval(_ws1, keyval, _ws2, _comment) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let Val::LiteralValue(literal_value) = &*keyval.val {
                        match format!("{}", literal_value).to_string().parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// DateTime. Naive.  
    /// 日付と時刻。ナイーブ。  
    pub fn get_naive_datetime_by_key(&self, key: &str) -> Option<NaiveDateTime> {
        if let Some(doc_elm) = self.get_val_by_key(key) {
            if let Keyval(_ws1, keyval, _ws2, _comment) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let Val::LiteralValue(literal_value) = &*keyval.val {
                        match format!("{}", literal_value).to_string().parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// Date. Naive.  
    /// 日付。ナイーブ。  
    pub fn get_naive_date_by_key(&self, key: &str) -> Option<NaiveDate> {
        if let Some(doc_elm) = self.get_val_by_key(key) {
            if let Keyval(_ws1, keyval, _ws2, _comment) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let Val::LiteralValue(literal_value) = &*keyval.val {
                        match format!("{}", literal_value).to_string().parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// Time. Naive.  
    /// 日時。ナイーブ。  
    pub fn get_naive_time_by_key(&self, key: &str) -> Option<NaiveTime> {
        if let Some(doc_elm) = self.get_val_by_key(key) {
            if let Keyval(_ws1, keyval, _ws2, _comment) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let Val::LiteralValue(literal_value) = &*keyval.val {
                        match format!("{}", literal_value).to_string().parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    pub fn push_element(&mut self, m: &Expression) {
        self.elements.push(m.clone());
    }
    pub fn to_debug_string(&self) -> String {
        format!("{:?}", self)
    }
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}
impl fmt::Display for TomlDocument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for elem in &self.elements {
            buf.push_str(&format!("{}", elem));
        }
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for TomlDocument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for elem in &self.elements {
            buf.push_str(&format!("{:?}", elem));
        }
        write!(f, "{}", buf)
    }
}
