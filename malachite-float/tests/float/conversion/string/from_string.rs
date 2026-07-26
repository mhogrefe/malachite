// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::num::conversion::traits::{FromSciString, FromStringBase};
use malachite_base::test_util::generators::{string_gen, string_gen_var_1};
use malachite_float::test_util::common::to_hex_string;
use malachite_float::test_util::generators::{float_gen, float_gen_var_12};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::panic::catch_unwind;
use std::str::FromStr;

// The four bases a `ComparableFloat` can be written in.
const BASES: [u8; 4] = [2, 8, 10, 16];

// Renders `x` in `base` the way `ComparableFloat` does, with the base prefix if `alt`. These are
// exactly the strings these readers have to invert.
fn render(x: &Float, base: u8, alt: bool) -> String {
    let c = ComparableFloatRef(x);
    match (base, alt) {
        (2, false) => format!("{c:b}"),
        (2, true) => format!("{c:#b}"),
        (8, false) => format!("{c:o}"),
        (8, true) => format!("{c:#o}"),
        (16, false) => format!("{c:x}"),
        (16, true) => format!("{c:#x}"),
        _ => format!("{c}"),
    }
}

// Checks every invariant of `Float::from_string_base` on one input, and the agreement of the two
// `FromStr` impls that specialize it. Called by both the unit tests and the property tests.
fn verify_from_string_base(base: u8, s: &str) {
    let result = Float::from_string_base(base, s);
    if let Some(x) = &result {
        assert!(x.is_valid());
    }
    // `FromStr` is `from_string_base` in base 10, for both wrappers.
    if base == 10 {
        assert_eq!(
            Float::from_str(s).ok().map(ComparableFloat),
            result.clone().map(ComparableFloat)
        );
        assert_eq!(
            ComparableFloat::from_str(s).ok(),
            result.clone().map(ComparableFloat)
        );
    }
    // With neither a `#` suffix nor a base prefix to strip, this is just `from_sci_string` over the
    // same string, so the two must agree.
    let prefix = match base {
        2 => "0b",
        8 => "0o",
        16 => "0x",
        _ => "",
    };
    let after_sign = s.strip_prefix('-').unwrap_or(s);
    if !s.contains('#') && (prefix.is_empty() || !after_sign.starts_with(prefix)) {
        let mut options = FromSciStringOptions::default();
        options.set_base(base);
        assert_eq!(
            Float::from_sci_string_with_options(s, options).map(ComparableFloat),
            result.map(ComparableFloat)
        );
    }
}

#[test]
fn test_from_string_base() {
    fn test(base: u8, s: &str, out: &str, out_hex: &str) {
        let x = Float::from_string_base(base, s).unwrap();
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        verify_from_string_base(base, s);
    }
    fn test_none(base: u8, s: &str) {
        assert!(Float::from_string_base(base, s).is_none());
        verify_from_string_base(base, s);
    }
    // The specials and the zeros are spelled the same in every base and carry no precision.
    test(10, "NaN", "NaN", "NaN");
    test(10, "Infinity", "Infinity", "Infinity");
    test(10, "-Infinity", "-Infinity", "-Infinity");
    test(10, "0.0", "0.0", "0x0.0");
    test(10, "-0.0", "-0.0", "-0x0.0");
    test(16, "NaN", "NaN", "NaN");
    test(16, "Infinity", "Infinity", "Infinity");
    test(16, "0x0.0", "0.0", "0x0.0");
    test(16, "-0x0.0", "-0.0", "-0x0.0");
    // Without a `#` suffix the precision is inferred from the digits.
    test(10, "1.0", "1.0", "0x1.0#1");
    // With one it is exact, which is what makes the round trip work.
    test(10, "1.0#1", "1.0", "0x1.0#1");
    test(10, "1.5#10", "1.5000", "0x1.800#10");
    test(10, "-1.5#10", "-1.5000", "-0x1.800#10");
    test(10, "0.1#10", "0.099976", "0x0.1998#10");
    test(10, "1.0e10#5", "1.02e10", "0x2.6E+8#5");
    // The base prefix is optional, since `{:x}` omits it and `{:#x}` writes it.
    test(16, "0x1.0#1", "1.0", "0x1.0#1");
    test(16, "1.0#1", "1.0", "0x1.0#1");
    test(2, "0b1.0#1", "1.0", "0x1.0#1");
    test(2, "1.0#1", "1.0", "0x1.0#1");
    test(8, "0o1.0#1", "1.0", "0x1.0#1");
    test(16, "0xff.0#8", "255.0", "0xff.0#8");
    test(16, "-0x1.8#4", "-1.50", "-0x1.8#4");
    test(2, "0b1.01#3", "1.2", "0x1.4#3");
    test(8, "0o7.7#6", "7.88", "0x7.e#6");
    // The hexadecimal form with an exponent, the one MPFR's grammar cannot read.
    test(16, "0x1.0E+25#1", "1.3e30", "0x1.0E+25#1");
    // A precision of zero is not a `Float` precision, and a malformed or absent one is not a
    // precision at all.
    test_none(10, "1.0#0");
    test_none(16, "0x1.0#0");
    test_none(10, "1.0#");
    test_none(10, "1.0#abc");
    test_none(10, "#");
    // The specials and the zeros never write a precision, so a string that claims one for them is
    // not something `ComparableFloat` wrote.
    test_none(10, "NaN#5");
    test_none(10, "Infinity#5");
    test_none(10, "0.0#5");
    test_none(10, "-0.0#5");
    // strings that are not numbers
    test_none(10, "");
    test_none(10, "abc");
    test_none(16, "0y1.0#1");
}

#[test]
fn test_from_str() {
    fn test(s: &str, out: &str, out_hex: &str) {
        let x = Float::from_str(s).unwrap();
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        // `ComparableFloat` reads the same strings, and compares by precision as well as value.
        assert_eq!(
            ComparableFloat::from_str(s).unwrap(),
            ComparableFloat(x.clone())
        );
        verify_from_string_base(10, s);
    }
    test("NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "-Infinity");
    test("0.0", "0.0", "0x0.0");
    test("-0.0", "-0.0", "-0x0.0");
    test("1.0", "1.0", "0x1.0#1");
    test("1.0#1", "1.0", "0x1.0#1");
    test("1.5#10", "1.5000", "0x1.800#10");
    test("0.1", "0.102", "0x0.1a#4");
    assert!(Float::from_str("abc").is_err());
    assert!(ComparableFloat::from_str("abc").is_err());
    let _ = Float::ZERO;
}

#[test]
fn from_string_base_fail() {
    // A base outside 2 to 36 is not a base, whatever the string is. In particular this is checked
    // before the string, so that a string that would be rejected anyway does not mask it.
    assert_panic!(Float::from_string_base(0, "1.0"));
    assert_panic!(Float::from_string_base(1, "1.0"));
    assert_panic!(Float::from_string_base(37, "1.0"));
    assert_panic!(Float::from_string_base(1, "1.0#0"));
    assert_panic!(Float::from_string_base(1, ""));
    let _ = Float::ZERO;
}

#[test]
fn from_string_base_properties() {
    // The round trip that the `#` suffix exists for: what `ComparableFloat` writes in a base,
    // `from_string_base` reads back in that base, exactly, precision and all. Both the plain and
    // the alternate rendering, since the base prefix is optional.
    fn round_trip(x: Float) {
        for base in BASES {
            for alt in [false, true] {
                let s = render(&x, base, alt);
                let y = Float::from_string_base(base, &s).unwrap();
                assert_eq!(ComparableFloat(y), ComparableFloat(x.clone()), "{s:?}");
                verify_from_string_base(base, &s);
            }
        }
        // The base-10 rendering is what `FromStr` reads.
        let s = format!("{}", ComparableFloatRef(&x));
        assert_eq!(
            ComparableFloat::from_str(&s).unwrap(),
            ComparableFloat(x.clone())
        );
        assert_eq!(
            ComparableFloat(Float::from_str(&s).unwrap()),
            ComparableFloat(x)
        );
    }
    float_gen().test_properties(round_trip);
    // Extreme exponents and precisions, where the digit count and the exponent are largest.
    float_gen_var_12().test_properties(round_trip);

    // Arbitrary strings, for the rejection paths: reading one must not panic, whatever the base,
    // and the invariants must hold for whatever it does return.
    string_gen().test_properties(|s| {
        for base in BASES {
            verify_from_string_base(base, &s);
        }
    });

    // Strings of ASCII characters, which reach further into the parser than arbitrary Unicode does
    // before being rejected.
    string_gen_var_1().test_properties(|s| {
        for base in BASES {
            verify_from_string_base(base, &s);
        }
    });
}
