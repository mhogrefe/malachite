// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Two};
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
    float_gen, float_pair_gen, float_pair_gen_var_10,
    float_unsigned_rounding_mode_triple_gen_var_1,
};
use malachite_float::{ComparableFloatRef, Float};
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
