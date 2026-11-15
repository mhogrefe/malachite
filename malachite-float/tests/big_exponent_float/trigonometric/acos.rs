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
        || !x.is_zero() || o == Equal,
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

fn acos_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().acos_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.acos_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.acos_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().acos_prec(prec), &r));
        assert!(same_pair(&x.acos_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.acos_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn acos_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::acos_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::acos_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::acos_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::acos_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

fn acos_with_period_prec_round_all(
    x: &BigExponentFloat,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().acos_with_period_prec_round(u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.acos_with_period_prec_round_ref(u, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.acos_with_period_prec_round_assign(u, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().acos_with_period_prec(u, prec), &r));
        assert!(same_pair(&x.acos_with_period_prec_ref(u, prec), &r));
        let mut z = x.clone();
        let o = z.acos_with_period_prec_assign(u, prec);
        assert!(same_pair(&(z, o), &r));
    }
    if u == 2 {
        assert!(same_pair(&acos_pi_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn acos_with_period_rational_prec_round_all(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::acos_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::acos_with_period_rational_prec_round_ref(x, u, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::acos_with_period_rational_prec(x.clone(), u, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::acos_with_period_rational_prec_ref(x, u, prec),
            &r
        ));
    }
    if u == 2 {
        assert!(same_pair(&acos_pi_rational_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn acos_pi_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().acos_pi_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.acos_pi_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.acos_pi_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().acos_pi_prec(prec), &r));
        assert!(same_pair(&x.acos_pi_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.acos_pi_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn acos_pi_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::acos_pi_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::acos_pi_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::acos_pi_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::acos_pi_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_acos_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = acos_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_trig_prec_round_generic(InverseTrigFunction::Acos, &x, None, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 10, Floor, "1.570", "0x1.92", Less);
    test("0.50", "0x0.8", 10, Floor, "1.047", "0x1.0c", Less);
    test("-0.50", "-0x0.8", 10, Floor, "2.094", "0x2.18", Less);
    test("1.0", "0x1", 10, Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1", 10, Floor, "3.141", "0x3.24", Less);
    test("2.0", "0x2", 10, Exact, "NaN", "NaN", Equal);
    test("-2.0", "-0x2", 10, Exact, "NaN", "NaN", Equal);
    test("0.38", "0x0.6", 1, Nearest, "1.0", "0x1.0", Less);
    test("0.38", "0x0.6", 10, Floor, "1.1855", "0x1.2f8", Less);
    test("0.38", "0x0.6", 10, Ceiling, "1.19", "0x1.3", Greater);
    test(
        "0.38",
        "0x0.6",
        100,
        Nearest,
        "1.186399552299257536193126818672",
        "0x1.2fb7e18d1d463855a07354fb8",
        Less,
    );
    test("-0.38", "-0x0.6", 1, Nearest, "2.0", "0x2.0", Greater);
    test("-0.38", "-0x0.6", 10, Floor, "1.9551", "0x1.f48", Less);
    test("-0.38", "-0x0.6", 10, Ceiling, "1.957", "0x1.f5", Greater);
    test(
        "-0.38",
        "-0x0.6",
        100,
        Nearest,
        "1.955193101290535702269516564607",
        "0x1.f48788fb685cd07d72a635328",
        Greater,
    );
    test("0.9961", "0x0.ff", 1, Nearest, "0.062", "0x0.1", Less);
    test("0.9961", "0x0.ff", 10, Floor, "0.08838", "0x0.16a", Less);
    test(
        "0.9961", "0x0.ff", 10, Ceiling, "0.088501", "0x0.16a8", Greater,
    );
    test(
        "0.9961",
        "0x0.ff",
        100,
        Nearest,
        "0.08841714521444942661517562582139",
        "0x0.16a2818b1a0bd78aa408c2db54",
        Greater,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        1,
        Nearest,
        "5.8e-11",
        "0x4.0E-9",
        Less,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        10,
        Floor,
        "8.231e-11",
        "0x5.a8E-9",
        Less,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        10,
        Ceiling,
        "8.2423e-11",
        "0x5.aaE-9",
        Greater,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        100,
        Nearest,
        "8.231806349783990583394700774049e-11",
        "0x5.a827999fcef32422d3772d13E-9",
        Less,
    );
    test(
        "-0.9999999999999999999966",
        "-0x0.fffffffffffffffff",
        1,
        Nearest,
        "4.0",
        "0x4.0",
        Greater,
    );
    test(
        "-0.9999999999999999999966",
        "-0x0.fffffffffffffffff",
        10,
        Floor,
        "3.141",
        "0x3.24",
        Less,
    );
    test(
        "-0.9999999999999999999966",
        "-0x0.fffffffffffffffff",
        10,
        Ceiling,
        "3.1445",
        "0x3.25",
        Greater,
    );
    test(
        "-0.9999999999999999999966",
        "-0x0.fffffffffffffffff",
        100,
        Nearest,
        "3.1415926535074751749648034774445",
        "0x3.243f6a882b208f39162a57ebc",
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
        "2.0",
        "0x2.0",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Floor,
        "1.570",
        "0x1.92",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Ceiling,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        100,
        Nearest,
        "1.570796326794896619231321691640",
        "0x1.921fb54442d18469898cc517",
        Less,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        1,
        Nearest,
        "2.0",
        "0x2.0",
        Greater,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Floor,
        "1.570",
        "0x1.92",
        Less,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Ceiling,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        100,
        Nearest,
        "1.570796326794896619231321691640",
        "0x1.921fb54442d18469898cc517",
        Less,
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
fn test_acos_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = acos_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("0", 10, Floor, "1.570", "0x1.92", Less);
    test("1", 10, Exact, "0.0", "0x0.0", Equal);
    test("-1", 10, Floor, "3.141", "0x3.24", Less);
    test("1/2", 10, Floor, "1.047", "0x1.0c", Less);
    test("2", 10, Exact, "NaN", "NaN", Equal);
    test("1/3", 1, Nearest, "1.0", "0x1.0", Less);
    test("1/3", 10, Floor, "1.230", "0x1.3b", Less);
    test("1/3", 10, Ceiling, "1.2324", "0x1.3b8", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "1.230959417340774682134929178248",
        "0x1.3b2028082e8d3bb0b49d8acf",
        Less,
    );
    test("-2/3", 1, Nearest, "2.0", "0x2.0", Less);
    test("-2/3", 10, Floor, "2.297", "0x2.4c", Less);
    test("-2/3", 10, Ceiling, "2.3008", "0x2.4d", Greater);
    test(
        "-2/3",
        100,
        Nearest,
        "2.300523983021862982686118351453",
        "0x2.4cef23c6be187653154af0f18",
        Less,
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
        "2.0",
        "0x2.0",
        Greater,
    );
    test(
        "1/1000000000000000000000",
        10,
        Floor,
        "1.570",
        "0x1.92",
        Less,
    );
    test(
        "1/1000000000000000000000",
        10,
        Ceiling,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "1/1000000000000000000000",
        100,
        Nearest,
        "1.570796326794896619230321691640",
        "0x1.921fb54442d1846984d3d8148",
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
        "9.5e-7",
        "0x0.00001",
        Less,
    );
    test(
        "999999999999/1000000000000",
        10,
        Floor,
        "1.4137e-6",
        "0x0.000017b8",
        Less,
    );
    test(
        "999999999999/1000000000000",
        10,
        Ceiling,
        "1.416e-6",
        "0x0.000017c",
        Greater,
    );
    test(
        "999999999999/1000000000000",
        100,
        Nearest,
        "1.4142135623732128999318865086476e-6",
        "0x0.000017ba0041886bbd076947ad1e72",
        Greater,
    );
}

#[test]
fn test_acos_with_period_prec_round() {
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
        let r = acos_with_period_prec_round_all(&x, u, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_trig_prec_round_generic(InverseTrigFunction::Acos, &x, Some(u), prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 7, 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 7, 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 7, 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 24, 10, Exact, "6.0", "0x6.0", Equal);
    test("0.0", "0x0", 7, 10, Exact, "1.8", "0x1.c", Equal);
    test("0.0", "0x0", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8", 24, 10, Exact, "4.0", "0x4.0", Equal);
    test("0.50", "0x0.8", 7, 10, Nearest, "1.1660", "0x1.2a8", Less);
    test("0.50", "0x0.8", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("-0.50", "-0x0.8", 24, 10, Exact, "8.0", "0x8.0", Equal);
    test("-0.50", "-0x0.8", 7, 10, Nearest, "2.3320", "0x2.55", Less);
    test("-0.50", "-0x0.8", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", 24, 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", 7, 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1", 24, 10, Exact, "12.0", "0xc.0", Equal);
    test("-1.0", "-0x1", 7, 10, Exact, "3.5", "0x3.8", Equal);
    test("-1.0", "-0x1", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("2.0", "0x2", 24, 10, Exact, "NaN", "NaN", Equal);
    test("2.0", "0x2", 7, 10, Exact, "NaN", "NaN", Equal);
    test("2.0", "0x2", 0, 10, Exact, "NaN", "NaN", Equal);
    test("-2.0", "-0x2", 24, 10, Exact, "NaN", "NaN", Equal);
    test("-2.0", "-0x2", 7, 10, Exact, "NaN", "NaN", Equal);
    test("-2.0", "-0x2", 0, 10, Exact, "NaN", "NaN", Equal);
    test(
        "0.38", "0x0.6", 7, 10, Nearest, "1.3223", "0x1.528", Greater,
    );
    test("0.38", "0x0.6", 7, 10, Floor, "1.320", "0x1.52", Less);
    test(
        "0.38",
        "0x0.6",
        7,
        100,
        Nearest,
        "1.321749472613069061308118259949",
        "0x1.525e2c6660dd76eb43d417d48",
        Greater,
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
        "1.8",
        "0x1.c",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        7,
        10,
        Floor,
        "1.7480",
        "0x1.bf8",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        7,
        100,
        Nearest,
        "1.8",
        "0x1.c",
        Greater,
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
        "90.0",
        "0x5a.0",
        Less,
    );
    test(
        "-2.8e-12041199827",
        "-0x1.0E-10000000000",
        360,
        10,
        Floor,
        "90.0",
        "0x5a.0",
        Less,
    );
    test(
        "-2.8e-12041199827",
        "-0x1.0E-10000000000",
        360,
        100,
        Nearest,
        "90.0",
        "0x5a.0",
        Less,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        5,
        10,
        Nearest,
        "6.55e-11",
        "0x4.8E-9",
        Less,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        5,
        10,
        Floor,
        "6.55e-11",
        "0x4.8E-9",
        Less,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        5,
        100,
        Nearest,
        "6.5506633557168684605527479303446e-11",
        "0x4.8067a685bff52cf83960f1078E-9",
        Less,
    );
}

#[test]
fn test_acos_with_period_rational_prec_round() {
    let test =
        |x: &str, u: u64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            let x = Rational::from_str(x).unwrap();
            let r = acos_with_period_rational_prec_round_all(&x, u, prec, rm);
            assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
            assert_eq!(r.0.to_string(), out);
            assert_eq!(r.1, o, "{x} {prec} {rm}");
        };
    test("0", 24, 10, Exact, "6.0", "0x6.0", Equal);
    test("0", 7, 10, Exact, "1.8", "0x1.c", Equal);
    test("1", 24, 10, Exact, "0.0", "0x0.0", Equal);
    test("1", 7, 10, Exact, "0.0", "0x0.0", Equal);
    test("1/2", 24, 10, Exact, "4.0", "0x4.0", Equal);
    test("1/2", 7, 10, Nearest, "1.1660", "0x1.2a8", Less);
    test("-2", 24, 10, Exact, "NaN", "NaN", Equal);
    test("-2", 7, 10, Exact, "NaN", "NaN", Equal);
    test("1/3", 7, 10, Nearest, "1.371", "0x1.5f", Less);
    test("1/3", 7, 10, Floor, "1.371", "0x1.5f", Less);
    test(
        "1/3",
        7,
        100,
        Nearest,
        "1.3713929321071254455979944572785",
        "0x1.5f139b715d9b6139ccc4c2ea6",
        Less,
    );
    test("-7/3", 7, 10, Nearest, "NaN", "NaN", Equal);
    test("-7/3", 7, 10, Floor, "NaN", "NaN", Equal);
    test("-7/3", 7, 100, Nearest, "NaN", "NaN", Equal);
    test(
        "1/10000000000000000000000000",
        5,
        10,
        Nearest,
        "1.2",
        "0x1.4",
        Greater,
    );
    test(
        "1/10000000000000000000000000",
        5,
        10,
        Floor,
        "1.2480",
        "0x1.3f8",
        Less,
    );
    test(
        "1/10000000000000000000000000",
        5,
        100,
        Nearest,
        "1.249999999999999999999999920423",
        "0x1.3fffffffffffffffffffe75f4",
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
    test("-1/2", 7, 10, Nearest, "2.3320", "0x2.55", Less);
    test("-1/2", 7, 10, Floor, "2.3320", "0x2.55", Less);
    test(
        "-1/2",
        7,
        100,
        Nearest,
        "2.3333333333333333333333333333323",
        "0x2.5555555555555555555555554",
        Less,
    );
}

#[test]
fn test_acos_pi_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = acos_pi_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_trig_prec_round_generic(InverseTrigFunction::Acos, &x, Some(2), prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 10, Exact, "0.50", "0x0.8", Equal);
    test("0.50", "0x0.8", 10, Nearest, "0.33350", "0x0.556", Greater);
    test("0.50", "0x0.8", 10, Floor, "0.3330", "0x0.554", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        Greater,
    );
    test(
        "-0.50", "-0x0.8", 10, Nearest, "0.66699", "0x0.aac", Greater,
    );
    test("-0.50", "-0x0.8", 10, Floor, "0.6660", "0x0.aa8", Less);
    test(
        "-0.50",
        "-0x0.8",
        100,
        Nearest,
        "0.66666666666666666666666666666693",
        "0x0.aaaaaaaaaaaaaaaaaaaaaaaab",
        Greater,
    );
    test("1.0", "0x1", 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", 100, Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1", 10, Exact, "1.0", "0x1.0", Equal);
    test("-1.0", "-0x1", 10, Floor, "1.0", "0x1.0", Equal);
    test("-1.0", "-0x1", 100, Exact, "1.0", "0x1.0", Equal);
    test("2.0", "0x2", 10, Exact, "NaN", "NaN", Equal);
    test("2.0", "0x2", 10, Floor, "NaN", "NaN", Equal);
    test("2.0", "0x2", 100, Exact, "NaN", "NaN", Equal);
    test("0.38", "0x0.6", 10, Nearest, "0.37744", "0x0.60a", Less);
    test("0.38", "0x0.6", 10, Floor, "0.37744", "0x0.60a", Less);
    test(
        "0.38",
        "0x0.6",
        100,
        Nearest,
        "0.3776427064608768746594623599853",
        "0x0.60ad3141d2886b1ea5aa4ff39",
        Less,
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
        "0.50",
        "0x0.8",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Floor,
        "0.49951",
        "0x0.7fe",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        100,
        Nearest,
        "0.50",
        "0x0.8",
        Greater,
    );
}

#[test]
fn test_acos_pi_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = acos_pi_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("1/2", 10, Nearest, "0.33350", "0x0.556", Greater);
    test("1/2", 10, Floor, "0.3330", "0x0.554", Less);
    test(
        "1/2",
        100,
        Nearest,
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        Greater,
    );
    test("-1/2", 10, Nearest, "0.66699", "0x0.aac", Greater);
    test("-1/2", 10, Floor, "0.6660", "0x0.aa8", Less);
    test(
        "-1/2",
        100,
        Nearest,
        "0.66666666666666666666666666666693",
        "0x0.aaaaaaaaaaaaaaaaaaaaaaaab",
        Greater,
    );
    test("1", 10, Exact, "0.0", "0x0.0", Equal);
    test("1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1", 100, Exact, "0.0", "0x0.0", Equal);
    test("2", 10, Exact, "NaN", "NaN", Equal);
    test("2", 10, Floor, "NaN", "NaN", Equal);
    test("2", 100, Exact, "NaN", "NaN", Equal);
    test("1/3", 10, Nearest, "0.3916", "0x0.644", Less);
    test("1/3", 10, Floor, "0.3916", "0x0.644", Less);
    test(
        "1/3",
        100,
        Nearest,
        "0.3918265520306072701708555592224",
        "0x0.644ebeb2ad07d2a2ccca80d54",
        Less,
    );
    test("-5/7", 10, Nearest, "0.75293", "0x0.c0c", Less);
    test("-5/7", 10, Floor, "0.75293", "0x0.c0c", Less);
    test(
        "-5/7",
        100,
        Nearest,
        "0.753248285571150139032715333042",
        "0x0.c0d4e1304bd10b9945673dc5",
        Less,
    );
}

#[test]
#[should_panic]
fn acos_prec_round_fail_1() {
    BigExponentFloat::from(0.375).acos_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn acos_prec_round_fail_2() {
    BigExponentFloat::from(0.375).acos_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn acos_prec_round_fail_3() {
    BigExponentFloat::acos_rational_prec_round(Rational::from_unsigneds(1u8, 3), 0, Floor);
}

#[test]
#[should_panic]
fn acos_prec_round_fail_4() {
    BigExponentFloat::acos_rational_prec_round(Rational::from_unsigneds(1u8, 3), 10, Exact);
}

#[test]
#[should_panic]
fn acos_prec_round_fail_5() {
    BigExponentFloat::from(0.375).acos_with_period_prec_round(7, 0, Floor);
}

#[test]
#[should_panic]
fn acos_prec_round_fail_6() {
    BigExponentFloat::from(0.375).acos_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn acos_prec_round_fail_7() {
    BigExponentFloat::acos_with_period_rational_prec_round(
        Rational::from_unsigneds(1u8, 3),
        7,
        0,
        Floor,
    );
}

#[test]
#[should_panic]
fn acos_prec_round_fail_8() {
    BigExponentFloat::from(0.375).acos_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn acos_prec_round_fail_9() {
    BigExponentFloat::from(0.375).acos_pi_prec_round(10, Exact);
}

#[test]
fn acos_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties(
        |(x, prec, rm)| {
            let r = acos_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with the decomposition wherever both apply.
            assert!(same_pair(
                &inverse_trig_prec_round_generic(InverseTrigFunction::Acos, &x, None, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(&acos_prec_round_all(&x, prec, Exact), &r));
            }
            if rm != Exact
                && big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&acos_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn acos_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_43().test_properties(|(x, prec, rm)| {
        // the `Float` generator's `Exact` cases are not all exact for this function
        if x.is_zero() || rm == Exact {
            return;
        }
        let (y, o) = x.acos_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) {
            let r = acos_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
        if rm != Exact {
            for u in PERIODS {
                let (y, o) = x.acos_with_period_prec_round_ref(u, prec, rm);
                if float_result_comparable(&y, o) {
                    let r = acos_with_period_prec_round_all(
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
fn acos_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_11().test_properties(|(x, prec, rm)| {
        let r = acos_rational_prec_round_all(&x, prec, rm);
        check_result(&r, prec);
        let (y, o) = Float::acos_rational_prec_round_ref(&x, prec, rm);
        if float_result_comparable(&y, o) {
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn acos_with_period_prec_round_properties() {
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_9().test_properties(
        |(x, u, prec, rm)| {
            let r = acos_with_period_prec_round_all(&x, u, prec, rm);
            assert!(same_pair(
                &inverse_trig_prec_round_generic(InverseTrigFunction::Acos, &x, Some(u), prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(
                    &acos_with_period_prec_round_all(&x, u, prec, Exact),
                    &r
                ));
            }
            if rm != Exact
                && big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(
                    &acos_with_period_rational_prec_round_all(&q, u, prec, rm),
                    &r
                ));
            }
        },
    );
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties(
        |(x, prec, rm)| {
            acos_with_period_prec_round_all(&x, 2, prec, rm);
        },
    );
}

#[test]
fn acos_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_8().test_properties(
        |(x, u, prec, rm)| {
            let r = acos_with_period_rational_prec_round_all(&x, u, prec, rm);
            check_result(&r, prec);
            let (y, o) = Float::acos_with_period_rational_prec_round_ref(&x, u, prec, rm);
            if float_result_comparable(&y, o) {
                assert!(same(&r.0, &float_to_big_exponent_float(&y)));
                assert_eq!(r.1, o);
            }
        },
    );
}
