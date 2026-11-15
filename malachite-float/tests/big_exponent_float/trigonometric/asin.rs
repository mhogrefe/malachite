// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::trigonometric::atan::{
    InverseTrigFunction, inverse_trig_prec_round_generic,
};
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, float_to_big_exponent_float,
};
use malachite_float::test_util::generators::*;
use malachite_float::{BigExponentFloat, Float};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

fn same_pair(x: &(BigExponentFloat, Ordering), y: &(BigExponentFloat, Ordering)) -> bool {
    same(&x.0, &y.0) && x.1 == y.1
}

fn float_result_comparable(x: &Float, o: Ordering) -> bool {
    x.get_exponent().map_or_else(
        // an inexact zero or infinity is an underflow or overflow
        || o == Equal,
        |e| e > Float::MIN_EXPONENT + 1 && e < Float::MAX_EXPONENT - 1,
    )
}

fn hex(s: &str) -> BigExponentFloat {
    BigExponentFloat::from_string_base(16, s).unwrap()
}

// A `NaN` or zero result is exact; any other result is finite and has at most `prec` bits.
fn check_result(r: &(BigExponentFloat, Ordering), prec: u64) {
    if r.0.is_nan() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.is_finite());
        assert!(r.0.get_prec().unwrap() <= prec);
    }
}

const PERIODS: [u64; 5] = [0, 1, 2, 7, 360];

fn asin_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().asin_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.asin_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.asin_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().asin_prec(prec), &r));
        assert!(same_pair(&x.asin_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.asin_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn asin_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::asin_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::asin_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::asin_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::asin_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

fn asin_with_period_prec_round_all(
    x: &BigExponentFloat,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().asin_with_period_prec_round(u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.asin_with_period_prec_round_ref(u, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.asin_with_period_prec_round_assign(u, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().asin_with_period_prec(u, prec), &r));
        assert!(same_pair(&x.asin_with_period_prec_ref(u, prec), &r));
        let mut z = x.clone();
        let o = z.asin_with_period_prec_assign(u, prec);
        assert!(same_pair(&(z, o), &r));
    }
    if u == 2 {
        assert!(same_pair(&asin_pi_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn asin_with_period_rational_prec_round_all(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::asin_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::asin_with_period_rational_prec_round_ref(x, u, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::asin_with_period_rational_prec(x.clone(), u, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::asin_with_period_rational_prec_ref(x, u, prec),
            &r
        ));
    }
    if u == 2 {
        assert!(same_pair(&asin_pi_rational_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn asin_pi_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().asin_pi_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.asin_pi_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.asin_pi_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().asin_pi_prec(prec), &r));
        assert!(same_pair(&x.asin_pi_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.asin_pi_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn asin_pi_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::asin_pi_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::asin_pi_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::asin_pi_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::asin_pi_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_asin_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = asin_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_trig_prec_round_generic(InverseTrigFunction::Asin, &x, None, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 10, Exact, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8", 10, Floor, "0.5234", "0x0.86", Less);
    test("-0.50", "-0x0.8", 10, Floor, "-0.52441", "-0x0.864", Less);
    test("1.0", "0x1", 10, Floor, "1.570", "0x1.92", Less);
    test("-1.0", "-0x1", 10, Floor, "-1.5723", "-0x1.928", Less);
    test("2.0", "0x2", 10, Exact, "NaN", "NaN", Equal);
    test("-2.0", "-0x2", 10, Exact, "NaN", "NaN", Equal);
    test("0.38", "0x0.6", 1, Nearest, "0.50", "0x0.8", Greater);
    test("0.38", "0x0.6", 10, Floor, "0.38428", "0x0.626", Less);
    test("0.38", "0x0.6", 10, Ceiling, "0.3848", "0x0.628", Greater);
    test(
        "0.38",
        "0x0.6",
        100,
        Nearest,
        "0.384396774495639083038194872967",
        "0x0.6267d3b7258b4c13e919701b8",
        Greater,
    );
    test("-0.38", "-0x0.6", 1, Nearest, "-0.50", "-0x0.8", Less);
    test("-0.38", "-0x0.6", 10, Floor, "-0.3848", "-0x0.628", Less);
    test(
        "-0.38", "-0x0.6", 10, Ceiling, "-0.38428", "-0x0.626", Greater,
    );
    test(
        "-0.38",
        "-0x0.6",
        100,
        Nearest,
        "-0.384396774495639083038194872967",
        "-0x0.6267d3b7258b4c13e919701b8",
        Less,
    );
    test("0.9961", "0x0.ff", 1, Nearest, "1.0", "0x1.0", Less);
    test("0.9961", "0x0.ff", 10, Floor, "1.480", "0x1.7b", Less);
    test(
        "0.9961", "0x0.ff", 10, Ceiling, "1.4824", "0x1.7b8", Greater,
    );
    test(
        "0.9961",
        "0x0.ff",
        100,
        Nearest,
        "1.4823791815804471926161460658177",
        "0x1.7b7d33b928c5acdee584023ba",
        Less,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        1,
        Nearest,
        "2.0",
        "0x2.0",
        Greater,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        10,
        Floor,
        "1.570",
        "0x1.92",
        Less,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        10,
        Ceiling,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        100,
        Nearest,
        "1.5707963267125785557334817858064",
        "0x1.921fb543e84f0acf8c9d92d4e",
        Greater,
    );
    test(
        "-0.9999999999999999999966",
        "-0x0.fffffffffffffffff",
        1,
        Nearest,
        "-2.0",
        "-0x2.0",
        Less,
    );
    test(
        "-0.9999999999999999999966",
        "-0x0.fffffffffffffffff",
        10,
        Floor,
        "-1.5723",
        "-0x1.928",
        Less,
    );
    test(
        "-0.9999999999999999999966",
        "-0x0.fffffffffffffffff",
        10,
        Ceiling,
        "-1.570",
        "-0x1.92",
        Greater,
    );
    test(
        "-0.9999999999999999999966",
        "-0x0.fffffffffffffffff",
        100,
        Nearest,
        "-1.5707963267125785557334817858064",
        "-0x1.921fb543e84f0acf8c9d92d4e",
        Less,
    );
    test("3.0", "0x3", 1, Nearest, "NaN", "NaN", Equal);
    test("3.0", "0x3", 10, Floor, "NaN", "NaN", Equal);
    test("3.0", "0x3", 10, Ceiling, "NaN", "NaN", Equal);
    test("3.0", "0x3", 100, Nearest, "NaN", "NaN", Equal);
    test("-1024.5", "-0x400.8", 1, Nearest, "NaN", "NaN", Equal);
    test("-1024.5", "-0x400.8", 10, Floor, "NaN", "NaN", Equal);
    test("-1024.5", "-0x400.8", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1024.5", "-0x400.8", 100, Nearest, "NaN", "NaN", Equal);
    test("7.6e22", "0x1.0E+19", 1, Nearest, "NaN", "NaN", Equal);
    test("7.6e22", "0x1.0E+19", 10, Floor, "NaN", "NaN", Equal);
    test("7.6e22", "0x1.0E+19", 10, Ceiling, "NaN", "NaN", Equal);
    test("7.6e22", "0x1.0E+19", 100, Nearest, "NaN", "NaN", Equal);
    test("1.3e30", "0x1.0E+25", 1, Nearest, "NaN", "NaN", Equal);
    test("1.3e30", "0x1.0E+25", 10, Floor, "NaN", "NaN", Equal);
    test("1.3e30", "0x1.0E+25", 10, Ceiling, "NaN", "NaN", Equal);
    test("1.3e30", "0x1.0E+25", 100, Nearest, "NaN", "NaN", Equal);
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        1,
        Nearest,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Floor,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Ceiling,
        "2.7644e-12041199827",
        "0x1.008E-10000000000",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        100,
        Nearest,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Less,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        1,
        Nearest,
        "-1.1e-12041199826",
        "-0x4.0E-10000000000",
        Less,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Floor,
        "-8.2878e-12041199827",
        "-0x3.01E-10000000000",
        Less,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Ceiling,
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        Greater,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        100,
        Nearest,
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000000002",
        "0x1.000000000000000001",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000000002",
        "0x1.000000000000000001",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000000002",
        "0x1.000000000000000001",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000000002",
        "0x1.000000000000000001",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
}

#[test]
fn test_asin_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = asin_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("0", 10, Exact, "0.0", "0x0.0", Equal);
    test("1", 10, Floor, "1.570", "0x1.92", Less);
    test("-1", 10, Floor, "-1.5723", "-0x1.928", Less);
    test("1/2", 10, Floor, "0.5234", "0x0.86", Less);
    test("2", 10, Exact, "NaN", "NaN", Equal);
    test("1/3", 1, Nearest, "0.25", "0x0.4", Less);
    test("1/3", 10, Floor, "0.33936", "0x0.56e", Less);
    test("1/3", 10, Ceiling, "0.3398", "0x0.57", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "0.33983690945412193709639251339",
        "0x0.56ff8d3c144448b8d4ef3a48",
        Greater,
    );
    test("-2/3", 1, Nearest, "-0.50", "-0x0.8", Greater);
    test("-2/3", 10, Floor, "-0.7305", "-0x0.bb", Less);
    test("-2/3", 10, Ceiling, "-0.72949", "-0x0.bac", Greater);
    test(
        "-2/3",
        100,
        Nearest,
        "-0.7297276562269663634547966598131",
        "-0x0.bacf6e827b46f1e98bbe2bda8",
        Greater,
    );
    test("7/3", 1, Nearest, "NaN", "NaN", Equal);
    test("7/3", 10, Floor, "NaN", "NaN", Equal);
    test("7/3", 10, Ceiling, "NaN", "NaN", Equal);
    test("7/3", 100, Nearest, "NaN", "NaN", Equal);
    test("-7/3", 1, Nearest, "NaN", "NaN", Equal);
    test("-7/3", 10, Floor, "NaN", "NaN", Equal);
    test("-7/3", 10, Ceiling, "NaN", "NaN", Equal);
    test("-7/3", 100, Nearest, "NaN", "NaN", Equal);
    test(
        "1/1000000000000000000000",
        1,
        Nearest,
        "8.5e-22",
        "0x4.0E-18",
        Less,
    );
    test(
        "1/1000000000000000000000",
        10,
        Floor,
        "9.992e-22",
        "0x4.b8E-18",
        Less,
    );
    test(
        "1/1000000000000000000000",
        10,
        Ceiling,
        "1.0009e-21",
        "0x4.baE-18",
        Greater,
    );
    test(
        "1/1000000000000000000000",
        100,
        Nearest,
        "1.0000000000000000000000000000004e-21",
        "0x4.b8ed0283a6d3df769fb7e0b78E-18",
        Greater,
    );
    test("1000000000000000000000/7", 1, Nearest, "NaN", "NaN", Equal);
    test("1000000000000000000000/7", 10, Floor, "NaN", "NaN", Equal);
    test("1000000000000000000000/7", 10, Ceiling, "NaN", "NaN", Equal);
    test(
        "1000000000000000000000/7",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "999999999999/1000000000000",
        1,
        Nearest,
        "2.0",
        "0x2.0",
        Greater,
    );
    test(
        "999999999999/1000000000000",
        10,
        Floor,
        "1.570",
        "0x1.92",
        Less,
    );
    test(
        "999999999999/1000000000000",
        10,
        Ceiling,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "999999999999/1000000000000",
        100,
        Nearest,
        "1.5707949125813342460184217597538",
        "0x1.921f9d8a428ffbfdcc855bcf6",
        Greater,
    );
}

#[test]
fn test_asin_with_period_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                u: u64,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = asin_with_period_prec_round_all(&x, u, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_trig_prec_round_generic(InverseTrigFunction::Asin, &x, Some(u), prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 7, 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 7, 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 7, 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 24, 10, Exact, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", 7, 10, Exact, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8", 24, 10, Exact, "2.0", "0x2.0", Equal);
    test("0.50", "0x0.8", 7, 10, Nearest, "0.58301", "0x0.954", Less);
    test("0.50", "0x0.8", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("-0.50", "-0x0.8", 24, 10, Exact, "-2.0", "-0x2.0", Equal);
    test(
        "-0.50", "-0x0.8", 7, 10, Nearest, "-0.58301", "-0x0.954", Greater,
    );
    test("-0.50", "-0x0.8", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", 24, 10, Exact, "6.0", "0x6.0", Equal);
    test("1.0", "0x1", 7, 10, Exact, "1.8", "0x1.c", Equal);
    test("1.0", "0x1", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1", 24, 10, Exact, "-6.0", "-0x6.0", Equal);
    test("-1.0", "-0x1", 7, 10, Exact, "-1.8", "-0x1.c", Equal);
    test("-1.0", "-0x1", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("2.0", "0x2", 24, 10, Exact, "NaN", "NaN", Equal);
    test("2.0", "0x2", 7, 10, Exact, "NaN", "NaN", Equal);
    test("2.0", "0x2", 0, 10, Exact, "NaN", "NaN", Equal);
    test("-2.0", "-0x2", 24, 10, Exact, "NaN", "NaN", Equal);
    test("-2.0", "-0x2", 7, 10, Exact, "NaN", "NaN", Equal);
    test("-2.0", "-0x2", 0, 10, Exact, "NaN", "NaN", Equal);
    test("0.38", "0x0.6", 7, 10, Nearest, "0.42822", "0x0.6da", Less);
    test("0.38", "0x0.6", 7, 10, Floor, "0.42822", "0x0.6da", Less);
    test(
        "0.38",
        "0x0.6",
        7,
        100,
        Nearest,
        "0.428250527386930938691881740051",
        "0x0.6da1d3999f228914bc2be82b8",
        Less,
    );
    test("-3.0", "-0x3", 7, 10, Nearest, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", 7, 10, Floor, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", 7, 100, Nearest, "NaN", "NaN", Equal);
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        7,
        10,
        Nearest,
        "3.072e-12041199827",
        "0x1.1dE-10000000000",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        7,
        10,
        Floor,
        "3.072e-12041199827",
        "0x1.1dE-10000000000",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        7,
        100,
        Nearest,
        "3.0737628908302917218934856579376e-12041199827",
        "0x1.1d34a60108f725097944d9af6E-10000000000",
        Less,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        360,
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        360,
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        360,
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.8e-12041199827",
        "-0x1.0E-10000000000",
        360,
        10,
        Nearest,
        "-1.5813e-12041199825",
        "-0x3.95E-9999999999",
        Less,
    );
    test(
        "-2.8e-12041199827",
        "-0x1.0E-10000000000",
        360,
        10,
        Floor,
        "-1.5813e-12041199825",
        "-0x3.95E-9999999999",
        Less,
    );
    test(
        "-2.8e-12041199827",
        "-0x1.0E-10000000000",
        360,
        100,
        Nearest,
        "-1.580792343855578599830935481226e-12041199825",
        "-0x3.94bb834c783ef70c2a5d4dfdE-9999999999",
        Greater,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        5,
        10,
        Nearest,
        "1.2",
        "0x1.4",
        Greater,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        5,
        10,
        Floor,
        "1.2480",
        "0x1.3f8",
        Less,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        5,
        100,
        Nearest,
        "1.249999999934493366442831315395",
        "0x1.3fffffffb7f98597a400ad308",
        Greater,
    );
}

#[test]
fn test_asin_with_period_rational_prec_round() {
    let test =
        |x: &str, u: u64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            let x = Rational::from_str(x).unwrap();
            let r = asin_with_period_rational_prec_round_all(&x, u, prec, rm);
            assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
            assert_eq!(r.0.to_string(), out);
            assert_eq!(r.1, o, "{x} {prec} {rm}");
        };
    test("0", 24, 10, Exact, "0.0", "0x0.0", Equal);
    test("0", 7, 10, Exact, "0.0", "0x0.0", Equal);
    test("1", 24, 10, Exact, "6.0", "0x6.0", Equal);
    test("1", 7, 10, Exact, "1.8", "0x1.c", Equal);
    test("1/2", 24, 10, Exact, "2.0", "0x2.0", Equal);
    test("1/2", 7, 10, Nearest, "0.58301", "0x0.954", Less);
    test("-2", 24, 10, Exact, "NaN", "NaN", Equal);
    test("-2", 7, 10, Exact, "NaN", "NaN", Equal);
    test("1/3", 7, 10, Nearest, "0.37842", "0x0.60e", Less);
    test("1/3", 7, 10, Floor, "0.37842", "0x0.60e", Less);
    test(
        "1/3",
        7,
        100,
        Nearest,
        "0.3786070678928745544020055427215",
        "0x0.60ec648ea2649ec6333b3d15a",
        Greater,
    );
    test("-7/3", 7, 10, Nearest, "NaN", "NaN", Equal);
    test("-7/3", 7, 10, Floor, "NaN", "NaN", Equal);
    test("-7/3", 7, 100, Nearest, "NaN", "NaN", Equal);
    test(
        "1/10000000000000000000000000",
        5,
        10,
        Nearest,
        "7.957e-26",
        "0x1.8aE-21",
        Less,
    );
    test(
        "1/10000000000000000000000000",
        5,
        10,
        Floor,
        "7.957e-26",
        "0x1.8aE-21",
        Less,
    );
    test(
        "1/10000000000000000000000000",
        5,
        100,
        Nearest,
        "7.957747154594766788444188168626e-26",
        "0x1.8a0c6df84a6b45ae164ea4f64E-21",
        Greater,
    );
    test(
        "10000000000000000000000000",
        5,
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "10000000000000000000000000",
        5,
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "10000000000000000000000000",
        5,
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test("-1/2", 7, 10, Nearest, "-0.58301", "-0x0.954", Greater);
    test("-1/2", 7, 10, Floor, "-0.5840", "-0x0.958", Less);
    test(
        "-1/2",
        7,
        100,
        Nearest,
        "-0.58333333333333333333333333333307",
        "-0x0.9555555555555555555555555",
        Greater,
    );
}

#[test]
fn test_asin_pi_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = asin_pi_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_trig_prec_round_generic(InverseTrigFunction::Asin, &x, Some(2), prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 10, Exact, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8", 10, Nearest, "0.16675", "0x0.2ab", Greater);
    test("0.50", "0x0.8", 10, Floor, "0.1665", "0x0.2aa", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "0.16666666666666666666666666666673",
        "0x0.2aaaaaaaaaaaaaaaaaaaaaaaac",
        Greater,
    );
    test("-0.50", "-0x0.8", 10, Nearest, "-0.16675", "-0x0.2ab", Less);
    test("-0.50", "-0x0.8", 10, Floor, "-0.16675", "-0x0.2ab", Less);
    test(
        "-0.50",
        "-0x0.8",
        100,
        Nearest,
        "-0.16666666666666666666666666666673",
        "-0x0.2aaaaaaaaaaaaaaaaaaaaaaaac",
        Less,
    );
    test("1.0", "0x1", 10, Exact, "0.50", "0x0.8", Equal);
    test("1.0", "0x1", 10, Floor, "0.50", "0x0.8", Equal);
    test("1.0", "0x1", 100, Exact, "0.50", "0x0.8", Equal);
    test("-1.0", "-0x1", 10, Exact, "-0.50", "-0x0.8", Equal);
    test("-1.0", "-0x1", 10, Floor, "-0.50", "-0x0.8", Equal);
    test("-1.0", "-0x1", 100, Exact, "-0.50", "-0x0.8", Equal);
    test("2.0", "0x2", 10, Exact, "NaN", "NaN", Equal);
    test("2.0", "0x2", 10, Floor, "NaN", "NaN", Equal);
    test("2.0", "0x2", 100, Exact, "NaN", "NaN", Equal);
    test("0.38", "0x0.6", 10, Nearest, "0.1223", "0x0.1f5", Less);
    test("0.38", "0x0.6", 10, Floor, "0.1223", "0x0.1f5", Less);
    test(
        "0.38",
        "0x0.6",
        100,
        Nearest,
        "0.12235729353912312534053764001464",
        "0x0.1f52cebe2d7794e15a55b00c6e",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Nearest,
        "8.784e-12041199828",
        "0x5.18E-10000000001",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Floor,
        "8.7701e-12041199828",
        "0x5.16E-10000000001",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        100,
        Nearest,
        "8.7821796880865477768385304512518e-12041199828",
        "0x5.17cc1b727220a94fe13abe8f8E-10000000001",
        Less,
    );
}

#[test]
fn test_asin_pi_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = asin_pi_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("1/2", 10, Nearest, "0.16675", "0x0.2ab", Greater);
    test("1/2", 10, Floor, "0.1665", "0x0.2aa", Less);
    test(
        "1/2",
        100,
        Nearest,
        "0.16666666666666666666666666666673",
        "0x0.2aaaaaaaaaaaaaaaaaaaaaaaac",
        Greater,
    );
    test("-1/2", 10, Nearest, "-0.16675", "-0x0.2ab", Less);
    test("-1/2", 10, Floor, "-0.16675", "-0x0.2ab", Less);
    test(
        "-1/2",
        100,
        Nearest,
        "-0.16666666666666666666666666666673",
        "-0x0.2aaaaaaaaaaaaaaaaaaaaaaaac",
        Less,
    );
    test("1", 10, Exact, "0.50", "0x0.8", Equal);
    test("1", 10, Floor, "0.50", "0x0.8", Equal);
    test("1", 100, Exact, "0.50", "0x0.8", Equal);
    test("2", 10, Exact, "NaN", "NaN", Equal);
    test("2", 10, Floor, "NaN", "NaN", Equal);
    test("2", 100, Exact, "NaN", "NaN", Equal);
    test("1/3", 10, Nearest, "0.1082", "0x0.1bb", Less);
    test("1/3", 10, Floor, "0.1082", "0x0.1bb", Less);
    test(
        "1/3",
        100,
        Nearest,
        "0.108173447969392729829144440778",
        "0x0.1bb1414d52f82d5d33357f2ac",
        Greater,
    );
    test("-5/7", 10, Nearest, "-0.25342", "-0x0.40e", Less);
    test("-5/7", 10, Floor, "-0.25342", "-0x0.40e", Less);
    test(
        "-5/7",
        100,
        Nearest,
        "-0.253248285571150139032715333042",
        "-0x0.40d4e1304bd10b9945673dc5",
        Greater,
    );
}

#[test]
#[should_panic]
fn asin_prec_round_fail_1() {
    BigExponentFloat::from(0.375).asin_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn asin_prec_round_fail_2() {
    BigExponentFloat::from(0.375).asin_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn asin_prec_round_fail_3() {
    BigExponentFloat::asin_rational_prec_round(Rational::from_unsigneds(1u8, 3), 0, Floor);
}

#[test]
#[should_panic]
fn asin_prec_round_fail_4() {
    BigExponentFloat::asin_rational_prec_round(Rational::from_unsigneds(1u8, 3), 10, Exact);
}

#[test]
#[should_panic]
fn asin_prec_round_fail_5() {
    BigExponentFloat::from(0.375).asin_with_period_prec_round(7, 0, Floor);
}

#[test]
#[should_panic]
fn asin_prec_round_fail_6() {
    BigExponentFloat::from(0.375).asin_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn asin_prec_round_fail_7() {
    BigExponentFloat::asin_with_period_rational_prec_round(
        Rational::from_unsigneds(1u8, 3),
        7,
        0,
        Floor,
    );
}

#[test]
#[should_panic]
fn asin_prec_round_fail_8() {
    BigExponentFloat::from(0.375).asin_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn asin_prec_round_fail_9() {
    BigExponentFloat::from(0.375).asin_pi_prec_round(10, Exact);
}

#[test]
fn asin_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties(
        |(x, prec, rm)| {
            let r = asin_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with the decomposition wherever both apply.
            assert!(same_pair(
                &inverse_trig_prec_round_generic(InverseTrigFunction::Asin, &x, None, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(&asin_prec_round_all(&x, prec, Exact), &r));
            }
            if x.is_finite() && !x.is_zero() && rm != Exact {
                // asin is odd
                let (s, o) = asin_prec_round_all(&-&x, prec, -rm);
                assert!(same(&s, &-&r.0));
                assert_eq!(o, r.1.reverse());
            }
            if rm != Exact
                && big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&asin_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn asin_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        // the `Float` generator's `Exact` cases are not all exact for this function
        if x.is_zero() || rm == Exact {
            return;
        }
        let (y, o) = x.asin_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) {
            let r = asin_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
        if rm != Exact {
            for u in PERIODS {
                let (y, o) = x.asin_with_period_prec_round_ref(u, prec, rm);
                if float_result_comparable(&y, o) {
                    let r = asin_with_period_prec_round_all(
                        &float_to_big_exponent_float(&x),
                        u,
                        prec,
                        rm,
                    );
                    assert!(same(&r.0, &float_to_big_exponent_float(&y)));
                    assert_eq!(r.1, o);
                }
            }
        }
    });
}

#[test]
fn asin_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        let r = asin_rational_prec_round_all(&x, prec, rm);
        check_result(&r, prec);
        let (y, o) = Float::asin_rational_prec_round_ref(&x, prec, rm);
        if float_result_comparable(&y, o) {
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn asin_with_period_prec_round_properties() {
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_9().test_properties(
        |(x, u, prec, rm)| {
            let r = asin_with_period_prec_round_all(&x, u, prec, rm);
            assert!(same_pair(
                &inverse_trig_prec_round_generic(InverseTrigFunction::Asin, &x, Some(u), prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(
                    &asin_with_period_prec_round_all(&x, u, prec, Exact),
                    &r
                ));
            }
            if rm != Exact
                && big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(
                    &asin_with_period_rational_prec_round_all(&q, u, prec, rm),
                    &r
                ));
            }
        },
    );
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties(
        |(x, prec, rm)| {
            asin_with_period_prec_round_all(&x, 2, prec, rm);
        },
    );
}

#[test]
fn asin_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_7().test_properties(
        |(x, u, prec, rm)| {
            let r = asin_with_period_rational_prec_round_all(&x, u, prec, rm);
            check_result(&r, prec);
            let (y, o) = Float::asin_with_period_rational_prec_round_ref(&x, u, prec, rm);
            if float_result_comparable(&y, o) {
                assert!(same(&r.0, &float_to_big_exponent_float(&y)));
                assert_eq!(r.1, o);
            }
        },
    );
}
