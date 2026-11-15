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

fn cot_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().cot_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.cot_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.cot_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().cot_prec(prec), &r));
        assert!(same_pair(&x.cot_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.cot_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn cot_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::cot_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::cot_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::cot_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::cot_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

fn cot_with_period_prec_round_all(
    x: &BigExponentFloat,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().cot_with_period_prec_round(u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.cot_with_period_prec_round_ref(u, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.cot_with_period_prec_round_assign(u, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().cot_with_period_prec(u, prec), &r));
        assert!(same_pair(&x.cot_with_period_prec_ref(u, prec), &r));
        let mut z = x.clone();
        let o = z.cot_with_period_prec_assign(u, prec);
        assert!(same_pair(&(z, o), &r));
    }
    if u == 2 {
        assert!(same_pair(&cot_pi_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn cot_with_period_rational_prec_round_all(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::cot_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::cot_with_period_rational_prec_round_ref(x, u, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::cot_with_period_rational_prec(x.clone(), u, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::cot_with_period_rational_prec_ref(x, u, prec),
            &r
        ));
    }
    if u == 2 {
        assert!(same_pair(&cot_pi_rational_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn cot_pi_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().cot_pi_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.cot_pi_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.cot_pi_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().cot_pi_prec(prec), &r));
        assert!(same_pair(&x.cot_pi_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.cot_pi_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn cot_pi_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::cot_pi_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::cot_pi_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::cot_pi_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::cot_pi_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_cot_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = cot_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_prec_round_generic(TrigFunction::Cot, &x, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 10, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1", 1, Nearest, "0.50", "0x0.8", Less);
    test("1.0", "0x1", 10, Floor, "0.64160", "0x0.a44", Less);
    test("1.0", "0x1", 10, Ceiling, "0.6426", "0x0.a48", Greater);
    test(
        "1.0",
        "0x1",
        100,
        Nearest,
        "0.6420926159343307030064199865942",
        "0x0.a4602e8270e7ec1a29d95ba18",
        Less,
    );
    test("-1.0", "-0x1", 1, Nearest, "-0.50", "-0x0.8", Greater);
    test("-1.0", "-0x1", 10, Floor, "-0.6426", "-0x0.a48", Less);
    test("-1.0", "-0x1", 10, Ceiling, "-0.64160", "-0x0.a44", Greater);
    test(
        "-1.0",
        "-0x1",
        100,
        Nearest,
        "-0.6420926159343307030064199865942",
        "-0x0.a4602e8270e7ec1a29d95ba18",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        Nearest,
        "-9.0e15",
        "-0x2.0E+13",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Floor,
        "-8.1716e15",
        "-0x1.d08E+13",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Ceiling,
        "-8.16e15",
        "-0x1.dE+13",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        100,
        Nearest,
        "-8165619676597684.8779838685207650",
        "-0x1d02967c31cdb4.e0c38d01b656",
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
    test(
        "0.00024", "0x0.001", 1, Nearest, "4.1e3", "0x1.0E+3", Greater,
    );
    test("0.00024", "0x0.001", 10, Floor, "4092.0", "0xffc.0", Less);
    test(
        "0.00024", "0x0.001", 10, Ceiling, "4.1e3", "0x1.0E+3", Greater,
    );
    test(
        "0.00024",
        "0x0.001",
        100,
        Nearest,
        "4095.9999186197913432907708672728",
        "0xfff.fffaaaaaaa4fa4fa46f9c5",
        Greater,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        1,
        Nearest,
        "-1.0e34",
        "-0x2.0E+28",
        Greater,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        10,
        Floor,
        "-1.1541e34",
        "-0x2.39E+28",
        Less,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        10,
        Ceiling,
        "-1.152e34",
        "-0x2.38E+28",
        Greater,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        100,
        Nearest,
        "-1.1531617793685780863830991908192e34",
        "-0x2.388d7aae4ef32745dcc032794E+28",
        Less,
    );
    test("7.6e22", "0x1.0E+19", 1, Nearest, "1.0", "0x1.0", Less);
    test("7.6e22", "0x1.0E+19", 10, Floor, "1.137", "0x1.23", Less);
    test(
        "7.6e22",
        "0x1.0E+19",
        10,
        Ceiling,
        "1.1387",
        "0x1.238",
        Greater,
    );
    test(
        "7.6e22",
        "0x1.0E+19",
        100,
        Nearest,
        "1.138090142646361551138823776822",
        "0x1.2359e02690ebc8525a843c67",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        1,
        Nearest,
        "-0.50",
        "-0x0.8",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Floor,
        "-0.56152",
        "-0x0.8fc",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Ceiling,
        "-0.5605",
        "-0x0.8f8",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        100,
        Nearest,
        "-0.5608666041597162150715467191825",
        "-0x0.8f94f42a48d8d6d91333a3326",
        Greater,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        1,
        Nearest,
        "-1.0",
        "-0x1.0",
        Greater,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        10,
        Floor,
        "-1.023",
        "-0x1.06",
        Less,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        10,
        Ceiling,
        "-1.0215",
        "-0x1.058",
        Greater,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        100,
        Nearest,
        "-1.022360323348193507053504619646",
        "-0x1.05b967f9822b81470ff3c3308",
        Greater,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        1,
        Nearest,
        "9.1e12041199825",
        "0x4.0E+9999999999",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        10,
        Floor,
        "1.207e12041199826",
        "0x5.54E+9999999999",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        10,
        Ceiling,
        "1.2088e12041199826",
        "0x5.56E+9999999999",
        Greater,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        100,
        Nearest,
        "1.2081658445059049654194101037121e12041199826",
        "0x5.5555555555555555555555558E+9999999999",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        1,
        Nearest,
        "-4.2e323228496",
        "-0x1.0E+268435456",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        10,
        Floor,
        "-4.2e323228496",
        "-0x1.0E+268435456",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        10,
        Ceiling,
        "-4.1931e323228496",
        "-0xf.fcE+268435455",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        100,
        Nearest,
        "-4.2e323228496",
        "-0x1.0E+268435456",
        Less,
    );
}

#[test]
fn test_cot_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = cot_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_rational_prec_round_generic(TrigFunction::Cot, &x, prec, rm),
            &r
        ));
    };
    test("0", 10, Exact, "NaN", "NaN", Equal);
    test("1/3", 1, Nearest, "2.0", "0x2.0", Less);
    test("1/3", 10, Floor, "2.8867", "0x2.e3", Less);
    test("1/3", 10, Ceiling, "2.891", "0x2.e4", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "2.888057036277276859205300274354",
        "0x2.e357b4b7cb28b1c850ee99938",
        Less,
    );
    test("-22/7", 1, Nearest, "-1.0e3", "-0x4.0E+2", Less);
    test("-22/7", 10, Floor, "-791.00", "-0x317.0", Less);
    test("-22/7", 10, Ceiling, "-790.0", "-0x316.0", Greater);
    test(
        "-22/7",
        100,
        Nearest,
        "-790.83270443113289503853207687955",
        "-0x316.d52c1e1af337af1a2226a54",
        Less,
    );
    test("355/113", 1, Nearest, "4.2e6", "0x4.0E+5", Greater);
    test("355/113", 10, Floor, "3.7478e6", "0x3.93E+5", Less);
    test("355/113", 10, Ceiling, "3.752e6", "0x3.94E+5", Greater);
    test(
        "355/113",
        100,
        Nearest,
        "3748629.0926627268654052703099727",
        "0x393315.17b8be9571f1d3966c7c",
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
        "0.068726",
        "0x0.1198",
        Less,
    );
    test(
        "1000000000000000000000000000001/7",
        10,
        Ceiling,
        "0.06885",
        "0x0.11a",
        Greater,
    );
    test(
        "1000000000000000000000000000001/7",
        100,
        Nearest,
        "0.06875905392219617617312897333539",
        "0x0.119a317fe654e8a5ca411e9c44",
        Less,
    );
}

#[test]
fn test_cot_with_period_prec_round() {
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
        let r = cot_with_period_prec_round_all(&x, u, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_with_period_prec_round_generic(TrigFunction::Cot, &x, u, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 7, 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 7, 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 7, 10, Floor, "NaN", "NaN", Equal);
    test("1.0", "0x1", 0, 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 7, 10, Exact, "NaN", "NaN", Equal);
    test("0.0", "0x0", 24, 10, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1", 24, 10, Nearest, "3.7305", "0x3.bb", Less);
    test("2.0", "0x2", 24, 10, Nearest, "1.7324", "0x1.bb8", Greater);
    test("3.0", "0x3", 24, 10, Exact, "1.0", "0x1.0", Equal);
    test("4.0", "0x4", 24, 10, Nearest, "0.57715", "0x0.93c", Less);
    test("5.0", "0x5", 24, 10, Nearest, "0.26807", "0x0.44a", Greater);
    test("6.0", "0x6", 24, 10, Exact, "0.0", "0x0.0", Equal);
    test("7.0", "0x7", 24, 10, Nearest, "-0.26807", "-0x0.44a", Less);
    test(
        "8.0", "0x8", 24, 10, Nearest, "-0.57715", "-0x0.93c", Greater,
    );
    test("9.00", "0x9", 24, 10, Exact, "-1.0", "-0x1.0", Equal);
    test("10.0", "0xa", 24, 10, Nearest, "-1.7324", "-0x1.bb8", Less);
    test(
        "11.0", "0xb", 24, 10, Nearest, "-3.7305", "-0x3.bb", Greater,
    );
    test("12.0", "0xc", 24, 10, Exact, "NaN", "NaN", Equal);
    test("13.0", "0xd", 24, 10, Nearest, "3.7305", "0x3.bb", Less);
    test("14.0", "0xe", 24, 10, Nearest, "1.7324", "0x1.bb8", Greater);
    test("15.0", "0xf", 24, 10, Exact, "1.0", "0x1.0", Equal);
    test("16.0", "0x10", 24, 10, Nearest, "0.57715", "0x0.93c", Less);
    test(
        "17.0", "0x11", 24, 10, Nearest, "0.26807", "0x0.44a", Greater,
    );
    test("18.0", "0x12", 24, 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "19.0", "0x13", 24, 10, Nearest, "-0.26807", "-0x0.44a", Less,
    );
    test(
        "20.0", "0x14", 24, 10, Nearest, "-0.57715", "-0x0.93c", Greater,
    );
    test("21.0", "0x15", 24, 10, Exact, "-1.0", "-0x1.0", Equal);
    test("22.0", "0x16", 24, 10, Nearest, "-1.7324", "-0x1.bb8", Less);
    test(
        "23.0", "0x17", 24, 10, Nearest, "-3.7305", "-0x3.bb", Greater,
    );
    test("24.0", "0x18", 24, 10, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1", 7, 10, Nearest, "0.79785", "0x0.cc4", Greater);
    test("1.0", "0x1", 7, 10, Floor, "0.797", "0x0.cc", Less);
    test(
        "1.0",
        "0x1",
        7,
        100,
        Nearest,
        "0.7974733888824039614156882542146",
        "0x0.cc27374cae22920d08fa9eb72",
        Greater,
    );
    test("-5.0", "-0x5", 7, 10, Nearest, "-0.22827", "-0x0.3a7", Less);
    test("-5.0", "-0x5", 7, 10, Floor, "-0.22827", "-0x0.3a7", Less);
    test(
        "-5.0",
        "-0x5",
        7,
        100,
        Nearest,
        "-0.22824347439014993807761136206094",
        "-0x0.3a6e2a1207f6a9b9e8d3ace66c",
        Greater,
    );
    test("0.50", "0x0.8", 3, 10, Nearest, "0.57715", "0x0.93c", Less);
    test("0.50", "0x0.8", 3, 10, Floor, "0.57715", "0x0.93c", Less);
    test(
        "0.50",
        "0x0.8",
        3,
        100,
        Nearest,
        "0.5773502691896257645091487805021",
        "0x0.93cd3a2c8198e2690c7c0f258",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        10,
        Nearest,
        "-0.22827",
        "-0x0.3a7",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        10,
        Floor,
        "-0.22827",
        "-0x0.3a7",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        100,
        Nearest,
        "-0.22824347439014993807761136206094",
        "-0x0.3a6e2a1207f6a9b9e8d3ace66c",
        Greater,
    );
    test(
        "5.4e12041199826",
        "0x1.8E+10000000000",
        1000,
        10,
        Nearest,
        "2.352",
        "0x2.5a",
        Greater,
    );
    test(
        "5.4e12041199826",
        "0x1.8E+10000000000",
        1000,
        10,
        Floor,
        "2.3477",
        "0x2.59",
        Less,
    );
    test(
        "5.4e12041199826",
        "0x1.8E+10000000000",
        1000,
        100,
        Nearest,
        "2.3512870827578039118058033794771",
        "0x2.59edf343f3b72499d9c3c96c4",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        7,
        10,
        Nearest,
        "4.035e12041199826",
        "0x1.1dE+10000000000",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        7,
        10,
        Floor,
        "4.035e12041199826",
        "0x1.1dE+10000000000",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        7,
        100,
        Nearest,
        "4.0379968907860884514823170082660e12041199826",
        "0x1.1d34a60108f725097944d9af6E+10000000000",
        Less,
    );
    test(
        "6.7e1204120",
        "0x7.0E+1000000",
        7,
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "6.7e1204120",
        "0x7.0E+1000000",
        7,
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "6.7e1204120",
        "0x7.0E+1000000",
        7,
        100,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        10,
        Nearest,
        "0.8105",
        "0x0.cf8",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        10,
        Floor,
        "0.8105",
        "0x0.cf8",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        100,
        Nearest,
        "0.81055430987050903061968055342029",
        "0x0.cf807cbc8693604a803e67413",
        Greater,
    );
}

#[test]
fn test_cot_with_period_rational_prec_round() {
    let test =
        |x: &str, u: u64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            let x = Rational::from_str(x).unwrap();
            let r = cot_with_period_rational_prec_round_all(&x, u, prec, rm);
            assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
            assert_eq!(r.0.to_string(), out);
            assert_eq!(r.1, o, "{x} {prec} {rm}");
            assert!(same_pair(
                &trig_with_period_rational_prec_round_generic(TrigFunction::Cot, &x, u, prec, rm),
                &r
            ));
        };
    test("0", 7, 10, Exact, "NaN", "NaN", Equal);
    test("1", 0, 10, Floor, "NaN", "NaN", Equal);
    test("1/3", 1, 10, Nearest, "-0.57715", "-0x0.93c", Greater);
    test("1/3", 1, 10, Floor, "-0.578", "-0x0.94", Less);
    test(
        "1/3",
        1,
        100,
        Nearest,
        "-0.5773502691896257645091487805021",
        "-0x0.93cd3a2c8198e2690c7c0f258",
        Less,
    );
    test("5/8", 1, 10, Exact, "1.0", "0x1.0", Equal);
    test("5/8", 1, 10, Floor, "1.0", "0x1.0", Equal);
    test("5/8", 1, 100, Exact, "1.0", "0x1.0", Equal);
    test("1/7", 7, 10, Nearest, "7.7578", "0x7.c2", Greater);
    test("1/7", 7, 10, Floor, "7.75", "0x7.c", Less);
    test(
        "1/7",
        7,
        100,
        Nearest,
        "7.75580252766444089118974005128",
        "0x7.c17c46428d8b6fdf4b5b6f9",
        Less,
    );
    test(
        "100000000000000000001/3",
        7,
        10,
        Nearest,
        "-0.3926",
        "-0x0.648",
        Less,
    );
    test(
        "100000000000000000001/3",
        7,
        10,
        Floor,
        "-0.3926",
        "-0x0.648",
        Less,
    );
    test(
        "100000000000000000001/3",
        7,
        100,
        Nearest,
        "-0.3924710788101024795045696015201",
        "-0x0.6478fc101d7fa18686c85dbb9",
        Greater,
    );
    test("-22/7", 2, 10, Nearest, "-2.078", "-0x2.14", Less);
    test("-22/7", 2, 10, Floor, "-2.078", "-0x2.14", Less);
    test(
        "-22/7",
        2,
        100,
        Nearest,
        "-2.076521396572336567163538861485",
        "-0x2.1396e7ffb8f14e9eae80f44f",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        5,
        10,
        Nearest,
        "7.957e23",
        "0xa.88E+19",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        5,
        10,
        Floor,
        "7.957e23",
        "0xa.88E+19",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        5,
        100,
        Nearest,
        "795774715459476678844418.8168621",
        "0xa88308a4db40e9d92002.d11de",
        Less,
    );
}

#[test]
fn test_cot_pi_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = cot_pi_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_with_period_prec_round_generic(TrigFunction::Cot, &x, 2, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 10, Exact, "NaN", "NaN", Equal);
    test("0.50", "0x0.8", 10, Exact, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8", 100, Exact, "0.0", "0x0.0", Equal);
    test("0.25", "0x0.4", 10, Exact, "1.0", "0x1.0", Equal);
    test("0.25", "0x0.4", 10, Ceiling, "1.0", "0x1.0", Equal);
    test("0.25", "0x0.4", 100, Exact, "1.0", "0x1.0", Equal);
    test(
        "0.1666666666666665",
        "0x0.2aaaaaaaaaaaa",
        10,
        Nearest,
        "1.7324",
        "0x1.bb8",
        Greater,
    );
    test(
        "0.1666666666666665",
        "0x0.2aaaaaaaaaaaa",
        10,
        Ceiling,
        "1.7324",
        "0x1.bb8",
        Greater,
    );
    test(
        "0.1666666666666665",
        "0x0.2aaaaaaaaaaaa",
        100,
        Nearest,
        "1.732050807568879153723978612778",
        "0x1.bb67ae8584cb2d45b735990bc",
        Less,
    );
    test("0.38", "0x0.6", 10, Nearest, "0.414", "0x0.6a", Less);
    test("0.38", "0x0.6", 10, Ceiling, "0.41455", "0x0.6a2", Greater);
    test(
        "0.38",
        "0x0.6",
        100,
        Nearest,
        "0.41421356237309504880168872420957",
        "0x0.6a09e667f3bcc908b2fb1366e8",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Exact,
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
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        10,
        Nearest,
        "3.8448e12041199825",
        "0x1.b28E+9999999999",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        10,
        Ceiling,
        "3.849e12041199825",
        "0x1.b3E+9999999999",
        Greater,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        100,
        Nearest,
        "3.845711324558179477602206674542e12041199825",
        "0x1.b2995e7b7b60386ff5be3f854E+9999999999",
        Greater,
    );
    test("-1.5", "-0x1.8", 10, Exact, "0.0", "0x0.0", Equal);
    test("-1.5", "-0x1.8", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("-1.5", "-0x1.8", 100, Exact, "0.0", "0x0.0", Equal);
}

#[test]
fn test_cot_pi_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = cot_pi_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_with_period_rational_prec_round_generic(TrigFunction::Cot, &x, 2, prec, rm),
            &r
        ));
    };
    test("0", 10, Exact, "NaN", "NaN", Equal);
    test("1/6", 10, Nearest, "1.7324", "0x1.bb8", Greater);
    test("1/6", 10, Ceiling, "1.7324", "0x1.bb8", Greater);
    test(
        "1/6",
        100,
        Nearest,
        "1.732050807568877293527446341506",
        "0x1.bb67ae8584caa73b25742d708",
        Greater,
    );
    test("1/3", 10, Nearest, "0.57715", "0x0.93c", Less);
    test("1/3", 10, Ceiling, "0.578", "0x0.94", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "0.5773502691896257645091487805021",
        "0x0.93cd3a2c8198e2690c7c0f258",
        Greater,
    );
    test("1/4", 10, Exact, "1.0", "0x1.0", Equal);
    test("1/4", 10, Ceiling, "1.0", "0x1.0", Equal);
    test("1/4", 100, Exact, "1.0", "0x1.0", Equal);
    test("1/5", 10, Nearest, "1.3770", "0x1.608", Greater);
    test("1/5", 10, Ceiling, "1.3770", "0x1.608", Greater);
    test(
        "1/5",
        100,
        Nearest,
        "1.3763819204711735382072095819103",
        "0x1.605a90c73ab78ce92e8998e12",
        Less,
    );
    test("7/2", 10, Exact, "0.0", "0x0.0", Equal);
    test("7/2", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("7/2", 100, Exact, "0.0", "0x0.0", Equal);
    test("-5/6", 10, Nearest, "1.7324", "0x1.bb8", Greater);
    test("-5/6", 10, Ceiling, "1.7324", "0x1.bb8", Greater);
    test(
        "-5/6",
        100,
        Nearest,
        "1.732050807568877293527446341506",
        "0x1.bb67ae8584caa73b25742d708",
        Greater,
    );
}

#[test]
#[should_panic]
fn cot_prec_round_fail_1() {
    BigExponentFloat::ONE.cot_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn cot_prec_round_fail_2() {
    BigExponentFloat::ONE.cot_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn cot_prec_round_fail_3() {
    BigExponentFloat::cot_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn cot_prec_round_fail_4() {
    BigExponentFloat::cot_rational_prec_round(Rational::ONE, 10, Exact);
}

#[test]
#[should_panic]
fn cot_prec_round_fail_5() {
    BigExponentFloat::ONE.cot_with_period_prec_round(7, 0, Floor);
}

#[test]
#[should_panic]
fn cot_prec_round_fail_6() {
    BigExponentFloat::ONE.cot_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn cot_prec_round_fail_7() {
    BigExponentFloat::cot_with_period_rational_prec_round(Rational::ONE, 7, 0, Floor);
}

#[test]
#[should_panic]
fn cot_prec_round_fail_8() {
    BigExponentFloat::cot_with_period_rational_prec_round(Rational::ONE, 7, 10, Exact);
}

#[test]
#[should_panic]
fn cot_prec_round_fail_9() {
    BigExponentFloat::from(0.375).cot_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn cot_prec_round_fail_10() {
    BigExponentFloat::from(0.375).cot_pi_prec_round(10, Exact);
}

#[test]
fn cot_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_21().test_properties(
        |(x, prec, rm)| {
            let r = cot_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with the implementation that does not use them.
            assert!(same_pair(
                &trig_prec_round_generic(TrigFunction::Cot, &x, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if x.is_finite() && !x.is_zero() {
                // cot is odd
                let (s, o) = cot_prec_round_all(&-&x, prec, -rm);
                assert!(same(&s, &-&r.0));
                assert_eq!(o, r.1.reverse());
            }
            if big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&cot_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn cot_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        let (y, o) = x.cot_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) && !x.is_zero() {
            let r = cot_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn cot_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        let r = cot_rational_prec_round_all(&x, prec, rm);
        assert!(same_pair(
            &trig_rational_prec_round_generic(TrigFunction::Cot, &x, prec, rm),
            &r
        ));
        check_result(&r, prec);
        let (y, o) = Float::cot_rational_prec_round_ref(&x, prec, rm);
        if float_result_comparable(&y, o) && x != 0u32 {
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn cot_with_period_prec_round_properties() {
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_8().test_properties(
        |(x, u, prec, rm)| {
            let r = cot_with_period_prec_round_all(&x, u, prec, rm);
            assert!(same_pair(
                &trig_with_period_prec_round_generic(TrigFunction::Cot, &x, u, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(
                    &cot_with_period_prec_round_all(&x, u, prec, Exact),
                    &r
                ));
            }
            if x.is_finite() && !x.is_zero() && u != 0 {
                let (s, o) = cot_with_period_prec_round_all(&-&x, u, prec, -rm);
                assert!(same(&s, &-&r.0));
                assert_eq!(o, r.1.reverse());
                if big_exponent_float_exponent_is_small(&x) {
                    // The function has period u.
                    let y = &x + BigExponentFloat::from(u);
                    assert!(same_pair(
                        &cot_with_period_prec_round_all(&y, u, prec, rm),
                        &r
                    ));
                    let q = Rational::try_from(&x).unwrap();
                    assert!(same_pair(
                        &cot_with_period_rational_prec_round_all(&q, u, prec, rm),
                        &r
                    ));
                }
            }
        },
    );
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_22().test_properties(
        |(x, prec, rm)| {
            cot_with_period_prec_round_all(&x, 2, prec, rm);
        },
    );
}

#[test]
fn cot_with_period_prec_round_float_agreement_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17().test_properties(
        |(x, u, prec, rm)| {
            if rm == Exact || x.is_zero() {
                return;
            }
            let (y, o) = x.cot_with_period_prec_round_ref(u, prec, rm);
            if float_result_comparable(&y, o) {
                let r =
                    cot_with_period_prec_round_all(&float_to_big_exponent_float(&x), u, prec, rm);
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
fn cot_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_13().test_properties(
        |(x, u, prec, rm)| {
            let r = cot_with_period_rational_prec_round_all(&x, u, prec, rm);
            assert!(same_pair(
                &trig_with_period_rational_prec_round_generic(TrigFunction::Cot, &x, u, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(
                    &cot_with_period_rational_prec_round_all(&x, u, prec, Exact),
                    &r
                ));
            }
            if x != 0u32 {
                let (y, o) = Float::cot_with_period_rational_prec_round_ref(&x, u, prec, rm);
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
                cot_with_period_rational_prec_round_all(&x, 2, prec, rm);
            }
        },
    );
}
