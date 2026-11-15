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

fn tan_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().tan_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.tan_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.tan_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().tan_prec(prec), &r));
        assert!(same_pair(&x.tan_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.tan_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn tan_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::tan_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::tan_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::tan_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::tan_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

fn tan_with_period_prec_round_all(
    x: &BigExponentFloat,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().tan_with_period_prec_round(u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.tan_with_period_prec_round_ref(u, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.tan_with_period_prec_round_assign(u, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().tan_with_period_prec(u, prec), &r));
        assert!(same_pair(&x.tan_with_period_prec_ref(u, prec), &r));
        let mut z = x.clone();
        let o = z.tan_with_period_prec_assign(u, prec);
        assert!(same_pair(&(z, o), &r));
    }
    if u == 2 {
        assert!(same_pair(&tan_pi_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn tan_with_period_rational_prec_round_all(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::tan_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::tan_with_period_rational_prec_round_ref(x, u, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::tan_with_period_rational_prec(x.clone(), u, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::tan_with_period_rational_prec_ref(x, u, prec),
            &r
        ));
    }
    if u == 2 {
        assert!(same_pair(&tan_pi_rational_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn tan_pi_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().tan_pi_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.tan_pi_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.tan_pi_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().tan_pi_prec(prec), &r));
        assert!(same_pair(&x.tan_pi_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.tan_pi_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn tan_pi_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::tan_pi_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::tan_pi_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::tan_pi_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::tan_pi_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_tan_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = tan_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_prec_round_generic(TrigFunction::Tan, &x, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", 1, Nearest, "2.0", "0x2.0", Greater);
    test("1.0", "0x1", 10, Floor, "1.5566", "0x1.8e8", Less);
    test("1.0", "0x1", 10, Ceiling, "1.559", "0x1.8f", Greater);
    test(
        "1.0",
        "0x1",
        100,
        Nearest,
        "1.557407724654902230506974807459",
        "0x1.8eb245cbee3a5b8acc7d41324",
        Greater,
    );
    test("-1.0", "-0x1", 1, Nearest, "-2.0", "-0x2.0", Less);
    test("-1.0", "-0x1", 10, Floor, "-1.559", "-0x1.8f", Less);
    test("-1.0", "-0x1", 10, Ceiling, "-1.5566", "-0x1.8e8", Greater);
    test(
        "-1.0",
        "-0x1",
        100,
        Nearest,
        "-1.557407724654902230506974807459",
        "-0x1.8eb245cbee3a5b8acc7d41324",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        Nearest,
        "-1.1e-16",
        "-0x8.0E-14",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Floor,
        "-1.2251e-16",
        "-0x8.d4E-14",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Ceiling,
        "-1.223e-16",
        "-0x8.dE-14",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        100,
        Nearest,
        "-1.224646799147353177226065932275e-16",
        "-0x8.d313198a2e03707344a40938E-14",
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
    test("0.00024", "0x0.001", 1, Nearest, "0.00024", "0x0.001", Less);
    test("0.00024", "0x0.001", 10, Floor, "0.00024", "0x0.001", Less);
    test(
        "0.00024",
        "0x0.001",
        10,
        Ceiling,
        "0.00024462",
        "0x0.001008",
        Greater,
    );
    test(
        "0.00024",
        "0x0.001",
        100,
        Nearest,
        "0.0002441406298506385251038517907606",
        "0x0.0010000005555557777778548548",
        Less,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        1,
        Nearest,
        "-9.6e-35",
        "-0x8.0E-29",
        Less,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        10,
        Floor,
        "-8.6799e-35",
        "-0x7.36E-29",
        Less,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        10,
        Ceiling,
        "-8.670e-35",
        "-0x7.34E-29",
        Greater,
    );
    test(
        "3.1415926535897932384626433832795028",
        "0x3.243f6a8885a308d313198a2e037",
        100,
        Nearest,
        "-8.6718101301237810247970440260409e-35",
        "-0x7.344a4093822299f31d0082ef8E-29",
        Greater,
    );
    test("7.6e22", "0x1.0E+19", 1, Nearest, "1.0", "0x1.0", Greater);
    test("7.6e22", "0x1.0E+19", 10, Floor, "0.87793", "0x0.e0c", Less);
    test(
        "7.6e22",
        "0x1.0E+19",
        10,
        Ceiling,
        "0.8789",
        "0x0.e1",
        Greater,
    );
    test(
        "7.6e22",
        "0x1.0E+19",
        100,
        Nearest,
        "0.8786650218011155689082513206257",
        "0x0.e0f030dcc6610d1fb01e77ee2",
        Less,
    );
    test("1.3e30", "0x1.0E+25", 1, Nearest, "-2.0", "-0x2.0", Less);
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Floor,
        "-1.7832",
        "-0x1.c88",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        10,
        Ceiling,
        "-1.78",
        "-0x1.c8",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25",
        100,
        Nearest,
        "-1.78295514937671908862526289167",
        "-0x1.c86fbfa8cecc31efc7a12116",
        Less,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        1,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        10,
        Floor,
        "-0.9785",
        "-0x0.fa8",
        Less,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        10,
        Ceiling,
        "-0.97754",
        "-0x0.fa4",
        Greater,
    );
    test(
        "-3.0e4816",
        "-0x1.0E+4000",
        100,
        Nearest,
        "-0.9781287254233769093023183850113",
        "-0x0.fa66a4e6f8b8a7231907f3fba",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        1,
        Nearest,
        "1.1e-12041199826",
        "0x4.0E-10000000000",
        Greater,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        10,
        Floor,
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        Less,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        10,
        Ceiling,
        "8.2878e-12041199827",
        "0x3.01E-10000000000",
        Greater,
    );
    test(
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        100,
        Nearest,
        "8.3e-12041199827",
        "0x3.0E-10000000000",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        1,
        Nearest,
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        10,
        Floor,
        "-2.3872e-323228497",
        "-0x1.008E-268435456",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        10,
        Ceiling,
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        100,
        Nearest,
        "-2.4e-323228497",
        "-0x1.0E-268435456",
        Greater,
    );
}

#[test]
fn test_tan_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = tan_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_rational_prec_round_generic(TrigFunction::Tan, &x, prec, rm),
            &r
        ));
    };
    test("0", 10, Exact, "0.0", "0x0.0", Equal);
    test("1/3", 1, Nearest, "0.25", "0x0.4", Less);
    test("1/3", 10, Floor, "0.34619", "0x0.58a", Less);
    test("1/3", 10, Ceiling, "0.3467", "0x0.58c", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "0.3462535495105754910385435656097",
        "0x0.58a41297459735e7bc3b8169f",
        Less,
    );
    test("-22/7", 1, Nearest, "-0.00098", "-0x0.004", Greater);
    test("-22/7", 10, Floor, "-0.0012646", "-0x0.0052e", Less);
    test("-22/7", 10, Ceiling, "-0.001263", "-0x0.0052c", Greater);
    test(
        "-22/7",
        100,
        Nearest,
        "-0.0012644899412946341567363163883632",
        "-0x0.0052de9ef1a8c413757001f149c8",
        Greater,
    );
    test("355/113", 1, Nearest, "2.4e-7", "0x4.0E-6", Less);
    test("355/113", 10, Floor, "2.664e-7", "0x4.78E-6", Less);
    test("355/113", 10, Ceiling, "2.6682e-7", "0x4.7aE-6", Greater);
    test(
        "355/113",
        100,
        Nearest,
        "2.6676418906242864029404960194488e-7",
        "0x4.79be53e7514a37760d79f4f28E-6",
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
        "14.53",
        "0xe.88",
        Less,
    );
    test(
        "1000000000000000000000000000001/7",
        10,
        Ceiling,
        "14.547",
        "0xe.8c",
        Greater,
    );
    test(
        "1000000000000000000000000000001/7",
        100,
        Nearest,
        "14.543539257121585270505183895331",
        "0xe.8b2563856de97a374e3fef6b",
        Greater,
    );
}

#[test]
fn test_tan_with_period_prec_round() {
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
        let r = tan_with_period_prec_round_all(&x, u, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_with_period_prec_round_generic(TrigFunction::Tan, &x, u, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 7, 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 7, 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 7, 10, Floor, "NaN", "NaN", Equal);
    test("1.0", "0x1", 0, 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 7, 10, Exact, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", 24, 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", 24, 10, Nearest, "0.26807", "0x0.44a", Greater);
    test("2.0", "0x2", 24, 10, Nearest, "0.57715", "0x0.93c", Less);
    test("3.0", "0x3", 24, 10, Exact, "1.0", "0x1.0", Equal);
    test("4.0", "0x4", 24, 10, Nearest, "1.7324", "0x1.bb8", Greater);
    test("5.0", "0x5", 24, 10, Nearest, "3.7305", "0x3.bb", Less);
    test("6.0", "0x6", 24, 10, Exact, "NaN", "NaN", Equal);
    test("7.0", "0x7", 24, 10, Nearest, "-3.7305", "-0x3.bb", Greater);
    test("8.0", "0x8", 24, 10, Nearest, "-1.7324", "-0x1.bb8", Less);
    test("9.00", "0x9", 24, 10, Exact, "-1.0", "-0x1.0", Equal);
    test(
        "10.0", "0xa", 24, 10, Nearest, "-0.57715", "-0x0.93c", Greater,
    );
    test("11.0", "0xb", 24, 10, Nearest, "-0.26807", "-0x0.44a", Less);
    test("12.0", "0xc", 24, 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "13.0", "0xd", 24, 10, Nearest, "0.26807", "0x0.44a", Greater,
    );
    test("14.0", "0xe", 24, 10, Nearest, "0.57715", "0x0.93c", Less);
    test("15.0", "0xf", 24, 10, Exact, "1.0", "0x1.0", Equal);
    test(
        "16.0", "0x10", 24, 10, Nearest, "1.7324", "0x1.bb8", Greater,
    );
    test("17.0", "0x11", 24, 10, Nearest, "3.7305", "0x3.bb", Less);
    test("18.0", "0x12", 24, 10, Exact, "NaN", "NaN", Equal);
    test(
        "19.0", "0x13", 24, 10, Nearest, "-3.7305", "-0x3.bb", Greater,
    );
    test("20.0", "0x14", 24, 10, Nearest, "-1.7324", "-0x1.bb8", Less);
    test("21.0", "0x15", 24, 10, Exact, "-1.0", "-0x1.0", Equal);
    test(
        "22.0", "0x16", 24, 10, Nearest, "-0.57715", "-0x0.93c", Greater,
    );
    test(
        "23.0", "0x17", 24, 10, Nearest, "-0.26807", "-0x0.44a", Less,
    );
    test("24.0", "0x18", 24, 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", 7, 10, Nearest, "1.254", "0x1.41", Less);
    test("1.0", "0x1", 7, 10, Floor, "1.254", "0x1.41", Less);
    test(
        "1.0",
        "0x1",
        7,
        100,
        Nearest,
        "1.25396033766270383757091097834",
        "0x1.41038b70be0fe580daa1f884",
        Greater,
    );
    test("-5.0", "-0x5", 7, 10, Nearest, "-4.3828", "-0x4.62", Less);
    test("-5.0", "-0x5", 7, 10, Floor, "-4.3828", "-0x4.62", Less);
    test(
        "-5.0",
        "-0x5",
        7,
        100,
        Nearest,
        "-4.3812862675348230724046890850316",
        "-0x4.619bfa1179d946f745d595848",
        Greater,
    );
    test(
        "0.50", "0x0.8", 3, 10, Nearest, "1.7324", "0x1.bb8", Greater,
    );
    test("0.50", "0x0.8", 3, 10, Floor, "1.730", "0x1.bb", Less);
    test(
        "0.50",
        "0x0.8",
        3,
        100,
        Nearest,
        "1.732050807568877293527446341506",
        "0x1.bb67ae8584caa73b25742d708",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        10,
        Nearest,
        "-4.3828",
        "-0x4.62",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        10,
        Floor,
        "-4.3828",
        "-0x4.62",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        7,
        100,
        Nearest,
        "-4.3812862675348230724046890850316",
        "-0x4.619bfa1179d946f745d595848",
        Greater,
    );
    test(
        "5.4e12041199826",
        "0x1.8E+10000000000",
        1000,
        10,
        Nearest,
        "0.42529",
        "0x0.6ce",
        Less,
    );
    test(
        "5.4e12041199826",
        "0x1.8E+10000000000",
        1000,
        10,
        Floor,
        "0.42529",
        "0x0.6ce",
        Less,
    );
    test(
        "5.4e12041199826",
        "0x1.8E+10000000000",
        1000,
        100,
        Nearest,
        "0.4252989808573731485414851349006",
        "0x0.6ce064ddcdf6d6c55b84934cc",
        Greater,
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
        "1.234",
        "0x1.3c",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        10,
        Floor,
        "1.2324",
        "0x1.3b8",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        100,
        Nearest,
        "1.2337236232322989781150511585024",
        "0x1.3bd54fb615d97e13450671426",
        Greater,
    );
}

#[test]
fn test_tan_with_period_rational_prec_round() {
    let test =
        |x: &str, u: u64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            let x = Rational::from_str(x).unwrap();
            let r = tan_with_period_rational_prec_round_all(&x, u, prec, rm);
            assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
            assert_eq!(r.0.to_string(), out);
            assert_eq!(r.1, o, "{x} {prec} {rm}");
            assert!(same_pair(
                &trig_with_period_rational_prec_round_generic(TrigFunction::Tan, &x, u, prec, rm),
                &r
            ));
        };
    test("0", 7, 10, Exact, "0.0", "0x0.0", Equal);
    test("1", 0, 10, Floor, "NaN", "NaN", Equal);
    test("1/3", 1, 10, Nearest, "-1.7324", "-0x1.bb8", Less);
    test("1/3", 1, 10, Floor, "-1.7324", "-0x1.bb8", Less);
    test(
        "1/3",
        1,
        100,
        Nearest,
        "-1.732050807568877293527446341506",
        "-0x1.bb67ae8584caa73b25742d708",
        Less,
    );
    test("5/8", 1, 10, Exact, "1.0", "0x1.0", Equal);
    test("5/8", 1, 10, Floor, "1.0", "0x1.0", Equal);
    test("5/8", 1, 100, Exact, "1.0", "0x1.0", Equal);
    test("1/7", 7, 10, Nearest, "0.129", "0x0.21", Less);
    test("1/7", 7, 10, Floor, "0.129", "0x0.21", Less);
    test(
        "1/7",
        7,
        100,
        Nearest,
        "0.1289357221813042989364988814825",
        "0x0.2101ee760e0a3d61d9a39761a8",
        Greater,
    );
    test(
        "100000000000000000001/3",
        7,
        10,
        Nearest,
        "-2.547",
        "-0x2.8c",
        Greater,
    );
    test(
        "100000000000000000001/3",
        7,
        10,
        Floor,
        "-2.5508",
        "-0x2.8d",
        Less,
    );
    test(
        "100000000000000000001/3",
        7,
        100,
        Nearest,
        "-2.547958445834555343603732029852",
        "-0x2.8c4701346d28b8fbf493c0268",
        Less,
    );
    test("-22/7", 2, 10, Nearest, "-0.4814", "-0x0.7b4", Greater);
    test("-22/7", 2, 10, Floor, "-0.48193", "-0x0.7b6", Less);
    test(
        "-22/7",
        2,
        100,
        Nearest,
        "-0.4815746188075286443321623530571",
        "-0x0.7b4879665cac2a849c8bb6e0e",
        Less,
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
fn test_tan_pi_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = tan_pi_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_with_period_prec_round_generic(TrigFunction::Tan, &x, 2, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 10, Exact, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8", 10, Exact, "NaN", "NaN", Equal);
    test("0.50", "0x0.8", 10, Ceiling, "NaN", "NaN", Equal);
    test("0.50", "0x0.8", 100, Exact, "NaN", "NaN", Equal);
    test("0.25", "0x0.4", 10, Exact, "1.0", "0x1.0", Equal);
    test("0.25", "0x0.4", 10, Ceiling, "1.0", "0x1.0", Equal);
    test("0.25", "0x0.4", 100, Exact, "1.0", "0x1.0", Equal);
    test(
        "0.1666666666666665",
        "0x0.2aaaaaaaaaaaa",
        10,
        Nearest,
        "0.57715",
        "0x0.93c",
        Less,
    );
    test(
        "0.1666666666666665",
        "0x0.2aaaaaaaaaaaa",
        10,
        Ceiling,
        "0.578",
        "0x0.94",
        Greater,
    );
    test(
        "0.1666666666666665",
        "0x0.2aaaaaaaaaaaa",
        100,
        Nearest,
        "0.57735026918962514444363802341190",
        "0x0.93cd3a2c8198b5badbe6409c7",
        Less,
    );
    test("0.38", "0x0.6", 10, Nearest, "2.414", "0x2.6a", Less);
    test("0.38", "0x0.6", 10, Ceiling, "2.4180", "0x2.6b", Greater);
    test(
        "0.38",
        "0x0.6",
        100,
        Nearest,
        "2.414213562373095048801688724211",
        "0x2.6a09e667f3bcc908b2fb1367",
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
    test("-1.5", "-0x1.8", 10, Exact, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8", 100, Exact, "NaN", "NaN", Equal);
}

#[test]
fn test_tan_pi_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = tan_pi_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &trig_with_period_rational_prec_round_generic(TrigFunction::Tan, &x, 2, prec, rm),
            &r
        ));
    };
    test("0", 10, Exact, "0.0", "0x0.0", Equal);
    test("1/6", 10, Nearest, "0.57715", "0x0.93c", Less);
    test("1/6", 10, Ceiling, "0.578", "0x0.94", Greater);
    test(
        "1/6",
        100,
        Nearest,
        "0.5773502691896257645091487805021",
        "0x0.93cd3a2c8198e2690c7c0f258",
        Greater,
    );
    test("1/3", 10, Nearest, "1.7324", "0x1.bb8", Greater);
    test("1/3", 10, Ceiling, "1.7324", "0x1.bb8", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "1.732050807568877293527446341506",
        "0x1.bb67ae8584caa73b25742d708",
        Greater,
    );
    test("1/4", 10, Exact, "1.0", "0x1.0", Equal);
    test("1/4", 10, Ceiling, "1.0", "0x1.0", Equal);
    test("1/4", 100, Exact, "1.0", "0x1.0", Equal);
    test("1/5", 10, Nearest, "0.7266", "0x0.ba", Greater);
    test("1/5", 10, Ceiling, "0.7266", "0x0.ba", Greater);
    test(
        "1/5",
        100,
        Nearest,
        "0.72654252800536088589546675748034",
        "0x0.b9feb0ecefaa170af5086c5d7",
        Less,
    );
    test("7/2", 10, Exact, "NaN", "NaN", Equal);
    test("7/2", 10, Ceiling, "NaN", "NaN", Equal);
    test("7/2", 100, Exact, "NaN", "NaN", Equal);
    test("-5/6", 10, Nearest, "0.57715", "0x0.93c", Less);
    test("-5/6", 10, Ceiling, "0.578", "0x0.94", Greater);
    test(
        "-5/6",
        100,
        Nearest,
        "0.5773502691896257645091487805021",
        "0x0.93cd3a2c8198e2690c7c0f258",
        Greater,
    );
}

#[test]
#[should_panic]
fn tan_prec_round_fail_1() {
    BigExponentFloat::ONE.tan_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn tan_prec_round_fail_2() {
    BigExponentFloat::ONE.tan_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn tan_prec_round_fail_3() {
    BigExponentFloat::tan_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn tan_prec_round_fail_4() {
    BigExponentFloat::tan_rational_prec_round(Rational::ONE, 10, Exact);
}

#[test]
#[should_panic]
fn tan_prec_round_fail_5() {
    BigExponentFloat::ONE.tan_with_period_prec_round(7, 0, Floor);
}

#[test]
#[should_panic]
fn tan_prec_round_fail_6() {
    BigExponentFloat::ONE.tan_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn tan_prec_round_fail_7() {
    BigExponentFloat::tan_with_period_rational_prec_round(Rational::ONE, 7, 0, Floor);
}

#[test]
#[should_panic]
fn tan_prec_round_fail_8() {
    BigExponentFloat::tan_with_period_rational_prec_round(Rational::ONE, 7, 10, Exact);
}

#[test]
#[should_panic]
fn tan_prec_round_fail_9() {
    BigExponentFloat::from(0.375).tan_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn tan_prec_round_fail_10() {
    BigExponentFloat::from(0.375).tan_pi_prec_round(10, Exact);
}

#[test]
fn tan_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_21().test_properties(
        |(x, prec, rm)| {
            let r = tan_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with the implementation that does not use them.
            assert!(same_pair(
                &trig_prec_round_generic(TrigFunction::Tan, &x, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if x.is_finite() && !x.is_zero() {
                // tan is odd
                let (s, o) = tan_prec_round_all(&-&x, prec, -rm);
                assert!(same(&s, &-&r.0));
                assert_eq!(o, r.1.reverse());
            }
            if big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&tan_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn tan_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        let (y, o) = x.tan_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) && !x.is_zero() {
            let r = tan_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn tan_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        let r = tan_rational_prec_round_all(&x, prec, rm);
        assert!(same_pair(
            &trig_rational_prec_round_generic(TrigFunction::Tan, &x, prec, rm),
            &r
        ));
        check_result(&r, prec);
        let (y, o) = Float::tan_rational_prec_round_ref(&x, prec, rm);
        if float_result_comparable(&y, o) && x != 0u32 {
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn tan_with_period_prec_round_properties() {
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_8().test_properties(
        |(x, u, prec, rm)| {
            let r = tan_with_period_prec_round_all(&x, u, prec, rm);
            assert!(same_pair(
                &trig_with_period_prec_round_generic(TrigFunction::Tan, &x, u, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(
                    &tan_with_period_prec_round_all(&x, u, prec, Exact),
                    &r
                ));
            }
            if x.is_finite() && !x.is_zero() && u != 0 {
                let (s, o) = tan_with_period_prec_round_all(&-&x, u, prec, -rm);
                assert!(same(&s, &-&r.0));
                assert_eq!(o, r.1.reverse());
                if big_exponent_float_exponent_is_small(&x) {
                    // The function has period u.
                    let y = &x + BigExponentFloat::from(u);
                    assert!(same_pair(
                        &tan_with_period_prec_round_all(&y, u, prec, rm),
                        &r
                    ));
                    let q = Rational::try_from(&x).unwrap();
                    assert!(same_pair(
                        &tan_with_period_rational_prec_round_all(&q, u, prec, rm),
                        &r
                    ));
                }
            }
        },
    );
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_22().test_properties(
        |(x, prec, rm)| {
            tan_with_period_prec_round_all(&x, 2, prec, rm);
        },
    );
}

#[test]
fn tan_with_period_prec_round_float_agreement_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17().test_properties(
        |(x, u, prec, rm)| {
            if rm == Exact || x.is_zero() {
                return;
            }
            let (y, o) = x.tan_with_period_prec_round_ref(u, prec, rm);
            if float_result_comparable(&y, o) {
                let r =
                    tan_with_period_prec_round_all(&float_to_big_exponent_float(&x), u, prec, rm);
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
fn tan_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_13().test_properties(
        |(x, u, prec, rm)| {
            let r = tan_with_period_rational_prec_round_all(&x, u, prec, rm);
            assert!(same_pair(
                &trig_with_period_rational_prec_round_generic(TrigFunction::Tan, &x, u, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(
                    &tan_with_period_rational_prec_round_all(&x, u, prec, Exact),
                    &r
                ));
            }
            if x != 0u32 {
                let (y, o) = Float::tan_with_period_rational_prec_round_ref(&x, u, prec, rm);
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
                tan_with_period_rational_prec_round_all(&x, 2, prec, rm);
            }
        },
    );
}
