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

fn acsc_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().acsc_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.acsc_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.acsc_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().acsc_prec(prec), &r));
        assert!(same_pair(&x.acsc_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.acsc_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn acsc_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::acsc_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::acsc_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::acsc_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::acsc_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

fn acsc_with_period_prec_round_all(
    x: &BigExponentFloat,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().acsc_with_period_prec_round(u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.acsc_with_period_prec_round_ref(u, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.acsc_with_period_prec_round_assign(u, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().acsc_with_period_prec(u, prec), &r));
        assert!(same_pair(&x.acsc_with_period_prec_ref(u, prec), &r));
        let mut z = x.clone();
        let o = z.acsc_with_period_prec_assign(u, prec);
        assert!(same_pair(&(z, o), &r));
    }
    if u == 2 {
        assert!(same_pair(&acsc_pi_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn acsc_with_period_rational_prec_round_all(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::acsc_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::acsc_with_period_rational_prec_round_ref(x, u, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::acsc_with_period_rational_prec(x.clone(), u, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::acsc_with_period_rational_prec_ref(x, u, prec),
            &r
        ));
    }
    if u == 2 {
        assert!(same_pair(&acsc_pi_rational_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn acsc_pi_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().acsc_pi_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.acsc_pi_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.acsc_pi_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().acsc_pi_prec(prec), &r));
        assert!(same_pair(&x.acsc_pi_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.acsc_pi_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn acsc_pi_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::acsc_pi_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::acsc_pi_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::acsc_pi_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::acsc_pi_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_acsc_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = acsc_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_trig_prec_round_generic(InverseTrigFunction::Acsc, &x, None, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", 10, Exact, "NaN", "NaN", Equal);
    test("0.50", "0x0.8", 10, Exact, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 10, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1", 10, Floor, "1.570", "0x1.92", Less);
    test("-1.0", "-0x1", 10, Floor, "-1.5723", "-0x1.928", Less);
    test("2.0", "0x2", 10, Floor, "0.5234", "0x0.86", Less);
    test("-2.0", "-0x2", 10, Floor, "-0.52441", "-0x0.864", Less);
    test("0.38", "0x0.6", 1, Nearest, "NaN", "NaN", Equal);
    test("0.38", "0x0.6", 10, Floor, "NaN", "NaN", Equal);
    test("0.38", "0x0.6", 10, Ceiling, "NaN", "NaN", Equal);
    test("0.38", "0x0.6", 100, Nearest, "NaN", "NaN", Equal);
    test("-0.38", "-0x0.6", 1, Nearest, "NaN", "NaN", Equal);
    test("-0.38", "-0x0.6", 10, Floor, "NaN", "NaN", Equal);
    test("-0.38", "-0x0.6", 10, Ceiling, "NaN", "NaN", Equal);
    test("-0.38", "-0x0.6", 100, Nearest, "NaN", "NaN", Equal);
    test("0.9961", "0x0.ff", 1, Nearest, "NaN", "NaN", Equal);
    test("0.9961", "0x0.ff", 10, Floor, "NaN", "NaN", Equal);
    test("0.9961", "0x0.ff", 10, Ceiling, "NaN", "NaN", Equal);
    test("0.9961", "0x0.ff", 100, Nearest, "NaN", "NaN", Equal);
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.9999999999999999999966",
        "-0x0.fffffffffffffffff",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.9999999999999999999966",
        "-0x0.fffffffffffffffff",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.9999999999999999999966",
        "-0x0.fffffffffffffffff",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.9999999999999999999966",
        "-0x0.fffffffffffffffff",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test("3.0", "0x3", 1, Nearest, "0.25", "0x0.4", Less);
    test("3.0", "0x3", 10, Floor, "0.33936", "0x0.56e", Less);
    test("3.0", "0x3", 10, Ceiling, "0.3398", "0x0.57", Greater);
    test(
        "3.0",
        "0x3",
        100,
        Nearest,
        "0.33983690945412193709639251339",
        "0x0.56ff8d3c144448b8d4ef3a48",
        Greater,
    );
    test(
        "-1024.5", "-0x400.8", 1, Nearest, "-0.00098", "-0x0.004", Less,
    );
    test(
        "-1024.5", "-0x400.8", 10, Floor, "-0.00098", "-0x0.004", Less,
    );
    test(
        "-1024.5",
        "-0x400.8",
        10,
        Ceiling,
        "-0.00097561",
        "-0x0.003ff",
        Greater,
    );
    test(
        "-1024.5",
        "-0x400.8",
        100,
        Nearest,
        "-0.0009760860505521529174075537314367",
        "-0x0.003ff801aa4ac370a41f3f2d5d2",
        Less,
    );
    test(
        "7.6e22",
        "0x1.0E+19",
        1,
        Nearest,
        "1.3e-23",
        "0x1.0E-19",
        Less,
    );
    test(
        "7.6e22",
        "0x1.0E+19",
        10,
        Floor,
        "1.3e-23",
        "0x1.0E-19",
        Less,
    );
    test(
        "7.6e22",
        "0x1.0E+19",
        10,
        Ceiling,
        "1.3261e-23",
        "0x1.008E-19",
        Greater,
    );
    test(
        "7.6e22",
        "0x1.0E+19",
        100,
        Nearest,
        "1.3e-23",
        "0x1.0E-19",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        1,
        Nearest,
        "7.9e-31",
        "0x1.0E-25",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Floor,
        "7.9e-31",
        "0x1.0E-25",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Ceiling,
        "7.9040e-31",
        "0x1.008E-25",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        100,
        Nearest,
        "7.9e-31",
        "0x1.0E-25",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        1,
        Nearest,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Floor,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Ceiling,
        "2.7644e-12041199827",
        "0x1.008E-10000000000",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        100,
        Nearest,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Less,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        1,
        Nearest,
        "6.9e-12041199828",
        "0x4.0E-10000000001",
        Less,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        10,
        Floor,
        "9.188e-12041199828",
        "0x5.54E-10000000001",
        Less,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        10,
        Ceiling,
        "9.2012e-12041199828",
        "0x5.56E-10000000001",
        Greater,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        100,
        Nearest,
        "9.1966770635327334409489410646665e-12041199828",
        "0x5.5555555555555555555555558E-10000000001",
        Greater,
    );
    test(
        "1.0000000000000000000002",
        "0x1.000000000000000001",
        1,
        Nearest,
        "2.0",
        "0x2.0",
        Greater,
    );
    test(
        "1.0000000000000000000002",
        "0x1.000000000000000001",
        10,
        Floor,
        "1.570",
        "0x1.92",
        Less,
    );
    test(
        "1.0000000000000000000002",
        "0x1.000000000000000001",
        10,
        Ceiling,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "1.0000000000000000000002",
        "0x1.000000000000000001",
        100,
        Nearest,
        "1.570796326774317103356861715182",
        "0x1.921fb5442c30e6030a50f8868",
        Greater,
    );
}

#[test]
fn test_acsc_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = acsc_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("0", 10, Exact, "NaN", "NaN", Equal);
    test("1", 10, Floor, "1.570", "0x1.92", Less);
    test("-1", 10, Floor, "-1.5723", "-0x1.928", Less);
    test("1/2", 10, Exact, "NaN", "NaN", Equal);
    test("2", 10, Floor, "0.5234", "0x0.86", Less);
    test("1/3", 1, Nearest, "NaN", "NaN", Equal);
    test("1/3", 10, Floor, "NaN", "NaN", Equal);
    test("1/3", 10, Ceiling, "NaN", "NaN", Equal);
    test("1/3", 100, Nearest, "NaN", "NaN", Equal);
    test("-2/3", 1, Nearest, "NaN", "NaN", Equal);
    test("-2/3", 10, Floor, "NaN", "NaN", Equal);
    test("-2/3", 10, Ceiling, "NaN", "NaN", Equal);
    test("-2/3", 100, Nearest, "NaN", "NaN", Equal);
    test("7/3", 1, Nearest, "0.50", "0x0.8", Greater);
    test("7/3", 10, Floor, "0.44287", "0x0.716", Less);
    test("7/3", 10, Ceiling, "0.4434", "0x0.718", Greater);
    test(
        "7/3",
        100,
        Nearest,
        "0.44291104407363893384365934727225",
        "0x0.71629e41555fda217d40230be8",
        Less,
    );
    test("-7/3", 1, Nearest, "-0.50", "-0x0.8", Less);
    test("-7/3", 10, Floor, "-0.4434", "-0x0.718", Less);
    test("-7/3", 10, Ceiling, "-0.44287", "-0x0.716", Greater);
    test(
        "-7/3",
        100,
        Nearest,
        "-0.44291104407363893384365934727225",
        "-0x0.71629e41555fda217d40230be8",
        Greater,
    );
    test("1/1000000000000000000000", 1, Nearest, "NaN", "NaN", Equal);
    test("1/1000000000000000000000", 10, Floor, "NaN", "NaN", Equal);
    test("1/1000000000000000000000", 10, Ceiling, "NaN", "NaN", Equal);
    test(
        "1/1000000000000000000000",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1000000000000000000000/7",
        1,
        Nearest,
        "6.8e-21",
        "0x2.0E-17",
        Less,
    );
    test(
        "1000000000000000000000/7",
        10,
        Floor,
        "6.99e-21",
        "0x2.1E-17",
        Less,
    );
    test(
        "1000000000000000000000/7",
        10,
        Ceiling,
        "7.0013e-21",
        "0x2.11E-17",
        Greater,
    );
    test(
        "1000000000000000000000/7",
        100,
        Nearest,
        "7.0000000000000000000000000000011e-21",
        "0x2.10e7b11998fcb1c3e5e072504E-17",
        Greater,
    );
    test(
        "999999999999/1000000000000",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test("999999999999/1000000000000", 10, Floor, "NaN", "NaN", Equal);
    test(
        "999999999999/1000000000000",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "999999999999/1000000000000",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
}

#[test]
fn test_acsc_with_period_prec_round() {
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
        let r = acsc_with_period_prec_round_all(&x, u, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_trig_prec_round_generic(InverseTrigFunction::Acsc, &x, Some(u), prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 7, 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 7, 10, Floor, "0.0", "0x0.0", Equal);
    test(
        "-Infinity",
        "-Infinity",
        7,
        10,
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    test("0.0", "0x0", 24, 10, Exact, "NaN", "NaN", Equal);
    test("0.0", "0x0", 7, 10, Exact, "NaN", "NaN", Equal);
    test("0.0", "0x0", 0, 10, Exact, "NaN", "NaN", Equal);
    test("0.50", "0x0.8", 24, 10, Exact, "NaN", "NaN", Equal);
    test("0.50", "0x0.8", 7, 10, Exact, "NaN", "NaN", Equal);
    test("0.50", "0x0.8", 0, 10, Exact, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 24, 10, Exact, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 7, 10, Exact, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 0, 10, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1", 24, 10, Exact, "6.0", "0x6.0", Equal);
    test("1.0", "0x1", 7, 10, Exact, "1.8", "0x1.c", Equal);
    test("1.0", "0x1", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1", 24, 10, Exact, "-6.0", "-0x6.0", Equal);
    test("-1.0", "-0x1", 7, 10, Exact, "-1.8", "-0x1.c", Equal);
    test("-1.0", "-0x1", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("2.0", "0x2", 24, 10, Exact, "2.0", "0x2.0", Equal);
    test("2.0", "0x2", 7, 10, Nearest, "0.58301", "0x0.954", Less);
    test("2.0", "0x2", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2", 24, 10, Exact, "-2.0", "-0x2.0", Equal);
    test(
        "-2.0", "-0x2", 7, 10, Nearest, "-0.58301", "-0x0.954", Greater,
    );
    test("-2.0", "-0x2", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("0.38", "0x0.6", 7, 10, Nearest, "NaN", "NaN", Equal);
    test("0.38", "0x0.6", 7, 10, Floor, "NaN", "NaN", Equal);
    test("0.38", "0x0.6", 7, 100, Nearest, "NaN", "NaN", Equal);
    test(
        "-3.0", "-0x3", 7, 10, Nearest, "-0.37842", "-0x0.60e", Greater,
    );
    test("-3.0", "-0x3", 7, 10, Floor, "-0.3789", "-0x0.61", Less);
    test(
        "-3.0",
        "-0x3",
        7,
        100,
        Nearest,
        "-0.3786070678928745544020055427215",
        "-0x0.60ec648ea2649ec6333b3d15a",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        10,
        Nearest,
        "3.072e-12041199827",
        "0x1.1dE-10000000000",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        10,
        Floor,
        "3.072e-12041199827",
        "0x1.1dE-10000000000",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        100,
        Nearest,
        "3.0737628908302917218934856579376e-12041199827",
        "0x1.1d34a60108f725097944d9af6E-10000000000",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        7,
        10,
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
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        7,
        100,
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
        Nearest,
        "5.2680e-12041199826",
        "0x1.318E-9999999999",
        Less,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        360,
        10,
        Floor,
        "5.2680e-12041199826",
        "0x1.318E-9999999999",
        Less,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        360,
        100,
        Nearest,
        "5.2693078128519286661031182707506e-12041199826",
        "0x1.3193d66ed2bfa7aeb8c9c4a9aE-9999999999",
        Less,
    );
    test(
        "-2.8e-12041199827",
        "-0x1.0E-10000000000",
        360,
        10,
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
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.8e-12041199827",
        "-0x1.0E-10000000000",
        360,
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        5,
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        5,
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        5,
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
}

#[test]
fn test_acsc_with_period_rational_prec_round() {
    let test =
        |x: &str, u: u64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            let x = Rational::from_str(x).unwrap();
            let r = acsc_with_period_rational_prec_round_all(&x, u, prec, rm);
            assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
            assert_eq!(r.0.to_string(), out);
            assert_eq!(r.1, o, "{x} {prec} {rm}");
        };
    test("0", 24, 10, Exact, "NaN", "NaN", Equal);
    test("0", 7, 10, Exact, "NaN", "NaN", Equal);
    test("1", 24, 10, Exact, "6.0", "0x6.0", Equal);
    test("1", 7, 10, Exact, "1.8", "0x1.c", Equal);
    test("1/2", 24, 10, Exact, "NaN", "NaN", Equal);
    test("1/2", 7, 10, Exact, "NaN", "NaN", Equal);
    test("-2", 24, 10, Exact, "-2.0", "-0x2.0", Equal);
    test("-2", 7, 10, Nearest, "-0.58301", "-0x0.954", Greater);
    test("1/3", 7, 10, Nearest, "NaN", "NaN", Equal);
    test("1/3", 7, 10, Floor, "NaN", "NaN", Equal);
    test("1/3", 7, 100, Nearest, "NaN", "NaN", Equal);
    test("-7/3", 7, 10, Nearest, "-0.49365", "-0x0.7e6", Less);
    test("-7/3", 7, 10, Floor, "-0.49365", "-0x0.7e6", Less);
    test(
        "-7/3",
        7,
        100,
        Nearest,
        "-0.49344037410018366000553609459417",
        "-0x0.7e521bbd494a946d42d0e4bd08",
        Less,
    );
    test(
        "1/10000000000000000000000000",
        5,
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1/10000000000000000000000000",
        5,
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1/10000000000000000000000000",
        5,
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "10000000000000000000000000",
        5,
        10,
        Nearest,
        "7.957e-26",
        "0x1.8aE-21",
        Less,
    );
    test(
        "10000000000000000000000000",
        5,
        10,
        Floor,
        "7.957e-26",
        "0x1.8aE-21",
        Less,
    );
    test(
        "10000000000000000000000000",
        5,
        100,
        Nearest,
        "7.957747154594766788444188168626e-26",
        "0x1.8a0c6df84a6b45ae164ea4f64E-21",
        Greater,
    );
    test("-1/2", 7, 10, Nearest, "NaN", "NaN", Equal);
    test("-1/2", 7, 10, Floor, "NaN", "NaN", Equal);
    test("-1/2", 7, 100, Nearest, "NaN", "NaN", Equal);
}

#[test]
fn test_acsc_pi_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = acsc_pi_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_trig_prec_round_generic(InverseTrigFunction::Acsc, &x, Some(2), prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", 10, Exact, "NaN", "NaN", Equal);
    test("0.50", "0x0.8", 10, Exact, "NaN", "NaN", Equal);
    test("0.50", "0x0.8", 10, Floor, "NaN", "NaN", Equal);
    test("0.50", "0x0.8", 100, Exact, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 10, Exact, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 10, Floor, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 100, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1", 10, Exact, "0.50", "0x0.8", Equal);
    test("1.0", "0x1", 10, Floor, "0.50", "0x0.8", Equal);
    test("1.0", "0x1", 100, Exact, "0.50", "0x0.8", Equal);
    test("-1.0", "-0x1", 10, Exact, "-0.50", "-0x0.8", Equal);
    test("-1.0", "-0x1", 10, Floor, "-0.50", "-0x0.8", Equal);
    test("-1.0", "-0x1", 100, Exact, "-0.50", "-0x0.8", Equal);
    test("2.0", "0x2", 10, Nearest, "0.16675", "0x0.2ab", Greater);
    test("2.0", "0x2", 10, Floor, "0.1665", "0x0.2aa", Less);
    test(
        "2.0",
        "0x2",
        100,
        Nearest,
        "0.16666666666666666666666666666673",
        "0x0.2aaaaaaaaaaaaaaaaaaaaaaaac",
        Greater,
    );
    test("0.38", "0x0.6", 10, Exact, "NaN", "NaN", Equal);
    test("0.38", "0x0.6", 10, Floor, "NaN", "NaN", Equal);
    test("0.38", "0x0.6", 100, Exact, "NaN", "NaN", Equal);
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Nearest,
        "8.784e-12041199828",
        "0x5.18E-10000000001",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Floor,
        "8.7701e-12041199828",
        "0x5.16E-10000000001",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        100,
        Nearest,
        "8.7821796880865477768385304512518e-12041199828",
        "0x5.17cc1b727220a94fe13abe8f8E-10000000001",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
}

#[test]
fn test_acsc_pi_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = acsc_pi_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("1/2", 10, Exact, "NaN", "NaN", Equal);
    test("1/2", 10, Floor, "NaN", "NaN", Equal);
    test("1/2", 100, Exact, "NaN", "NaN", Equal);
    test("-1/2", 10, Exact, "NaN", "NaN", Equal);
    test("-1/2", 10, Floor, "NaN", "NaN", Equal);
    test("-1/2", 100, Exact, "NaN", "NaN", Equal);
    test("1", 10, Exact, "0.50", "0x0.8", Equal);
    test("1", 10, Floor, "0.50", "0x0.8", Equal);
    test("1", 100, Exact, "0.50", "0x0.8", Equal);
    test("2", 10, Nearest, "0.16675", "0x0.2ab", Greater);
    test("2", 10, Floor, "0.1665", "0x0.2aa", Less);
    test(
        "2",
        100,
        Nearest,
        "0.16666666666666666666666666666673",
        "0x0.2aaaaaaaaaaaaaaaaaaaaaaaac",
        Greater,
    );
    test("1/3", 10, Exact, "NaN", "NaN", Equal);
    test("1/3", 10, Floor, "NaN", "NaN", Equal);
    test("1/3", 100, Exact, "NaN", "NaN", Equal);
    test("-5/7", 10, Exact, "NaN", "NaN", Equal);
    test("-5/7", 10, Floor, "NaN", "NaN", Equal);
    test("-5/7", 100, Exact, "NaN", "NaN", Equal);
}

#[test]
#[should_panic]
fn acsc_prec_round_fail_1() {
    BigExponentFloat::from(3).acsc_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn acsc_prec_round_fail_2() {
    BigExponentFloat::from(3).acsc_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn acsc_prec_round_fail_3() {
    BigExponentFloat::acsc_rational_prec_round(Rational::from(3), 0, Floor);
}

#[test]
#[should_panic]
fn acsc_prec_round_fail_4() {
    BigExponentFloat::acsc_rational_prec_round(Rational::from(3), 10, Exact);
}

#[test]
#[should_panic]
fn acsc_prec_round_fail_5() {
    BigExponentFloat::from(3).acsc_with_period_prec_round(7, 0, Floor);
}

#[test]
#[should_panic]
fn acsc_prec_round_fail_6() {
    BigExponentFloat::from(3).acsc_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn acsc_prec_round_fail_7() {
    BigExponentFloat::acsc_with_period_rational_prec_round(Rational::from(3), 7, 0, Floor);
}

#[test]
#[should_panic]
fn acsc_prec_round_fail_8() {
    BigExponentFloat::from(3).acsc_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn acsc_prec_round_fail_9() {
    BigExponentFloat::from(3).acsc_pi_prec_round(10, Exact);
}

#[test]
fn acsc_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties(
        |(x, prec, rm)| {
            let r = acsc_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with the decomposition wherever both apply.
            assert!(same_pair(
                &inverse_trig_prec_round_generic(InverseTrigFunction::Acsc, &x, None, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(&acsc_prec_round_all(&x, prec, Exact), &r));
            }
            if x.is_finite() && !x.is_zero() && rm != Exact {
                // acsc is odd
                let (s, o) = acsc_prec_round_all(&-&x, prec, -rm);
                assert!(same(&s, &-&r.0));
                assert_eq!(o, r.1.reverse());
            }
            if rm != Exact
                && big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&acsc_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn acsc_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_49().test_properties(|(x, prec, rm)| {
        // the `Float` generator's `Exact` cases are not all exact for this function
        if x.is_zero() || rm == Exact {
            return;
        }
        let (y, o) = x.acsc_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) {
            let r = acsc_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
        if rm != Exact {
            for u in PERIODS {
                let (y, o) = x.acsc_with_period_prec_round_ref(u, prec, rm);
                if float_result_comparable(&y, o) {
                    let r = acsc_with_period_prec_round_all(
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
fn acsc_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_13().test_properties(|(x, prec, rm)| {
        let r = acsc_rational_prec_round_all(&x, prec, rm);
        check_result(&r, prec);
        let (y, o) = Float::acsc_rational_prec_round_ref(&x, prec, rm);
        if float_result_comparable(&y, o) {
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn acsc_with_period_prec_round_properties() {
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_9().test_properties(
        |(x, u, prec, rm)| {
            let r = acsc_with_period_prec_round_all(&x, u, prec, rm);
            assert!(same_pair(
                &inverse_trig_prec_round_generic(InverseTrigFunction::Acsc, &x, Some(u), prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(
                    &acsc_with_period_prec_round_all(&x, u, prec, Exact),
                    &r
                ));
            }
            if rm != Exact
                && big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(
                    &acsc_with_period_rational_prec_round_all(&q, u, prec, rm),
                    &r
                ));
            }
        },
    );
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties(
        |(x, prec, rm)| {
            acsc_with_period_prec_round_all(&x, 2, prec, rm);
        },
    );
}

#[test]
fn acsc_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_10().test_properties(
        |(x, u, prec, rm)| {
            let r = acsc_with_period_rational_prec_round_all(&x, u, prec, rm);
            check_result(&r, prec);
            let (y, o) = Float::acsc_with_period_rational_prec_round_ref(&x, u, prec, rm);
            if float_result_comparable(&y, o) {
                assert!(same(&r.0, &float_to_big_exponent_float(&y)));
                assert_eq!(r.1, o);
            }
        },
    );
}
