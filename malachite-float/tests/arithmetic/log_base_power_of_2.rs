// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{LogBase2, LogBasePowerOf2, LogBasePowerOf2Assign};
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
    rug_log_base_power_of_2_round,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_signed_rounding_mode_triple_gen_var_7, float_signed_rounding_mode_triple_gen_var_8,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_5,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_6,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::platform::Limb;
use std::panic::catch_unwind;

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
