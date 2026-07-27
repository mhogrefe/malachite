// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::string_is_subset;
use malachite_base::test_util::generators::string_gen;
use malachite_float::test_util::common::{parse_hex_string, to_hex_string};
use malachite_float::test_util::generators::{float_gen, float_gen_var_12};
use malachite_float::{ComparableFloat, Float};
use std::collections::HashMap;

#[test]
fn test_serde() {
    let test = |s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(serde_json::to_string(&x).unwrap(), out);
        assert_eq!(
            to_hex_string(&serde_json::from_str::<Float>(out).unwrap()),
            s_hex
        );
        // `ComparableFloat` adds no data of its own, so it is written exactly the same way.
        let c = ComparableFloat(x);
        assert_eq!(serde_json::to_string(&c).unwrap(), out);
        assert_eq!(serde_json::from_str::<ComparableFloat>(out).unwrap(), c);
    };
    // The specials and the zeros carry no precision, so they are written as `Display` writes them.
    test("NaN", "\"NaN\"");
    test("Infinity", "\"Infinity\"");
    test("-Infinity", "\"-Infinity\"");
    test("0x0.0", "\"0x0.0\"");
    test("-0x0.0", "\"-0x0.0\"");
    // Everything else carries its precision after a `#`, which is what makes the round trip
    // preserve it.
    test("0x1.0#1", "\"0x1.0#1\"");
    test("0x1.8#2", "\"0x1.8#2\"");
    test("-0x1.8#4", "\"-0x1.8#4\"");
    test("0xff.0#8", "\"0xff.0#8\"");
    test("0x0.555555555555555556#70", "\"0x0.555555555555555556#70\"");
    // A large exponent is written in the same scientific form the hexadecimal `Display` uses.
    test("0x1.0E+25#1", "\"0x1.0E+25#1\"");
}

#[test]
fn serde_properties() {
    fn check(x: &Float) {
        let s = serde_json::to_string(x).unwrap();
        let y: Float = serde_json::from_str(&s).unwrap();
        assert!(y.is_valid());
        // Value, sign of zero, and precision all survive, which is what `ComparableFloat` compares.
        assert_eq!(ComparableFloat(y), ComparableFloat(x.clone()));
        // The encoding is the hexadecimal `ComparableFloat` string, wrapped in quotes.
        assert_eq!(s, format!("{:?}", to_hex_string(x)));
        // `ComparableFloat` is written the same way and round-trips too, which is what lets it be
        // used as a map key: the encoding is a string, so it is a valid key in formats that require
        // one.
        let c = ComparableFloat(x.clone());
        assert_eq!(serde_json::to_string(&c).unwrap(), s);
        assert_eq!(serde_json::from_str::<ComparableFloat>(&s).unwrap(), c);
        // The quote, the sign, the point, the `#`, the hexadecimal digits and `0x` prefix, the
        // `E+`/`E-` exponent, and the letters of `NaN` and `Infinity`.
        assert!(string_is_subset(&s, "\"-.#+0123456789abcdefxENIinty"));
    }
    float_gen().test_properties(|x| check(&x));
    // Extreme exponents and precisions, where the string is longest.
    float_gen_var_12().test_properties(|x| check(&x));

    // The use `ComparableFloat` exists for: a collection keyed by one.
    float_gen().test_properties(|x| {
        let m = HashMap::from([(ComparableFloat(x), 0u8)]);
        let s = serde_json::to_string(&m).unwrap();
        assert_eq!(
            serde_json::from_str::<HashMap<ComparableFloat, u8>>(&s).unwrap(),
            m
        );
    });

    // Deserializing an arbitrary string must fail rather than panic, and must never produce an
    // invalid `Float`.
    string_gen().test_properties(|s| {
        if let Ok(x) = serde_json::from_str::<Float>(&format!("{s:?}")) {
            assert!(x.is_valid());
        }
    });
}
