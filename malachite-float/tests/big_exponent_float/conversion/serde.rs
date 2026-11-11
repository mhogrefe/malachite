// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::big_exponent_float_power_of_2;
use malachite_float::test_util::generators::big_exponent_float_gen;
use malachite_nz::integer::Integer;

#[test]
fn test_serde() {
    let test = |x: BigExponentFloat, out: &str| {
        let s = serde_json::to_string(&x).unwrap();
        assert_eq!(s, out);
        let y: BigExponentFloat = serde_json::from_str(&s).unwrap();
        assert_eq!(format!("{y:#x}"), format!("{x:#x}"));
    };
    test(BigExponentFloat::from(1.5f64), "\"0x1.8\"");
    test(BigExponentFloat::from(-1.5f64), "\"-0x1.8\"");
    test(BigExponentFloat::from(f64::NAN), "\"NaN\"");
    test(
        big_exponent_float_power_of_2(Integer::from(1u64 << 40)),
        "\"0x1.0E+274877906944\"",
    );
    assert!(serde_json::from_str::<BigExponentFloat>("\"0x1.g\"").is_err());
}

#[test]
fn serde_properties() {
    big_exponent_float_gen().test_properties(|x| {
        let s = serde_json::to_string(&x).unwrap();
        let y: BigExponentFloat = serde_json::from_str(&s).unwrap();
        assert!(y.is_valid());
        assert_eq!(format!("{y:#x}"), format!("{x:#x}"));
    });
}
