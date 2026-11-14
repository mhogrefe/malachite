// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, One, Two, Zero,
};
use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_18,
    float_unsigned_rounding_mode_triple_gen_var_25,
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

fn log_base_2_1_plus_x_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().log_base_2_1_plus_x_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.log_base_2_1_plus_x_prec_round_ref(prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.log_base_2_1_plus_x_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().log_base_2_1_plus_x_prec(prec), &r));
        assert!(same_pair(&x.log_base_2_1_plus_x_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.log_base_2_1_plus_x_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

#[test]
fn test_log_base_2_1_plus_x_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &log_base_2_1_plus_x_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_2_1_plus_x_prec_round_all(&inf, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_2_1_plus_x_prec_round_all(&ninf, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_2_1_plus_x_prec_round_all(&zero, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_2_1_plus_x_prec_round_all(&BigExponentFloat::NEGATIVE_ONE, 10, rm),
            &(ninf.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_2_1_plus_x_prec_round_all(&-BigExponentFloat::TWO, 10, rm),
            &(nan.clone(), Equal)
        ));
    }
    // The expected values agree with MPFR's.
    let cases: &[(&str, u64, RoundingMode, &str, Ordering)] = &[
        ("0x3.243f6a8885a30", 1, Nearest, "0x2.0", Less),
        ("0x3.243f6a8885a30", 10, Floor, "0x2.0c", Less),
        ("0x3.243f6a8885a30", 10, Ceiling, "0x2.0d", Greater),
        ("0x3.243f6a8885a30", 10, Down, "0x2.0c", Less),
        ("0x3.243f6a8885a30", 10, Up, "0x2.0d", Greater),
        ("0x3.243f6a8885a30", 10, Nearest, "0x2.0d", Greater),
        (
            "0x3.243f6a8885a30",
            100,
            Nearest,
            "0x2.0cd8f7baa695a2146d5d1f398",
            Less,
        ),
        ("0x0.8", 1, Nearest, "0x0.8", Less),
        ("0x0.8", 10, Floor, "0x0.95c", Less),
        ("0x0.8", 10, Ceiling, "0x0.96", Greater),
        ("0x0.8", 10, Down, "0x0.95c", Less),
        ("0x0.8", 10, Up, "0x0.96", Greater),
        ("0x0.8", 10, Nearest, "0x0.95c", Less),
        ("0x0.8", 100, Nearest, "0x0.95c01a39fbd6879fa00b120a", Less),
        ("0x3", 1, Nearest, "0x2.0", Equal),
        ("0x3", 10, Floor, "0x2.0", Equal),
        ("0x3", 10, Ceiling, "0x2.0", Equal),
        ("0x3", 10, Down, "0x2.0", Equal),
        ("0x3", 10, Up, "0x2.0", Equal),
        ("0x3", 10, Nearest, "0x2.0", Equal),
        ("0x3", 100, Nearest, "0x2.0", Equal),
        ("0x3", 300, Exact, "0x2.0", Equal),
        ("0x7", 1, Nearest, "0x4.0", Greater),
        ("0x7", 10, Floor, "0x3.0", Equal),
        ("0x7", 10, Ceiling, "0x3.0", Equal),
        ("0x7", 10, Down, "0x3.0", Equal),
        ("0x7", 10, Up, "0x3.0", Equal),
        ("0x7", 10, Nearest, "0x3.0", Equal),
        ("0x7", 100, Nearest, "0x3.0", Equal),
        ("0x7", 300, Exact, "0x3.0", Equal),
        ("0x64", 1, Nearest, "0x8.0", Greater),
        ("0x64", 10, Floor, "0x6.a8", Less),
        ("0x64", 10, Ceiling, "0x6.aa", Greater),
        ("0x64", 10, Down, "0x6.a8", Less),
        ("0x64", 10, Up, "0x6.aa", Greater),
        ("0x64", 10, Nearest, "0x6.a8", Less),
        ("0x64", 100, Nearest, "0x6.a8808c384547c6ef4a49bc59", Less),
        ("0x3e7", 1, Nearest, "0x8.0", Less),
        ("0x3e7", 10, Floor, "0x9.f4", Less),
        ("0x3e7", 10, Ceiling, "0x9.f8", Greater),
        ("0x3e7", 10, Down, "0x9.f4", Less),
        ("0x3e7", 10, Up, "0x9.f8", Greater),
        ("0x3e7", 10, Nearest, "0x9.f8", Greater),
        (
            "0x3e7",
            100,
            Nearest,
            "0x9.f73da38d9d4a83eb6e0f93f8",
            Greater,
        ),
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
            "0x9.f79c23dd887435959c9b866d",
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
            "0x21.3822cbd8aaedf916de6f2404",
            Greater,
        ),
        ("0x1.0E+20", 1, Nearest, "0x4.0E+1", Less),
        ("0x1.0E+20", 10, Floor, "0x5.0E+1", Less),
        ("0x1.0E+20", 10, Ceiling, "0x50.2", Greater),
        ("0x1.0E+20", 10, Down, "0x5.0E+1", Less),
        ("0x1.0E+20", 10, Up, "0x50.2", Greater),
        ("0x1.0E+20", 10, Nearest, "0x5.0E+1", Less),
        (
            "0x1.0E+20",
            100,
            Nearest,
            "0x50.000000000000000000017158",
            Greater,
        ),
        ("0x2.0E+20", 1, Nearest, "0x4.0E+1", Less),
        ("0x2.0E+20", 10, Floor, "0x51.0", Less),
        ("0x2.0E+20", 10, Ceiling, "0x51.2", Greater),
        ("0x2.0E+20", 10, Down, "0x51.0", Less),
        ("0x2.0E+20", 10, Up, "0x51.2", Greater),
        ("0x2.0E+20", 10, Nearest, "0x51.0", Less),
        (
            "0x2.0E+20",
            100,
            Nearest,
            "0x51.00000000000000000000b8a8",
            Less,
        ),
        ("0x1.0E+300000000", 1, Nearest, "0x4.0E+7", Less),
        ("0x1.0E+300000000", 10, Floor, "0x4.78E+7", Less),
        ("0x1.0E+300000000", 10, Ceiling, "0x4.7aE+7", Greater),
        ("0x1.0E+300000000", 10, Down, "0x4.78E+7", Less),
        ("0x1.0E+300000000", 10, Up, "0x4.7aE+7", Greater),
        ("0x1.0E+300000000", 10, Nearest, "0x4.78E+7", Less),
        ("0x1.0E+300000000", 100, Nearest, "0x4.7868cE+7", Less),
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
        ("0x1.0E-100", 1, Nearest, "0x1.0E-100", Less),
        ("0x1.0E-100", 10, Floor, "0x1.71E-100", Less),
        ("0x1.0E-100", 10, Ceiling, "0x1.718E-100", Greater),
        ("0x1.0E-100", 10, Down, "0x1.71E-100", Less),
        ("0x1.0E-100", 10, Up, "0x1.718E-100", Greater),
        ("0x1.0E-100", 10, Nearest, "0x1.718E-100", Greater),
        (
            "0x1.0E-100",
            100,
            Nearest,
            "0x1.71547652b82fe1777d0ffda0eE-100",
            Greater,
        ),
        ("0x1.8E-300000000", 1, Nearest, "0x2.0E-300000000", Less),
        ("0x1.8E-300000000", 10, Floor, "0x2.29E-300000000", Less),
        (
            "0x1.8E-300000000",
            10,
            Ceiling,
            "0x2.2aE-300000000",
            Greater,
        ),
        ("0x1.8E-300000000", 10, Down, "0x2.29E-300000000", Less),
        ("0x1.8E-300000000", 10, Up, "0x2.2aE-300000000", Greater),
        (
            "0x1.8E-300000000",
            10,
            Nearest,
            "0x2.2aE-300000000",
            Greater,
        ),
        (
            "0x1.8E-300000000",
            100,
            Nearest,
            "0x2.29feb17c1447d2333b97fc714E-300000000",
            Greater,
        ),
        (
            "-0x1.8E-300000000",
            1,
            Nearest,
            "-0x2.0E-300000000",
            Greater,
        ),
        ("-0x1.8E-300000000", 10, Floor, "-0x2.2aE-300000000", Less),
        (
            "-0x1.8E-300000000",
            10,
            Ceiling,
            "-0x2.29E-300000000",
            Greater,
        ),
        ("-0x1.8E-300000000", 10, Down, "-0x2.29E-300000000", Greater),
        ("-0x1.8E-300000000", 10, Up, "-0x2.2aE-300000000", Less),
        ("-0x1.8E-300000000", 10, Nearest, "-0x2.2aE-300000000", Less),
        (
            "-0x1.8E-300000000",
            100,
            Nearest,
            "-0x2.29feb17c1447d2333b97fc714E-300000000",
            Less,
        ),
        ("-0x0.8", 1, Nearest, "-0x1.0", Equal),
        ("-0x0.8", 10, Floor, "-0x1.0", Equal),
        ("-0x0.8", 10, Ceiling, "-0x1.0", Equal),
        ("-0x0.8", 10, Down, "-0x1.0", Equal),
        ("-0x0.8", 10, Up, "-0x1.0", Equal),
        ("-0x0.8", 10, Nearest, "-0x1.0", Equal),
        ("-0x0.8", 100, Nearest, "-0x1.0", Equal),
        ("-0x0.8", 300, Exact, "-0x1.0", Equal),
        ("-0x3", 1, Nearest, "NaN", Equal),
        ("-0x3", 10, Floor, "NaN", Equal),
        ("-0x3", 10, Ceiling, "NaN", Equal),
        ("-0x3", 10, Down, "NaN", Equal),
        ("-0x3", 10, Up, "NaN", Equal),
        ("-0x3", 10, Nearest, "NaN", Equal),
        ("-0x3", 100, Nearest, "NaN", Equal),
        ("-0x3", 300, Exact, "NaN", Equal),
        ("0x1.0", 1, Nearest, "0x1.0", Equal),
        ("0x1.0", 10, Floor, "0x1.0", Equal),
        ("0x1.0", 10, Ceiling, "0x1.0", Equal),
        ("0x1.0", 10, Down, "0x1.0", Equal),
        ("0x1.0", 10, Up, "0x1.0", Equal),
        ("0x1.0", 10, Nearest, "0x1.0", Equal),
        ("0x1.0", 100, Nearest, "0x1.0", Equal),
        ("0x1.0", 300, Exact, "0x1.0", Equal),
        ("0x0.ffffffff", 1, Nearest, "0x1.0", Greater),
        ("0x0.ffffffff", 10, Floor, "0x0.ffc", Less),
        ("0x0.ffffffff", 10, Ceiling, "0x1.0", Greater),
        ("0x0.ffffffff", 10, Down, "0x0.ffc", Less),
        ("0x0.ffffffff", 10, Up, "0x1.0", Greater),
        ("0x0.ffffffff", 10, Nearest, "0x1.0", Greater),
        (
            "0x0.ffffffff",
            100,
            Nearest,
            "0x0.ffffffff4755c4d675bd8079e",
            Greater,
        ),
    ];
    for &(x, prec, rm, out, o) in cases {
        let r = log_base_2_1_plus_x_prec_round_all(&hex(x), prec, rm);
        assert_eq!(format!("{:?}", r.0), out, "{x} {prec} {rm}");
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    }
}

#[test]
#[should_panic]
fn log_base_2_1_plus_x_prec_round_fail_1() {
    BigExponentFloat::from(3).log_base_2_1_plus_x_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn log_base_2_1_plus_x_prec_round_fail_2() {
    BigExponentFloat::from(core::f64::consts::PI).log_base_2_1_plus_x_prec_round(10, Exact);
}

fn log_base_2_1_plus_x_prec_round_properties_helper(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) {
    let r = log_base_2_1_plus_x_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = log_base_2_1_plus_x_prec_round_all(x, prec, Floor);
        let ceiling = log_base_2_1_plus_x_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(
                &log_base_2_1_plus_x_prec_round_all(x, prec, Exact),
                &r
            ));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = log_base_2_1_plus_x_prec_round_all(x, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // log_b(1 + x) is log_b of the exact sum 1 + x.
    if x.is_finite() && exponent_is_moderate(x) {
        let z = x + BigExponentFloat::ONE;
        assert!(same_pair(&z.log_base_2_prec_round_ref(prec, rm), &r));
    }
}

#[test]
fn log_base_2_1_plus_x_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_18().test_properties(
        |(x, prec, rm)| {
            log_base_2_1_plus_x_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn log_base_2_1_plus_x_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_25().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let (f, fo) = x.log_base_2_1_plus_x_prec_round_ref(prec, rm);
        if float_result_comparable(&f, fo) {
            let r = log_base_2_1_plus_x_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
