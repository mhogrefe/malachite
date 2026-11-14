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
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_10, float_unsigned_pair_gen_var_1,
    rational_unsigned_rounding_mode_triple_gen_var_20,
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

fn exp_x_minus_1_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().exp_x_minus_1_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.exp_x_minus_1_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.exp_x_minus_1_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().exp_x_minus_1_prec(prec), &r));
        assert!(same_pair(&x.exp_x_minus_1_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.exp_x_minus_1_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn exp_x_minus_1_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::exp_x_minus_1_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::exp_x_minus_1_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::exp_x_minus_1_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::exp_x_minus_1_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_exp_x_minus_1_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &exp_x_minus_1_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &exp_x_minus_1_prec_round_all(&inf, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &exp_x_minus_1_prec_round_all(&ninf, 10, rm),
            &(BigExponentFloat::NEGATIVE_ONE, Equal)
        ));
        assert!(same_pair(
            &exp_x_minus_1_prec_round_all(&zero, 10, rm),
            &(zero.clone(), Equal)
        ));
    }
    // The expected values agree with MPFR's wherever MPFR's exponent range allows.
    let cases: &[(&str, u64, RoundingMode, &str, Ordering)] = &[
        ("0x3.243f6a8885a30", 1, Nearest, "0x1.0E+1", Less),
        ("0x3.243f6a8885a30", 10, Floor, "0x16.2", Less),
        ("0x3.243f6a8885a30", 10, Ceiling, "0x16.28", Greater),
        ("0x3.243f6a8885a30", 10, Down, "0x16.2", Less),
        ("0x3.243f6a8885a30", 10, Up, "0x16.28", Greater),
        ("0x3.243f6a8885a30", 10, Nearest, "0x16.28", Greater),
        (
            "0x3.243f6a8885a30",
            100,
            Nearest,
            "0x16.24046eb09338d2991a30893e",
            Less,
        ),
        ("-0x3.243f6a8885a30", 1, Nearest, "-0x1.0", Less),
        ("-0x3.243f6a8885a30", 10, Floor, "-0x0.f5", Less),
        ("-0x3.243f6a8885a30", 10, Ceiling, "-0x0.f4c", Greater),
        ("-0x3.243f6a8885a30", 10, Down, "-0x0.f4c", Greater),
        ("-0x3.243f6a8885a30", 10, Up, "-0x0.f5", Less),
        ("-0x3.243f6a8885a30", 10, Nearest, "-0x0.f5", Less),
        (
            "-0x3.243f6a8885a30",
            100,
            Nearest,
            "-0x0.f4efeec253388335b0cf8dd92",
            Less,
        ),
        ("0x0.8", 1, Nearest, "0x0.8", Less),
        ("0x0.8", 10, Floor, "0x0.a6", Less),
        ("0x0.8", 10, Ceiling, "0x0.a64", Greater),
        ("0x0.8", 10, Down, "0x0.a6", Less),
        ("0x0.8", 10, Up, "0x0.a64", Greater),
        ("0x0.8", 10, Nearest, "0x0.a6", Less),
        (
            "0x0.8",
            100,
            Nearest,
            "0x0.a61298e1e069bc972dfefab6e",
            Greater,
        ),
        ("-0x0.8", 1, Nearest, "-0x0.8", Less),
        ("-0x0.8", 10, Floor, "-0x0.64c", Less),
        ("-0x0.8", 10, Ceiling, "-0x0.64a", Greater),
        ("-0x0.8", 10, Down, "-0x0.64a", Greater),
        ("-0x0.8", 10, Up, "-0x0.64c", Less),
        ("-0x0.8", 10, Nearest, "-0x0.64c", Less),
        (
            "-0x0.8",
            100,
            Nearest,
            "-0x0.64ba681c834fb00c298a5caad",
            Less,
        ),
        ("0x3", 1, Nearest, "0x1.0E+1", Less),
        ("0x3", 10, Floor, "0x13.1", Less),
        ("0x3", 10, Ceiling, "0x13.18", Greater),
        ("0x3", 10, Down, "0x13.1", Less),
        ("0x3", 10, Up, "0x13.18", Greater),
        ("0x3", 10, Nearest, "0x13.18", Greater),
        ("0x3", 100, Nearest, "0x13.15e5bf6fb105f2d4bdfc5374", Less),
        ("-0x3", 1, Nearest, "-0x1.0", Less),
        ("-0x3", 10, Floor, "-0x0.f38", Less),
        ("-0x3", 10, Ceiling, "-0x0.f34", Greater),
        ("-0x3", 10, Down, "-0x0.f34", Greater),
        ("-0x3", 10, Up, "-0x0.f38", Less),
        ("-0x3", 10, Nearest, "-0x0.f34", Greater),
        (
            "-0x3",
            100,
            Nearest,
            "-0x0.f341279998a7a89b5becfe6e3",
            Greater,
        ),
        ("0x64", 1, Nearest, "0x1.0E+36", Less),
        ("0x64", 10, Floor, "0x1.348E+36", Less),
        ("0x64", 10, Ceiling, "0x1.35E+36", Greater),
        ("0x64", 10, Down, "0x1.348E+36", Less),
        ("0x64", 10, Up, "0x1.35E+36", Greater),
        ("0x64", 10, Nearest, "0x1.348E+36", Less),
        (
            "0x64",
            100,
            Nearest,
            "0x1.3494a9b171bf4acc225093322E+36",
            Less,
        ),
        ("-0x64", 1, Nearest, "-0x1.0", Less),
        ("-0x64", 10, Floor, "-0x1.0", Less),
        ("-0x64", 10, Ceiling, "-0x0.ffc", Greater),
        ("-0x64", 10, Down, "-0x0.ffc", Greater),
        ("-0x64", 10, Up, "-0x1.0", Less),
        ("-0x64", 10, Nearest, "-0x1.0", Less),
        ("-0x64", 100, Nearest, "-0x1.0", Less),
        ("0x2540be400.0", 1, Nearest, "0x2.0E+3606737602", Greater),
        ("0x2540be400.0", 10, Floor, "0x1.daE+3606737602", Less),
        ("0x2540be400.0", 10, Ceiling, "0x1.da8E+3606737602", Greater),
        ("0x2540be400.0", 10, Down, "0x1.daE+3606737602", Less),
        ("0x2540be400.0", 10, Up, "0x1.da8E+3606737602", Greater),
        ("0x2540be400.0", 10, Nearest, "0x1.da8E+3606737602", Greater),
        (
            "0x2540be400.0",
            100,
            Nearest,
            "0x1.da4af321e8100802fc3be5738E+3606737602",
            Less,
        ),
        ("-0x2540be400.0", 1, Nearest, "-0x1.0", Less),
        ("-0x2540be400.0", 10, Floor, "-0x1.0", Less),
        ("-0x2540be400.0", 10, Ceiling, "-0x0.ffc", Greater),
        ("-0x2540be400.0", 10, Down, "-0x0.ffc", Greater),
        ("-0x2540be400.0", 10, Up, "-0x1.0", Less),
        ("-0x2540be400.0", 10, Nearest, "-0x1.0", Less),
        ("-0x2540be400.0", 100, Nearest, "-0x1.0", Less),
        (
            "0x1.0E+20",
            1,
            Nearest,
            "0x1.0E+436027821190162759409476",
            Greater,
        ),
        (
            "0x1.0E+20",
            10,
            Floor,
            "0xf.e4E+436027821190162759409475",
            Less,
        ),
        (
            "0x1.0E+20",
            10,
            Ceiling,
            "0xf.e8E+436027821190162759409475",
            Greater,
        ),
        (
            "0x1.0E+20",
            10,
            Down,
            "0xf.e4E+436027821190162759409475",
            Less,
        ),
        (
            "0x1.0E+20",
            10,
            Up,
            "0xf.e8E+436027821190162759409475",
            Greater,
        ),
        (
            "0x1.0E+20",
            10,
            Nearest,
            "0xf.e4E+436027821190162759409475",
            Less,
        ),
        (
            "0x1.0E+20",
            100,
            Nearest,
            "0xf.e5c7bf54c99c82105ab42b69E+436027821190162759409475",
            Less,
        ),
        ("-0x1.0E+20", 1, Nearest, "-0x1.0", Less),
        ("-0x1.0E+20", 10, Floor, "-0x1.0", Less),
        ("-0x1.0E+20", 10, Ceiling, "-0x0.ffc", Greater),
        ("-0x1.0E+20", 10, Down, "-0x0.ffc", Greater),
        ("-0x1.0E+20", 10, Up, "-0x1.0", Less),
        ("-0x1.0E+20", 10, Nearest, "-0x1.0", Less),
        ("-0x1.0E+20", 100, Nearest, "-0x1.0", Less),
        ("0x1.0E-100", 1, Nearest, "0x1.0E-100", Less),
        ("0x1.0E-100", 10, Floor, "0x1.0E-100", Less),
        ("0x1.0E-100", 10, Ceiling, "0x1.008E-100", Greater),
        ("0x1.0E-100", 10, Down, "0x1.0E-100", Less),
        ("0x1.0E-100", 10, Up, "0x1.008E-100", Greater),
        ("0x1.0E-100", 10, Nearest, "0x1.0E-100", Less),
        ("0x1.0E-100", 100, Nearest, "0x1.0E-100", Less),
        ("-0x1.0E-100", 1, Nearest, "-0x1.0E-100", Less),
        ("-0x1.0E-100", 10, Floor, "-0x1.0E-100", Less),
        ("-0x1.0E-100", 10, Ceiling, "-0xf.fcE-101", Greater),
        ("-0x1.0E-100", 10, Down, "-0xf.fcE-101", Greater),
        ("-0x1.0E-100", 10, Up, "-0x1.0E-100", Less),
        ("-0x1.0E-100", 10, Nearest, "-0x1.0E-100", Less),
        ("-0x1.0E-100", 100, Nearest, "-0x1.0E-100", Less),
        ("0x1.8E-300000000", 1, Nearest, "0x2.0E-300000000", Greater),
        ("0x1.8E-300000000", 10, Floor, "0x1.8E-300000000", Less),
        (
            "0x1.8E-300000000",
            10,
            Ceiling,
            "0x1.808E-300000000",
            Greater,
        ),
        ("0x1.8E-300000000", 10, Down, "0x1.8E-300000000", Less),
        ("0x1.8E-300000000", 10, Up, "0x1.808E-300000000", Greater),
        ("0x1.8E-300000000", 10, Nearest, "0x1.8E-300000000", Less),
        ("0x1.8E-300000000", 100, Nearest, "0x1.8E-300000000", Less),
        (
            "-0x1.8E-300000000",
            1,
            Nearest,
            "-0x1.0E-300000000",
            Greater,
        ),
        ("-0x1.8E-300000000", 10, Floor, "-0x1.8E-300000000", Less),
        (
            "-0x1.8E-300000000",
            10,
            Ceiling,
            "-0x1.7f8E-300000000",
            Greater,
        ),
        (
            "-0x1.8E-300000000",
            10,
            Down,
            "-0x1.7f8E-300000000",
            Greater,
        ),
        ("-0x1.8E-300000000", 10, Up, "-0x1.8E-300000000", Less),
        ("-0x1.8E-300000000", 10, Nearest, "-0x1.8E-300000000", Less),
        ("-0x1.8E-300000000", 100, Nearest, "-0x1.8E-300000000", Less),
        ("0x5.0E+7", 1, Nearest, "0x4.0E+484088126", Greater),
        ("0x5.0E+7", 10, Floor, "0x3.9aE+484088126", Less),
        ("0x5.0E+7", 10, Ceiling, "0x3.9bE+484088126", Greater),
        ("0x5.0E+7", 10, Down, "0x3.9aE+484088126", Less),
        ("0x5.0E+7", 10, Up, "0x3.9bE+484088126", Greater),
        ("0x5.0E+7", 10, Nearest, "0x3.9bE+484088126", Greater),
        (
            "0x5.0E+7",
            100,
            Nearest,
            "0x3.9ac6d287be5b0a228175e8a3E+484088126",
            Less,
        ),
        ("-0x5.0E+7", 1, Nearest, "-0x1.0", Less),
        ("-0x5.0E+7", 10, Floor, "-0x1.0", Less),
        ("-0x5.0E+7", 10, Ceiling, "-0x0.ffc", Greater),
        ("-0x5.0E+7", 10, Down, "-0x0.ffc", Greater),
        ("-0x5.0E+7", 10, Up, "-0x1.0", Less),
        ("-0x5.0E+7", 10, Nearest, "-0x1.0", Less),
        ("-0x5.0E+7", 100, Nearest, "-0x1.0", Less),
    ];
    for &(x, prec, rm, out, o) in cases {
        let r = exp_x_minus_1_prec_round_all(&hex(x), prec, rm);
        assert_eq!(format!("{:?}", r.0), out, "{x} {prec} {rm}");
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    }
}

#[test]
fn test_exp_x_minus_1_rational_prec_round() {
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &exp_x_minus_1_rational_prec_round_all(&Rational::ZERO, 10, rm),
            &(BigExponentFloat::ZERO, Equal)
        ));
    }
    // The expected values agree with MPFR's wherever MPFR's exponent range allows.
    let cases: &[(&str, u64, RoundingMode, &str, Ordering)] = &[
        ("1/3", 1, Nearest, "0x0.8", Greater),
        ("1/3", 10, Floor, "0x0.654", Less),
        ("1/3", 10, Ceiling, "0x0.656", Greater),
        ("1/3", 10, Down, "0x0.654", Less),
        ("1/3", 10, Up, "0x0.656", Greater),
        ("1/3", 10, Nearest, "0x0.654", Less),
        (
            "1/3",
            100,
            Nearest,
            "0x0.6546db1ba2d1310a7f6c6f27f8",
            Greater,
        ),
        ("-1/3", 1, Nearest, "-0x0.4", Greater),
        ("-1/3", 10, Floor, "-0x0.48a", Less),
        ("-1/3", 10, Ceiling, "-0x0.488", Greater),
        ("-1/3", 10, Down, "-0x0.488", Greater),
        ("-1/3", 10, Up, "-0x0.48a", Less),
        ("-1/3", 10, Nearest, "-0x0.48a", Less),
        ("-1/3", 100, Nearest, "-0x0.4891676e868ad9763a67b363b", Less),
        ("3/5", 1, Nearest, "0x1.0", Greater),
        ("3/5", 10, Floor, "0x0.d24", Less),
        ("3/5", 10, Ceiling, "0x0.d28", Greater),
        ("3/5", 10, Down, "0x0.d24", Less),
        ("3/5", 10, Up, "0x0.d28", Greater),
        ("3/5", 10, Nearest, "0x0.d28", Greater),
        ("3/5", 100, Nearest, "0x0.d27660b11a9ef66918b8417ce", Less),
        ("3", 1, Nearest, "0x1.0E+1", Less),
        ("3", 10, Floor, "0x13.1", Less),
        ("3", 10, Ceiling, "0x13.18", Greater),
        ("3", 10, Down, "0x13.1", Less),
        ("3", 10, Up, "0x13.18", Greater),
        ("3", 10, Nearest, "0x13.18", Greater),
        ("3", 100, Nearest, "0x13.15e5bf6fb105f2d4bdfc5374", Less),
        ("-3", 1, Nearest, "-0x1.0", Less),
        ("-3", 10, Floor, "-0x0.f38", Less),
        ("-3", 10, Ceiling, "-0x0.f34", Greater),
        ("-3", 10, Down, "-0x0.f34", Greater),
        ("-3", 10, Up, "-0x0.f38", Less),
        ("-3", 10, Nearest, "-0x0.f34", Greater),
        (
            "-3",
            100,
            Nearest,
            "-0x0.f341279998a7a89b5becfe6e3",
            Greater,
        ),
        ("100", 1, Nearest, "0x1.0E+36", Less),
        ("100", 10, Floor, "0x1.348E+36", Less),
        ("100", 10, Ceiling, "0x1.35E+36", Greater),
        ("100", 10, Down, "0x1.348E+36", Less),
        ("100", 10, Up, "0x1.35E+36", Greater),
        ("100", 10, Nearest, "0x1.348E+36", Less),
        (
            "100",
            100,
            Nearest,
            "0x1.3494a9b171bf4acc225093322E+36",
            Less,
        ),
        ("-100", 1, Nearest, "-0x1.0", Less),
        ("-100", 10, Floor, "-0x1.0", Less),
        ("-100", 10, Ceiling, "-0x0.ffc", Greater),
        ("-100", 10, Down, "-0x0.ffc", Greater),
        ("-100", 10, Up, "-0x1.0", Less),
        ("-100", 10, Nearest, "-0x1.0", Less),
        ("-100", 100, Nearest, "-0x1.0", Less),
        ("10000000000/3", 1, Nearest, "0x4.0E+1202245867", Greater),
        ("10000000000/3", 10, Floor, "0x3.18E+1202245867", Less),
        ("10000000000/3", 10, Ceiling, "0x3.19E+1202245867", Greater),
        ("10000000000/3", 10, Down, "0x3.18E+1202245867", Less),
        ("10000000000/3", 10, Up, "0x3.19E+1202245867", Greater),
        ("10000000000/3", 10, Nearest, "0x3.18E+1202245867", Less),
        (
            "10000000000/3",
            100,
            Nearest,
            "0x3.18494a338819c27f1502529ccE+1202245867",
            Greater,
        ),
        ("-10000000000/3", 1, Nearest, "-0x1.0", Less),
        ("-10000000000/3", 10, Floor, "-0x1.0", Less),
        ("-10000000000/3", 10, Ceiling, "-0x0.ffc", Greater),
        ("-10000000000/3", 10, Down, "-0x0.ffc", Greater),
        ("-10000000000/3", 10, Up, "-0x1.0", Less),
        ("-10000000000/3", 10, Nearest, "-0x1.0", Less),
        ("-10000000000/3", 100, Nearest, "-0x1.0", Less),
        ("1/10000000000", 1, Nearest, "0x8.0E-9", Greater),
        ("1/10000000000", 10, Floor, "0x6.deE-9", Less),
        ("1/10000000000", 10, Ceiling, "0x6.eE-9", Greater),
        ("1/10000000000", 10, Down, "0x6.deE-9", Less),
        ("1/10000000000", 10, Up, "0x6.eE-9", Greater),
        ("1/10000000000", 10, Nearest, "0x6.eE-9", Greater),
        (
            "1/10000000000",
            100,
            Nearest,
            "0x6.df37f6776938bebf05d27645E-9",
            Less,
        ),
        ("-1/10000000000", 1, Nearest, "-0x8.0E-9", Less),
        ("-1/10000000000", 10, Floor, "-0x6.eE-9", Less),
        ("-1/10000000000", 10, Ceiling, "-0x6.deE-9", Greater),
        ("-1/10000000000", 10, Down, "-0x6.deE-9", Greater),
        ("-1/10000000000", 10, Up, "-0x6.eE-9", Less),
        ("-1/10000000000", 10, Nearest, "-0x6.eE-9", Less),
        (
            "-1/10000000000",
            100,
            Nearest,
            "-0x6.df37f67475a49d2cbd8e0a9bE-9",
            Less,
        ),
        (
            "1208925819614629174706176/3",
            1,
            Nearest,
            "0x2.0E+145342607063387586469825",
            Less,
        ),
        (
            "1208925819614629174706176/3",
            10,
            Floor,
            "0x2.83E+145342607063387586469825",
            Less,
        ),
        (
            "1208925819614629174706176/3",
            10,
            Ceiling,
            "0x2.84E+145342607063387586469825",
            Greater,
        ),
        (
            "1208925819614629174706176/3",
            10,
            Down,
            "0x2.83E+145342607063387586469825",
            Less,
        ),
        (
            "1208925819614629174706176/3",
            10,
            Up,
            "0x2.84E+145342607063387586469825",
            Greater,
        ),
        (
            "1208925819614629174706176/3",
            10,
            Nearest,
            "0x2.84E+145342607063387586469825",
            Greater,
        ),
        (
            "1208925819614629174706176/3",
            100,
            Nearest,
            "0x2.83b33f00d8170fdf388d8bfdcE+145342607063387586469825",
            Greater,
        ),
    ];
    for &(x, prec, rm, out, o) in cases {
        let r = exp_x_minus_1_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out, "{x} {prec} {rm}");
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    }
}

#[test]
#[should_panic]
fn exp_x_minus_1_prec_round_fail_1() {
    BigExponentFloat::ONE.exp_x_minus_1_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn exp_x_minus_1_prec_round_fail_2() {
    BigExponentFloat::from(0.5).exp_x_minus_1_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn exp_x_minus_1_rational_prec_round_fail_1() {
    BigExponentFloat::exp_x_minus_1_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn exp_x_minus_1_rational_prec_round_fail_2() {
    BigExponentFloat::exp_x_minus_1_rational_prec_round(
        Rational::from_unsigneds(1u32, 3),
        10,
        Exact,
    );
}

fn exp_x_minus_1_prec_round_properties_helper(x: &BigExponentFloat, prec: u64, rm: RoundingMode) {
    let r = exp_x_minus_1_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if !r.0.is_nan() {
        assert_eq!(r.0.partial_cmp(&0u32), x.partial_cmp(&0u32));
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else {
        // The result lies between the results rounded down and up.
        let floor = exp_x_minus_1_prec_round_all(x, prec, Floor);
        let ceiling = exp_x_minus_1_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0 || r.0.is_nan());
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(&exp_x_minus_1_prec_round_all(x, prec, Exact), &r));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = exp_x_minus_1_prec_round_all(x, prec + 16, rm);
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
            &exp_x_minus_1_rational_prec_round_all(&q, prec, rm),
            &r
        ));
    }
}

#[test]
fn exp_x_minus_1_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_10().test_properties(
        |(x, prec, rm)| {
            exp_x_minus_1_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn exp_x_minus_1_prec_round_float_agreement_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        for rm in exhaustive_rounding_modes() {
            if rm == Exact {
                continue;
            }
            let x = &x;
            let (f, fo) = x.exp_x_minus_1_prec_round_ref(prec, rm);
            if float_result_comparable(&f, fo) {
                let r = exp_x_minus_1_prec_round_all(&float_to_big_exponent_float(x), prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        }
    });
}

#[test]
fn exp_x_minus_1_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_20().test_properties(|(x, prec, rm)| {
        let r = exp_x_minus_1_rational_prec_round_all(&x, prec, rm);
        if r.0.is_zero() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        if rm == Exact {
            assert_eq!(r.1, Equal);
        } else if rm != Nearest {
            let (y, o) = exp_x_minus_1_rational_prec_round_all(&x, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    });
}

#[test]
fn exp_x_minus_1_rational_prec_round_float_agreement_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_20().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let x = &x;
        let (f, fo) = Float::exp_x_minus_1_rational_prec_round_ref(x, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = exp_x_minus_1_rational_prec_round_all(x, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
