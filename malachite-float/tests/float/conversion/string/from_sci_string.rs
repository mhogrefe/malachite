// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::num::conversion::traits::{FromSciString, IntegerMantissaAndExponent};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    string_from_sci_string_options_pair_gen_var_3, unsigned_gen,
};
use malachite_float::float::conversion::string::strtofr::set_str;
use malachite_float::test_util::common::to_hex_string;
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, string_from_sci_string_options_unsigned_triple_gen_var_1,
    string_from_sci_string_options_unsigned_triple_gen_var_2,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::panic::catch_unwind;

// The five rounding modes that never panic.
const INEXACT_MODES: [RoundingMode; 5] = [Floor, Ceiling, Down, Up, Nearest];

// Whether `o` is a direction `rm` is allowed to round a value of the given sign in. `Nearest` may
// go either way, and an exact result is allowed under every mode.
fn ordering_valid_for(o: Ordering, rm: RoundingMode, sign: bool) -> bool {
    match rm {
        Floor => o != Greater,
        Ceiling => o != Less,
        // toward zero
        Down => o == Equal || (o == Less) == sign,
        // away from zero
        Up => o == Equal || (o == Greater) == sign,
        Exact => o == Equal,
        Nearest => true,
    }
}

const fn with_mode(options: FromSciStringOptions, rm: RoundingMode) -> FromSciStringOptions {
    let mut options = options;
    options.set_rounding_mode(rm);
    options
}

fn options_for(base: u8, rm: RoundingMode) -> FromSciStringOptions {
    let mut options = FromSciStringOptions::default();
    options.set_base(base);
    options.set_rounding_mode(rm);
    options
}

// Checks every invariant of `Float::from_sci_string_with_options_prec` on one input, and the
// agreement of the entry points that specialize it. Called by both the unit tests and the property
// tests, so that the unit tests get the full check for free.
fn verify_from_sci_string(s: &str, options: FromSciStringOptions, prec: u64) {
    let rm = options.get_rounding_mode();
    let result = Float::from_sci_string_with_options_prec(s, options, prec);
    let Some((x, o)) = result.clone() else {
        // A string that does not parse does not parse under any mode or precision.
        for m in INEXACT_MODES {
            assert!(
                Float::from_sci_string_with_options_prec(s, with_mode(options, m), prec).is_none()
            );
        }
        assert!(Float::from_sci_string_with_options(s, options).is_none());
        return;
    };
    assert!(x.is_valid());
    // A finite nonzero result always has the requested precision; zeros and the specials carry
    // none.
    if let Some(p) = x.get_prec() {
        assert_eq!(p, prec);
    } else {
        assert!(x.is_nan() || x.is_infinite() || x == 0u32);
    }
    assert!(ordering_valid_for(o, rm, x.is_sign_positive()));
    // The base-10 entry points are the same function with the default base.
    if options.get_base() == 10 {
        let expected = result.clone().map(|(x, o)| (ComparableFloat(x), o));
        assert_eq!(
            Float::from_sci_string_prec_round(s, prec, rm).map(|(x, o)| (ComparableFloat(x), o)),
            expected
        );
        if rm == Nearest {
            assert_eq!(
                Float::from_sci_string_prec(s, prec).map(|(x, o)| (ComparableFloat(x), o)),
                expected
            );
        }
    }
    if x.is_nan() {
        assert_eq!(o, Equal);
        return;
    }
    // The directed modes bracket the value, and agreeing modes mean an exact result.
    let results: Vec<(Float, Ordering)> = INEXACT_MODES
        .iter()
        .map(|&m| Float::from_sci_string_with_options_prec(s, with_mode(options, m), prec).unwrap())
        .collect();
    let (floor, floor_o) = &results[0];
    let (ceiling, ceiling_o) = &results[1];
    assert!(floor <= ceiling);
    for (m, (y, o)) in INEXACT_MODES.iter().zip(&results) {
        assert!(ordering_valid_for(*o, *m, y.is_sign_positive()));
        assert!(floor <= y && y <= ceiling);
    }
    if floor == ceiling {
        assert_eq!(*floor_o, Equal);
        assert_eq!(*ceiling_o, Equal);
        let (y, o) =
            Float::from_sci_string_with_options_prec(s, with_mode(options, Exact), prec).unwrap();
        assert_eq!(ComparableFloat(y), ComparableFloat(floor.clone()));
        assert_eq!(o, Equal);
    } else {
        assert_eq!(*floor_o, Less);
        assert_eq!(*ceiling_o, Greater);
    }
    let (down, up) = (&results[2].0, &results[3].0);
    if x.is_sign_positive() {
        assert_eq!(ComparableFloatRef(down), ComparableFloatRef(floor));
        assert_eq!(ComparableFloatRef(up), ComparableFloatRef(ceiling));
    } else {
        assert_eq!(ComparableFloatRef(down), ComparableFloatRef(ceiling));
        assert_eq!(ComparableFloatRef(up), ComparableFloatRef(floor));
    }
    // Where the two grammars read a string the same way, the MPFR port must agree. Only two classes
    // of string are read differently by both, rather than accepted by one and rejected by the
    // other. The first is `e` and `E`: MPFR takes them as an exponent marker only up to base 10,
    // while Malachite takes them as one in every base, needing an explicit sign from base 15 up.
    // The second is the names of the special values: MPFR reads them only up to base 16 -- the last
    // base in which `i`, worth 18, is not a digit -- while Malachite reads them in every base,
    // because that is what `Display` writes. So from base 24 up (`n` is 23) `NaN` is a special to
    // Malachite and a digit string to MPFR, and likewise from base 35 up (`y` is 34) for
    // `Infinity`.
    let base = options.get_base();
    let ambiguous = (base > 10 && s.bytes().any(|c| c == b'e' || c == b'E'))
        || (base > 16 && matches!(s, "NaN" | "Infinity" | "-Infinity"));
    if !ambiguous
        && rm != Exact
        && let Some((y, o2)) = set_str(s, base, prec, rm)
    {
        assert_eq!(
            ComparableFloat(y),
            ComparableFloat(x.clone()),
            "{s:?} base {base} prec {prec} {rm}"
        );
        assert_eq!(o2, o, "{s:?} base {base} prec {prec} {rm}");
    }
}

// The value of a string as an exact `Rational`, rounded once: an oracle that shares only the
// grammar with the function under test, since `Rational` uses the same `preprocess_sci_string` but
// none of the `Float` machinery. `None` where a `Rational` cannot express the result.
fn rational_oracle(
    s: &str,
    options: FromSciStringOptions,
    prec: u64,
) -> Option<(ComparableFloat, Ordering)> {
    let q = Rational::from_sci_string_with_options(s, options)?;
    if q == 0u32 {
        // `Rational` has no signed zero, so it cannot say which zero the string denotes.
        return None;
    }
    let (x, o) = Float::from_rational_prec_round(q, prec, options.get_rounding_mode());
    Some((ComparableFloat(x), o))
}

#[test]
fn test_from_sci_string_with_options_prec() {
    fn test(
        s: &str,
        base: u8,
        rm: RoundingMode,
        prec: u64,
        out: &str,
        out_hex: &str,
        ord: Ordering,
    ) {
        let options = options_for(base, rm);
        let (x, o) = Float::from_sci_string_with_options_prec(s, options, prec).unwrap();
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, ord);
        verify_from_sci_string(s, options, prec);
    }
    fn test_none(s: &str, base: u8) {
        let options = options_for(base, Nearest);
        assert!(Float::from_sci_string_with_options_prec(s, options, 53).is_none());
        verify_from_sci_string(s, options, 53);
    }
    // the special values, spelled as `Display` writes them
    test("NaN", 10, Nearest, 53, "NaN", "NaN", Equal);
    test("Infinity", 10, Nearest, 53, "Infinity", "Infinity", Equal);
    test(
        "-Infinity",
        10,
        Nearest,
        53,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    // zeros keep their sign, whatever the exponent says
    test("0", 10, Nearest, 53, "0.0", "0x0.0", Equal);
    test("-0", 10, Nearest, 53, "-0.0", "-0x0.0", Equal);
    test("0.000", 10, Nearest, 53, "0.0", "0x0.0", Equal);
    test("-0.000e100", 10, Nearest, 53, "-0.0", "-0x0.0", Equal);
    test("0e-100", 10, Nearest, 53, "0.0", "0x0.0", Equal);
    // ordinary values
    test("1", 10, Nearest, 1, "1.0", "0x1.0#1", Equal);
    test("1.5", 10, Nearest, 10, "1.5000", "0x1.800#10", Equal);
    test("-1.5", 10, Nearest, 10, "-1.5000", "-0x1.800#10", Equal);
    test(
        "255",
        10,
        Nearest,
        53,
        "255.00000000000000",
        "0xff.000000000000#53",
        Equal,
    );
    test(
        "3.1415926535897931",
        10,
        Nearest,
        53,
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Greater,
    );
    // leading and trailing zeros change nothing but the digit count
    test(
        "0.00123",
        10,
        Nearest,
        20,
        "0.0012299996",
        "0x0.00509bf8#20",
        Less,
    );
    test("1.500", 10, Nearest, 10, "1.5000", "0x1.800#10", Equal);
    test(
        "000255",
        10,
        Nearest,
        53,
        "255.00000000000000",
        "0xff.000000000000#53",
        Equal,
    );
    // 0.1 is not representable; the modes bracket it, and the sign-dependent pair swaps over
    test("0.1", 10, Floor, 4, "0.0938", "0x0.18#4", Less);
    test("0.1", 10, Ceiling, 4, "0.102", "0x0.1a#4", Greater);
    test("0.1", 10, Down, 4, "0.0938", "0x0.18#4", Less);
    test("0.1", 10, Up, 4, "0.102", "0x0.1a#4", Greater);
    test("0.1", 10, Nearest, 4, "0.102", "0x0.1a#4", Greater);
    test("-0.1", 10, Floor, 4, "-0.102", "-0x0.1a#4", Less);
    test("-0.1", 10, Ceiling, 4, "-0.0938", "-0x0.18#4", Greater);
    test("-0.1", 10, Down, 4, "-0.0938", "-0x0.18#4", Greater);
    test("-0.1", 10, Up, 4, "-0.102", "-0x0.1a#4", Less);
    // exponents, in both cases and with an explicit sign
    test(
        "1e10",
        10,
        Nearest,
        53,
        "10000000000.000000",
        "0x2540be400.00000#53",
        Equal,
    );
    test(
        "1E10",
        10,
        Nearest,
        53,
        "10000000000.000000",
        "0x2540be400.00000#53",
        Equal,
    );
    test(
        "1e-10",
        10,
        Nearest,
        53,
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        Greater,
    );
    test("1.5e+3", 10, Nearest, 10, "1500.0", "0x5dc.0#10", Equal);
    test("+1.5", 10, Nearest, 10, "1.5000", "0x1.800#10", Equal);
    // other bases
    test("1.01", 2, Nearest, 10, "1.2500", "0x1.400#10", Equal);
    test("-101", 2, Nearest, 10, "-5.0000", "-0x5.00#10", Equal);
    test("1e10", 2, Nearest, 10, "1024.0", "0x400.0#10", Equal);
    test("777", 8, Nearest, 10, "511.00", "0x1ff.0#10", Equal);
    test(
        "ff",
        16,
        Nearest,
        53,
        "255.00000000000000",
        "0xff.000000000000#53",
        Equal,
    );
    test(
        "FF",
        16,
        Nearest,
        53,
        "255.00000000000000",
        "0xff.000000000000#53",
        Equal,
    );
    test(
        "ff.8",
        16,
        Nearest,
        53,
        "255.50000000000000",
        "0xff.800000000000#53",
        Equal,
    );
    test("z", 36, Nearest, 10, "35.000", "0x23.0#10", Equal);
    test(
        "zz.z",
        36,
        Nearest,
        20,
        "1295.9727",
        "0x50f.f90#20",
        Greater,
    );
    // Malachite's own hexadecimal output, which MPFR cannot read at all: `E` is a hex digit, so
    // MPFR stops at it, while Malachite takes the explicit sign as marking an exponent.
    test("1.0E+25", 16, Nearest, 1, "1.3e30", "0x1.0E+25#1", Equal);
    // From base 15 up, an unsigned `e` is the digit 14 and not an exponent marker, so these two
    // strings mean quite different things. Below base 15 there is no such choice to make: `e` is
    // not a digit there, so it is always the marker.
    test("1e5", 14, Nearest, 20, "537824.00", "0x834e0.0#20", Equal);
    test("1e5", 15, Nearest, 20, "440.00000", "0x1b8.000#20", Equal);
    test("1e+5", 15, Nearest, 20, "759375.00", "0xb964f.0#20", Equal);
    test(
        "1e5",
        16,
        Nearest,
        53,
        "485.00000000000000",
        "0x1e5.00000000000#53",
        Equal,
    );
    // The special names are read in every base, even from base 24 (`NaN`) and 35 (`Infinity`) up,
    // where they are also valid digit strings. This is what makes `Display` readable in any base.
    test("NaN", 36, Nearest, 53, "NaN", "NaN", Equal);
    test("Infinity", 36, Nearest, 53, "Infinity", "Infinity", Equal);
    // overflow and underflow: `Nearest` and `Up` reach the infinity, `Down` stops at the largest
    // finite value
    test(
        "1e1000000000000000000",
        10,
        Nearest,
        53,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "-1e1000000000000000000",
        10,
        Nearest,
        53,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "1e1000000000000000000",
        10,
        Down,
        53,
        "2.0985787164673875e323228496",
        "0x7.ffffffffffffcE+268435455#53",
        Less,
    );
    test(
        "1e-1000000000000000000",
        10,
        Nearest,
        53,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "-1e-1000000000000000000",
        10,
        Nearest,
        53,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "1e-1000000000000000000",
        10,
        Up,
        53,
        "2.3825649048879511e-323228497",
        "0x1.0000000000000E-268435456#53",
        Greater,
    );
    // An exponent so large that the digit count cannot even be added to it. Only an upward overflow
    // is possible, since there is always at least one digit to add.
    // - `i64::exact_from(digits.len()).checked_add(exponent)` is `None`
    test(
        "1e9223372036854775807",
        10,
        Nearest,
        53,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "-1e9223372036854775807",
        10,
        Nearest,
        53,
        "-Infinity",
        "-Infinity",
        Less,
    );
    // The same exponent one lower, with a digit moved past the point, lands on `i64::MAX` exactly
    // instead: the addition succeeds and the overflow happens in the core.
    test(
        "1.5e9223372036854775806",
        10,
        Nearest,
        53,
        "Infinity",
        "Infinity",
        Greater,
    );
    // The other end, where the core underflows to a zero rather than the parser overflowing.
    test(
        "1e-9223372036854775808",
        10,
        Nearest,
        53,
        "0.0",
        "0x0.0",
        Less,
    );
    // strings that are not numbers
    test_none("", 10);
    test_none(" ", 10);
    test_none("abc", 10);
    test_none("1.2.3", 10);
    test_none("0x1", 10);
    test_none("e5", 10);
    test_none("1e", 10);
    test_none("--1", 10);
    test_none("1 ", 10);
    test_none("2", 2);
    test_none("g", 16);
    // the spellings MPFR accepts for the specials and Malachite does not
    test_none("nan", 10);
    test_none("inf", 10);
    test_none("@nan@", 10);
}

#[test]
fn test_from_sci_string() {
    fn test(s: &str, base: u8, out: &str, out_hex: &str) {
        let options = options_for(base, Nearest);
        let x = Float::from_sci_string_with_options(s, options).unwrap();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        if base == 10 {
            assert_eq!(
                Float::from_sci_string(s).map(ComparableFloat),
                Some(ComparableFloat(x))
            );
        }
    }
    // The specials and the zeros carry no precision, so there is nothing to infer.
    test("NaN", 10, "NaN", "NaN");
    test("Infinity", 10, "Infinity", "Infinity");
    test("-Infinity", 10, "-Infinity", "-Infinity");
    test("0", 10, "0.0", "0x0.0");
    test("-0", 10, "-0.0", "-0x0.0");
    // An exactly representable value is stored in the fewest bits that represent it, so these agree
    // with `Float::from(1.5)` and `Float::from(255)`.
    test("1", 10, "1.0", "0x1.0#1");
    test("1.5", 10, "1.5", "0x1.8#2");
    test("255", 10, "255.0", "0xff.0#8");
    test("0.5", 10, "0.50", "0x0.8#1");
    // Trailing zero bits are not part of the minimal representation either, so these are 5 and 1
    // bits rather than 7 and 11.
    test("100", 10, "100.0", "0x64.0#5");
    test("1024", 10, "1.0e3", "0x4.0E+2#1");
    // Trailing zeros raise the implied precision but not the value, so it is shrunk right back.
    test("1.0000000000000000", 10, "1.0", "0x1.0#1");
    // A value that is not exactly representable keeps the precision its digits imply, which for
    // short inputs is coarse: one decimal digit buys only four bits.
    test("0.1", 10, "0.102", "0x0.1a#4");
    test("3.14159", 10, "3.1415901", "0x3.243f4#20");
    // The precision comes from the digits alone, so a huge exponent does not raise it. That is what
    // keeps such a string cheap to read.
    test("1e10", 10, "9.66e9", "0x2.4E+8#4");
    test("1e100000000", 10, "9.80e99999999", "0x2.cE+83048202#4");
    // An exponent too large to add the digit count to. The implied precision does not matter here,
    // since rounding to nearest gives an infinity whatever it is.
    // - `Parsed::Overflow`
    test("1e9223372036854775807", 10, "Infinity", "Infinity");
    test("-1e9223372036854775807", 10, "-Infinity", "-Infinity");
    // A value too small to represent underflows to a zero, which carries no precision to infer.
    test("1e-9223372036854775808", 10, "0.0", "0x0.0");
    // In base 2 a digit is a bit, so the implied precision is exactly the digit count.
    test("1.0", 2, "1.0", "0x1.0#1");
    test("1.01", 2, "1.2", "0x1.4#3");
    test("ff", 16, "255.0", "0xff.0#8");
}

#[test]
fn from_sci_string_fail() {
    // `Exact` panics when the value is not exactly representable with the given precision.
    assert_panic!(Float::from_sci_string_with_options_prec(
        "0.1",
        options_for(10, Exact),
        10
    ));
    assert_panic!(Float::from_sci_string_prec_round("1.1", 1, Exact));
    // A precision of zero is not a `Float` precision.
    assert_panic!(Float::from_sci_string_prec("1", 0));
    assert_panic!(Float::from_sci_string_prec_round("1", 0, Nearest));
    assert_panic!(Float::from_sci_string_with_options_prec(
        "1",
        FromSciStringOptions::default(),
        0
    ));
    let _ = Float::ZERO;
}

#[test]
fn from_sci_string_properties() {
    // Valid input: the whole invariant set, including the cross-check against the MPFR port.
    string_from_sci_string_options_unsigned_triple_gen_var_1().test_properties(
        |(s, options, prec)| {
            assert!(Float::from_sci_string_with_options_prec(&s, options, prec).is_some());
            verify_from_sci_string(&s, options, prec);
        },
    );

    // Mostly-invalid strings, for the parser's rejection paths.
    string_from_sci_string_options_unsigned_triple_gen_var_2().test_properties(
        |(s, options, prec)| {
            verify_from_sci_string(&s, options, prec);
        },
    );

    // An independent oracle. This generator excludes the large exponents that would make the
    // `Rational` enormous, which is exactly what the oracle needs.
    string_from_sci_string_options_pair_gen_var_3().test_properties(|(s, options)| {
        for prec in [1u64, 2, 10, 53] {
            for rm in INEXACT_MODES {
                let options = with_mode(options, rm);
                if let Some(expected) = rational_oracle(&s, options, prec) {
                    assert_eq!(
                        Float::from_sci_string_with_options_prec(&s, options, prec)
                            .map(|(x, o)| (ComparableFloat(x), o)),
                        Some(expected),
                        "{s:?} base {} {rm} prec {prec}",
                        options.get_base()
                    );
                }
            }
        }
    });
}

#[test]
fn from_sci_string_round_trip_properties() {
    // A `Float`'s `Display` writes enough digits to identify it, so reading them back at its own
    // precision must give it exactly. This is the property the digit count is chosen for.
    fn round_trip(x: Float) {
        let Some(prec) = x.get_prec() else {
            return;
        };
        let s = x.to_string();
        let (y, o) = Float::from_sci_string_prec(&s, prec).unwrap();
        assert_eq!(ComparableFloat(y), ComparableFloat(x));
        // The digits are a rounded rendering, so the ternary value need not be `Equal`; what
        // round-trips is the value.
        let _ = o;
    }
    float_gen().test_properties(round_trip);
    // Extreme exponents and precisions, where the digit count and the exponent are largest.
    float_gen_var_12().test_properties(round_trip);
}

#[test]
fn from_sci_string_inferred_precision_properties() {
    // Without a precision the digits imply one: as many bits as they can carry, and then, when the
    // value they name is exactly representable, the fewest bits that represent it, which is what
    // `Float::from` does for a primitive float.
    float_gen().test_properties(|x| {
        let Some(prec) = x.get_prec() else {
            return;
        };
        let s = x.to_string();
        let y = Float::from_sci_string(&s).unwrap();
        let y_prec = y.get_prec().unwrap();
        // Reading the same string at the precision that was inferred gives the same value back.
        let (z, o) = Float::from_sci_string_prec(&s, y_prec).unwrap();
        assert_eq!(ComparableFloatRef(&z), ComparableFloatRef(&y), "{s:?}");
        // The shrink happens exactly when the digits are exactly representable at the implied
        // precision. A rounded value keeps that precision instead, since its low bits are not the
        // string's, so its mantissa may well end in a zero bit.
        if o == Equal {
            assert_eq!(
                y_prec,
                (&y).integer_mantissa().significant_bits(),
                "{s:?} -> {y}"
            );
        }
        // Reading at the original precision instead gives the original value back, so the inferred
        // reading is the same value at whatever precision the digits alone justify.
        let (w, _) = Float::from_sci_string_prec(&s, prec).unwrap();
        assert_eq!(ComparableFloat(w), ComparableFloat(x));
    });

    // An integer's digits name it exactly, so there is nothing to round and the shrink always
    // applies. That makes reading one the same as converting it, precision and all -- the agreement
    // with `Float::from` that the shrink exists for.
    unsigned_gen::<u32>().test_properties(|n| {
        assert_eq!(
            Float::from_sci_string(&n.to_string()).map(ComparableFloat),
            Some(ComparableFloat(Float::from(n)))
        );
    });
}
