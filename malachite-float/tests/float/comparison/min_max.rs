// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Two, Zero,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::comparison::min_max::{
    rug_max, rug_max_prec, rug_max_prec_round, rug_max_round, rug_min, rug_min_prec,
    rug_min_prec_round, rug_min_round,
};
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_39,
    float_float_unsigned_rounding_mode_quadruple_gen_var_16,
    float_float_unsigned_rounding_mode_quadruple_gen_var_17, float_float_unsigned_triple_gen_var_1,
    float_gen, float_pair_gen, float_pair_gen_var_10, float_rational_pair_gen,
    float_rational_rounding_mode_triple_gen_var_20, float_rational_rounding_mode_triple_gen_var_21,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_21,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_22,
    float_rational_unsigned_triple_gen_var_1, float_unsigned_rounding_mode_triple_gen_var_1,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::cmp::max;
use std::panic::catch_unwind;

#[test]
fn test_min() {
    let test = |s, s_hex, t, t_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (min, o) = x.clone().min(y.clone());
        assert!(min.is_valid());
        assert_eq!(min.to_string(), out);
        assert_eq!(to_hex_string(&min), out_hex);
        assert_eq!(o, Equal);

        let (min_alt, o_alt) = x.clone().min_val_ref(&y);
        assert!(min_alt.is_valid());
        assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
        assert_eq!(o_alt, o);
        let (min_alt, o_alt) = x.min_ref_val(y.clone());
        assert!(min_alt.is_valid());
        assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
        assert_eq!(o_alt, o);
        let (min_alt, o_alt) = x.min_ref_ref(&y);
        assert!(min_alt.is_valid());
        assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
        assert_eq!(o_alt, o);

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_min(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&y)
            ))),
            ComparableFloatRef(&min),
        );
    };
    // The 5x5 special-value matrix (NaN, +/-Infinity, +/-0.0 in both positions) covers all seven
    // branches of the operand-choice function for min.
    test("NaN", "NaN", "NaN", "NaN", "NaN", "NaN");
    test("NaN", "NaN", "Infinity", "Infinity", "Infinity", "Infinity");
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
    );
    test("NaN", "NaN", "0.0", "0x0.0", "0.0", "0x0.0");
    test("NaN", "NaN", "-0.0", "-0x0.0", "-0.0", "-0x0.0");
    test("Infinity", "Infinity", "NaN", "NaN", "Infinity", "Infinity");
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", "Infinity", "Infinity",
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
    );
    test("Infinity", "Infinity", "0.0", "0x0.0", "0.0", "0x0.0");
    test("Infinity", "Infinity", "-0.0", "-0x0.0", "-0.0", "-0x0.0");
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
    );
    test("0.0", "0x0.0", "NaN", "NaN", "0.0", "0x0.0");
    test("0.0", "0x0.0", "Infinity", "Infinity", "0.0", "0x0.0");
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
    );
    test("0.0", "0x0.0", "0.0", "0x0.0", "0.0", "0x0.0");
    test("0.0", "0x0.0", "-0.0", "-0x0.0", "-0.0", "-0x0.0");
    test("-0.0", "-0x0.0", "NaN", "NaN", "-0.0", "-0x0.0");
    test("-0.0", "-0x0.0", "Infinity", "Infinity", "-0.0", "-0x0.0");
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
    );
    test("-0.0", "-0x0.0", "0.0", "0x0.0", "-0.0", "-0x0.0");
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0", "-0.0", "-0x0.0");
    test("1.0", "0x1.0#1", "2.0", "0x2.0#2", "1.0", "0x1.0#2");
    test("2.0", "0x2.0#1", "1.0", "0x1.0#2", "1.0", "0x1.0#2");
    test("-1.0", "-0x1.0#1", "-2.0", "-0x2.0#2", "-2.0", "-0x2.0#2");
    test("1.0", "0x1.0#1", "1.00", "0x1.0#5", "1.00", "0x1.0#5");
    test("1.5", "0x1.8#2", "1.2", "0x1.4#3", "1.2", "0x1.4#3");
    test("-1.5", "-0x1.8#2", "1.2", "0x1.4#3", "-1.5", "-0x1.8#3");
    test("1.0", "0x1.0#1", "NaN", "NaN", "1.0", "0x1.0#1");
    test("NaN", "NaN", "1.0", "0x1.0#1", "1.0", "0x1.0#1");
    test(
        "1.0",
        "0x1.0#1",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
    );
    test("1.0", "0x1.0#1", "Infinity", "Infinity", "1.0", "0x1.0#1");
    test("-1.0", "-0x1.0#1", "0.0", "0x0.0", "-1.0", "-0x1.0#1");
    test("1.0", "0x1.0#1", "-0.0", "-0x0.0", "-0.0", "-0x0.0");
}

#[test]
fn test_min_prec() {
    let test = |s, s_hex, t, t_hex, prec, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (min, o) = x.clone().min_prec(y.clone(), prec);
        assert!(min.is_valid());
        assert_eq!(min.to_string(), out);
        assert_eq!(to_hex_string(&min), out_hex);
        assert_eq!(o, o_out);

        let (min_alt, o_alt) = x.clone().min_prec_val_ref(&y, prec);
        assert!(min_alt.is_valid());
        assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
        assert_eq!(o_alt, o);
        let (min_alt, o_alt) = x.min_prec_ref_val(y.clone(), prec);
        assert!(min_alt.is_valid());
        assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
        assert_eq!(o_alt, o);
        let (min_alt, o_alt) = x.min_prec_ref_ref(&y, prec);
        assert!(min_alt.is_valid());
        assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
        assert_eq!(o_alt, o);

        let (rug_min, rug_o) = rug_min_prec(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_min)),
            ComparableFloatRef(&min),
        );
        assert_eq!(rug_o, o);
    };
    test("NaN", "NaN", "NaN", "NaN", 10, "NaN", "NaN", Equal);
    test(
        "NaN",
        "NaN",
        "1.0",
        "0x1.0#1",
        10,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.5", "0x1.8#2", "5.0", "0x5.0#3", 1, "2.0", "0x2.0#1", Greater,
    );
    test(
        "2.5", "0x2.8#3", "5.0", "0x5.0#3", 1, "2.0", "0x2.0#1", Less,
    );
    test(
        "-1.5", "-0x1.8#2", "5.0", "0x5.0#3", 1, "-2.0", "-0x2.0#1", Less,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 10, "-0.0", "-0x0.0", Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        100,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    // - !x.is_nan() && y.is_nan()
    test(
        "1.5",
        "0x1.8#2",
        "NaN",
        "NaN",
        10,
        "1.5000",
        "0x1.800#10",
        Equal,
    );
    // - x.is_zero() && y.is_zero(), Choice::First
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 10, "-0.0", "-0x0.0", Equal,
    );
    // - comparison decides, Choice::Second
    test(
        "5.0",
        "0x5.0#3",
        "1.5",
        "0x1.8#2",
        10,
        "1.5000",
        "0x1.800#10",
        Equal,
    );
}

#[test]
fn min_prec_fail() {
    assert_panic!(Float::ONE.min_prec(Float::TWO, 0));
    // - the precision is validated even when both operands are NaN, matching Float::add_prec
    assert_panic!(Float::NAN.min_prec(Float::NAN, 0));
    assert_panic!(Float::ONE.min_prec_val_ref(&Float::TWO, 0));
    assert_panic!(Float::ONE.min_prec_ref_val(Float::TWO, 0));
    assert_panic!(Float::ONE.min_prec_ref_ref(&Float::TWO, 0));
}

#[test]
fn test_min_round() {
    let test = |s, s_hex, t, t_hex, rm: RoundingMode, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (min, o) = x.clone().min_round(y.clone(), rm);
        assert!(min.is_valid());
        assert_eq!(min.to_string(), out);
        assert_eq!(to_hex_string(&min), out_hex);
        assert_eq!(o, Equal);

        let (min_alt, o_alt) = x.clone().min_round_val_ref(&y, rm);
        assert!(min_alt.is_valid());
        assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
        assert_eq!(o_alt, o);
        let (min_alt, o_alt) = x.min_round_ref_val(y.clone(), rm);
        assert!(min_alt.is_valid());
        assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
        assert_eq!(o_alt, o);
        let (min_alt, o_alt) = x.min_round_ref_ref(&y, rm);
        assert!(min_alt.is_valid());
        assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_min, rug_o) = rug_min_round(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&y),
                rug_rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_min)),
                ComparableFloatRef(&min),
            );
            assert_eq!(rug_o, o);
        }
    };
    test("1.5", "0x1.8#2", "1.2", "0x1.4#3", Floor, "1.2", "0x1.4#3");
    test(
        "1.5", "0x1.8#2", "1.2", "0x1.4#3", Ceiling, "1.2", "0x1.4#3",
    );
    test("1.5", "0x1.8#2", "1.2", "0x1.4#3", Down, "1.2", "0x1.4#3");
    test("1.5", "0x1.8#2", "1.2", "0x1.4#3", Up, "1.2", "0x1.4#3");
    test(
        "1.5", "0x1.8#2", "1.2", "0x1.4#3", Nearest, "1.2", "0x1.4#3",
    );
    test("1.5", "0x1.8#2", "1.2", "0x1.4#3", Exact, "1.2", "0x1.4#3");
    test("1.0", "0x1.0#1", "1.5", "0x1.8#2", Exact, "1.0", "0x1.0#2");
    test("NaN", "NaN", "1.0", "0x1.0#1", Floor, "1.0", "0x1.0#1");
    test("0.0", "0x0.0", "-0.0", "-0x0.0", Down, "-0.0", "-0x0.0");
    // - x.is_nan() && y.is_nan()
    test("NaN", "NaN", "NaN", "NaN", Floor, "NaN", "NaN");
    // - !x.is_nan() && y.is_nan()
    test("1.5", "0x1.8#2", "NaN", "NaN", Ceiling, "1.5", "0x1.8#2");
    // - x.is_zero() && y.is_zero(), Choice::First
    test("-0.0", "-0x0.0", "0.0", "0x0.0", Up, "-0.0", "-0x0.0");
}

#[test]
fn test_min_prec_round() {
    let test =
        |s, s_hex, t, t_hex, prec, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
            let x = parse_hex_string(s_hex);
            assert_eq!(x.to_string(), s);
            let y = parse_hex_string(t_hex);
            assert_eq!(y.to_string(), t);

            let (min, o) = x.clone().min_prec_round(y.clone(), prec, rm);
            assert!(min.is_valid());
            assert_eq!(min.to_string(), out);
            assert_eq!(to_hex_string(&min), out_hex);
            assert_eq!(o, o_out);

            let (min_alt, o_alt) = x.clone().min_prec_round_val_ref(&y, prec, rm);
            assert!(min_alt.is_valid());
            assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
            assert_eq!(o_alt, o);
            let (min_alt, o_alt) = x.min_prec_round_ref_val(y.clone(), prec, rm);
            assert!(min_alt.is_valid());
            assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
            assert_eq!(o_alt, o);
            let (min_alt, o_alt) = x.min_prec_round_ref_ref(&y, prec, rm);
            assert!(min_alt.is_valid());
            assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
            assert_eq!(o_alt, o);

            if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
                let (rug_min, rug_o) = rug_min_prec_round(
                    &rug::Float::exact_from(&x),
                    &rug::Float::exact_from(&y),
                    prec,
                    rug_rm,
                );
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_min)),
                    ComparableFloatRef(&min),
                );
                assert_eq!(rug_o, o);
            }
        };
    test(
        "1.5", "0x1.8#2", "5.0", "0x5.0#3", 1, Floor, "1.0", "0x1.0#1", Less,
    );
    test(
        "1.5", "0x1.8#2", "5.0", "0x5.0#3", 1, Down, "1.0", "0x1.0#1", Less,
    );
    test(
        "1.5", "0x1.8#2", "5.0", "0x5.0#3", 1, Ceiling, "2.0", "0x2.0#1", Greater,
    );
    test(
        "1.5", "0x1.8#2", "5.0", "0x5.0#3", 1, Up, "2.0", "0x2.0#1", Greater,
    );
    test(
        "1.5", "0x1.8#2", "5.0", "0x5.0#3", 1, Nearest, "2.0", "0x2.0#1", Greater,
    );
    test(
        "1.5", "0x1.8#2", "5.0", "0x5.0#3", 2, Exact, "1.5", "0x1.8#2", Equal,
    );
    test("NaN", "NaN", "NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test(
        "NaN", "NaN", "1.5", "0x1.8#2", 1, Ceiling, "2.0", "0x2.0#1", Greater,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 10, Up, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-1.5", "-0x1.8#2", "5.0", "0x5.0#3", 1, Floor, "-2.0", "-0x2.0#1", Less,
    );
    test(
        "-1.5", "-0x1.8#2", "5.0", "0x5.0#3", 1, Ceiling, "-1.0", "-0x1.0#1", Greater,
    );
    // - !x.is_nan() && y.is_nan()
    test(
        "1.5", "0x1.8#2", "NaN", "NaN", 1, Floor, "1.0", "0x1.0#1", Less,
    );
    // - x.is_zero() && y.is_zero(), Choice::First
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 10, Up, "-0.0", "-0x0.0", Equal,
    );
    // - comparison decides, Choice::Second
    test(
        "5.0", "0x5.0#3", "1.5", "0x1.8#2", 1, Floor, "1.0", "0x1.0#1", Less,
    );
    // - comparison decides, Choice::Second
    test(
        "5.0", "0x5.0#3", "1.5", "0x1.8#2", 1, Ceiling, "2.0", "0x2.0#1", Greater,
    );
}

#[test]
fn min_prec_round_fail() {
    assert_panic!(Float::ONE.min_prec_round(Float::TWO, 0, Floor));
    // - the precision is validated even when both operands are NaN, matching Float::add_prec
    assert_panic!(Float::NAN.min_prec_round(Float::NAN, 0, Floor));
    assert_panic!(Float::ONE.min_prec_round_val_ref(&Float::TWO, 0, Floor));
    assert_panic!(Float::ONE.min_prec_round_ref_val(Float::TWO, 0, Floor));
    assert_panic!(Float::ONE.min_prec_round_ref_ref(&Float::TWO, 0, Floor));
    assert_panic!(parse_hex_string("0x1.8#2").min_prec_round(
        parse_hex_string("0x5.0#3"),
        1,
        Exact
    ));
}

#[test]
fn test_max() {
    let test = |s, s_hex, t, t_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (max, o) = x.clone().max(y.clone());
        assert!(max.is_valid());
        assert_eq!(max.to_string(), out);
        assert_eq!(to_hex_string(&max), out_hex);
        assert_eq!(o, Equal);

        let (max_alt, o_alt) = x.clone().max_val_ref(&y);
        assert!(max_alt.is_valid());
        assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
        assert_eq!(o_alt, o);
        let (max_alt, o_alt) = x.max_ref_val(y.clone());
        assert!(max_alt.is_valid());
        assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
        assert_eq!(o_alt, o);
        let (max_alt, o_alt) = x.max_ref_ref(&y);
        assert!(max_alt.is_valid());
        assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
        assert_eq!(o_alt, o);

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_max(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&y)
            ))),
            ComparableFloatRef(&max),
        );
    };
    // The 5x5 special-value matrix (NaN, +/-Infinity, +/-0.0 in both positions) covers all seven
    // branches of the operand-choice function for max.
    test("NaN", "NaN", "NaN", "NaN", "NaN", "NaN");
    test("NaN", "NaN", "Infinity", "Infinity", "Infinity", "Infinity");
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
    );
    test("NaN", "NaN", "0.0", "0x0.0", "0.0", "0x0.0");
    test("NaN", "NaN", "-0.0", "-0x0.0", "-0.0", "-0x0.0");
    test("Infinity", "Infinity", "NaN", "NaN", "Infinity", "Infinity");
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", "Infinity", "Infinity",
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", "Infinity", "Infinity",
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", "Infinity", "Infinity",
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
    );
    test("-Infinity", "-Infinity", "0.0", "0x0.0", "0.0", "0x0.0");
    test("-Infinity", "-Infinity", "-0.0", "-0x0.0", "-0.0", "-0x0.0");
    test("0.0", "0x0.0", "NaN", "NaN", "0.0", "0x0.0");
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", "Infinity", "Infinity",
    );
    test("0.0", "0x0.0", "-Infinity", "-Infinity", "0.0", "0x0.0");
    test("0.0", "0x0.0", "0.0", "0x0.0", "0.0", "0x0.0");
    test("0.0", "0x0.0", "-0.0", "-0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "NaN", "NaN", "-0.0", "-0x0.0");
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", "Infinity", "Infinity",
    );
    test("-0.0", "-0x0.0", "-Infinity", "-Infinity", "-0.0", "-0x0.0");
    test("-0.0", "-0x0.0", "0.0", "0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0", "-0.0", "-0x0.0");
    test("1.0", "0x1.0#1", "2.0", "0x2.0#2", "2.0", "0x2.0#2");
    test("2.0", "0x2.0#1", "1.0", "0x1.0#2", "2.0", "0x2.0#2");
    test("-1.0", "-0x1.0#1", "-2.0", "-0x2.0#2", "-1.0", "-0x1.0#2");
    test("1.0", "0x1.0#1", "1.00", "0x1.0#5", "1.00", "0x1.0#5");
    test("1.5", "0x1.8#2", "1.2", "0x1.4#3", "1.5", "0x1.8#3");
    test("-1.5", "-0x1.8#2", "1.2", "0x1.4#3", "1.2", "0x1.4#3");
    test("1.0", "0x1.0#1", "NaN", "NaN", "1.0", "0x1.0#1");
    test("NaN", "NaN", "1.0", "0x1.0#1", "1.0", "0x1.0#1");
    test("1.0", "0x1.0#1", "-Infinity", "-Infinity", "1.0", "0x1.0#1");
    test(
        "1.0", "0x1.0#1", "Infinity", "Infinity", "Infinity", "Infinity",
    );
    test("-1.0", "-0x1.0#1", "0.0", "0x0.0", "0.0", "0x0.0");
    test("1.0", "0x1.0#1", "-0.0", "-0x0.0", "1.0", "0x1.0#1");
}

#[test]
fn test_max_prec() {
    let test = |s, s_hex, t, t_hex, prec, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (max, o) = x.clone().max_prec(y.clone(), prec);
        assert!(max.is_valid());
        assert_eq!(max.to_string(), out);
        assert_eq!(to_hex_string(&max), out_hex);
        assert_eq!(o, o_out);

        let (max_alt, o_alt) = x.clone().max_prec_val_ref(&y, prec);
        assert!(max_alt.is_valid());
        assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
        assert_eq!(o_alt, o);
        let (max_alt, o_alt) = x.max_prec_ref_val(y.clone(), prec);
        assert!(max_alt.is_valid());
        assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
        assert_eq!(o_alt, o);
        let (max_alt, o_alt) = x.max_prec_ref_ref(&y, prec);
        assert!(max_alt.is_valid());
        assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
        assert_eq!(o_alt, o);

        let (rug_max, rug_o) = rug_max_prec(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_max)),
            ComparableFloatRef(&max),
        );
        assert_eq!(rug_o, o);
    };
    test("NaN", "NaN", "NaN", "NaN", 10, "NaN", "NaN", Equal);
    test(
        "NaN",
        "NaN",
        "1.0",
        "0x1.0#1",
        10,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        "2.0000",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.5", "0x1.8#2", "-5.0", "-0x5.0#3", 1, "2.0", "0x2.0#1", Greater,
    );
    test(
        "2.5", "0x2.8#3", "-5.0", "-0x5.0#3", 1, "2.0", "0x2.0#1", Less,
    );
    test(
        "-1.5", "-0x1.8#2", "-5.0", "-0x5.0#3", 1, "-2.0", "-0x2.0#1", Less,
    );
    test("0.0", "0x0.0", "-0.0", "-0x0.0", 10, "0.0", "0x0.0", Equal);
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        100,
        "2.0000000000000000000000000000000",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    // - !x.is_nan() && y.is_nan()
    test(
        "1.5",
        "0x1.8#2",
        "NaN",
        "NaN",
        10,
        "1.5000",
        "0x1.800#10",
        Equal,
    );
    // - x.is_zero() && y.is_zero(), Choice::Second
    test("-0.0", "-0x0.0", "0.0", "0x0.0", 10, "0.0", "0x0.0", Equal);
}

#[test]
fn max_prec_fail() {
    assert_panic!(Float::ONE.max_prec(Float::TWO, 0));
    // - the precision is validated even when both operands are NaN, matching Float::add_prec
    assert_panic!(Float::NAN.max_prec(Float::NAN, 0));
    assert_panic!(Float::ONE.max_prec_val_ref(&Float::TWO, 0));
    assert_panic!(Float::ONE.max_prec_ref_val(Float::TWO, 0));
    assert_panic!(Float::ONE.max_prec_ref_ref(&Float::TWO, 0));
}

#[test]
fn test_max_round() {
    let test = |s, s_hex, t, t_hex, rm: RoundingMode, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (max, o) = x.clone().max_round(y.clone(), rm);
        assert!(max.is_valid());
        assert_eq!(max.to_string(), out);
        assert_eq!(to_hex_string(&max), out_hex);
        assert_eq!(o, Equal);

        let (max_alt, o_alt) = x.clone().max_round_val_ref(&y, rm);
        assert!(max_alt.is_valid());
        assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
        assert_eq!(o_alt, o);
        let (max_alt, o_alt) = x.max_round_ref_val(y.clone(), rm);
        assert!(max_alt.is_valid());
        assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
        assert_eq!(o_alt, o);
        let (max_alt, o_alt) = x.max_round_ref_ref(&y, rm);
        assert!(max_alt.is_valid());
        assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_max, rug_o) = rug_max_round(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&y),
                rug_rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_max)),
                ComparableFloatRef(&max),
            );
            assert_eq!(rug_o, o);
        }
    };
    test("1.5", "0x1.8#2", "1.2", "0x1.4#3", Floor, "1.5", "0x1.8#3");
    test(
        "1.5", "0x1.8#2", "1.2", "0x1.4#3", Ceiling, "1.5", "0x1.8#3",
    );
    test("1.5", "0x1.8#2", "1.2", "0x1.4#3", Down, "1.5", "0x1.8#3");
    test("1.5", "0x1.8#2", "1.2", "0x1.4#3", Up, "1.5", "0x1.8#3");
    test(
        "1.5", "0x1.8#2", "1.2", "0x1.4#3", Nearest, "1.5", "0x1.8#3",
    );
    test("1.5", "0x1.8#2", "1.2", "0x1.4#3", Exact, "1.5", "0x1.8#3");
    test("1.0", "0x1.0#1", "1.5", "0x1.8#2", Exact, "1.5", "0x1.8#2");
    test("NaN", "NaN", "1.0", "0x1.0#1", Floor, "1.0", "0x1.0#1");
    test("0.0", "0x0.0", "-0.0", "-0x0.0", Down, "0.0", "0x0.0");
    // - x.is_nan() && y.is_nan()
    test("NaN", "NaN", "NaN", "NaN", Floor, "NaN", "NaN");
    // - !x.is_nan() && y.is_nan()
    test("1.5", "0x1.8#2", "NaN", "NaN", Ceiling, "1.5", "0x1.8#2");
    // - x.is_zero() && y.is_zero(), Choice::Second
    test("-0.0", "-0x0.0", "0.0", "0x0.0", Up, "0.0", "0x0.0");
}

#[test]
fn test_max_prec_round() {
    let test =
        |s, s_hex, t, t_hex, prec, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
            let x = parse_hex_string(s_hex);
            assert_eq!(x.to_string(), s);
            let y = parse_hex_string(t_hex);
            assert_eq!(y.to_string(), t);

            let (max, o) = x.clone().max_prec_round(y.clone(), prec, rm);
            assert!(max.is_valid());
            assert_eq!(max.to_string(), out);
            assert_eq!(to_hex_string(&max), out_hex);
            assert_eq!(o, o_out);

            let (max_alt, o_alt) = x.clone().max_prec_round_val_ref(&y, prec, rm);
            assert!(max_alt.is_valid());
            assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
            assert_eq!(o_alt, o);
            let (max_alt, o_alt) = x.max_prec_round_ref_val(y.clone(), prec, rm);
            assert!(max_alt.is_valid());
            assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
            assert_eq!(o_alt, o);
            let (max_alt, o_alt) = x.max_prec_round_ref_ref(&y, prec, rm);
            assert!(max_alt.is_valid());
            assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
            assert_eq!(o_alt, o);

            if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
                let (rug_max, rug_o) = rug_max_prec_round(
                    &rug::Float::exact_from(&x),
                    &rug::Float::exact_from(&y),
                    prec,
                    rug_rm,
                );
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_max)),
                    ComparableFloatRef(&max),
                );
                assert_eq!(rug_o, o);
            }
        };
    test(
        "1.5", "0x1.8#2", "-5.0", "-0x5.0#3", 1, Floor, "1.0", "0x1.0#1", Less,
    );
    test(
        "1.5", "0x1.8#2", "-5.0", "-0x5.0#3", 1, Down, "1.0", "0x1.0#1", Less,
    );
    test(
        "1.5", "0x1.8#2", "-5.0", "-0x5.0#3", 1, Ceiling, "2.0", "0x2.0#1", Greater,
    );
    test(
        "1.5", "0x1.8#2", "-5.0", "-0x5.0#3", 1, Up, "2.0", "0x2.0#1", Greater,
    );
    test(
        "1.5", "0x1.8#2", "-5.0", "-0x5.0#3", 1, Nearest, "2.0", "0x2.0#1", Greater,
    );
    test(
        "1.5", "0x1.8#2", "-5.0", "-0x5.0#3", 2, Exact, "1.5", "0x1.8#2", Equal,
    );
    test("NaN", "NaN", "NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test(
        "NaN", "NaN", "1.5", "0x1.8#2", 1, Floor, "1.0", "0x1.0#1", Less,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 10, Up, "0.0", "0x0.0", Equal,
    );
    test(
        "-1.5", "-0x1.8#2", "-5.0", "-0x5.0#3", 1, Floor, "-2.0", "-0x2.0#1", Less,
    );
    test(
        "-1.5", "-0x1.8#2", "-5.0", "-0x5.0#3", 1, Ceiling, "-1.0", "-0x1.0#1", Greater,
    );
    // - !x.is_nan() && y.is_nan()
    test(
        "1.5", "0x1.8#2", "NaN", "NaN", 1, Ceiling, "2.0", "0x2.0#1", Greater,
    );
    // - x.is_zero() && y.is_zero(), Choice::Second
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 10, Up, "0.0", "0x0.0", Equal,
    );
    // - comparison decides, Choice::Second
    test(
        "-5.0", "-0x5.0#3", "1.5", "0x1.8#2", 1, Floor, "1.0", "0x1.0#1", Less,
    );
    // - comparison decides, Choice::Second
    test(
        "-5.0", "-0x5.0#3", "1.5", "0x1.8#2", 1, Ceiling, "2.0", "0x2.0#1", Greater,
    );
}

#[test]
fn max_prec_round_fail() {
    assert_panic!(Float::ONE.max_prec_round(Float::TWO, 0, Floor));
    // - the precision is validated even when both operands are NaN, matching Float::add_prec
    assert_panic!(Float::NAN.max_prec_round(Float::NAN, 0, Floor));
    assert_panic!(Float::ONE.max_prec_round_val_ref(&Float::TWO, 0, Floor));
    assert_panic!(Float::ONE.max_prec_round_ref_val(Float::TWO, 0, Floor));
    assert_panic!(Float::ONE.max_prec_round_ref_ref(&Float::TWO, 0, Floor));
    assert_panic!(parse_hex_string("0x1.8#2").max_prec_round(
        parse_hex_string("-0x5.0#3"),
        1,
        Exact
    ));
}

#[allow(clippy::needless_pass_by_value)]
fn min_prec_round_properties_helper(x: Float, y: Float, prec: u64, rm: RoundingMode) {
    let (min, o) = x.clone().min_prec_round(y.clone(), prec, rm);
    assert!(min.is_valid());
    let (min_alt, o_alt) = x.clone().min_prec_round_val_ref(&y, prec, rm);
    assert!(min_alt.is_valid());
    assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
    assert_eq!(o_alt, o);
    let (min_alt, o_alt) = x.min_prec_round_ref_val(y.clone(), prec, rm);
    assert!(min_alt.is_valid());
    assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
    assert_eq!(o_alt, o);
    let (min_alt, o_alt) = x.min_prec_round_ref_ref(&y, prec, rm);
    assert!(min_alt.is_valid());
    assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_min, rug_o) = rug_min_prec_round(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
            rug_rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_min)),
            ComparableFloatRef(&min),
        );
        assert_eq!(rug_o, o);
    }

    if x.is_nan() && y.is_nan() {
        assert!(min.is_nan());
        assert_eq!(o, Equal);
    }
    if min.is_normal() {
        assert_eq!(min.get_prec(), Some(prec));
    }

    // The extremum itself is always exactly representable at the operands' own precisions, so this
    // reference value is exact and pins down both the selection and the rounding ternary.
    let (exact, o_exact) = x.min_round_ref_ref(&y, Floor);
    assert_eq!(o_exact, Equal);
    if !min.is_nan() {
        assert_eq!(o, min.partial_cmp(&exact).unwrap());
        assert!(exact == x || exact == y);
        if !x.is_nan() && !y.is_nan() {
            assert!(exact <= x);
            assert!(exact <= y);
        }
    }

    let (min_alt, o_alt) = y.min_prec_round_ref_ref(&x, prec, rm);
    assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
    assert_eq!(o_alt, o);

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.min_prec_round_ref_ref(&y, prec, rm);
            assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&min));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.min_prec_round_ref_ref(&y, prec, Exact));
    }
}

#[test]
fn min_prec_round_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_16().test_properties(
        |(x, y, prec, rm)| {
            min_prec_round_properties_helper(x, y, prec, rm);
        },
    );

    float_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        let (min, o) = x.min_prec_round_ref_val(Float::NAN, prec, rm);
        let (min_alt, o_alt) = Float::from_float_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&min), ComparableFloatRef(&min_alt));
        assert_eq!(o, o_alt);

        let (min, o) = Float::NAN.min_prec_round_val_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&min), ComparableFloatRef(&min_alt));
        assert_eq!(o, o_alt);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn min_prec_properties_helper(x: Float, y: Float, prec: u64) {
    let (min, o) = x.clone().min_prec(y.clone(), prec);
    assert!(min.is_valid());
    let (min_alt, o_alt) = x.clone().min_prec_val_ref(&y, prec);
    assert!(min_alt.is_valid());
    assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
    assert_eq!(o_alt, o);
    let (min_alt, o_alt) = x.min_prec_ref_val(y.clone(), prec);
    assert!(min_alt.is_valid());
    assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
    assert_eq!(o_alt, o);
    let (min_alt, o_alt) = x.min_prec_ref_ref(&y, prec);
    assert!(min_alt.is_valid());
    assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
    assert_eq!(o_alt, o);

    let (min_alt, o_alt) = x.min_prec_round_ref_ref(&y, prec, Nearest);
    assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
    assert_eq!(o_alt, o);

    let (rug_min, rug_o) = rug_min_prec(
        &rug::Float::exact_from(&x),
        &rug::Float::exact_from(&y),
        prec,
    );
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_min)),
        ComparableFloatRef(&min),
    );
    assert_eq!(rug_o, o);
}

#[test]
fn min_prec_properties() {
    float_float_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        min_prec_properties_helper(x, y, prec);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn min_round_properties_helper(x: Float, y: Float, rm: RoundingMode) {
    let prec = max(x.significant_bits(), y.significant_bits());
    let (min, o) = x.clone().min_round(y.clone(), rm);
    assert!(min.is_valid());
    // The result is one of the operands rounded to a precision at least as high as its own, so the
    // rounding is always exact.
    assert_eq!(o, Equal);
    let (min_alt, o_alt) = x.clone().min_round_val_ref(&y, rm);
    assert!(min_alt.is_valid());
    assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
    assert_eq!(o_alt, o);
    let (min_alt, o_alt) = x.min_round_ref_val(y.clone(), rm);
    assert!(min_alt.is_valid());
    assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
    assert_eq!(o_alt, o);
    let (min_alt, o_alt) = x.min_round_ref_ref(&y, rm);
    assert!(min_alt.is_valid());
    assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
    assert_eq!(o_alt, o);

    let (min_alt, o_alt) = x.min_prec_round_ref_ref(&y, prec, rm);
    assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
    assert_eq!(o_alt, o);

    for rm in exhaustive_rounding_modes() {
        let (s, oo) = x.min_round_ref_ref(&y, rm);
        assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&min));
        assert_eq!(oo, Equal);
    }

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_min, rug_o) = rug_min_round(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            rug_rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_min)),
            ComparableFloatRef(&min),
        );
        assert_eq!(rug_o, o);
    }
}

#[test]
fn min_round_properties() {
    float_float_rounding_mode_triple_gen_var_39().test_properties(|(x, y, rm)| {
        min_round_properties_helper(x, y, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn max_prec_round_properties_helper(x: Float, y: Float, prec: u64, rm: RoundingMode) {
    let (max, o) = x.clone().max_prec_round(y.clone(), prec, rm);
    assert!(max.is_valid());
    let (max_alt, o_alt) = x.clone().max_prec_round_val_ref(&y, prec, rm);
    assert!(max_alt.is_valid());
    assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
    assert_eq!(o_alt, o);
    let (max_alt, o_alt) = x.max_prec_round_ref_val(y.clone(), prec, rm);
    assert!(max_alt.is_valid());
    assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
    assert_eq!(o_alt, o);
    let (max_alt, o_alt) = x.max_prec_round_ref_ref(&y, prec, rm);
    assert!(max_alt.is_valid());
    assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_max, rug_o) = rug_max_prec_round(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
            rug_rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_max)),
            ComparableFloatRef(&max),
        );
        assert_eq!(rug_o, o);
    }

    if x.is_nan() && y.is_nan() {
        assert!(max.is_nan());
        assert_eq!(o, Equal);
    }
    if max.is_normal() {
        assert_eq!(max.get_prec(), Some(prec));
    }

    // The extremum itself is always exactly representable at the operands' own precisions, so this
    // reference value is exact and pins down both the selection and the rounding ternary.
    let (exact, o_exact) = x.max_round_ref_ref(&y, Floor);
    assert_eq!(o_exact, Equal);
    if !max.is_nan() {
        assert_eq!(o, max.partial_cmp(&exact).unwrap());
        assert!(exact == x || exact == y);
        if !x.is_nan() && !y.is_nan() {
            assert!(exact >= x);
            assert!(exact >= y);
        }
    }

    let (max_alt, o_alt) = y.max_prec_round_ref_ref(&x, prec, rm);
    assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
    assert_eq!(o_alt, o);

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.max_prec_round_ref_ref(&y, prec, rm);
            assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&max));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.max_prec_round_ref_ref(&y, prec, Exact));
    }
}

#[test]
fn max_prec_round_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_17().test_properties(
        |(x, y, prec, rm)| {
            max_prec_round_properties_helper(x, y, prec, rm);
        },
    );

    float_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        let (max, o) = x.max_prec_round_ref_val(Float::NAN, prec, rm);
        let (max_alt, o_alt) = Float::from_float_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&max), ComparableFloatRef(&max_alt));
        assert_eq!(o, o_alt);

        let (max, o) = Float::NAN.max_prec_round_val_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&max), ComparableFloatRef(&max_alt));
        assert_eq!(o, o_alt);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn max_prec_properties_helper(x: Float, y: Float, prec: u64) {
    let (max, o) = x.clone().max_prec(y.clone(), prec);
    assert!(max.is_valid());
    let (max_alt, o_alt) = x.clone().max_prec_val_ref(&y, prec);
    assert!(max_alt.is_valid());
    assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
    assert_eq!(o_alt, o);
    let (max_alt, o_alt) = x.max_prec_ref_val(y.clone(), prec);
    assert!(max_alt.is_valid());
    assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
    assert_eq!(o_alt, o);
    let (max_alt, o_alt) = x.max_prec_ref_ref(&y, prec);
    assert!(max_alt.is_valid());
    assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
    assert_eq!(o_alt, o);

    let (max_alt, o_alt) = x.max_prec_round_ref_ref(&y, prec, Nearest);
    assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
    assert_eq!(o_alt, o);

    let (rug_max, rug_o) = rug_max_prec(
        &rug::Float::exact_from(&x),
        &rug::Float::exact_from(&y),
        prec,
    );
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_max)),
        ComparableFloatRef(&max),
    );
    assert_eq!(rug_o, o);
}

#[test]
fn max_prec_properties() {
    float_float_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        max_prec_properties_helper(x, y, prec);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn max_round_properties_helper(x: Float, y: Float, rm: RoundingMode) {
    let prec = max(x.significant_bits(), y.significant_bits());
    let (max, o) = x.clone().max_round(y.clone(), rm);
    assert!(max.is_valid());
    // The result is one of the operands rounded to a precision at least as high as its own, so the
    // rounding is always exact.
    assert_eq!(o, Equal);
    let (max_alt, o_alt) = x.clone().max_round_val_ref(&y, rm);
    assert!(max_alt.is_valid());
    assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
    assert_eq!(o_alt, o);
    let (max_alt, o_alt) = x.max_round_ref_val(y.clone(), rm);
    assert!(max_alt.is_valid());
    assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
    assert_eq!(o_alt, o);
    let (max_alt, o_alt) = x.max_round_ref_ref(&y, rm);
    assert!(max_alt.is_valid());
    assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
    assert_eq!(o_alt, o);

    let (max_alt, o_alt) = x.max_prec_round_ref_ref(&y, prec, rm);
    assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
    assert_eq!(o_alt, o);

    for rm in exhaustive_rounding_modes() {
        let (s, oo) = x.max_round_ref_ref(&y, rm);
        assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&max));
        assert_eq!(oo, Equal);
    }

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_max, rug_o) = rug_max_round(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            rug_rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_max)),
            ComparableFloatRef(&max),
        );
        assert_eq!(rug_o, o);
    }
}

#[test]
fn max_round_properties() {
    float_float_rounding_mode_triple_gen_var_39().test_properties(|(x, y, rm)| {
        max_round_properties_helper(x, y, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn min_properties_helper(x: Float, y: Float) {
    let prec = max(x.significant_bits(), y.significant_bits());
    let (min, o) = x.clone().min(y.clone());
    assert!(min.is_valid());
    assert_eq!(o, Equal);
    let (min_alt, o_alt) = x.clone().min_val_ref(&y);
    assert!(min_alt.is_valid());
    assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
    assert_eq!(o_alt, o);
    let (min_alt, o_alt) = x.min_ref_val(y.clone());
    assert!(min_alt.is_valid());
    assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
    assert_eq!(o_alt, o);
    let (min_alt, o_alt) = x.min_ref_ref(&y);
    assert!(min_alt.is_valid());
    assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
    assert_eq!(o_alt, o);

    let (min_alt, o_alt) = x.min_round_ref_ref(&y, Nearest);
    assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
    assert_eq!(o_alt, o);
    let (min_alt, o_alt) = x.min_prec_ref_ref(&y, prec);
    assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
    assert_eq!(o_alt, o);

    // commutativity
    let (min_alt, o_alt) = y.min_ref_ref(&x);
    assert_eq!(ComparableFloatRef(&min_alt), ComparableFloatRef(&min));
    assert_eq!(o_alt, o);

    if !x.is_nan() && !y.is_nan() {
        assert!(min == x || min == y);
        assert!(min <= x);
        assert!(min <= y);
    }

    if !x.is_nan() && !y.is_nan() {
        let (max, _) = x.max_ref_ref(&y);
        assert!(min <= max);
        assert!(min == x && max == y || min == y && max == x);
    }

    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_min(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y)
        ))),
        ComparableFloatRef(&min),
    );
}

#[test]
fn min_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        min_properties_helper(x, y);
    });

    float_pair_gen_var_10().test_properties(|(x, y)| {
        min_properties_helper(x, y);
    });

    float_gen().test_properties(|x| {
        let (min, o) = x.min_ref_val(Float::NAN);
        assert_eq!(ComparableFloatRef(&min), ComparableFloatRef(&x));
        assert_eq!(o, Equal);
        let (min, o) = Float::NAN.min_val_ref(&x);
        assert_eq!(ComparableFloatRef(&min), ComparableFloatRef(&x));
        assert_eq!(o, Equal);

        if !x.is_nan() {
            let (min, o) = x.min_ref_val(Float::NEGATIVE_INFINITY);
            assert_eq!(min, Float::NEGATIVE_INFINITY);
            assert_eq!(o, Equal);
            let (min, o) = x.min_ref_val(Float::INFINITY);
            assert_eq!(ComparableFloatRef(&min), ComparableFloatRef(&x));
            assert_eq!(o, Equal);
        }

        // idempotence
        let (min, o) = x.min_ref_ref(&x);
        assert_eq!(ComparableFloatRef(&min), ComparableFloatRef(&x));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn max_properties_helper(x: Float, y: Float) {
    let prec = max(x.significant_bits(), y.significant_bits());
    let (max, o) = x.clone().max(y.clone());
    assert!(max.is_valid());
    assert_eq!(o, Equal);
    let (max_alt, o_alt) = x.clone().max_val_ref(&y);
    assert!(max_alt.is_valid());
    assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
    assert_eq!(o_alt, o);
    let (max_alt, o_alt) = x.max_ref_val(y.clone());
    assert!(max_alt.is_valid());
    assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
    assert_eq!(o_alt, o);
    let (max_alt, o_alt) = x.max_ref_ref(&y);
    assert!(max_alt.is_valid());
    assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
    assert_eq!(o_alt, o);

    let (max_alt, o_alt) = x.max_round_ref_ref(&y, Nearest);
    assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
    assert_eq!(o_alt, o);
    let (max_alt, o_alt) = x.max_prec_ref_ref(&y, prec);
    assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
    assert_eq!(o_alt, o);

    // commutativity
    let (max_alt, o_alt) = y.max_ref_ref(&x);
    assert_eq!(ComparableFloatRef(&max_alt), ComparableFloatRef(&max));
    assert_eq!(o_alt, o);

    if !x.is_nan() && !y.is_nan() {
        assert!(max == x || max == y);
        assert!(max >= x);
        assert!(max >= y);
    }

    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_max(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y)
        ))),
        ComparableFloatRef(&max),
    );
}

#[test]
fn max_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        max_properties_helper(x, y);
    });

    float_pair_gen_var_10().test_properties(|(x, y)| {
        max_properties_helper(x, y);
    });

    float_gen().test_properties(|x| {
        let (max, o) = x.max_ref_val(Float::NAN);
        assert_eq!(ComparableFloatRef(&max), ComparableFloatRef(&x));
        assert_eq!(o, Equal);
        let (max, o) = Float::NAN.max_val_ref(&x);
        assert_eq!(ComparableFloatRef(&max), ComparableFloatRef(&x));
        assert_eq!(o, Equal);

        if !x.is_nan() {
            let (max, o) = x.max_ref_val(Float::INFINITY);
            assert_eq!(max, Float::INFINITY);
            assert_eq!(o, Equal);
            let (max, o) = x.max_ref_val(Float::NEGATIVE_INFINITY);
            assert_eq!(ComparableFloatRef(&max), ComparableFloatRef(&x));
            assert_eq!(o, Equal);
        }

        // idempotence
        let (max, o) = x.max_ref_ref(&x);
        assert_eq!(ComparableFloatRef(&max), ComparableFloatRef(&x));
        assert_eq!(o, Equal);
    });
}

// The mixed Float-Rational min and max: the comparison is exact and only the winner is rounded.
#[test]
fn test_min_max_rational() {
    let third = Rational::from_signeds(1i32, 3i32);
    // the Float wins
    let x = Float::from(0.25f64);
    let (r, o) = x.min_rational_prec_round_ref_ref(&third, 10, Nearest);
    assert_eq!(
        ComparableFloat(r),
        ComparableFloat(Float::from_float_prec_round_ref(&x, 10, Nearest).0)
    );
    assert_eq!(o, Equal);
    let (r, o) = x.max_rational_prec_round_ref_ref(&third, 10, Nearest);
    let (expected, expected_o) = Float::from_rational_prec_round_ref(&third, 10, Nearest);
    assert_eq!(ComparableFloat(r), ComparableFloat(expected));
    assert_eq!(o, expected_o);
    // a NaN Float yields the other operand, as in the Float-Float functions
    let (r, o) = Float::NAN.min_rational_prec_round_ref_ref(&third, 10, Nearest);
    let (expected, expected_o) = Float::from_rational_prec_round_ref(&third, 10, Nearest);
    assert_eq!(ComparableFloat(r), ComparableFloat(expected));
    assert_eq!(o, expected_o);
    let (r, _) = Float::NAN.max_rational_prec_round_ref_ref(&third, 10, Nearest);
    assert!(!r.is_nan());
    // infinities compare exactly
    let (r, _) = Float::INFINITY.min_rational_prec_round_ref_ref(&third, 10, Nearest);
    assert_eq!(
        ComparableFloat(r),
        ComparableFloat(Float::from_rational_prec_round_ref(&third, 10, Nearest).0)
    );
    let (r, _) = Float::NEGATIVE_INFINITY.min_rational_prec_round_ref_ref(&third, 10, Nearest);
    assert_eq!(
        ComparableFloat(r),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    // zero ties: min preserves the negative zero, max prefers the positive zero
    let (r, o) = Float::NEGATIVE_ZERO.min_rational_prec_round_ref_ref(&Rational::ZERO, 10, Nearest);
    assert_eq!(ComparableFloat(r), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Equal);
    let (r, o) = Float::NEGATIVE_ZERO.max_rational_prec_round_ref_ref(&Rational::ZERO, 10, Nearest);
    assert_eq!(ComparableFloat(r), ComparableFloat(Float::ZERO));
    assert_eq!(o, Equal);
    let (r, _) = Float::ZERO.min_rational_prec_round_ref_ref(&Rational::ZERO, 10, Nearest);
    assert_eq!(ComparableFloat(r), ComparableFloat(Float::ZERO));
    // the boundary case that motivates the mixed function: q is just below x, so q is the true
    // minimum, even though q rounds (at the output precision) to x's value
    let x = Float::ONE;
    let q = Rational::ONE - (Rational::ONE >> 100i64);
    let (r, o) = x.min_rational_prec_round_ref_ref(&q, 1, Nearest);
    assert_eq!(r.to_string(), "1.0");
    assert_eq!(o, Greater);
    // pre-converting q would instead compare 1.0 with 1.0 and report an exact result
    let qf = Float::from_rational_prec_round_ref(&q, 1, Nearest).0;
    let (r_alt, o_alt) = x.min_prec_round_ref_ref(&qf, 1, Nearest);
    assert_eq!(r_alt.to_string(), "1.0");
    assert_eq!(o_alt, Equal);
}

#[allow(clippy::needless_pass_by_value)]
fn min_max_rational_prec_round_properties_helper(
    x: Float,
    y: Rational,
    prec: u64,
    rm: RoundingMode,
    is_max: bool,
) {
    type F = fn(&Float, &Rational, u64, RoundingMode) -> (Float, Ordering);
    let f: F = if is_max {
        Float::max_rational_prec_round_ref_ref
    } else {
        Float::min_rational_prec_round_ref_ref
    };
    let (result, o) = f(&x, &y, prec, rm);
    assert!(result.is_valid());

    if is_max {
        let (r2, o2) = x.clone().max_rational_prec_round(y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&result));
        assert_eq!(o2, o);
        let (r2, o2) = x.clone().max_rational_prec_round_val_ref(&y, prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&result));
        assert_eq!(o2, o);
        let (r2, o2) = x.max_rational_prec_round_ref_val(y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&result));
        assert_eq!(o2, o);
    } else {
        let (r2, o2) = x.clone().min_rational_prec_round(y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&result));
        assert_eq!(o2, o);
        let (r2, o2) = x.clone().min_rational_prec_round_val_ref(&y, prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&result));
        assert_eq!(o2, o);
        let (r2, o2) = x.min_rational_prec_round_ref_val(y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&result));
        assert_eq!(o2, o);
    }

    if result.is_normal() {
        assert_eq!(result.get_prec(), Some(prec));
    }

    // the result is the true winner, rounded
    match x.partial_cmp(&y) {
        None => {
            let (expected, expected_o) = Float::from_rational_prec_round_ref(&y, prec, rm);
            assert_eq!(ComparableFloatRef(&result), ComparableFloatRef(&expected));
            assert_eq!(o, expected_o);
        }
        Some(c) => {
            let float_wins = if is_max { c != Less } else { c != Greater };
            if float_wins && !(c == Equal && is_max && x == 0u32 && !x.is_sign_positive()) {
                let (expected, expected_o) = Float::from_float_prec_round_ref(&x, prec, rm);
                assert_eq!(ComparableFloatRef(&result), ComparableFloatRef(&expected));
                assert_eq!(o, expected_o);
            } else if !float_wins {
                let (expected, expected_o) = Float::from_rational_prec_round_ref(&y, prec, rm);
                assert_eq!(ComparableFloatRef(&result), ComparableFloatRef(&expected));
                assert_eq!(o, expected_o);
            } else {
                // max on a zero tie with a negative-zero Float prefers the positive zero
                assert_eq!(
                    ComparableFloat(result.clone()),
                    ComparableFloat(Float::ZERO)
                );
                assert_eq!(o, Equal);
            }
        }
    }

    // min(x, y) = -max(-x, -y)
    let (mut r2, o2) = if is_max {
        Float::min_rational_prec_round_ref_ref(&-&x, &-&y, prec, -rm)
    } else {
        Float::max_rational_prec_round_ref_ref(&-&x, &-&y, prec, -rm)
    };
    r2.neg_assign();
    assert_eq!(
        ComparableFloat(r2.abs_negative_zero()),
        ComparableFloat(result.abs_negative_zero_ref())
    );
    assert_eq!(o2.reverse(), o);

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = f(&x, &y, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(result.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(f(&x, &y, prec, Exact));
    }
}

#[test]
fn min_rational_prec_round_properties() {
    float_rational_unsigned_rounding_mode_quadruple_gen_var_21().test_properties(
        |(x, y, prec, rm)| {
            min_max_rational_prec_round_properties_helper(x, y, prec, rm, false);
        },
    );
}

#[test]
fn max_rational_prec_round_properties() {
    float_rational_unsigned_rounding_mode_quadruple_gen_var_22().test_properties(
        |(x, y, prec, rm)| {
            min_max_rational_prec_round_properties_helper(x, y, prec, rm, true);
        },
    );
}

// The shorthand levels agree with prec_round.
#[test]
fn min_max_rational_shorthand_properties() {
    float_rational_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        let (r, o) = x.min_rational_prec_round_ref_ref(&y, prec, Nearest);
        let (r2, o2) = x.min_rational_prec_ref_ref(&y, prec);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
        let (r2, o2) = x.clone().min_rational_prec(y.clone(), prec);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
        let (r, o) = x.max_rational_prec_round_ref_ref(&y, prec, Nearest);
        let (r2, o2) = x.max_rational_prec_ref_ref(&y, prec);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
    });

    float_rational_rounding_mode_triple_gen_var_20().test_properties(|(x, y, rm)| {
        let prec = x.significant_bits();
        let (r, o) = x.min_rational_prec_round_ref_ref(&y, prec, rm);
        let (r2, o2) = x.min_rational_round_ref_ref(&y, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
    });

    float_rational_rounding_mode_triple_gen_var_21().test_properties(|(x, y, rm)| {
        let prec = x.significant_bits();
        let (r, o) = x.max_rational_prec_round_ref_ref(&y, prec, rm);
        let (r2, o2) = x.max_rational_round_ref_ref(&y, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
    });

    float_rational_pair_gen().test_properties(|(x, y)| {
        let (r, o) = x.min_rational_round_ref_ref(&y, Nearest);
        let (r2, o2) = x.min_rational_ref_ref(&y);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
        let (r2, o2) = x.clone().min_rational(y.clone());
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
        let (r, o) = x.max_rational_round_ref_ref(&y, Nearest);
        let (r2, o2) = x.max_rational_ref_ref(&y);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
        let (r2, o2) = x.clone().max_rational_val_ref(&y);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
    });
}

#[test]
fn test_min_max_rational_units() {
    let test = |s,
                s_hex,
                t: &str,
                min_out: &str,
                min_hex: &str,
                min_o: Ordering,
                max_out: &str,
                max_hex: &str,
                max_o: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = t.parse::<Rational>().unwrap();

        let (r, o) = x.min_rational_prec_round_ref_ref(&y, 10, Nearest);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), min_out);
        assert_eq!(to_hex_string(&r), min_hex);
        assert_eq!(o, min_o);
        let (r2, o2) = x.clone().min_rational_prec_round(y.clone(), 10, Nearest);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
        let (r2, o2) = x.min_rational_prec_ref_ref(&y, 10);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);

        let (r, o) = x.max_rational_prec_round_ref_ref(&y, 10, Nearest);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), max_out);
        assert_eq!(to_hex_string(&r), max_hex);
        assert_eq!(o, max_o);
        let (r2, o2) = x.max_rational_prec_round_ref_val(y.clone(), 10, Nearest);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
    };
    // a NaN Float yields the Rational (rounded); infinities compare exactly; on zero ties min keeps
    // the Float's zero and max prefers the positive zero; otherwise the true winner is rounded to
    // precision 10
    test(
        "NaN",
        "NaN",
        "1/3",
        "0.33350",
        "0x0.556#10",
        Greater,
        "0.33350",
        "0x0.556#10",
        Greater,
    );
    test(
        "NaN",
        "NaN",
        "-1/3",
        "-0.33350",
        "-0x0.556#10",
        Less,
        "-0.33350",
        "-0x0.556#10",
        Less,
    );
    test(
        "NaN",
        "NaN",
        "22/7",
        "3.1445",
        "0x3.25#10",
        Greater,
        "3.1445",
        "0x3.25#10",
        Greater,
    );
    test(
        "NaN",
        "NaN",
        "7",
        "7.0000",
        "0x7.00#10",
        Equal,
        "7.0000",
        "0x7.00#10",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "3/8",
        "0.37500",
        "0x0.600#10",
        Equal,
        "0.37500",
        "0x0.600#10",
        Equal,
    );
    test(
        "NaN", "NaN", "0", "0.0", "0x0.0", Equal, "0.0", "0x0.0", Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "1/3",
        "0.33350",
        "0x0.556#10",
        Greater,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-1/3",
        "-0.33350",
        "-0x0.556#10",
        Less,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "22/7",
        "3.1445",
        "0x3.25#10",
        Greater,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "7",
        "7.0000",
        "0x7.00#10",
        Equal,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "3/8",
        "0.37500",
        "0x0.600#10",
        Equal,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "Infinity", "Infinity", "0", "0.0", "0x0.0", Equal, "Infinity", "Infinity", Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "1/3",
        "-Infinity",
        "-Infinity",
        Equal,
        "0.33350",
        "0x0.556#10",
        Greater,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-1/3",
        "-Infinity",
        "-Infinity",
        Equal,
        "-0.33350",
        "-0x0.556#10",
        Less,
    );
    test(
        "-Infinity",
        "-Infinity",
        "22/7",
        "-Infinity",
        "-Infinity",
        Equal,
        "3.1445",
        "0x3.25#10",
        Greater,
    );
    test(
        "-Infinity",
        "-Infinity",
        "7",
        "-Infinity",
        "-Infinity",
        Equal,
        "7.0000",
        "0x7.00#10",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "3/8",
        "-Infinity",
        "-Infinity",
        Equal,
        "0.37500",
        "0x0.600#10",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0",
        "-Infinity",
        "-Infinity",
        Equal,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "1/3",
        "0.0",
        "0x0.0",
        Equal,
        "0.33350",
        "0x0.556#10",
        Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "-1/3",
        "-0.33350",
        "-0x0.556#10",
        Less,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "22/7",
        "0.0",
        "0x0.0",
        Equal,
        "3.1445",
        "0x3.25#10",
        Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "7",
        "0.0",
        "0x0.0",
        Equal,
        "7.0000",
        "0x7.00#10",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "3/8",
        "0.0",
        "0x0.0",
        Equal,
        "0.37500",
        "0x0.600#10",
        Equal,
    );
    test(
        "0.0", "0x0.0", "0", "0.0", "0x0.0", Equal, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "1/3",
        "-0.0",
        "-0x0.0",
        Equal,
        "0.33350",
        "0x0.556#10",
        Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-1/3",
        "-0.33350",
        "-0x0.556#10",
        Less,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "22/7",
        "-0.0",
        "-0x0.0",
        Equal,
        "3.1445",
        "0x3.25#10",
        Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "7",
        "-0.0",
        "-0x0.0",
        Equal,
        "7.0000",
        "0x7.00#10",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "3/8",
        "-0.0",
        "-0x0.0",
        Equal,
        "0.37500",
        "0x0.600#10",
        Equal,
    );
    test(
        "-0.0", "-0x0.0", "0", "-0.0", "-0x0.0", Equal, "0.0", "0x0.0", Equal,
    );
    test(
        "10.0",
        "0xa.0#3",
        "1/3",
        "0.33350",
        "0x0.556#10",
        Greater,
        "10.000",
        "0xa.00#10",
        Equal,
    );
    test(
        "10.0",
        "0xa.0#3",
        "-1/3",
        "-0.33350",
        "-0x0.556#10",
        Less,
        "10.000",
        "0xa.00#10",
        Equal,
    );
    test(
        "10.0",
        "0xa.0#3",
        "22/7",
        "3.1445",
        "0x3.25#10",
        Greater,
        "10.000",
        "0xa.00#10",
        Equal,
    );
    test(
        "10.0",
        "0xa.0#3",
        "7",
        "7.0000",
        "0x7.00#10",
        Equal,
        "10.000",
        "0xa.00#10",
        Equal,
    );
    test(
        "10.0",
        "0xa.0#3",
        "3/8",
        "0.37500",
        "0x0.600#10",
        Equal,
        "10.000",
        "0xa.00#10",
        Equal,
    );
    test(
        "10.0",
        "0xa.0#3",
        "0",
        "0.0",
        "0x0.0",
        Equal,
        "10.000",
        "0xa.00#10",
        Equal,
    );
    test(
        "-10.0",
        "-0xa.0#3",
        "1/3",
        "-10.000",
        "-0xa.00#10",
        Equal,
        "0.33350",
        "0x0.556#10",
        Greater,
    );
    test(
        "-10.0",
        "-0xa.0#3",
        "-1/3",
        "-10.000",
        "-0xa.00#10",
        Equal,
        "-0.33350",
        "-0x0.556#10",
        Less,
    );
    test(
        "-10.0",
        "-0xa.0#3",
        "22/7",
        "-10.000",
        "-0xa.00#10",
        Equal,
        "3.1445",
        "0x3.25#10",
        Greater,
    );
    test(
        "-10.0",
        "-0xa.0#3",
        "7",
        "-10.000",
        "-0xa.00#10",
        Equal,
        "7.0000",
        "0x7.00#10",
        Equal,
    );
    test(
        "-10.0",
        "-0xa.0#3",
        "3/8",
        "-10.000",
        "-0xa.00#10",
        Equal,
        "0.37500",
        "0x0.600#10",
        Equal,
    );
    test(
        "-10.0",
        "-0xa.0#3",
        "0",
        "-10.000",
        "-0xa.00#10",
        Equal,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        "1/3",
        "0.33350",
        "0x0.556#10",
        Greater,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        "-1/3",
        "-0.33350",
        "-0x0.556#10",
        Less,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        "22/7",
        "3.0000",
        "0x3.00#10",
        Equal,
        "3.1445",
        "0x3.25#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        "7",
        "3.0000",
        "0x3.00#10",
        Equal,
        "7.0000",
        "0x7.00#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        "3/8",
        "0.37500",
        "0x0.600#10",
        Equal,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        "0",
        "0.0",
        "0x0.0",
        Equal,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "10.5",
        "0xa.8#6",
        "1/3",
        "0.33350",
        "0x0.556#10",
        Greater,
        "10.500",
        "0xa.80#10",
        Equal,
    );
    test(
        "10.5",
        "0xa.8#6",
        "-1/3",
        "-0.33350",
        "-0x0.556#10",
        Less,
        "10.500",
        "0xa.80#10",
        Equal,
    );
    test(
        "10.5",
        "0xa.8#6",
        "22/7",
        "3.1445",
        "0x3.25#10",
        Greater,
        "10.500",
        "0xa.80#10",
        Equal,
    );
    test(
        "10.5",
        "0xa.8#6",
        "7",
        "7.0000",
        "0x7.00#10",
        Equal,
        "10.500",
        "0xa.80#10",
        Equal,
    );
    test(
        "10.5",
        "0xa.8#6",
        "3/8",
        "0.37500",
        "0x0.600#10",
        Equal,
        "10.500",
        "0xa.80#10",
        Equal,
    );
    test(
        "10.5",
        "0xa.8#6",
        "0",
        "0.0",
        "0x0.0",
        Equal,
        "10.500",
        "0xa.80#10",
        Equal,
    );
}

#[test]
fn test_min_max_rational_prec_round() {
    let test = |s,
                s_hex,
                t: &str,
                prec,
                rm: RoundingMode,
                min_out: &str,
                min_hex: &str,
                min_o: Ordering,
                max_out: &str,
                max_hex: &str,
                max_o: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = t.parse::<Rational>().unwrap();

        let (r, o) = x.min_rational_prec_round_ref_ref(&y, prec, rm);
        assert_eq!(r.to_string(), min_out);
        assert_eq!(to_hex_string(&r), min_hex);
        assert_eq!(o, min_o);
        let (r, o) = x.max_rational_prec_round_ref_ref(&y, prec, rm);
        assert_eq!(r.to_string(), max_out);
        assert_eq!(to_hex_string(&r), max_hex);
        assert_eq!(o, max_o);
    };
    // rounding the winner: 3 vs 22/7 and 4 vs 22/7 at precision 2 under each direction
    test(
        "3.0", "0x3.0#2", "22/7", 2, Floor, "3.0", "0x3.0#2", Equal, "3.0", "0x3.0#2", Less,
    );
    test(
        "3.0", "0x3.0#2", "22/7", 2, Ceiling, "3.0", "0x3.0#2", Equal, "4.0", "0x4.0#2", Greater,
    );
    test(
        "3.0", "0x3.0#2", "22/7", 2, Nearest, "3.0", "0x3.0#2", Equal, "3.0", "0x3.0#2", Less,
    );
    test(
        "4.0", "0x4.0#1", "22/7", 2, Floor, "3.0", "0x3.0#2", Less, "4.0", "0x4.0#2", Equal,
    );
    test(
        "4.0", "0x4.0#1", "22/7", 2, Ceiling, "4.0", "0x4.0#2", Greater, "4.0", "0x4.0#2", Equal,
    );
}

#[test]
fn min_max_rational_fail() {
    assert_panic!(Float::from(1u32).min_rational_prec_round(
        Rational::from_signeds(22i32, 7i32),
        0,
        Nearest
    ));
    assert_panic!(Float::from(1u32).max_rational_prec_round(
        Rational::from_signeds(22i32, 7i32),
        0,
        Nearest
    ));
    // Exact when the winner needs rounding
    assert_panic!(Float::from(4u32).min_rational_prec_round(
        Rational::from_signeds(22i32, 7i32),
        2,
        Exact
    ));
}
