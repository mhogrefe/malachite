// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, FromStringBase};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_17,
    float_unsigned_rounding_mode_triple_gen_var_23,
    rational_unsigned_rounding_mode_triple_gen_var_7,
};
use malachite_float::{BigExponentFloat, Float};
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

fn log_base_2_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().log_base_2_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.log_base_2_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.log_base_2_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().log_base_2_prec(prec), &r));
        assert!(same_pair(&x.log_base_2_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.log_base_2_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn log_base_2_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::log_base_2_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::log_base_2_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::log_base_2_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::log_base_2_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_log_base_2_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    let one = BigExponentFloat::ONE;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &log_base_2_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_2_prec_round_all(&inf, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_2_prec_round_all(&ninf, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_2_prec_round_all(&zero, 10, rm),
            &(ninf.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_2_prec_round_all(&one, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_2_prec_round_all(&BigExponentFloat::NEGATIVE_ONE, 10, rm),
            &(nan.clone(), Equal)
        ));
    }
    // The expected values agree with MPFR's.
    let cases: &[(&str, u64, RoundingMode, &str, Ordering)] = &[
        ("0x3.243f6a8885a30", 1, Nearest, "0x2.0", Greater),
        ("0x3.243f6a8885a30", 10, Floor, "0x1.a68", Less),
        ("0x3.243f6a8885a30", 10, Ceiling, "0x1.a7", Greater),
        ("0x3.243f6a8885a30", 10, Down, "0x1.a68", Less),
        ("0x3.243f6a8885a30", 10, Up, "0x1.a7", Greater),
        ("0x3.243f6a8885a30", 10, Nearest, "0x1.a7", Greater),
        (
            "0x3.243f6a8885a30",
            100,
            Nearest,
            "0x1.a6c873498ddf71a36f477a776",
            Less,
        ),
        ("0x0.8", 1, Nearest, "-0x1.0", Equal),
        ("0x0.8", 10, Floor, "-0x1.0", Equal),
        ("0x0.8", 10, Ceiling, "-0x1.0", Equal),
        ("0x0.8", 10, Down, "-0x1.0", Equal),
        ("0x0.8", 10, Up, "-0x1.0", Equal),
        ("0x0.8", 10, Nearest, "-0x1.0", Equal),
        ("0x0.8", 100, Nearest, "-0x1.0", Equal),
        ("0x0.8", 300, Exact, "-0x1.0", Equal),
        ("0x3", 1, Nearest, "0x2.0", Greater),
        ("0x3", 10, Floor, "0x1.958", Less),
        ("0x3", 10, Ceiling, "0x1.96", Greater),
        ("0x3", 10, Down, "0x1.958", Less),
        ("0x3", 10, Up, "0x1.96", Greater),
        ("0x3", 10, Nearest, "0x1.96", Greater),
        ("0x3", 100, Nearest, "0x1.95c01a39fbd6879fa00b120a", Less),
        ("0x7", 1, Nearest, "0x2.0", Less),
        ("0x7", 10, Floor, "0x2.ce", Less),
        ("0x7", 10, Ceiling, "0x2.cf", Greater),
        ("0x7", 10, Down, "0x2.ce", Less),
        ("0x7", 10, Up, "0x2.cf", Greater),
        ("0x7", 10, Nearest, "0x2.cf", Greater),
        ("0x7", 100, Nearest, "0x2.ceaecfea80859b332ac903a4", Less),
        ("0x64", 1, Nearest, "0x8.0", Greater),
        ("0x64", 10, Floor, "0x6.a4", Less),
        ("0x64", 10, Ceiling, "0x6.a6", Greater),
        ("0x64", 10, Down, "0x6.a4", Less),
        ("0x64", 10, Up, "0x6.a6", Greater),
        ("0x64", 10, Nearest, "0x6.a4", Less),
        (
            "0x64",
            100,
            Nearest,
            "0x6.a4d3c25e68dc57f2495fb7fa8",
            Greater,
        ),
        ("0x3e7", 1, Nearest, "0x8.0", Less),
        ("0x3e7", 10, Floor, "0x9.f4", Less),
        ("0x3e7", 10, Ceiling, "0x9.f8", Greater),
        ("0x3e7", 10, Down, "0x9.f4", Less),
        ("0x3e7", 10, Up, "0x9.f8", Greater),
        ("0x3e7", 10, Nearest, "0x9.f8", Greater),
        ("0x3e7", 100, Nearest, "0x9.f6df0b095d5cbebea25ecd9e", Less),
        ("0x3e8", 1, Nearest, "0x8.0", Less),
        ("0x3e8", 10, Floor, "0x9.f4", Less),
        ("0x3e8", 10, Ceiling, "0x9.f8", Greater),
        ("0x3e8", 10, Down, "0x9.f4", Less),
        ("0x3e8", 10, Up, "0x9.f8", Greater),
        ("0x3e8", 10, Nearest, "0x9.f8", Greater),
        (
            "0x3e8",
            100,
            Nearest,
            "0x9.f73da38d9d4a83eb6e0f93f8",
            Greater,
        ),
        ("0x2540be400.0", 1, Nearest, "0x2.0E+1", Less),
        ("0x2540be400.0", 10, Floor, "0x21.3", Less),
        ("0x2540be400.0", 10, Ceiling, "0x21.4", Greater),
        ("0x2540be400.0", 10, Down, "0x21.3", Less),
        ("0x2540be400.0", 10, Up, "0x21.4", Greater),
        ("0x2540be400.0", 10, Nearest, "0x21.4", Greater),
        (
            "0x2540be400.0",
            100,
            Nearest,
            "0x21.3822cbd80c4db7bb6ede97e4",
            Less,
        ),
        ("0x1.0E+20", 1, Nearest, "0x4.0E+1", Less),
        ("0x1.0E+20", 10, Floor, "0x5.0E+1", Equal),
        ("0x1.0E+20", 10, Ceiling, "0x5.0E+1", Equal),
        ("0x1.0E+20", 10, Down, "0x5.0E+1", Equal),
        ("0x1.0E+20", 10, Up, "0x5.0E+1", Equal),
        ("0x1.0E+20", 10, Nearest, "0x5.0E+1", Equal),
        ("0x1.0E+20", 100, Nearest, "0x5.0E+1", Equal),
        ("0x1.0E+20", 300, Exact, "0x5.0E+1", Equal),
        ("0x2.0E+20", 1, Nearest, "0x4.0E+1", Less),
        ("0x2.0E+20", 10, Floor, "0x51.0", Equal),
        ("0x2.0E+20", 10, Ceiling, "0x51.0", Equal),
        ("0x2.0E+20", 10, Down, "0x51.0", Equal),
        ("0x2.0E+20", 10, Up, "0x51.0", Equal),
        ("0x2.0E+20", 10, Nearest, "0x51.0", Equal),
        ("0x2.0E+20", 100, Nearest, "0x51.0", Equal),
        ("0x2.0E+20", 300, Exact, "0x51.0", Equal),
        ("0x1.0E+300000000", 1, Nearest, "0x4.0E+7", Less),
        ("0x1.0E+300000000", 10, Floor, "0x4.78E+7", Less),
        ("0x1.0E+300000000", 10, Ceiling, "0x4.7aE+7", Greater),
        ("0x1.0E+300000000", 10, Down, "0x4.78E+7", Less),
        ("0x1.0E+300000000", 10, Up, "0x4.7aE+7", Greater),
        ("0x1.0E+300000000", 10, Nearest, "0x4.78E+7", Less),
        ("0x1.0E+300000000", 100, Nearest, "0x4.7868cE+7", Equal),
        ("0x1.0E+300000000", 300, Exact, "0x4.7868cE+7", Equal),
        ("0x1.8E+300000000", 1, Nearest, "0x4.0E+7", Less),
        ("0x1.8E+300000000", 10, Floor, "0x4.78E+7", Less),
        ("0x1.8E+300000000", 10, Ceiling, "0x4.7aE+7", Greater),
        ("0x1.8E+300000000", 10, Down, "0x4.78E+7", Less),
        ("0x1.8E+300000000", 10, Up, "0x4.7aE+7", Greater),
        ("0x1.8E+300000000", 10, Nearest, "0x4.78E+7", Less),
        (
            "0x1.8E+300000000",
            100,
            Nearest,
            "0x47868c00.95c01a39fbd6879fa",
            Less,
        ),
        ("0x1.0E-100", 1, Nearest, "-0x2.0E+2", Less),
        ("0x1.0E-100", 10, Floor, "-0x1.9E+2", Equal),
        ("0x1.0E-100", 10, Ceiling, "-0x1.9E+2", Equal),
        ("0x1.0E-100", 10, Down, "-0x1.9E+2", Equal),
        ("0x1.0E-100", 10, Up, "-0x1.9E+2", Equal),
        ("0x1.0E-100", 10, Nearest, "-0x1.9E+2", Equal),
        ("0x1.0E-100", 100, Nearest, "-0x1.9E+2", Equal),
        ("0x1.0E-100", 300, Exact, "-0x1.9E+2", Equal),
        ("0x1.8E-300000000", 1, Nearest, "-0x4.0E+7", Greater),
        ("0x1.8E-300000000", 10, Floor, "-0x4.7aE+7", Less),
        ("0x1.8E-300000000", 10, Ceiling, "-0x4.78E+7", Greater),
        ("0x1.8E-300000000", 10, Down, "-0x4.78E+7", Greater),
        ("0x1.8E-300000000", 10, Up, "-0x4.7aE+7", Less),
        ("0x1.8E-300000000", 10, Nearest, "-0x4.78E+7", Greater),
        (
            "0x1.8E-300000000",
            100,
            Nearest,
            "-0x47868bff.6a3fe5c6042978606",
            Less,
        ),
        ("-0x1.8E-300000000", 1, Nearest, "NaN", Equal),
        ("-0x1.8E-300000000", 10, Floor, "NaN", Equal),
        ("-0x1.8E-300000000", 10, Ceiling, "NaN", Equal),
        ("-0x1.8E-300000000", 10, Down, "NaN", Equal),
        ("-0x1.8E-300000000", 10, Up, "NaN", Equal),
        ("-0x1.8E-300000000", 10, Nearest, "NaN", Equal),
        ("-0x1.8E-300000000", 100, Nearest, "NaN", Equal),
        ("-0x1.8E-300000000", 300, Exact, "NaN", Equal),
        ("-0x0.8", 1, Nearest, "NaN", Equal),
        ("-0x0.8", 10, Floor, "NaN", Equal),
        ("-0x0.8", 10, Ceiling, "NaN", Equal),
        ("-0x0.8", 10, Down, "NaN", Equal),
        ("-0x0.8", 10, Up, "NaN", Equal),
        ("-0x0.8", 10, Nearest, "NaN", Equal),
        ("-0x0.8", 100, Nearest, "NaN", Equal),
        ("-0x0.8", 300, Exact, "NaN", Equal),
        ("-0x3", 1, Nearest, "NaN", Equal),
        ("-0x3", 10, Floor, "NaN", Equal),
        ("-0x3", 10, Ceiling, "NaN", Equal),
        ("-0x3", 10, Down, "NaN", Equal),
        ("-0x3", 10, Up, "NaN", Equal),
        ("-0x3", 10, Nearest, "NaN", Equal),
        ("-0x3", 100, Nearest, "NaN", Equal),
        ("-0x3", 300, Exact, "NaN", Equal),
        ("0x1.0", 1, Nearest, "0x0.0", Equal),
        ("0x1.0", 10, Floor, "0x0.0", Equal),
        ("0x1.0", 10, Ceiling, "0x0.0", Equal),
        ("0x1.0", 10, Down, "0x0.0", Equal),
        ("0x1.0", 10, Up, "0x0.0", Equal),
        ("0x1.0", 10, Nearest, "0x0.0", Equal),
        ("0x1.0", 100, Nearest, "0x0.0", Equal),
        ("0x1.0", 300, Exact, "0x0.0", Equal),
        ("0x0.ffffffff", 1, Nearest, "-0x1.0E-8", Greater),
        ("0x0.ffffffff", 10, Floor, "-0x1.718E-8", Less),
        ("0x0.ffffffff", 10, Ceiling, "-0x1.71E-8", Greater),
        ("0x0.ffffffff", 10, Down, "-0x1.71E-8", Greater),
        ("0x0.ffffffff", 10, Up, "-0x1.718E-8", Less),
        ("0x0.ffffffff", 10, Nearest, "-0x1.718E-8", Less),
        (
            "0x0.ffffffff",
            100,
            Nearest,
            "-0x1.7154765370da1ca1544415cdeE-8",
            Less,
        ),
    ];
    for &(x, prec, rm, out, o) in cases {
        let r = log_base_2_prec_round_all(&hex(x), prec, rm);
        assert_eq!(format!("{:?}", r.0), out, "{x} {prec} {rm}");
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    }
}

#[test]
#[should_panic]
fn log_base_2_prec_round_fail_1() {
    BigExponentFloat::from(3).log_base_2_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn log_base_2_prec_round_fail_2() {
    BigExponentFloat::from(core::f64::consts::PI).log_base_2_prec_round(10, Exact);
}

#[test]
fn test_log_base_2_rational_prec_round() {
    // The expected values agree with MPFR's.
    let cases: &[(&str, u64, RoundingMode, &str, Ordering)] = &[
        ("3/5", 1, Nearest, "-0x0.8", Greater),
        ("3/5", 10, Floor, "-0x0.bcc", Less),
        ("3/5", 10, Ceiling, "-0x0.bc8", Greater),
        ("3/5", 10, Down, "-0x0.bc8", Greater),
        ("3/5", 10, Up, "-0x0.bcc", Less),
        ("3/5", 10, Nearest, "-0x0.bcc", Less),
        (
            "3/5",
            100,
            Nearest,
            "-0x0.bca9c6f53897a45984a4c9f33",
            Greater,
        ),
        ("1/3", 1, Nearest, "-0x2.0", Less),
        ("1/3", 10, Floor, "-0x1.96", Less),
        ("1/3", 10, Ceiling, "-0x1.958", Greater),
        ("1/3", 10, Down, "-0x1.958", Greater),
        ("1/3", 10, Up, "-0x1.96", Less),
        ("1/3", 10, Nearest, "-0x1.96", Less),
        (
            "1/3",
            100,
            Nearest,
            "-0x1.95c01a39fbd6879fa00b120a",
            Greater,
        ),
        ("100", 1, Nearest, "0x8.0", Greater),
        ("100", 10, Floor, "0x6.a4", Less),
        ("100", 10, Ceiling, "0x6.a6", Greater),
        ("100", 10, Down, "0x6.a4", Less),
        ("100", 10, Up, "0x6.a6", Greater),
        ("100", 10, Nearest, "0x6.a4", Less),
        (
            "100",
            100,
            Nearest,
            "0x6.a4d3c25e68dc57f2495fb7fa8",
            Greater,
        ),
        ("1000", 1, Nearest, "0x8.0", Less),
        ("1000", 10, Floor, "0x9.f4", Less),
        ("1000", 10, Ceiling, "0x9.f8", Greater),
        ("1000", 10, Down, "0x9.f4", Less),
        ("1000", 10, Up, "0x9.f8", Greater),
        ("1000", 10, Nearest, "0x9.f8", Greater),
        (
            "1000",
            100,
            Nearest,
            "0x9.f73da38d9d4a83eb6e0f93f8",
            Greater,
        ),
        ("1/1000", 1, Nearest, "-0x8.0", Greater),
        ("1/1000", 10, Floor, "-0x9.f8", Less),
        ("1/1000", 10, Ceiling, "-0x9.f4", Greater),
        ("1/1000", 10, Down, "-0x9.f4", Greater),
        ("1/1000", 10, Up, "-0x9.f8", Less),
        ("1/1000", 10, Nearest, "-0x9.f8", Less),
        (
            "1/1000",
            100,
            Nearest,
            "-0x9.f73da38d9d4a83eb6e0f93f8",
            Less,
        ),
        ("10000000000/3", 1, Nearest, "0x2.0E+1", Greater),
        ("10000000000/3", 10, Floor, "0x1f.a", Less),
        ("10000000000/3", 10, Ceiling, "0x1f.a8", Greater),
        ("10000000000/3", 10, Down, "0x1f.a", Less),
        ("10000000000/3", 10, Up, "0x1f.a8", Greater),
        ("10000000000/3", 10, Nearest, "0x1f.a", Less),
        (
            "10000000000/3",
            100,
            Nearest,
            "0x1f.a262b19e1077301bced385da",
            Less,
        ),
        ("1", 1, Nearest, "0x0.0", Equal),
        ("1", 10, Floor, "0x0.0", Equal),
        ("1", 10, Ceiling, "0x0.0", Equal),
        ("1", 10, Down, "0x0.0", Equal),
        ("1", 10, Up, "0x0.0", Equal),
        ("1", 10, Nearest, "0x0.0", Equal),
        ("1", 100, Nearest, "0x0.0", Equal),
        ("1", 300, Exact, "0x0.0", Equal),
        ("0", 1, Nearest, "-Infinity", Equal),
        ("0", 10, Floor, "-Infinity", Equal),
        ("0", 10, Ceiling, "-Infinity", Equal),
        ("0", 10, Down, "-Infinity", Equal),
        ("0", 10, Up, "-Infinity", Equal),
        ("0", 10, Nearest, "-Infinity", Equal),
        ("0", 100, Nearest, "-Infinity", Equal),
        ("0", 300, Exact, "-Infinity", Equal),
        ("-1", 1, Nearest, "NaN", Equal),
        ("-1", 10, Floor, "NaN", Equal),
        ("-1", 10, Ceiling, "NaN", Equal),
        ("-1", 10, Down, "NaN", Equal),
        ("-1", 10, Up, "NaN", Equal),
        ("-1", 10, Nearest, "NaN", Equal),
        ("-1", 100, Nearest, "NaN", Equal),
        ("-1", 300, Exact, "NaN", Equal),
    ];
    for &(x, prec, rm, out, o) in cases {
        let r = log_base_2_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out, "{x} {prec} {rm}");
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    }
}

#[test]
#[should_panic]
fn log_base_2_rational_prec_round_fail() {
    BigExponentFloat::log_base_2_rational_prec_round(Rational::from_unsigneds(1u32, 3), 10, Exact);
}

fn log_base_2_prec_round_properties_helper(x: &BigExponentFloat, prec: u64, rm: RoundingMode) {
    let r = log_base_2_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = log_base_2_prec_round_all(x, prec, Floor);
        let ceiling = log_base_2_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(&log_base_2_prec_round_all(x, prec, Exact), &r));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = log_base_2_prec_round_all(x, prec + 16, rm);
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
            &log_base_2_rational_prec_round_all(&q, prec, rm),
            &r
        ));
    }
}

#[test]
fn log_base_2_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_17().test_properties(
        |(x, prec, rm)| {
            log_base_2_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn log_base_2_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_23().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let (f, fo) = x.log_base_2_prec_round_ref(prec, rm);
        if float_result_comparable(&f, fo) {
            let r = log_base_2_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}

#[test]
fn log_base_2_rational_prec_round_float_agreement_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_7().test_properties(|(x, prec, rm)| {
        let (f, fo) = Float::log_base_2_rational_prec_round_ref(&x, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = log_base_2_rational_prec_round_all(&x, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
