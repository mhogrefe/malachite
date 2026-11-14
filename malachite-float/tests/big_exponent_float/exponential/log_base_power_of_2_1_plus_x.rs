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
    big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_4,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_7,
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

fn log_base_power_of_2_1_plus_x_prec_round_all(
    x: &BigExponentFloat,
    k: i64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x
        .clone()
        .log_base_power_of_2_1_plus_x_prec_round(k, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.log_base_power_of_2_1_plus_x_prec_round_ref(k, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.log_base_power_of_2_1_plus_x_prec_round_assign(k, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(
            &x.clone().log_base_power_of_2_1_plus_x_prec(k, prec),
            &r
        ));
        assert!(same_pair(
            &x.log_base_power_of_2_1_plus_x_prec_ref(k, prec),
            &r
        ));
        let mut z = x.clone();
        let o = z.log_base_power_of_2_1_plus_x_prec_assign(k, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

#[test]
fn test_log_base_power_of_2_1_plus_x_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &log_base_power_of_2_1_plus_x_prec_round_all(&nan, 3, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_1_plus_x_prec_round_all(&inf, 3, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_1_plus_x_prec_round_all(&ninf, 3, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_1_plus_x_prec_round_all(&zero, 3, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_1_plus_x_prec_round_all(
                &BigExponentFloat::NEGATIVE_ONE,
                3,
                10,
                rm
            ),
            &(ninf.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_1_plus_x_prec_round_all(&-BigExponentFloat::TWO, 3, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_1_plus_x_prec_round_all(&nan, -3, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_1_plus_x_prec_round_all(&inf, -3, 10, rm),
            &(ninf.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_1_plus_x_prec_round_all(&ninf, -3, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_1_plus_x_prec_round_all(&zero, -3, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_1_plus_x_prec_round_all(
                &BigExponentFloat::NEGATIVE_ONE,
                -3,
                10,
                rm
            ),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_1_plus_x_prec_round_all(&-BigExponentFloat::TWO, -3, 10, rm),
            &(nan.clone(), Equal)
        ));
    }
    // The expected values agree with MPFR's (computed through the base-2 logarithm divided by 3).
    let cases: &[(&str, u64, RoundingMode, &str, Ordering)] = &[
        ("0x3.243f6a8885a30", 1, Nearest, "0x0.8", Less),
        ("0x3.243f6a8885a30", 10, Floor, "0x0.aec", Less),
        ("0x3.243f6a8885a30", 10, Ceiling, "0x0.af", Greater),
        ("0x3.243f6a8885a30", 10, Down, "0x0.aec", Less),
        ("0x3.243f6a8885a30", 10, Up, "0x0.af", Greater),
        ("0x3.243f6a8885a30", 10, Nearest, "0x0.af", Greater),
        (
            "0x3.243f6a8885a30",
            100,
            Nearest,
            "0x0.aef2fd3e37873606cf1f0a689",
            Greater,
        ),
        ("0x0.8", 1, Nearest, "0x0.4", Greater),
        ("0x0.8", 10, Floor, "0x0.31e", Less),
        ("0x0.8", 10, Ceiling, "0x0.31f", Greater),
        ("0x0.8", 10, Down, "0x0.31e", Less),
        ("0x0.8", 10, Up, "0x0.31f", Greater),
        ("0x0.8", 10, Nearest, "0x0.31f", Greater),
        (
            "0x0.8",
            100,
            Nearest,
            "0x0.31eab368a947828a8aae5b58ac",
            Less,
        ),
        ("0x3", 1, Nearest, "0x0.8", Less),
        ("0x3", 10, Floor, "0x0.aa8", Less),
        ("0x3", 10, Ceiling, "0x0.aac", Greater),
        ("0x3", 10, Down, "0x0.aa8", Less),
        ("0x3", 10, Up, "0x0.aac", Greater),
        ("0x3", 10, Nearest, "0x0.aac", Greater),
        (
            "0x3",
            100,
            Nearest,
            "0x0.aaaaaaaaaaaaaaaaaaaaaaaab",
            Greater,
        ),
        ("0x7", 1, Nearest, "0x1.0", Equal),
        ("0x7", 10, Floor, "0x1.0", Equal),
        ("0x7", 10, Ceiling, "0x1.0", Equal),
        ("0x7", 10, Down, "0x1.0", Equal),
        ("0x7", 10, Up, "0x1.0", Equal),
        ("0x7", 10, Nearest, "0x1.0", Equal),
        ("0x7", 100, Nearest, "0x1.0", Equal),
        ("0x7", 300, Exact, "0x1.0", Equal),
        ("0x64", 1, Nearest, "0x2.0", Less),
        ("0x64", 10, Floor, "0x2.38", Less),
        ("0x64", 10, Ceiling, "0x2.39", Greater),
        ("0x64", 10, Down, "0x2.38", Less),
        ("0x64", 10, Up, "0x2.39", Greater),
        ("0x64", 10, Nearest, "0x2.38", Less),
        (
            "0x64",
            100,
            Nearest,
            "0x2.382ad9681717ecfa6e18941dc",
            Greater,
        ),
        ("0x3e7", 1, Nearest, "0x4.0", Greater),
        ("0x3e7", 10, Floor, "0x3.52", Less),
        ("0x3e7", 10, Ceiling, "0x3.53", Greater),
        ("0x3e7", 10, Down, "0x3.52", Less),
        ("0x3e7", 10, Up, "0x3.53", Greater),
        ("0x3e7", 10, Nearest, "0x3.52", Less),
        (
            "0x3e7",
            100,
            Nearest,
            "0x3.5269e12f346e2bf924afdbfd4",
            Greater,
        ),
        ("0x3e8", 1, Nearest, "0x4.0", Greater),
        ("0x3e8", 10, Floor, "0x3.52", Less),
        ("0x3e8", 10, Ceiling, "0x3.53", Greater),
        ("0x3e8", 10, Down, "0x3.52", Less),
        ("0x3e8", 10, Up, "0x3.53", Greater),
        ("0x3e8", 10, Nearest, "0x3.53", Greater),
        ("0x3e8", 100, Nearest, "0x3.52896149d826bc873433d7798", Less),
        ("0x2540be400.0", 1, Nearest, "0x8.0", Less),
        ("0x2540be400.0", 10, Floor, "0xb.1", Less),
        ("0x2540be400.0", 10, Ceiling, "0xb.14", Greater),
        ("0x2540be400.0", 10, Down, "0xb.1", Less),
        ("0x2540be400.0", 10, Up, "0xb.14", Greater),
        ("0x2540be400.0", 10, Nearest, "0xb.14", Greater),
        (
            "0x2540be400.0",
            100,
            Nearest,
            "0xb.12b643f2e3a4a85cf4cfb6ac",
            Greater,
        ),
        ("0x1.0E+20", 1, Nearest, "0x2.0E+1", Greater),
        ("0x1.0E+20", 10, Floor, "0x1a.a8", Less),
        ("0x1.0E+20", 10, Ceiling, "0x1a.b", Greater),
        ("0x1.0E+20", 10, Down, "0x1a.a8", Less),
        ("0x1.0E+20", 10, Up, "0x1a.b", Greater),
        ("0x1.0E+20", 10, Nearest, "0x1a.a8", Less),
        (
            "0x1.0E+20",
            100,
            Nearest,
            "0x1a.aaaaaaaaaaaaaaaaaaab25c6",
            Less,
        ),
        ("0x2.0E+20", 1, Nearest, "0x2.0E+1", Greater),
        ("0x2.0E+20", 10, Floor, "0x1b.0", Less),
        ("0x2.0E+20", 10, Ceiling, "0x1b.08", Greater),
        ("0x2.0E+20", 10, Down, "0x1b.0", Less),
        ("0x2.0E+20", 10, Up, "0x1b.08", Greater),
        ("0x2.0E+20", 10, Nearest, "0x1b.0", Less),
        (
            "0x2.0E+20",
            100,
            Nearest,
            "0x1b.000000000000000000003d8e",
            Less,
        ),
        ("0x1.0E+300000000", 1, Nearest, "0x1.0E+7", Less),
        ("0x1.0E+300000000", 10, Floor, "0x1.7dE+7", Less),
        ("0x1.0E+300000000", 10, Ceiling, "0x1.7d8E+7", Greater),
        ("0x1.0E+300000000", 10, Down, "0x1.7dE+7", Less),
        ("0x1.0E+300000000", 10, Up, "0x1.7d8E+7", Greater),
        ("0x1.0E+300000000", 10, Nearest, "0x1.7d8E+7", Greater),
        ("0x1.0E+300000000", 100, Nearest, "0x1.7d784E+7", Less),
        ("0x1.8E+300000000", 1, Nearest, "0x1.0E+7", Less),
        ("0x1.8E+300000000", 10, Floor, "0x1.7dE+7", Less),
        ("0x1.8E+300000000", 10, Ceiling, "0x1.7d8E+7", Greater),
        ("0x1.8E+300000000", 10, Down, "0x1.7dE+7", Less),
        ("0x1.8E+300000000", 10, Up, "0x1.7d8E+7", Greater),
        ("0x1.8E+300000000", 10, Nearest, "0x1.7d8E+7", Greater),
        (
            "0x1.8E+300000000",
            100,
            Nearest,
            "0x17d78400.31eab368a947828a8a",
            Less,
        ),
        ("0x1.0E-100", 1, Nearest, "0x8.0E-101", Greater),
        ("0x1.0E-100", 10, Floor, "0x7.bE-101", Less),
        ("0x1.0E-100", 10, Ceiling, "0x7.b2E-101", Greater),
        ("0x1.0E-100", 10, Down, "0x7.bE-101", Less),
        ("0x1.0E-100", 10, Up, "0x7.b2E-101", Greater),
        ("0x1.0E-100", 10, Nearest, "0x7.b2E-101", Greater),
        (
            "0x1.0E-100",
            100,
            Nearest,
            "0x7.b1c2770e80ff5d27f05548afE-101",
            Less,
        ),
        ("0x1.8E-300000000", 1, Nearest, "0x8.0E-300000001", Less),
        ("0x1.8E-300000000", 10, Floor, "0xb.88E-300000001", Less),
        (
            "0x1.8E-300000000",
            10,
            Ceiling,
            "0xb.8cE-300000001",
            Greater,
        ),
        ("0x1.8E-300000000", 10, Down, "0xb.88E-300000001", Less),
        ("0x1.8E-300000000", 10, Up, "0xb.8cE-300000001", Greater),
        (
            "0x1.8E-300000000",
            10,
            Nearest,
            "0xb.8cE-300000001",
            Greater,
        ),
        (
            "0x1.8E-300000000",
            100,
            Nearest,
            "0xb.8aa3b295c17f0bbbe87fed07E-300000001",
            Greater,
        ),
        (
            "-0x1.8E-300000000",
            1,
            Nearest,
            "-0x8.0E-300000001",
            Greater,
        ),
        ("-0x1.8E-300000000", 10, Floor, "-0xb.8cE-300000001", Less),
        (
            "-0x1.8E-300000000",
            10,
            Ceiling,
            "-0xb.88E-300000001",
            Greater,
        ),
        ("-0x1.8E-300000000", 10, Down, "-0xb.88E-300000001", Greater),
        ("-0x1.8E-300000000", 10, Up, "-0xb.8cE-300000001", Less),
        ("-0x1.8E-300000000", 10, Nearest, "-0xb.8cE-300000001", Less),
        (
            "-0x1.8E-300000000",
            100,
            Nearest,
            "-0xb.8aa3b295c17f0bbbe87fed07E-300000001",
            Less,
        ),
        ("-0x0.8", 1, Nearest, "-0x0.4", Greater),
        ("-0x0.8", 10, Floor, "-0x0.556", Less),
        ("-0x0.8", 10, Ceiling, "-0x0.554", Greater),
        ("-0x0.8", 10, Down, "-0x0.554", Greater),
        ("-0x0.8", 10, Up, "-0x0.556", Less),
        ("-0x0.8", 10, Nearest, "-0x0.556", Less),
        (
            "-0x0.8",
            100,
            Nearest,
            "-0x0.55555555555555555555555558",
            Less,
        ),
        ("-0x3", 1, Nearest, "NaN", Equal),
        ("-0x3", 10, Floor, "NaN", Equal),
        ("-0x3", 10, Ceiling, "NaN", Equal),
        ("-0x3", 10, Down, "NaN", Equal),
        ("-0x3", 10, Up, "NaN", Equal),
        ("-0x3", 10, Nearest, "NaN", Equal),
        ("-0x3", 100, Nearest, "NaN", Equal),
        ("-0x3", 300, Exact, "NaN", Equal),
        ("0x1.0", 1, Nearest, "0x0.4", Less),
        ("0x1.0", 10, Floor, "0x0.554", Less),
        ("0x1.0", 10, Ceiling, "0x0.556", Greater),
        ("0x1.0", 10, Down, "0x0.554", Less),
        ("0x1.0", 10, Up, "0x0.556", Greater),
        ("0x1.0", 10, Nearest, "0x0.556", Greater),
        (
            "0x1.0",
            100,
            Nearest,
            "0x0.55555555555555555555555558",
            Greater,
        ),
        ("0x0.ffffffff", 1, Nearest, "0x0.4", Less),
        ("0x0.ffffffff", 10, Floor, "0x0.554", Less),
        ("0x0.ffffffff", 10, Ceiling, "0x0.556", Greater),
        ("0x0.ffffffff", 10, Down, "0x0.554", Less),
        ("0x0.ffffffff", 10, Up, "0x0.556", Greater),
        ("0x0.ffffffff", 10, Nearest, "0x0.556", Greater),
        (
            "0x0.ffffffff",
            100,
            Nearest,
            "0x0.5555555517c7419cd1e9d57df",
            Less,
        ),
    ];
    for &(x, prec, rm, out, o) in cases {
        let r = log_base_power_of_2_1_plus_x_prec_round_all(&hex(x), 3, prec, rm);
        assert_eq!(format!("{:?}", r.0), out, "{x} {prec} {rm}");
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    }
}

#[test]
#[should_panic]
fn log_base_power_of_2_1_plus_x_prec_round_fail_1() {
    BigExponentFloat::from(3).log_base_power_of_2_1_plus_x_prec_round(3, 0, Floor);
}

#[test]
#[should_panic]
fn log_base_power_of_2_1_plus_x_prec_round_fail_2() {
    BigExponentFloat::from(core::f64::consts::PI)
        .log_base_power_of_2_1_plus_x_prec_round(3, 10, Exact);
}

#[test]
#[should_panic]
fn log_base_power_of_2_1_plus_x_prec_round_fail_3() {
    BigExponentFloat::from(3).log_base_power_of_2_1_plus_x_prec_round(0, 10, Floor);
}

fn log_base_power_of_2_1_plus_x_prec_round_properties_helper(
    x: &BigExponentFloat,
    k: i64,
    prec: u64,
    rm: RoundingMode,
) {
    let r = log_base_power_of_2_1_plus_x_prec_round_all(x, k, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = log_base_power_of_2_1_plus_x_prec_round_all(x, k, prec, Floor);
        let ceiling = log_base_power_of_2_1_plus_x_prec_round_all(x, k, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(
                &log_base_power_of_2_1_plus_x_prec_round_all(x, k, prec, Exact),
                &r
            ));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = log_base_power_of_2_1_plus_x_prec_round_all(x, k, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // log_b(1 + x) is log_b of the exact sum 1 + x.
    if x.is_finite() && exponent_is_moderate(x) {
        let z = x + BigExponentFloat::ONE;
        assert!(same_pair(
            &z.log_base_power_of_2_prec_round_ref(k, prec, rm),
            &r
        ));
    }
    // log_(2^1) is log_2.
    assert!(same_pair(
        &x.log_base_power_of_2_1_plus_x_prec_round_ref(1, prec, rm),
        &x.log_base_2_1_plus_x_prec_round_ref(prec, rm)
    ));
}

#[test]
fn log_base_power_of_2_1_plus_x_prec_round_properties() {
    big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_4().test_properties(
        |(x, k, prec, rm)| {
            log_base_power_of_2_1_plus_x_prec_round_properties_helper(&x, k, prec, rm);
        },
    );
}

#[test]
fn log_base_power_of_2_1_plus_x_prec_round_float_agreement_properties() {
    float_signed_unsigned_rounding_mode_quadruple_gen_var_7().test_properties(
        |(x, k, prec, rm)| {
            if rm == Exact {
                return;
            }
            let (f, fo) = x.log_base_power_of_2_1_plus_x_prec_round_ref(k, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = log_base_power_of_2_1_plus_x_prec_round_all(
                    &float_to_big_exponent_float(&x),
                    k,
                    prec,
                    rm,
                );
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
