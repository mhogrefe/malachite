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

fn acot_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().acot_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.acot_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.acot_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().acot_prec(prec), &r));
        assert!(same_pair(&x.acot_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.acot_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn acot_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::acot_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::acot_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::acot_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::acot_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

fn acot_with_period_prec_round_all(
    x: &BigExponentFloat,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().acot_with_period_prec_round(u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.acot_with_period_prec_round_ref(u, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.acot_with_period_prec_round_assign(u, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().acot_with_period_prec(u, prec), &r));
        assert!(same_pair(&x.acot_with_period_prec_ref(u, prec), &r));
        let mut z = x.clone();
        let o = z.acot_with_period_prec_assign(u, prec);
        assert!(same_pair(&(z, o), &r));
    }
    if u == 2 {
        assert!(same_pair(&acot_pi_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn acot_with_period_rational_prec_round_all(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::acot_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::acot_with_period_rational_prec_round_ref(x, u, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::acot_with_period_rational_prec(x.clone(), u, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::acot_with_period_rational_prec_ref(x, u, prec),
            &r
        ));
    }
    if u == 2 {
        assert!(same_pair(&acot_pi_rational_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn acot_pi_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().acot_pi_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.acot_pi_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.acot_pi_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().acot_pi_prec(prec), &r));
        assert!(same_pair(&x.acot_pi_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.acot_pi_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn acot_pi_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::acot_pi_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::acot_pi_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::acot_pi_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::acot_pi_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_acot_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = acot_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_trig_prec_round_generic(InverseTrigFunction::Acot, &x, None, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", 10, Floor, "1.570", "0x1.92", Less);
    test("0.50", "0x0.8", 10, Floor, "1.105", "0x1.1b", Less);
    test("-0.50", "-0x0.8", 10, Floor, "-1.1074", "-0x1.1b8", Less);
    test("1.0", "0x1", 10, Floor, "0.7852", "0x0.c9", Less);
    test("-1.0", "-0x1", 10, Floor, "-0.78613", "-0x0.c94", Less);
    test("2.0", "0x2", 10, Floor, "0.46338", "0x0.76a", Less);
    test("-2.0", "-0x2", 10, Floor, "-0.4639", "-0x0.76c", Less);
    test("0.38", "0x0.6", 1, Nearest, "1.0", "0x1.0", Less);
    test("0.38", "0x0.6", 10, Floor, "1.211", "0x1.36", Less);
    test("0.38", "0x0.6", 10, Ceiling, "1.2129", "0x1.368", Greater);
    test(
        "0.38",
        "0x0.6",
        100,
        Nearest,
        "1.2120256565243243988354016277139",
        "0x1.3647503caf55c22fc43b3485a",
        Greater,
    );
    test("-0.38", "-0x0.6", 1, Nearest, "-1.0", "-0x1.0", Greater);
    test("-0.38", "-0x0.6", 10, Floor, "-1.2129", "-0x1.368", Less);
    test("-0.38", "-0x0.6", 10, Ceiling, "-1.211", "-0x1.36", Greater);
    test(
        "-0.38",
        "-0x0.6",
        100,
        Nearest,
        "-1.2120256565243243988354016277139",
        "-0x1.3647503caf55c22fc43b3485a",
        Less,
    );
    test("0.9961", "0x0.ff", 1, Nearest, "1.0", "0x1.0", Greater);
    test("0.9961", "0x0.ff", 10, Floor, "0.7871", "0x0.c98", Less);
    test(
        "0.9961", "0x0.ff", 10, Ceiling, "0.78809", "0x0.c9c", Greater,
    );
    test(
        "0.9961",
        "0x0.ff",
        100,
        Nearest,
        "0.7873551080617448543918922157524",
        "0x0.c9901ab776b7abcc153c4e376",
        Greater,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        10,
        Floor,
        "0.7852",
        "0x0.c9",
        Less,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        10,
        Ceiling,
        "0.78613",
        "0x0.c94",
        Greater,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        100,
        Nearest,
        "0.7853981633974483096173549117143",
        "0x0.c90fdaa22168c234ccc6628b8",
        Less,
    );
    test(
        "-0.9999999999999999999966",
        "-0x0.fffffffffffffffff",
        1,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-0.9999999999999999999966",
        "-0x0.fffffffffffffffff",
        10,
        Floor,
        "-0.78613",
        "-0x0.c94",
        Less,
    );
    test(
        "-0.9999999999999999999966",
        "-0x0.fffffffffffffffff",
        10,
        Ceiling,
        "-0.7852",
        "-0x0.c9",
        Greater,
    );
    test(
        "-0.9999999999999999999966",
        "-0x0.fffffffffffffffff",
        100,
        Nearest,
        "-0.7853981633974483096173549117143",
        "-0x0.c90fdaa22168c234ccc6628b8",
        Greater,
    );
    test("3.0", "0x3", 1, Nearest, "0.25", "0x0.4", Less);
    test("3.0", "0x3", 10, Floor, "0.3213", "0x0.524", Less);
    test("3.0", "0x3", 10, Ceiling, "0.32178", "0x0.526", Greater);
    test(
        "3.0",
        "0x3",
        100,
        Nearest,
        "0.3217505543966421934014046143585",
        "0x0.525e3e8c9a7b84920cd43f952",
        Less,
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
        "-0.00097608558557243179537776512541524",
        "-0x0.003ff7ffab0a9b75a35faef39bd4",
        Less,
    );
    test(
        "7.6e22",
        "0x1.0E+19",
        1,
        Nearest,
        "1.3e-23",
        "0x1.0E-19",
        Greater,
    );
    test(
        "7.6e22",
        "0x1.0E+19",
        10,
        Floor,
        "1.3222e-23",
        "0xf.fcE-20",
        Less,
    );
    test(
        "7.6e22",
        "0x1.0E+19",
        10,
        Ceiling,
        "1.3e-23",
        "0x1.0E-19",
        Greater,
    );
    test(
        "7.6e22",
        "0x1.0E+19",
        100,
        Nearest,
        "1.3e-23",
        "0x1.0E-19",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        1,
        Nearest,
        "7.9e-31",
        "0x1.0E-25",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Floor,
        "7.8809e-31",
        "0xf.fcE-26",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Ceiling,
        "7.9e-31",
        "0x1.0E-25",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        100,
        Nearest,
        "7.9e-31",
        "0x1.0E-25",
        Greater,
    );
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
        "-2.0",
        "-0x2.0",
        Less,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Floor,
        "-1.5723",
        "-0x1.928",
        Less,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Ceiling,
        "-1.570",
        "-0x1.92",
        Greater,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        100,
        Nearest,
        "-1.570796326794896619231321691640",
        "-0x1.921fb54442d18469898cc517",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        1,
        Nearest,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Floor,
        "2.7563e-12041199827",
        "0xf.fcE-10000000001",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Ceiling,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        100,
        Nearest,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Greater,
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
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "1.0000000000000000000002",
        "0x1.000000000000000001",
        10,
        Floor,
        "0.7852",
        "0x0.c9",
        Less,
    );
    test(
        "1.0000000000000000000002",
        "0x1.000000000000000001",
        10,
        Ceiling,
        "0.78613",
        "0x0.c94",
        Greater,
    );
    test(
        "1.0000000000000000000002",
        "0x1.000000000000000001",
        100,
        Nearest,
        "0.7853981633974483096155549667014",
        "0x0.c90fdaa22168c234c446628b8",
        Less,
    );
}

#[test]
fn test_acot_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = acot_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("0", 10, Floor, "1.570", "0x1.92", Less);
    test("1", 10, Floor, "0.7852", "0x0.c9", Less);
    test("-1", 10, Floor, "-0.78613", "-0x0.c94", Less);
    test("1/2", 10, Floor, "1.105", "0x1.1b", Less);
    test("2", 10, Floor, "0.46338", "0x0.76a", Less);
    test("1/3", 1, Nearest, "1.0", "0x1.0", Less);
    test("1/3", 10, Floor, "1.2480", "0x1.3f8", Less);
    test("1/3", 10, Ceiling, "1.2", "0x1.4", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "1.2490457723982544258299170772811",
        "0x1.3fc176b7a855ffd77cb88581e",
        Greater,
    );
    test("-2/3", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-2/3", 10, Floor, "-0.98340", "-0x0.fbc", Less);
    test("-2/3", 10, Ceiling, "-0.9824", "-0x0.fb8", Greater);
    test(
        "-2/3",
        100,
        Nearest,
        "-0.9827937232473290679857106110149",
        "-0x0.fb985e940fb4d9007887af0cc",
        Less,
    );
    test("7/3", 1, Nearest, "0.50", "0x0.8", Greater);
    test("7/3", 10, Floor, "0.40479", "0x0.67a", Less);
    test("7/3", 10, Ceiling, "0.4053", "0x0.67c", Greater);
    test(
        "7/3",
        100,
        Nearest,
        "0.40489178628508342331207292900957",
        "0x0.67a6fcf48371c3548e65ac60e8",
        Greater,
    );
    test("-7/3", 1, Nearest, "-0.50", "-0x0.8", Less);
    test("-7/3", 10, Floor, "-0.4053", "-0x0.67c", Less);
    test("-7/3", 10, Ceiling, "-0.40479", "-0x0.67a", Greater);
    test(
        "-7/3",
        100,
        Nearest,
        "-0.40489178628508342331207292900957",
        "-0x0.67a6fcf48371c3548e65ac60e8",
        Less,
    );
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
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "999999999999/1000000000000",
        10,
        Floor,
        "0.7852",
        "0x0.c9",
        Less,
    );
    test(
        "999999999999/1000000000000",
        10,
        Ceiling,
        "0.78613",
        "0x0.c94",
        Greater,
    );
    test(
        "999999999999/1000000000000",
        100,
        Nearest,
        "0.78539816339794830961566109581952",
        "0x0.c90fdaa221f57f00ce3600735",
        Less,
    );
}

#[test]
fn test_acot_with_period_prec_round() {
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
        let r = acot_with_period_prec_round_all(&x, u, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_trig_prec_round_generic(InverseTrigFunction::Acot, &x, Some(u), prec, rm),
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
    test("0.0", "0x0", 24, 10, Exact, "6.0", "0x6.0", Equal);
    test("0.0", "0x0", 7, 10, Exact, "1.8", "0x1.c", Equal);
    test("0.0", "0x0", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8", 24, 10, Nearest, "4.2266", "0x4.3a", Less);
    test("0.50", "0x0.8", 7, 10, Nearest, "1.234", "0x1.3c", Greater);
    test("0.50", "0x0.8", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "-0.50", "-0x0.8", 24, 10, Nearest, "-4.2266", "-0x4.3a", Greater,
    );
    test("-0.50", "-0x0.8", 7, 10, Nearest, "-1.234", "-0x1.3c", Less);
    test("-0.50", "-0x0.8", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", 24, 10, Exact, "3.0", "0x3.0", Equal);
    test("1.0", "0x1", 7, 10, Exact, "0.88", "0x0.e", Equal);
    test("1.0", "0x1", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1", 24, 10, Exact, "-3.0", "-0x3.0", Equal);
    test("-1.0", "-0x1", 7, 10, Exact, "-0.88", "-0x0.e", Equal);
    test("-1.0", "-0x1", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("2.0", "0x2", 24, 10, Nearest, "1.7715", "0x1.c58", Greater);
    test("2.0", "0x2", 7, 10, Nearest, "0.51660", "0x0.844", Greater);
    test("2.0", "0x2", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2", 24, 10, Nearest, "-1.7715", "-0x1.c58", Less);
    test("-2.0", "-0x2", 7, 10, Nearest, "-0.51660", "-0x0.844", Less);
    test("-2.0", "-0x2", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("0.38", "0x0.6", 7, 10, Nearest, "1.3496", "0x1.598", Less);
    test("0.38", "0x0.6", 7, 10, Floor, "1.3496", "0x1.598", Less);
    test(
        "0.38",
        "0x0.6",
        7,
        100,
        Nearest,
        "1.35029912073032152733873530769",
        "0x1.59ad34035ab286bd048819ec",
        Less,
    );
    test(
        "-3.0", "-0x3", 7, 10, Nearest, "-0.3584", "-0x0.5bc", Greater,
    );
    test("-3.0", "-0x3", 7, 10, Floor, "-0.35889", "-0x0.5be", Less);
    test(
        "-3.0",
        "-0x3",
        7,
        100,
        Nearest,
        "-0.35845733822348354038609623321337",
        "-0x0.5bc3dc30ae5ac533a2414003c8",
        Greater,
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
        "-90.0",
        "-0x5a.0",
        Less,
    );
    test(
        "-2.8e-12041199827",
        "-0x1.0E-10000000000",
        360,
        10,
        Floor,
        "-90.0",
        "-0x5a.0",
        Less,
    );
    test(
        "-2.8e-12041199827",
        "-0x1.0E-10000000000",
        360,
        100,
        Nearest,
        "-90.0",
        "-0x5a.0",
        Less,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        5,
        10,
        Nearest,
        "0.62",
        "0x0.a",
        Less,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        5,
        10,
        Floor,
        "0.62",
        "0x0.a",
        Less,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        5,
        100,
        Nearest,
        "0.62500000000000000000134809480522",
        "0x0.a000000000000000065dbf225",
        Greater,
    );
}

#[test]
fn test_acot_with_period_rational_prec_round() {
    let test =
        |x: &str, u: u64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            let x = Rational::from_str(x).unwrap();
            let r = acot_with_period_rational_prec_round_all(&x, u, prec, rm);
            assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
            assert_eq!(r.0.to_string(), out);
            assert_eq!(r.1, o, "{x} {prec} {rm}");
        };
    test("0", 24, 10, Exact, "6.0", "0x6.0", Equal);
    test("0", 7, 10, Exact, "1.8", "0x1.c", Equal);
    test("1", 24, 10, Exact, "3.0", "0x3.0", Equal);
    test("1", 7, 10, Exact, "0.88", "0x0.e", Equal);
    test("1/2", 24, 10, Nearest, "4.2266", "0x4.3a", Less);
    test("1/2", 7, 10, Nearest, "1.234", "0x1.3c", Greater);
    test("-2", 24, 10, Nearest, "-1.7715", "-0x1.c58", Less);
    test("-2", 7, 10, Nearest, "-0.51660", "-0x0.844", Less);
    test("1/3", 7, 10, Nearest, "1.391", "0x1.64", Less);
    test("1/3", 7, 10, Floor, "1.391", "0x1.64", Less);
    test(
        "1/3",
        7,
        100,
        Nearest,
        "1.391542661776516459613903766787",
        "0x1.643c23cf51a53acc5dbebffc4",
        Greater,
    );
    test("-7/3", 7, 10, Nearest, "-0.4512", "-0x0.738", Less);
    test("-7/3", 7, 10, Floor, "-0.4512", "-0x0.738", Less);
    test(
        "-7/3",
        7,
        100,
        Nearest,
        "-0.4510837044320481044674609371983",
        "-0x0.737a38be4b4f2cc5c45d1fbde",
        Greater,
    );
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
    test("-1/2", 7, 10, Nearest, "-1.234", "-0x1.3c", Less);
    test("-1/2", 7, 10, Floor, "-1.234", "-0x1.3c", Less);
    test(
        "-1/2",
        7,
        100,
        Nearest,
        "-1.233457338223483540386096233213",
        "-0x1.3bc3dc30ae5ac533a2414003c",
        Greater,
    );
}

#[test]
fn test_acot_pi_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = acot_pi_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_trig_prec_round_generic(InverseTrigFunction::Acot, &x, Some(2), prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", 10, Exact, "0.50", "0x0.8", Equal);
    test("0.50", "0x0.8", 10, Nearest, "0.3525", "0x0.5a4", Greater);
    test("0.50", "0x0.8", 10, Floor, "0.35205", "0x0.5a2", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "0.35241638234956672582459892377519",
        "0x0.5a37f5c4c419ef3352ee124a38",
        Less,
    );
    test("-0.50", "-0x0.8", 10, Nearest, "-0.3525", "-0x0.5a4", Less);
    test("-0.50", "-0x0.8", 10, Floor, "-0.3525", "-0x0.5a4", Less);
    test(
        "-0.50",
        "-0x0.8",
        100,
        Nearest,
        "-0.35241638234956672582459892377519",
        "-0x0.5a37f5c4c419ef3352ee124a38",
        Greater,
    );
    test("1.0", "0x1", 10, Exact, "0.25", "0x0.4", Equal);
    test("1.0", "0x1", 10, Floor, "0.25", "0x0.4", Equal);
    test("1.0", "0x1", 100, Exact, "0.25", "0x0.4", Equal);
    test("-1.0", "-0x1", 10, Exact, "-0.25", "-0x0.4", Equal);
    test("-1.0", "-0x1", 10, Floor, "-0.25", "-0x0.4", Equal);
    test("-1.0", "-0x1", 100, Exact, "-0.25", "-0x0.4", Equal);
    test("2.0", "0x2", 10, Nearest, "0.14771", "0x0.25d", Greater);
    test("2.0", "0x2", 10, Floor, "0.1475", "0x0.25c", Less);
    test(
        "2.0",
        "0x2",
        100,
        Nearest,
        "0.1475836176504332741754010762248",
        "0x0.25c80a3b3be610ccad11edb5c8",
        Greater,
    );
    test("0.38", "0x0.6", 10, Nearest, "0.3857", "0x0.62c", Less);
    test("0.38", "0x0.6", 10, Floor, "0.3857", "0x0.62c", Less);
    test(
        "0.38",
        "0x0.6",
        100,
        Nearest,
        "0.3857997487800918649539243736257",
        "0x0.62c3c5b7d0c54b116f02508c9",
        Less,
    );
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
fn test_acot_pi_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = acot_pi_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("1/2", 10, Nearest, "0.3525", "0x0.5a4", Greater);
    test("1/2", 10, Floor, "0.35205", "0x0.5a2", Less);
    test(
        "1/2",
        100,
        Nearest,
        "0.35241638234956672582459892377519",
        "0x0.5a37f5c4c419ef3352ee124a38",
        Less,
    );
    test("-1/2", 10, Nearest, "-0.3525", "-0x0.5a4", Less);
    test("-1/2", 10, Floor, "-0.3525", "-0x0.5a4", Less);
    test(
        "-1/2",
        100,
        Nearest,
        "-0.35241638234956672582459892377519",
        "-0x0.5a37f5c4c419ef3352ee124a38",
        Greater,
    );
    test("1", 10, Exact, "0.25", "0x0.4", Equal);
    test("1", 10, Floor, "0.25", "0x0.4", Equal);
    test("1", 100, Exact, "0.25", "0x0.4", Equal);
    test("2", 10, Nearest, "0.14771", "0x0.25d", Greater);
    test("2", 10, Floor, "0.1475", "0x0.25c", Less);
    test(
        "2",
        100,
        Nearest,
        "0.1475836176504332741754010762248",
        "0x0.25c80a3b3be610ccad11edb5c8",
        Greater,
    );
    test("1/3", 10, Nearest, "0.3975", "0x0.65c", Less);
    test("1/3", 10, Floor, "0.3975", "0x0.65c", Less);
    test(
        "1/3",
        100,
        Nearest,
        "0.39758361765043327417540107622481",
        "0x0.65c80a3b3be610ccad11edb5c8",
        Greater,
    );
    test("-5/7", 10, Nearest, "-0.3027", "-0x0.4d8", Less);
    test("-5/7", 10, Floor, "-0.3027", "-0x0.4d8", Less);
    test(
        "-5/7",
        100,
        Nearest,
        "-0.3025684567112534299507781696762",
        "-0x0.4d75205a60432ff2c16aa9c14",
        Greater,
    );
}

#[test]
#[should_panic]
fn acot_prec_round_fail_1() {
    BigExponentFloat::from(0.375).acot_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn acot_prec_round_fail_2() {
    BigExponentFloat::from(0.375).acot_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn acot_prec_round_fail_3() {
    BigExponentFloat::acot_rational_prec_round(Rational::from_unsigneds(1u8, 3), 0, Floor);
}

#[test]
#[should_panic]
fn acot_prec_round_fail_4() {
    BigExponentFloat::acot_rational_prec_round(Rational::from_unsigneds(1u8, 3), 10, Exact);
}

#[test]
#[should_panic]
fn acot_prec_round_fail_5() {
    BigExponentFloat::from(0.375).acot_with_period_prec_round(7, 0, Floor);
}

#[test]
#[should_panic]
fn acot_prec_round_fail_6() {
    BigExponentFloat::from(0.375).acot_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn acot_prec_round_fail_7() {
    BigExponentFloat::acot_with_period_rational_prec_round(
        Rational::from_unsigneds(1u8, 3),
        7,
        0,
        Floor,
    );
}

#[test]
#[should_panic]
fn acot_prec_round_fail_8() {
    BigExponentFloat::from(0.375).acot_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn acot_prec_round_fail_9() {
    BigExponentFloat::from(0.375).acot_pi_prec_round(10, Exact);
}

#[test]
fn acot_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties(
        |(x, prec, rm)| {
            let r = acot_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with the decomposition wherever both apply.
            assert!(same_pair(
                &inverse_trig_prec_round_generic(InverseTrigFunction::Acot, &x, None, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(&acot_prec_round_all(&x, prec, Exact), &r));
            }
            if rm != Exact
                && big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&acot_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn acot_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_52().test_properties(|(x, prec, rm)| {
        // the `Float` generator's `Exact` cases are not all exact for this function
        if x.is_zero() || rm == Exact {
            return;
        }
        let (y, o) = x.acot_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) {
            let r = acot_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
        if rm != Exact {
            for u in PERIODS {
                let (y, o) = x.acot_with_period_prec_round_ref(u, prec, rm);
                if float_result_comparable(&y, o) {
                    let r = acot_with_period_prec_round_all(
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
fn acot_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_14().test_properties(|(x, prec, rm)| {
        let r = acot_rational_prec_round_all(&x, prec, rm);
        check_result(&r, prec);
        let (y, o) = Float::acot_rational_prec_round_ref(&x, prec, rm);
        if float_result_comparable(&y, o) {
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn acot_with_period_prec_round_properties() {
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_9().test_properties(
        |(x, u, prec, rm)| {
            let r = acot_with_period_prec_round_all(&x, u, prec, rm);
            assert!(same_pair(
                &inverse_trig_prec_round_generic(InverseTrigFunction::Acot, &x, Some(u), prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(
                    &acot_with_period_prec_round_all(&x, u, prec, Exact),
                    &r
                ));
            }
            if rm != Exact
                && big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(
                    &acot_with_period_rational_prec_round_all(&q, u, prec, rm),
                    &r
                ));
            }
        },
    );
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties(
        |(x, prec, rm)| {
            acot_with_period_prec_round_all(&x, 2, prec, rm);
        },
    );
}

#[test]
fn acot_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_11().test_properties(
        |(x, u, prec, rm)| {
            let r = acot_with_period_rational_prec_round_all(&x, u, prec, rm);
            check_result(&r, prec);
            let (y, o) = Float::acot_with_period_rational_prec_round_ref(&x, u, prec, rm);
            if float_result_comparable(&y, o) {
                assert!(same(&r.0, &float_to_big_exponent_float(&y)));
                assert_eq!(r.1, o);
            }
        },
    );
}
