// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, FromStringBase};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_power_of_2, float_to_big_exponent_float,
};
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_11, float_unsigned_pair_gen_var_1,
    rational_unsigned_rounding_mode_triple_gen_var_21,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

fn same_pair(x: &(BigExponentFloat, Ordering), y: &(BigExponentFloat, Ordering)) -> bool {
    same(&x.0, &y.0) && x.1 == y.1
}

fn float_result_comparable(x: &Float, o: Ordering) -> bool {
    x.get_exponent().map_or_else(
        || !x.is_infinite() && (!x.is_zero() || o == Equal),
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

fn power_of_2_of_big_exponent_float_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::power_of_2_of_big_exponent_float_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::power_of_2_of_big_exponent_float_prec_round_ref(x, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.power_of_2_of_big_exponent_float_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::power_of_2_of_big_exponent_float_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::power_of_2_of_big_exponent_float_prec_ref(x, prec),
            &r
        ));
        let mut z = x.clone();
        let o = z.power_of_2_of_big_exponent_float_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn power_of_2_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::power_of_2_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::power_of_2_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::power_of_2_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::power_of_2_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_power_of_2_of_big_exponent_float_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &power_of_2_of_big_exponent_float_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &power_of_2_of_big_exponent_float_prec_round_all(&inf, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &power_of_2_of_big_exponent_float_prec_round_all(&ninf, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &power_of_2_of_big_exponent_float_prec_round_all(&zero, 10, rm),
            &(BigExponentFloat::ONE, Equal)
        ));
    }
    // The expected values agree with MPFR's wherever MPFR's exponent range allows.
    let cases: &[(&str, u64, RoundingMode, &str, Ordering)] = &[
        ("0x3.243f6a8885a30", 1, Nearest, "0x8.0", Less),
        ("0x3.243f6a8885a30", 10, Floor, "0x8.d", Less),
        ("0x3.243f6a8885a30", 10, Ceiling, "0x8.d4", Greater),
        ("0x3.243f6a8885a30", 10, Down, "0x8.d", Less),
        ("0x3.243f6a8885a30", 10, Up, "0x8.d4", Greater),
        ("0x3.243f6a8885a30", 10, Nearest, "0x8.d4", Greater),
        (
            "0x3.243f6a8885a30",
            100,
            Nearest,
            "0x8.d331bf3337c170dfcdee149f",
            Less,
        ),
        ("-0x3.243f6a8885a30", 1, Nearest, "0x0.2", Greater),
        ("-0x3.243f6a8885a30", 10, Floor, "0x0.1d", Less),
        ("-0x3.243f6a8885a30", 10, Ceiling, "0x0.1d08", Greater),
        ("-0x3.243f6a8885a30", 10, Down, "0x0.1d", Less),
        ("-0x3.243f6a8885a30", 10, Up, "0x0.1d08", Greater),
        ("-0x3.243f6a8885a30", 10, Nearest, "0x0.1d", Less),
        (
            "-0x3.243f6a8885a30",
            100,
            Nearest,
            "0x0.1d0231bd5e9cfd1c56d8c57fba",
            Greater,
        ),
        ("0x0.8", 1, Nearest, "0x1.0", Less),
        ("0x0.8", 10, Floor, "0x1.6a", Less),
        ("0x0.8", 10, Ceiling, "0x1.6a8", Greater),
        ("0x0.8", 10, Down, "0x1.6a", Less),
        ("0x0.8", 10, Up, "0x1.6a8", Greater),
        ("0x0.8", 10, Nearest, "0x1.6a", Less),
        ("0x0.8", 100, Nearest, "0x1.6a09e667f3bcc908b2fb1366e", Less),
        ("-0x0.8", 1, Nearest, "0x0.8", Less),
        ("-0x0.8", 10, Floor, "0x0.b5", Less),
        ("-0x0.8", 10, Ceiling, "0x0.b54", Greater),
        ("-0x0.8", 10, Down, "0x0.b5", Less),
        ("-0x0.8", 10, Up, "0x0.b54", Greater),
        ("-0x0.8", 10, Nearest, "0x0.b5", Less),
        (
            "-0x0.8",
            100,
            Nearest,
            "0x0.b504f333f9de6484597d89b37",
            Less,
        ),
        ("0x3", 1, Nearest, "0x8.0", Equal),
        ("0x3", 10, Floor, "0x8.0", Equal),
        ("0x3", 10, Ceiling, "0x8.0", Equal),
        ("0x3", 10, Down, "0x8.0", Equal),
        ("0x3", 10, Up, "0x8.0", Equal),
        ("0x3", 10, Nearest, "0x8.0", Equal),
        ("0x3", 100, Nearest, "0x8.0", Equal),
        ("-0x3", 1, Nearest, "0x0.2", Equal),
        ("-0x3", 10, Floor, "0x0.2", Equal),
        ("-0x3", 10, Ceiling, "0x0.2", Equal),
        ("-0x3", 10, Down, "0x0.2", Equal),
        ("-0x3", 10, Up, "0x0.2", Equal),
        ("-0x3", 10, Nearest, "0x0.2", Equal),
        ("-0x3", 100, Nearest, "0x0.2", Equal),
        ("0x64", 1, Nearest, "0x1.0E+25", Equal),
        ("0x64", 10, Floor, "0x1.0E+25", Equal),
        ("0x64", 10, Ceiling, "0x1.0E+25", Equal),
        ("0x64", 10, Down, "0x1.0E+25", Equal),
        ("0x64", 10, Up, "0x1.0E+25", Equal),
        ("0x64", 10, Nearest, "0x1.0E+25", Equal),
        ("0x64", 100, Nearest, "0x1.0E+25", Equal),
        ("-0x64", 1, Nearest, "0x1.0E-25", Equal),
        ("-0x64", 10, Floor, "0x1.0E-25", Equal),
        ("-0x64", 10, Ceiling, "0x1.0E-25", Equal),
        ("-0x64", 10, Down, "0x1.0E-25", Equal),
        ("-0x64", 10, Up, "0x1.0E-25", Equal),
        ("-0x64", 10, Nearest, "0x1.0E-25", Equal),
        ("-0x64", 100, Nearest, "0x1.0E-25", Equal),
        ("0x2540be400.0", 1, Nearest, "0x1.0E+2500000000", Equal),
        ("0x2540be400.0", 10, Floor, "0x1.0E+2500000000", Equal),
        ("0x2540be400.0", 10, Ceiling, "0x1.0E+2500000000", Equal),
        ("0x2540be400.0", 10, Down, "0x1.0E+2500000000", Equal),
        ("0x2540be400.0", 10, Up, "0x1.0E+2500000000", Equal),
        ("0x2540be400.0", 10, Nearest, "0x1.0E+2500000000", Equal),
        ("0x2540be400.0", 100, Nearest, "0x1.0E+2500000000", Equal),
        ("-0x2540be400.0", 1, Nearest, "0x1.0E-2500000000", Equal),
        ("-0x2540be400.0", 10, Floor, "0x1.0E-2500000000", Equal),
        ("-0x2540be400.0", 10, Ceiling, "0x1.0E-2500000000", Equal),
        ("-0x2540be400.0", 10, Down, "0x1.0E-2500000000", Equal),
        ("-0x2540be400.0", 10, Up, "0x1.0E-2500000000", Equal),
        ("-0x2540be400.0", 10, Nearest, "0x1.0E-2500000000", Equal),
        ("-0x2540be400.0", 100, Nearest, "0x1.0E-2500000000", Equal),
        (
            "0x1.0E+20",
            1,
            Nearest,
            "0x1.0E+302231454903657293676544",
            Equal,
        ),
        (
            "0x1.0E+20",
            10,
            Floor,
            "0x1.0E+302231454903657293676544",
            Equal,
        ),
        (
            "0x1.0E+20",
            10,
            Ceiling,
            "0x1.0E+302231454903657293676544",
            Equal,
        ),
        (
            "0x1.0E+20",
            10,
            Down,
            "0x1.0E+302231454903657293676544",
            Equal,
        ),
        (
            "0x1.0E+20",
            10,
            Up,
            "0x1.0E+302231454903657293676544",
            Equal,
        ),
        (
            "0x1.0E+20",
            10,
            Nearest,
            "0x1.0E+302231454903657293676544",
            Equal,
        ),
        (
            "0x1.0E+20",
            100,
            Nearest,
            "0x1.0E+302231454903657293676544",
            Equal,
        ),
        (
            "-0x1.0E+20",
            1,
            Nearest,
            "0x1.0E-302231454903657293676544",
            Equal,
        ),
        (
            "-0x1.0E+20",
            10,
            Floor,
            "0x1.0E-302231454903657293676544",
            Equal,
        ),
        (
            "-0x1.0E+20",
            10,
            Ceiling,
            "0x1.0E-302231454903657293676544",
            Equal,
        ),
        (
            "-0x1.0E+20",
            10,
            Down,
            "0x1.0E-302231454903657293676544",
            Equal,
        ),
        (
            "-0x1.0E+20",
            10,
            Up,
            "0x1.0E-302231454903657293676544",
            Equal,
        ),
        (
            "-0x1.0E+20",
            10,
            Nearest,
            "0x1.0E-302231454903657293676544",
            Equal,
        ),
        (
            "-0x1.0E+20",
            100,
            Nearest,
            "0x1.0E-302231454903657293676544",
            Equal,
        ),
        ("0x1.0E-100", 1, Nearest, "0x1.0", Less),
        ("0x1.0E-100", 10, Floor, "0x1.0", Less),
        ("0x1.0E-100", 10, Ceiling, "0x1.008", Greater),
        ("0x1.0E-100", 10, Down, "0x1.0", Less),
        ("0x1.0E-100", 10, Up, "0x1.008", Greater),
        ("0x1.0E-100", 10, Nearest, "0x1.0", Less),
        ("0x1.0E-100", 100, Nearest, "0x1.0", Less),
        ("-0x1.0E-100", 1, Nearest, "0x1.0", Greater),
        ("-0x1.0E-100", 10, Floor, "0x0.ffc", Less),
        ("-0x1.0E-100", 10, Ceiling, "0x1.0", Greater),
        ("-0x1.0E-100", 10, Down, "0x0.ffc", Less),
        ("-0x1.0E-100", 10, Up, "0x1.0", Greater),
        ("-0x1.0E-100", 10, Nearest, "0x1.0", Greater),
        ("-0x1.0E-100", 100, Nearest, "0x1.0", Greater),
        ("0x1.8E-300000000", 1, Nearest, "0x1.0", Less),
        ("0x1.8E-300000000", 10, Floor, "0x1.0", Less),
        ("0x1.8E-300000000", 10, Ceiling, "0x1.008", Greater),
        ("0x1.8E-300000000", 10, Down, "0x1.0", Less),
        ("0x1.8E-300000000", 10, Up, "0x1.008", Greater),
        ("0x1.8E-300000000", 10, Nearest, "0x1.0", Less),
        ("0x1.8E-300000000", 100, Nearest, "0x1.0", Less),
        ("-0x1.8E-300000000", 1, Nearest, "0x1.0", Greater),
        ("-0x1.8E-300000000", 10, Floor, "0x0.ffc", Less),
        ("-0x1.8E-300000000", 10, Ceiling, "0x1.0", Greater),
        ("-0x1.8E-300000000", 10, Down, "0x0.ffc", Less),
        ("-0x1.8E-300000000", 10, Up, "0x1.0", Greater),
        ("-0x1.8E-300000000", 10, Nearest, "0x1.0", Greater),
        ("-0x1.8E-300000000", 100, Nearest, "0x1.0", Greater),
        ("0x5.0E+7", 1, Nearest, "0x1.0E+335544320", Equal),
        ("0x5.0E+7", 10, Floor, "0x1.0E+335544320", Equal),
        ("0x5.0E+7", 10, Ceiling, "0x1.0E+335544320", Equal),
        ("0x5.0E+7", 10, Down, "0x1.0E+335544320", Equal),
        ("0x5.0E+7", 10, Up, "0x1.0E+335544320", Equal),
        ("0x5.0E+7", 10, Nearest, "0x1.0E+335544320", Equal),
        ("0x5.0E+7", 100, Nearest, "0x1.0E+335544320", Equal),
        ("-0x5.0E+7", 1, Nearest, "0x1.0E-335544320", Equal),
        ("-0x5.0E+7", 10, Floor, "0x1.0E-335544320", Equal),
        ("-0x5.0E+7", 10, Ceiling, "0x1.0E-335544320", Equal),
        ("-0x5.0E+7", 10, Down, "0x1.0E-335544320", Equal),
        ("-0x5.0E+7", 10, Up, "0x1.0E-335544320", Equal),
        ("-0x5.0E+7", 10, Nearest, "0x1.0E-335544320", Equal),
        ("-0x5.0E+7", 100, Nearest, "0x1.0E-335544320", Equal),
        ("0x3", 300, Exact, "0x8.0", Equal),
        ("-0x3", 300, Exact, "0x0.2", Equal),
        ("0x64", 300, Exact, "0x1.0E+25", Equal),
        ("-0x64", 300, Exact, "0x1.0E-25", Equal),
    ];
    for &(x, prec, rm, out, o) in cases {
        let r = power_of_2_of_big_exponent_float_prec_round_all(&hex(x), prec, rm);
        assert_eq!(format!("{:?}", r.0), out, "{x} {prec} {rm}");
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    }
}

#[test]
fn test_power_of_2_rational_prec_round() {
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &power_of_2_rational_prec_round_all(&Rational::ZERO, 10, rm),
            &(BigExponentFloat::ONE, Equal)
        ));
    }
    // The expected values agree with MPFR's wherever MPFR's exponent range allows.
    let cases: &[(&str, u64, RoundingMode, &str, Ordering)] = &[
        ("1/3", 1, Nearest, "0x1.0", Less),
        ("1/3", 10, Floor, "0x1.428", Less),
        ("1/3", 10, Ceiling, "0x1.43", Greater),
        ("1/3", 10, Down, "0x1.428", Less),
        ("1/3", 10, Up, "0x1.43", Greater),
        ("1/3", 10, Nearest, "0x1.428", Less),
        (
            "1/3",
            100,
            Nearest,
            "0x1.428a2f98d728ae223ddab715c",
            Greater,
        ),
        ("-1/3", 1, Nearest, "0x1.0", Greater),
        ("-1/3", 10, Floor, "0x0.cb", Less),
        ("-1/3", 10, Ceiling, "0x0.cb4", Greater),
        ("-1/3", 10, Down, "0x0.cb", Less),
        ("-1/3", 10, Up, "0x0.cb4", Greater),
        ("-1/3", 10, Nearest, "0x0.cb4", Greater),
        (
            "-1/3",
            100,
            Nearest,
            "0x0.cb2ff529eb71e41582cccd5a2",
            Greater,
        ),
        ("3/5", 1, Nearest, "0x2.0", Greater),
        ("3/5", 10, Floor, "0x1.84", Less),
        ("3/5", 10, Ceiling, "0x1.848", Greater),
        ("3/5", 10, Down, "0x1.84", Less),
        ("3/5", 10, Up, "0x1.848", Greater),
        ("3/5", 10, Nearest, "0x1.84", Less),
        ("3/5", 100, Nearest, "0x1.8406003b2ae5c6b99347320ea", Less),
        ("3", 1, Nearest, "0x8.0", Equal),
        ("3", 10, Floor, "0x8.0", Equal),
        ("3", 10, Ceiling, "0x8.0", Equal),
        ("3", 10, Down, "0x8.0", Equal),
        ("3", 10, Up, "0x8.0", Equal),
        ("3", 10, Nearest, "0x8.0", Equal),
        ("3", 100, Nearest, "0x8.0", Equal),
        ("-3", 1, Nearest, "0x0.2", Equal),
        ("-3", 10, Floor, "0x0.2", Equal),
        ("-3", 10, Ceiling, "0x0.2", Equal),
        ("-3", 10, Down, "0x0.2", Equal),
        ("-3", 10, Up, "0x0.2", Equal),
        ("-3", 10, Nearest, "0x0.2", Equal),
        ("-3", 100, Nearest, "0x0.2", Equal),
        ("100", 1, Nearest, "0x1.0E+25", Equal),
        ("100", 10, Floor, "0x1.0E+25", Equal),
        ("100", 10, Ceiling, "0x1.0E+25", Equal),
        ("100", 10, Down, "0x1.0E+25", Equal),
        ("100", 10, Up, "0x1.0E+25", Equal),
        ("100", 10, Nearest, "0x1.0E+25", Equal),
        ("100", 100, Nearest, "0x1.0E+25", Equal),
        ("-100", 1, Nearest, "0x1.0E-25", Equal),
        ("-100", 10, Floor, "0x1.0E-25", Equal),
        ("-100", 10, Ceiling, "0x1.0E-25", Equal),
        ("-100", 10, Down, "0x1.0E-25", Equal),
        ("-100", 10, Up, "0x1.0E-25", Equal),
        ("-100", 10, Nearest, "0x1.0E-25", Equal),
        ("-100", 100, Nearest, "0x1.0E-25", Equal),
        ("10000000000/3", 1, Nearest, "0x2.0E+833333333", Less),
        ("10000000000/3", 10, Floor, "0x2.85E+833333333", Less),
        ("10000000000/3", 10, Ceiling, "0x2.86E+833333333", Greater),
        ("10000000000/3", 10, Down, "0x2.85E+833333333", Less),
        ("10000000000/3", 10, Up, "0x2.86E+833333333", Greater),
        ("10000000000/3", 10, Nearest, "0x2.85E+833333333", Less),
        (
            "10000000000/3",
            100,
            Nearest,
            "0x2.85145f31ae515c447bb56e2b8E+833333333",
            Greater,
        ),
        ("-10000000000/3", 1, Nearest, "0x8.0E-833333334", Greater),
        ("-10000000000/3", 10, Floor, "0x6.58E-833333334", Less),
        ("-10000000000/3", 10, Ceiling, "0x6.5aE-833333334", Greater),
        ("-10000000000/3", 10, Down, "0x6.58E-833333334", Less),
        ("-10000000000/3", 10, Up, "0x6.5aE-833333334", Greater),
        ("-10000000000/3", 10, Nearest, "0x6.5aE-833333334", Greater),
        (
            "-10000000000/3",
            100,
            Nearest,
            "0x6.597fa94f5b8f20ac16666ad1E-833333334",
            Greater,
        ),
        ("1/10000000000", 1, Nearest, "0x1.0", Less),
        ("1/10000000000", 10, Floor, "0x1.0", Less),
        ("1/10000000000", 10, Ceiling, "0x1.008", Greater),
        ("1/10000000000", 10, Down, "0x1.0", Less),
        ("1/10000000000", 10, Up, "0x1.008", Greater),
        ("1/10000000000", 10, Nearest, "0x1.0", Less),
        (
            "1/10000000000",
            100,
            Nearest,
            "0x1.000000004c365bd0908848ce8",
            Greater,
        ),
        ("-1/10000000000", 1, Nearest, "0x1.0", Greater),
        ("-1/10000000000", 10, Floor, "0x0.ffc", Less),
        ("-1/10000000000", 10, Ceiling, "0x1.0", Greater),
        ("-1/10000000000", 10, Down, "0x0.ffc", Less),
        ("-1/10000000000", 10, Up, "0x1.0", Greater),
        ("-1/10000000000", 10, Nearest, "0x1.0", Greater),
        (
            "-1/10000000000",
            100,
            Nearest,
            "0x0.ffffffffb3c9a42f862809403",
            Less,
        ),
        (
            "1208925819614629174706176/3",
            1,
            Nearest,
            "0x2.0E+100743818301219097892181",
            Less,
        ),
        (
            "1208925819614629174706176/3",
            10,
            Floor,
            "0x2.85E+100743818301219097892181",
            Less,
        ),
        (
            "1208925819614629174706176/3",
            10,
            Ceiling,
            "0x2.86E+100743818301219097892181",
            Greater,
        ),
        (
            "1208925819614629174706176/3",
            10,
            Down,
            "0x2.85E+100743818301219097892181",
            Less,
        ),
        (
            "1208925819614629174706176/3",
            10,
            Up,
            "0x2.86E+100743818301219097892181",
            Greater,
        ),
        (
            "1208925819614629174706176/3",
            10,
            Nearest,
            "0x2.85E+100743818301219097892181",
            Less,
        ),
        (
            "1208925819614629174706176/3",
            100,
            Nearest,
            "0x2.85145f31ae515c447bb56e2b8E+100743818301219097892181",
            Greater,
        ),
        ("3", 300, Exact, "0x8.0", Equal),
        ("-3", 300, Exact, "0x0.2", Equal),
        ("100", 300, Exact, "0x1.0E+25", Equal),
        ("-100", 300, Exact, "0x1.0E-25", Equal),
    ];
    for &(x, prec, rm, out, o) in cases {
        let r = power_of_2_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out, "{x} {prec} {rm}");
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    }
}

#[test]
#[should_panic]
fn power_of_2_of_big_exponent_float_prec_round_fail_1() {
    BigExponentFloat::power_of_2_of_big_exponent_float_prec_round(BigExponentFloat::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn power_of_2_of_big_exponent_float_prec_round_fail_2() {
    BigExponentFloat::power_of_2_of_big_exponent_float_prec_round(
        BigExponentFloat::from(0.5),
        10,
        Exact,
    );
}

#[test]
#[should_panic]
fn power_of_2_rational_prec_round_fail_1() {
    BigExponentFloat::power_of_2_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn power_of_2_rational_prec_round_fail_2() {
    BigExponentFloat::power_of_2_rational_prec_round(Rational::from_unsigneds(1u32, 3), 10, Exact);
}

fn power_of_2_of_big_exponent_float_prec_round_properties_helper(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) {
    let r = power_of_2_of_big_exponent_float_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if !r.0.is_nan() {
        assert!(r.0 >= 0u32);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else {
        // The result lies between the results rounded down and up.
        let floor = power_of_2_of_big_exponent_float_prec_round_all(x, prec, Floor);
        let ceiling = power_of_2_of_big_exponent_float_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0 || r.0.is_nan());
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(
                &power_of_2_of_big_exponent_float_prec_round_all(x, prec, Exact),
                &r
            ));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = power_of_2_of_big_exponent_float_prec_round_all(x, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // A `BigExponentFloat` that is not too large or too small gives the same result as the equal
    // `Rational`.
    if x.is_finite() && exponent_is_moderate(x) {
        let q = Rational::exact_from(x);
        assert!(same_pair(
            &power_of_2_rational_prec_round_all(&q, prec, rm),
            &r
        ));
    }
    // At an integer n, the exact value is a `Rational`.
    if let Ok(n) = Integer::try_from(x)
        && n.unsigned_abs_ref() <= &1000u32
    {
        let n = i64::exact_from(&n);
        let exact = Rational::power_of_2(n);
        assert!(same_pair(
            &BigExponentFloat::from_rational_prec_round(exact, prec, rm),
            &r
        ));
    }
    // 2^(x + k) = 2^x * 2^k for any integer k, even beyond `Float`'s exponent range.
    if x.is_finite() && exponent_is_moderate(x) {
        for k in [
            Integer::ONE,
            Integer::NEGATIVE_ONE,
            Integer::from_str("100000000000000000000").unwrap(),
        ] {
            let s = power_of_2_of_big_exponent_float_prec_round_all(
                &(x + BigExponentFloat::from(k.clone())),
                prec,
                rm,
            );
            assert!(same_pair(
                &s,
                &(&r.0 * big_exponent_float_power_of_2(k), r.1)
            ));
        }
    }
}

#[test]
fn power_of_2_of_big_exponent_float_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_11().test_properties(
        |(x, prec, rm)| {
            power_of_2_of_big_exponent_float_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn power_of_2_of_big_exponent_float_prec_round_float_agreement_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        for rm in exhaustive_rounding_modes() {
            if rm == Exact {
                continue;
            }
            let x = &x;
            let (f, fo) = Float::power_of_2_of_float_prec_round_ref(x, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = power_of_2_of_big_exponent_float_prec_round_all(
                    &float_to_big_exponent_float(x),
                    prec,
                    rm,
                );
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        }
    });
}

#[test]
fn power_of_2_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_21().test_properties(|(x, prec, rm)| {
        let r = power_of_2_rational_prec_round_all(&x, prec, rm);
        if r.0.is_zero() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        if rm == Exact {
            assert_eq!(r.1, Equal);
        } else if rm != Nearest {
            let (y, o) = power_of_2_rational_prec_round_all(&x, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    });
}

#[test]
fn power_of_2_rational_prec_round_float_agreement_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_21().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let x = &x;
        let (f, fo) = Float::power_of_2_rational_prec_round_ref(x, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = power_of_2_rational_prec_round_all(x, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
