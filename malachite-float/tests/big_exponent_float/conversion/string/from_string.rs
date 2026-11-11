// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{FromStringBase, ToStringBase};
use malachite_float::BigExponentFloat;
use malachite_float::test_util::generators::big_exponent_float_gen;

#[test]
fn test_from_string_base() {
    let test = |base, s, out: Option<&str>| {
        assert_eq!(
            BigExponentFloat::from_string_base(base, s)
                .map(|x| format!("{x:#x}"))
                .as_deref(),
            out
        );
    };
    test(16, "NaN", Some("NaN"));
    test(16, "-Infinity", Some("-Infinity"));
    test(16, "0x0.0", Some("0x0.0"));
    test(16, "-0x1.8", Some("-0x1.8"));
    test(16, "1.8", Some("0x1.8"));
    test(16, "1.8E+2", Some("0x1.8E+2"));
    test(
        16,
        "0x1.0E+25000000000000000000",
        Some("0x1.0E+25000000000000000000"),
    );
    test(2, "0b1.1", Some("0x1.8"));
    test(2, "-1.1e-1", Some("-0x0.c"));
    test(8, "0o1.4", Some("0x1.8"));
    test(4, "1.2", Some("0x1.8"));
    test(32, "1.g", Some("0x1.8"));
    test(32, "1.GE+1", Some("0x3.0E+1"));
    test(16, "12", Some("0x12.0"));
    test(16, ".8", Some("0x0.8"));
    // - in hexadecimal, an `E` with no sign after it is a digit
    test(16, "1.8E", Some("0x1.8e"));
    test(16, "1.8E2", Some("0x1.8e2"));
    // - in binary, the exponent's sign is optional
    test(2, "1.1E10", Some("0x6.0E+2"));
    // - invalid strings
    test(16, "", None);
    test(16, ".", None);
    test(16, "1.g", None);
    test(16, "1.8.0", None);
    // - an exponent marker with no exponent
    test(2, "1.1E", None);
    test(2, "1.1E+", None);
    test(16, "0x", None);
    test(2, "0x1.0", None);
    test(16, "--1", None);
}

#[test]
#[should_panic]
fn from_string_base_fail() {
    BigExponentFloat::from_string_base(10, "1.5");
}

#[test]
fn from_string_base_properties() {
    big_exponent_float_gen().test_properties(|x| {
        for base in [2, 4, 8, 16, 32] {
            let s = x.to_string_base(base);
            let y = BigExponentFloat::from_string_base(base, &s).unwrap();
            assert!(y.is_valid());
            assert_eq!(format!("{y:#x}"), format!("{x:#x}"));
            let y =
                BigExponentFloat::from_string_base(base, &x.to_string_base_upper(base)).unwrap();
            assert_eq!(format!("{y:#x}"), format!("{x:#x}"));
        }
        for (base, s) in [(2, format!("{x:#b}")), (8, format!("{x:#o}")), (16, format!("{x:#x}"))] {
            let y = BigExponentFloat::from_string_base(base, &s).unwrap();
            assert_eq!(format!("{y:#x}"), format!("{x:#x}"));
        }
    });
}
