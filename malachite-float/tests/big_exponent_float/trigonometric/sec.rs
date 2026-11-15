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

fn sec_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().sec_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.sec_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.sec_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().sec_prec(prec), &r));
        assert!(same_pair(&x.sec_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.sec_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn sec_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::sec_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::sec_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::sec_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::sec_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

fn sec_with_period_prec_round_all(
    x: &BigExponentFloat,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().sec_with_period_prec_round(u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.sec_with_period_prec_round_ref(u, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.sec_with_period_prec_round_assign(u, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().sec_with_period_prec(u, prec), &r));
        assert!(same_pair(&x.sec_with_period_prec_ref(u, prec), &r));
        let mut z = x.clone();
        let o = z.sec_with_period_prec_assign(u, prec);
        assert!(same_pair(&(z, o), &r));
    }
    if u == 2 {
        assert!(same_pair(&sec_pi_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn sec_with_period_rational_prec_round_all(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::sec_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::sec_with_period_rational_prec_round_ref(x, u, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::sec_with_period_rational_prec(x.clone(), u, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::sec_with_period_rational_prec_ref(x, u, prec),
            &r
        ));
    }
    if u == 2 {
        assert!(same_pair(&sec_pi_rational_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn sec_pi_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().sec_pi_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.sec_pi_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.sec_pi_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().sec_pi_prec(prec), &r));
        assert!(same_pair(&x.sec_pi_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.sec_pi_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn sec_pi_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::sec_pi_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::sec_pi_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::sec_pi_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::sec_pi_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_sec_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = sec_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_prec_round_generic(TrigFunction::Sec, &x, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 10, Exact, "1.0", "0x1.0", Equal);
    test("1.0", "0x1", 1, Nearest, "2.0", "0x2.0", Greater);
    test("1.0", "0x1", 10, Floor, "1.8496", "0x1.d98", Less);
    test("1.0", "0x1", 10, Ceiling, "1.852", "0x1.da", Greater);
    test(
        "1.0",
        "0x1",
        100,
        Nearest,
        "1.850815717680925617911753241398",
        "0x1.d9cf0f125cc29bcec42581638",
        Less,
    );
    test("-1.0", "-0x1", 1, Nearest, "2.0", "0x2.0", Greater);
    test("-1.0", "-0x1", 10, Floor, "1.8496", "0x1.d98", Less);
    test("-1.0", "-0x1", 10, Ceiling, "1.852", "0x1.da", Greater);
    test(
        "-1.0",
        "-0x1",
        100,
        Nearest,
        "1.850815717680925617911753241398",
        "0x1.d9cf0f125cc29bcec42581638",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        Nearest,
        "-1.0",
        "-0x1.0",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Floor,
        "-1.0020",
        "-0x1.008",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Ceiling,
        "-1.0",
        "-0x1.0",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        100,
        Nearest,
        "-1.0",
        "-0x1.0",
        Greater,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18",
        1,
        Nearest,
        "1.8e16",
        "0x4.0E+13",
        Greater,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18",
        10,
        Floor,
        "1.63e16",
        "0x3.aE+13",
        Less,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18",
        10,
        Ceiling,
        "1.6343e16",
        "0x3.a1E+13",
        Greater,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18",
        100,
        Nearest,
        "16331239353195369.755967737041530",
        "0x3a052cf8639b69.c1871a036cac",
        Greater,
    );
    test("0.00024", "0x0.001", 1, Nearest, "1.0", "0x1.0", Less);
    test("0.00024", "0x0.001", 10, Floor, "1.0", "0x1.0", Less);
    test(
        "0.00024", "0x0.001", 10, Ceiling, "1.0020", "0x1.008", Greater,
    );
    test(
        "0.00024",
        "0x0.001",
        100,
        Nearest,
        "1.0000000298023231278440135240663",
        "0x1.0000008000003555556b05b06",
        Less,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        1,
        Nearest,
        "-1.0",
        "-0x1.0",
        Greater,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        10,
        Floor,
        "-1.0020",
        "-0x1.008",
        Less,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        10,
        Ceiling,
        "-1.0",
        "-0x1.0",
        Greater,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        100,
        Nearest,
        "-1.0",
        "-0x1.0",
        Greater,
    );
    test("7.6e22", "0x1.0E+19", 1, Nearest, "-1.0", "-0x1.0", Greater);
    test("7.6e22", "0x1.0E+19", 10, Floor, "-1.332", "-0x1.55", Less);
    test(
        "7.6e22",
        "0x1.0E+19",
        10,
        Ceiling,
        "-1.3301",
        "-0x1.548",
        Greater,
    );
    test(
        "7.6e22",
        "0x1.0E+19",
        100,
        Nearest,
        "-1.331184517839940511527402808301",
        "-0x1.54c882311067178f84a9d7004",
        Less,
    );
    test("1.3e30", "0x1.0E+25", 1, Nearest, "2.0", "0x2.0", Less);
    test("1.3e30", "0x1.0E+25", 10, Floor, "2.0430", "0x2.0b", Less);
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Ceiling,
        "2.047",
        "0x2.0c",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        100,
        Nearest,
        "2.0442429074571736614847672609",
        "0x2.0b5380d09bc7aa3fcf01626",
        Greater,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        10,
        Floor,
        "1.398",
        "0x1.66",
        Less,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        10,
        Ceiling,
        "1.4004",
        "0x1.668",
        Greater,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        100,
        Nearest,
        "1.398833729754312100445104403614",
        "0x1.6619f7a1d62311428a56fb9c8",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        10,
        Floor,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        10,
        Floor,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
}

#[test]
fn test_sec_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = sec_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_rational_prec_round_generic(TrigFunction::Sec, &x, prec, rm),
            &r
        ));
    };
    test("0", 10, Exact, "1.0", "0x1.0", Equal);
    test("1/3", 1, Nearest, "1.0", "0x1.0", Less);
    test("1/3", 10, Floor, "1.0566", "0x1.0e8", Less);
    test("1/3", 10, Ceiling, "1.059", "0x1.0f", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "1.058249271461441901459521777641",
        "0x1.0ee96c9bf15609d47a83b4e7",
        Greater,
    );
    test("-22/7", 1, Nearest, "-1.0", "-0x1.0", Greater);
    test("-22/7", 10, Floor, "-1.0020", "-0x1.008", Less);
    test("-22/7", 10, Ceiling, "-1.0", "-0x1.0", Greater);
    test(
        "-22/7",
        100,
        Nearest,
        "-1.00000079946708624384267440160",
        "-0x1.00000d69af5b7b3e72a647ea",
        Less,
    );
    test("355/113", 1, Nearest, "-1.0", "-0x1.0", Greater);
    test("355/113", 10, Floor, "-1.0020", "-0x1.008", Less);
    test("355/113", 10, Ceiling, "-1.0", "-0x1.0", Greater);
    test(
        "355/113",
        100,
        Nearest,
        "-1.000000000000035581566283066953",
        "-0x1.00000000000a03ec0c05edd1",
        Greater,
    );
    test(
        "1000000000000000000000000000001/7",
        1,
        Nearest,
        "16.0",
        "0x1.0E+1",
        Greater,
    );
    test(
        "1000000000000000000000000000001/7",
        10,
        Floor,
        "14.56",
        "0xe.9",
        Less,
    );
    test(
        "1000000000000000000000000000001/7",
        10,
        Ceiling,
        "14.578",
        "0xe.94",
        Greater,
    );
    test(
        "1000000000000000000000000000001/7",
        100,
        Nearest,
        "14.577878244910562853426307937390",
        "0xe.93efd422f5f3717e1030fa31",
        Greater,
    );
}

#[test]
fn test_sec_with_period_prec_round() {
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
        let r = sec_with_period_prec_round_all(&x, u, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_with_period_prec_round_generic(TrigFunction::Sec, &x, u, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 7, 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 7, 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 7, 10, Floor, "NaN", "NaN", Equal);
    test("1.0", "0x1", 0, 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 7, 10, Exact, "1.0", "0x1.0", Equal);
    test("0.0", "0x0", 24, 10, Exact, "1.0", "0x1.0", Equal);
    test("1.0", "0x1", 24, 10, Nearest, "1.035", "0x1.09", Less);
    test("2.0", "0x2", 24, 10, Nearest, "1.1543", "0x1.278", Less);
    test("3.0", "0x3", 24, 10, Nearest, "1.414", "0x1.6a", Less);
    test("4.0", "0x4", 24, 10, Exact, "2.0", "0x2.0", Equal);
    test("5.0", "0x5", 24, 10, Nearest, "3.8633", "0x3.dd", Less);
    test("6.0", "0x6", 24, 10, Exact, "NaN", "NaN", Equal);
    test("7.0", "0x7", 24, 10, Nearest, "-3.8633", "-0x3.dd", Greater);
    test("8.0", "0x8", 24, 10, Exact, "-2.0", "-0x2.0", Equal);
    test("9.00", "0x9", 24, 10, Nearest, "-1.414", "-0x1.6a", Greater);
    test(
        "10.0", "0xa", 24, 10, Nearest, "-1.1543", "-0x1.278", Greater,
    );
    test("11.0", "0xb", 24, 10, Nearest, "-1.035", "-0x1.09", Greater);
    test("12.0", "0xc", 24, 10, Exact, "-1.0", "-0x1.0", Equal);
    test("13.0", "0xd", 24, 10, Nearest, "-1.035", "-0x1.09", Greater);
    test(
        "14.0", "0xe", 24, 10, Nearest, "-1.1543", "-0x1.278", Greater,
    );
    test("15.0", "0xf", 24, 10, Nearest, "-1.414", "-0x1.6a", Greater);
    test("16.0", "0x10", 24, 10, Exact, "-2.0", "-0x2.0", Equal);
    test(
        "17.0", "0x11", 24, 10, Nearest, "-3.8633", "-0x3.dd", Greater,
    );
    test("18.0", "0x12", 24, 10, Exact, "NaN", "NaN", Equal);
    test("19.0", "0x13", 24, 10, Nearest, "3.8633", "0x3.dd", Less);
    test("20.0", "0x14", 24, 10, Exact, "2.0", "0x2.0", Equal);
    test("21.0", "0x15", 24, 10, Nearest, "1.414", "0x1.6a", Less);
    test("22.0", "0x16", 24, 10, Nearest, "1.1543", "0x1.278", Less);
    test("23.0", "0x17", 24, 10, Nearest, "1.035", "0x1.09", Less);
    test("24.0", "0x18", 24, 10, Exact, "1.0", "0x1.0", Equal);
    test("1.0", "0x1", 7, 10, Nearest, "1.6035", "0x1.9a8", Less);
    test("1.0", "0x1", 7, 10, Floor, "1.6035", "0x1.9a8", Less);
    test(
        "1.0",
        "0x1",
        7,
        100,
        Nearest,
        "1.60387547160967650494440927803",
        "0x1.9a9795396b8e1a4dc627d254",
        Greater,
    );
    test(
        "-5.0", "-0x5", 7, 10, Nearest, "-4.4922", "-0x4.7e", Greater,
    );
    test("-5.0", "-0x5", 7, 10, Floor, "-4.50", "-0x4.8", Less);
    test(
        "-5.0",
        "-0x5",
        7,
        100,
        Nearest,
        "-4.493959207434934122100019536018",
        "-0x4.7e741c517dba2d97de167915",
        Less,
    );
    test("0.50", "0x0.8", 3, 10, Exact, "2.0", "0x2.0", Equal);
    test("0.50", "0x0.8", 3, 10, Floor, "2.0", "0x2.0", Equal);
    test("0.50", "0x0.8", 3, 100, Exact, "2.0", "0x2.0", Equal);
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        10,
        Nearest,
        "-4.4922",
        "-0x4.7e",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        10,
        Floor,
        "-4.50",
        "-0x4.8",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        100,
        Nearest,
        "-4.493959207434934122100019536018",
        "-0x4.7e741c517dba2d97de167915",
        Less,
    );
    test(
        "5.4e12041199826",
        "0x1.8E+10000000000",
        1000,
        10,
        Nearest,
        "1.086",
        "0x1.16",
        Less,
    );
    test(
        "5.4e12041199826",
        "0x1.8E+10000000000",
        1000,
        10,
        Floor,
        "1.086",
        "0x1.16",
        Less,
    );
    test(
        "5.4e12041199826",
        "0x1.8E+10000000000",
        1000,
        100,
        Nearest,
        "1.0866826690061456261447081099242",
        "0x1.1630d5dc82ea57c18842f8ce2",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        7,
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        7,
        10,
        Floor,
        "1.0",
        "0x1.0",
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
        Less,
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
        "1.5879",
        "0x1.968",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        10,
        Floor,
        "1.5879",
        "0x1.968",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        100,
        Nearest,
        "1.5881038941207315114227608491648",
        "0x1.968dfa0fe6170117361e89162",
        Greater,
    );
}

#[test]
fn test_sec_with_period_rational_prec_round() {
    let test =
        |x: &str, u: u64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            let x = Rational::from_str(x).unwrap();
            let r = sec_with_period_rational_prec_round_all(&x, u, prec, rm);
            assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
            assert_eq!(r.0.to_string(), out);
            assert_eq!(r.1, o, "{x} {prec} {rm}");
            assert!(same_pair(
                &trig_with_period_rational_prec_round_generic(TrigFunction::Sec, &x, u, prec, rm),
                &r
            ));
        };
    test("0", 7, 10, Exact, "1.0", "0x1.0", Equal);
    test("1", 0, 10, Floor, "NaN", "NaN", Equal);
    test("1/3", 1, 10, Exact, "-2.0", "-0x2.0", Equal);
    test("1/3", 1, 10, Floor, "-2.0", "-0x2.0", Equal);
    test("1/3", 1, 100, Exact, "-2.0", "-0x2.0", Equal);
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
    test("1/7", 7, 10, Nearest, "1.008", "0x1.02", Less);
    test("1/7", 7, 10, Floor, "1.008", "0x1.02", Less);
    test(
        "1/7",
        7,
        100,
        Nearest,
        "1.0082779480155333067954753068072",
        "0x1.021e80ec0134423274821d512",
        Less,
    );
    test(
        "100000000000000000001/3",
        7,
        10,
        Nearest,
        "2.7383",
        "0x2.bd",
        Greater,
    );
    test(
        "100000000000000000001/3",
        7,
        10,
        Floor,
        "2.734",
        "0x2.bc",
        Less,
    );
    test(
        "100000000000000000001/3",
        7,
        100,
        Nearest,
        "2.737168654230068209153465404004",
        "0x2.bcb715bd8df2132575fdb37e8",
        Less,
    );
    test("-22/7", 2, 10, Nearest, "-1.109", "-0x1.1c", Greater);
    test("-22/7", 2, 10, Floor, "-1.1113", "-0x1.1c8", Less);
    test(
        "-22/7",
        2,
        100,
        Nearest,
        "-1.109916264174742382844389742012",
        "-0x1.1c2378e7edd3ecb5e811593f",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        5,
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        5,
        10,
        Floor,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        5,
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
}

#[test]
fn test_sec_pi_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = sec_pi_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_with_period_prec_round_generic(TrigFunction::Sec, &x, 2, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 10, Exact, "1.0", "0x1.0", Equal);
    test("0.50", "0x0.8", 10, Exact, "NaN", "NaN", Equal);
    test("0.50", "0x0.8", 10, Ceiling, "NaN", "NaN", Equal);
    test("0.50", "0x0.8", 100, Exact, "NaN", "NaN", Equal);
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
        "1.1543",
        "0x1.278",
        Less,
    );
    test(
        "0.1666666666666665",
        "0x0.2aaaaaaaaaaaa",
        10,
        Ceiling,
        "1.16",
        "0x1.28",
        Greater,
    );
    test(
        "0.1666666666666665",
        "0x0.2aaaaaaaaaaaa",
        100,
        Nearest,
        "1.154700538379251218985542182459",
        "0x1.279a74590331ae7b00ad37068",
        Greater,
    );
    test("0.38", "0x0.6", 10, Nearest, "2.6133", "0x2.9d", Greater);
    test("0.38", "0x0.6", 10, Ceiling, "2.6133", "0x2.9d", Greater);
    test(
        "0.38",
        "0x0.6",
        100,
        Nearest,
        "2.613125929752753055713286346855",
        "0x2.9cf5d2289e1f79182de4c709",
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
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test("-1.5", "-0x1.8", 10, Exact, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8", 100, Exact, "NaN", "NaN", Equal);
}

#[test]
fn test_sec_pi_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = sec_pi_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_with_period_rational_prec_round_generic(TrigFunction::Sec, &x, 2, prec, rm),
            &r
        ));
    };
    test("0", 10, Exact, "1.0", "0x1.0", Equal);
    test("1/6", 10, Nearest, "1.1543", "0x1.278", Less);
    test("1/6", 10, Ceiling, "1.16", "0x1.28", Greater);
    test(
        "1/6",
        100,
        Nearest,
        "1.154700538379251529018297561004",
        "0x1.279a74590331c4d218f81e4b",
        Greater,
    );
    test("1/3", 10, Exact, "2.0", "0x2.0", Equal);
    test("1/3", 10, Ceiling, "2.0", "0x2.0", Equal);
    test("1/3", 100, Exact, "2.0", "0x2.0", Equal);
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
    test("1/5", 10, Nearest, "1.2363", "0x1.3c8", Greater);
    test("1/5", 10, Ceiling, "1.2363", "0x1.3c8", Greater);
    test(
        "1/5",
        100,
        Nearest,
        "1.236067977499789696409173668732",
        "0x1.3c6ef372fe94f82be73980c0c",
        Greater,
    );
    test("7/2", 10, Exact, "NaN", "NaN", Equal);
    test("7/2", 10, Ceiling, "NaN", "NaN", Equal);
    test("7/2", 100, Exact, "NaN", "NaN", Equal);
    test("-5/6", 10, Nearest, "-1.1543", "-0x1.278", Greater);
    test("-5/6", 10, Ceiling, "-1.1543", "-0x1.278", Greater);
    test(
        "-5/6",
        100,
        Nearest,
        "-1.154700538379251529018297561004",
        "-0x1.279a74590331c4d218f81e4b",
        Less,
    );
}

#[test]
#[should_panic]
fn sec_prec_round_fail_1() {
    BigExponentFloat::ONE.sec_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn sec_prec_round_fail_2() {
    BigExponentFloat::ONE.sec_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn sec_prec_round_fail_3() {
    BigExponentFloat::sec_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn sec_prec_round_fail_4() {
    BigExponentFloat::sec_rational_prec_round(Rational::ONE, 10, Exact);
}

#[test]
#[should_panic]
fn sec_prec_round_fail_5() {
    BigExponentFloat::ONE.sec_with_period_prec_round(7, 0, Floor);
}

#[test]
#[should_panic]
fn sec_prec_round_fail_6() {
    BigExponentFloat::ONE.sec_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn sec_prec_round_fail_7() {
    BigExponentFloat::sec_with_period_rational_prec_round(Rational::ONE, 7, 0, Floor);
}

#[test]
#[should_panic]
fn sec_prec_round_fail_8() {
    BigExponentFloat::sec_with_period_rational_prec_round(Rational::ONE, 7, 10, Exact);
}

#[test]
#[should_panic]
fn sec_prec_round_fail_9() {
    BigExponentFloat::from(0.375).sec_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn sec_prec_round_fail_10() {
    BigExponentFloat::from(0.375).sec_pi_prec_round(10, Exact);
}

#[test]
fn sec_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_21().test_properties(
        |(x, prec, rm)| {
            let r = sec_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with the implementation that does not use them.
            assert!(same_pair(
                &trig_prec_round_generic(TrigFunction::Sec, &x, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if x.is_finite() && !x.is_zero() {
                // sec is even
                assert!(same_pair(&sec_prec_round_all(&-&x, prec, rm), &r));
            }
            if big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&sec_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn sec_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        let (y, o) = x.sec_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) && !x.is_zero() {
            let r = sec_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn sec_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        let r = sec_rational_prec_round_all(&x, prec, rm);
        assert!(same_pair(
            &trig_rational_prec_round_generic(TrigFunction::Sec, &x, prec, rm),
            &r
        ));
        check_result(&r, prec);
        let (y, o) = Float::sec_rational_prec_round_ref(&x, prec, rm);
        if float_result_comparable(&y, o) && x != 0u32 {
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn sec_with_period_prec_round_properties() {
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_8().test_properties(
        |(x, u, prec, rm)| {
            let r = sec_with_period_prec_round_all(&x, u, prec, rm);
            assert!(same_pair(
                &trig_with_period_prec_round_generic(TrigFunction::Sec, &x, u, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(
                    &sec_with_period_prec_round_all(&x, u, prec, Exact),
                    &r
                ));
            }
            if x.is_finite() && !x.is_zero() && u != 0 {
                assert!(same_pair(
                    &sec_with_period_prec_round_all(&-&x, u, prec, rm),
                    &r
                ));
                if big_exponent_float_exponent_is_small(&x) {
                    // The function has period u.
                    let y = &x + BigExponentFloat::from(u);
                    assert!(same_pair(
                        &sec_with_period_prec_round_all(&y, u, prec, rm),
                        &r
                    ));
                    let q = Rational::try_from(&x).unwrap();
                    assert!(same_pair(
                        &sec_with_period_rational_prec_round_all(&q, u, prec, rm),
                        &r
                    ));
                }
            }
        },
    );
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_22().test_properties(
        |(x, prec, rm)| {
            sec_with_period_prec_round_all(&x, 2, prec, rm);
        },
    );
}

#[test]
fn sec_with_period_prec_round_float_agreement_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17().test_properties(
        |(x, u, prec, rm)| {
            if rm == Exact || x.is_zero() {
                return;
            }
            let (y, o) = x.sec_with_period_prec_round_ref(u, prec, rm);
            if float_result_comparable(&y, o) {
                let r =
                    sec_with_period_prec_round_all(&float_to_big_exponent_float(&x), u, prec, rm);
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
fn sec_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_13().test_properties(
        |(x, u, prec, rm)| {
            let r = sec_with_period_rational_prec_round_all(&x, u, prec, rm);
            assert!(same_pair(
                &trig_with_period_rational_prec_round_generic(TrigFunction::Sec, &x, u, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(
                    &sec_with_period_rational_prec_round_all(&x, u, prec, Exact),
                    &r
                ));
            }
            if x != 0u32 {
                let (y, o) = Float::sec_with_period_rational_prec_round_ref(&x, u, prec, rm);
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
                sec_with_period_rational_prec_round_all(&x, 2, prec, rm);
            }
        },
    );
}
