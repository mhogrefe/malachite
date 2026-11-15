// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::num::conversion::traits::{FromSciString, FromStringBase, ToStringBase};
use malachite_float::big_exponent_float::ComparableBigExponentFloat;
use malachite_float::test_util::big_exponent_float::big_exponent_float_to_float;
use malachite_float::test_util::generators::big_exponent_float_gen;
use malachite_float::{BigExponentFloat, Float};

#[test]
fn test_from_string_base() {
    let test = |base, s, out: Option<&str>, out_hex: Option<&str>| {
        let x = BigExponentFloat::from_string_base(base, s);
        assert_eq!(x.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!(x.map(|x| format!("{x:#x}")).as_deref(), out_hex);
    };
    test(16, "NaN", Some("NaN"), Some("NaN"));
    test(16, "-Infinity", Some("-Infinity"), Some("-Infinity"));
    test(16, "0x0.0", Some("0.0"), Some("0x0.0"));
    test(16, "-0x1.8", Some("-1.5"), Some("-0x1.8"));
    test(16, "1.8", Some("1.5"), Some("0x1.8"));
    test(16, "1.8E+2", Some("3.8e2"), Some("0x1.8E+2"));
    test(
        16,
        "0x1.0E+25000000000000000000",
        Some("2.4e30102999566398119521"),
        Some("0x1.0E+25000000000000000000"),
    );
    test(2, "0b1.1", Some("1.5"), Some("0x1.8"));
    test(2, "-1.1e-1", Some("-0.75"), Some("-0x0.c"));
    test(8, "0o1.4", Some("1.5"), Some("0x1.8"));
    test(4, "1.2", Some("1.5"), Some("0x1.8"));
    test(32, "1.g", Some("1.5"), Some("0x1.8"));
    test(32, "1.GE+1", Some("48.0"), Some("0x3.0E+1"));
    test(16, "12", Some("18.0"), Some("0x12.0"));
    test(16, ".8", Some("0.50"), Some("0x0.8"));
    // - in hexadecimal, an `E` with no sign after it is a digit
    test(16, "1.8E", Some("1.555"), Some("0x1.8e"));
    test(16, "1.8E2", Some("1.5552"), Some("0x1.8e2"));
    // - in binary, the exponent's sign is optional
    test(2, "1.1E10", Some("1.5e3"), Some("0x6.0E+2"));
    // - invalid strings
    test(16, "", None, None);
    test(16, ".", None, None);
    test(16, "1.g", None, None);
    test(16, "1.8.0", None, None);
    // - an exponent marker with no exponent
    test(2, "1.1E", None, None);
    test(2, "1.1E+", None, None);
    test(16, "0x", None, None);
    test(2, "0x1.0", None, None);
    test(16, "--1", None, None);
}

#[test]
#[should_panic]
fn from_string_base_fail_1() {
    BigExponentFloat::from_string_base(1, "1.5");
}

#[test]
#[should_panic]
fn from_string_base_fail_2() {
    BigExponentFloat::from_string_base(37, "1.5");
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

#[test]
fn test_from_string_base_non_power_of_2() {
    let test = |s: &str, base: u8, out: &str, out_hex: &str| {
        let x = BigExponentFloat::from_string_base(base, s).unwrap();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out, "{s} {base}");
        assert_eq!(format!("{x:?}"), out_hex, "{s} {base}");
    };
    test("1.5", 10, "1.5", "0x1.8");
    test("0.1", 10, "0.102", "0x0.1a");
    test("0.10", 10, "0.0996", "0x0.198");
    test("-2.5e-7", 10, "-2.496e-7", "-0x4.3E-6");
    test(
        "1e100000000000000000000",
        10,
        "9.8e99999999999999999999",
        "0x8.0E+83048202372184058696",
    );
    test(
        "-3.70e-100000000000000000000",
        10,
        "-3.698e-100000000000000000000",
        "-0x7.3cE-83048202372184058697",
    );
    test("1.1e1000000000", 10, "1.098e1000000000", "0x8.2E+830482023");
    test("0.0", 10, "0.0", "0x0.0");
    test("NaN", 10, "NaN", "NaN");
    test("0.1", 3, "0.38", "0x0.6");
    test(
        "2.1e+100000000000000000000",
        3,
        "7.4e47712125471966243729",
        "0x6.0E+39624062518028904536",
    );
    test("ff", 16, "255.0", "0xff.0");
    test(
        "a.b8e-40000000000",
        16,
        "6.211e-48164799306",
        "0xa.b8E-40000000000",
    );
    test("zz.z", 36, "1295.97", "0x50f.f8");
    test(
        "1.z1e+99999999999",
        36,
        "2.93461e155630250075",
        "0x1.0782E+129248125035",
    );

    let test_invalid = |s: &str, base: u8| {
        assert!(BigExponentFloat::from_string_base(base, s).is_none());
    };
    test_invalid("abc", 10);
}

#[test]
fn test_from_str() {
    let test = |s: &str, out: &str, out_hex: &str| {
        let x = BigExponentFloat::from_str(s).unwrap();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out, "{s}");
        assert_eq!(format!("{x:?}"), out_hex, "{s}");
    };
    test("1.5", "1.5", "0x1.8");
    test("0.1", "0.102", "0x0.1a");
    test("0.10", "0.0996", "0x0.198");
    test("-2.5e-7", "-2.496e-7", "-0x4.3E-6");
    test(
        "1e100000000000000000000",
        "9.8e99999999999999999999",
        "0x8.0E+83048202372184058696",
    );
    test(
        "-3.70e-100000000000000000000",
        "-3.698e-100000000000000000000",
        "-0x7.3cE-83048202372184058697",
    );
    test("1.1e1000000000", "1.098e1000000000", "0x8.2E+830482023");
    test("0.0", "0.0", "0x0.0");
    test("NaN", "NaN", "NaN");

    let test_invalid = |s: &str| {
        assert!(BigExponentFloat::from_str(s).is_err());
    };
    test_invalid("abc");
}

#[test]
fn from_string_base_non_power_of_2_properties() {
    big_exponent_float_gen().test_properties(|x| {
        for base in [3, 10, 36] {
            let s = x.to_string_base(base);
            let mut options = FromSciStringOptions::default();
            options.set_base(base);
            let y = BigExponentFloat::from_string_base(base, &s).unwrap();
            assert!(y.is_valid());
            // - agreement with `Float`, when both can represent the value
            if let Some(f) = Float::from_string_base(base, &s)
                && f.is_finite()
                && f != 0u32
                && big_exponent_float_to_float(&y).is_some()
            {
                assert_eq!(
                    ComparableBigExponentFloat(y.clone()),
                    ComparableBigExponentFloat(BigExponentFloat::from(f))
                );
            }
            assert_eq!(
                ComparableBigExponentFloat(y.clone()),
                ComparableBigExponentFloat(
                    BigExponentFloat::from_sci_string_with_options(&s, options).unwrap()
                )
            );
            if base == 10 {
                assert_eq!(
                    ComparableBigExponentFloat(y),
                    ComparableBigExponentFloat(BigExponentFloat::from_str(&s).unwrap())
                );
            }
        }
    });
}
