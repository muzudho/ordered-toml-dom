extern crate tomboy_toml_dom;

use tomboy_toml_dom::Toml;

fn main() {
    // Read a toml.
    // Toml読取。
    let doc = Toml::from_file("./resource/example.toml");

    // Read a number.
    // 数値読取。
    assert_eq!(doc.get_int_by_key_v2("age"), Ok(Some(40i128)));
    assert_eq!(doc.get_int_by_key_v2("age"), Ok(Some(40isize)));
    assert_eq!(doc.get_int_by_key_v2("age"), Ok(Some(40u128)));
    assert_eq!(doc.get_int_by_key_v2("age"), Ok(Some(40usize)));
    assert_eq!(doc.get_float_by_key_v2("weight"), Ok(Some(93.5f32)));
    assert_eq!(doc.get_float_by_key_v2("weight"), Ok(Some(93.5f64)));

    assert_eq!(doc.get_int_by_key_v2("i32_max"), Ok(Some(2147483647i32)));
    assert_eq!(doc.get_int_by_key_v2("i32_min"), Ok(Some(-2147483648i32)));

    assert_eq!(
        doc.get_int_by_key_v2("i128_max"),
        Ok(Some(
            170_141_183_460_469_231_731_687_303_715_884_105_727i128
        ))
    );
    assert_eq!(
        doc.get_int_by_key_v2("i128_min"),
        Ok(Some(
            -170_141_183_460_469_231_731_687_303_715_884_105_728i128
        ))
    );
    assert_eq!(
        doc.get_int_by_key_v2("u128_max"),
        Ok(Some(
            340_282_366_920_938_463_463_374_607_431_768_211_455u128
        ))
    );

    assert_eq!(doc.get_int_by_key_v2("hex1"), Ok(Some(0xDEADBEEFi128)));
    assert_eq!(doc.get_int_by_key_v2("hex2"), Ok(Some(0xdeadbeefi128)));
    assert_eq!(doc.get_int_by_key_v2("hex3"), Ok(Some(0xdead_beefi128)));
    assert_eq!(doc.get_int_by_key_v2("oct1"), Ok(Some(0o01234567)));
    assert_eq!(doc.get_int_by_key_v2("oct2"), Ok(Some(0o755)));
    assert_eq!(doc.get_int_by_key_v2("bin1"), Ok(Some(0b11010110)));
    assert_eq!(doc.get_float_by_key_v2("float1"), Ok(Some(1.0)));
    assert_eq!(doc.get_float_by_key_v2("float2"), Ok(Some(3.1415)));
    assert_eq!(doc.get_float_by_key_v2("float3"), Ok(Some(-0.01)));
    assert_eq!(doc.get_float_by_key_v2("float4"), Ok(Some(5e+22)));
    assert_eq!(doc.get_float_by_key_v2("float5"), Ok(Some(1e06)));
    assert_eq!(doc.get_float_by_key_v2("float6"), Ok(Some(-2E-2)));
    assert_eq!(doc.get_float_by_key_v2("float7"), Ok(Some(6.626e-34)));
    assert_eq!(
        doc.get_float_by_key_v2("float8"),
        Ok(Some(224_617.445_991_228))
    );
    assert_eq!(
        doc.get_float_by_key_v2("infinite1"),
        Ok(Some(f64::INFINITY))
    );
    assert_eq!(
        doc.get_float_by_key_v2("infinite2"),
        Ok(Some(f64::INFINITY))
    );
    assert_eq!(
        doc.get_float_by_key_v2("infinite3"),
        Ok(Some(-f64::INFINITY))
    );
    assert!(
        if let Ok(Some(n)) = doc.get_float_by_key_v2::<f64>("not1") {
            n.is_nan() && n.is_sign_positive()
        } else {
            false
        }
    );
    assert!(
        if let Ok(Some(n)) = doc.get_float_by_key_v2::<f64>("not2") {
            n.is_nan() && n.is_sign_positive()
        } else {
            false
        }
    );
    assert!(
        if let Ok(Some(n)) = doc.get_float_by_key_v2::<f64>("not3") {
            n.is_nan() && n.is_sign_negative()
        } else {
            false
        }
    );
}
