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

fn asec_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().asec_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.asec_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.asec_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().asec_prec(prec), &r));
        assert!(same_pair(&x.asec_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.asec_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn asec_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::asec_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::asec_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::asec_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::asec_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

fn asec_with_period_prec_round_all(
    x: &BigExponentFloat,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().asec_with_period_prec_round(u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.asec_with_period_prec_round_ref(u, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.asec_with_period_prec_round_assign(u, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().asec_with_period_prec(u, prec), &r));
        assert!(same_pair(&x.asec_with_period_prec_ref(u, prec), &r));
        let mut z = x.clone();
        let o = z.asec_with_period_prec_assign(u, prec);
        assert!(same_pair(&(z, o), &r));
    }
    if u == 2 {
        assert!(same_pair(&asec_pi_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn asec_with_period_rational_prec_round_all(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::asec_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::asec_with_period_rational_prec_round_ref(x, u, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::asec_with_period_rational_prec(x.clone(), u, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::asec_with_period_rational_prec_ref(x, u, prec),
            &r
        ));
    }
    if u == 2 {
        assert!(same_pair(&asec_pi_rational_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn asec_pi_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().asec_pi_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.asec_pi_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.asec_pi_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().asec_pi_prec(prec), &r));
        assert!(same_pair(&x.asec_pi_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.asec_pi_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn asec_pi_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::asec_pi_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::asec_pi_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::asec_pi_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::asec_pi_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_asec_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = asec_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_trig_prec_round_generic(InverseTrigFunction::Asec, &x, None, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "1.570", "0x1.92", Less);
    test("-Infinity", "-Infinity", 10, Floor, "1.570", "0x1.92", Less);
    test("0.0", "0x0", 10, Exact, "NaN", "NaN", Equal);
    test("0.50", "0x0.8", 10, Exact, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 10, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1", 10, Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1", 10, Floor, "3.141", "0x3.24", Less);
    test("2.0", "0x2", 10, Floor, "1.047", "0x1.0c", Less);
    test("-2.0", "-0x2", 10, Floor, "2.094", "0x2.18", Less);
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
    test("3.0", "0x3", 1, Nearest, "1.0", "0x1.0", Less);
    test("3.0", "0x3", 10, Floor, "1.230", "0x1.3b", Less);
    test("3.0", "0x3", 10, Ceiling, "1.2324", "0x1.3b8", Greater);
    test(
        "3.0",
        "0x3",
        100,
        Nearest,
        "1.230959417340774682134929178248",
        "0x1.3b2028082e8d3bb0b49d8acf",
        Less,
    );
    test("-1024.5", "-0x400.8", 1, Nearest, "2.0", "0x2.0", Greater);
    test("-1024.5", "-0x400.8", 10, Floor, "1.570", "0x1.92", Less);
    test(
        "-1024.5", "-0x400.8", 10, Ceiling, "1.5723", "0x1.928", Greater,
    );
    test(
        "-1024.5",
        "-0x400.8",
        100,
        Nearest,
        "1.5717724128454487721487292453712",
        "0x1.925fad45ed1c47da2dac04446",
        Greater,
    );
    test("7.6e22", "0x1.0E+19", 1, Nearest, "2.0", "0x2.0", Greater);
    test("7.6e22", "0x1.0E+19", 10, Floor, "1.570", "0x1.92", Less);
    test(
        "7.6e22",
        "0x1.0E+19",
        10,
        Ceiling,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "7.6e22",
        "0x1.0E+19",
        100,
        Nearest,
        "1.570796326794896619231308456750",
        "0x1.921fb54442d18469897cc517",
        Less,
    );
    test("1.3e30", "0x1.0E+25", 1, Nearest, "2.0", "0x2.0", Greater);
    test("1.3e30", "0x1.0E+25", 10, Floor, "1.570", "0x1.92", Less);
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Ceiling,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        100,
        Nearest,
        "1.570796326794896619231321691640",
        "0x1.921fb54442d18469898cc517",
        Greater,
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
        "2.0",
        "0x2.0",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Floor,
        "1.570",
        "0x1.92",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Ceiling,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        100,
        Nearest,
        "1.570796326794896619231321691640",
        "0x1.921fb54442d18469898cc517",
        Less,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        1,
        Nearest,
        "2.0",
        "0x2.0",
        Greater,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        10,
        Floor,
        "1.570",
        "0x1.92",
        Less,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        10,
        Ceiling,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        100,
        Nearest,
        "1.570796326794896619231321691640",
        "0x1.921fb54442d18469898cc517",
        Less,
    );
    test(
        "1.0000000000000000000002",
        "0x1.000000000000000001",
        1,
        Nearest,
        "1.5e-11",
        "0x1.0E-9",
        Less,
    );
    test(
        "1.0000000000000000000002",
        "0x1.000000000000000001",
        10,
        Floor,
        "2.058e-11",
        "0x1.6aE-9",
        Less,
    );
    test(
        "1.0000000000000000000002",
        "0x1.000000000000000001",
        10,
        Ceiling,
        "2.0606e-11",
        "0x1.6a8E-9",
        Greater,
    );
    test(
        "1.0000000000000000000002",
        "0x1.000000000000000001",
        100,
        Nearest,
        "2.0579515874459976458479125641644e-11",
        "0x1.6a09e667f3bcc908b26439f1aE-9",
        Greater,
    );
}

#[test]
fn test_asec_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = asec_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("0", 10, Exact, "NaN", "NaN", Equal);
    test("1", 10, Exact, "0.0", "0x0.0", Equal);
    test("-1", 10, Floor, "3.141", "0x3.24", Less);
    test("1/2", 10, Exact, "NaN", "NaN", Equal);
    test("2", 10, Floor, "1.047", "0x1.0c", Less);
    test("1/3", 1, Nearest, "NaN", "NaN", Equal);
    test("1/3", 10, Floor, "NaN", "NaN", Equal);
    test("1/3", 10, Ceiling, "NaN", "NaN", Equal);
    test("1/3", 100, Nearest, "NaN", "NaN", Equal);
    test("-2/3", 1, Nearest, "NaN", "NaN", Equal);
    test("-2/3", 10, Floor, "NaN", "NaN", Equal);
    test("-2/3", 10, Ceiling, "NaN", "NaN", Equal);
    test("-2/3", 100, Nearest, "NaN", "NaN", Equal);
    test("7/3", 1, Nearest, "1.0", "0x1.0", Less);
    test("7/3", 10, Floor, "1.1270", "0x1.208", Less);
    test("7/3", 10, Ceiling, "1.129", "0x1.21", Greater);
    test(
        "7/3",
        100,
        Nearest,
        "1.1278852827212576853876623443678",
        "0x1.20bd1702ed71aa480c4ca20b2",
        Greater,
    );
    test("-7/3", 1, Nearest, "2.0", "0x2.0", Less);
    test("-7/3", 10, Floor, "2.0117", "0x2.03", Less);
    test("-7/3", 10, Ceiling, "2.016", "0x2.04", Greater);
    test(
        "-7/3",
        100,
        Nearest,
        "2.013707370868535553074981038913",
        "0x2.0382538598315e8b06cce823",
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
        "2.0",
        "0x2.0",
        Greater,
    );
    test(
        "1000000000000000000000/7",
        10,
        Floor,
        "1.570",
        "0x1.92",
        Less,
    );
    test(
        "1000000000000000000000/7",
        10,
        Ceiling,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "1000000000000000000000/7",
        100,
        Nearest,
        "1.5707963267948966192243216916393",
        "0x1.921fb54442d18469687e4a056",
        Less,
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
fn test_asec_with_period_prec_round() {
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
        let r = asec_with_period_prec_round_all(&x, u, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_trig_prec_round_generic(InverseTrigFunction::Asec, &x, Some(u), prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 7, 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 7, 10, Floor, "1.8", "0x1.c", Equal);
    test(
        "-Infinity",
        "-Infinity",
        7,
        10,
        Floor,
        "1.8",
        "0x1.c",
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
    test("1.0", "0x1", 24, 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", 7, 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1", 24, 10, Exact, "12.0", "0xc.0", Equal);
    test("-1.0", "-0x1", 7, 10, Exact, "3.5", "0x3.8", Equal);
    test("-1.0", "-0x1", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("2.0", "0x2", 24, 10, Exact, "4.0", "0x4.0", Equal);
    test("2.0", "0x2", 7, 10, Nearest, "1.1660", "0x1.2a8", Less);
    test("2.0", "0x2", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2", 24, 10, Exact, "8.0", "0x8.0", Equal);
    test("-2.0", "-0x2", 7, 10, Nearest, "2.3320", "0x2.55", Less);
    test("-2.0", "-0x2", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("0.38", "0x0.6", 7, 10, Nearest, "NaN", "NaN", Equal);
    test("0.38", "0x0.6", 7, 10, Floor, "NaN", "NaN", Equal);
    test("0.38", "0x0.6", 7, 100, Nearest, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", 7, 10, Nearest, "2.1289", "0x2.21", Greater);
    test("-3.0", "-0x3", 7, 10, Floor, "2.12", "0x2.2", Less);
    test(
        "-3.0",
        "-0x3",
        7,
        100,
        Nearest,
        "2.128607067892874554402005542720",
        "0x2.20ec648ea2649ec6333b3d158",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
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
        7,
        100,
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
        "90.0",
        "0x5a.0",
        Greater,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        360,
        10,
        Floor,
        "89.875",
        "0x59.e",
        Less,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        360,
        100,
        Nearest,
        "90.0",
        "0x5a.0",
        Greater,
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
fn test_asec_with_period_rational_prec_round() {
    let test =
        |x: &str, u: u64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            let x = Rational::from_str(x).unwrap();
            let r = asec_with_period_rational_prec_round_all(&x, u, prec, rm);
            assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
            assert_eq!(r.0.to_string(), out);
            assert_eq!(r.1, o, "{x} {prec} {rm}");
        };
    test("0", 24, 10, Exact, "NaN", "NaN", Equal);
    test("0", 7, 10, Exact, "NaN", "NaN", Equal);
    test("1", 24, 10, Exact, "0.0", "0x0.0", Equal);
    test("1", 7, 10, Exact, "0.0", "0x0.0", Equal);
    test("1/2", 24, 10, Exact, "NaN", "NaN", Equal);
    test("1/2", 7, 10, Exact, "NaN", "NaN", Equal);
    test("-2", 24, 10, Exact, "8.0", "0x8.0", Equal);
    test("-2", 7, 10, Nearest, "2.3320", "0x2.55", Less);
    test("1/3", 7, 10, Nearest, "NaN", "NaN", Equal);
    test("1/3", 7, 10, Floor, "NaN", "NaN", Equal);
    test("1/3", 7, 100, Nearest, "NaN", "NaN", Equal);
    test("-7/3", 7, 10, Nearest, "2.242", "0x2.3e", Less);
    test("-7/3", 7, 10, Floor, "2.242", "0x2.3e", Less);
    test(
        "-7/3",
        7,
        100,
        Nearest,
        "2.243440374100183660005536094594",
        "0x2.3e521bbd494a946d42d0e4bd",
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
        "1.2",
        "0x1.4",
        Greater,
    );
    test(
        "10000000000000000000000000",
        5,
        10,
        Floor,
        "1.2480",
        "0x1.3f8",
        Less,
    );
    test(
        "10000000000000000000000000",
        5,
        100,
        Nearest,
        "1.249999999999999999999999920423",
        "0x1.3fffffffffffffffffffe75f4",
        Greater,
    );
    test("-1/2", 7, 10, Nearest, "NaN", "NaN", Equal);
    test("-1/2", 7, 10, Floor, "NaN", "NaN", Equal);
    test("-1/2", 7, 100, Nearest, "NaN", "NaN", Equal);
}

#[test]
fn test_asec_pi_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = asec_pi_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_trig_prec_round_generic(InverseTrigFunction::Asec, &x, Some(2), prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "0.50", "0x0.8", Equal);
    test("0.0", "0x0", 10, Exact, "NaN", "NaN", Equal);
    test("0.50", "0x0.8", 10, Exact, "NaN", "NaN", Equal);
    test("0.50", "0x0.8", 10, Floor, "NaN", "NaN", Equal);
    test("0.50", "0x0.8", 100, Exact, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 10, Exact, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 10, Floor, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 100, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1", 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", 100, Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1", 10, Exact, "1.0", "0x1.0", Equal);
    test("-1.0", "-0x1", 10, Floor, "1.0", "0x1.0", Equal);
    test("-1.0", "-0x1", 100, Exact, "1.0", "0x1.0", Equal);
    test("2.0", "0x2", 10, Nearest, "0.33350", "0x0.556", Greater);
    test("2.0", "0x2", 10, Floor, "0.3330", "0x0.554", Less);
    test(
        "2.0",
        "0x2",
        100,
        Nearest,
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
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
        "0.50",
        "0x0.8",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Floor,
        "0.49951",
        "0x0.7fe",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        100,
        Nearest,
        "0.50",
        "0x0.8",
        Greater,
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
fn test_asec_pi_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = asec_pi_rational_prec_round_all(&x, prec, rm);
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
    test("1", 10, Exact, "0.0", "0x0.0", Equal);
    test("1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1", 100, Exact, "0.0", "0x0.0", Equal);
    test("2", 10, Nearest, "0.33350", "0x0.556", Greater);
    test("2", 10, Floor, "0.3330", "0x0.554", Less);
    test(
        "2",
        100,
        Nearest,
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
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
fn asec_prec_round_fail_1() {
    BigExponentFloat::from(3).asec_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn asec_prec_round_fail_2() {
    BigExponentFloat::from(3).asec_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn asec_prec_round_fail_3() {
    BigExponentFloat::asec_rational_prec_round(Rational::from(3), 0, Floor);
}

#[test]
#[should_panic]
fn asec_prec_round_fail_4() {
    BigExponentFloat::asec_rational_prec_round(Rational::from(3), 10, Exact);
}

#[test]
#[should_panic]
fn asec_prec_round_fail_5() {
    BigExponentFloat::from(3).asec_with_period_prec_round(7, 0, Floor);
}

#[test]
#[should_panic]
fn asec_prec_round_fail_6() {
    BigExponentFloat::from(3).asec_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn asec_prec_round_fail_7() {
    BigExponentFloat::asec_with_period_rational_prec_round(Rational::from(3), 7, 0, Floor);
}

#[test]
#[should_panic]
fn asec_prec_round_fail_8() {
    BigExponentFloat::from(3).asec_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn asec_prec_round_fail_9() {
    BigExponentFloat::from(3).asec_pi_prec_round(10, Exact);
}

#[test]
fn asec_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties(
        |(x, prec, rm)| {
            let r = asec_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with the decomposition wherever both apply.
            assert!(same_pair(
                &inverse_trig_prec_round_generic(InverseTrigFunction::Asec, &x, None, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(&asec_prec_round_all(&x, prec, Exact), &r));
            }
            if rm != Exact
                && big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&asec_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn asec_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_46().test_properties(|(x, prec, rm)| {
        // the `Float` generator's `Exact` cases are not all exact for this function
        if x.is_zero() || rm == Exact {
            return;
        }
        let (y, o) = x.asec_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) {
            let r = asec_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
        if rm != Exact {
            for u in PERIODS {
                let (y, o) = x.asec_with_period_prec_round_ref(u, prec, rm);
                if float_result_comparable(&y, o) {
                    let r = asec_with_period_prec_round_all(
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
fn asec_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_12().test_properties(|(x, prec, rm)| {
        let r = asec_rational_prec_round_all(&x, prec, rm);
        check_result(&r, prec);
        let (y, o) = Float::asec_rational_prec_round_ref(&x, prec, rm);
        if float_result_comparable(&y, o) {
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn asec_with_period_prec_round_properties() {
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_9().test_properties(
        |(x, u, prec, rm)| {
            let r = asec_with_period_prec_round_all(&x, u, prec, rm);
            assert!(same_pair(
                &inverse_trig_prec_round_generic(InverseTrigFunction::Asec, &x, Some(u), prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(
                    &asec_with_period_prec_round_all(&x, u, prec, Exact),
                    &r
                ));
            }
            if rm != Exact
                && big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(
                    &asec_with_period_rational_prec_round_all(&q, u, prec, rm),
                    &r
                ));
            }
        },
    );
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties(
        |(x, prec, rm)| {
            asec_with_period_prec_round_all(&x, 2, prec, rm);
        },
    );
}

#[test]
fn asec_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_9().test_properties(
        |(x, u, prec, rm)| {
            let r = asec_with_period_rational_prec_round_all(&x, u, prec, rm);
            check_result(&r, prec);
            let (y, o) = Float::asec_with_period_rational_prec_round_ref(&x, u, prec, rm);
            if float_result_comparable(&y, o) {
                assert!(same(&r.0, &float_to_big_exponent_float(&y)));
                assert_eq!(r.1, o);
            }
        },
    );
}
