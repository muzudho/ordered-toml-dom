extern crate tomboy_toml_dom;

use tomboy_toml_dom::Toml;

fn main() {
    // Read a toml.
    // Toml読取。
    let doc = Toml::from_file("./resource/example.toml");

    // Read a number.
    // 数値読取。
    assert_eq!(doc.get_int_by_key("age"), Some(40i128));
    assert_eq!(doc.get_int_by_key("age"), Some(40isize));
    assert_eq!(doc.get_int_by_key("age"), Some(40u128));
    assert_eq!(doc.get_int_by_key("age"), Some(40usize));
    assert_eq!(doc.get_float_by_key("weight"), Some(93.5f32));
    assert_eq!(doc.get_float_by_key("weight"), Some(93.5f64));

    assert_eq!(doc.get_int_by_key("i32_max"), Some(2147483647i32));
    assert_eq!(doc.get_int_by_key("i32_min"), Some(-2147483648i32));

    assert_eq!(
        doc.get_int_by_key("i128_max"),
        Some(170_141_183_460_469_231_731_687_303_715_884_105_727i128)
    );
    assert_eq!(
        doc.get_int_by_key("i128_min"),
        Some(-170_141_183_460_469_231_731_687_303_715_884_105_728i128)
    );
    assert_eq!(
        doc.get_int_by_key("u128_max"),
        Some(340_282_366_920_938_463_463_374_607_431_768_211_455u128)
    );

    assert_eq!(doc.get_int_by_key("hex1"), Some(0xDEADBEEFi128));
    assert_eq!(doc.get_int_by_key("hex2"), Some(0xdeadbeefi128));
    assert_eq!(doc.get_int_by_key("hex3"), Some(0xdead_beefi128));
    assert_eq!(doc.get_int_by_key("oct1"), Some(0o01234567));
    assert_eq!(doc.get_int_by_key("oct2"), Some(0o755));
    assert_eq!(doc.get_int_by_key("bin1"), Some(0b11010110));
    assert_eq!(doc.get_float_by_key("float1"), Some(1.0));
    assert_eq!(doc.get_float_by_key("float2"), Some(3.1415));
    assert_eq!(doc.get_float_by_key("float3"), Some(-0.01));
    assert_eq!(doc.get_float_by_key("float4"), Some(5e+22));
    assert_eq!(doc.get_float_by_key("float5"), Some(1e06));
    assert_eq!(doc.get_float_by_key("float6"), Some(-2E-2));
    assert_eq!(doc.get_float_by_key("float7"), Some(6.626e-34));
    assert_eq!(doc.get_float_by_key("float8"), Some(224_617.445_991_228));
    assert_eq!(doc.get_float_by_key("infinite1"), Some(f64::INFINITY));
    assert_eq!(doc.get_float_by_key("infinite2"), Some(f64::INFINITY));
    assert_eq!(doc.get_float_by_key("infinite3"), Some(-f64::INFINITY));
    assert!(if let Some(n) = doc.get_float_by_key::<f64>("not1") {
        n.is_nan() && n.is_sign_positive()
    } else {
        false
    });
    assert!(if let Some(n) = doc.get_float_by_key::<f64>("not2") {
        n.is_nan() && n.is_sign_positive()
    } else {
        false
    });
    assert!(if let Some(n) = doc.get_float_by_key::<f64>("not3") {
        n.is_nan() && n.is_sign_negative()
    } else {
        false
    });
}
