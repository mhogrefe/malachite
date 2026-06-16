// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    LogBase2, LogBasePowerOf2, LogBasePowerOf2Assign, PowerOf2, Reciprocal,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::signed_rounding_mode_pair_gen_var_3;
use malachite_float::test_util::arithmetic::log_base_power_of_2::{
    rug_log_base_power_of_2, rug_log_base_power_of_2_prec, rug_log_base_power_of_2_prec_round,
    rug_log_base_power_of_2_rational_prec, rug_log_base_power_of_2_rational_prec_round,
    rug_log_base_power_of_2_round,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_signed_rounding_mode_triple_gen_var_7, float_signed_rounding_mode_triple_gen_var_8,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_5,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_6,
    rational_signed_unsigned_rounding_mode_quadruple_gen_var_1,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::platform::Limb;
use malachite_q::Rational;
use malachite_q::test_util::generators::rational_signed_unsigned_triple_gen_var_2;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_log_base_power_of_2() {
    let test = |s, s_hex, pow: i64, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let log = x.clone().log_base_power_of_2(pow);
        assert!(log.is_valid());

        assert_eq!(log.to_string(), out);
        assert_eq!(to_hex_string(&log), out_hex);

        let log_alt = (&x).log_base_power_of_2(pow);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&log_alt));

        let mut log_alt = x.clone();
        log_alt.log_base_power_of_2_assign(pow);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&log_alt));

        // log_{2^pow}(x) = log_2(x) / pow, except x = 1 always gives +0.0 (MPFR may give -0.0 for
        // negative pow).
        assert_eq!(
            ComparableFloatRef(
                &Float::from(&rug_log_base_power_of_2(&rug::Float::exact_from(&x), pow))
                    .abs_negative_zero()
            ),
            ComparableFloatRef(&log.abs_negative_zero_ref())
        );
    };
    test("NaN", "NaN", 1, "NaN", "NaN");
    test("NaN", "NaN", -1, "NaN", "NaN");
    test("Infinity", "Infinity", 1, "Infinity", "Infinity");
    test("Infinity", "Infinity", -1, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", 1, "NaN", "NaN");
    test("-Infinity", "-Infinity", -1, "NaN", "NaN");
    test("0.0", "0x0.0", 1, "-Infinity", "-Infinity");
    test("0.0", "0x0.0", -1, "Infinity", "Infinity");
    test("-0.0", "-0x0.0", 1, "-Infinity", "-Infinity");
    test("-0.0", "-0x0.0", -1, "Infinity", "Infinity");
    test("1.0", "0x1.0#1", 1, "0.0", "0x0.0");
    test("1.0", "0x1.0#1", 2, "0.0", "0x0.0");
    test("1.0", "0x1.0#1", -3, "0.0", "0x0.0");
    test("-1.0", "-0x1.0#1", 1, "NaN", "NaN");
    test("-1.0", "-0x1.0#1", 2, "NaN", "NaN");
    test("2.0", "0x2.0#1", 1, "1.0", "0x1.0#1");
    test("8.0", "0x8.0#1", 2, "2.0", "0x2.0#1");
    test("2.0e1", "0x1.0E+1#1", 2, "2.0", "0x2.0#1");
    test("4.0", "0x4.0#1", 2, "1.0", "0x1.0#1");
    test("4.0", "0x4.0#2", 3, "0.8", "0x0.c#2");
    test("6.0e1", "0x4.0E+1#1", 3, "2.0", "0x2.0#1");
    test("123.0", "0x7b.0#7", 1, "6.94", "0x6.f#7");
    test("123.0", "0x7b.0#7", 2, "3.47", "0x3.78#7");
    test("123.0", "0x7b.0#7", -1, "-6.94", "-0x6.f#7");
    test("-123.0", "-0x7b.0#7", 2, "NaN", "NaN");
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        2,
        "0.8257480647361594",
        "0x0.d36439a4c6efb8#53",
    );
}

#[test]
fn test_log_base_power_of_2_prec() {
    let test = |s, s_hex, pow: i64, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (log, o) = x.clone().log_base_power_of_2_prec(pow, prec);
        assert!(log.is_valid());

        assert_eq!(log.to_string(), out);
        assert_eq!(to_hex_string(&log), out_hex);
        assert_eq!(o, o_out);

        let (log_alt, o_alt) = x.log_base_power_of_2_prec_ref(pow, prec);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&log_alt));
        assert_eq!(o_alt, o);

        let mut log_alt = x.clone();
        let o_alt = log_alt.log_base_power_of_2_prec_assign(pow, prec);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&log_alt));
        assert_eq!(o_alt, o);

        let (rug_log, rug_o) = rug_log_base_power_of_2_prec(&rug::Float::exact_from(&x), pow, prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log).abs_negative_zero()),
            ComparableFloatRef(&log.abs_negative_zero_ref()),
        );
        assert_eq!(rug_o, o);
    };
    test("NaN", "NaN", 1, 1, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, 1, "Infinity", "Infinity", Equal);
    test(
        "Infinity",
        "Infinity",
        -1,
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-Infinity", "-Infinity", 1, 1, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, 1, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", -1, 1, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", -2, 1, "Infinity", "Infinity", Equal);
    test("1.0", "0x1.0#1", 1, 1, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 3, 10, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 1, 10, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 1, 1, "1.0", "0x1.0#1", Equal);
    test("8.0", "0x8.0#1", 2, 2, "1.5", "0x1.8#2", Equal);
    test("8.0", "0x8.0#1", 2, 10, "1.5", "0x1.800#10", Equal);
    test("2.0e1", "0x1.0E+1#1", 2, 1, "2.0", "0x2.0#1", Equal);
    test("4.0", "0x4.0#1", 3, 1, "0.5", "0x0.8#1", Less);
    test("4.0", "0x4.0#1", 3, 10, "0.667", "0x0.aac#10", Greater);
    test("123.0", "0x7b.0#7", 2, 10, "3.473", "0x3.79#10", Greater);
    test("123.0", "0x7b.0#7", 8, 10, "0.868", "0x0.de4#10", Greater);
    test("123.0", "0x7b.0#7", 10, 10, "0.694", "0x0.b1c#10", Greater);
    test("-123.0", "-0x7b.0#7", 2, 10, "NaN", "NaN", Equal);
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        2,
        10,
        "0.826",
        "0x0.d38#10",
        Greater,
    );
}

#[test]
fn log_base_power_of_2_prec_fail() {
    assert_panic!(Float::NAN.log_base_power_of_2_prec(1, 0));
    assert_panic!(Float::NAN.log_base_power_of_2_prec_ref(1, 0));
    assert_panic!(Float::NAN.log_base_power_of_2_prec(0, 1));
    assert_panic!(Float::NAN.log_base_power_of_2_prec_ref(0, 1));
    assert_panic!({
        let mut x = Float::NAN;
        x.log_base_power_of_2_prec_assign(1, 0)
    });
    assert_panic!({
        let mut x = Float::NAN;
        x.log_base_power_of_2_prec_assign(0, 1)
    });
}

#[test]
fn test_log_base_power_of_2_round() {
    let test = |s, s_hex, pow: i64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (log, o) = x.clone().log_base_power_of_2_round(pow, rm);
        assert!(log.is_valid());

        assert_eq!(log.to_string(), out);
        assert_eq!(to_hex_string(&log), out_hex);
        assert_eq!(o, o_out);

        let (log_alt, o_alt) = x.log_base_power_of_2_round_ref(pow, rm);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&log_alt));
        assert_eq!(o_alt, o);

        let mut log_alt = x.clone();
        let o_alt = log_alt.log_base_power_of_2_round_assign(pow, rm);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&log_alt));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_log, rug_o) =
                rug_log_base_power_of_2_round(&rug::Float::exact_from(&x), pow, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_log).abs_negative_zero()),
                ComparableFloatRef(&log.abs_negative_zero_ref()),
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 1, Floor, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity",
        "Infinity",
        -1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-Infinity", "-Infinity", 1, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, Floor, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", -1, Floor, "Infinity", "Infinity", Equal);
    test("1.0", "0x1.0#1", 1, Exact, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 1, Exact, "1.0", "0x1.0#1", Equal);
    test("8.0", "0x8.0#2", 2, Exact, "1.5", "0x1.8#2", Equal);
    test("123.0", "0x7b.0#7", 2, Floor, "3.47", "0x3.78#7", Less);
    test("123.0", "0x7b.0#7", 2, Ceiling, "3.5", "0x3.80#7", Greater);
    test("123.0", "0x7b.0#7", 2, Down, "3.47", "0x3.78#7", Less);
    test("123.0", "0x7b.0#7", 2, Up, "3.5", "0x3.80#7", Greater);
    test("123.0", "0x7b.0#7", 2, Nearest, "3.47", "0x3.78#7", Less);
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        2,
        Floor,
        "0.8257480647361594",
        "0x0.d36439a4c6efb8#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        2,
        Nearest,
        "0.8257480647361594",
        "0x0.d36439a4c6efb8#53",
        Less,
    );
}

#[test]
fn log_base_power_of_2_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(THREE.log_base_power_of_2_round(0, Floor));
    assert_panic!(THREE.log_base_power_of_2_round_ref(0, Floor));
    // - 3 is not a power of 2, so its base-2 logarithm is irrational
    assert_panic!(THREE.log_base_power_of_2_round(2, Exact));
    assert_panic!(THREE.log_base_power_of_2_round_ref(2, Exact));
    assert_panic!({
        let mut x = THREE;
        x.log_base_power_of_2_round_assign(2, Exact);
    });
    // - log_8(4) = 2/3, which is not representable
    const FOUR: Float = Float::const_from_unsigned(4);
    assert_panic!(FOUR.log_base_power_of_2_round(3, Exact));
    assert_panic!(FOUR.log_base_power_of_2_round_ref(3, Exact));
    assert_panic!({
        let mut x = FOUR;
        x.log_base_power_of_2_round_assign(3, Exact);
    });
}

#[test]
fn test_log_base_power_of_2_prec_round() {
    let test = |s, s_hex, pow: i64, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (log, o) = x.clone().log_base_power_of_2_prec_round(pow, prec, rm);
        assert!(log.is_valid());

        assert_eq!(log.to_string(), out);
        assert_eq!(to_hex_string(&log), out_hex);
        assert_eq!(o, o_out);

        let (log_alt, o_alt) = x.log_base_power_of_2_prec_round_ref(pow, prec, rm);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&log_alt));
        assert_eq!(o_alt, o);

        let mut log_alt = x.clone();
        let o_alt = log_alt.log_base_power_of_2_prec_round_assign(pow, prec, rm);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&log_alt));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_log, rug_o) =
                rug_log_base_power_of_2_prec_round(&rug::Float::exact_from(&x), pow, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_log).abs_negative_zero()),
                ComparableFloatRef(&log.abs_negative_zero_ref()),
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 1, 1, Floor, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 1, 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity",
        "Infinity",
        -1,
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-Infinity", "-Infinity", 1, 1, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, 1, Floor, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", -1, 1, Floor, "Infinity", "Infinity", Equal);
    test(
        "-0.0",
        "-0x0.0",
        2,
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("1.0", "0x1.0#1", 1, 1, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", -3, 10, Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 1, 1, Floor, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 1, 1, Exact, "1.0", "0x1.0#1", Equal);
    test("8.0", "0x8.0#1", 2, 2, Exact, "1.5", "0x1.8#2", Equal);
    test("2.0e1", "0x1.0E+1#1", 2, 1, Exact, "2.0", "0x2.0#1", Equal);
    test("6.0e1", "0x4.0E+1#1", 3, 1, Exact, "2.0", "0x2.0#1", Equal);
    test("4.0", "0x4.0#1", 3, 1, Floor, "0.5", "0x0.8#1", Less);
    test("4.0", "0x4.0#1", 3, 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test(
        "4.0",
        "0x4.0#1",
        3,
        10,
        Nearest,
        "0.667",
        "0x0.aac#10",
        Greater,
    );
    test("123.0", "0x7b.0#7", 2, 1, Floor, "2.0", "0x2.0#1", Less);
    test(
        "123.0", "0x7b.0#7", 2, 1, Ceiling, "4.0", "0x4.0#1", Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        2,
        10,
        Floor,
        "3.469",
        "0x3.78#10",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        2,
        10,
        Ceiling,
        "3.473",
        "0x3.79#10",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        -2,
        10,
        Floor,
        "-3.473",
        "-0x3.79#10",
        Less,
    );
    test("-123.0", "-0x7b.0#7", 2, 10, Floor, "NaN", "NaN", Equal);
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        2,
        10,
        Floor,
        "0.825",
        "0x0.d34#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        2,
        10,
        Nearest,
        "0.826",
        "0x0.d38#10",
        Greater,
    );
}

#[test]
fn log_base_power_of_2_prec_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(Float::one_prec(1).log_base_power_of_2_prec_round(1, 0, Floor));
    assert_panic!(Float::one_prec(1).log_base_power_of_2_prec_round_ref(1, 0, Floor));
    assert_panic!(Float::one_prec(1).log_base_power_of_2_prec_round(0, 1, Floor));
    assert_panic!(Float::one_prec(1).log_base_power_of_2_prec_round_ref(0, 1, Floor));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.log_base_power_of_2_prec_round_assign(1, 0, Floor)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.log_base_power_of_2_prec_round_assign(0, 1, Floor)
    });

    // - 3 is not a power of 2, so its base-2 logarithm is irrational
    assert_panic!(THREE.log_base_power_of_2_prec_round(2, 1, Exact));
    assert_panic!(THREE.log_base_power_of_2_prec_round_ref(2, 1, Exact));
    assert_panic!({
        let mut x = THREE;
        x.log_base_power_of_2_prec_round_assign(2, 1, Exact)
    });
    // - log_8(4) = 2/3, which is not representable
    assert_panic!(Float::const_from_unsigned(4).log_base_power_of_2_prec_round(3, 100, Exact));
    assert_panic!(Float::const_from_unsigned(4).log_base_power_of_2_prec_round_ref(3, 100, Exact));
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_power_of_2_prec_round_properties_helper(
    x: Float,
    pow: i64,
    prec: u64,
    rm: RoundingMode,
) {
    let (log, o) = x.clone().log_base_power_of_2_prec_round(pow, prec, rm);
    assert!(log.is_valid());

    let (log_alt, o_alt) = x.log_base_power_of_2_prec_round_ref(pow, prec, rm);
    assert!(log_alt.is_valid());
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_power_of_2_prec_round_assign(pow, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_log, rug_o) =
            rug_log_base_power_of_2_prec_round(&rug::Float::exact_from(&x), pow, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log).abs_negative_zero()),
            ComparableFloatRef(&log.abs_negative_zero_ref()),
        );
        assert_eq!(rug_o, o);
    }

    // log_{2^1}(x) == log_2(x)
    if pow == 1 {
        let (log_2_alt, o_alt) = x.log_base_2_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&log_2_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);
    }

    if x < 0u32 {
        assert!(log.is_nan());
    }

    if log.is_normal() {
        assert_eq!(log.get_prec(), Some(prec));
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.log_base_power_of_2_prec_round_ref(pow, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(log.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.log_base_power_of_2_prec_round_ref(pow, prec, Exact));
    }
}

#[test]
fn log_base_power_of_2_prec_round_properties() {
    float_signed_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, pow, prec, rm)| {
            log_base_power_of_2_prec_round_properties_helper(x, pow, prec, rm);
        },
    );

    float_signed_unsigned_rounding_mode_quadruple_gen_var_6().test_properties(
        |(x, pow, prec, rm)| {
            log_base_power_of_2_prec_round_properties_helper(x, pow, prec, rm);
        },
    );

    signed_rounding_mode_pair_gen_var_3::<i64>().test_properties(|(pow, rm)| {
        let prec = 10;
        let (log, o) = Float::NAN.log_base_power_of_2_prec_round(pow, prec, rm);
        assert!(log.is_nan());
        assert_eq!(o, Equal);

        let (log, o) = Float::INFINITY.log_base_power_of_2_prec_round(pow, prec, rm);
        if pow > 0 {
            assert_eq!(log, Float::INFINITY);
        } else {
            assert_eq!(log, Float::NEGATIVE_INFINITY);
        }
        assert_eq!(o, Equal);

        let (log, o) = Float::NEGATIVE_INFINITY.log_base_power_of_2_prec_round(pow, prec, rm);
        assert!(log.is_nan());
        assert_eq!(o, Equal);

        let (log, o) = Float::ZERO.log_base_power_of_2_prec_round(pow, prec, rm);
        if pow > 0 {
            assert_eq!(log, Float::NEGATIVE_INFINITY);
        } else {
            assert_eq!(log, Float::INFINITY);
        }
        assert_eq!(o, Equal);

        let (log, o) = Float::NEGATIVE_ZERO.log_base_power_of_2_prec_round(pow, prec, rm);
        if pow > 0 {
            assert_eq!(log, Float::NEGATIVE_INFINITY);
        } else {
            assert_eq!(log, Float::INFINITY);
        }
        assert_eq!(o, Equal);

        let (log, o) = Float::ONE.log_base_power_of_2_prec_round(pow, prec, rm);
        assert_eq!(ComparableFloat(log), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (log, o) = Float::NEGATIVE_ONE.log_base_power_of_2_prec_round(pow, prec, rm);
        assert_eq!(ComparableFloat(log), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_power_of_2_prec_properties_helper(x: Float, pow: i64, prec: u64) {
    let (log, o) = x.clone().log_base_power_of_2_prec(pow, prec);
    assert!(log.is_valid());

    let (log_alt, o_alt) = x.log_base_power_of_2_prec_ref(pow, prec);
    assert!(log_alt.is_valid());
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_power_of_2_prec_assign(pow, prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    let (rug_log, rug_o) = rug_log_base_power_of_2_prec(&rug::Float::exact_from(&x), pow, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_log).abs_negative_zero()),
        ComparableFloatRef(&log.abs_negative_zero_ref()),
    );
    assert_eq!(rug_o, o);

    let (log_alt, o_alt) = x.log_base_power_of_2_prec_round_ref(pow, prec, Nearest);
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    if pow == 1 {
        let (log_2_alt, o_alt) = x.log_base_2_prec_ref(prec);
        assert_eq!(ComparableFloatRef(&log_2_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);
    }

    if x < 0u32 {
        assert!(log.is_nan());
    }

    if log.is_normal() {
        assert_eq!(log.get_prec(), Some(prec));
    }
}

#[test]
fn log_base_power_of_2_prec_properties() {
    float_signed_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, pow, prec, _)| {
            log_base_power_of_2_prec_properties_helper(x, pow, prec);
        },
    );

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_signed_unsigned_rounding_mode_quadruple_gen_var_5().test_properties_with_config(
        &config,
        |(x, pow, prec, _)| {
            log_base_power_of_2_prec_properties_helper(x, pow, prec);
        },
    );

    float_signed_unsigned_rounding_mode_quadruple_gen_var_6().test_properties(
        |(x, pow, prec, _)| {
            log_base_power_of_2_prec_properties_helper(x, pow, prec);
        },
    );
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_power_of_2_round_properties_helper(x: Float, pow: i64, rm: RoundingMode) {
    let (log, o) = x.clone().log_base_power_of_2_round(pow, rm);
    assert!(log.is_valid());

    let (log_alt, o_alt) = x.log_base_power_of_2_round_ref(pow, rm);
    assert!(log_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_power_of_2_round_assign(pow, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    let (log_alt, o_alt) = x.log_base_power_of_2_prec_round_ref(pow, x.significant_bits(), rm);
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    if pow == 1 {
        let (log_2_alt, o_alt) = x.log_base_2_round_ref(rm);
        assert_eq!(ComparableFloatRef(&log_2_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);
    }

    if x < 0u32 {
        assert!(log.is_nan());
    }

    if log.is_normal() {
        assert_eq!(log.get_prec(), Some(x.get_prec().unwrap()));
    }

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_log, rug_o) =
            rug_log_base_power_of_2_round(&rug::Float::exact_from(&x), pow, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log).abs_negative_zero()),
            ComparableFloatRef(&log.abs_negative_zero_ref()),
        );
        assert_eq!(rug_o, o);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.log_base_power_of_2_round_ref(pow, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(log.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.log_base_power_of_2_round_ref(pow, Exact));
    }
}

#[test]
fn log_base_power_of_2_round_properties() {
    float_signed_rounding_mode_triple_gen_var_7().test_properties(|(x, pow, rm)| {
        log_base_power_of_2_round_properties_helper(x, pow, rm);
    });

    float_signed_rounding_mode_triple_gen_var_8().test_properties(|(x, pow, rm)| {
        log_base_power_of_2_round_properties_helper(x, pow, rm);
    });

    signed_rounding_mode_pair_gen_var_3::<i64>().test_properties(|(pow, rm)| {
        let (log, o) = Float::NAN.log_base_power_of_2_round(pow, rm);
        assert!(log.is_nan());
        assert_eq!(o, Equal);

        let (log, o) = Float::INFINITY.log_base_power_of_2_round(pow, rm);
        if pow > 0 {
            assert_eq!(log, Float::INFINITY);
        } else {
            assert_eq!(log, Float::NEGATIVE_INFINITY);
        }
        assert_eq!(o, Equal);

        let (log, o) = Float::NEGATIVE_INFINITY.log_base_power_of_2_round(pow, rm);
        assert!(log.is_nan());
        assert_eq!(o, Equal);

        let (log, o) = Float::ZERO.log_base_power_of_2_round(pow, rm);
        if pow > 0 {
            assert_eq!(log, Float::NEGATIVE_INFINITY);
        } else {
            assert_eq!(log, Float::INFINITY);
        }
        assert_eq!(o, Equal);

        let (log, o) = Float::NEGATIVE_ZERO.log_base_power_of_2_round(pow, rm);
        if pow > 0 {
            assert_eq!(log, Float::NEGATIVE_INFINITY);
        } else {
            assert_eq!(log, Float::INFINITY);
        }
        assert_eq!(o, Equal);

        let (log, o) = Float::ONE.log_base_power_of_2_round(pow, rm);
        assert_eq!(ComparableFloat(log), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (log, o) = Float::NEGATIVE_ONE.log_base_power_of_2_round(pow, rm);
        assert_eq!(ComparableFloat(log), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_power_of_2_properties_helper(x: Float, pow: i64) {
    let log = x.clone().log_base_power_of_2(pow);
    assert!(log.is_valid());

    let log_alt = (&x).log_base_power_of_2(pow);
    assert!(log_alt.is_valid());
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));

    let mut x_alt = x.clone();
    x_alt.log_base_power_of_2_assign(pow);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));

    let log_alt = x
        .log_base_power_of_2_prec_round_ref(pow, x.significant_bits(), Nearest)
        .0;
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    let log_alt = x.log_base_power_of_2_prec_ref(pow, x.significant_bits()).0;
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    let log_alt = x.log_base_power_of_2_round_ref(pow, Nearest).0;
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));

    if pow == 1 {
        let log_2_alt = (&x).log_base_2();
        assert_eq!(ComparableFloatRef(&log_2_alt), ComparableFloatRef(&log));
    }

    if x < 0u32 {
        assert!(log.is_nan());
    }

    let rug_log = rug_log_base_power_of_2(&rug::Float::exact_from(&x), pow);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_log).abs_negative_zero()),
        ComparableFloatRef(&log.abs_negative_zero_ref()),
    );
}

#[test]
fn log_base_power_of_2_properties() {
    float_signed_rounding_mode_triple_gen_var_7().test_properties(|(x, pow, _)| {
        log_base_power_of_2_properties_helper(x, pow);
    });

    float_signed_rounding_mode_triple_gen_var_8().test_properties(|(x, pow, _)| {
        log_base_power_of_2_properties_helper(x, pow);
    });
}

#[test]
fn test_log_base_power_of_2_rational_prec() {
    let test = |s, pow: i64, prec, out, out_hex, out_o| {
        let u = Rational::from_str(s).unwrap();

        let (log, o) = Float::log_base_power_of_2_rational_prec(u.clone(), pow, prec);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(to_hex_string(&log), out_hex);
        assert_eq!(o, out_o);

        let (log, o) = Float::log_base_power_of_2_rational_prec_ref(&u, pow, prec);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(to_hex_string(&log), out_hex);
        assert_eq!(o, out_o);

        let (rug_log, rug_o) = rug_log_base_power_of_2_rational_prec(&u, pow, prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log)),
            ComparableFloatRef(&log),
        );
        assert_eq!(rug_o, o);
    };
    // special cases
    test("0", 2, 5, "-Infinity", "-Infinity", Equal);
    test("0", -2, 5, "Infinity", "Infinity", Equal);
    test("-1", 2, 5, "NaN", "NaN", Equal);
    test("-3/5", -2, 20, "NaN", "NaN", Equal);
    test("1", 2, 10, "0.0", "0x0.0", Equal);
    // exact cases
    test("8", 1, 10, "3.0", "0x3.00#10", Equal); // log_2(8) = 3
    test("8", 2, 10, "1.5", "0x1.800#10", Equal); // log_4(8) = 3/2, exact
    test("16", 2, 10, "2.0", "0x2.00#10", Equal); // log_4(16) = 2
    test("1/4", 2, 10, "-1.0", "-0x1.000#10", Equal); // log_4(1/4) = -1
    // log_8(4) = 2/3, not representable
    test(
        "4",
        3,
        100,
        "0.666666666666666666666666666667",
        "0x0.aaaaaaaaaaaaaaaaaaaaaaaab#100",
        Greater,
    );
    test("3/5", 2, 20, "-0.3684826", "-0x0.5e54e0#20", Greater);
    test("3/5", 3, 20, "-0.2456553", "-0x0.3ee344#20", Less);

    // Rationals astronomically close to a power of 2 must not be reported as exact.
    let test_big = |u: Rational, pow: i64, prec, out: &str, out_hex: &str, out_o| {
        let (log, o) = Float::log_base_power_of_2_rational_prec(u.clone(), pow, prec);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(to_hex_string(&log), out_hex);
        assert_eq!(o, out_o);

        let (log, o) = Float::log_base_power_of_2_rational_prec_ref(&u, pow, prec);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(to_hex_string(&log), out_hex);
        assert_eq!(o, out_o);

        let (rug_log, rug_o) = rug_log_base_power_of_2_rational_prec(&u, pow, prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log)),
            ComparableFloatRef(&log),
        );
        assert_eq!(rug_o, o);
    };
    // log_{2^1000}(2^1000) = 1, exact
    test_big(
        Rational::power_of_2(1000i64),
        1000,
        10,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    // 2^1000 is a power of 2, but 500 needs 9 bits and prec is 5, so log_4 is inexact.
    test_big(
        Rational::power_of_2(1000i64),
        2,
        5,
        "5.0e2",
        "0x1.fE+2#5",
        Less,
    );
    // 2^1000 + 1 is NOT a power of 2; even at prec 100 the result must be reported inexact.
    test_big(
        Rational::power_of_2(1000i64) + Rational::ONE,
        2,
        100,
        "500.0",
        "0x1f4.00000000000000000000000#100",
        Less,
    );
    // 2^100000 + 1: a much larger near-power-of-2. The internal full-product sizes land in the Toom
    // band below MUL_FFT_THRESHOLD at 32-bit limbs, which previously under-sized the mul-high
    // scratch and panicked; this guards that path (the result value is limb-width-independent).
    test_big(
        Rational::power_of_2(100000i64) + Rational::ONE,
        2,
        100,
        "50000.0",
        "0xc350.000000000000000000000#100",
        Less,
    );
    test_big(
        Rational::power_of_2(1000i64) - Rational::ONE,
        2,
        5,
        "5.0e2",
        "0x1.fE+2#5",
        Less,
    );
    // 1 +/- 2^-1000 is astronomically close to 1 (whose log is 0) but not equal to it.
    test_big(
        Rational::ONE + Rational::power_of_2(-1000i64),
        2,
        100,
        "6.73207397128341653511518350322e-302",
        "0xb.8aa3b295c17f0bbbe87fed07E-251#100",
        Greater,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-1000i64),
        2,
        100,
        "-6.73207397128341653511518350322e-302",
        "-0xb.8aa3b295c17f0bbbe87fed07E-251#100",
        Less,
    );
}

#[test]
fn test_log_base_power_of_2_rational_prec_round() {
    let test = |s, pow: i64, prec, rm, out, out_hex, out_o| {
        let u = Rational::from_str(s).unwrap();

        let (log, o) = Float::log_base_power_of_2_rational_prec_round(u.clone(), pow, prec, rm);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(to_hex_string(&log), out_hex);
        assert_eq!(o, out_o);

        let (log, o) = Float::log_base_power_of_2_rational_prec_round_ref(&u, pow, prec, rm);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(to_hex_string(&log), out_hex);
        assert_eq!(o, out_o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_log, rug_o) =
                rug_log_base_power_of_2_rational_prec_round(&u, pow, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_log)),
                ComparableFloatRef(&log),
            );
            assert_eq!(rug_o, o);
        }
    };
    test("8", 2, 10, Floor, "1.5", "0x1.800#10", Equal);
    test("8", 2, 10, Exact, "1.5", "0x1.800#10", Equal);
    test("3/5", 2, 20, Floor, "-0.3684831", "-0x0.5e54e8#20", Less);
    test(
        "3/5",
        2,
        20,
        Ceiling,
        "-0.3684826",
        "-0x0.5e54e0#20",
        Greater,
    );
    test("3/5", 2, 20, Down, "-0.3684826", "-0x0.5e54e0#20", Greater);
    test("3/5", 2, 20, Up, "-0.3684831", "-0x0.5e54e8#20", Less);
    test(
        "3/5",
        2,
        20,
        Nearest,
        "-0.3684826",
        "-0x0.5e54e0#20",
        Greater,
    );
    test(
        "4",
        3,
        100,
        Floor,
        "0.666666666666666666666666666666",
        "0x0.aaaaaaaaaaaaaaaaaaaaaaaaa#100",
        Less,
    );
    test(
        "4",
        3,
        100,
        Ceiling,
        "0.666666666666666666666666666667",
        "0x0.aaaaaaaaaaaaaaaaaaaaaaaab#100",
        Greater,
    );

    // 2^100000 + 1 is not a power of 2: Floor and Ceiling must straddle the integer.
    let test_big = |u: Rational, pow: i64, prec, rm, out: &str, out_hex: &str, out_o| {
        let (log, o) = Float::log_base_power_of_2_rational_prec_round(u.clone(), pow, prec, rm);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(to_hex_string(&log), out_hex);
        assert_eq!(o, out_o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_log, rug_o) =
                rug_log_base_power_of_2_rational_prec_round(&u, pow, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_log)),
                ComparableFloatRef(&log),
            );
            assert_eq!(rug_o, o);
        }
    };
    test_big(
        Rational::power_of_2(1000i64) + Rational::ONE,
        2,
        100,
        Floor,
        "500.0",
        "0x1f4.00000000000000000000000#100",
        Less,
    );
    test_big(
        Rational::power_of_2(1000i64) + Rational::ONE,
        2,
        100,
        Ceiling,
        "500.0000000000000000000000000004",
        "0x1f4.00000000000000000000002#100",
        Greater,
    );
}

#[test]
fn log_base_power_of_2_rational_prec_fail() {
    assert_panic!(Float::log_base_power_of_2_rational_prec(
        Rational::ZERO,
        2,
        0
    ));
    assert_panic!(Float::log_base_power_of_2_rational_prec(
        Rational::ONE,
        2,
        0
    ));
    assert_panic!(Float::log_base_power_of_2_rational_prec(
        Rational::ONE,
        0,
        1
    ));
    assert_panic!(Float::log_base_power_of_2_rational_prec_ref(
        &Rational::ZERO,
        2,
        0
    ));
    assert_panic!(Float::log_base_power_of_2_rational_prec_ref(
        &Rational::ONE,
        0,
        1
    ));
}

#[test]
fn log_base_power_of_2_rational_prec_round_fail() {
    assert_panic!(Float::log_base_power_of_2_rational_prec_round(
        Rational::ZERO,
        2,
        0,
        Floor
    ));
    assert_panic!(Float::log_base_power_of_2_rational_prec_round(
        Rational::ONE,
        0,
        1,
        Floor
    ));
    // - 3 is not a power of 2, so its base-2 logarithm is irrational
    assert_panic!(Float::log_base_power_of_2_rational_prec_round(
        Rational::from(3u32),
        2,
        1,
        Exact
    ));
    // - log_8(4) = 2/3, which is not representable
    assert_panic!(Float::log_base_power_of_2_rational_prec_round(
        Rational::from(4u32),
        3,
        100,
        Exact
    ));
    assert_panic!(Float::log_base_power_of_2_rational_prec_round_ref(
        &Rational::ZERO,
        2,
        0,
        Floor
    ));
    assert_panic!(Float::log_base_power_of_2_rational_prec_round_ref(
        &Rational::ONE,
        0,
        1,
        Floor
    ));
    assert_panic!(Float::log_base_power_of_2_rational_prec_round_ref(
        &Rational::from(3u32),
        2,
        1,
        Exact
    ));
    assert_panic!(Float::log_base_power_of_2_rational_prec_round_ref(
        &Rational::from(4u32),
        3,
        100,
        Exact
    ));
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_power_of_2_rational_prec_round_properties_helper(
    x: Rational,
    pow: i64,
    prec: u64,
    rm: RoundingMode,
) {
    let (log, o) = Float::log_base_power_of_2_rational_prec_round(x.clone(), pow, prec, rm);
    assert!(log.is_valid());

    let (log_alt, o_alt) = Float::log_base_power_of_2_rational_prec_round_ref(&x, pow, prec, rm);
    assert!(log_alt.is_valid());
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_log, rug_o) = rug_log_base_power_of_2_rational_prec_round(&x, pow, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log).abs_negative_zero()),
            ComparableFloatRef(&log.abs_negative_zero_ref()),
        );
        assert_eq!(rug_o, o);
    }

    // log_{2^1}(x) == log_2(x), and pow * log_{2^pow}(x) == log_2(x), so for pow == 1 the result
    // matches the base-2 logarithm exactly.
    if pow == 1 {
        let (log_2_alt, o_alt) = Float::log_base_2_rational_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&log_2_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);
    }

    // log_{2^pow}(1/x) = -log_{2^pow}(x): negating the rounding mode and the result, and reversing
    // the ordering, gives the same Float.
    if x != 0u32 {
        let (log_alt, o_alt) =
            Float::log_base_power_of_2_rational_prec_round((&x).reciprocal(), pow, prec, -rm);
        assert!(log_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&(-log_alt).abs_negative_zero()),
            ComparableFloatRef(&log.abs_negative_zero_ref())
        );
        assert_eq!(o_alt.reverse(), o);
    }

    if x < 0u32 {
        assert!(log.is_nan());
    }

    if log.is_normal() {
        assert_eq!(log.get_prec(), Some(prec));

        // Cross-check: pow * log_{2^pow}(x) == log_2(x). The exact value of the result times pow
        // must lie between the base-2 logarithm's Floor and Ceiling brackets (computed at the same
        // precision), since both are correctly-rounded approximations of the same real number.
        let (l2_lo, _) = Float::log_base_2_rational_prec_round_ref(&x, prec, Floor);
        let (l2_hi, _) = Float::log_base_2_rational_prec_round_ref(&x, prec, Ceiling);
        if l2_lo.is_normal() && l2_hi.is_normal() {
            let prod = Rational::exact_from(&log) * Rational::from(pow);
            let r_lo = Rational::exact_from(&l2_lo);
            let r_hi = Rational::exact_from(&l2_hi);
            let (lo, hi) = if r_lo <= r_hi {
                (r_lo, r_hi)
            } else {
                (r_hi, r_lo)
            };
            // Allow a generous slack on each side, scaled by the magnitude of the brackets and by
            // |pow|, to account for the independent roundings of the two logarithms (the result is
            // rounded at its own scale, which is |pow| times smaller than log_2(x)).
            let scale = hi.floor_log_base_2_abs().max(lo.floor_log_base_2_abs());
            let slack = Rational::from(pow.unsigned_abs())
                * Rational::power_of_2(scale - i64::exact_from(prec) + 2);
            assert!(prod >= &lo - &slack);
            assert!(prod <= &hi + &slack);
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = Float::log_base_power_of_2_rational_prec_round_ref(&x, pow, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(log.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::log_base_power_of_2_rational_prec_round_ref(
            &x, pow, prec, Exact
        ));
    }
}

#[test]
fn log_base_power_of_2_rational_prec_round_properties() {
    rational_signed_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, pow, prec, rm)| {
            log_base_power_of_2_rational_prec_round_properties_helper(x, pow, prec, rm);
        },
    );

    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    rational_signed_unsigned_rounding_mode_quadruple_gen_var_1().test_properties_with_config(
        &config,
        |(x, pow, prec, rm)| {
            log_base_power_of_2_rational_prec_round_properties_helper(x, pow, prec, rm);
        },
    );

    signed_rounding_mode_pair_gen_var_3::<i64>().test_properties(|(pow, rm)| {
        let prec = 10;
        assert_eq!(
            Float::log_base_power_of_2_rational_prec_round(Rational::ZERO, pow, prec, rm),
            (
                if pow > 0 {
                    Float::NEGATIVE_INFINITY
                } else {
                    Float::INFINITY
                },
                Equal
            )
        );

        // log_{2^pow}(1) = 0, but the zero may be signed (e.g. for negative pow).
        let (log, o) = Float::log_base_power_of_2_rational_prec_round(Rational::ONE, pow, prec, rm);
        assert_eq!(
            ComparableFloat(log.abs_negative_zero()),
            ComparableFloat(Float::ZERO)
        );
        assert_eq!(o, Equal);

        let (log, o) =
            Float::log_base_power_of_2_rational_prec_round(Rational::NEGATIVE_ONE, pow, prec, rm);
        assert_eq!(ComparableFloat(log), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_power_of_2_rational_prec_properties_helper(x: Rational, pow: i64, prec: u64) {
    let (log, o) = Float::log_base_power_of_2_rational_prec(x.clone(), pow, prec);
    assert!(log.is_valid());

    let (log_alt, o_alt) = Float::log_base_power_of_2_rational_prec_ref(&x, pow, prec);
    assert!(log_alt.is_valid());
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    let (log_alt, o_alt) =
        Float::log_base_power_of_2_rational_prec_round_ref(&x, pow, prec, Nearest);
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    let (rug_log, rug_o) = rug_log_base_power_of_2_rational_prec(&x, pow, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_log).abs_negative_zero()),
        ComparableFloatRef(&log.abs_negative_zero_ref()),
    );
    assert_eq!(rug_o, o);

    if pow == 1 {
        let (log_2_alt, o_alt) = Float::log_base_2_rational_prec_ref(&x, prec);
        assert_eq!(ComparableFloatRef(&log_2_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);
    }

    if x != 0u32 {
        let (log_alt, o_alt) =
            Float::log_base_power_of_2_rational_prec((&x).reciprocal(), pow, prec);
        assert!(log_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&(-log_alt).abs_negative_zero()),
            ComparableFloatRef(&log.abs_negative_zero_ref())
        );
        assert_eq!(o_alt.reverse(), o);
    }

    if x < 0u32 {
        assert!(log.is_nan());
    }

    if log.is_normal() {
        assert_eq!(log.get_prec(), Some(prec));
    }
}

#[test]
fn log_base_power_of_2_rational_prec_properties() {
    rational_signed_unsigned_triple_gen_var_2::<i64, u64>().test_properties(|(x, pow, prec)| {
        log_base_power_of_2_rational_prec_properties_helper(x, pow, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    rational_signed_unsigned_triple_gen_var_2::<i64, u64>().test_properties_with_config(
        &config,
        |(x, pow, prec)| {
            log_base_power_of_2_rational_prec_properties_helper(x, pow, prec);
        },
    );

    signed_rounding_mode_pair_gen_var_3::<i64>().test_properties(|(pow, _rm)| {
        let prec = 10;
        assert_eq!(
            Float::log_base_power_of_2_rational_prec(Rational::ZERO, pow, prec),
            (
                if pow > 0 {
                    Float::NEGATIVE_INFINITY
                } else {
                    Float::INFINITY
                },
                Equal
            )
        );

        // log_{2^pow}(1) = 0, but the zero may be signed (e.g. for negative pow).
        let (log, o) = Float::log_base_power_of_2_rational_prec(Rational::ONE, pow, prec);
        assert_eq!(
            ComparableFloat(log.abs_negative_zero()),
            ComparableFloat(Float::ZERO)
        );
        assert_eq!(o, Equal);

        let (log, o) = Float::log_base_power_of_2_rational_prec(Rational::NEGATIVE_ONE, pow, prec);
        assert_eq!(ComparableFloat(log), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);
    });
}
