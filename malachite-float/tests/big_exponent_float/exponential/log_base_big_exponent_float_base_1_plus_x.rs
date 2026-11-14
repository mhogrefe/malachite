// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::test_util::generators::{
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_12,
    float_float_unsigned_rounding_mode_quadruple_gen_var_13,
};
use malachite_float::{BigExponentFloat, Float};
use std::cmp::Ordering::{self, *};

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

fn same_pair(x: &(BigExponentFloat, Ordering), y: &(BigExponentFloat, Ordering)) -> bool {
    same(&x.0, &y.0) && x.1 == y.1
}

fn float_result_comparable(x: &Float, o: Ordering) -> bool {
    x.get_exponent().map_or_else(
        || !x.is_zero() || o == Equal,
        |e| e > Float::MIN_EXPONENT + 1 && e < Float::MAX_EXPONENT - 1,
    )
}

fn hex(s: &str) -> BigExponentFloat {
    BigExponentFloat::from_string_base(16, s).unwrap()
}

// Whether the exponent of `x`, if it has one, is at most 256 in absolute value, so that exact
// conversions and sums involving x stay small.
fn exponent_is_moderate(x: &BigExponentFloat) -> bool {
    x.get_exponent()
        .is_none_or(|e| e.unsigned_abs_ref() <= &256u32)
}

fn log_base_big_exponent_float_base_1_plus_x_prec_round_all(
    x: &BigExponentFloat,
    b: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x
        .clone()
        .log_base_big_exponent_float_base_1_plus_x_prec_round(b, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.log_base_big_exponent_float_base_1_plus_x_prec_round_ref(b, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.log_base_big_exponent_float_base_1_plus_x_prec_round_assign(b, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(
            &x.clone()
                .log_base_big_exponent_float_base_1_plus_x_prec(b, prec),
            &r
        ));
        assert!(same_pair(
            &x.log_base_big_exponent_float_base_1_plus_x_prec_ref(b, prec),
            &r
        ));
        let mut z = x.clone();
        let o = z.log_base_big_exponent_float_base_1_plus_x_prec_assign(b, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

#[test]
fn test_log_base_big_exponent_float_base_1_plus_x_prec_round() {
    // The expected values agree with FLINT's arb (see the `arf_logarithm_general` oracle mode).
    let cases: &[(&str, &str, u64, RoundingMode, &str, Ordering)] = &[
        ("0x3.243f6a8885a30", "0x3", 1, Nearest, "0x1.0", Less),
        ("0x3.243f6a8885a30", "0x3", 10, Floor, "0x1.4b", Less),
        ("0x3.243f6a8885a30", "0x3", 10, Ceiling, "0x1.4b8", Greater),
        ("0x3.243f6a8885a30", "0x3", 10, Down, "0x1.4b", Less),
        ("0x3.243f6a8885a30", "0x3", 10, Up, "0x1.4b8", Greater),
        ("0x3.243f6a8885a30", "0x3", 10, Nearest, "0x1.4b", Less),
        (
            "0x3.243f6a8885a30",
            "0x3",
            100,
            Nearest,
            "0x1.4b24550d616216685f5c9e1f6",
            Greater,
        ),
        ("0x3.243f6a8885a30", "0x0.8", 1, Nearest, "-0x2.0", Greater),
        ("0x3.243f6a8885a30", "0x0.8", 10, Floor, "-0x2.0d", Less),
        (
            "0x3.243f6a8885a30",
            "0x0.8",
            10,
            Ceiling,
            "-0x2.0c",
            Greater,
        ),
        ("0x3.243f6a8885a30", "0x0.8", 10, Down, "-0x2.0c", Greater),
        ("0x3.243f6a8885a30", "0x0.8", 10, Up, "-0x2.0d", Less),
        ("0x3.243f6a8885a30", "0x0.8", 10, Nearest, "-0x2.0d", Less),
        (
            "0x3.243f6a8885a30",
            "0x0.8",
            100,
            Nearest,
            "-0x2.0cd8f7baa695a2146d5d1f398",
            Greater,
        ),
        ("0x3.243f6a8885a30", "0x1.0", 1, Nearest, "NaN", Equal),
        ("0x3.243f6a8885a30", "0x1.0", 10, Floor, "NaN", Equal),
        ("0x3.243f6a8885a30", "0x1.0", 10, Ceiling, "NaN", Equal),
        ("0x3.243f6a8885a30", "0x1.0", 10, Down, "NaN", Equal),
        ("0x3.243f6a8885a30", "0x1.0", 10, Up, "NaN", Equal),
        ("0x3.243f6a8885a30", "0x1.0", 10, Nearest, "NaN", Equal),
        ("0x3.243f6a8885a30", "0x1.0", 100, Nearest, "NaN", Equal),
        ("0x3.243f6a8885a30", "0x1.0", 300, Exact, "NaN", Equal),
        ("0x0.8", "0x3", 1, Nearest, "0x0.4", Less),
        ("0x0.8", "0x3", 10, Floor, "0x0.5e6", Less),
        ("0x0.8", "0x3", 10, Ceiling, "0x0.5e8", Greater),
        ("0x0.8", "0x3", 10, Down, "0x0.5e6", Less),
        ("0x0.8", "0x3", 10, Up, "0x0.5e8", Greater),
        ("0x0.8", "0x3", 10, Nearest, "0x0.5e8", Greater),
        (
            "0x0.8",
            "0x3",
            100,
            Nearest,
            "0x0.5e7b633e565616b1fbc150887",
            Greater,
        ),
        ("0x0.8", "0x0.8", 1, Nearest, "-0x0.8", Greater),
        ("0x0.8", "0x0.8", 10, Floor, "-0x0.96", Less),
        ("0x0.8", "0x0.8", 10, Ceiling, "-0x0.95c", Greater),
        ("0x0.8", "0x0.8", 10, Down, "-0x0.95c", Greater),
        ("0x0.8", "0x0.8", 10, Up, "-0x0.96", Less),
        ("0x0.8", "0x0.8", 10, Nearest, "-0x0.95c", Greater),
        (
            "0x0.8",
            "0x0.8",
            100,
            Nearest,
            "-0x0.95c01a39fbd6879fa00b120a",
            Greater,
        ),
        ("0x0.8", "0x1.0", 1, Nearest, "NaN", Equal),
        ("0x0.8", "0x1.0", 10, Floor, "NaN", Equal),
        ("0x0.8", "0x1.0", 10, Ceiling, "NaN", Equal),
        ("0x0.8", "0x1.0", 10, Down, "NaN", Equal),
        ("0x0.8", "0x1.0", 10, Up, "NaN", Equal),
        ("0x0.8", "0x1.0", 10, Nearest, "NaN", Equal),
        ("0x0.8", "0x1.0", 100, Nearest, "NaN", Equal),
        ("0x0.8", "0x1.0", 300, Exact, "NaN", Equal),
        ("-0x0.8", "0x3", 1, Nearest, "-0x0.8", Greater),
        ("-0x0.8", "0x3", 10, Floor, "-0x0.a1c", Less),
        ("-0x0.8", "0x3", 10, Ceiling, "-0x0.a18", Greater),
        ("-0x0.8", "0x3", 10, Down, "-0x0.a18", Greater),
        ("-0x0.8", "0x3", 10, Up, "-0x0.a1c", Less),
        ("-0x0.8", "0x3", 10, Nearest, "-0x0.a18", Greater),
        (
            "-0x0.8",
            "0x3",
            100,
            Nearest,
            "-0x0.a1849cc1a9a9e94e043eaf779",
            Greater,
        ),
        ("-0x0.8", "0x0.8", 1, Nearest, "0x1.0", Equal),
        ("-0x0.8", "0x0.8", 10, Floor, "0x1.0", Equal),
        ("-0x0.8", "0x0.8", 10, Ceiling, "0x1.0", Equal),
        ("-0x0.8", "0x0.8", 10, Down, "0x1.0", Equal),
        ("-0x0.8", "0x0.8", 10, Up, "0x1.0", Equal),
        ("-0x0.8", "0x0.8", 10, Nearest, "0x1.0", Equal),
        ("-0x0.8", "0x0.8", 100, Nearest, "0x1.0", Equal),
        ("-0x0.8", "0x0.8", 300, Exact, "0x1.0", Equal),
        ("-0x0.8", "0x1.0", 1, Nearest, "NaN", Equal),
        ("-0x0.8", "0x1.0", 10, Floor, "NaN", Equal),
        ("-0x0.8", "0x1.0", 10, Ceiling, "NaN", Equal),
        ("-0x0.8", "0x1.0", 10, Down, "NaN", Equal),
        ("-0x0.8", "0x1.0", 10, Up, "NaN", Equal),
        ("-0x0.8", "0x1.0", 10, Nearest, "NaN", Equal),
        ("-0x0.8", "0x1.0", 100, Nearest, "NaN", Equal),
        ("-0x0.8", "0x1.0", 300, Exact, "NaN", Equal),
        ("0x2", "0x3", 1, Nearest, "0x1.0", Equal),
        ("0x2", "0x3", 10, Floor, "0x1.0", Equal),
        ("0x2", "0x3", 10, Ceiling, "0x1.0", Equal),
        ("0x2", "0x3", 10, Down, "0x1.0", Equal),
        ("0x2", "0x3", 10, Up, "0x1.0", Equal),
        ("0x2", "0x3", 10, Nearest, "0x1.0", Equal),
        ("0x2", "0x3", 100, Nearest, "0x1.0", Equal),
        ("0x2", "0x3", 300, Exact, "0x1.0", Equal),
        ("0x2", "0x0.8", 1, Nearest, "-0x2.0", Less),
        ("0x2", "0x0.8", 10, Floor, "-0x1.96", Less),
        ("0x2", "0x0.8", 10, Ceiling, "-0x1.958", Greater),
        ("0x2", "0x0.8", 10, Down, "-0x1.958", Greater),
        ("0x2", "0x0.8", 10, Up, "-0x1.96", Less),
        ("0x2", "0x0.8", 10, Nearest, "-0x1.96", Less),
        (
            "0x2",
            "0x0.8",
            100,
            Nearest,
            "-0x1.95c01a39fbd6879fa00b120a",
            Greater,
        ),
        ("0x2", "0x1.0", 1, Nearest, "NaN", Equal),
        ("0x2", "0x1.0", 10, Floor, "NaN", Equal),
        ("0x2", "0x1.0", 10, Ceiling, "NaN", Equal),
        ("0x2", "0x1.0", 10, Down, "NaN", Equal),
        ("0x2", "0x1.0", 10, Up, "NaN", Equal),
        ("0x2", "0x1.0", 10, Nearest, "NaN", Equal),
        ("0x2", "0x1.0", 100, Nearest, "NaN", Equal),
        ("0x2", "0x1.0", 300, Exact, "NaN", Equal),
        ("0x8", "0x3", 1, Nearest, "0x2.0", Equal),
        ("0x8", "0x3", 10, Floor, "0x2.0", Equal),
        ("0x8", "0x3", 10, Ceiling, "0x2.0", Equal),
        ("0x8", "0x3", 10, Down, "0x2.0", Equal),
        ("0x8", "0x3", 10, Up, "0x2.0", Equal),
        ("0x8", "0x3", 10, Nearest, "0x2.0", Equal),
        ("0x8", "0x3", 100, Nearest, "0x2.0", Equal),
        ("0x8", "0x3", 300, Exact, "0x2.0", Equal),
        ("0x8", "0x0.8", 1, Nearest, "-0x4.0", Less),
        ("0x8", "0x0.8", 10, Floor, "-0x3.2c", Less),
        ("0x8", "0x0.8", 10, Ceiling, "-0x3.2b", Greater),
        ("0x8", "0x0.8", 10, Down, "-0x3.2b", Greater),
        ("0x8", "0x0.8", 10, Up, "-0x3.2c", Less),
        ("0x8", "0x0.8", 10, Nearest, "-0x3.2c", Less),
        (
            "0x8",
            "0x0.8",
            100,
            Nearest,
            "-0x3.2b803473f7ad0f3f40162414",
            Greater,
        ),
        ("0x8", "0x1.0", 1, Nearest, "NaN", Equal),
        ("0x8", "0x1.0", 10, Floor, "NaN", Equal),
        ("0x8", "0x1.0", 10, Ceiling, "NaN", Equal),
        ("0x8", "0x1.0", 10, Down, "NaN", Equal),
        ("0x8", "0x1.0", 10, Up, "NaN", Equal),
        ("0x8", "0x1.0", 10, Nearest, "NaN", Equal),
        ("0x8", "0x1.0", 100, Nearest, "NaN", Equal),
        ("0x8", "0x1.0", 300, Exact, "NaN", Equal),
        ("0x1a", "0x3", 1, Nearest, "0x4.0", Greater),
        ("0x1a", "0x3", 10, Floor, "0x3.0", Equal),
        ("0x1a", "0x3", 10, Ceiling, "0x3.0", Equal),
        ("0x1a", "0x3", 10, Down, "0x3.0", Equal),
        ("0x1a", "0x3", 10, Up, "0x3.0", Equal),
        ("0x1a", "0x3", 10, Nearest, "0x3.0", Equal),
        ("0x1a", "0x3", 100, Nearest, "0x3.0", Equal),
        ("0x1a", "0x3", 300, Exact, "0x3.0", Equal),
        ("0x1a", "0x0.8", 1, Nearest, "-0x4.0", Greater),
        ("0x1a", "0x0.8", 10, Floor, "-0x4.c2", Less),
        ("0x1a", "0x0.8", 10, Ceiling, "-0x4.c", Greater),
        ("0x1a", "0x0.8", 10, Down, "-0x4.c", Greater),
        ("0x1a", "0x0.8", 10, Up, "-0x4.c2", Less),
        ("0x1a", "0x0.8", 10, Nearest, "-0x4.c2", Less),
        (
            "0x1a",
            "0x0.8",
            100,
            Nearest,
            "-0x4.c1404eadf38396dee021361e",
            Greater,
        ),
        ("0x1a", "0x1.0", 1, Nearest, "NaN", Equal),
        ("0x1a", "0x1.0", 10, Floor, "NaN", Equal),
        ("0x1a", "0x1.0", 10, Ceiling, "NaN", Equal),
        ("0x1a", "0x1.0", 10, Down, "NaN", Equal),
        ("0x1a", "0x1.0", 10, Up, "NaN", Equal),
        ("0x1a", "0x1.0", 10, Nearest, "NaN", Equal),
        ("0x1a", "0x1.0", 100, Nearest, "NaN", Equal),
        ("0x1a", "0x1.0", 300, Exact, "NaN", Equal),
        ("0x1.0E+300000000", "0x3", 1, Nearest, "0x2.0E+7", Less),
        ("0x1.0E+300000000", "0x3", 10, Floor, "0x2.d2E+7", Less),
        ("0x1.0E+300000000", "0x3", 10, Ceiling, "0x2.d3E+7", Greater),
        ("0x1.0E+300000000", "0x3", 10, Down, "0x2.d2E+7", Less),
        ("0x1.0E+300000000", "0x3", 10, Up, "0x2.d3E+7", Greater),
        ("0x1.0E+300000000", "0x3", 10, Nearest, "0x2.d2E+7", Less),
        (
            "0x1.0E+300000000",
            "0x3",
            100,
            Nearest,
            "0x2d20ab38.4926d76dad9a45cc4c",
            Greater,
        ),
        (
            "0x1.0E+300000000",
            "0x0.8",
            1,
            Nearest,
            "-0x4.0E+7",
            Greater,
        ),
        ("0x1.0E+300000000", "0x0.8", 10, Floor, "-0x4.7aE+7", Less),
        (
            "0x1.0E+300000000",
            "0x0.8",
            10,
            Ceiling,
            "-0x4.78E+7",
            Greater,
        ),
        ("0x1.0E+300000000", "0x0.8", 10, Down, "-0x4.78E+7", Greater),
        ("0x1.0E+300000000", "0x0.8", 10, Up, "-0x4.7aE+7", Less),
        (
            "0x1.0E+300000000",
            "0x0.8",
            10,
            Nearest,
            "-0x4.78E+7",
            Greater,
        ),
        (
            "0x1.0E+300000000",
            "0x0.8",
            100,
            Nearest,
            "-0x4.7868cE+7",
            Greater,
        ),
        ("0x1.0E+300000000", "0x1.0", 1, Nearest, "NaN", Equal),
        ("0x1.0E+300000000", "0x1.0", 10, Floor, "NaN", Equal),
        ("0x1.0E+300000000", "0x1.0", 10, Ceiling, "NaN", Equal),
        ("0x1.0E+300000000", "0x1.0", 10, Down, "NaN", Equal),
        ("0x1.0E+300000000", "0x1.0", 10, Up, "NaN", Equal),
        ("0x1.0E+300000000", "0x1.0", 10, Nearest, "NaN", Equal),
        ("0x1.0E+300000000", "0x1.0", 100, Nearest, "NaN", Equal),
        ("0x1.0E+300000000", "0x1.0", 300, Exact, "NaN", Equal),
        (
            "0x1.8E-300000000",
            "0x3",
            1,
            Nearest,
            "0x1.0E-300000000",
            Less,
        ),
        (
            "0x1.8E-300000000",
            "0x3",
            10,
            Floor,
            "0x1.5d8E-300000000",
            Less,
        ),
        (
            "0x1.8E-300000000",
            "0x3",
            10,
            Ceiling,
            "0x1.5eE-300000000",
            Greater,
        ),
        (
            "0x1.8E-300000000",
            "0x3",
            10,
            Down,
            "0x1.5d8E-300000000",
            Less,
        ),
        (
            "0x1.8E-300000000",
            "0x3",
            10,
            Up,
            "0x1.5eE-300000000",
            Greater,
        ),
        (
            "0x1.8E-300000000",
            "0x3",
            10,
            Nearest,
            "0x1.5d8E-300000000",
            Less,
        ),
        (
            "0x1.8E-300000000",
            "0x3",
            100,
            Nearest,
            "0x1.5d88282cd90fe05161c272afeE-300000000",
            Greater,
        ),
        (
            "0x1.8E-300000000",
            "0x0.8",
            1,
            Nearest,
            "-0x2.0E-300000000",
            Greater,
        ),
        (
            "0x1.8E-300000000",
            "0x0.8",
            10,
            Floor,
            "-0x2.2aE-300000000",
            Less,
        ),
        (
            "0x1.8E-300000000",
            "0x0.8",
            10,
            Ceiling,
            "-0x2.29E-300000000",
            Greater,
        ),
        (
            "0x1.8E-300000000",
            "0x0.8",
            10,
            Down,
            "-0x2.29E-300000000",
            Greater,
        ),
        (
            "0x1.8E-300000000",
            "0x0.8",
            10,
            Up,
            "-0x2.2aE-300000000",
            Less,
        ),
        (
            "0x1.8E-300000000",
            "0x0.8",
            10,
            Nearest,
            "-0x2.2aE-300000000",
            Less,
        ),
        (
            "0x1.8E-300000000",
            "0x0.8",
            100,
            Nearest,
            "-0x2.29feb17c1447d2333b97fc714E-300000000",
            Less,
        ),
        ("0x1.8E-300000000", "0x1.0", 1, Nearest, "NaN", Equal),
        ("0x1.8E-300000000", "0x1.0", 10, Floor, "NaN", Equal),
        ("0x1.8E-300000000", "0x1.0", 10, Ceiling, "NaN", Equal),
        ("0x1.8E-300000000", "0x1.0", 10, Down, "NaN", Equal),
        ("0x1.8E-300000000", "0x1.0", 10, Up, "NaN", Equal),
        ("0x1.8E-300000000", "0x1.0", 10, Nearest, "NaN", Equal),
        ("0x1.8E-300000000", "0x1.0", 100, Nearest, "NaN", Equal),
        ("0x1.8E-300000000", "0x1.0", 300, Exact, "NaN", Equal),
        ("-0x3", "0x3", 1, Nearest, "NaN", Equal),
        ("-0x3", "0x3", 10, Floor, "NaN", Equal),
        ("-0x3", "0x3", 10, Ceiling, "NaN", Equal),
        ("-0x3", "0x3", 10, Down, "NaN", Equal),
        ("-0x3", "0x3", 10, Up, "NaN", Equal),
        ("-0x3", "0x3", 10, Nearest, "NaN", Equal),
        ("-0x3", "0x3", 100, Nearest, "NaN", Equal),
        ("-0x3", "0x3", 300, Exact, "NaN", Equal),
        ("-0x3", "0x0.8", 1, Nearest, "NaN", Equal),
        ("-0x3", "0x0.8", 10, Floor, "NaN", Equal),
        ("-0x3", "0x0.8", 10, Ceiling, "NaN", Equal),
        ("-0x3", "0x0.8", 10, Down, "NaN", Equal),
        ("-0x3", "0x0.8", 10, Up, "NaN", Equal),
        ("-0x3", "0x0.8", 10, Nearest, "NaN", Equal),
        ("-0x3", "0x0.8", 100, Nearest, "NaN", Equal),
        ("-0x3", "0x0.8", 300, Exact, "NaN", Equal),
        ("-0x3", "0x1.0", 1, Nearest, "NaN", Equal),
        ("-0x3", "0x1.0", 10, Floor, "NaN", Equal),
        ("-0x3", "0x1.0", 10, Ceiling, "NaN", Equal),
        ("-0x3", "0x1.0", 10, Down, "NaN", Equal),
        ("-0x3", "0x1.0", 10, Up, "NaN", Equal),
        ("-0x3", "0x1.0", 10, Nearest, "NaN", Equal),
        ("-0x3", "0x1.0", 100, Nearest, "NaN", Equal),
        ("-0x3", "0x1.0", 300, Exact, "NaN", Equal),
        ("0x0", "0x3", 1, Nearest, "0x0.0", Equal),
        ("0x0", "0x3", 10, Floor, "0x0.0", Equal),
        ("0x0", "0x3", 10, Ceiling, "0x0.0", Equal),
        ("0x0", "0x3", 10, Down, "0x0.0", Equal),
        ("0x0", "0x3", 10, Up, "0x0.0", Equal),
        ("0x0", "0x3", 10, Nearest, "0x0.0", Equal),
        ("0x0", "0x3", 100, Nearest, "0x0.0", Equal),
        ("0x0", "0x3", 300, Exact, "0x0.0", Equal),
        ("0x0", "0x0.8", 1, Nearest, "0x0.0", Equal),
        ("0x0", "0x0.8", 10, Floor, "0x0.0", Equal),
        ("0x0", "0x0.8", 10, Ceiling, "0x0.0", Equal),
        ("0x0", "0x0.8", 10, Down, "0x0.0", Equal),
        ("0x0", "0x0.8", 10, Up, "0x0.0", Equal),
        ("0x0", "0x0.8", 10, Nearest, "0x0.0", Equal),
        ("0x0", "0x0.8", 100, Nearest, "0x0.0", Equal),
        ("0x0", "0x0.8", 300, Exact, "0x0.0", Equal),
        ("0x0", "0x1.0", 1, Nearest, "NaN", Equal),
        ("0x0", "0x1.0", 10, Floor, "NaN", Equal),
        ("0x0", "0x1.0", 10, Ceiling, "NaN", Equal),
        ("0x0", "0x1.0", 10, Down, "NaN", Equal),
        ("0x0", "0x1.0", 10, Up, "NaN", Equal),
        ("0x0", "0x1.0", 10, Nearest, "NaN", Equal),
        ("0x0", "0x1.0", 100, Nearest, "NaN", Equal),
        ("0x0", "0x1.0", 300, Exact, "NaN", Equal),
        ("0x2540be400.0", "0x3", 1, Nearest, "0x1.0E+1", Less),
        ("0x2540be400.0", "0x3", 10, Floor, "0x14.f", Less),
        ("0x2540be400.0", "0x3", 10, Ceiling, "0x14.f8", Greater),
        ("0x2540be400.0", "0x3", 10, Down, "0x14.f", Less),
        ("0x2540be400.0", "0x3", 10, Up, "0x14.f8", Greater),
        ("0x2540be400.0", "0x3", 10, Nearest, "0x14.f8", Greater),
        (
            "0x2540be400.0",
            "0x3",
            100,
            Nearest,
            "0x14.f5832b7ae9c6d09fa0e6795e",
            Less,
        ),
        ("0x2540be400.0", "0x0.8", 1, Nearest, "-0x2.0E+1", Greater),
        ("0x2540be400.0", "0x0.8", 10, Floor, "-0x21.4", Less),
        ("0x2540be400.0", "0x0.8", 10, Ceiling, "-0x21.3", Greater),
        ("0x2540be400.0", "0x0.8", 10, Down, "-0x21.3", Greater),
        ("0x2540be400.0", "0x0.8", 10, Up, "-0x21.4", Less),
        ("0x2540be400.0", "0x0.8", 10, Nearest, "-0x21.4", Less),
        (
            "0x2540be400.0",
            "0x0.8",
            100,
            Nearest,
            "-0x21.3822cbd8aaedf916de6f2404",
            Less,
        ),
        ("0x2540be400.0", "0x1.0", 1, Nearest, "NaN", Equal),
        ("0x2540be400.0", "0x1.0", 10, Floor, "NaN", Equal),
        ("0x2540be400.0", "0x1.0", 10, Ceiling, "NaN", Equal),
        ("0x2540be400.0", "0x1.0", 10, Down, "NaN", Equal),
        ("0x2540be400.0", "0x1.0", 10, Up, "NaN", Equal),
        ("0x2540be400.0", "0x1.0", 10, Nearest, "NaN", Equal),
        ("0x2540be400.0", "0x1.0", 100, Nearest, "NaN", Equal),
        ("0x2540be400.0", "0x1.0", 300, Exact, "NaN", Equal),
        (
            "0xd.0f17968968E+416942668",
            "0x3",
            10,
            Nearest,
            "0x3.ecE+7",
            Greater,
        ),
        (
            "0xc.2977b473e4E-268435507",
            "0x3",
            10,
            Nearest,
            "0xb.14E-268435507",
            Greater,
        ),
        (
            "0x6.44a13472e5E+388273872",
            "0x3",
            10,
            Nearest,
            "0x3.a6E+7",
            Less,
        ),
        (
            "0xa.766a9bd7e6E-268435507",
            "0x3",
            10,
            Nearest,
            "0x9.84E-268435507",
            Less,
        ),
        (
            "0x1.288e4af4d2fcE+401569551",
            "0x3",
            10,
            Nearest,
            "0x3.c7E+7",
            Greater,
        ),
        (
            "0xc.9754173298aE-268435507",
            "0x3",
            10,
            Nearest,
            "0xb.78E-268435507",
            Greater,
        ),
        (
            "0x1.c183a55eed38E+409879313",
            "0x3",
            10,
            Nearest,
            "0x3.daE+7",
            Less,
        ),
        (
            "0xf.f03efa7e188E-268435507",
            "0x3",
            10,
            Nearest,
            "0xe.8E-268435507",
            Less,
        ),
        (
            "0x3.1ef199a98426E+417358108",
            "0x3",
            10,
            Nearest,
            "0x3.edE+7",
            Greater,
        ),
        (
            "0xc.85c02b892f1E-268435507",
            "0x3",
            10,
            Nearest,
            "0xb.68E-268435507",
            Greater,
        ),
        (
            "0xf.bf6c57e5b6eE+405308942",
            "0x3",
            10,
            Nearest,
            "0x3.cfE+7",
            Less,
        ),
        (
            "0x1.0bec8ff179bE-268435506",
            "0x3",
            10,
            Nearest,
            "0xf.3cE-268435507",
            Less,
        ),
        (
            "0x6.9e9c1d3437928E+417358107",
            "0x3",
            10,
            Nearest,
            "0x3.edE+7",
            Greater,
        ),
        (
            "0xd.31225957b0798E-268435507",
            "0x3",
            10,
            Nearest,
            "0xc.04E-268435507",
            Greater,
        ),
        (
            "0x4.bf2ed608df1aE+373731824",
            "0x3",
            10,
            Nearest,
            "0x3.83E+7",
            Less,
        ),
        (
            "0xd.7bd6fdd3b94aE-268435507",
            "0x3",
            10,
            Nearest,
            "0xc.44E-268435507",
            Less,
        ),
        (
            "0x7.1c2ef93ce1E+385988778",
            "0x3",
            10,
            Floor,
            "0x3.a1E+7",
            Less,
        ),
        (
            "0xb.dc9092016E-268435507",
            "0x3",
            10,
            Floor,
            "0xa.ccE-268435507",
            Less,
        ),
        (
            "0x7.1c2ef93ce1E+385988778",
            "0x3",
            10,
            Ceiling,
            "0x3.a2E+7",
            Greater,
        ),
        (
            "0xb.dc9092016E-268435507",
            "0x3",
            10,
            Ceiling,
            "0xa.dE-268435507",
            Greater,
        ),
        (
            "0x3.df748114408E+398453337",
            "0x3",
            10,
            Floor,
            "0x3.beE+7",
            Less,
        ),
        (
            "0xa.511049ddeaE-268435507",
            "0x3",
            10,
            Floor,
            "0x9.6E-268435507",
            Less,
        ),
        (
            "0x3.df748114408E+398453337",
            "0x3",
            10,
            Ceiling,
            "0x3.bfE+7",
            Greater,
        ),
        (
            "0xa.511049ddeaE-268435507",
            "0x3",
            10,
            Ceiling,
            "0x9.64E-268435507",
            Greater,
        ),
        (
            "0x6.3e87804b4a5E+376017013",
            "0x3",
            10,
            Floor,
            "0x3.89E+7",
            Less,
        ),
        (
            "0x1.0bc968465ce8E-268435506",
            "0x3",
            10,
            Floor,
            "0xf.3cE-268435507",
            Less,
        ),
        (
            "0x6.3e87804b4a5E+376017013",
            "0x3",
            10,
            Ceiling,
            "0x3.8aE+7",
            Greater,
        ),
        (
            "0x1.0bc968465ce8E-268435506",
            "0x3",
            10,
            Ceiling,
            "0xf.4E-268435507",
            Greater,
        ),
        (
            "0x1.f7c527569cc8E+375186031",
            "0x3",
            10,
            Floor,
            "0x3.86E+7",
            Less,
        ),
        (
            "0x9.c471097214cE-268435507",
            "0x3",
            10,
            Floor,
            "0x8.eE-268435507",
            Less,
        ),
        (
            "0x1.f7c527569cc8E+375186031",
            "0x3",
            10,
            Ceiling,
            "0x3.87E+7",
            Greater,
        ),
        (
            "0x9.c471097214cE-268435507",
            "0x3",
            10,
            Ceiling,
            "0x8.e4E-268435507",
            Greater,
        ),
        (
            "0x2.4a4f88ae1be6E+405101200",
            "0x3",
            10,
            Floor,
            "0x3.cfE+7",
            Less,
        ),
        (
            "0x1.151ffcbeb8a4E-268435506",
            "0x3",
            10,
            Floor,
            "0xf.c4E-268435507",
            Less,
        ),
        (
            "0x2.4a4f88ae1be6E+405101200",
            "0x3",
            10,
            Ceiling,
            "0x3.dE+7",
            Greater,
        ),
        (
            "0x1.151ffcbeb8a4E-268435506",
            "0x3",
            10,
            Ceiling,
            "0xf.c8E-268435507",
            Greater,
        ),
        (
            "0x1.f9ad7d768e79E+398453384",
            "0x3",
            10,
            Floor,
            "0x3.beE+7",
            Less,
        ),
        (
            "0xa.59da536a92b8E-268435507",
            "0x3",
            10,
            Floor,
            "0x9.68E-268435507",
            Less,
        ),
        (
            "0x1.f9ad7d768e79E+398453384",
            "0x3",
            10,
            Ceiling,
            "0x3.bfE+7",
            Greater,
        ),
        (
            "0xa.59da536a92b8E-268435507",
            "0x3",
            10,
            Ceiling,
            "0x9.6cE-268435507",
            Greater,
        ),
        (
            "0x3.206cb14aaf0f2E+405932176",
            "0x3",
            10,
            Floor,
            "0x3.d1E+7",
            Less,
        ),
        (
            "0xc.a6b582d158cbE-268435507",
            "0x3",
            10,
            Floor,
            "0xb.84E-268435507",
            Less,
        ),
        (
            "0x3.206cb14aaf0f2E+405932176",
            "0x3",
            10,
            Ceiling,
            "0x3.d2E+7",
            Greater,
        ),
        (
            "0xc.a6b582d158cbE-268435507",
            "0x3",
            10,
            Ceiling,
            "0xb.88E-268435507",
            Greater,
        ),
        (
            "0x1.e9254b26f72e2E+410918037",
            "0x3",
            10,
            Floor,
            "0x3.dcE+7",
            Less,
        ),
        (
            "0x9.cd3affe312d48E-268435507",
            "0x3",
            10,
            Floor,
            "0x8.e8E-268435507",
            Less,
        ),
        (
            "0x1.e9254b26f72e2E+410918037",
            "0x3",
            10,
            Ceiling,
            "0x3.ddE+7",
            Greater,
        ),
        (
            "0x9.cd3affe312d48E-268435507",
            "0x3",
            10,
            Ceiling,
            "0x8.ecE-268435507",
            Greater,
        ),
        (
            "0x4.11ed06daa99e79c4f938dd31feb41e6f2E+415297507",
            "0x3",
            100,
            Nearest,
            "0x3e78a608.47706167b63fcc332c",
            Greater,
        ),
        (
            "0xa.b8c5b0e499e0c33cd610861f56d389f76E-268435507",
            "0x3",
            100,
            Nearest,
            "0x9.c2663bf392c0b857ef11b7c5E-268435507",
            Greater,
        ),
        (
            "0x4.2b36adc73e9c50b0a9a088a5bdce14b06E+392325556",
            "0x3",
            100,
            Nearest,
            "0x3b04062a.c45670dd597b0526",
            Less,
        ),
        (
            "0x1.0dd45a8b6d6bacc4f068aed8edd08e2026E-268435506",
            "0x3",
            100,
            Nearest,
            "0xf.59c01bb16735648a6d51815E-268435507",
            Less,
        ),
        (
            "0x3.6f2e00b60a767bdf63f0654d75bafa6a21cE+421959829",
            "0x3",
            100,
            Nearest,
            "0x3f79350c.d5cd207a948f0f8d98",
            Greater,
        ),
        (
            "0xc.c88add630a14ce2256d98e128d581c950eE-268435507",
            "0x3",
            100,
            Nearest,
            "0xb.a2cbe7706c669aab616a8408E-268435507",
            Greater,
        ),
        (
            "0x5.075df1ae1d8a16be92c405932663055dca8E+399838660",
            "0x3",
            100,
            Nearest,
            "0x3c25586e.5bd7fb1de7fb8d66b8",
            Less,
        ),
        (
            "0xb.0077c8789a91d776fb14463dac6ffa49e1E-268435507",
            "0x3",
            100,
            Nearest,
            "0xa.03a8d94a07304b4d70c74e33E-268435507",
            Less,
        ),
        (
            "0xc.935cf3367cd063dc97de4c9d5f08431a45cE+391473587",
            "0x3",
            100,
            Nearest,
            "0x3ae33739.6836d2a94cbb679b84",
            Greater,
        ),
        (
            "0xe.5046205d2d55205b5df600645910c00cb7cE-268435507",
            "0x3",
            100,
            Nearest,
            "0xd.075da99e12e54bd7f6ab041bE-268435507",
            Greater,
        ),
        (
            "0x8.f497828cfc3850cf85f51c025cd3b46393E+397038291",
            "0x3",
            100,
            Nearest,
            "0x3bb9819e.6448901391b90636d",
            Less,
        ),
        (
            "0xf.d2ecd7dd7811ff98ce554da334d6e4dda4E-268435507",
            "0x3",
            100,
            Nearest,
            "0xe.674f9d3985095830a2028263E-268435507",
            Less,
        ),
        (
            "0x9.10e818abfae2d1d2bf2fbd1d387de40511b4E+405349653",
            "0x3",
            100,
            Nearest,
            "0x3cf99154.b787f0a600347c94bc",
            Greater,
        ),
        (
            "0xc.fad02a471042401d4f66da29b896dcc88928E-268435507",
            "0x3",
            100,
            Nearest,
            "0xb.d08e0bcaa723e5fc39f2488cE-268435507",
            Greater,
        ),
        (
            "0x2.1a2ce3f70bf87751a0526211c648467105a2E+392967422",
            "0x3",
            100,
            Nearest,
            "0x3b1cbddb.9295c50575715bf0fc",
            Less,
        ),
        (
            "0xb.a4ff5db432d3c47767ef35946c8de8b89f4E-268435507",
            "0x3",
            100,
            Nearest,
            "0xa.996bbed1b76cdd258b586136E-268435507",
            Less,
        ),
        (
            "0x2.2995d0d77e18e82e8ad40858c7ab589b2cE+408806686",
            "0x3",
            100,
            Floor,
            "0x3d7eb1b7.53cfb3b8452eaa4944",
            Less,
        ),
        (
            "0xe.9d618f48d361ba83ad2869987c7fc5dbE-268435507",
            "0x3",
            100,
            Floor,
            "0xd.4d8d44e0b98d81820b95da31E-268435507",
            Less,
        ),
        (
            "0x2.2995d0d77e18e82e8ad40858c7ab589b2cE+408806686",
            "0x3",
            100,
            Ceiling,
            "0x3d7eb1b7.53cfb3b8452eaa4948",
            Greater,
        ),
        (
            "0xe.9d618f48d361ba83ad2869987c7fc5dbE-268435507",
            "0x3",
            100,
            Ceiling,
            "0xd.4d8d44e0b98d81820b95da32E-268435507",
            Greater,
        ),
        (
            "0x2.5c8091903fa3dfc96f13345e000609d2bE+389827247",
            "0x3",
            100,
            Floor,
            "0x3aa3d124.529c92cc9e3a23582",
            Less,
        ),
        (
            "0xb.8c387abd326a04289c3ea92935e06cdd7E-268435507",
            "0x3",
            100,
            Floor,
            "0xa.82de338291221ebbb5a059caE-268435507",
            Less,
        ),
        (
            "0x2.5c8091903fa3dfc96f13345e000609d2bE+389827247",
            "0x3",
            100,
            Ceiling,
            "0x3aa3d124.529c92cc9e3a235824",
            Greater,
        ),
        (
            "0xb.8c387abd326a04289c3ea92935e06cdd7E-268435507",
            "0x3",
            100,
            Ceiling,
            "0xa.82de338291221ebbb5a059cbE-268435507",
            Greater,
        ),
        (
            "0x3.71b0ee0aa56d0f9bdd6d5e5aa35e5c5e29cE+419178695",
            "0x3",
            100,
            Floor,
            "0x3f0e1bdc.14a78093a929089bf4",
            Less,
        ),
        (
            "0xd.62897579afe2cdad484ededa1d4e8e7dffE-268435507",
            "0x3",
            100,
            Floor,
            "0xc.2ef7e547608292ed2d2c3663E-268435507",
            Less,
        ),
        (
            "0x3.71b0ee0aa56d0f9bdd6d5e5aa35e5c5e29cE+419178695",
            "0x3",
            100,
            Ceiling,
            "0x3f0e1bdc.14a78093a929089bf8",
            Greater,
        ),
        (
            "0xd.62897579afe2cdad484ededa1d4e8e7dffE-268435507",
            "0x3",
            100,
            Ceiling,
            "0xc.2ef7e547608292ed2d2c3664E-268435507",
            Greater,
        ),
        (
            "0x7.1943aa330363b432488d2e4d1b90f60af88E+391522291",
            "0x3",
            100,
            Floor,
            "0x3ae5175c.18f19eb94907be2b4",
            Less,
        ),
        (
            "0xe.50c1da2f987108b80e241c68af6684b621E-268435507",
            "0x3",
            100,
            Floor,
            "0xd.07ce485f673d13a2c9b7a90eE-268435507",
            Less,
        ),
        (
            "0x7.1943aa330363b432488d2e4d1b90f60af88E+391522291",
            "0x3",
            100,
            Ceiling,
            "0x3ae5175c.18f19eb94907be2b44",
            Greater,
        ),
        (
            "0xe.50c1da2f987108b80e241c68af6684b621E-268435507",
            "0x3",
            100,
            Ceiling,
            "0xd.07ce485f673d13a2c9b7a90fE-268435507",
            Greater,
        ),
        (
            "0xe.5fe168ad65322a252d74b36ccdbe1594064E+405548291",
            "0x3",
            100,
            Floor,
            "0x3d01378f.a24f9e2dc8143d4f4c",
            Less,
        ),
        (
            "0xb.439bed73cd3acfb0bdc68d477ec263b7468E-268435507",
            "0x3",
            100,
            Floor,
            "0xa.40c62c0f08cf69014ccd01dfE-268435507",
            Less,
        ),
        (
            "0xe.5fe168ad65322a252d74b36ccdbe1594064E+405548291",
            "0x3",
            100,
            Ceiling,
            "0x3d01378f.a24f9e2dc8143d4f5",
            Greater,
        ),
        (
            "0xb.439bed73cd3acfb0bdc68d477ec263b7468E-268435507",
            "0x3",
            100,
            Ceiling,
            "0xa.40c62c0f08cf69014ccd01eE-268435507",
            Greater,
        ),
        (
            "0x9.c8f37abaaabde9ad34694b2c9f5e739742E+420353704",
            "0x3",
            100,
            Floor,
            "0x3f3b5b6d.961de91bf1bbb8e5d8",
            Less,
        ),
        (
            "0xd.5e81a878dff6094658d555a97368b944abcE-268435507",
            "0x3",
            100,
            Floor,
            "0xc.2b4cb5c6c00e43a509b165f4E-268435507",
            Less,
        ),
        (
            "0x9.c8f37abaaabde9ad34694b2c9f5e739742E+420353704",
            "0x3",
            100,
            Ceiling,
            "0x3f3b5b6d.961de91bf1bbb8e5dc",
            Greater,
        ),
        (
            "0xd.5e81a878dff6094658d555a97368b944abcE-268435507",
            "0x3",
            100,
            Ceiling,
            "0xc.2b4cb5c6c00e43a509b165f5E-268435507",
            Greater,
        ),
        (
            "0x3.7a44d14c5739823a60416080a0caa6583615E+396114737",
            "0x3",
            100,
            Floor,
            "0x3b95f0f6.bd7dfdd17437ae4804",
            Less,
        ),
        (
            "0xb.49af61412521de42f94fa2dce7951f58c63cE-268435507",
            "0x3",
            100,
            Floor,
            "0xa.464e017f69271bd95eec7513E-268435507",
            Less,
        ),
        (
            "0x3.7a44d14c5739823a60416080a0caa6583615E+396114737",
            "0x3",
            100,
            Ceiling,
            "0x3b95f0f6.bd7dfdd17437ae4808",
            Greater,
        ),
        (
            "0xb.49af61412521de42f94fa2dce7951f58c63cE-268435507",
            "0x3",
            100,
            Ceiling,
            "0xa.464e017f69271bd95eec7514E-268435507",
            Greater,
        ),
        (
            "0xd.c2f54b4aebe0ddfe409bfa4e6e8f5d40df68E+388915663",
            "0x3",
            100,
            Floor,
            "0x3a80b680.0d9fa00b9d03e5d3b8",
            Less,
        ),
        (
            "0xf.cfcd56ccbd47477864d6a60b90a9f465cdccE-268435507",
            "0x3",
            100,
            Floor,
            "0xe.6477dfc59fdbd104934dd9eeE-268435507",
            Less,
        ),
        (
            "0xd.c2f54b4aebe0ddfe409bfa4e6e8f5d40df68E+388915663",
            "0x3",
            100,
            Ceiling,
            "0x3a80b680.0d9fa00b9d03e5d3bc",
            Greater,
        ),
        (
            "0xf.cfcd56ccbd47477864d6a60b90a9f465cdccE-268435507",
            "0x3",
            100,
            Ceiling,
            "0xe.6477dfc59fdbd104934dd9efE-268435507",
            Greater,
        ),
    ];
    for &(x, b, prec, rm, out, o) in cases {
        let r =
            log_base_big_exponent_float_base_1_plus_x_prec_round_all(&hex(x), &hex(b), prec, rm);
        assert_eq!(format!("{:?}", r.0), out, "{x} {b} {prec} {rm}");
        assert_eq!(r.1, o, "{x} {b} {prec} {rm}");
    }
}

#[test]
#[should_panic]
fn log_base_big_exponent_float_base_1_plus_x_prec_round_fail_1() {
    BigExponentFloat::from(5).log_base_big_exponent_float_base_1_plus_x_prec_round(
        &BigExponentFloat::from(3),
        0,
        Floor,
    );
}

#[test]
#[should_panic]
fn log_base_big_exponent_float_base_1_plus_x_prec_round_fail_2() {
    BigExponentFloat::from(core::f64::consts::PI)
        .log_base_big_exponent_float_base_1_plus_x_prec_round(
            &BigExponentFloat::from(3),
            10,
            Exact,
        );
}

fn log_base_big_exponent_float_base_1_plus_x_prec_round_properties_helper(
    x: &BigExponentFloat,
    b: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) {
    let r = log_base_big_exponent_float_base_1_plus_x_prec_round_all(x, b, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = log_base_big_exponent_float_base_1_plus_x_prec_round_all(x, b, prec, Floor);
        let ceiling = log_base_big_exponent_float_base_1_plus_x_prec_round_all(x, b, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(
                &log_base_big_exponent_float_base_1_plus_x_prec_round_all(x, b, prec, Exact),
                &r
            ));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) =
                log_base_big_exponent_float_base_1_plus_x_prec_round_all(x, b, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // log_b(1 + x) is log_b of the exact sum 1 + x.
    if x.is_finite() && exponent_is_moderate(x) {
        let z = x + BigExponentFloat::ONE;
        assert!(same_pair(
            &z.log_base_big_exponent_float_base_prec_round_ref(b, prec, rm),
            &r
        ));
    }
    // A base of 1 gives NaN, as in `arf_div`, since ln(1) = 0 and there is no signed zero.
    if *b == 1u32 {
        assert!(r.0.is_nan());
    }
}

#[test]
fn log_base_big_exponent_float_base_1_plus_x_prec_round_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_12()
        .test_properties(|(x, b, prec, rm)| {
            log_base_big_exponent_float_base_1_plus_x_prec_round_properties_helper(
                &x, &b, prec, rm,
            );
        });
}

#[test]
fn log_base_big_exponent_float_base_1_plus_x_prec_round_float_agreement_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_13().test_properties(
        |(x, b, prec, rm)| {
            if rm == Exact {
                return;
            }
            let (f, fo) = x.log_base_float_base_1_plus_x_prec_round_ref(&b, prec, rm);
            if float_result_comparable(&f, fo) && b != 1u32 {
                let r = log_base_big_exponent_float_base_1_plus_x_prec_round_all(
                    &float_to_big_exponent_float(&x),
                    &float_to_big_exponent_float(&b),
                    prec,
                    rm,
                );
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
