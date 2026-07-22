// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::string::options::ToSciOptions;
use malachite_base::num::conversion::traits::{ExactFrom, ToSci};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_float::Float;
use malachite_float::conversion::string::to_sci::{to_sci_string, to_sci_valid};
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::float_to_sci_options_pair_gen_var_1;
use malachite_q::Rational;
use std::panic::catch_unwind;

// Applies the `Float` `Display` convention to a `Rational` sci string: the mantissa must contain a
// point, so `.0` is inserted before the exponent, or appended, when it lacks one. Only valid for
// bases up to 14, where the first 'e' or 'E' in the string is unambiguously the exponent indicator.
fn insert_point(s: &str) -> String {
    if let Some(i) = s.find(['e', 'E']) {
        if s[..i].contains('.') {
            s.to_string()
        } else {
            format!("{}.0{}", &s[..i], &s[i..])
        }
    } else if s.contains('.') {
        s.to_string()
    } else {
        format!("{s}.0")
    }
}

#[test]
fn test_to_sci_string() {
    // to_sci_string with options built from the default by `configure`.
    fn test(s: &str, s_hex: &str, configure: &dyn Fn(&mut ToSciOptions), out: &str) {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let mut options = ToSciOptions::default();
        configure(&mut options);
        assert_eq!(to_sci_string(&x, options), out, "{s} {options:?}");
    }
    let default = &|_: &mut ToSciOptions| {};

    // specials and zeros
    test("NaN", "NaN", default, "NaN");
    test("Infinity", "Infinity", default, "Infinity");
    test("-Infinity", "-Infinity", default, "-Infinity");
    test("0.0", "0x0.0", default, "0.0");
    test("-0.0", "-0x0.0", default, "-0.0");
    test(
        "0.0",
        "0x0.0",
        &|o| {
            o.set_precision(3);
            o.set_include_trailing_zeros(true);
        },
        "0.00",
    );
    test(
        "-0.0",
        "-0x0.0",
        &|o| {
            o.set_scale(2);
            o.set_include_trailing_zeros(true);
        },
        "-0.00",
    );

    // default options (precision 16, trailing zeros trimmed, threshold -6): these agree with the
    // current shortest-round-trip Display output...
    test("1.5", "0x1.8#2", default, "1.5");
    test("-1.5", "-0x1.8#2", default, "-1.5");
    test("1.0", "0x1.0#1", default, "1.0");
    test("255.0", "0xff.0#8", default, "255.0");
    test("1000000.0", "0xf4240.0#20", default, "1000000.0");
    test("1234.5", "0x4d2.8#12", default, "1234.5");
    test("0.50", "0x0.8#1", default, "0.5");
    // ...but the digits may differ: Display shows the shortest string that round-trips, while these
    // are the value's actual digits. The prec-1 float 8.0e-6 is exactly 2^-17...
    test("7.6e-6", "0x0.00008#1", default, "7.62939453125e-6");
    // ...and the shortest string that round-trips this prec-53 value is "0.0012340000000000001",
    // while rounding it to 16 significant digits gives back the digits of 0.001234.
    test(
        "0.0012340000000000001",
        "0x0.0050df15a4acf314#53",
        default,
        "0.001234",
    );

    // precision
    test("1234.5", "0x4d2.8#12", &|o| o.set_precision(2), "1.2e3");
    test("1234.5", "0x4d2.8#12", &|o| o.set_precision(4), "1234.0");
    test("1.5", "0x1.8#2", &|o| o.set_precision(1), "2.0");
    test(
        "1.5",
        "0x1.8#2",
        &|o| {
            o.set_precision(1);
            o.set_rounding_mode(Down);
        },
        "1.0",
    );
    test(
        "1.5",
        "0x1.8#2",
        &|o| {
            o.set_precision(5);
            o.set_include_trailing_zeros(true);
        },
        "1.5000",
    );
    // rounding up to a power of the base increases the exponent
    test("9.50", "0x9.8#5", &|o| o.set_precision(1), "1.0e1");

    // scale
    test("1.5", "0x1.8#2", &|o| o.set_scale(0), "2.0");
    test(
        "1.5",
        "0x1.8#2",
        &|o| {
            o.set_scale(0);
            o.set_rounding_mode(Down);
        },
        "1.0",
    );
    test(
        "1.5",
        "0x1.8#2",
        &|o| {
            o.set_scale(3);
            o.set_include_trailing_zeros(true);
        },
        "1.500",
    );
    test(
        "-1.5",
        "-0x1.8#2",
        &|o| {
            o.set_scale(0);
            o.set_rounding_mode(Floor);
        },
        "-2.0",
    );
    test(
        "-1.5",
        "-0x1.8#2",
        &|o| {
            o.set_scale(0);
            o.set_rounding_mode(Ceiling);
        },
        "-1.0",
    );
    // values that round to 0 or to 1 in the last place
    test("0.25", "0x0.4#1", &|o| o.set_scale(0), "0.0");
    test(
        "0.25",
        "0x0.4#1",
        &|o| {
            o.set_scale(0);
            o.set_rounding_mode(Up);
        },
        "1.0",
    );
    // a tie rounds to the even option, 0
    test("0.50", "0x0.8#1", &|o| o.set_scale(0), "0.0");
    test("0.75", "0x0.c#2", &|o| o.set_scale(0), "1.0");
    test(
        "-4.9276e-6",
        "-0x0.000052ac#13",
        &|o| o.set_scale(2),
        "-0.0",
    );

    // complete
    test("1.5", "0x1.8#2", &|o| o.set_size_complete(), "1.5");
    test(
        "0.00098",
        "0x0.004#1",
        &|o| o.set_size_complete(),
        "0.0009765625",
    );
    test("255.0", "0xff.0#8", &|o| o.set_size_complete(), "255.0");

    // bases
    test("5.0", "0x5.0#3", &|o| o.set_base(2), "101.0");
    test("255.0", "0xff.0#8", &|o| o.set_base(16), "ff.0");
    test(
        "255.0",
        "0xff.0#8",
        &|o| {
            o.set_base(16);
            o.set_uppercase();
        },
        "FF.0",
    );
    test("0.50", "0x0.8#1", &|o| o.set_base(16), "0.8");
    // in bases 15 and up, a positive exponent always gets an explicit sign, to distinguish the
    // exponent indicator from the digit 'e'
    test(
        "1000000.0",
        "0xf4240.0#20",
        &|o| {
            o.set_base(16);
            o.set_precision(2);
        },
        "f.4e+4",
    );
    // bases above 16 use digits past 'f'
    test("255.0", "0xff.0#8", &|o| o.set_base(20), "cf.0");
    test("1.5", "0x1.8#2", &|o| o.set_base(20), "1.a");
    test(
        "1.5",
        "0x1.8#2",
        &|o| {
            o.set_base(20);
            o.set_uppercase();
        },
        "1.A",
    );
    // in base 20 and up, 'e' is a digit; the exponent sign rule keeps it distinguishable from the
    // exponent indicator, as in "e.0e+2" (14 * 20^2)
    test("14.0", "0xe.0#4", &|o| o.set_base(20), "e.0");
    test(
        "5600.0",
        "0x1.5eE+3#8",
        &|o| {
            o.set_base(20);
            o.set_precision(1);
        },
        "e.0e+2",
    );
    test(
        "5600.0",
        "0x1.5eE+3#8",
        &|o| {
            o.set_base(20);
            o.set_precision(1);
            o.set_e_uppercase();
        },
        "e.0E+2",
    );
    // an odd base: 1/2 is non-terminating (0.5 = 0.aaa... in base 21), and rounding the repeating
    // tail is a tie in every position, which get_str resolves upward here
    test(
        "3.0",
        "0x3.0#2",
        &|o| {
            o.set_base(21);
            o.set_size_complete();
        },
        "3.0",
    );
    test("1.5", "0x1.8#2", &|o| o.set_base(21), "1.aaaaaaaaaaaaaab");
    test(
        "1.5",
        "0x1.8#2",
        &|o| {
            o.set_base(21);
            o.set_precision(3);
        },
        "1.ab",
    );
    // base 32 is a power of 2, so any Float is exactly representable
    test("255.0", "0xff.0#8", &|o| o.set_base(32), "7v.0");
    test("0.50", "0x0.8#1", &|o| o.set_base(32), "0.g");
    test(
        "0.00098",
        "0x0.004#1",
        &|o| {
            o.set_base(32);
            o.set_size_complete();
        },
        "0.01",
    );
    test(
        "1.5",
        "0x1.8#2",
        &|o| {
            o.set_base(32);
            o.set_precision(2);
            o.set_rounding_mode(Exact);
        },
        "1.g",
    );
    // 2^-17 is 8 * 32^-4, above the default -6 threshold in base 32 but below a -3 one (with no
    // explicit '+' on the negative exponent, even in a base above 14)
    test("7.6e-6", "0x0.00008#1", &|o| o.set_base(32), "0.0008");
    test(
        "7.6e-6",
        "0x0.00008#1",
        &|o| {
            o.set_base(32);
            o.set_neg_exp_threshold(-3);
        },
        "8.0e-4",
    );
    // base 36, the largest
    test("255.0", "0xff.0#8", &|o| o.set_base(36), "73.0");
    test("35.0", "0x23.0#6", &|o| o.set_base(36), "z.0");
    test(
        "35.0",
        "0x23.0#6",
        &|o| {
            o.set_base(36);
            o.set_uppercase();
        },
        "Z.0",
    );
    test("0.50", "0x0.8#1", &|o| o.set_base(36), "0.i");
    test(
        "0.50",
        "0x0.8#1",
        &|o| {
            o.set_base(36);
            o.set_size_complete();
        },
        "0.i",
    );

    // exponent formatting
    test(
        "7.6e-6",
        "0x0.00008#1",
        &|o| o.set_e_uppercase(),
        "7.62939453125E-6",
    );
    test(
        "8.8290e30",
        "0x6.f70E+25#13",
        &|o| o.set_precision(3),
        "8.83e30",
    );
    test(
        "8.8290e30",
        "0x6.f70E+25#13",
        &|o| {
            o.set_precision(3);
            o.set_force_exponent_plus_sign(true);
        },
        "8.83e+30",
    );
    test(
        "7.6e-6",
        "0x0.00008#1",
        &|o| o.set_neg_exp_threshold(-10),
        "0.00000762939453125",
    );

    // Exact is allowed whenever the value is representable in the requested digits
    test(
        "1.5",
        "0x1.8#2",
        &|o| {
            o.set_precision(2);
            o.set_rounding_mode(Exact);
        },
        "1.5",
    );
    test(
        "0.50",
        "0x0.8#1",
        &|o| {
            o.set_scale(1);
            o.set_rounding_mode(Exact);
        },
        "0.5",
    );
}

#[test]
fn test_to_sci_string_panics() {
    // Exact panics when the value is not representable in the requested digits...
    assert_panic!({
        let mut options = ToSciOptions::default();
        options.set_precision(1);
        options.set_rounding_mode(Exact);
        to_sci_string(&Float::from(1.5), options)
    });
    // ...including when it rounds all the way to 0 or 1 in the last place
    assert_panic!({
        let mut options = ToSciOptions::default();
        options.set_scale(0);
        options.set_rounding_mode(Exact);
        to_sci_string(&Float::from(0.5), options)
    });
    // a fractional value has a non-terminating expansion in an odd base
    assert_panic!({
        let mut options = ToSciOptions::default();
        options.set_base(3);
        options.set_size_complete();
        to_sci_string(&Float::from(0.5), options)
    });
}

#[test]
fn test_to_sci_valid() {
    fn test(s: &str, s_hex: &str, configure: &dyn Fn(&mut ToSciOptions), out: bool) {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let mut options = ToSciOptions::default();
        configure(&mut options);
        assert_eq!(to_sci_valid(&x, options), out, "{s} {options:?}");
    }
    let default = &|_: &mut ToSciOptions| {};

    // specials are always representable
    test("NaN", "NaN", default, true);
    test("Infinity", "Infinity", default, true);
    test("0.0", "0x0.0", &|o| o.set_size_complete(), true);
    // non-Exact conversions with a precision or scale always succeed
    test("1.5", "0x1.8#2", default, true);
    test("1.5", "0x1.8#2", &|o| o.set_precision(1), true);
    // Exact requires the digits to suffice
    test(
        "1.5",
        "0x1.8#2",
        &|o| {
            o.set_precision(1);
            o.set_rounding_mode(Exact);
        },
        false,
    );
    test(
        "1.5",
        "0x1.8#2",
        &|o| {
            o.set_precision(2);
            o.set_rounding_mode(Exact);
        },
        true,
    );
    test(
        "0.50",
        "0x0.8#1",
        &|o| {
            o.set_scale(0);
            o.set_rounding_mode(Exact);
        },
        false,
    );
    test(
        "0.50",
        "0x0.8#1",
        &|o| {
            o.set_scale(1);
            o.set_rounding_mode(Exact);
        },
        true,
    );
    // Complete requires a terminating expansion: any dyadic rational terminates in an even base,
    // and only integers terminate in an odd base
    test("0.50", "0x0.8#1", &|o| o.set_size_complete(), true);
    test(
        "0.50",
        "0x0.8#1",
        &|o| {
            o.set_base(3);
            o.set_size_complete();
        },
        false,
    );
    test(
        "3.0",
        "0x3.0#2",
        &|o| {
            o.set_base(3);
            o.set_size_complete();
        },
        true,
    );
    // the same rules in bases above 16: 32 is a power of 2, 21 is odd
    test(
        "0.50",
        "0x0.8#1",
        &|o| {
            o.set_base(32);
            o.set_size_complete();
        },
        true,
    );
    test(
        "0.50",
        "0x0.8#1",
        &|o| {
            o.set_base(21);
            o.set_size_complete();
        },
        false,
    );
    test(
        "3.0",
        "0x3.0#2",
        &|o| {
            o.set_base(21);
            o.set_size_complete();
        },
        true,
    );
    test(
        "1.5",
        "0x1.8#2",
        &|o| {
            o.set_base(21);
            o.set_precision(5);
            o.set_rounding_mode(Exact);
        },
        false,
    );
    test(
        "1.5",
        "0x1.8#2",
        &|o| {
            o.set_base(32);
            o.set_precision(2);
            o.set_rounding_mode(Exact);
        },
        true,
    );
}

#[test]
fn to_sci_string_properties() {
    float_to_sci_options_pair_gen_var_1().test_properties(|(x, options)| {
        let s = to_sci_string(&x, options);
        assert!(s.is_ascii());
        assert!(to_sci_valid(&x, options));
        if x.is_nan() {
            assert_eq!(s, "NaN");
        } else {
            assert_eq!(s.starts_with('-'), x.is_sign_negative());
            if x.is_infinite() {
                assert_eq!(s.trim_start_matches('-'), "Infinity");
            } else {
                // the Float Display convention: a finite value always shows a point
                assert!(s.contains('.'), "{s:?}");
            }
        }
        // Cross-check against Rational::to_sci, which must produce the same string up to the
        // inserted point. The comparison is gated to bases whose digits cannot include 'e' (so the
        // exponent position is unambiguous) and to moderate exponents (`Rational::exact_from` of a
        // Float with an extreme exponent is enormous).
        if let Some(exponent) = x.get_exponent()
            && exponent.unsigned_abs() <= 10_000
            && options.get_base() <= 14
        {
            let q = Rational::exact_from(&x);
            assert!(q.fmt_sci_valid(options));
            assert_eq!(
                s,
                insert_point(&q.to_sci_with_options(options).to_string()),
                "{x} {options:?}"
            );
        }
    });
}
