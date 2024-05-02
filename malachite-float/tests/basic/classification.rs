// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_float::test_util::common::{parse_hex_string, to_hex_string};
use malachite_float::test_util::generators::float_gen;
use malachite_float::{ComparableFloatRef, Float};
use std::num::FpCategory;

#[test]
fn test_is_nan() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(x.is_nan(), out);
        assert_eq!(rug::Float::try_from(&x).unwrap().is_nan(), out);
    };
    test("NaN", "NaN", true);
    test("Infinity", "Infinity", false);
    test("-Infinity", "-Infinity", false);
    test("0.0", "0x0.0", false);
    test("-0.0", "-0x0.0", false);

    test("1.0", "0x1.0#1", false);
    test("2.0", "0x2.0#1", false);
    test("0.5", "0x0.8#1", false);
    test("0.33333333333333331", "0x0.55555555555554#53", false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", false);

    test("-1.0", "-0x1.0#1", false);
    test("-2.0", "-0x2.0#1", false);
    test("-0.5", "-0x0.8#1", false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", false);
}

#[test]
fn is_nan_properties() {
    float_gen().test_properties(|x| {
        let is_nan = x.is_nan();
        assert_eq!(
            is_nan,
            ComparableFloatRef(&x) == ComparableFloatRef(&Float::NAN)
        );
        assert_eq!(is_nan, x.classify() == FpCategory::Nan);
        assert_eq!(is_nan, !x.is_finite() && !x.is_infinite());
        assert_eq!(is_nan, rug::Float::try_from(&x).unwrap().is_nan());
    });

    primitive_float_gen::<f64>().test_properties(|x| {
        assert_eq!(Float::from(x).is_nan(), x.is_nan());
    });
}

#[test]
fn test_is_finite() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(x.is_finite(), out);
        assert_eq!(rug::Float::try_from(&x).unwrap().is_finite(), out);
    };
    test("NaN", "NaN", false);
    test("Infinity", "Infinity", false);
    test("-Infinity", "-Infinity", false);
    test("0.0", "0x0.0", true);
    test("-0.0", "-0x0.0", true);

    test("1.0", "0x1.0#1", true);
    test("2.0", "0x2.0#1", true);
    test("0.5", "0x0.8#1", true);
    test("0.33333333333333331", "0x0.55555555555554#53", true);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", true);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", true);

    test("-1.0", "-0x1.0#1", true);
    test("-2.0", "-0x2.0#1", true);
    test("-0.5", "-0x0.8#1", true);
    test("-0.33333333333333331", "-0x0.55555555555554#53", true);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", true);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", true);
}

#[test]
fn is_finite_properties() {
    float_gen().test_properties(|x| {
        let is_finite = x.is_finite();
        assert_eq!(is_finite, !x.is_nan() && !x.is_infinite());
        assert_eq!(
            is_finite,
            x > Float::NEGATIVE_INFINITY && x < Float::INFINITY
        );
        assert_eq!(is_finite, rug::Float::try_from(&x).unwrap().is_finite());
    });

    primitive_float_gen::<f64>().test_properties(|x| {
        assert_eq!(Float::from(x).is_finite(), x.is_finite());
    });
}

#[test]
fn test_is_infinite() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(x.is_infinite(), out);
        assert_eq!(rug::Float::try_from(&x).unwrap().is_infinite(), out);
    };
    test("NaN", "NaN", false);
    test("Infinity", "Infinity", true);
    test("-Infinity", "-Infinity", true);
    test("0.0", "0x0.0", false);
    test("-0.0", "-0x0.0", false);

    test("1.0", "0x1.0#1", false);
    test("2.0", "0x2.0#1", false);
    test("0.5", "0x0.8#1", false);
    test("0.33333333333333331", "0x0.55555555555554#53", false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", false);

    test("-1.0", "-0x1.0#1", false);
    test("-2.0", "-0x2.0#1", false);
    test("-0.5", "-0x0.8#1", false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", false);
}

#[test]
fn is_infinite_properties() {
    float_gen().test_properties(|x| {
        let is_infinite = x.is_infinite();
        assert_eq!(x.classify() == FpCategory::Infinite, is_infinite);
        assert_eq!(is_infinite, !x.is_nan() && !x.is_finite());
        assert_eq!(
            is_infinite,
            x == Float::NEGATIVE_INFINITY || x == Float::INFINITY
        );
        assert_eq!(is_infinite, rug::Float::try_from(&x).unwrap().is_infinite());
    });

    primitive_float_gen::<f64>().test_properties(|x| {
        assert_eq!(Float::from(x).is_infinite(), x.is_infinite());
    });
}

#[test]
fn test_is_positive_zero() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(x.is_positive_zero(), out);
        let fx = rug::Float::try_from(&x).unwrap();
        assert_eq!(fx.is_zero() && fx.is_sign_positive(), out);
    };
    test("NaN", "NaN", false);
    test("Infinity", "Infinity", false);
    test("-Infinity", "-Infinity", false);
    test("0.0", "0x0.0", true);
    test("-0.0", "-0x0.0", false);

    test("1.0", "0x1.0#1", false);
    test("2.0", "0x2.0#1", false);
    test("0.5", "0x0.8#1", false);
    test("0.33333333333333331", "0x0.55555555555554#53", false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", false);

    test("-1.0", "-0x1.0#1", false);
    test("-2.0", "-0x2.0#1", false);
    test("-0.5", "-0x0.8#1", false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", false);
}

#[test]
fn is_positive_zero_properties() {
    float_gen().test_properties(|x| {
        let is_positive_zero = x.is_positive_zero();
        assert_eq!(
            is_positive_zero,
            ComparableFloatRef(&x) == ComparableFloatRef(&Float::ZERO)
        );
        assert_eq!(is_positive_zero, x == Float::ZERO && x.is_sign_positive());
        let fx = rug::Float::try_from(&x).unwrap();
        assert_eq!(is_positive_zero, fx.is_zero() && fx.is_sign_positive());
    });

    primitive_float_gen::<f64>().test_properties(|x| {
        assert_eq!(
            Float::from(x).is_positive_zero(),
            x == 0.0 && x.is_sign_positive()
        );
    });
}

#[test]
fn test_is_negative_zero() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(x.is_negative_zero(), out);
        let fx = rug::Float::try_from(&x).unwrap();
        assert_eq!(fx.is_zero() && fx.is_sign_negative(), out);
    };
    test("NaN", "NaN", false);
    test("Infinity", "Infinity", false);
    test("-Infinity", "-Infinity", false);
    test("0.0", "0x0.0", false);
    test("-0.0", "-0x0.0", true);

    test("1.0", "0x1.0#1", false);
    test("2.0", "0x2.0#1", false);
    test("0.5", "0x0.8#1", false);
    test("0.33333333333333331", "0x0.55555555555554#53", false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", false);

    test("-1.0", "-0x1.0#1", false);
    test("-2.0", "-0x2.0#1", false);
    test("-0.5", "-0x0.8#1", false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", false);
}

#[test]
fn is_negative_zero_properties() {
    float_gen().test_properties(|x| {
        let is_negative_zero = x.is_negative_zero();
        assert_eq!(
            is_negative_zero,
            ComparableFloatRef(&x) == ComparableFloatRef(&Float::NEGATIVE_ZERO)
        );
        assert_eq!(is_negative_zero, x == Float::ZERO && x.is_sign_negative());
        let fx = rug::Float::try_from(&x).unwrap();
        assert_eq!(is_negative_zero, fx.is_zero() && fx.is_sign_negative());
    });

    primitive_float_gen::<f64>().test_properties(|x| {
        assert_eq!(
            Float::from(x).is_negative_zero(),
            x == 0.0 && x.is_sign_negative()
        );
    });
}

#[test]
fn test_is_zero() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(x.is_zero(), out);
        assert_eq!(rug::Float::try_from(&x).unwrap().is_zero(), out);
    };
    test("NaN", "NaN", false);
    test("Infinity", "Infinity", false);
    test("-Infinity", "-Infinity", false);
    test("0.0", "0x0.0", true);
    test("-0.0", "-0x0.0", true);

    test("1.0", "0x1.0#1", false);
    test("2.0", "0x2.0#1", false);
    test("0.5", "0x0.8#1", false);
    test("0.33333333333333331", "0x0.55555555555554#53", false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", false);

    test("-1.0", "-0x1.0#1", false);
    test("-2.0", "-0x2.0#1", false);
    test("-0.5", "-0x0.8#1", false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", false);
}

#[test]
fn is_zero_properties() {
    float_gen().test_properties(|x| {
        let is_zero = x.is_zero();
        assert_eq!(is_zero, x == Float::ZERO);
        assert_eq!(is_zero, x.classify() == FpCategory::Zero);
        assert_eq!(is_zero, rug::Float::try_from(&x).unwrap().is_zero());
    });

    primitive_float_gen::<f64>().test_properties(|x| {
        assert_eq!(Float::from(x).is_zero(), x == 0.0);
    });
}

#[test]
fn test_is_normal() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(x.is_normal(), out);
        assert_eq!(rug::Float::try_from(&x).unwrap().is_normal(), out);
    };
    test("NaN", "NaN", false);
    test("Infinity", "Infinity", false);
    test("-Infinity", "-Infinity", false);
    test("0.0", "0x0.0", false);
    test("-0.0", "-0x0.0", false);

    test("1.0", "0x1.0#1", true);
    test("2.0", "0x2.0#1", true);
    test("0.5", "0x0.8#1", true);
    test("0.33333333333333331", "0x0.55555555555554#53", true);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", true);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", true);

    test("-1.0", "-0x1.0#1", true);
    test("-2.0", "-0x2.0#1", true);
    test("-0.5", "-0x0.8#1", true);
    test("-0.33333333333333331", "-0x0.55555555555554#53", true);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", true);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", true);
}

#[test]
fn is_normal_properties() {
    float_gen().test_properties(|x| {
        let is_normal = x.is_normal();
        assert_eq!(is_normal, x.is_finite() && !x.is_zero());
        assert_eq!(is_normal, x.classify() == FpCategory::Normal);
        assert_eq!(is_normal, rug::Float::try_from(&x).unwrap().is_normal());
    });
}

#[test]
fn test_is_sign_positive() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(x.is_sign_positive(), out);
        let fx = rug::Float::try_from(&x).unwrap();
        assert_eq!(!fx.is_nan() && fx.is_sign_positive(), out);
    };
    test("NaN", "NaN", false);
    test("Infinity", "Infinity", true);
    test("-Infinity", "-Infinity", false);
    test("0.0", "0x0.0", true);
    test("-0.0", "-0x0.0", false);

    test("1.0", "0x1.0#1", true);
    test("2.0", "0x2.0#1", true);
    test("0.5", "0x0.8#1", true);
    test("0.33333333333333331", "0x0.55555555555554#53", true);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", true);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", true);

    test("-1.0", "-0x1.0#1", false);
    test("-2.0", "-0x2.0#1", false);
    test("-0.5", "-0x0.8#1", false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", false);
}

#[test]
fn is_sign_positive_properties() {
    float_gen().test_properties(|x| {
        let is_sign_positive = x.is_sign_positive();
        assert_eq!(is_sign_positive, x.is_positive_zero() || x > Float::ZERO);
        assert_eq!(is_sign_positive, !x.is_nan() && !x.is_sign_negative());
        let fx = rug::Float::try_from(&x).unwrap();
        assert_eq!(is_sign_positive, !fx.is_nan() && fx.is_sign_positive());
    });

    primitive_float_gen::<f64>().test_properties(|x| {
        assert_eq!(
            Float::from(x).is_sign_positive(),
            !x.is_nan() && x.is_sign_positive()
        );
    });
}

#[test]
fn test_is_sign_negative() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(x.is_sign_negative(), out);
        let fx = rug::Float::try_from(&x).unwrap();
        assert_eq!(!fx.is_nan() && fx.is_sign_negative(), out);
    };
    test("NaN", "NaN", false);
    test("Infinity", "Infinity", false);
    test("-Infinity", "-Infinity", true);
    test("0.0", "0x0.0", false);
    test("-0.0", "-0x0.0", true);

    test("1.0", "0x1.0#1", false);
    test("2.0", "0x2.0#1", false);
    test("0.5", "0x0.8#1", false);
    test("0.33333333333333331", "0x0.55555555555554#53", false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", false);

    test("-1.0", "-0x1.0#1", true);
    test("-2.0", "-0x2.0#1", true);
    test("-0.5", "-0x0.8#1", true);
    test("-0.33333333333333331", "-0x0.55555555555554#53", true);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", true);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", true);
}

#[test]
fn is_sign_negative_properties() {
    float_gen().test_properties(|x| {
        let is_sign_negative = x.is_sign_negative();
        assert_eq!(is_sign_negative, x.is_negative_zero() || x < Float::ZERO);
        assert_eq!(is_sign_negative, !x.is_nan() && !x.is_sign_positive());
        let fx = rug::Float::try_from(&x).unwrap();
        assert_eq!(is_sign_negative, !fx.is_nan() && fx.is_sign_negative());
    });

    primitive_float_gen::<f64>().test_properties(|x| {
        assert_eq!(
            Float::from(x).is_sign_negative(),
            !x.is_nan() && x.is_sign_negative()
        );
    });
}

#[test]
fn test_classify() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(x.classify(), out);
        assert_eq!(rug::Float::try_from(&x).unwrap().classify(), out);
    };
    test("NaN", "NaN", FpCategory::Nan);
    test("Infinity", "Infinity", FpCategory::Infinite);
    test("-Infinity", "-Infinity", FpCategory::Infinite);
    test("0.0", "0x0.0", FpCategory::Zero);
    test("-0.0", "-0x0.0", FpCategory::Zero);

    test("1.0", "0x1.0#1", FpCategory::Normal);
    test("2.0", "0x2.0#1", FpCategory::Normal);
    test("0.5", "0x0.8#1", FpCategory::Normal);
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        FpCategory::Normal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        FpCategory::Normal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        FpCategory::Normal,
    );

    test("-1.0", "-0x1.0#1", FpCategory::Normal);
    test("-2.0", "-0x2.0#1", FpCategory::Normal);
    test("-0.5", "-0x0.8#1", FpCategory::Normal);
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        FpCategory::Normal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        FpCategory::Normal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        FpCategory::Normal,
    );
}

#[test]
fn classify_properties() {
    float_gen().test_properties(|x| {
        assert_eq!(x.classify(), rug::Float::try_from(&x).unwrap().classify());
    });
}

#[allow(clippy::redundant_closure_for_method_calls)]
#[test]
fn test_to_non_nan() {
    let test = |s, s_hex, out: Option<&str>, out_hex: Option<&str>| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let actual_out = x.to_non_nan();
        assert!(actual_out.as_ref().map_or(true, Float::is_valid));
        let actual_out_alt = x.into_non_nan();
        assert!(actual_out_alt.as_ref().map_or(true, Float::is_valid));
        assert_eq!(actual_out_alt, actual_out);

        let s = actual_out.as_ref().map(|x| x.to_string());
        assert_eq!(s.as_deref(), out);
        let s = actual_out.map(|x| to_hex_string(&x));
        assert_eq!(s.as_deref(), out_hex);
    };
    test("NaN", "NaN", None, None);
    test("Infinity", "Infinity", Some("Infinity"), Some("Infinity"));
    test(
        "-Infinity",
        "-Infinity",
        Some("-Infinity"),
        Some("-Infinity"),
    );
    test("0.0", "0x0.0", Some("0.0"), Some("0x0.0"));
    test("-0.0", "-0x0.0", Some("-0.0"), Some("-0x0.0"));

    test("1.0", "0x1.0#1", Some("1.0"), Some("0x1.0#1"));
    test("2.0", "0x2.0#1", Some("2.0"), Some("0x2.0#1"));
    test("0.5", "0x0.8#1", Some("0.5"), Some("0x0.8#1"));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Some("0.33333333333333331"),
        Some("0x0.55555555555554#53"),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Some("1.4142135623730951"),
        Some("0x1.6a09e667f3bcd#53"),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Some("3.1415926535897931"),
        Some("0x3.243f6a8885a30#53"),
    );

    test("-1.0", "-0x1.0#1", Some("-1.0"), Some("-0x1.0#1"));
    test("-2.0", "-0x2.0#1", Some("-2.0"), Some("-0x2.0#1"));
    test("-0.5", "-0x0.8#1", Some("-0.5"), Some("-0x0.8#1"));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Some("-0.33333333333333331"),
        Some("-0x0.55555555555554#53"),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        Some("-1.4142135623730951"),
        Some("-0x1.6a09e667f3bcd#53"),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Some("-3.1415926535897931"),
        Some("-0x3.243f6a8885a30#53"),
    );
}

#[test]
fn to_non_nan_properties() {
    float_gen().test_properties(|x| {
        let nn = x.to_non_nan();
        if let Some(nn) = nn {
            assert!(nn.is_valid());
            let nn_alt = x.clone().into_non_nan().unwrap();
            assert!(nn_alt.is_valid());
            assert_eq!(ComparableFloatRef(&nn), ComparableFloatRef(&nn_alt));
            assert_eq!(ComparableFloatRef(&nn), ComparableFloatRef(&x));
            assert!(!nn.is_nan());
        } else {
            assert!(x.is_nan());
            assert!(x.into_non_nan().is_none());
        }
    });
}

#[allow(clippy::redundant_closure_for_method_calls)]
#[test]
fn test_to_finite() {
    let test = |s, s_hex, out: Option<&str>, out_hex: Option<&str>| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let actual_out = x.to_finite();
        assert!(actual_out.as_ref().map_or(true, Float::is_valid));
        let actual_out_alt = x.into_finite();
        assert!(actual_out_alt.as_ref().map_or(true, Float::is_valid));
        assert_eq!(actual_out_alt, actual_out);

        let s = actual_out.as_ref().map(|x| x.to_string());
        assert_eq!(s.as_deref(), out);
        let s = actual_out.map(|x| to_hex_string(&x));
        assert_eq!(s.as_deref(), out_hex);
    };
    test("NaN", "NaN", None, None);
    test("Infinity", "Infinity", None, None);
    test("-Infinity", "-Infinity", None, None);
    test("0.0", "0x0.0", Some("0.0"), Some("0x0.0"));
    test("-0.0", "-0x0.0", Some("-0.0"), Some("-0x0.0"));

    test("1.0", "0x1.0#1", Some("1.0"), Some("0x1.0#1"));
    test("2.0", "0x2.0#1", Some("2.0"), Some("0x2.0#1"));
    test("0.5", "0x0.8#1", Some("0.5"), Some("0x0.8#1"));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Some("0.33333333333333331"),
        Some("0x0.55555555555554#53"),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Some("1.4142135623730951"),
        Some("0x1.6a09e667f3bcd#53"),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Some("3.1415926535897931"),
        Some("0x3.243f6a8885a30#53"),
    );

    test("-1.0", "-0x1.0#1", Some("-1.0"), Some("-0x1.0#1"));
    test("-2.0", "-0x2.0#1", Some("-2.0"), Some("-0x2.0#1"));
    test("-0.5", "-0x0.8#1", Some("-0.5"), Some("-0x0.8#1"));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Some("-0.33333333333333331"),
        Some("-0x0.55555555555554#53"),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        Some("-1.4142135623730951"),
        Some("-0x1.6a09e667f3bcd#53"),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Some("-3.1415926535897931"),
        Some("-0x3.243f6a8885a30#53"),
    );
}

#[test]
fn to_finite_properties() {
    float_gen().test_properties(|x| {
        let f = x.to_finite();
        if let Some(f) = f {
            assert!(f.is_valid());
            let f_alt = x.clone().into_non_nan().unwrap();
            assert!(f_alt.is_valid());
            assert_eq!(ComparableFloatRef(&f), ComparableFloatRef(&f_alt));
            assert_eq!(ComparableFloatRef(&f), ComparableFloatRef(&x));
            assert!(f.is_finite());
        } else {
            assert!(!x.is_finite());
            assert!(x.into_finite().is_none());
        }
    });
}
