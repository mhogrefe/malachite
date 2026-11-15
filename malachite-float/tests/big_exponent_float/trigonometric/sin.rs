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

fn sin_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().sin_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.sin_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.sin_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().sin_prec(prec), &r));
        assert!(same_pair(&x.sin_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.sin_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn sin_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::sin_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::sin_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::sin_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::sin_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

fn sin_with_period_prec_round_all(
    x: &BigExponentFloat,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().sin_with_period_prec_round(u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.sin_with_period_prec_round_ref(u, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.sin_with_period_prec_round_assign(u, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().sin_with_period_prec(u, prec), &r));
        assert!(same_pair(&x.sin_with_period_prec_ref(u, prec), &r));
        let mut z = x.clone();
        let o = z.sin_with_period_prec_assign(u, prec);
        assert!(same_pair(&(z, o), &r));
    }
    if u == 2 {
        assert!(same_pair(&sin_pi_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn sin_with_period_rational_prec_round_all(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::sin_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::sin_with_period_rational_prec_round_ref(x, u, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::sin_with_period_rational_prec(x.clone(), u, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::sin_with_period_rational_prec_ref(x, u, prec),
            &r
        ));
    }
    if u == 2 {
        assert!(same_pair(&sin_pi_rational_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn sin_pi_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().sin_pi_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.sin_pi_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.sin_pi_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().sin_pi_prec(prec), &r));
        assert!(same_pair(&x.sin_pi_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.sin_pi_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn sin_pi_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::sin_pi_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::sin_pi_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::sin_pi_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::sin_pi_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_sin_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = sin_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_prec_round_generic(TrigFunction::Sin, &x, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", 1, Nearest, "1.0", "0x1.0", Greater);
    test("1.0", "0x1", 10, Floor, "0.84082", "0x0.d74", Less);
    test("1.0", "0x1", 10, Ceiling, "0.8418", "0x0.d78", Greater);
    test(
        "1.0",
        "0x1",
        100,
        Nearest,
        "0.8414709848078965066525023216300",
        "0x0.d76aa47848677020c6e9e909c",
        Less,
    );
    test("-1.0", "-0x1", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-1.0", "-0x1", 10, Floor, "-0.8418", "-0x0.d78", Less);
    test("-1.0", "-0x1", 10, Ceiling, "-0.84082", "-0x0.d74", Greater);
    test(
        "-1.0",
        "-0x1",
        100,
        Nearest,
        "-0.8414709848078965066525023216300",
        "-0x0.d76aa47848677020c6e9e909c",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        Nearest,
        "1.1e-16",
        "0x8.0E-14",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Floor,
        "1.223e-16",
        "0x8.dE-14",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Ceiling,
        "1.2251e-16",
        "0x8.d4E-14",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        100,
        Nearest,
        "1.224646799147353177226065932275e-16",
        "0x8.d313198a2e03707344a40938E-14",
        Less,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18",
        10,
        Floor,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18",
        10,
        Ceiling,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "0.00024", "0x0.001", 1, Nearest, "0.00024", "0x0.001", Greater,
    );
    test(
        "0.00024",
        "0x0.001",
        10,
        Floor,
        "0.00024390",
        "0x0.000ffc",
        Less,
    );
    test(
        "0.00024", "0x0.001", 10, Ceiling, "0.00024", "0x0.001", Greater,
    );
    test(
        "0.00024",
        "0x0.001",
        100,
        Nearest,
        "0.0002441406225746808025002058385388",
        "0x0.000ffffffd55555577777776a76a",
        Less,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        1,
        Nearest,
        "9.6e-35",
        "0x8.0E-29",
        Greater,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        10,
        Floor,
        "8.670e-35",
        "0x7.34E-29",
        Less,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        10,
        Ceiling,
        "8.6799e-35",
        "0x7.36E-29",
        Greater,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        100,
        Nearest,
        "8.6718101301237810247970440260409e-35",
        "0x7.344a4093822299f31d0082ef8E-29",
        Less,
    );
    test(
        "7.6e22",
        "0x1.0E+19",
        1,
        Nearest,
        "-0.50",
        "-0x0.8",
        Greater,
    );
    test("7.6e22", "0x1.0E+19", 10, Floor, "-0.6602", "-0x0.a9", Less);
    test(
        "7.6e22",
        "0x1.0E+19",
        10,
        Ceiling,
        "-0.65918",
        "-0x0.a8c",
        Greater,
    );
    test(
        "7.6e22",
        "0x1.0E+19",
        100,
        Nearest,
        "-0.6600625307954226372299911311654",
        "-0x0.a8f9dba714d23ee03d436a758",
        Less,
    );
    test("1.3e30", "0x1.0E+25", 1, Nearest, "-1.0", "-0x1.0", Less);
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Floor,
        "-0.8730",
        "-0x0.df8",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Ceiling,
        "-0.87207",
        "-0x0.df4",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        100,
        Nearest,
        "-0.87218360541826730978071977821350",
        "-0x0.df476cbd60fac5f54ce60f245",
        Less,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        1,
        Nearest,
        "-0.50",
        "-0x0.8",
        Greater,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        10,
        Floor,
        "-0.70020",
        "-0x0.b34",
        Less,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        10,
        Ceiling,
        "-0.6992",
        "-0x0.b3",
        Greater,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        100,
        Nearest,
        "-0.69924588220729647502440407928844",
        "-0x0.b301c733f168e24db63555fbb",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        1,
        Nearest,
        "5.5e-12041199827",
        "0x2.0E-10000000000",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        10,
        Floor,
        "8.2662e-12041199827",
        "0x2.ffE-10000000000",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        10,
        Ceiling,
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        Greater,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        100,
        Nearest,
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        1,
        Nearest,
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        10,
        Floor,
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        10,
        Ceiling,
        "-2.3802e-323228497",
        "-0xf.fcE-268435457",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        100,
        Nearest,
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        Less,
    );
}

#[test]
fn test_sin_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = sin_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_rational_prec_round_generic(TrigFunction::Sin, &x, prec, rm),
            &r
        ));
    };
    test("0", 10, Exact, "0.0", "0x0.0", Equal);
    test("1/3", 1, Nearest, "0.25", "0x0.4", Less);
    test("1/3", 10, Floor, "0.3271", "0x0.53c", Less);
    test("1/3", 10, Ceiling, "0.32764", "0x0.53e", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "0.3271946967961522441733440852675",
        "0x0.53c3081a2a031ab144c484c79",
        Less,
    );
    test("-22/7", 1, Nearest, "0.00098", "0x0.004", Less);
    test("-22/7", 10, Floor, "0.001263", "0x0.0052c", Less);
    test("-22/7", 10, Ceiling, "0.0012646", "0x0.0052e", Greater);
    test(
        "-22/7",
        100,
        Nearest,
        "0.001264488930377353400360350475647",
        "0x0.0052de9a9a24d90da7cdffe3013",
        Greater,
    );
    test("355/113", 1, Nearest, "-2.4e-7", "-0x4.0E-6", Greater);
    test("355/113", 10, Floor, "-2.6682e-7", "-0x4.7aE-6", Less);
    test("355/113", 10, Ceiling, "-2.664e-7", "-0x4.78E-6", Greater);
    test(
        "355/113",
        100,
        Nearest,
        "-2.667641890624191484063745288735e-7",
        "-0x4.79be53e7511d64791a304597E-6",
        Greater,
    );
    test(
        "1000000000000000000000000000001/7",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "1000000000000000000000000000001/7",
        10,
        Floor,
        "0.99707",
        "0x0.ff4",
        Less,
    );
    test(
        "1000000000000000000000000000001/7",
        10,
        Ceiling,
        "0.9980",
        "0x0.ff8",
        Greater,
    );
    test(
        "1000000000000000000000000000001/7",
        100,
        Nearest,
        "0.99764444542531653878545050657672",
        "0x0.ff65a05a23464b266f30f73c3",
        Less,
    );
}

#[test]
fn test_sin_with_period_prec_round() {
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
        let r = sin_with_period_prec_round_all(&x, u, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_with_period_prec_round_generic(TrigFunction::Sin, &x, u, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 7, 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 7, 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 7, 10, Floor, "NaN", "NaN", Equal);
    test("1.0", "0x1", 0, 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 7, 10, Exact, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", 24, 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", 24, 10, Nearest, "0.2588", "0x0.424", Less);
    test("2.0", "0x2", 24, 10, Exact, "0.50", "0x0.8", Equal);
    test("3.0", "0x3", 24, 10, Nearest, "0.7070", "0x0.b5", Less);
    test("4.0", "0x4", 24, 10, Nearest, "0.86621", "0x0.ddc", Greater);
    test("5.0", "0x5", 24, 10, Nearest, "0.96582", "0x0.f74", Less);
    test("6.0", "0x6", 24, 10, Exact, "1.0", "0x1.0", Equal);
    test("7.0", "0x7", 24, 10, Nearest, "0.96582", "0x0.f74", Less);
    test("8.0", "0x8", 24, 10, Nearest, "0.86621", "0x0.ddc", Greater);
    test("9.00", "0x9", 24, 10, Nearest, "0.7070", "0x0.b5", Less);
    test("10.0", "0xa", 24, 10, Exact, "0.50", "0x0.8", Equal);
    test("11.0", "0xb", 24, 10, Nearest, "0.2588", "0x0.424", Less);
    test("12.0", "0xc", 24, 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "13.0", "0xd", 24, 10, Nearest, "-0.2588", "-0x0.424", Greater,
    );
    test("14.0", "0xe", 24, 10, Exact, "-0.50", "-0x0.8", Equal);
    test(
        "15.0", "0xf", 24, 10, Nearest, "-0.7070", "-0x0.b5", Greater,
    );
    test(
        "16.0", "0x10", 24, 10, Nearest, "-0.86621", "-0x0.ddc", Less,
    );
    test(
        "17.0", "0x11", 24, 10, Nearest, "-0.96582", "-0x0.f74", Greater,
    );
    test("18.0", "0x12", 24, 10, Exact, "-1.0", "-0x1.0", Equal);
    test(
        "19.0", "0x13", 24, 10, Nearest, "-0.96582", "-0x0.f74", Greater,
    );
    test(
        "20.0", "0x14", 24, 10, Nearest, "-0.86621", "-0x0.ddc", Less,
    );
    test(
        "21.0", "0x15", 24, 10, Nearest, "-0.7070", "-0x0.b5", Greater,
    );
    test("22.0", "0x16", 24, 10, Exact, "-0.50", "-0x0.8", Equal);
    test(
        "23.0", "0x17", 24, 10, Nearest, "-0.2588", "-0x0.424", Greater,
    );
    test("24.0", "0x18", 24, 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", 7, 10, Nearest, "0.78223", "0x0.c84", Greater);
    test("1.0", "0x1", 7, 10, Floor, "0.781", "0x0.c8", Less);
    test(
        "1.0",
        "0x1",
        7,
        100,
        Nearest,
        "0.7818314824680298087084445266737",
        "0x0.c8261ba82ef2585d9acce7402",
        Less,
    );
    test("-5.0", "-0x5", 7, 10, Nearest, "0.9746", "0x0.f98", Less);
    test("-5.0", "-0x5", 7, 10, Floor, "0.9746", "0x0.f98", Less);
    test(
        "-5.0",
        "-0x5",
        7,
        100,
        Nearest,
        "0.97492791218182360701813168299400",
        "0x0.f994e02ac74b471caa5277a8f",
        Greater,
    );
    test(
        "0.50", "0x0.8", 3, 10, Nearest, "0.86621", "0x0.ddc", Greater,
    );
    test("0.50", "0x0.8", 3, 10, Floor, "0.8652", "0x0.dd8", Less);
    test(
        "0.50",
        "0x0.8",
        3,
        100,
        Nearest,
        "0.8660254037844386467637231707531",
        "0x0.ddb3d742c265539d92ba16b84",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        10,
        Nearest,
        "0.9746",
        "0x0.f98",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        10,
        Floor,
        "0.9746",
        "0x0.f98",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        100,
        Nearest,
        "0.97492791218182360701813168299400",
        "0x0.f994e02ac74b471caa5277a8f",
        Greater,
    );
    test(
        "5.4e12041199826",
        "0x1.8E+10000000000",
        1000,
        10,
        Nearest,
        "0.3916",
        "0x0.644",
        Greater,
    );
    test(
        "5.4e12041199826",
        "0x1.8E+10000000000",
        1000,
        10,
        Floor,
        "0.39111",
        "0x0.642",
        Less,
    );
    test(
        "5.4e12041199826",
        "0x1.8E+10000000000",
        1000,
        100,
        Nearest,
        "0.3913736668372024181994386277913",
        "0x0.6431108b94d5975596007429e",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        7,
        10,
        Nearest,
        "2.4761e-12041199827",
        "0xe.5cE-10000000001",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        7,
        10,
        Floor,
        "2.4761e-12041199827",
        "0xe.5cE-10000000001",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        7,
        100,
        Nearest,
        "2.4764754085913303615561731683098e-12041199827",
        "0xe.5c8f9dddd0a0285732bde565E-10000000001",
        Greater,
    );
    test(
        "6.7e1204120",
        "0x7.0E+1000000",
        7,
        10,
        Exact,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "6.7e1204120",
        "0x7.0E+1000000",
        7,
        10,
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "6.7e1204120",
        "0x7.0E+1000000",
        7,
        100,
        Exact,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        10,
        Nearest,
        "0.77637",
        "0x0.c6c",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        10,
        Floor,
        "0.77637",
        "0x0.c6c",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        100,
        Nearest,
        "0.776853219615937995273992444381",
        "0x0.c6dfda440af2473007811c99",
        Less,
    );
}

#[test]
fn test_sin_with_period_rational_prec_round() {
    let test =
        |x: &str, u: u64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            let x = Rational::from_str(x).unwrap();
            let r = sin_with_period_rational_prec_round_all(&x, u, prec, rm);
            assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
            assert_eq!(r.0.to_string(), out);
            assert_eq!(r.1, o, "{x} {prec} {rm}");
            assert!(same_pair(
                &trig_with_period_rational_prec_round_generic(TrigFunction::Sin, &x, u, prec, rm),
                &r
            ));
        };
    test("0", 7, 10, Exact, "0.0", "0x0.0", Equal);
    test("1", 0, 10, Floor, "NaN", "NaN", Equal);
    test("1/3", 1, 10, Nearest, "0.86621", "0x0.ddc", Greater);
    test("1/3", 1, 10, Floor, "0.8652", "0x0.dd8", Less);
    test(
        "1/3",
        1,
        100,
        Nearest,
        "0.8660254037844386467637231707531",
        "0x0.ddb3d742c265539d92ba16b84",
        Greater,
    );
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
    test("1/7", 7, 10, Nearest, "0.1279", "0x0.20c", Greater);
    test("1/7", 7, 10, Floor, "0.12769", "0x0.20b", Less);
    test(
        "1/7",
        7,
        100,
        Nearest,
        "0.12787716168450601059056272223094",
        "0x0.20bc8ec3571b1f01845edf80ec",
        Less,
    );
    test(
        "100000000000000000001/3",
        7,
        10,
        Nearest,
        "-0.93066",
        "-0x0.ee4",
        Greater,
    );
    test(
        "100000000000000000001/3",
        7,
        10,
        Floor,
        "-0.9316",
        "-0x0.ee8",
        Less,
    );
    test(
        "100000000000000000001/3",
        7,
        100,
        Nearest,
        "-0.93087374864420425563779924195136",
        "-0x0.ee4dbdf321bc70ab216365d55",
        Less,
    );
    test("-22/7", 2, 10, Nearest, "0.43408", "0x0.6f2", Greater);
    test("-22/7", 2, 10, Floor, "0.4336", "0x0.6f", Less);
    test(
        "-22/7",
        2,
        100,
        Nearest,
        "0.43388373911755812047576833284851",
        "0x0.6f130135c6af04015dcb6bd938",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        5,
        10,
        Nearest,
        "1.257e-24",
        "0x1.85E-20",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        5,
        10,
        Floor,
        "1.2553e-24",
        "0x1.848E-20",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        5,
        100,
        Nearest,
        "1.2566370614359172953850573533115e-24",
        "0x1.84e90b9a6982b2fb17c214742E-20",
        Less,
    );
}

#[test]
fn test_sin_pi_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = sin_pi_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_with_period_prec_round_generic(TrigFunction::Sin, &x, 2, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 10, Exact, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8", 10, Exact, "1.0", "0x1.0", Equal);
    test("0.50", "0x0.8", 10, Ceiling, "1.0", "0x1.0", Equal);
    test("0.50", "0x0.8", 100, Exact, "1.0", "0x1.0", Equal);
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
        "0.50",
        "0x0.8",
        Greater,
    );
    test(
        "0.1666666666666665",
        "0x0.2aaaaaaaaaaaa",
        10,
        Ceiling,
        "0.50",
        "0x0.8",
        Greater,
    );
    test(
        "0.1666666666666665",
        "0x0.2aaaaaaaaaaaa",
        100,
        Nearest,
        "0.4999999999999995972556367553402",
        "0x0.7fffffffffffe2faad8491bc3",
        Greater,
    );
    test("0.38", "0x0.6", 10, Nearest, "0.9238", "0x0.ec8", Less);
    test("0.38", "0x0.6", 10, Ceiling, "0.92480", "0x0.ecc", Greater);
    test(
        "0.38",
        "0x0.6",
        100,
        Nearest,
        "0.92387953251128675612818318939698",
        "0x0.ec835e79946a31457e610231b",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Exact,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        100,
        Exact,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        10,
        Nearest,
        "2.5995e-12041199826",
        "0x9.6cE-10000000000",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        10,
        Ceiling,
        "2.604e-12041199826",
        "0x9.7E-10000000000",
        Greater,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        100,
        Nearest,
        "2.600299179020896879633981826724e-12041199826",
        "0x9.6cbe3f9990e91a79394c9e8aE-10000000000",
        Less,
    );
    test("-1.5", "-0x1.8", 10, Exact, "1.0", "0x1.0", Equal);
    test("-1.5", "-0x1.8", 10, Ceiling, "1.0", "0x1.0", Equal);
    test("-1.5", "-0x1.8", 100, Exact, "1.0", "0x1.0", Equal);
}

#[test]
fn test_sin_pi_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = sin_pi_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_with_period_rational_prec_round_generic(TrigFunction::Sin, &x, 2, prec, rm),
            &r
        ));
    };
    test("0", 10, Exact, "0.0", "0x0.0", Equal);
    test("1/6", 10, Exact, "0.50", "0x0.8", Equal);
    test("1/6", 10, Ceiling, "0.50", "0x0.8", Equal);
    test("1/6", 100, Exact, "0.50", "0x0.8", Equal);
    test("1/3", 10, Nearest, "0.86621", "0x0.ddc", Greater);
    test("1/3", 10, Ceiling, "0.86621", "0x0.ddc", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "0.8660254037844386467637231707531",
        "0x0.ddb3d742c265539d92ba16b84",
        Greater,
    );
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
    test("1/5", 10, Nearest, "0.5879", "0x0.968", Greater);
    test("1/5", 10, Ceiling, "0.5879", "0x0.968", Greater);
    test(
        "1/5",
        100,
        Nearest,
        "0.58778525229247312916870595463909",
        "0x0.96791823aad2ef6da132d6e9b",
        Greater,
    );
    test("7/2", 10, Exact, "-1.0", "-0x1.0", Equal);
    test("7/2", 10, Ceiling, "-1.0", "-0x1.0", Equal);
    test("7/2", 100, Exact, "-1.0", "-0x1.0", Equal);
    test("-5/6", 10, Exact, "-0.50", "-0x0.8", Equal);
    test("-5/6", 10, Ceiling, "-0.50", "-0x0.8", Equal);
    test("-5/6", 100, Exact, "-0.50", "-0x0.8", Equal);
}

#[test]
#[should_panic]
fn sin_prec_round_fail_1() {
    BigExponentFloat::ONE.sin_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn sin_prec_round_fail_2() {
    BigExponentFloat::ONE.sin_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn sin_prec_round_fail_3() {
    BigExponentFloat::sin_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn sin_prec_round_fail_4() {
    BigExponentFloat::sin_rational_prec_round(Rational::ONE, 10, Exact);
}

#[test]
#[should_panic]
fn sin_prec_round_fail_5() {
    BigExponentFloat::ONE.sin_with_period_prec_round(7, 0, Floor);
}

#[test]
#[should_panic]
fn sin_prec_round_fail_6() {
    BigExponentFloat::ONE.sin_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn sin_prec_round_fail_7() {
    BigExponentFloat::sin_with_period_rational_prec_round(Rational::ONE, 7, 0, Floor);
}

#[test]
#[should_panic]
fn sin_prec_round_fail_8() {
    BigExponentFloat::sin_with_period_rational_prec_round(Rational::ONE, 7, 10, Exact);
}

#[test]
#[should_panic]
fn sin_prec_round_fail_9() {
    BigExponentFloat::from(0.375).sin_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn sin_prec_round_fail_10() {
    BigExponentFloat::from(0.375).sin_pi_prec_round(10, Exact);
}

#[test]
fn sin_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_21().test_properties(
        |(x, prec, rm)| {
            let r = sin_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with the implementation that does not use them.
            assert!(same_pair(
                &trig_prec_round_generic(TrigFunction::Sin, &x, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if x.is_finite() && !x.is_zero() {
                // sin is odd
                let (s, o) = sin_prec_round_all(&-&x, prec, -rm);
                assert!(same(&s, &-&r.0));
                assert_eq!(o, r.1.reverse());
            }
            if big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&sin_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn sin_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        let (y, o) = x.sin_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) && !x.is_zero() {
            let r = sin_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn sin_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        let r = sin_rational_prec_round_all(&x, prec, rm);
        assert!(same_pair(
            &trig_rational_prec_round_generic(TrigFunction::Sin, &x, prec, rm),
            &r
        ));
        check_result(&r, prec);
        let (y, o) = Float::sin_rational_prec_round_ref(&x, prec, rm);
        if float_result_comparable(&y, o) && x != 0u32 {
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn sin_with_period_prec_round_properties() {
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_8().test_properties(
        |(x, u, prec, rm)| {
            let r = sin_with_period_prec_round_all(&x, u, prec, rm);
            assert!(same_pair(
                &trig_with_period_prec_round_generic(TrigFunction::Sin, &x, u, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(
                    &sin_with_period_prec_round_all(&x, u, prec, Exact),
                    &r
                ));
            }
            if x.is_finite() && !x.is_zero() && u != 0 {
                let (s, o) = sin_with_period_prec_round_all(&-&x, u, prec, -rm);
                assert!(same(&s, &-&r.0));
                assert_eq!(o, r.1.reverse());
                if big_exponent_float_exponent_is_small(&x) {
                    // The function has period u.
                    let y = &x + BigExponentFloat::from(u);
                    assert!(same_pair(
                        &sin_with_period_prec_round_all(&y, u, prec, rm),
                        &r
                    ));
                    let q = Rational::try_from(&x).unwrap();
                    assert!(same_pair(
                        &sin_with_period_rational_prec_round_all(&q, u, prec, rm),
                        &r
                    ));
                }
            }
        },
    );
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_22().test_properties(
        |(x, prec, rm)| {
            sin_with_period_prec_round_all(&x, 2, prec, rm);
        },
    );
}

#[test]
fn sin_with_period_prec_round_float_agreement_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17().test_properties(
        |(x, u, prec, rm)| {
            if rm == Exact || x.is_zero() {
                return;
            }
            let (y, o) = x.sin_with_period_prec_round_ref(u, prec, rm);
            if float_result_comparable(&y, o) {
                let r =
                    sin_with_period_prec_round_all(&float_to_big_exponent_float(&x), u, prec, rm);
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
fn sin_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_13().test_properties(
        |(x, u, prec, rm)| {
            let r = sin_with_period_rational_prec_round_all(&x, u, prec, rm);
            assert!(same_pair(
                &trig_with_period_rational_prec_round_generic(TrigFunction::Sin, &x, u, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(
                    &sin_with_period_rational_prec_round_all(&x, u, prec, Exact),
                    &r
                ));
            }
            if x != 0u32 {
                let (y, o) = Float::sin_with_period_rational_prec_round_ref(&x, u, prec, rm);
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
                sin_with_period_rational_prec_round_all(&x, 2, prec, rm);
            }
        },
    );
}
