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

fn csc_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().csc_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.csc_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.csc_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().csc_prec(prec), &r));
        assert!(same_pair(&x.csc_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.csc_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn csc_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::csc_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::csc_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::csc_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::csc_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

fn csc_with_period_prec_round_all(
    x: &BigExponentFloat,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().csc_with_period_prec_round(u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.csc_with_period_prec_round_ref(u, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.csc_with_period_prec_round_assign(u, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().csc_with_period_prec(u, prec), &r));
        assert!(same_pair(&x.csc_with_period_prec_ref(u, prec), &r));
        let mut z = x.clone();
        let o = z.csc_with_period_prec_assign(u, prec);
        assert!(same_pair(&(z, o), &r));
    }
    if u == 2 {
        assert!(same_pair(&csc_pi_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn csc_with_period_rational_prec_round_all(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::csc_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::csc_with_period_rational_prec_round_ref(x, u, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::csc_with_period_rational_prec(x.clone(), u, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::csc_with_period_rational_prec_ref(x, u, prec),
            &r
        ));
    }
    if u == 2 {
        assert!(same_pair(&csc_pi_rational_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn csc_pi_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().csc_pi_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.csc_pi_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.csc_pi_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().csc_pi_prec(prec), &r));
        assert!(same_pair(&x.csc_pi_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.csc_pi_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn csc_pi_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::csc_pi_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::csc_pi_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::csc_pi_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::csc_pi_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_csc_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = csc_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_prec_round_generic(TrigFunction::Csc, &x, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 10, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1", 1, Nearest, "1.0", "0x1.0", Less);
    test("1.0", "0x1", 10, Floor, "1.19", "0x1.3", Less);
    test("1.0", "0x1", 10, Ceiling, "1.1895", "0x1.308", Greater);
    test(
        "1.0",
        "0x1",
        100,
        Nearest,
        "1.188395105778121216261599452374",
        "0x1.303aa9620b223e32e68b9bddc",
        Less,
    );
    test("-1.0", "-0x1", 1, Nearest, "-1.0", "-0x1.0", Greater);
    test("-1.0", "-0x1", 10, Floor, "-1.1895", "-0x1.308", Less);
    test("-1.0", "-0x1", 10, Ceiling, "-1.19", "-0x1.3", Greater);
    test(
        "-1.0",
        "-0x1",
        100,
        Nearest,
        "-1.188395105778121216261599452374",
        "-0x1.303aa9620b223e32e68b9bddc",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        Nearest,
        "9.0e15",
        "0x2.0E+13",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Floor,
        "8.16e15",
        "0x1.dE+13",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Ceiling,
        "8.1716e15",
        "0x1.d08E+13",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        100,
        Nearest,
        "8165619676597684.8779838685207650",
        "0x1d02967c31cdb4.e0c38d01b656",
        Greater,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18",
        10,
        Floor,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test("0.00024", "0x0.001", 1, Nearest, "4.1e3", "0x1.0E+3", Less);
    test("0.00024", "0x0.001", 10, Floor, "4.1e3", "0x1.0E+3", Less);
    test(
        "0.00024", "0x0.001", 10, Ceiling, "4104.0", "0x1008.0", Greater,
    );
    test(
        "0.00024",
        "0x0.001",
        100,
        Nearest,
        "4096.00004069010444962057566323",
        "0x1000.0002aaaaaafa4fa502b58",
        Less,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        1,
        Nearest,
        "1.0e34",
        "0x2.0E+28",
        Less,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        10,
        Floor,
        "1.152e34",
        "0x2.38E+28",
        Less,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        10,
        Ceiling,
        "1.1541e34",
        "0x2.39E+28",
        Greater,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        100,
        Nearest,
        "1.1531617793685780863830991908192e34",
        "0x2.388d7aae4ef32745dcc032794E+28",
        Greater,
    );
    test("7.6e22", "0x1.0E+19", 1, Nearest, "-2.0", "-0x2.0", Less);
    test("7.6e22", "0x1.0E+19", 10, Floor, "-1.516", "-0x1.84", Less);
    test(
        "7.6e22",
        "0x1.0E+19",
        10,
        Ceiling,
        "-1.5137",
        "-0x1.838",
        Greater,
    );
    test(
        "7.6e22",
        "0x1.0E+19",
        100,
        Nearest,
        "-1.5150079777970859198387856801816",
        "-0x1.83d79015d14ccd375b4fdbcee",
        Greater,
    );
    test("1.3e30", "0x1.0E+25", 1, Nearest, "-1.0", "-0x1.0", Greater);
    test("1.3e30", "0x1.0E+25", 10, Floor, "-1.148", "-0x1.26", Less);
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Ceiling,
        "-1.1465",
        "-0x1.258",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        100,
        Nearest,
        "-1.1465475775830900068211209364114",
        "-0x1.2584245d0703501ed9171fbba",
        Less,
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
        "-1.4316",
        "-0x1.6e8",
        Less,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        10,
        Ceiling,
        "-1.430",
        "-0x1.6e",
        Greater,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        100,
        Nearest,
        "-1.4301121042619780518017536271321",
        "-0x1.6e1bd3ad6b3f8474f867b967e",
        Less,
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
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        10,
        Floor,
        "-4.2054e323228496",
        "-0x1.008E+268435456",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        10,
        Ceiling,
        "-4.2e323228496",
        "-0x1.0E+268435456",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        100,
        Nearest,
        "-4.2e323228496",
        "-0x1.0E+268435456",
        Greater,
    );
}

#[test]
fn test_csc_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = csc_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_rational_prec_round_generic(TrigFunction::Csc, &x, prec, rm),
            &r
        ));
    };
    test("0", 10, Exact, "NaN", "NaN", Equal);
    test("1/3", 1, Nearest, "4.0", "0x4.0", Greater);
    test("1/3", 10, Floor, "3.055", "0x3.0e", Less);
    test("1/3", 10, Ceiling, "3.0586", "0x3.0f", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "3.0562842545795193204625163549239",
        "0x3.0e68a518b2e2d583b9214f68c",
        Less,
    );
    test("-22/7", 1, Nearest, "1.0e3", "0x4.0E+2", Greater);
    test("-22/7", 10, Floor, "790.0", "0x316.0", Less);
    test("-22/7", 10, Ceiling, "791.00", "0x317.0", Greater);
    test(
        "-22/7",
        100,
        Nearest,
        "790.83333667585081293439700481863",
        "0x316.d5558d69562b0f1ad779954",
        Less,
    );
    test("355/113", 1, Nearest, "-4.2e6", "-0x4.0E+5", Less);
    test("355/113", 10, Floor, "-3.752e6", "-0x3.94E+5", Less);
    test("355/113", 10, Ceiling, "-3.7478e6", "-0x3.93E+5", Greater);
    test(
        "355/113",
        100,
        Nearest,
        "-3748629.0926628602474998015219207",
        "-0x393315.17b8c0d2511bc73f0664",
        Less,
    );
    test(
        "1000000000000000000000000000001/7",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "1000000000000000000000000000001/7",
        10,
        Floor,
        "1.0020",
        "0x1.008",
        Less,
    );
    test(
        "1000000000000000000000000000001/7",
        10,
        Ceiling,
        "1.004",
        "0x1.01",
        Greater,
    );
    test(
        "1000000000000000000000000000001/7",
        100,
        Nearest,
        "1.0023611163130159456725363584483",
        "0x1.009abcf558b150327c1ccd816",
        Less,
    );
}

#[test]
fn test_csc_with_period_prec_round() {
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
        let r = csc_with_period_prec_round_all(&x, u, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_with_period_prec_round_generic(TrigFunction::Csc, &x, u, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 7, 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 7, 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 7, 10, Floor, "NaN", "NaN", Equal);
    test("1.0", "0x1", 0, 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 7, 10, Exact, "NaN", "NaN", Equal);
    test("0.0", "0x0", 24, 10, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1", 24, 10, Nearest, "3.8633", "0x3.dd", Less);
    test("2.0", "0x2", 24, 10, Exact, "2.0", "0x2.0", Equal);
    test("3.0", "0x3", 24, 10, Nearest, "1.414", "0x1.6a", Less);
    test("4.0", "0x4", 24, 10, Nearest, "1.1543", "0x1.278", Less);
    test("5.0", "0x5", 24, 10, Nearest, "1.035", "0x1.09", Less);
    test("6.0", "0x6", 24, 10, Exact, "1.0", "0x1.0", Equal);
    test("7.0", "0x7", 24, 10, Nearest, "1.035", "0x1.09", Less);
    test("8.0", "0x8", 24, 10, Nearest, "1.1543", "0x1.278", Less);
    test("9.00", "0x9", 24, 10, Nearest, "1.414", "0x1.6a", Less);
    test("10.0", "0xa", 24, 10, Exact, "2.0", "0x2.0", Equal);
    test("11.0", "0xb", 24, 10, Nearest, "3.8633", "0x3.dd", Less);
    test("12.0", "0xc", 24, 10, Exact, "NaN", "NaN", Equal);
    test(
        "13.0", "0xd", 24, 10, Nearest, "-3.8633", "-0x3.dd", Greater,
    );
    test("14.0", "0xe", 24, 10, Exact, "-2.0", "-0x2.0", Equal);
    test("15.0", "0xf", 24, 10, Nearest, "-1.414", "-0x1.6a", Greater);
    test(
        "16.0", "0x10", 24, 10, Nearest, "-1.1543", "-0x1.278", Greater,
    );
    test(
        "17.0", "0x11", 24, 10, Nearest, "-1.035", "-0x1.09", Greater,
    );
    test("18.0", "0x12", 24, 10, Exact, "-1.0", "-0x1.0", Equal);
    test(
        "19.0", "0x13", 24, 10, Nearest, "-1.035", "-0x1.09", Greater,
    );
    test(
        "20.0", "0x14", 24, 10, Nearest, "-1.1543", "-0x1.278", Greater,
    );
    test(
        "21.0", "0x15", 24, 10, Nearest, "-1.414", "-0x1.6a", Greater,
    );
    test("22.0", "0x16", 24, 10, Exact, "-2.0", "-0x2.0", Equal);
    test(
        "23.0", "0x17", 24, 10, Nearest, "-3.8633", "-0x3.dd", Greater,
    );
    test("24.0", "0x18", 24, 10, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1", 7, 10, Nearest, "1.2793", "0x1.478", Greater);
    test("1.0", "0x1", 7, 10, Floor, "1.277", "0x1.47", Less);
    test(
        "1.0",
        "0x1",
        7,
        100,
        Nearest,
        "1.27904800768993260574785060727",
        "0x1.476fb0b30acebc91a5865598",
        Greater,
    );
    test("-5.0", "-0x5", 7, 10, Nearest, "1.0254", "0x1.068", Less);
    test("-5.0", "-0x5", 7, 10, Floor, "1.0254", "0x1.068", Less);
    test(
        "-5.0",
        "-0x5",
        7,
        100,
        Nearest,
        "1.025716863272553899493299616275",
        "0x1.0695615eb6193bc6f1ce4b9d8",
        Less,
    );
    test("0.50", "0x0.8", 3, 10, Nearest, "1.1543", "0x1.278", Less);
    test("0.50", "0x0.8", 3, 10, Floor, "1.1543", "0x1.278", Less);
    test(
        "0.50",
        "0x0.8",
        3,
        100,
        Nearest,
        "1.154700538379251529018297561004",
        "0x1.279a74590331c4d218f81e4b",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        10,
        Nearest,
        "1.0254",
        "0x1.068",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        10,
        Floor,
        "1.0254",
        "0x1.068",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        100,
        Nearest,
        "1.025716863272553899493299616275",
        "0x1.0695615eb6193bc6f1ce4b9d8",
        Less,
    );
    test(
        "5.4e12041199826",
        "0x1.8E+10000000000",
        1000,
        10,
        Nearest,
        "2.555",
        "0x2.8e",
        Less,
    );
    test(
        "5.4e12041199826",
        "0x1.8E+10000000000",
        1000,
        10,
        Floor,
        "2.555",
        "0x2.8e",
        Less,
    );
    test(
        "5.4e12041199826",
        "0x1.8E+10000000000",
        1000,
        100,
        Nearest,
        "2.5551029226909243668292818471557",
        "0x2.8e1b39a2df1d03a28a7f0d7bc",
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
        "1.2871",
        "0x1.498",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        10,
        Floor,
        "1.2871",
        "0x1.498",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        100,
        Nearest,
        "1.28724445590169747421017931786",
        "0x1.4988da480e1b59d8e5d54b6e",
        Greater,
    );
}

#[test]
fn test_csc_with_period_rational_prec_round() {
    let test =
        |x: &str, u: u64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            let x = Rational::from_str(x).unwrap();
            let r = csc_with_period_rational_prec_round_all(&x, u, prec, rm);
            assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
            assert_eq!(r.0.to_string(), out);
            assert_eq!(r.1, o, "{x} {prec} {rm}");
            assert!(same_pair(
                &trig_with_period_rational_prec_round_generic(TrigFunction::Csc, &x, u, prec, rm),
                &r
            ));
        };
    test("0", 7, 10, Exact, "NaN", "NaN", Equal);
    test("1", 0, 10, Floor, "NaN", "NaN", Equal);
    test("1/3", 1, 10, Nearest, "1.1543", "0x1.278", Less);
    test("1/3", 1, 10, Floor, "1.1543", "0x1.278", Less);
    test(
        "1/3",
        1,
        100,
        Nearest,
        "1.154700538379251529018297561004",
        "0x1.279a74590331c4d218f81e4b",
        Greater,
    );
    test("5/8", 1, 10, Nearest, "-1.414", "-0x1.6a", Greater);
    test("5/8", 1, 10, Floor, "-1.4160", "-0x1.6a8", Less);
    test(
        "5/8",
        1,
        100,
        Nearest,
        "-1.4142135623730950488016887242092",
        "-0x1.6a09e667f3bcc908b2fb1366e",
        Greater,
    );
    test("1/7", 7, 10, Nearest, "7.8203", "0x7.d2", Greater);
    test("1/7", 7, 10, Floor, "7.812", "0x7.d", Less);
    test(
        "1/7",
        7,
        100,
        Nearest,
        "7.8200046578071889544434169985",
        "0x7.d1ebd343d97bf4243eaa644",
        Greater,
    );
    test(
        "100000000000000000001/3",
        7,
        10,
        Nearest,
        "-1.074",
        "-0x1.13",
        Greater,
    );
    test(
        "100000000000000000001/3",
        7,
        10,
        Floor,
        "-1.0762",
        "-0x1.138",
        Less,
    );
    test(
        "100000000000000000001/3",
        7,
        100,
        Nearest,
        "-1.0742595346108712436739542242785",
        "-0x1.1302ac4091e322f0652c2f412",
        Less,
    );
    test("-22/7", 2, 10, Nearest, "2.305", "0x2.4e", Less);
    test("-22/7", 2, 10, Floor, "2.305", "0x2.4e", Less);
    test(
        "-22/7",
        2,
        100,
        Nearest,
        "2.304764870962486505241150223547",
        "0x2.4e051211c0e7f8589754a1358",
        Less,
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
fn test_csc_pi_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = csc_pi_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_with_period_prec_round_generic(TrigFunction::Csc, &x, 2, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 10, Exact, "NaN", "NaN", Equal);
    test("0.50", "0x0.8", 10, Exact, "1.0", "0x1.0", Equal);
    test("0.50", "0x0.8", 10, Ceiling, "1.0", "0x1.0", Equal);
    test("0.50", "0x0.8", 100, Exact, "1.0", "0x1.0", Equal);
    test("0.25", "0x0.4", 10, Nearest, "1.414", "0x1.6a", Less);
    test("0.25", "0x0.4", 10, Ceiling, "1.4160", "0x1.6a8", Greater);
    test(
        "0.25",
        "0x0.4",
        100,
        Nearest,
        "1.4142135623730950488016887242092",
        "0x1.6a09e667f3bcc908b2fb1366e",
        Less,
    );
    test(
        "0.1666666666666665",
        "0x0.2aaaaaaaaaaaa",
        10,
        Nearest,
        "2.0",
        "0x2.0",
        Less,
    );
    test(
        "0.1666666666666665",
        "0x0.2aaaaaaaaaaaa",
        10,
        Ceiling,
        "2.0039",
        "0x2.01",
        Greater,
    );
    test(
        "0.1666666666666665",
        "0x0.2aaaaaaaaaaaa",
        100,
        Nearest,
        "2.000000000000001610977452978642",
        "0x2.000000000000741549edb90f8",
        Greater,
    );
    test("0.38", "0x0.6", 10, Nearest, "1.082", "0x1.15", Less);
    test("0.38", "0x0.6", 10, Ceiling, "1.0840", "0x1.158", Greater);
    test(
        "0.38",
        "0x0.6",
        100,
        Nearest,
        "1.082392200292393968799446410733",
        "0x1.1517a7bdb3894bfdcb9f41bdc",
        Greater,
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
    test("-1.5", "-0x1.8", 10, Exact, "1.0", "0x1.0", Equal);
    test("-1.5", "-0x1.8", 10, Ceiling, "1.0", "0x1.0", Equal);
    test("-1.5", "-0x1.8", 100, Exact, "1.0", "0x1.0", Equal);
}

#[test]
fn test_csc_pi_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = csc_pi_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_with_period_rational_prec_round_generic(TrigFunction::Csc, &x, 2, prec, rm),
            &r
        ));
    };
    test("0", 10, Exact, "NaN", "NaN", Equal);
    test("1/6", 10, Exact, "2.0", "0x2.0", Equal);
    test("1/6", 10, Ceiling, "2.0", "0x2.0", Equal);
    test("1/6", 100, Exact, "2.0", "0x2.0", Equal);
    test("1/3", 10, Nearest, "1.1543", "0x1.278", Less);
    test("1/3", 10, Ceiling, "1.16", "0x1.28", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "1.154700538379251529018297561004",
        "0x1.279a74590331c4d218f81e4b",
        Greater,
    );
    test("1/4", 10, Nearest, "1.414", "0x1.6a", Less);
    test("1/4", 10, Ceiling, "1.4160", "0x1.6a8", Greater);
    test(
        "1/4",
        100,
        Nearest,
        "1.4142135623730950488016887242092",
        "0x1.6a09e667f3bcc908b2fb1366e",
        Less,
    );
    test("1/5", 10, Nearest, "1.7012", "0x1.b38", Less);
    test("1/5", 10, Ceiling, "1.703", "0x1.b4", Greater);
    test(
        "1/5",
        100,
        Nearest,
        "1.701301616704079864363080994126",
        "0x1.b38880b4603e47d84b4a2f23",
        Less,
    );
    test("7/2", 10, Exact, "-1.0", "-0x1.0", Equal);
    test("7/2", 10, Ceiling, "-1.0", "-0x1.0", Equal);
    test("7/2", 100, Exact, "-1.0", "-0x1.0", Equal);
    test("-5/6", 10, Exact, "-2.0", "-0x2.0", Equal);
    test("-5/6", 10, Ceiling, "-2.0", "-0x2.0", Equal);
    test("-5/6", 100, Exact, "-2.0", "-0x2.0", Equal);
}

#[test]
#[should_panic]
fn csc_prec_round_fail_1() {
    BigExponentFloat::ONE.csc_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn csc_prec_round_fail_2() {
    BigExponentFloat::ONE.csc_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn csc_prec_round_fail_3() {
    BigExponentFloat::csc_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn csc_prec_round_fail_4() {
    BigExponentFloat::csc_rational_prec_round(Rational::ONE, 10, Exact);
}

#[test]
#[should_panic]
fn csc_prec_round_fail_5() {
    BigExponentFloat::ONE.csc_with_period_prec_round(7, 0, Floor);
}

#[test]
#[should_panic]
fn csc_prec_round_fail_6() {
    BigExponentFloat::ONE.csc_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn csc_prec_round_fail_7() {
    BigExponentFloat::csc_with_period_rational_prec_round(Rational::ONE, 7, 0, Floor);
}

#[test]
#[should_panic]
fn csc_prec_round_fail_8() {
    BigExponentFloat::csc_with_period_rational_prec_round(Rational::ONE, 7, 10, Exact);
}

#[test]
#[should_panic]
fn csc_prec_round_fail_9() {
    BigExponentFloat::from(0.375).csc_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn csc_prec_round_fail_10() {
    BigExponentFloat::from(0.375).csc_pi_prec_round(10, Exact);
}

#[test]
fn csc_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_21().test_properties(
        |(x, prec, rm)| {
            let r = csc_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with the implementation that does not use them.
            assert!(same_pair(
                &trig_prec_round_generic(TrigFunction::Csc, &x, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if x.is_finite() && !x.is_zero() {
                // csc is odd
                let (s, o) = csc_prec_round_all(&-&x, prec, -rm);
                assert!(same(&s, &-&r.0));
                assert_eq!(o, r.1.reverse());
            }
            if big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&csc_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn csc_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        let (y, o) = x.csc_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) && !x.is_zero() {
            let r = csc_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn csc_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        let r = csc_rational_prec_round_all(&x, prec, rm);
        assert!(same_pair(
            &trig_rational_prec_round_generic(TrigFunction::Csc, &x, prec, rm),
            &r
        ));
        check_result(&r, prec);
        let (y, o) = Float::csc_rational_prec_round_ref(&x, prec, rm);
        if float_result_comparable(&y, o) && x != 0u32 {
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn csc_with_period_prec_round_properties() {
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_8().test_properties(
        |(x, u, prec, rm)| {
            let r = csc_with_period_prec_round_all(&x, u, prec, rm);
            assert!(same_pair(
                &trig_with_period_prec_round_generic(TrigFunction::Csc, &x, u, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(
                    &csc_with_period_prec_round_all(&x, u, prec, Exact),
                    &r
                ));
            }
            if x.is_finite() && !x.is_zero() && u != 0 {
                let (s, o) = csc_with_period_prec_round_all(&-&x, u, prec, -rm);
                assert!(same(&s, &-&r.0));
                assert_eq!(o, r.1.reverse());
                if big_exponent_float_exponent_is_small(&x) {
                    // The function has period u.
                    let y = &x + BigExponentFloat::from(u);
                    assert!(same_pair(
                        &csc_with_period_prec_round_all(&y, u, prec, rm),
                        &r
                    ));
                    let q = Rational::try_from(&x).unwrap();
                    assert!(same_pair(
                        &csc_with_period_rational_prec_round_all(&q, u, prec, rm),
                        &r
                    ));
                }
            }
        },
    );
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_22().test_properties(
        |(x, prec, rm)| {
            csc_with_period_prec_round_all(&x, 2, prec, rm);
        },
    );
}

#[test]
fn csc_with_period_prec_round_float_agreement_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17().test_properties(
        |(x, u, prec, rm)| {
            if rm == Exact || x.is_zero() {
                return;
            }
            let (y, o) = x.csc_with_period_prec_round_ref(u, prec, rm);
            if float_result_comparable(&y, o) {
                let r =
                    csc_with_period_prec_round_all(&float_to_big_exponent_float(&x), u, prec, rm);
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
fn csc_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_13().test_properties(
        |(x, u, prec, rm)| {
            let r = csc_with_period_rational_prec_round_all(&x, u, prec, rm);
            assert!(same_pair(
                &trig_with_period_rational_prec_round_generic(TrigFunction::Csc, &x, u, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(
                    &csc_with_period_rational_prec_round_all(&x, u, prec, Exact),
                    &r
                ));
            }
            if x != 0u32 {
                let (y, o) = Float::csc_with_period_rational_prec_round_ref(&x, u, prec, rm);
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
                csc_with_period_rational_prec_round_all(&x, 2, prec, rm);
            }
        },
    );
}
