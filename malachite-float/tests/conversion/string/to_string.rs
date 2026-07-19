// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::{ToDebugString, string_is_subset};
use malachite_float::ComparableFloatRef;
use malachite_float::conversion::string::get_str::get_str_ndigits;
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::float_gen;

// The number of significant digits in a formatted mantissa: the digits of `s` up to any exponent
// part, ignoring the sign, the point, and leading zeros (which only position the point).
fn significant_digit_count(s: &str) -> usize {
    let s = s.split(['e', 'E']).next().unwrap();
    s.chars()
        .filter(char::is_ascii_alphanumeric)
        .skip_while(|&c| c == '0')
        .count()
}

#[test]
pub fn test_to_string() {
    fn test(s_hex: &str, out: &str) {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), out);
        assert_eq!(x.to_debug_string(), out);
    }
    // specials and zeros
    test("NaN", "NaN");
    test("Infinity", "Infinity");
    test("-Infinity", "-Infinity");
    test("0x0.0", "0.0");
    test("-0x0.0", "-0.0");
    // Every Float of precision p is shown with 1 + ceil(p log10(2)) significant digits — enough to
    // round-trip, the same count for every value of that precision, trailing zeros included.
    test("0x1.0#1", "1.0");
    test("0x1.8#2", "1.5");
    test("-0x1.8#2", "-1.5");
    test("0x0.8#1", "0.50");
    test("0xff.0#8", "255.0");
    test("0xf4240.0#20", "1000000.0");
    test("0x4d2.8#12", "1234.5");
    test("0x0.00008#1", "7.6e-6");
    test("0x0.004#1", "0.00098");
    test("0x0.0050df15a4acf314#53", "0.0012340000000000001");
    test("0x0.1999999999999a#53", "0.10000000000000001");
    test("0x6.f70E+25#13", "8.8290e30");
    test("0x1.921fb54442d18#53", "1.5707963267948966");
    test("-0x2192f69.48#33", "-35204969.281");
    // extreme exponents are no problem for the get_str-based implementation
    test("0x1.0E+250000000#1", "4.6e301029995");
    test("0x1.0E-250000000#1", "2.2e-301029996");
}

#[test]
fn to_string_properties() {
    float_gen().test_properties(|x| {
        let s = x.to_string();
        assert_eq!(x.to_debug_string(), s);
        assert!(s.is_ascii());
        if x.is_nan() {
            assert_eq!(s, "NaN");
        } else {
            assert_eq!(s.starts_with('-'), x.is_sign_negative());
            if x.is_infinite() {
                assert_eq!(s.trim_start_matches('-'), "Infinity");
            } else {
                // a finite value always shows a point, and uses lowercase scientific notation
                assert!(s.contains('.'), "{s:?}");
                assert!(string_is_subset(&s, "-.0123456789e"));
                // The round-trip digit count, uniform for each precision — except that when all
                // the digits are integral, the point-forcing convention appends a `.0` whose zero
                // is not a significant digit ("16.0" for the two-digit prec-1 value 16, but
                // "255.0" for the genuinely four-digit prec-8 value 255).
                if let Some(precision) = x.get_prec() {
                    let count = significant_digit_count(&s);
                    let expected = get_str_ndigits(10, precision);
                    assert!(
                        count == expected || s.ends_with(".0") && count == expected + 1,
                        "{s:?}"
                    );
                }
            }
        }
    });
}

#[test]
pub fn test_to_binary_string() {
    fn test(s_hex: &str, out: &str, out_prefixed: &str) {
        let x = parse_hex_string(s_hex);
        assert_eq!(format!("{x:b}"), out);
        assert_eq!(format!("{x:#b}"), out_prefixed);
    }
    test("NaN", "NaN", "NaN");
    test("-Infinity", "-Infinity", "-Infinity");
    test("0x0.0", "0.0", "0b0.0");
    test("-0x0.0", "-0.0", "-0b0.0");
    test("0x1.0#1", "1.0", "0b1.0");
    test("0x1.8#2", "1.1", "0b1.1");
    test("-0x1.8#2", "-1.1", "-0b1.1");
    test("0x0.8#1", "0.1", "0b0.1");
    test("0xff.0#8", "11111111.0", "0b11111111.0");
    test("0x5.0#3", "101.0", "0b101.0");
    test("0x0.00008#1", "1.0E-17", "0b1.0E-17");
    test("0x6.f70E+25#13", "1.101111011100E102", "0b1.101111011100E102");
    test("0x0.004#1", "1.0E-10", "0b1.0E-10");
}

#[test]
fn to_binary_string_properties() {
    float_gen().test_properties(|x| {
        let s = format!("{x:b}");
        assert!(s.is_ascii());
        // the alternate form only inserts the prefix, after any sign, for non-special values
        let prefixed = if x.is_nan() || x.is_infinite() {
            s.clone()
        } else if let Some(body) = s.strip_prefix('-') {
            format!("-0b{body}")
        } else {
            format!("0b{s}")
        };
        assert_eq!(format!("{x:#b}"), prefixed);
        if x.is_finite() && !x.is_zero() {
            // digits 2-9 can still appear in the decimal exponent
            assert!(string_is_subset(&s, "-.01E+23456789"));
            // Binary digits represent the value exactly, one digit per bit of precision — except
            // that the point-forcing convention's `.0` zero is not a significant digit (and can
            // sit in the mantissa of a scientific form, as in "1.0E1" for the prec-1 value 2).
            let count = u64::try_from(significant_digit_count(&s)).unwrap();
            let precision = x.get_prec().unwrap();
            let mantissa = s.split('E').next().unwrap();
            assert!(
                count == precision || mantissa.ends_with(".0") && count == precision + 1,
                "{s:?}"
            );
        }
    });
}

#[test]
pub fn test_to_octal_string() {
    fn test(s_hex: &str, out: &str, out_prefixed: &str) {
        let x = parse_hex_string(s_hex);
        assert_eq!(format!("{x:o}"), out);
        assert_eq!(format!("{x:#o}"), out_prefixed);
    }
    test("NaN", "NaN", "NaN");
    test("-Infinity", "-Infinity", "-Infinity");
    test("0x0.0", "0.0", "0o0.0");
    test("-0x0.0", "-0.0", "-0o0.0");
    test("0x1.0#1", "1.0", "0o1.0");
    test("0x1.8#2", "1.4", "0o1.4");
    test("-0x1.8#2", "-1.4", "-0o1.4");
    test("0x0.8#1", "0.4", "0o0.4");
    test("0xff.0#8", "377.0", "0o377.0");
    test("0x5.0#3", "5.0", "0o5.0");
    test("0x0.00008#1", "2.0E-6", "0o2.0E-6");
    test("0x6.f70E+25#13", "1.5734E34", "0o1.5734E34");
    test("0x0.004#1", "0.0004", "0o0.0004");
}

#[test]
fn to_octal_string_properties() {
    float_gen().test_properties(|x| {
        let s = format!("{x:o}");
        assert!(s.is_ascii());
        let prefixed = if x.is_nan() || x.is_infinite() {
            s.clone()
        } else if let Some(body) = s.strip_prefix('-') {
            format!("-0o{body}")
        } else {
            format!("0o{s}")
        };
        assert_eq!(format!("{x:#o}"), prefixed);
        if x.is_finite() && !x.is_zero() {
            // digits 8 and 9 can still appear in the decimal exponent
            assert!(string_is_subset(&s, "-.01234567E+89"));
        }
    });
}

#[test]
pub fn test_to_lower_hex_string() {
    fn test(s_hex: &str, out: &str, out_prefixed: &str) {
        let x = parse_hex_string(s_hex);
        assert_eq!(format!("{x:x}"), out);
        assert_eq!(format!("{x:#x}"), out_prefixed);
    }
    test("NaN", "NaN", "NaN");
    test("-Infinity", "-Infinity", "-Infinity");
    test("0x0.0", "0.0", "0x0.0");
    test("-0x0.0", "-0.0", "-0x0.0");
    test("0x1.0#1", "1.0", "0x1.0");
    test("0x1.8#2", "1.8", "0x1.8");
    test("-0x1.8#2", "-1.8", "-0x1.8");
    test("0x0.8#1", "0.8", "0x0.8");
    test("0xff.0#8", "ff.0", "0xff.0");
    test("0x5.0#3", "5.0", "0x5.0");
    test("0x0.00008#1", "0.00008", "0x0.00008");
    test("0x6.f70E+25#13", "6.f70E+25", "0x6.f70E+25");
    test("0x0.004#1", "0.004", "0x0.004");
}

#[test]
pub fn test_to_upper_hex_string() {
    fn test(s_hex: &str, out: &str, out_prefixed: &str) {
        let x = parse_hex_string(s_hex);
        assert_eq!(format!("{x:X}"), out);
        assert_eq!(format!("{x:#X}"), out_prefixed);
    }
    test("NaN", "NaN", "NaN");
    test("-Infinity", "-Infinity", "-Infinity");
    test("0x0.0", "0.0", "0x0.0");
    test("-0x0.0", "-0.0", "-0x0.0");
    test("0x1.0#1", "1.0", "0x1.0");
    test("0x1.8#2", "1.8", "0x1.8");
    test("-0x1.8#2", "-1.8", "-0x1.8");
    test("0x0.8#1", "0.8", "0x0.8");
    test("0xff.0#8", "FF.0", "0xFF.0");
    test("0x5.0#3", "5.0", "0x5.0");
    test("0x0.00008#1", "0.00008", "0x0.00008");
    test("0x6.f70E+25#13", "6.F70E+25", "0x6.F70E+25");
    test("0x0.004#1", "0.004", "0x0.004");
}

#[test]
fn to_hex_string_properties() {
    float_gen().test_properties(|x| {
        let lower = format!("{x:x}");
        let upper = format!("{x:X}");
        assert!(lower.is_ascii());
        let prefixed = if x.is_nan() || x.is_infinite() {
            lower.clone()
        } else if let Some(body) = lower.strip_prefix('-') {
            format!("-0x{body}")
        } else {
            format!("0x{lower}")
        };
        assert_eq!(format!("{x:#x}"), prefixed);
        if x.is_finite() {
            assert_eq!(upper, lower.to_ascii_uppercase());
        } else {
            // the specials are not case-converted
            assert_eq!(upper, lower);
        }
        // The ComparableFloat hex form round-trips exactly, preserving value, sign, and precision.
        // (parse_hex_string itself asserts that re-rendering the parsed value gives its input.)
        let s = format!("{:#x}", ComparableFloatRef(&x));
        let y = parse_hex_string(&s);
        assert_eq!(ComparableFloatRef(&y), ComparableFloatRef(&x));
    });
}
