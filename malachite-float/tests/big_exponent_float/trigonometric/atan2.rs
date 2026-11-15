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
use malachite_float::big_exponent_float::trigonometric::atan::atan2_prec_round_generic;
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

fn atan2_prec_round_all(
    y: &BigExponentFloat,
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = y.clone().atan2_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &y.clone().atan2_prec_round_val_ref(x, prec, rm),
        &r
    ));
    assert!(same_pair(
        &y.atan2_prec_round_ref_val(x.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(&y.atan2_prec_round_ref_ref(x, prec, rm), &r));
    let mut z = y.clone();
    let o = z.atan2_prec_round_assign(x.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = y.clone();
    let o = z.atan2_prec_round_assign_ref(x, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&y.clone().atan2_prec(x.clone(), prec), &r));
        assert!(same_pair(&y.clone().atan2_prec_val_ref(x, prec), &r));
        assert!(same_pair(&y.atan2_prec_ref_val(x.clone(), prec), &r));
        assert!(same_pair(&y.atan2_prec_ref_ref(x, prec), &r));
        let mut z = y.clone();
        let o = z.atan2_prec_assign(x.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = y.clone();
        let o = z.atan2_prec_assign_ref(x, prec);
        assert!(same_pair(&(z, o), &r));
    }
    assert!(same_pair(
        &atan2_prec_round_generic(y, x, None, prec, rm),
        &r
    ));
    r
}

fn atan2_rational_prec_round_all(
    y: &Rational,
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::atan2_rational_prec_round(y.clone(), x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::atan2_rational_prec_round_val_ref(y.clone(), x, prec, rm),
        &r
    ));
    assert!(same_pair(
        &BigExponentFloat::atan2_rational_prec_round_ref_val(y, x.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &BigExponentFloat::atan2_rational_prec_round_ref_ref(y, x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::atan2_rational_prec(y.clone(), x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::atan2_rational_prec_ref_ref(y, x, prec),
            &r
        ));
    }
    r
}

fn atan2_with_period_prec_round_all(
    y: &BigExponentFloat,
    x: &BigExponentFloat,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = y
        .clone()
        .atan2_with_period_prec_round(x.clone(), u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &y.clone()
            .atan2_with_period_prec_round_val_ref(x, u, prec, rm),
        &r
    ));
    assert!(same_pair(
        &y.atan2_with_period_prec_round_ref_val(x.clone(), u, prec, rm),
        &r
    ));
    assert!(same_pair(
        &y.atan2_with_period_prec_round_ref_ref(x, u, prec, rm),
        &r
    ));
    let mut z = y.clone();
    let o = z.atan2_with_period_prec_round_assign(x.clone(), u, prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = y.clone();
    let o = z.atan2_with_period_prec_round_assign_ref(x, u, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(
            &y.clone().atan2_with_period_prec(x.clone(), u, prec),
            &r
        ));
        assert!(same_pair(
            &y.clone().atan2_with_period_prec_val_ref(x, u, prec),
            &r
        ));
        assert!(same_pair(
            &y.atan2_with_period_prec_ref_val(x.clone(), u, prec),
            &r
        ));
        assert!(same_pair(&y.atan2_with_period_prec_ref_ref(x, u, prec), &r));
        let mut z = y.clone();
        let o = z.atan2_with_period_prec_assign(x.clone(), u, prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = y.clone();
        let o = z.atan2_with_period_prec_assign_ref(x, u, prec);
        assert!(same_pair(&(z, o), &r));
    }
    assert!(same_pair(
        &atan2_prec_round_generic(y, x, Some(u), prec, rm),
        &r
    ));
    r
}

fn atan2_with_period_rational_prec_round_all(
    y: &Rational,
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r =
        BigExponentFloat::atan2_with_period_rational_prec_round(y.clone(), x.clone(), u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::atan2_with_period_rational_prec_round_val_ref(y.clone(), x, u, prec, rm),
        &r
    ));
    assert!(same_pair(
        &BigExponentFloat::atan2_with_period_rational_prec_round_ref_val(y, x.clone(), u, prec, rm),
        &r
    ));
    assert!(same_pair(
        &BigExponentFloat::atan2_with_period_rational_prec_round_ref_ref(y, x, u, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::atan2_with_period_rational_prec(y.clone(), x.clone(), u, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::atan2_with_period_rational_prec_ref_ref(y, x, u, prec),
            &r
        ));
    }
    r
}

fn atan2_pi_prec_round_all(
    y: &BigExponentFloat,
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = y.clone().atan2_pi_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &y.clone().atan2_pi_prec_round_val_ref(x, prec, rm),
        &r
    ));
    assert!(same_pair(
        &y.atan2_pi_prec_round_ref_val(x.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(&y.atan2_pi_prec_round_ref_ref(x, prec, rm), &r));
    let mut z = y.clone();
    let o = z.atan2_pi_prec_round_assign(x.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = y.clone();
    let o = z.atan2_pi_prec_round_assign_ref(x, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&y.clone().atan2_pi_prec(x.clone(), prec), &r));
        assert!(same_pair(&y.clone().atan2_pi_prec_val_ref(x, prec), &r));
        assert!(same_pair(&y.atan2_pi_prec_ref_val(x.clone(), prec), &r));
        assert!(same_pair(&y.atan2_pi_prec_ref_ref(x, prec), &r));
        let mut z = y.clone();
        let o = z.atan2_pi_prec_assign(x.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = y.clone();
        let o = z.atan2_pi_prec_assign_ref(x, prec);
        assert!(same_pair(&(z, o), &r));
    }
    assert!(same_pair(
        &atan2_prec_round_generic(y, x, Some(2), prec, rm),
        &r
    ));
    r
}

fn atan2_pi_rational_prec_round_all(
    y: &Rational,
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::atan2_pi_rational_prec_round(y.clone(), x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::atan2_pi_rational_prec_round_val_ref(y.clone(), x, prec, rm),
        &r
    ));
    assert!(same_pair(
        &BigExponentFloat::atan2_pi_rational_prec_round_ref_val(y, x.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &BigExponentFloat::atan2_pi_rational_prec_round_ref_ref(y, x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::atan2_pi_rational_prec(y.clone(), x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::atan2_pi_rational_prec_ref_ref(y, x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_atan2_prec_round() {
    let test = |y: &str,
                y_hex: &str,
                x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(y_hex).to_string(), y);
        assert_eq!(hex(x_hex).to_string(), x);
        let y = hex(y_hex);
        let x = hex(x_hex);
        let r = atan2_prec_round_all(&y, &x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{y} {x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{y} {x} {prec} {rm}");
    };
    test("NaN", "NaN", "1.0", "0x1", 10, Floor, "NaN", "NaN", Equal);
    test("1.0", "0x1", "NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", "1.0", "0x1", 10, Floor, "1.570", "0x1.92", Less,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 10, Floor, "0.7852", "0x0.c9", Less,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        10,
        Floor,
        "2.3555",
        "0x2.5b",
        Less,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        10,
        Floor,
        "-2.359",
        "-0x2.5c",
        Less,
    );
    test(
        "1.0", "0x1", "Infinity", "Infinity", 10, Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "1.0",
        "0x1",
        "-Infinity",
        "-Infinity",
        10,
        Floor,
        "3.141",
        "0x3.24",
        Less,
    );
    test(
        "-1.0",
        "-0x1",
        "-Infinity",
        "-Infinity",
        10,
        Floor,
        "-3.1445",
        "-0x3.25",
        Less,
    );
    test(
        "0.0",
        "0x0",
        "-Infinity",
        "-Infinity",
        10,
        Floor,
        "3.141",
        "0x3.24",
        Less,
    );
    test("0.0", "0x0", "1.0", "0x1", 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "0.0", "0x0", "-1.0", "-0x1", 10, Floor, "3.141", "0x3.24", Less,
    );
    test("0.0", "0x0", "0.0", "0x0", 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "1.0", "0x1", "0.0", "0x0", 10, Floor, "1.570", "0x1.92", Less,
    );
    test(
        "-1.0", "-0x1", "0.0", "0x0", 10, Floor, "-1.5723", "-0x1.928", Less,
    );
    test(
        "1.0", "0x1", "1.0", "0x1", 10, Floor, "0.7852", "0x0.c9", Less,
    );
    test(
        "-1.0", "-0x1", "-1.0", "-0x1", 10, Floor, "-2.359", "-0x2.5c", Less,
    );
    test(
        "1.0", "0x1", "-1.0", "-0x1", 10, Floor, "2.3555", "0x2.5b", Less,
    );
    test(
        "3.0", "0x3", "-2.0", "-0x2", 1, Nearest, "2.0", "0x2.0", Less,
    );
    test(
        "3.0", "0x3", "-2.0", "-0x2", 10, Floor, "2.156", "0x2.28", Less,
    );
    test(
        "3.0", "0x3", "-2.0", "-0x2", 10, Ceiling, "2.1602", "0x2.29", Greater,
    );
    test(
        "3.0",
        "0x3",
        "-2.0",
        "-0x2",
        100,
        Nearest,
        "2.1587989303424641704769327722645",
        "0x2.28a70bf475ee2fd29a91db214",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "1.0",
        "0x1",
        1,
        Nearest,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "1.0",
        "0x1",
        10,
        Floor,
        "2.7563e-12041199827",
        "0xf.fcE-10000000001",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "1.0",
        "0x1",
        10,
        Ceiling,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "1.0",
        "0x1",
        100,
        Nearest,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Greater,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        "1.0",
        "0x1",
        1,
        Nearest,
        "5.5e-12041199827",
        "0x2.0E-10000000000",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        "1.0",
        "0x1",
        10,
        Floor,
        "8.2662e-12041199827",
        "0x2.ffE-10000000000",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        "1.0",
        "0x1",
        10,
        Ceiling,
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        Greater,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        "1.0",
        "0x1",
        100,
        Nearest,
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "-1.0",
        "-0x1",
        1,
        Nearest,
        "4.0",
        "0x4.0",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "-1.0",
        "-0x1",
        10,
        Floor,
        "3.141",
        "0x3.24",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "-1.0",
        "-0x1",
        10,
        Ceiling,
        "3.1445",
        "0x3.25",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "-1.0",
        "-0x1",
        100,
        Nearest,
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        "3.0",
        "0x3",
        1,
        Nearest,
        "2.0",
        "0x2.0",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        "3.0",
        "0x3",
        10,
        Floor,
        "1.570",
        "0x1.92",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        "3.0",
        "0x3",
        100,
        Nearest,
        "1.570796326794896619231321691640",
        "0x1.921fb54442d18469898cc517",
        Less,
    );
    test(
        "-3.6e12041199826",
        "-0x1.0E+10000000000",
        "-3.0",
        "-0x3",
        1,
        Nearest,
        "-2.0",
        "-0x2.0",
        Less,
    );
    test(
        "-3.6e12041199826",
        "-0x1.0E+10000000000",
        "-3.0",
        "-0x3",
        10,
        Floor,
        "-1.5723",
        "-0x1.928",
        Less,
    );
    test(
        "-3.6e12041199826",
        "-0x1.0E+10000000000",
        "-3.0",
        "-0x3",
        10,
        Ceiling,
        "-1.570",
        "-0x1.92",
        Greater,
    );
    test(
        "-3.6e12041199826",
        "-0x1.0E+10000000000",
        "-3.0",
        "-0x3",
        100,
        Nearest,
        "-1.570796326794896619231321691640",
        "-0x1.921fb54442d18469898cc517",
        Greater,
    );
    test(
        "6.3e30",
        "0x5.0E+25",
        "2.4e29",
        "0x3.0E+24",
        1,
        Nearest,
        "2.0",
        "0x2.0",
        Greater,
    );
    test(
        "6.3e30",
        "0x5.0E+25",
        "2.4e29",
        "0x3.0E+24",
        10,
        Floor,
        "1.5332",
        "0x1.888",
        Less,
    );
    test(
        "6.3e30",
        "0x5.0E+25",
        "2.4e29",
        "0x3.0E+24",
        10,
        Ceiling,
        "1.535",
        "0x1.89",
        Greater,
    );
    test(
        "6.3e30",
        "0x5.0E+25",
        "2.4e29",
        "0x3.0E+24",
        100,
        Nearest,
        "1.5333138901032351300012623742424",
        "0x1.888742547ef5762d08bf3d392",
        Less,
    );
    test(
        "-7.0",
        "-0x7",
        "7.6e-1205",
        "0x1.0E-1000",
        1,
        Nearest,
        "-2.0",
        "-0x2.0",
        Less,
    );
    test(
        "-7.0",
        "-0x7",
        "7.6e-1205",
        "0x1.0E-1000",
        10,
        Floor,
        "-1.5723",
        "-0x1.928",
        Less,
    );
    test(
        "-7.0",
        "-0x7",
        "7.6e-1205",
        "0x1.0E-1000",
        10,
        Ceiling,
        "-1.570",
        "-0x1.92",
        Greater,
    );
    test(
        "-7.0",
        "-0x7",
        "7.6e-1205",
        "0x1.0E-1000",
        100,
        Nearest,
        "-1.570796326794896619231321691640",
        "-0x1.921fb54442d18469898cc517",
        Greater,
    );
}

#[test]
fn test_atan2_with_period_prec_round() {
    let test = |y: &str,
                y_hex: &str,
                x: &str,
                x_hex: &str,
                u: u64,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(y_hex).to_string(), y);
        assert_eq!(hex(x_hex).to_string(), x);
        let y = hex(y_hex);
        let x = hex(x_hex);
        let r = atan2_with_period_prec_round_all(&y, &x, u, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{y} {x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{y} {x} {prec} {rm}");
    };
    test(
        "0.0", "0x0", "1.0", "0x1", 8, 10, Exact, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0", "1.0", "0x1", 7, 10, Exact, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0", "1.0", "0x1", 0, 10, Exact, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0", "-1.0", "-0x1", 8, 10, Exact, "4.0", "0x4.0", Equal,
    );
    test(
        "0.0", "0x0", "-1.0", "-0x1", 7, 10, Exact, "3.5", "0x3.8", Equal,
    );
    test(
        "0.0", "0x0", "-1.0", "-0x1", 0, 10, Exact, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0", "0.0", "0x0", 8, 10, Exact, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0", "0.0", "0x0", 7, 10, Exact, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0", "0.0", "0x0", 0, 10, Exact, "0.0", "0x0.0", Equal,
    );
    test(
        "1.0", "0x1", "0.0", "0x0", 8, 10, Exact, "2.0", "0x2.0", Equal,
    );
    test(
        "1.0", "0x1", "0.0", "0x0", 7, 10, Exact, "1.8", "0x1.c", Equal,
    );
    test(
        "1.0", "0x1", "0.0", "0x0", 0, 10, Exact, "0.0", "0x0.0", Equal,
    );
    test(
        "-1.0", "-0x1", "0.0", "0x0", 8, 10, Exact, "-2.0", "-0x2.0", Equal,
    );
    test(
        "-1.0", "-0x1", "0.0", "0x0", 7, 10, Exact, "-1.8", "-0x1.c", Equal,
    );
    test(
        "-1.0", "-0x1", "0.0", "0x0", 0, 10, Exact, "0.0", "0x0.0", Equal,
    );
    test(
        "1.0", "0x1", "1.0", "0x1", 8, 10, Exact, "1.0", "0x1.0", Equal,
    );
    test(
        "1.0", "0x1", "1.0", "0x1", 7, 10, Exact, "0.88", "0x0.e", Equal,
    );
    test(
        "1.0", "0x1", "1.0", "0x1", 0, 10, Exact, "0.0", "0x0.0", Equal,
    );
    test(
        "-1.0", "-0x1", "-1.0", "-0x1", 8, 10, Exact, "-3.0", "-0x3.0", Equal,
    );
    test(
        "-1.0", "-0x1", "-1.0", "-0x1", 7, 10, Exact, "-2.62", "-0x2.a", Equal,
    );
    test(
        "-1.0", "-0x1", "-1.0", "-0x1", 0, 10, Exact, "0.0", "0x0.0", Equal,
    );
    test(
        "1.0", "0x1", "-1.0", "-0x1", 8, 10, Exact, "3.0", "0x3.0", Equal,
    );
    test(
        "1.0", "0x1", "-1.0", "-0x1", 7, 10, Exact, "2.62", "0x2.a", Equal,
    );
    test(
        "1.0", "0x1", "-1.0", "-0x1", 0, 10, Exact, "0.0", "0x0.0", Equal,
    );
    test(
        "3.0", "0x3", "-2.0", "-0x2", 7, 10, Nearest, "2.406", "0x2.68", Greater,
    );
    test(
        "3.0", "0x3", "-2.0", "-0x2", 7, 10, Floor, "2.4023", "0x2.67", Less,
    );
    test(
        "3.0",
        "0x3",
        "-2.0",
        "-0x2",
        7,
        100,
        Nearest,
        "2.405084646338495856651884616476",
        "0x2.67b3a09c22b4718cdabf188a",
        Less,
    );
    test(
        "3.0", "0x3", "-2.0", "-0x2", 360, 10, Nearest, "123.8", "0x7b.c", Greater,
    );
    test(
        "3.0", "0x3", "-2.0", "-0x2", 360, 10, Floor, "123.62", "0x7b.a", Less,
    );
    test(
        "3.0",
        "0x3",
        "-2.0",
        "-0x2",
        360,
        100,
        Nearest,
        "123.6900675259797869135254945616",
        "0x7b.b0a843f01d6486951663c97",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "1.0",
        "0x1",
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
        "1.0",
        "0x1",
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
        "1.0",
        "0x1",
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
        "1.0",
        "0x1",
        360,
        10,
        Nearest,
        "1.5813e-12041199825",
        "0x3.95E-9999999999",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "1.0",
        "0x1",
        360,
        10,
        Floor,
        "1.580e-12041199825",
        "0x3.94E-9999999999",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "1.0",
        "0x1",
        360,
        100,
        Nearest,
        "1.580792343855578599830935481226e-12041199825",
        "0x3.94bb834c783ef70c2a5d4dfdE-9999999999",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        "1.0",
        "0x1",
        7,
        10,
        Nearest,
        "9.225e-12041199827",
        "0x3.58E-10000000000",
        Greater,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        "1.0",
        "0x1",
        7,
        10,
        Floor,
        "9.2146e-12041199827",
        "0x3.57E-10000000000",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        "1.0",
        "0x1",
        7,
        100,
        Nearest,
        "9.2212886724908751656804569738171e-12041199827",
        "0x3.579df2031ae56f1c6bce8d0e4E-10000000000",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        "1.0",
        "0x1",
        360,
        10,
        Nearest,
        "4.75e-12041199825",
        "0xa.cE-9999999999",
        Greater,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        "1.0",
        "0x1",
        360,
        10,
        Floor,
        "4.7386e-12041199825",
        "0xa.bcE-9999999999",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        "1.0",
        "0x1",
        360,
        100,
        Nearest,
        "4.7423770315667357994928064436777e-12041199825",
        "0xa.be3289e568bce5247f17e9f7E-9999999999",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "-1.0",
        "-0x1",
        7,
        10,
        Nearest,
        "3.5",
        "0x3.8",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "-1.0",
        "-0x1",
        7,
        10,
        Floor,
        "3.4961",
        "0x3.7f",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "-1.0",
        "-0x1",
        7,
        100,
        Nearest,
        "3.5",
        "0x3.8",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "-1.0",
        "-0x1",
        360,
        10,
        Nearest,
        "180.0",
        "0xb4.0",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "-1.0",
        "-0x1",
        360,
        10,
        Floor,
        "179.75",
        "0xb3.c",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "-1.0",
        "-0x1",
        360,
        100,
        Nearest,
        "180.0",
        "0xb4.0",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        "3.0",
        "0x3",
        7,
        10,
        Nearest,
        "1.8",
        "0x1.c",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        "3.0",
        "0x3",
        7,
        10,
        Floor,
        "1.7480",
        "0x1.bf8",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        "3.0",
        "0x3",
        7,
        100,
        Nearest,
        "1.8",
        "0x1.c",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        "3.0",
        "0x3",
        360,
        10,
        Nearest,
        "90.0",
        "0x5a.0",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        "3.0",
        "0x3",
        360,
        10,
        Floor,
        "89.875",
        "0x59.e",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        "3.0",
        "0x3",
        360,
        100,
        Nearest,
        "90.0",
        "0x5a.0",
        Greater,
    );
    test(
        "-3.6e12041199826",
        "-0x1.0E+10000000000",
        "-3.0",
        "-0x3",
        7,
        10,
        Nearest,
        "-1.8",
        "-0x1.c",
        Greater,
    );
    test(
        "-3.6e12041199826",
        "-0x1.0E+10000000000",
        "-3.0",
        "-0x3",
        7,
        10,
        Floor,
        "-1.7520",
        "-0x1.c08",
        Less,
    );
    test(
        "-3.6e12041199826",
        "-0x1.0E+10000000000",
        "-3.0",
        "-0x3",
        7,
        100,
        Nearest,
        "-1.8",
        "-0x1.c",
        Greater,
    );
    test(
        "-3.6e12041199826",
        "-0x1.0E+10000000000",
        "-3.0",
        "-0x3",
        360,
        10,
        Nearest,
        "-90.0",
        "-0x5a.0",
        Greater,
    );
    test(
        "-3.6e12041199826",
        "-0x1.0E+10000000000",
        "-3.0",
        "-0x3",
        360,
        10,
        Floor,
        "-90.125",
        "-0x5a.2",
        Less,
    );
    test(
        "-3.6e12041199826",
        "-0x1.0E+10000000000",
        "-3.0",
        "-0x3",
        360,
        100,
        Nearest,
        "-90.0",
        "-0x5a.0",
        Greater,
    );
    test(
        "6.3e30",
        "0x5.0E+25",
        "2.4e29",
        "0x3.0E+24",
        7,
        10,
        Nearest,
        "1.7090",
        "0x1.b58",
        Greater,
    );
    test(
        "6.3e30",
        "0x5.0E+25",
        "2.4e29",
        "0x3.0E+24",
        7,
        10,
        Floor,
        "1.707",
        "0x1.b5",
        Less,
    );
    test(
        "6.3e30",
        "0x5.0E+25",
        "2.4e29",
        "0x3.0E+24",
        7,
        100,
        Nearest,
        "1.708241394449751322007550439470",
        "0x1.b54f4edad5ca487184408dc1c",
        Greater,
    );
    test(
        "6.3e30",
        "0x5.0E+25",
        "2.4e29",
        "0x3.0E+24",
        360,
        10,
        Nearest,
        "87.875",
        "0x57.e",
        Greater,
    );
    test(
        "6.3e30",
        "0x5.0E+25",
        "2.4e29",
        "0x3.0E+24",
        360,
        10,
        Floor,
        "87.75",
        "0x57.c",
        Less,
    );
    test(
        "6.3e30",
        "0x5.0E+25",
        "2.4e29",
        "0x3.0E+24",
        360,
        100,
        Nearest,
        "87.85241457170149656038830831558",
        "0x57.da37d764177e8da96cf7e81",
        Greater,
    );
    test(
        "-7.0",
        "-0x7",
        "7.6e-1205",
        "0x1.0E-1000",
        7,
        10,
        Nearest,
        "-1.8",
        "-0x1.c",
        Less,
    );
    test(
        "-7.0",
        "-0x7",
        "7.6e-1205",
        "0x1.0E-1000",
        7,
        10,
        Floor,
        "-1.8",
        "-0x1.c",
        Less,
    );
    test(
        "-7.0",
        "-0x7",
        "7.6e-1205",
        "0x1.0E-1000",
        7,
        100,
        Nearest,
        "-1.8",
        "-0x1.c",
        Less,
    );
    test(
        "-7.0",
        "-0x7",
        "7.6e-1205",
        "0x1.0E-1000",
        360,
        10,
        Nearest,
        "-90.0",
        "-0x5a.0",
        Less,
    );
    test(
        "-7.0",
        "-0x7",
        "7.6e-1205",
        "0x1.0E-1000",
        360,
        10,
        Floor,
        "-90.0",
        "-0x5a.0",
        Less,
    );
    test(
        "-7.0",
        "-0x7",
        "7.6e-1205",
        "0x1.0E-1000",
        360,
        100,
        Nearest,
        "-90.0",
        "-0x5a.0",
        Less,
    );
}

#[test]
fn test_atan2_pi_prec_round() {
    let test = |y: &str,
                y_hex: &str,
                x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(y_hex).to_string(), y);
        assert_eq!(hex(x_hex).to_string(), x);
        let y = hex(y_hex);
        let x = hex(x_hex);
        let r = atan2_pi_prec_round_all(&y, &x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{y} {x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{y} {x} {prec} {rm}");
    };
    test("0.0", "0x0", "1.0", "0x1", 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "0.0", "0x0", "-1.0", "-0x1", 10, Exact, "1.0", "0x1.0", Equal,
    );
    test("0.0", "0x0", "0.0", "0x0", 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "1.0", "0x1", "0.0", "0x0", 10, Exact, "0.50", "0x0.8", Equal,
    );
    test(
        "-1.0", "-0x1", "0.0", "0x0", 10, Exact, "-0.50", "-0x0.8", Equal,
    );
    test(
        "1.0", "0x1", "1.0", "0x1", 10, Exact, "0.25", "0x0.4", Equal,
    );
    test(
        "-1.0", "-0x1", "-1.0", "-0x1", 10, Exact, "-0.75", "-0x0.c", Equal,
    );
    test(
        "1.0", "0x1", "-1.0", "-0x1", 10, Exact, "0.75", "0x0.c", Equal,
    );
    test(
        "3.0", "0x3", "-2.0", "-0x2", 10, Nearest, "0.688", "0x0.b", Greater,
    );
    test(
        "3.0", "0x3", "-2.0", "-0x2", 10, Floor, "0.68652", "0x0.afc", Less,
    );
    test(
        "3.0",
        "0x3",
        "-2.0",
        "-0x2",
        100,
        Nearest,
        "0.687167041810998816186252747565",
        "0x0.afea2de377a14503ac36994c",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "1.0",
        "0x1",
        10,
        Nearest,
        "8.784e-12041199828",
        "0x5.18E-10000000001",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "1.0",
        "0x1",
        10,
        Floor,
        "8.7701e-12041199828",
        "0x5.16E-10000000001",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "1.0",
        "0x1",
        100,
        Nearest,
        "8.7821796880865477768385304512518e-12041199828",
        "0x5.17cc1b727220a94fe13abe8f8E-10000000001",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        "1.0",
        "0x1",
        10,
        Nearest,
        "2.635e-12041199827",
        "0xf.48E-10000000001",
        Greater,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        "1.0",
        "0x1",
        10,
        Floor,
        "2.6324e-12041199827",
        "0xf.44E-10000000001",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        "1.0",
        "0x1",
        100,
        Nearest,
        "2.6346539064259643330515591353766e-12041199827",
        "0xf.476452575661fbefa3b03bafE-10000000001",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "-1.0",
        "-0x1",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "-1.0",
        "-0x1",
        10,
        Floor,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "-1.0",
        "-0x1",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        "3.0",
        "0x3",
        10,
        Nearest,
        "0.50",
        "0x0.8",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        "3.0",
        "0x3",
        10,
        Floor,
        "0.49951",
        "0x0.7fe",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        "3.0",
        "0x3",
        100,
        Nearest,
        "0.50",
        "0x0.8",
        Greater,
    );
    test(
        "-3.6e12041199826",
        "-0x1.0E+10000000000",
        "-3.0",
        "-0x3",
        10,
        Nearest,
        "-0.50",
        "-0x0.8",
        Greater,
    );
    test(
        "-3.6e12041199826",
        "-0x1.0E+10000000000",
        "-3.0",
        "-0x3",
        10,
        Floor,
        "-0.50098",
        "-0x0.804",
        Less,
    );
    test(
        "-3.6e12041199826",
        "-0x1.0E+10000000000",
        "-3.0",
        "-0x3",
        100,
        Nearest,
        "-0.50",
        "-0x0.8",
        Greater,
    );
    test(
        "6.3e30",
        "0x5.0E+25",
        "2.4e29",
        "0x3.0E+24",
        10,
        Nearest,
        "0.4883",
        "0x0.7d",
        Greater,
    );
    test(
        "6.3e30",
        "0x5.0E+25",
        "2.4e29",
        "0x3.0E+24",
        10,
        Floor,
        "0.48779",
        "0x0.7ce",
        Less,
    );
    test(
        "6.3e30",
        "0x5.0E+25",
        "2.4e29",
        "0x3.0E+24",
        100,
        Nearest,
        "0.4880689698427860920021572684198",
        "0x0.7cf21687aacc14b2b81271a51",
        Greater,
    );
    test(
        "-7.0",
        "-0x7",
        "7.6e-1205",
        "0x1.0E-1000",
        10,
        Nearest,
        "-0.50",
        "-0x0.8",
        Less,
    );
    test(
        "-7.0",
        "-0x7",
        "7.6e-1205",
        "0x1.0E-1000",
        10,
        Floor,
        "-0.50",
        "-0x0.8",
        Less,
    );
    test(
        "-7.0",
        "-0x7",
        "7.6e-1205",
        "0x1.0E-1000",
        100,
        Nearest,
        "-0.50",
        "-0x0.8",
        Less,
    );
}

#[test]
fn test_atan2_rational_prec_round() {
    let test =
        |y: &str, x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            let y = Rational::from_str(y).unwrap();
            let x = Rational::from_str(x).unwrap();
            let r = atan2_rational_prec_round_all(&y, &x, prec, rm);
            assert_eq!(format!("{:?}", r.0), out_hex, "{y} {x} {prec} {rm}");
            assert_eq!(r.0.to_string(), out);
            assert_eq!(r.1, o, "{y} {x} {prec} {rm}");
        };
    test("0", "1", 10, Exact, "0.0", "0x0.0", Equal);
    test("0", "-1", 10, Floor, "3.141", "0x3.24", Less);
    test("1", "0", 10, Floor, "1.570", "0x1.92", Less);
    test("1", "1", 10, Floor, "0.7852", "0x0.c9", Less);
    test("-1", "-1", 10, Floor, "-2.359", "-0x2.5c", Less);
    test("1/3", "-2/5", 1, Nearest, "2.0", "0x2.0", Less);
    test("1/3", "-2/5", 10, Floor, "2.445", "0x2.72", Less);
    test("1/3", "-2/5", 10, Ceiling, "2.4492", "0x2.73", Greater);
    test(
        "1/3",
        "-2/5",
        100,
        Nearest,
        "2.4468543773930900423453692127665",
        "0x2.72650c68fa4eb4e762bd3301c",
        Less,
    );
    test("-7", "1/1000000000000", 1, Nearest, "-2.0", "-0x2.0", Less);
    test(
        "-7",
        "1/1000000000000",
        10,
        Floor,
        "-1.5723",
        "-0x1.928",
        Less,
    );
    test(
        "-7",
        "1/1000000000000",
        10,
        Ceiling,
        "-1.570",
        "-0x1.92",
        Greater,
    );
    test(
        "-7",
        "1/1000000000000",
        100,
        Nearest,
        "-1.5707963267947537620884645487820",
        "-0x1.921fb54442a94e78624864efe",
        Greater,
    );
    test(
        "1/1000000000000",
        "7",
        1,
        Nearest,
        "1.1e-13",
        "0x2.0E-11",
        Less,
    );
    test(
        "1/1000000000000",
        "7",
        10,
        Floor,
        "1.4277e-13",
        "0x2.83E-11",
        Less,
    );
    test(
        "1/1000000000000",
        "7",
        10,
        Ceiling,
        "1.430e-13",
        "0x2.84E-11",
        Greater,
    );
    test(
        "1/1000000000000",
        "7",
        100,
        Nearest,
        "1.428571428571428571428571418853e-13",
        "0x2.835f12744602715b4ed9428d8E-11",
        Less,
    );
    test("-1", "3", 1, Nearest, "-0.25", "-0x0.4", Greater);
    test("-1", "3", 10, Floor, "-0.32178", "-0x0.526", Less);
    test("-1", "3", 10, Ceiling, "-0.3213", "-0x0.524", Greater);
    test(
        "-1",
        "3",
        100,
        Nearest,
        "-0.3217505543966421934014046143585",
        "-0x0.525e3e8c9a7b84920cd43f952",
        Greater,
    );
}

#[test]
fn test_atan2_with_period_rational_prec_round() {
    let test = |y: &str,
                x: &str,
                u: u64,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        let y = Rational::from_str(y).unwrap();
        let x = Rational::from_str(x).unwrap();
        let r = atan2_with_period_rational_prec_round_all(&y, &x, u, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{y} {x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{y} {x} {prec} {rm}");
    };
    test("0", "1", 8, 10, Exact, "0.0", "0x0.0", Equal);
    test("0", "-1", 8, 10, Exact, "4.0", "0x4.0", Equal);
    test("1", "0", 8, 10, Exact, "2.0", "0x2.0", Equal);
    test("1", "1", 8, 10, Exact, "1.0", "0x1.0", Equal);
    test("-1", "-1", 8, 10, Exact, "-3.0", "-0x3.0", Equal);
    test("1/3", "-2/5", 7, 10, Nearest, "2.727", "0x2.ba", Greater);
    test("1/3", "-2/5", 7, 10, Floor, "2.7227", "0x2.b9", Less);
    test(
        "1/3",
        "-2/5",
        7,
        100,
        Nearest,
        "2.7260027843170656721001210312911",
        "0x2.b9db5187725d789c65b83cf54",
        Less,
    );
    test(
        "-7",
        "1/1000000000000",
        7,
        10,
        Nearest,
        "-1.8",
        "-0x1.c",
        Less,
    );
    test(
        "-7",
        "1/1000000000000",
        7,
        10,
        Floor,
        "-1.8",
        "-0x1.c",
        Less,
    );
    test(
        "-7",
        "1/1000000000000",
        7,
        100,
        Nearest,
        "-1.7499999999998408450569081046650",
        "-0x1.bfffffffffd333ad7f28e9282",
        Less,
    );
    test(
        "1/1000000000000",
        "7",
        7,
        10,
        Nearest,
        "1.5921e-13",
        "0x2.cdE-11",
        Greater,
    );
    test(
        "1/1000000000000",
        "7",
        7,
        10,
        Floor,
        "1.590e-13",
        "0x2.ccE-11",
        Less,
    );
    test(
        "1/1000000000000",
        "7",
        7,
        100,
        Nearest,
        "1.5915494309189533576888376228989e-13",
        "0x2.ccc5280d716d7eee85fe95decE-11",
        Greater,
    );
    test("-1", "3", 7, 10, Nearest, "-0.3584", "-0x0.5bc", Greater);
    test("-1", "3", 7, 10, Floor, "-0.35889", "-0x0.5be", Less);
    test(
        "-1",
        "3",
        7,
        100,
        Nearest,
        "-0.35845733822348354038609623321337",
        "-0x0.5bc3dc30ae5ac533a2414003c8",
        Greater,
    );
}

#[test]
fn test_atan2_pi_rational_prec_round() {
    let test =
        |y: &str, x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            let y = Rational::from_str(y).unwrap();
            let x = Rational::from_str(x).unwrap();
            let r = atan2_pi_rational_prec_round_all(&y, &x, prec, rm);
            assert_eq!(format!("{:?}", r.0), out_hex, "{y} {x} {prec} {rm}");
            assert_eq!(r.0.to_string(), out);
            assert_eq!(r.1, o, "{y} {x} {prec} {rm}");
        };
    test("0", "1", 10, Exact, "0.0", "0x0.0", Equal);
    test("0", "-1", 10, Exact, "1.0", "0x1.0", Equal);
    test("1", "0", 10, Exact, "0.50", "0x0.8", Equal);
    test("1", "1", 10, Exact, "0.25", "0x0.4", Equal);
    test("-1", "-1", 10, Exact, "-0.75", "-0x0.c", Equal);
    test("1/3", "-2/5", 10, Nearest, "0.7793", "0x0.c78", Greater);
    test("1/3", "-2/5", 10, Floor, "0.77832", "0x0.c74", Less);
    test(
        "1/3",
        "-2/5",
        100,
        Nearest,
        "0.7788579383763044777428917232265",
        "0x0.c7633bdd8e63d95141a25a8f4",
        Greater,
    );
    test(
        "-7",
        "1/1000000000000",
        10,
        Nearest,
        "-0.50",
        "-0x0.8",
        Less,
    );
    test("-7", "1/1000000000000", 10, Floor, "-0.50", "-0x0.8", Less);
    test(
        "-7",
        "1/1000000000000",
        100,
        Nearest,
        "-0.4999999999999545271591166013326",
        "-0x0.7ffffffffff333562454d4e6e",
        Greater,
    );
    test(
        "1/1000000000000",
        "7",
        10,
        Nearest,
        "4.5464e-14",
        "0xc.ccE-12",
        Less,
    );
    test(
        "1/1000000000000",
        "7",
        10,
        Floor,
        "4.5464e-14",
        "0xc.ccE-12",
        Less,
    );
    test(
        "1/1000000000000",
        "7",
        100,
        Nearest,
        "4.547284088339866736253821779711e-14",
        "0xc.cca9dbab2b191fb01b673f68E-12",
        Greater,
    );
    test("-1", "3", 10, Nearest, "-0.10242", "-0x0.1a38", Less);
    test("-1", "3", 10, Floor, "-0.10242", "-0x0.1a38", Less);
    test(
        "-1",
        "3",
        100,
        Nearest,
        "-0.10241638234956672582459892377529",
        "-0x0.1a37f5c4c419ef3352ee124a3a",
        Less,
    );
}

#[test]
fn atan2_prec_round_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_17()
        .test_properties(|(y, x, prec, rm)| {
            let r = atan2_prec_round_all(&y, &x, prec, rm);
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(&atan2_prec_round_all(&y, &x, prec, Exact), &r));
            }
            if !y.is_zero() && !y.is_nan() && !x.is_nan() && rm != Exact {
                // atan2 is odd in y
                let (s, o) = atan2_prec_round_all(&-&y, &x, prec, -rm);
                assert!(same(&s, &-&r.0));
                assert_eq!(o, r.1.reverse());
            }
            // multiplying both arguments by the same power of 2 does not change the angle
            if y.is_finite() && x.is_finite() {
                assert!(same_pair(
                    &atan2_prec_round_all(&(&y << 5u32), &(&x << 5u32), prec, rm),
                    &r
                ));
            }
            if rm != Exact {
                for u in PERIODS {
                    let s = atan2_with_period_prec_round_all(&y, &x, u, prec, rm);
                    check_result(&s, prec);
                }
            }
            if rm != Exact
                && big_exponent_float_exponent_is_small(&y)
                && big_exponent_float_exponent_is_small(&x)
                && let (Ok(qy), Ok(qx)) = (Rational::try_from(&y), Rational::try_from(&x))
            {
                assert!(same_pair(
                    &atan2_rational_prec_round_all(&qy, &qx, prec, rm),
                    &r
                ));
            }
        });
}

#[test]
fn atan2_prec_round_float_agreement_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_24().test_properties(
        |(y, x, prec, rm)| {
            // the `Float` generator's `Exact` cases are not all exact for atan2
            if y.is_zero() || rm == Exact {
                return;
            }
            let (z, o) = y.atan2_prec_round_ref_ref(&x, prec, rm);
            let yb = float_to_big_exponent_float(&y);
            let xb = float_to_big_exponent_float(&x);
            if float_result_comparable(&z, o) {
                let r = atan2_prec_round_all(&yb, &xb, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&z)));
                assert_eq!(r.1, o);
            }
            if rm != Exact {
                for u in PERIODS {
                    let (z, o) = y.atan2_with_period_prec_round_ref_ref(&x, u, prec, rm);
                    if float_result_comparable(&z, o) {
                        let r = atan2_with_period_prec_round_all(&yb, &xb, u, prec, rm);
                        assert!(same(&r.0, &float_to_big_exponent_float(&z)));
                        assert_eq!(r.1, o);
                    }
                }
            }
        },
    );
}

#[test]
fn atan2_rational_prec_round_properties() {
    rational_rational_unsigned_rounding_mode_quadruple_gen_var_4().test_properties(
        |(y, x, prec, rm)| {
            let r = atan2_rational_prec_round_all(&y, &x, prec, rm);
            check_result(&r, prec);
            let (z, o) = Float::atan2_rational_prec_round_ref(&y, &x, prec, rm);
            if float_result_comparable(&z, o) {
                assert!(same(&r.0, &float_to_big_exponent_float(&z)));
                assert_eq!(r.1, o);
            }
            if rm != Exact {
                for u in PERIODS {
                    let s = atan2_with_period_rational_prec_round_all(&y, &x, u, prec, rm);
                    let (z, o) =
                        Float::atan2_with_period_rational_prec_round_ref(&y, &x, u, prec, rm);
                    if float_result_comparable(&z, o) {
                        assert!(same(&s.0, &float_to_big_exponent_float(&z)));
                        assert_eq!(s.1, o);
                    }
                    if u == 2 {
                        assert!(same_pair(
                            &atan2_pi_rational_prec_round_all(&y, &x, prec, rm),
                            &s
                        ));
                    }
                }
            }
        },
    );
}

#[test]
#[should_panic]
fn atan2_prec_round_fail_1() {
    BigExponentFloat::ONE.atan2_prec_round(BigExponentFloat::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn atan2_prec_round_fail_2() {
    BigExponentFloat::ONE.atan2_prec_round(BigExponentFloat::from(2), 10, Exact);
}

#[test]
#[should_panic]
fn atan2_rational_prec_round_fail_1() {
    BigExponentFloat::atan2_rational_prec_round(Rational::ONE, Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn atan2_with_period_prec_round_fail_1() {
    BigExponentFloat::ONE.atan2_with_period_prec_round(BigExponentFloat::from(2), 7, 10, Exact);
}
