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
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_16,
    float_unsigned_rounding_mode_triple_gen_var_21,
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

fn ln_1_plus_x_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().ln_1_plus_x_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.ln_1_plus_x_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.ln_1_plus_x_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().ln_1_plus_x_prec(prec), &r));
        assert!(same_pair(&x.ln_1_plus_x_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.ln_1_plus_x_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

#[test]
fn test_ln_1_plus_x_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &ln_1_plus_x_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_1_plus_x_prec_round_all(&inf, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_1_plus_x_prec_round_all(&ninf, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_1_plus_x_prec_round_all(&zero, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_1_plus_x_prec_round_all(&BigExponentFloat::NEGATIVE_ONE, 10, rm),
            &(ninf.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_1_plus_x_prec_round_all(&-BigExponentFloat::TWO, 10, rm),
            &(nan.clone(), Equal)
        ));
    }
    // The expected values agree with MPFR's.
    let cases: &[(&str, u64, RoundingMode, &str, Ordering)] = &[
        ("0x3.243f6a8885a30", 1, Nearest, "0x1.0", Less),
        ("0x3.243f6a8885a30", 10, Floor, "0x1.6b8", Less),
        ("0x3.243f6a8885a30", 10, Ceiling, "0x1.6c", Greater),
        ("0x3.243f6a8885a30", 10, Down, "0x1.6b8", Less),
        ("0x3.243f6a8885a30", 10, Up, "0x1.6c", Greater),
        ("0x3.243f6a8885a30", 10, Nearest, "0x1.6c", Greater),
        (
            "0x3.243f6a8885a30",
            100,
            Nearest,
            "0x1.6bcbed09f00aece2ea800b6b",
            Greater,
        ),
        ("0x0.8", 1, Nearest, "0x0.8", Greater),
        ("0x0.8", 10, Floor, "0x0.67c", Less),
        ("0x0.8", 10, Ceiling, "0x0.67e", Greater),
        ("0x0.8", 10, Down, "0x0.67c", Less),
        ("0x0.8", 10, Up, "0x0.67e", Greater),
        ("0x0.8", 10, Nearest, "0x0.67c", Less),
        ("0x0.8", 100, Nearest, "0x0.67cc8fb2fe612fcada35d9bd", Less),
        ("0x3", 1, Nearest, "0x1.0", Less),
        ("0x3", 10, Floor, "0x1.628", Less),
        ("0x3", 10, Ceiling, "0x1.63", Greater),
        ("0x3", 10, Down, "0x1.628", Less),
        ("0x3", 10, Up, "0x1.63", Greater),
        ("0x3", 10, Nearest, "0x1.63", Greater),
        ("0x3", 100, Nearest, "0x1.62e42fefa39ef35793c7673", Less),
        ("0x7", 1, Nearest, "0x2.0", Less),
        ("0x7", 10, Floor, "0x2.14", Less),
        ("0x7", 10, Ceiling, "0x2.15", Greater),
        ("0x7", 10, Down, "0x2.14", Less),
        ("0x7", 10, Up, "0x2.15", Greater),
        ("0x7", 10, Nearest, "0x2.14", Less),
        ("0x7", 100, Nearest, "0x2.145647e7756e6d035dab1ac8", Less),
        ("0x64", 1, Nearest, "0x4.0", Less),
        ("0x64", 10, Floor, "0x4.9c", Less),
        ("0x64", 10, Ceiling, "0x4.9e", Greater),
        ("0x64", 10, Down, "0x4.9c", Less),
        ("0x64", 10, Up, "0x4.9e", Greater),
        ("0x64", 10, Nearest, "0x4.9e", Greater),
        ("0x64", 100, Nearest, "0x4.9d7889c6ee8c2f8e3679653f8", Less),
        ("0x3e7", 1, Nearest, "0x8.0", Greater),
        ("0x3e7", 10, Floor, "0x6.e8", Less),
        ("0x3e7", 10, Ceiling, "0x6.ea", Greater),
        ("0x3e7", 10, Down, "0x6.e8", Less),
        ("0x3e7", 10, Up, "0x6.ea", Greater),
        ("0x3e7", 10, Nearest, "0x6.e8", Less),
        ("0x3e7", 100, Nearest, "0x6.e862a663ffe81112fc120a0a", Less),
        ("0x3e8", 1, Nearest, "0x8.0", Greater),
        ("0x3e8", 10, Floor, "0x6.e8", Less),
        ("0x3e8", 10, Ceiling, "0x6.ea", Greater),
        ("0x3e8", 10, Down, "0x6.e8", Less),
        ("0x3e8", 10, Up, "0x6.ea", Greater),
        ("0x3e8", 10, Nearest, "0x6.e8", Less),
        ("0x3e8", 100, Nearest, "0x6.e8a427393e190c0f51e66f788", Less),
        ("0x2540be400.0", 1, Nearest, "0x1.0E+1", Less),
        ("0x2540be400.0", 10, Floor, "0x17.0", Less),
        ("0x2540be400.0", 10, Ceiling, "0x17.08", Greater),
        ("0x2540be400.0", 10, Down, "0x17.0", Less),
        ("0x2540be400.0", 10, Up, "0x17.08", Greater),
        ("0x2540be400.0", 10, Nearest, "0x17.08", Greater),
        (
            "0x2540be400.0",
            100,
            Nearest,
            "0x17.069e2aa3184e62fbe4ebc0a",
            Greater,
        ),
        ("0x1.0E+20", 1, Nearest, "0x4.0E+1", Greater),
        ("0x1.0E+20", 10, Floor, "0x37.7", Less),
        ("0x1.0E+20", 10, Ceiling, "0x37.8", Greater),
        ("0x1.0E+20", 10, Down, "0x37.7", Less),
        ("0x1.0E+20", 10, Up, "0x37.8", Greater),
        ("0x1.0E+20", 10, Nearest, "0x37.7", Less),
        (
            "0x1.0E+20",
            100,
            Nearest,
            "0x37.73a77d7190d605af17291f8",
            Less,
        ),
        ("0x2.0E+20", 1, Nearest, "0x4.0E+1", Greater),
        ("0x2.0E+20", 10, Floor, "0x38.2", Less),
        ("0x2.0E+20", 10, Ceiling, "0x38.3", Greater),
        ("0x2.0E+20", 10, Down, "0x38.2", Less),
        ("0x2.0E+20", 10, Up, "0x38.3", Greater),
        ("0x2.0E+20", 10, Nearest, "0x38.2", Less),
        (
            "0x2.0E+20",
            100,
            Nearest,
            "0x38.2519956962a57f5ae10c5318",
            Less,
        ),
        ("0x1.0E+300000000", 1, Nearest, "0x4.0E+7", Greater),
        ("0x1.0E+300000000", 10, Floor, "0x3.19E+7", Less),
        ("0x1.0E+300000000", 10, Ceiling, "0x3.1aE+7", Greater),
        ("0x1.0E+300000000", 10, Down, "0x3.19E+7", Less),
        ("0x1.0E+300000000", 10, Up, "0x3.1aE+7", Greater),
        ("0x1.0E+300000000", 10, Nearest, "0x3.19E+7", Less),
        (
            "0x1.0E+300000000",
            100,
            Nearest,
            "0x3193e768.ac03e415cb73c667b",
            Less,
        ),
        ("0x1.8E+300000000", 1, Nearest, "0x4.0E+7", Greater),
        ("0x1.8E+300000000", 10, Floor, "0x3.19E+7", Less),
        ("0x1.8E+300000000", 10, Ceiling, "0x3.1aE+7", Greater),
        ("0x1.8E+300000000", 10, Down, "0x3.19E+7", Less),
        ("0x1.8E+300000000", 10, Up, "0x3.1aE+7", Greater),
        ("0x1.8E+300000000", 10, Nearest, "0x3.19E+7", Less),
        (
            "0x1.8E+300000000",
            100,
            Nearest,
            "0x3193e769.13d073c8c9d4f6328c",
            Greater,
        ),
        ("0x1.0E-100", 1, Nearest, "0x1.0E-100", Greater),
        ("0x1.0E-100", 10, Floor, "0xf.fcE-101", Less),
        ("0x1.0E-100", 10, Ceiling, "0x1.0E-100", Greater),
        ("0x1.0E-100", 10, Down, "0xf.fcE-101", Less),
        ("0x1.0E-100", 10, Up, "0x1.0E-100", Greater),
        ("0x1.0E-100", 10, Nearest, "0x1.0E-100", Greater),
        ("0x1.0E-100", 100, Nearest, "0x1.0E-100", Greater),
        ("0x1.8E-300000000", 1, Nearest, "0x1.0E-300000000", Less),
        ("0x1.8E-300000000", 10, Floor, "0x1.7f8E-300000000", Less),
        ("0x1.8E-300000000", 10, Ceiling, "0x1.8E-300000000", Greater),
        ("0x1.8E-300000000", 10, Down, "0x1.7f8E-300000000", Less),
        ("0x1.8E-300000000", 10, Up, "0x1.8E-300000000", Greater),
        ("0x1.8E-300000000", 10, Nearest, "0x1.8E-300000000", Greater),
        (
            "0x1.8E-300000000",
            100,
            Nearest,
            "0x1.8E-300000000",
            Greater,
        ),
        ("-0x1.8E-300000000", 1, Nearest, "-0x2.0E-300000000", Less),
        ("-0x1.8E-300000000", 10, Floor, "-0x1.808E-300000000", Less),
        (
            "-0x1.8E-300000000",
            10,
            Ceiling,
            "-0x1.8E-300000000",
            Greater,
        ),
        ("-0x1.8E-300000000", 10, Down, "-0x1.8E-300000000", Greater),
        ("-0x1.8E-300000000", 10, Up, "-0x1.808E-300000000", Less),
        (
            "-0x1.8E-300000000",
            10,
            Nearest,
            "-0x1.8E-300000000",
            Greater,
        ),
        (
            "-0x1.8E-300000000",
            100,
            Nearest,
            "-0x1.8E-300000000",
            Greater,
        ),
        ("-0x0.8", 1, Nearest, "-0x0.8", Greater),
        ("-0x0.8", 10, Floor, "-0x0.b18", Less),
        ("-0x0.8", 10, Ceiling, "-0x0.b14", Greater),
        ("-0x0.8", 10, Down, "-0x0.b14", Greater),
        ("-0x0.8", 10, Up, "-0x0.b18", Less),
        ("-0x0.8", 10, Nearest, "-0x0.b18", Less),
        (
            "-0x0.8",
            100,
            Nearest,
            "-0x0.b17217f7d1cf79abc9e3b398",
            Greater,
        ),
        ("-0x3", 1, Nearest, "NaN", Equal),
        ("-0x3", 10, Floor, "NaN", Equal),
        ("-0x3", 10, Ceiling, "NaN", Equal),
        ("-0x3", 10, Down, "NaN", Equal),
        ("-0x3", 10, Up, "NaN", Equal),
        ("-0x3", 10, Nearest, "NaN", Equal),
        ("-0x3", 100, Nearest, "NaN", Equal),
        ("-0x3", 300, Exact, "NaN", Equal),
        ("0x1.0", 1, Nearest, "0x0.8", Less),
        ("0x1.0", 10, Floor, "0x0.b14", Less),
        ("0x1.0", 10, Ceiling, "0x0.b18", Greater),
        ("0x1.0", 10, Down, "0x0.b14", Less),
        ("0x1.0", 10, Up, "0x0.b18", Greater),
        ("0x1.0", 10, Nearest, "0x0.b18", Greater),
        ("0x1.0", 100, Nearest, "0x0.b17217f7d1cf79abc9e3b398", Less),
        ("0x0.ffffffff", 1, Nearest, "0x0.8", Less),
        ("0x0.ffffffff", 10, Floor, "0x0.b14", Less),
        ("0x0.ffffffff", 10, Ceiling, "0x0.b18", Greater),
        ("0x0.ffffffff", 10, Down, "0x0.b14", Less),
        ("0x0.ffffffff", 10, Up, "0x0.b18", Greater),
        ("0x0.ffffffff", 10, Nearest, "0x0.b18", Greater),
        (
            "0x0.ffffffff",
            100,
            Nearest,
            "0x0.b17217f751cf79aba9e3b398",
            Greater,
        ),
    ];
    for &(x, prec, rm, out, o) in cases {
        let r = ln_1_plus_x_prec_round_all(&hex(x), prec, rm);
        assert_eq!(format!("{:?}", r.0), out, "{x} {prec} {rm}");
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    }
}

#[test]
#[should_panic]
fn ln_1_plus_x_prec_round_fail_1() {
    BigExponentFloat::from(3).ln_1_plus_x_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn ln_1_plus_x_prec_round_fail_2() {
    BigExponentFloat::from(core::f64::consts::PI).ln_1_plus_x_prec_round(10, Exact);
}

fn ln_1_plus_x_prec_round_properties_helper(x: &BigExponentFloat, prec: u64, rm: RoundingMode) {
    let r = ln_1_plus_x_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = ln_1_plus_x_prec_round_all(x, prec, Floor);
        let ceiling = ln_1_plus_x_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(&ln_1_plus_x_prec_round_all(x, prec, Exact), &r));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = ln_1_plus_x_prec_round_all(x, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // log_b(1 + x) is log_b of the exact sum 1 + x.
    if x.is_finite() && exponent_is_moderate(x) {
        let z = x + BigExponentFloat::ONE;
        assert!(same_pair(&z.ln_prec_round_ref(prec, rm), &r));
    }
}

#[test]
fn ln_1_plus_x_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_16().test_properties(
        |(x, prec, rm)| {
            ln_1_plus_x_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn ln_1_plus_x_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_21().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let (f, fo) = x.ln_1_plus_x_prec_round_ref(prec, rm);
        if float_result_comparable(&f, fo) {
            let r = ln_1_plus_x_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
