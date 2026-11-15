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
use malachite_float::big_exponent_float::trigonometric::sin_cos::{
    TrigFunction, trig_prec_round_generic, trig_rational_prec_round_generic,
    trig_with_period_prec_round_generic, trig_with_period_rational_prec_round_generic,
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

fn cos_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().cos_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.cos_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.cos_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().cos_prec(prec), &r));
        assert!(same_pair(&x.cos_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.cos_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn cos_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::cos_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::cos_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::cos_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::cos_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

fn cos_with_period_prec_round_all(
    x: &BigExponentFloat,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().cos_with_period_prec_round(u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.cos_with_period_prec_round_ref(u, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.cos_with_period_prec_round_assign(u, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().cos_with_period_prec(u, prec), &r));
        assert!(same_pair(&x.cos_with_period_prec_ref(u, prec), &r));
        let mut z = x.clone();
        let o = z.cos_with_period_prec_assign(u, prec);
        assert!(same_pair(&(z, o), &r));
    }
    if u == 2 {
        assert!(same_pair(&cos_pi_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn cos_with_period_rational_prec_round_all(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::cos_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::cos_with_period_rational_prec_round_ref(x, u, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::cos_with_period_rational_prec(x.clone(), u, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::cos_with_period_rational_prec_ref(x, u, prec),
            &r
        ));
    }
    if u == 2 {
        assert!(same_pair(&cos_pi_rational_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn cos_pi_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().cos_pi_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.cos_pi_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.cos_pi_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().cos_pi_prec(prec), &r));
        assert!(same_pair(&x.cos_pi_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.cos_pi_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn cos_pi_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::cos_pi_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::cos_pi_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::cos_pi_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::cos_pi_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_cos_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = cos_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_prec_round_generic(TrigFunction::Cos, &x, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 10, Exact, "1.0", "0x1.0", Equal);
    test("1.0", "0x1", 1, Nearest, "0.50", "0x0.8", Less);
    test("1.0", "0x1", 10, Floor, "0.54004", "0x0.8a4", Less);
    test("1.0", "0x1", 10, Ceiling, "0.5410", "0x0.8a8", Greater);
    test(
        "1.0",
        "0x1",
        100,
        Nearest,
        "0.54030230586813971740093660744335",
        "0x0.8a51407da8345c91c2466d977",
        Greater,
    );
    test("-1.0", "-0x1", 1, Nearest, "0.50", "0x0.8", Less);
    test("-1.0", "-0x1", 10, Floor, "0.54004", "0x0.8a4", Less);
    test("-1.0", "-0x1", 10, Ceiling, "0.5410", "0x0.8a8", Greater);
    test(
        "-1.0",
        "-0x1",
        100,
        Nearest,
        "0.54030230586813971740093660744335",
        "0x0.8a51407da8345c91c2466d977",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Floor,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        100,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18",
        1,
        Nearest,
        "5.6e-17",
        "0x4.0E-14",
        Less,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18",
        10,
        Floor,
        "6.115e-17",
        "0x4.68E-14",
        Less,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18",
        10,
        Ceiling,
        "6.1257e-17",
        "0x4.6aE-14",
        Greater,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18",
        100,
        Nearest,
        "6.123233995736765886130329661374e-17",
        "0x4.69898cc51701b839a252049cE-14",
        Less,
    );
    test("0.00024", "0x0.001", 1, Nearest, "1.0", "0x1.0", Greater);
    test("0.00024", "0x0.001", 10, Floor, "0.99902", "0x0.ffc", Less);
    test("0.00024", "0x0.001", 10, Ceiling, "1.0", "0x1.0", Greater);
    test(
        "0.00024",
        "0x0.001",
        100,
        Nearest,
        "0.999999970197677760334423822579",
        "0x0.ffffff8000000aaaaaaa4fa5",
        Greater,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        1,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        10,
        Floor,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        100,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test("7.6e22", "0x1.0E+19", 1, Nearest, "-1.0", "-0x1.0", Less);
    test(
        "7.6e22",
        "0x1.0E+19",
        10,
        Floor,
        "-0.7520",
        "-0x0.c08",
        Less,
    );
    test(
        "7.6e22",
        "0x1.0E+19",
        10,
        Ceiling,
        "-0.75098",
        "-0x0.c04",
        Greater,
    );
    test(
        "7.6e22",
        "0x1.0E+19",
        100,
        Nearest,
        "-0.75121065982848096340864210609052",
        "-0x0.c04f57805eb234b9fd4add6af",
        Less,
    );
    test("1.3e30", "0x1.0E+25", 1, Nearest, "0.50", "0x0.8", Greater);
    test("1.3e30", "0x1.0E+25", 10, Floor, "0.48877", "0x0.7d2", Less);
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Ceiling,
        "0.4893",
        "0x0.7d4",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        100,
        Nearest,
        "0.4891786569747214499057893087514",
        "0x0.7d3acffd9b8db34b71d86c7ff",
        Greater,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        1,
        Nearest,
        "0.50",
        "0x0.8",
        Less,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        10,
        Floor,
        "0.7148",
        "0x0.b7",
        Less,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        10,
        Ceiling,
        "0.71582",
        "0x0.b74",
        Greater,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        100,
        Nearest,
        "0.71488124623334445318119153297722",
        "0x0.b7027515188f5e063356388a1",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        10,
        Floor,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        10,
        Ceiling,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        10,
        Floor,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        10,
        Ceiling,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
}

#[test]
fn test_cos_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = cos_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_rational_prec_round_generic(TrigFunction::Cos, &x, prec, rm),
            &r
        ));
    };
    test("0", 10, Exact, "1.0", "0x1.0", Equal);
    test("1/3", 1, Nearest, "1.0", "0x1.0", Greater);
    test("1/3", 10, Floor, "0.94434", "0x0.f1c", Less);
    test("1/3", 10, Ceiling, "0.9453", "0x0.f2", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "0.94495694631473766438828400767583",
        "0x0.f1e8b2cc8cc1656b6998d964d",
        Less,
    );
    test("-22/7", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-22/7", 10, Floor, "-1.0", "-0x1.0", Less);
    test("-22/7", 10, Ceiling, "-0.99902", "-0x0.ffc", Greater);
    test(
        "-22/7",
        100,
        Nearest,
        "-0.9999992005335529032683357396563",
        "-0x0.fffff29651586c28bc01f9fb2",
        Greater,
    );
    test("355/113", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("355/113", 10, Floor, "-1.0", "-0x1.0", Less);
    test("355/113", 10, Ceiling, "-0.99902", "-0x0.ffc", Greater);
    test(
        "355/113",
        100,
        Nearest,
        "-0.9999999999999644184337169343126",
        "-0x0.fffffffffff5fc13f3fa12934",
        Greater,
    );
    test(
        "1000000000000000000000000000001/7",
        1,
        Nearest,
        "0.062",
        "0x0.1",
        Less,
    );
    test(
        "1000000000000000000000000000001/7",
        10,
        Floor,
        "0.068481",
        "0x0.1188",
        Less,
    );
    test(
        "1000000000000000000000000000001/7",
        10,
        Ceiling,
        "0.06860",
        "0x0.119",
        Greater,
    );
    test(
        "1000000000000000000000000000001/7",
        100,
        Nearest,
        "0.06859708821817884018371411125831",
        "0x0.118f942a7f76b441cfa0b3bd88",
        Less,
    );
}

#[test]
fn test_cos_with_period_prec_round() {
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
        let r = cos_with_period_prec_round_all(&x, u, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_with_period_prec_round_generic(TrigFunction::Cos, &x, u, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 7, 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 7, 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 7, 10, Floor, "NaN", "NaN", Equal);
    test("1.0", "0x1", 0, 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 7, 10, Exact, "1.0", "0x1.0", Equal);
    test("0.0", "0x0", 24, 10, Exact, "1.0", "0x1.0", Equal);
    test("1.0", "0x1", 24, 10, Nearest, "0.96582", "0x0.f74", Less);
    test("2.0", "0x2", 24, 10, Nearest, "0.86621", "0x0.ddc", Greater);
    test("3.0", "0x3", 24, 10, Nearest, "0.7070", "0x0.b5", Less);
    test("4.0", "0x4", 24, 10, Exact, "0.50", "0x0.8", Equal);
    test("5.0", "0x5", 24, 10, Nearest, "0.2588", "0x0.424", Less);
    test("6.0", "0x6", 24, 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "7.0", "0x7", 24, 10, Nearest, "-0.2588", "-0x0.424", Greater,
    );
    test("8.0", "0x8", 24, 10, Exact, "-0.50", "-0x0.8", Equal);
    test(
        "9.00", "0x9", 24, 10, Nearest, "-0.7070", "-0x0.b5", Greater,
    );
    test("10.0", "0xa", 24, 10, Nearest, "-0.86621", "-0x0.ddc", Less);
    test(
        "11.0", "0xb", 24, 10, Nearest, "-0.96582", "-0x0.f74", Greater,
    );
    test("12.0", "0xc", 24, 10, Exact, "-1.0", "-0x1.0", Equal);
    test(
        "13.0", "0xd", 24, 10, Nearest, "-0.96582", "-0x0.f74", Greater,
    );
    test("14.0", "0xe", 24, 10, Nearest, "-0.86621", "-0x0.ddc", Less);
    test(
        "15.0", "0xf", 24, 10, Nearest, "-0.7070", "-0x0.b5", Greater,
    );
    test("16.0", "0x10", 24, 10, Exact, "-0.50", "-0x0.8", Equal);
    test(
        "17.0", "0x11", 24, 10, Nearest, "-0.2588", "-0x0.424", Greater,
    );
    test("18.0", "0x12", 24, 10, Exact, "0.0", "0x0.0", Equal);
    test("19.0", "0x13", 24, 10, Nearest, "0.2588", "0x0.424", Less);
    test("20.0", "0x14", 24, 10, Exact, "0.50", "0x0.8", Equal);
    test("21.0", "0x15", 24, 10, Nearest, "0.7070", "0x0.b5", Less);
    test(
        "22.0", "0x16", 24, 10, Nearest, "0.86621", "0x0.ddc", Greater,
    );
    test("23.0", "0x17", 24, 10, Nearest, "0.96582", "0x0.f74", Less);
    test("24.0", "0x18", 24, 10, Exact, "1.0", "0x1.0", Equal);
    test("1.0", "0x1", 7, 10, Nearest, "0.6230", "0x0.9f8", Less);
    test("1.0", "0x1", 7, 10, Floor, "0.6230", "0x0.9f8", Less);
    test(
        "1.0",
        "0x1",
        7,
        100,
        Nearest,
        "0.6234898018587335305250048840045",
        "0x0.9f9d07145f6e8b65f7859e454",
        Greater,
    );
    test(
        "-5.0", "-0x5", 7, 10, Nearest, "-0.22241", "-0x0.38f", Greater,
    );
    test("-5.0", "-0x5", 7, 10, Floor, "-0.223", "-0x0.39", Less);
    test(
        "-5.0",
        "-0x5",
        7,
        100,
        Nearest,
        "-0.22252093395631440428890256449676",
        "-0x0.38f721c6048b04d285fba9b03c",
        Greater,
    );
    test("0.50", "0x0.8", 3, 10, Exact, "0.50", "0x0.8", Equal);
    test("0.50", "0x0.8", 3, 10, Floor, "0.50", "0x0.8", Equal);
    test("0.50", "0x0.8", 3, 100, Exact, "0.50", "0x0.8", Equal);
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        10,
        Nearest,
        "-0.22241",
        "-0x0.38f",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        10,
        Floor,
        "-0.223",
        "-0x0.39",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        100,
        Nearest,
        "-0.22252093395631440428890256449676",
        "-0x0.38f721c6048b04d285fba9b03c",
        Greater,
    );
    test(
        "5.4e12041199826",
        "0x1.8E+10000000000",
        1000,
        10,
        Nearest,
        "0.9199",
        "0x0.eb8",
        Less,
    );
    test(
        "5.4e12041199826",
        "0x1.8E+10000000000",
        1000,
        10,
        Floor,
        "0.9199",
        "0x0.eb8",
        Less,
    );
    test(
        "5.4e12041199826",
        "0x1.8E+10000000000",
        1000,
        100,
        Nearest,
        "0.92023184736587033863851496136251",
        "0x0.eb9450792c904cee28dea6d4b",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        7,
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        7,
        10,
        Floor,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        7,
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "6.7e1204120",
        "0x7.0E+1000000",
        7,
        10,
        Exact,
        "1.0",
        "0x1.0",
        Equal,
    );
    test(
        "6.7e1204120",
        "0x7.0E+1000000",
        7,
        10,
        Floor,
        "1.0",
        "0x1.0",
        Equal,
    );
    test(
        "6.7e1204120",
        "0x7.0E+1000000",
        7,
        100,
        Exact,
        "1.0",
        "0x1.0",
        Equal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        10,
        Nearest,
        "0.62988",
        "0x0.a14",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        10,
        Floor,
        "0.6289",
        "0x0.a1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        100,
        Nearest,
        "0.62968172529647961028780420129084",
        "0x0.a132d251098849909778cf4db",
        Less,
    );
}

#[test]
fn test_cos_with_period_rational_prec_round() {
    let test =
        |x: &str, u: u64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            let x = Rational::from_str(x).unwrap();
            let r = cos_with_period_rational_prec_round_all(&x, u, prec, rm);
            assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
            assert_eq!(r.0.to_string(), out);
            assert_eq!(r.1, o, "{x} {prec} {rm}");
            assert!(same_pair(
                &trig_with_period_rational_prec_round_generic(TrigFunction::Cos, &x, u, prec, rm),
                &r
            ));
        };
    test("0", 7, 10, Exact, "1.0", "0x1.0", Equal);
    test("1", 0, 10, Floor, "NaN", "NaN", Equal);
    test("1/3", 1, 10, Exact, "-0.50", "-0x0.8", Equal);
    test("1/3", 1, 10, Floor, "-0.50", "-0x0.8", Equal);
    test("1/3", 1, 100, Exact, "-0.50", "-0x0.8", Equal);
    test("5/8", 1, 10, Nearest, "-0.7070", "-0x0.b5", Greater);
    test("5/8", 1, 10, Floor, "-0.70801", "-0x0.b54", Less);
    test(
        "5/8",
        1,
        100,
        Nearest,
        "-0.70710678118654752440084436210459",
        "-0x0.b504f333f9de6484597d89b37",
        Greater,
    );
    test("1/7", 7, 10, Nearest, "0.9922", "0x0.fe", Greater);
    test("1/7", 7, 10, Floor, "0.99121", "0x0.fdc", Less);
    test(
        "1/7",
        7,
        100,
        Nearest,
        "0.9917900138232461089574427772189",
        "0x0.fde5f349dec7640d126c636a8",
        Greater,
    );
    test(
        "100000000000000000001/3",
        7,
        10,
        Nearest,
        "0.3652",
        "0x0.5d8",
        Less,
    );
    test(
        "100000000000000000001/3",
        7,
        10,
        Floor,
        "0.3652",
        "0x0.5d8",
        Less,
    );
    test(
        "100000000000000000001/3",
        7,
        100,
        Nearest,
        "0.3653410243663950145447379989296",
        "0x0.5d86fd478a723e3129009a13f",
        Less,
    );
    test("-22/7", 2, 10, Nearest, "-0.90137", "-0x0.e6c", Less);
    test("-22/7", 2, 10, Floor, "-0.90137", "-0x0.e6c", Less);
    test(
        "-22/7",
        2,
        100,
        Nearest,
        "-0.900968867902419126236102319507",
        "-0x0.e6a5e54e5ae386937189f495",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        5,
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        5,
        10,
        Floor,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        5,
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
}

#[test]
fn test_cos_pi_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = cos_pi_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_with_period_prec_round_generic(TrigFunction::Cos, &x, 2, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 10, Exact, "1.0", "0x1.0", Equal);
    test("0.50", "0x0.8", 10, Exact, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8", 100, Exact, "0.0", "0x0.0", Equal);
    test("0.25", "0x0.4", 10, Nearest, "0.7070", "0x0.b5", Less);
    test("0.25", "0x0.4", 10, Ceiling, "0.70801", "0x0.b54", Greater);
    test(
        "0.25",
        "0x0.4",
        100,
        Nearest,
        "0.70710678118654752440084436210459",
        "0x0.b504f333f9de6484597d89b37",
        Less,
    );
    test(
        "0.1666666666666665",
        "0x0.2aaaaaaaaaaaa",
        10,
        Nearest,
        "0.86621",
        "0x0.ddc",
        Greater,
    );
    test(
        "0.1666666666666665",
        "0x0.2aaaaaaaaaaaa",
        10,
        Ceiling,
        "0.86621",
        "0x0.ddc",
        Greater,
    );
    test(
        "0.1666666666666665",
        "0x0.2aaaaaaaaaaaa",
        100,
        Nearest,
        "0.8660254037844388792882897046616",
        "0x0.ddb3d742c265645ee4f2442ba",
        Greater,
    );
    test("0.38", "0x0.6", 10, Nearest, "0.383", "0x0.62", Greater);
    test("0.38", "0x0.6", 10, Ceiling, "0.383", "0x0.62", Greater);
    test(
        "0.38",
        "0x0.6",
        100,
        Nearest,
        "0.3826834323650897717284599840304",
        "0x0.61f78a9abaa58b4698916152d",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Exact,
        "1.0",
        "0x1.0",
        Equal,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Ceiling,
        "1.0",
        "0x1.0",
        Equal,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        100,
        Exact,
        "1.0",
        "0x1.0",
        Equal,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        10,
        Ceiling,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test("-1.5", "-0x1.8", 10, Exact, "0.0", "0x0.0", Equal);
    test("-1.5", "-0x1.8", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("-1.5", "-0x1.8", 100, Exact, "0.0", "0x0.0", Equal);
}

#[test]
fn test_cos_pi_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = cos_pi_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_with_period_rational_prec_round_generic(TrigFunction::Cos, &x, 2, prec, rm),
            &r
        ));
    };
    test("0", 10, Exact, "1.0", "0x1.0", Equal);
    test("1/6", 10, Nearest, "0.86621", "0x0.ddc", Greater);
    test("1/6", 10, Ceiling, "0.86621", "0x0.ddc", Greater);
    test(
        "1/6",
        100,
        Nearest,
        "0.8660254037844386467637231707531",
        "0x0.ddb3d742c265539d92ba16b84",
        Greater,
    );
    test("1/3", 10, Exact, "0.50", "0x0.8", Equal);
    test("1/3", 10, Ceiling, "0.50", "0x0.8", Equal);
    test("1/3", 100, Exact, "0.50", "0x0.8", Equal);
    test("1/4", 10, Nearest, "0.7070", "0x0.b5", Less);
    test("1/4", 10, Ceiling, "0.70801", "0x0.b54", Greater);
    test(
        "1/4",
        100,
        Nearest,
        "0.70710678118654752440084436210459",
        "0x0.b504f333f9de6484597d89b37",
        Less,
    );
    test("1/5", 10, Nearest, "0.8086", "0x0.cf", Less);
    test("1/5", 10, Ceiling, "0.80957", "0x0.cf4", Greater);
    test(
        "1/5",
        100,
        Nearest,
        "0.80901699437494742410229341718289",
        "0x0.cf1bbcdcbfa53e0af9ce60303",
        Greater,
    );
    test("7/2", 10, Exact, "0.0", "0x0.0", Equal);
    test("7/2", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("7/2", 100, Exact, "0.0", "0x0.0", Equal);
    test("-5/6", 10, Nearest, "-0.86621", "-0x0.ddc", Less);
    test("-5/6", 10, Ceiling, "-0.8652", "-0x0.dd8", Greater);
    test(
        "-5/6",
        100,
        Nearest,
        "-0.8660254037844386467637231707531",
        "-0x0.ddb3d742c265539d92ba16b84",
        Less,
    );
}

#[test]
#[should_panic]
fn cos_prec_round_fail_1() {
    BigExponentFloat::ONE.cos_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn cos_prec_round_fail_2() {
    BigExponentFloat::ONE.cos_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn cos_prec_round_fail_3() {
    BigExponentFloat::cos_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn cos_prec_round_fail_4() {
    BigExponentFloat::cos_rational_prec_round(Rational::ONE, 10, Exact);
}

#[test]
#[should_panic]
fn cos_prec_round_fail_5() {
    BigExponentFloat::ONE.cos_with_period_prec_round(7, 0, Floor);
}

#[test]
#[should_panic]
fn cos_prec_round_fail_6() {
    BigExponentFloat::ONE.cos_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn cos_prec_round_fail_7() {
    BigExponentFloat::cos_with_period_rational_prec_round(Rational::ONE, 7, 0, Floor);
}

#[test]
#[should_panic]
fn cos_prec_round_fail_8() {
    BigExponentFloat::cos_with_period_rational_prec_round(Rational::ONE, 7, 10, Exact);
}

#[test]
#[should_panic]
fn cos_prec_round_fail_9() {
    BigExponentFloat::from(0.375).cos_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn cos_prec_round_fail_10() {
    BigExponentFloat::from(0.375).cos_pi_prec_round(10, Exact);
}

#[test]
fn cos_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_21().test_properties(
        |(x, prec, rm)| {
            let r = cos_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with the implementation that does not use them.
            assert!(same_pair(
                &trig_prec_round_generic(TrigFunction::Cos, &x, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if x.is_finite() && !x.is_zero() {
                // cos is even
                assert!(same_pair(&cos_prec_round_all(&-&x, prec, rm), &r));
            }
            if big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&cos_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn cos_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        let (y, o) = x.cos_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) && !x.is_zero() {
            let r = cos_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn cos_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        let r = cos_rational_prec_round_all(&x, prec, rm);
        assert!(same_pair(
            &trig_rational_prec_round_generic(TrigFunction::Cos, &x, prec, rm),
            &r
        ));
        check_result(&r, prec);
        let (y, o) = Float::cos_rational_prec_round_ref(&x, prec, rm);
        if float_result_comparable(&y, o) && x != 0u32 {
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn cos_with_period_prec_round_properties() {
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_8().test_properties(
        |(x, u, prec, rm)| {
            let r = cos_with_period_prec_round_all(&x, u, prec, rm);
            assert!(same_pair(
                &trig_with_period_prec_round_generic(TrigFunction::Cos, &x, u, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(
                    &cos_with_period_prec_round_all(&x, u, prec, Exact),
                    &r
                ));
            }
            if x.is_finite() && !x.is_zero() && u != 0 {
                assert!(same_pair(
                    &cos_with_period_prec_round_all(&-&x, u, prec, rm),
                    &r
                ));
                if big_exponent_float_exponent_is_small(&x) {
                    // The function has period u.
                    let y = &x + BigExponentFloat::from(u);
                    assert!(same_pair(
                        &cos_with_period_prec_round_all(&y, u, prec, rm),
                        &r
                    ));
                    let q = Rational::try_from(&x).unwrap();
                    assert!(same_pair(
                        &cos_with_period_rational_prec_round_all(&q, u, prec, rm),
                        &r
                    ));
                }
            }
        },
    );
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_22().test_properties(
        |(x, prec, rm)| {
            cos_with_period_prec_round_all(&x, 2, prec, rm);
        },
    );
}

#[test]
fn cos_with_period_prec_round_float_agreement_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17().test_properties(
        |(x, u, prec, rm)| {
            if rm == Exact || x.is_zero() {
                return;
            }
            let (y, o) = x.cos_with_period_prec_round_ref(u, prec, rm);
            if float_result_comparable(&y, o) {
                let r =
                    cos_with_period_prec_round_all(&float_to_big_exponent_float(&x), u, prec, rm);
                if y.is_infinite() {
                    // a pole
                    assert!(r.0.is_nan());
                } else {
                    assert!(same(&r.0, &float_to_big_exponent_float(&y)));
                    assert_eq!(r.1, o);
                }
            }
        },
    );
}

#[test]
fn cos_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_13().test_properties(
        |(x, u, prec, rm)| {
            let r = cos_with_period_rational_prec_round_all(&x, u, prec, rm);
            assert!(same_pair(
                &trig_with_period_rational_prec_round_generic(TrigFunction::Cos, &x, u, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(
                    &cos_with_period_rational_prec_round_all(&x, u, prec, Exact),
                    &r
                ));
            }
            if x != 0u32 {
                let (y, o) = Float::cos_with_period_rational_prec_round_ref(&x, u, prec, rm);
                if float_result_comparable(&y, o) {
                    if y.is_infinite() {
                        assert!(r.0.is_nan());
                    } else {
                        assert!(same(&r.0, &float_to_big_exponent_float(&y)));
                        assert_eq!(r.1, o);
                    }
                }
            }
            if rm != Exact {
                cos_with_period_rational_prec_round_all(&x, 2, prec, rm);
            }
        },
    );
}
