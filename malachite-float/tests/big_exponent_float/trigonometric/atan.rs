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

fn atan_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().atan_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.atan_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.atan_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().atan_prec(prec), &r));
        assert!(same_pair(&x.atan_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.atan_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn atan_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::atan_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::atan_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::atan_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::atan_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

fn atan_with_period_prec_round_all(
    x: &BigExponentFloat,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().atan_with_period_prec_round(u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.atan_with_period_prec_round_ref(u, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.atan_with_period_prec_round_assign(u, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().atan_with_period_prec(u, prec), &r));
        assert!(same_pair(&x.atan_with_period_prec_ref(u, prec), &r));
        let mut z = x.clone();
        let o = z.atan_with_period_prec_assign(u, prec);
        assert!(same_pair(&(z, o), &r));
    }
    if u == 2 {
        assert!(same_pair(&atan_pi_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn atan_with_period_rational_prec_round_all(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::atan_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::atan_with_period_rational_prec_round_ref(x, u, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::atan_with_period_rational_prec(x.clone(), u, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::atan_with_period_rational_prec_ref(x, u, prec),
            &r
        ));
    }
    if u == 2 {
        assert!(same_pair(&atan_pi_rational_prec_round_all(x, prec, rm), &r));
    }
    r
}

fn atan_pi_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().atan_pi_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.atan_pi_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.atan_pi_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().atan_pi_prec(prec), &r));
        assert!(same_pair(&x.atan_pi_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.atan_pi_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn atan_pi_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::atan_pi_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::atan_pi_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::atan_pi_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::atan_pi_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_atan_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = atan_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_trig_prec_round_generic(InverseTrigFunction::Atan, &x, None, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "1.570", "0x1.92", Less);
    test(
        "-Infinity",
        "-Infinity",
        10,
        Floor,
        "-1.5723",
        "-0x1.928",
        Less,
    );
    test("0.0", "0x0", 10, Exact, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8", 10, Floor, "0.46338", "0x0.76a", Less);
    test("-0.50", "-0x0.8", 10, Floor, "-0.4639", "-0x0.76c", Less);
    test("1.0", "0x1", 10, Floor, "0.7852", "0x0.c9", Less);
    test("-1.0", "-0x1", 10, Floor, "-0.78613", "-0x0.c94", Less);
    test("2.0", "0x2", 10, Floor, "1.105", "0x1.1b", Less);
    test("-2.0", "-0x2", 10, Floor, "-1.1074", "-0x1.1b8", Less);
    test("0.38", "0x0.6", 1, Nearest, "0.25", "0x0.4", Less);
    test("0.38", "0x0.6", 10, Floor, "0.3584", "0x0.5bc", Less);
    test("0.38", "0x0.6", 10, Ceiling, "0.35889", "0x0.5be", Greater);
    test(
        "0.38",
        "0x0.6",
        100,
        Nearest,
        "0.3587706702705722203959200639265",
        "0x0.5bd86507937bc239c55190917",
        Greater,
    );
    test("-0.38", "-0x0.6", 1, Nearest, "-0.25", "-0x0.4", Greater);
    test("-0.38", "-0x0.6", 10, Floor, "-0.35889", "-0x0.5be", Less);
    test(
        "-0.38", "-0x0.6", 10, Ceiling, "-0.3584", "-0x0.5bc", Greater,
    );
    test(
        "-0.38",
        "-0x0.6",
        100,
        Nearest,
        "-0.3587706702705722203959200639265",
        "-0x0.5bd86507937bc239c55190917",
        Less,
    );
    test("0.9961", "0x0.ff", 1, Nearest, "1.0", "0x1.0", Greater);
    test("0.9961", "0x0.ff", 10, Floor, "0.7832", "0x0.c88", Less);
    test(
        "0.9961", "0x0.ff", 10, Ceiling, "0.78418", "0x0.c8c", Greater,
    );
    test(
        "0.9961",
        "0x0.ff",
        100,
        Nearest,
        "0.78344121873315176483942947588805",
        "0x0.c88f9a8ccc19d89d745076dfb",
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
        "0.7853981633974483096139667799253",
        "0x0.c90fdaa22168c234bcc6628b8",
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
        "-0.7853981633974483096139667799253",
        "-0x0.c90fdaa22168c234bcc6628b8",
        Greater,
    );
    test("3.0", "0x3", 1, Nearest, "1.0", "0x1.0", Less);
    test("3.0", "0x3", 10, Floor, "1.2480", "0x1.3f8", Less);
    test("3.0", "0x3", 10, Ceiling, "1.2", "0x1.4", Greater);
    test(
        "3.0",
        "0x3",
        100,
        Nearest,
        "1.2490457723982544258299170772811",
        "0x1.3fc176b7a855ffd77cb88581e",
        Greater,
    );
    test("-1024.5", "-0x400.8", 1, Nearest, "-2.0", "-0x2.0", Less);
    test("-1024.5", "-0x400.8", 10, Floor, "-1.570", "-0x1.92", Less);
    test(
        "-1024.5", "-0x400.8", 10, Ceiling, "-1.5684", "-0x1.918", Greater,
    );
    test(
        "-1024.5",
        "-0x400.8",
        100,
        Nearest,
        "-1.5698202412093241874359439265140",
        "-0x1.91dfbd4497c6e8f3e62d16236",
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
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Floor,
        "2.7563e-12041199827",
        "0xf.fcE-10000000001",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Ceiling,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        100,
        Nearest,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Greater,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        1,
        Nearest,
        "-5.5e-12041199827",
        "-0x2.0E-10000000000",
        Greater,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Floor,
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        Less,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Ceiling,
        "-8.2662e-12041199827",
        "-0x2.ffE-10000000000",
        Greater,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        100,
        Nearest,
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        Less,
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
        "0.7853981633974483096157667249382",
        "0x0.c90fdaa22168c234c546628b8",
        Less,
    );
}

#[test]
fn test_atan_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = atan_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("0", 10, Exact, "0.0", "0x0.0", Equal);
    test("1", 10, Floor, "0.7852", "0x0.c9", Less);
    test("-1", 10, Floor, "-0.78613", "-0x0.c94", Less);
    test("1/2", 10, Floor, "0.46338", "0x0.76a", Less);
    test("2", 10, Floor, "1.105", "0x1.1b", Less);
    test("1/3", 1, Nearest, "0.25", "0x0.4", Less);
    test("1/3", 10, Floor, "0.3213", "0x0.524", Less);
    test("1/3", 10, Ceiling, "0.32178", "0x0.526", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "0.3217505543966421934014046143585",
        "0x0.525e3e8c9a7b84920cd43f952",
        Less,
    );
    test("-2/3", 1, Nearest, "-0.50", "-0x0.8", Greater);
    test("-2/3", 10, Floor, "-0.58887", "-0x0.96c", Less);
    test("-2/3", 10, Ceiling, "-0.5879", "-0x0.968", Greater);
    test(
        "-2/3",
        100,
        Nearest,
        "-0.5880026035475675512456110806248",
        "-0x0.968756b0331cab691105160a4",
        Greater,
    );
    test("7/3", 1, Nearest, "1.0", "0x1.0", Less);
    test("7/3", 10, Floor, "1.164", "0x1.2a", Less);
    test("7/3", 10, Ceiling, "1.1660", "0x1.2a8", Greater);
    test(
        "7/3",
        100,
        Nearest,
        "1.1659045405098131959192487626305",
        "0x1.2a78b84fbf5fc114fb2718b62",
        Greater,
    );
    test("-7/3", 1, Nearest, "-1.0", "-0x1.0", Greater);
    test("-7/3", 10, Floor, "-1.1660", "-0x1.2a8", Less);
    test("-7/3", 10, Ceiling, "-1.164", "-0x1.2a", Greater);
    test(
        "-7/3",
        100,
        Nearest,
        "-1.1659045405098131959192487626305",
        "-0x1.2a78b84fbf5fc114fb2718b62",
        Less,
    );
    test(
        "1/1000000000000000000000",
        1,
        Nearest,
        "8.5e-22",
        "0x4.0E-18",
        Less,
    );
    test(
        "1/1000000000000000000000",
        10,
        Floor,
        "9.992e-22",
        "0x4.b8E-18",
        Less,
    );
    test(
        "1/1000000000000000000000",
        10,
        Ceiling,
        "1.0009e-21",
        "0x4.baE-18",
        Greater,
    );
    test(
        "1/1000000000000000000000",
        100,
        Nearest,
        "1.0000000000000000000000000000004e-21",
        "0x4.b8ed0283a6d3df769fb7e0b78E-18",
        Greater,
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
        "0.78539816339694830961566059582015",
        "0x0.c90fdaa220dc0568bb56c4a3b",
        Greater,
    );
}

#[test]
fn test_atan_with_period_prec_round() {
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
        let r = atan_with_period_prec_round_all(&x, u, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_trig_prec_round_generic(InverseTrigFunction::Atan, &x, Some(u), prec, rm),
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
        "-1.8",
        "-0x1.c",
        Equal,
    );
    test("0.0", "0x0", 24, 10, Exact, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", 7, 10, Exact, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "0.50", "0x0.8", 24, 10, Nearest, "1.7715", "0x1.c58", Greater,
    );
    test(
        "0.50", "0x0.8", 7, 10, Nearest, "0.51660", "0x0.844", Greater,
    );
    test("0.50", "0x0.8", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "-0.50", "-0x0.8", 24, 10, Nearest, "-1.7715", "-0x1.c58", Less,
    );
    test(
        "-0.50", "-0x0.8", 7, 10, Nearest, "-0.51660", "-0x0.844", Less,
    );
    test("-0.50", "-0x0.8", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", 24, 10, Exact, "3.0", "0x3.0", Equal);
    test("1.0", "0x1", 7, 10, Exact, "0.88", "0x0.e", Equal);
    test("1.0", "0x1", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1", 24, 10, Exact, "-3.0", "-0x3.0", Equal);
    test("-1.0", "-0x1", 7, 10, Exact, "-0.88", "-0x0.e", Equal);
    test("-1.0", "-0x1", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("2.0", "0x2", 24, 10, Nearest, "4.2266", "0x4.3a", Less);
    test("2.0", "0x2", 7, 10, Nearest, "1.234", "0x1.3c", Greater);
    test("2.0", "0x2", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "-2.0", "-0x2", 24, 10, Nearest, "-4.2266", "-0x4.3a", Greater,
    );
    test("-2.0", "-0x2", 7, 10, Nearest, "-1.234", "-0x1.3c", Less);
    test("-2.0", "-0x2", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "0.38", "0x0.6", 7, 10, Nearest, "0.39990", "0x0.666", Greater,
    );
    test("0.38", "0x0.6", 7, 10, Floor, "0.3994", "0x0.664", Less);
    test(
        "0.38",
        "0x0.6",
        7,
        100,
        Nearest,
        "0.39970087926967847266126469231",
        "0x0.6652cbfca54d7942fb77e614",
        Greater,
    );
    test("-3.0", "-0x3", 7, 10, Nearest, "-1.391", "-0x1.64", Greater);
    test("-3.0", "-0x3", 7, 10, Floor, "-1.3926", "-0x1.648", Less);
    test(
        "-3.0",
        "-0x3",
        7,
        100,
        Nearest,
        "-1.391542661776516459613903766787",
        "-0x1.643c23cf51a53acc5dbebffc4",
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
        "3.072e-12041199827",
        "0x1.1dE-10000000000",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
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
        7,
        100,
        Nearest,
        "3.0737628908302917218934856579376e-12041199827",
        "0x1.1d34a60108f725097944d9af6E-10000000000",
        Less,
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
        "-1.5813e-12041199825",
        "-0x3.95E-9999999999",
        Less,
    );
    test(
        "-2.8e-12041199827",
        "-0x1.0E-10000000000",
        360,
        10,
        Floor,
        "-1.5813e-12041199825",
        "-0x3.95E-9999999999",
        Less,
    );
    test(
        "-2.8e-12041199827",
        "-0x1.0E-10000000000",
        360,
        100,
        Nearest,
        "-1.580792343855578599830935481226e-12041199825",
        "-0x3.94bb834c783ef70c2a5d4dfdE-9999999999",
        Greater,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        5,
        10,
        Nearest,
        "0.62",
        "0x0.a",
        Greater,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        5,
        10,
        Floor,
        "0.62402",
        "0x0.9fc",
        Less,
    );
    test(
        "0.9999999999999999999966",
        "0x0.fffffffffffffffff",
        5,
        100,
        Nearest,
        "0.62499999999999999999865190519478",
        "0x0.9ffffffffffffffff9a240ddb",
        Less,
    );
}

#[test]
fn test_atan_with_period_rational_prec_round() {
    let test =
        |x: &str, u: u64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            let x = Rational::from_str(x).unwrap();
            let r = atan_with_period_rational_prec_round_all(&x, u, prec, rm);
            assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
            assert_eq!(r.0.to_string(), out);
            assert_eq!(r.1, o, "{x} {prec} {rm}");
        };
    test("0", 24, 10, Exact, "0.0", "0x0.0", Equal);
    test("0", 7, 10, Exact, "0.0", "0x0.0", Equal);
    test("1", 24, 10, Exact, "3.0", "0x3.0", Equal);
    test("1", 7, 10, Exact, "0.88", "0x0.e", Equal);
    test("1/2", 24, 10, Nearest, "1.7715", "0x1.c58", Greater);
    test("1/2", 7, 10, Nearest, "0.51660", "0x0.844", Greater);
    test("-2", 24, 10, Nearest, "-4.2266", "-0x4.3a", Greater);
    test("-2", 7, 10, Nearest, "-1.234", "-0x1.3c", Less);
    test("1/3", 7, 10, Nearest, "0.3584", "0x0.5bc", Less);
    test("1/3", 7, 10, Floor, "0.3584", "0x0.5bc", Less);
    test(
        "1/3",
        7,
        100,
        Nearest,
        "0.35845733822348354038609623321337",
        "0x0.5bc3dc30ae5ac533a2414003c8",
        Less,
    );
    test("-7/3", 7, 10, Nearest, "-1.2988", "-0x1.4c8", Greater);
    test("-7/3", 7, 10, Floor, "-1.301", "-0x1.4d", Less);
    test(
        "-7/3",
        7,
        100,
        Nearest,
        "-1.2989162955679518955325390628017",
        "-0x1.4c85c741b4b0d33a3ba2e0422",
        Less,
    );
    test(
        "1/10000000000000000000000000",
        5,
        10,
        Nearest,
        "7.957e-26",
        "0x1.8aE-21",
        Less,
    );
    test(
        "1/10000000000000000000000000",
        5,
        10,
        Floor,
        "7.957e-26",
        "0x1.8aE-21",
        Less,
    );
    test(
        "1/10000000000000000000000000",
        5,
        100,
        Nearest,
        "7.957747154594766788444188168626e-26",
        "0x1.8a0c6df84a6b45ae164ea4f64E-21",
        Greater,
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
    test("-1/2", 7, 10, Nearest, "-0.51660", "-0x0.844", Less);
    test("-1/2", 7, 10, Floor, "-0.51660", "-0x0.844", Less);
    test(
        "-1/2",
        7,
        100,
        Nearest,
        "-0.51654266177651645961390376678624",
        "-0x0.843c23cf51a53acc5dbebffc3",
        Greater,
    );
}

#[test]
fn test_atan_pi_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = atan_pi_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_trig_prec_round_generic(InverseTrigFunction::Atan, &x, Some(2), prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "0.50", "0x0.8", Equal);
    test("0.0", "0x0", 10, Exact, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8", 10, Nearest, "0.14771", "0x0.25d", Greater);
    test("0.50", "0x0.8", 10, Floor, "0.1475", "0x0.25c", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "0.1475836176504332741754010762248",
        "0x0.25c80a3b3be610ccad11edb5c8",
        Greater,
    );
    test("-0.50", "-0x0.8", 10, Nearest, "-0.14771", "-0x0.25d", Less);
    test("-0.50", "-0x0.8", 10, Floor, "-0.14771", "-0x0.25d", Less);
    test(
        "-0.50",
        "-0x0.8",
        100,
        Nearest,
        "-0.1475836176504332741754010762248",
        "-0x0.25c80a3b3be610ccad11edb5c8",
        Less,
    );
    test("1.0", "0x1", 10, Exact, "0.25", "0x0.4", Equal);
    test("1.0", "0x1", 10, Floor, "0.25", "0x0.4", Equal);
    test("1.0", "0x1", 100, Exact, "0.25", "0x0.4", Equal);
    test("-1.0", "-0x1", 10, Exact, "-0.25", "-0x0.4", Equal);
    test("-1.0", "-0x1", 10, Floor, "-0.25", "-0x0.4", Equal);
    test("-1.0", "-0x1", 100, Exact, "-0.25", "-0x0.4", Equal);
    test("2.0", "0x2", 10, Nearest, "0.3525", "0x0.5a4", Greater);
    test("2.0", "0x2", 10, Floor, "0.35205", "0x0.5a2", Less);
    test(
        "2.0",
        "0x2",
        100,
        Nearest,
        "0.35241638234956672582459892377519",
        "0x0.5a37f5c4c419ef3352ee124a38",
        Less,
    );
    test("0.38", "0x0.6", 10, Nearest, "0.1143", "0x0.1d4", Greater);
    test("0.38", "0x0.6", 10, Floor, "0.11414", "0x0.1d38", Less);
    test(
        "0.38",
        "0x0.6",
        100,
        Nearest,
        "0.1142002512199081350460756263741",
        "0x0.1d3c3a482f3ab4ee90fdaf736c",
        Less,
    );
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
        "8.784e-12041199828",
        "0x5.18E-10000000001",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Floor,
        "8.7701e-12041199828",
        "0x5.16E-10000000001",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        100,
        Nearest,
        "8.7821796880865477768385304512518e-12041199828",
        "0x5.17cc1b727220a94fe13abe8f8E-10000000001",
        Less,
    );
}

#[test]
fn test_atan_pi_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = atan_pi_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("1/2", 10, Nearest, "0.14771", "0x0.25d", Greater);
    test("1/2", 10, Floor, "0.1475", "0x0.25c", Less);
    test(
        "1/2",
        100,
        Nearest,
        "0.1475836176504332741754010762248",
        "0x0.25c80a3b3be610ccad11edb5c8",
        Greater,
    );
    test("-1/2", 10, Nearest, "-0.14771", "-0x0.25d", Less);
    test("-1/2", 10, Floor, "-0.14771", "-0x0.25d", Less);
    test(
        "-1/2",
        100,
        Nearest,
        "-0.1475836176504332741754010762248",
        "-0x0.25c80a3b3be610ccad11edb5c8",
        Less,
    );
    test("1", 10, Exact, "0.25", "0x0.4", Equal);
    test("1", 10, Floor, "0.25", "0x0.4", Equal);
    test("1", 100, Exact, "0.25", "0x0.4", Equal);
    test("2", 10, Nearest, "0.3525", "0x0.5a4", Greater);
    test("2", 10, Floor, "0.35205", "0x0.5a2", Less);
    test(
        "2",
        100,
        Nearest,
        "0.35241638234956672582459892377519",
        "0x0.5a37f5c4c419ef3352ee124a38",
        Less,
    );
    test("1/3", 10, Nearest, "0.10242", "0x0.1a38", Greater);
    test("1/3", 10, Floor, "0.1023", "0x0.1a3", Less);
    test(
        "1/3",
        100,
        Nearest,
        "0.10241638234956672582459892377529",
        "0x0.1a37f5c4c419ef3352ee124a3a",
        Greater,
    );
    test("-5/7", 10, Nearest, "-0.19751", "-0x0.329", Less);
    test("-5/7", 10, Floor, "-0.19751", "-0x0.329", Less);
    test(
        "-5/7",
        100,
        Nearest,
        "-0.19743154328874657004922183032356",
        "-0x0.328adfa59fbcd00d3e95563ebc",
        Greater,
    );
}

#[test]
#[should_panic]
fn atan_prec_round_fail_1() {
    BigExponentFloat::from(0.375).atan_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn atan_prec_round_fail_2() {
    BigExponentFloat::from(0.375).atan_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn atan_prec_round_fail_3() {
    BigExponentFloat::atan_rational_prec_round(Rational::from_unsigneds(1u8, 3), 0, Floor);
}

#[test]
#[should_panic]
fn atan_prec_round_fail_4() {
    BigExponentFloat::atan_rational_prec_round(Rational::from_unsigneds(1u8, 3), 10, Exact);
}

#[test]
#[should_panic]
fn atan_prec_round_fail_5() {
    BigExponentFloat::from(0.375).atan_with_period_prec_round(7, 0, Floor);
}

#[test]
#[should_panic]
fn atan_prec_round_fail_6() {
    BigExponentFloat::from(0.375).atan_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn atan_prec_round_fail_7() {
    BigExponentFloat::atan_with_period_rational_prec_round(
        Rational::from_unsigneds(1u8, 3),
        7,
        0,
        Floor,
    );
}

#[test]
#[should_panic]
fn atan_prec_round_fail_8() {
    BigExponentFloat::from(0.375).atan_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn atan_prec_round_fail_9() {
    BigExponentFloat::from(0.375).atan_pi_prec_round(10, Exact);
}

#[test]
fn atan_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties(
        |(x, prec, rm)| {
            let r = atan_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with the decomposition wherever both apply.
            assert!(same_pair(
                &inverse_trig_prec_round_generic(InverseTrigFunction::Atan, &x, None, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(&atan_prec_round_all(&x, prec, Exact), &r));
            }
            if x.is_finite() && !x.is_zero() && rm != Exact {
                // atan is odd
                let (s, o) = atan_prec_round_all(&-&x, prec, -rm);
                assert!(same(&s, &-&r.0));
                assert_eq!(o, r.1.reverse());
            }
            if rm != Exact
                && big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&atan_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn atan_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        // the `Float` generator's `Exact` cases are not all exact for this function
        if x.is_zero() || rm == Exact {
            return;
        }
        let (y, o) = x.atan_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) {
            let r = atan_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
        if rm != Exact {
            for u in PERIODS {
                let (y, o) = x.atan_with_period_prec_round_ref(u, prec, rm);
                if float_result_comparable(&y, o) {
                    let r = atan_with_period_prec_round_all(
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
fn atan_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        let r = atan_rational_prec_round_all(&x, prec, rm);
        check_result(&r, prec);
        let (y, o) = Float::atan_rational_prec_round_ref(&x, prec, rm);
        if float_result_comparable(&y, o) {
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn atan_with_period_prec_round_properties() {
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_9().test_properties(
        |(x, u, prec, rm)| {
            let r = atan_with_period_prec_round_all(&x, u, prec, rm);
            assert!(same_pair(
                &inverse_trig_prec_round_generic(InverseTrigFunction::Atan, &x, Some(u), prec, rm),
                &r
            ));
            check_result(&r, prec);
            if r.1 == Equal && !r.0.is_nan() {
                assert!(same_pair(
                    &atan_with_period_prec_round_all(&x, u, prec, Exact),
                    &r
                ));
            }
            if rm != Exact
                && big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(
                    &atan_with_period_rational_prec_round_all(&q, u, prec, rm),
                    &r
                ));
            }
        },
    );
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties(
        |(x, prec, rm)| {
            atan_with_period_prec_round_all(&x, 2, prec, rm);
        },
    );
}

#[test]
fn atan_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_6().test_properties(
        |(x, u, prec, rm)| {
            let r = atan_with_period_rational_prec_round_all(&x, u, prec, rm);
            check_result(&r, prec);
            let (y, o) = Float::atan_with_period_rational_prec_round_ref(&x, u, prec, rm);
            if float_result_comparable(&y, o) {
                assert!(same(&r.0, &float_to_big_exponent_float(&y)));
                assert_eq!(r.1, o);
            }
        },
    );
}
