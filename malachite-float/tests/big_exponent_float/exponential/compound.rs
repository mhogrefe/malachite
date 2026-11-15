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
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::test_util::generators::{
    big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_5,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_17,
};
use malachite_float::{BigExponentFloat, Float};
use std::cmp::Ordering::{self, *};

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

// Whether the exponent of `x`, if it has one, is at most 256 in absolute value, so that exact
// conversions and sums involving x stay small.
fn exponent_is_moderate(x: &BigExponentFloat) -> bool {
    x.get_exponent()
        .is_none_or(|e| e.unsigned_abs_ref() <= &256u32)
}

fn compound_prec_round_all(
    x: &BigExponentFloat,
    b: i64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().compound_prec_round(b, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.compound_prec_round_ref(b, prec, rm), &r));
    let mut z = x.clone();
    let o = z.compound_prec_round_assign(b, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().compound_prec(b, prec), &r));
        assert!(same_pair(&x.compound_prec_ref(b, prec), &r));
        let mut z = x.clone();
        let o = z.compound_prec_assign(b, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

#[test]
fn test_compound_prec_round() {
    // The expected values agree with FLINT's arb (see the `arf_logarithm_general` oracle mode).
    let test = |x: &str,
                x_hex: &str,
                b: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let r = compound_prec_round_all(&hex(x_hex), b.parse::<i64>().unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {b} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {b} {prec} {rm}");
    };
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        1,
        Nearest,
        "64.0",
        "0x4.0E+1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        10,
        Floor,
        "71.00",
        "0x47.0",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        10,
        Ceiling,
        "71.125",
        "0x47.2",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        10,
        Down,
        "71.00",
        "0x47.0",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        10,
        Up,
        "71.125",
        "0x47.2",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        10,
        Nearest,
        "71.00",
        "0x47.0",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        100,
        Nearest,
        "71.039867844337269445535014412757",
        "0x47.0a34c76f9730f9d2cd8f6758",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        300,
        Exact,
        "71.039867844337269445535014412800892272694653209",
        "0x47.0a34c76f9730f9d2cd8f675b782a821d5bfb",
        Equal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "-2",
        1,
        Nearest,
        "0.062",
        "0x0.1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "-2",
        10,
        Floor,
        "0.058289",
        "0x0.0eec",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "-2",
        10,
        Ceiling,
        "0.05835",
        "0x0.0ef",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "-2",
        10,
        Down,
        "0.058289",
        "0x0.0eec",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "-2",
        10,
        Up,
        "0.05835",
        "0x0.0ef",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "-2",
        10,
        Nearest,
        "0.058289",
        "0x0.0eec",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "-2",
        100,
        Nearest,
        "0.05829955459186468327723691089322",
        "0x0.0eecb83857eabad36f64ae2332",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "1000000",
        1,
        Nearest,
        "3.0e617167",
        "0x4.0E+512546",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "1000000",
        10,
        Floor,
        "2.4066e617167",
        "0x3.2bE+512546",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "1000000",
        10,
        Ceiling,
        "2.410e617167",
        "0x3.2cE+512546",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "1000000",
        10,
        Down,
        "2.4066e617167",
        "0x3.2bE+512546",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "1000000",
        10,
        Up,
        "2.410e617167",
        "0x3.2cE+512546",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "1000000",
        10,
        Nearest,
        "2.4066e617167",
        "0x3.2bE+512546",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "1000000",
        100,
        Nearest,
        "2.4077830705577390137594714655877e617167",
        "0x3.2b628d58b1fa5d291b3360144E+512546",
        Greater,
    );
    test("0.50", "0x0.8", "3", 1, Nearest, "4.0", "0x4.0", Greater);
    test("0.50", "0x0.8", "3", 10, Floor, "3.38", "0x3.6", Equal);
    test("0.50", "0x0.8", "3", 10, Ceiling, "3.38", "0x3.6", Equal);
    test("0.50", "0x0.8", "3", 10, Down, "3.38", "0x3.6", Equal);
    test("0.50", "0x0.8", "3", 10, Up, "3.38", "0x3.6", Equal);
    test("0.50", "0x0.8", "3", 10, Nearest, "3.38", "0x3.6", Equal);
    test("0.50", "0x0.8", "3", 100, Nearest, "3.38", "0x3.6", Equal);
    test("0.50", "0x0.8", "3", 300, Exact, "3.38", "0x3.6", Equal);
    test("0.50", "0x0.8", "-2", 1, Nearest, "0.50", "0x0.8", Greater);
    test("0.50", "0x0.8", "-2", 10, Floor, "0.4443", "0x0.71c", Less);
    test(
        "0.50", "0x0.8", "-2", 10, Ceiling, "0.44482", "0x0.71e", Greater,
    );
    test("0.50", "0x0.8", "-2", 10, Down, "0.4443", "0x0.71c", Less);
    test("0.50", "0x0.8", "-2", 10, Up, "0.44482", "0x0.71e", Greater);
    test(
        "0.50", "0x0.8", "-2", 10, Nearest, "0.4443", "0x0.71c", Less,
    );
    test(
        "0.50",
        "0x0.8",
        "-2",
        100,
        Nearest,
        "0.4444444444444444444444444444444",
        "0x0.71c71c71c71c71c71c71c71c7",
        Less,
    );
    test(
        "0.50",
        "0x0.8",
        "1000000",
        1,
        Nearest,
        "1.3e176091",
        "0x4.0E+146240",
        Less,
    );
    test(
        "0.50",
        "0x0.8",
        "1000000",
        10,
        Floor,
        "1.815e176091",
        "0x5.a8E+146240",
        Less,
    );
    test(
        "0.50",
        "0x0.8",
        "1000000",
        10,
        Ceiling,
        "1.8172e176091",
        "0x5.aaE+146240",
        Greater,
    );
    test(
        "0.50",
        "0x0.8",
        "1000000",
        10,
        Down,
        "1.815e176091",
        "0x5.a8E+146240",
        Less,
    );
    test(
        "0.50",
        "0x0.8",
        "1000000",
        10,
        Up,
        "1.8172e176091",
        "0x5.aaE+146240",
        Greater,
    );
    test(
        "0.50",
        "0x0.8",
        "1000000",
        10,
        Nearest,
        "1.815e176091",
        "0x5.a8E+146240",
        Less,
    );
    test(
        "0.50",
        "0x0.8",
        "1000000",
        100,
        Nearest,
        "1.815748446407366073936138510257e176091",
        "0x5.a8e0f6076b7d71b9c4eaee69E+146240",
        Greater,
    );
    test("-0.50", "-0x0.8", "3", 1, Nearest, "0.12", "0x0.2", Equal);
    test("-0.50", "-0x0.8", "3", 10, Floor, "0.12", "0x0.2", Equal);
    test("-0.50", "-0x0.8", "3", 10, Ceiling, "0.12", "0x0.2", Equal);
    test("-0.50", "-0x0.8", "3", 10, Down, "0.12", "0x0.2", Equal);
    test("-0.50", "-0x0.8", "3", 10, Up, "0.12", "0x0.2", Equal);
    test("-0.50", "-0x0.8", "3", 10, Nearest, "0.12", "0x0.2", Equal);
    test("-0.50", "-0x0.8", "3", 100, Nearest, "0.12", "0x0.2", Equal);
    test("-0.50", "-0x0.8", "3", 300, Exact, "0.12", "0x0.2", Equal);
    test("-0.50", "-0x0.8", "-2", 1, Nearest, "4.0", "0x4.0", Equal);
    test("-0.50", "-0x0.8", "-2", 10, Floor, "4.0", "0x4.0", Equal);
    test("-0.50", "-0x0.8", "-2", 10, Ceiling, "4.0", "0x4.0", Equal);
    test("-0.50", "-0x0.8", "-2", 10, Down, "4.0", "0x4.0", Equal);
    test("-0.50", "-0x0.8", "-2", 10, Up, "4.0", "0x4.0", Equal);
    test("-0.50", "-0x0.8", "-2", 10, Nearest, "4.0", "0x4.0", Equal);
    test("-0.50", "-0x0.8", "-2", 100, Nearest, "4.0", "0x4.0", Equal);
    test("-0.50", "-0x0.8", "-2", 300, Exact, "4.0", "0x4.0", Equal);
    test(
        "-0.50",
        "-0x0.8",
        "1000000",
        1,
        Nearest,
        "1.0e-301030",
        "0x1.0E-250000",
        Equal,
    );
    test(
        "-0.50",
        "-0x0.8",
        "1000000",
        10,
        Floor,
        "1.0e-301030",
        "0x1.0E-250000",
        Equal,
    );
    test(
        "-0.50",
        "-0x0.8",
        "1000000",
        10,
        Ceiling,
        "1.0e-301030",
        "0x1.0E-250000",
        Equal,
    );
    test(
        "-0.50",
        "-0x0.8",
        "1000000",
        10,
        Down,
        "1.0e-301030",
        "0x1.0E-250000",
        Equal,
    );
    test(
        "-0.50",
        "-0x0.8",
        "1000000",
        10,
        Up,
        "1.0e-301030",
        "0x1.0E-250000",
        Equal,
    );
    test(
        "-0.50",
        "-0x0.8",
        "1000000",
        10,
        Nearest,
        "1.0e-301030",
        "0x1.0E-250000",
        Equal,
    );
    test(
        "-0.50",
        "-0x0.8",
        "1000000",
        100,
        Nearest,
        "1.0e-301030",
        "0x1.0E-250000",
        Equal,
    );
    test(
        "-0.50",
        "-0x0.8",
        "1000000",
        300,
        Exact,
        "1.0e-301030",
        "0x1.0E-250000",
        Equal,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "3",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "3",
        10,
        Floor,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "3",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "3",
        10,
        Down,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "3",
        10,
        Up,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "3",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "3",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "-2",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "-2",
        10,
        Floor,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "-2",
        10,
        Ceiling,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "-2",
        10,
        Down,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "-2",
        10,
        Up,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "-2",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "-2",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "1000000",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "1000000",
        10,
        Floor,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "1000000",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "1000000",
        10,
        Down,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "1000000",
        10,
        Up,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "1000000",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        "1000000",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "3",
        1,
        Nearest,
        "1.8e72",
        "0x1.0E+60",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "3",
        10,
        Floor,
        "1.8e72",
        "0x1.0E+60",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "3",
        10,
        Ceiling,
        "1.7703e72",
        "0x1.008E+60",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "3",
        10,
        Down,
        "1.8e72",
        "0x1.0E+60",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "3",
        10,
        Up,
        "1.7703e72",
        "0x1.008E+60",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "3",
        10,
        Nearest,
        "1.8e72",
        "0x1.0E+60",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "3",
        100,
        Nearest,
        "1.7668470647783843295833019e72",
        "0x1.00000000000000000003E+60",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "3",
        300,
        Exact,
        "1766847064778384329583301885247830508536238507933743884429509056614367233.0",
        "0x1000000000000000000030000000000000000000300000000000000000001.0",
        Equal,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "-2",
        1,
        Nearest,
        "6.8e-49",
        "0x1.0E-40",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "-2",
        10,
        Floor,
        "6.8356e-49",
        "0xf.fcE-41",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "-2",
        10,
        Ceiling,
        "6.8e-49",
        "0x1.0E-40",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "-2",
        10,
        Down,
        "6.8356e-49",
        "0xf.fcE-41",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "-2",
        10,
        Up,
        "6.8e-49",
        "0x1.0E-40",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "-2",
        10,
        Nearest,
        "6.8e-49",
        "0x1.0E-40",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "-2",
        100,
        Nearest,
        "6.842277657836020854119762e-49",
        "0xf.ffffffffffffffffffeE-41",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "1000000",
        1,
        Nearest,
        "4.5e24082399",
        "0x1.0E+20000000",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "1000000",
        10,
        Floor,
        "4.5e24082399",
        "0x1.0E+20000000",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "1000000",
        10,
        Ceiling,
        "4.5078e24082399",
        "0x1.008E+20000000",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "1000000",
        10,
        Down,
        "4.5e24082399",
        "0x1.0E+20000000",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "1000000",
        10,
        Up,
        "4.5078e24082399",
        "0x1.008E+20000000",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "1000000",
        10,
        Nearest,
        "4.5e24082399",
        "0x1.0E+20000000",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        "1000000",
        100,
        Nearest,
        "4.49902592379675381465023e24082399",
        "0x1.000000000000000f424E+20000000",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3",
        1,
        Nearest,
        "2.5e1083707984",
        "0x1.0E+900000000",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3",
        10,
        Floor,
        "2.5e1083707984",
        "0x1.0E+900000000",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3",
        10,
        Ceiling,
        "2.4614e1083707984",
        "0x1.008E+900000000",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3",
        10,
        Down,
        "2.5e1083707984",
        "0x1.0E+900000000",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3",
        10,
        Up,
        "2.4614e1083707984",
        "0x1.008E+900000000",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3",
        10,
        Nearest,
        "2.5e1083707984",
        "0x1.0E+900000000",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3",
        100,
        Nearest,
        "2.5e1083707984",
        "0x1.0E+900000000",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "-2",
        1,
        Nearest,
        "2.5e-722471990",
        "0x1.0E-600000000",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "-2",
        10,
        Floor,
        "2.5470e-722471990",
        "0xf.fcE-600000001",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "-2",
        10,
        Ceiling,
        "2.5e-722471990",
        "0x1.0E-600000000",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "-2",
        10,
        Down,
        "2.5470e-722471990",
        "0xf.fcE-600000001",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "-2",
        10,
        Up,
        "2.5e-722471990",
        "0x1.0E-600000000",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "-2",
        10,
        Nearest,
        "2.5e-722471990",
        "0x1.0E-600000000",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "-2",
        100,
        Nearest,
        "2.5e-722471990",
        "0x1.0E-600000000",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "1000000",
        1,
        Nearest,
        "2.7e361235994796777",
        "0x1.0E+300000000000000",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "1000000",
        10,
        Floor,
        "2.7e361235994796777",
        "0x1.0E+300000000000000",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "1000000",
        10,
        Ceiling,
        "2.7234e361235994796777",
        "0x1.008E+300000000000000",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "1000000",
        10,
        Down,
        "2.7e361235994796777",
        "0x1.0E+300000000000000",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "1000000",
        10,
        Up,
        "2.7234e361235994796777",
        "0x1.008E+300000000000000",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "1000000",
        10,
        Nearest,
        "2.7e361235994796777",
        "0x1.0E+300000000000000",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "1000000",
        100,
        Nearest,
        "2.7e361235994796777",
        "0x1.0E+300000000000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        10,
        Floor,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        10,
        Down,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        10,
        Up,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "-2",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "-2",
        10,
        Floor,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "-2",
        10,
        Ceiling,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "-2",
        10,
        Down,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "-2",
        10,
        Up,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "-2",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "-2",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "1000000",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "1000000",
        10,
        Floor,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "1000000",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "1000000",
        10,
        Down,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "1000000",
        10,
        Up,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "1000000",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "1000000",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test("3.0", "0x3", "3", 1, Nearest, "64.0", "0x4.0E+1", Equal);
    test("3.0", "0x3", "3", 10, Floor, "64.0", "0x4.0E+1", Equal);
    test("3.0", "0x3", "3", 10, Ceiling, "64.0", "0x4.0E+1", Equal);
    test("3.0", "0x3", "3", 10, Down, "64.0", "0x4.0E+1", Equal);
    test("3.0", "0x3", "3", 10, Up, "64.0", "0x4.0E+1", Equal);
    test("3.0", "0x3", "3", 10, Nearest, "64.0", "0x4.0E+1", Equal);
    test("3.0", "0x3", "3", 100, Nearest, "64.0", "0x4.0E+1", Equal);
    test("3.0", "0x3", "3", 300, Exact, "64.0", "0x4.0E+1", Equal);
    test("3.0", "0x3", "-2", 1, Nearest, "0.062", "0x0.1", Equal);
    test("3.0", "0x3", "-2", 10, Floor, "0.062", "0x0.1", Equal);
    test("3.0", "0x3", "-2", 10, Ceiling, "0.062", "0x0.1", Equal);
    test("3.0", "0x3", "-2", 10, Down, "0.062", "0x0.1", Equal);
    test("3.0", "0x3", "-2", 10, Up, "0.062", "0x0.1", Equal);
    test("3.0", "0x3", "-2", 10, Nearest, "0.062", "0x0.1", Equal);
    test("3.0", "0x3", "-2", 100, Nearest, "0.062", "0x0.1", Equal);
    test("3.0", "0x3", "-2", 300, Exact, "0.062", "0x0.1", Equal);
    test(
        "3.0",
        "0x3",
        "1000000",
        1,
        Nearest,
        "9.8e602059",
        "0x1.0E+500000",
        Equal,
    );
    test(
        "3.0",
        "0x3",
        "1000000",
        10,
        Floor,
        "9.8e602059",
        "0x1.0E+500000",
        Equal,
    );
    test(
        "3.0",
        "0x3",
        "1000000",
        10,
        Ceiling,
        "9.8e602059",
        "0x1.0E+500000",
        Equal,
    );
    test(
        "3.0",
        "0x3",
        "1000000",
        10,
        Down,
        "9.8e602059",
        "0x1.0E+500000",
        Equal,
    );
    test(
        "3.0",
        "0x3",
        "1000000",
        10,
        Up,
        "9.8e602059",
        "0x1.0E+500000",
        Equal,
    );
    test(
        "3.0",
        "0x3",
        "1000000",
        10,
        Nearest,
        "9.8e602059",
        "0x1.0E+500000",
        Equal,
    );
    test(
        "3.0",
        "0x3",
        "1000000",
        100,
        Nearest,
        "9.8e602059",
        "0x1.0E+500000",
        Equal,
    );
    test(
        "3.0",
        "0x3",
        "1000000",
        300,
        Exact,
        "9.8e602059",
        "0x1.0E+500000",
        Equal,
    );
    test("-1.0", "-0x1.0", "3", 1, Nearest, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0", "3", 10, Floor, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0", "3", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0", "3", 10, Down, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0", "3", 10, Up, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0", "3", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0", "3", 100, Nearest, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0", "3", 300, Exact, "0.0", "0x0.0", Equal);
    test(
        "-1.0", "-0x1.0", "-2", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "-1.0", "-0x1.0", "-2", 10, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "-1.0", "-0x1.0", "-2", 10, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "-1.0", "-0x1.0", "-2", 10, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "-1.0", "-0x1.0", "-2", 10, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "-1.0", "-0x1.0", "-2", 10, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "-1.0", "-0x1.0", "-2", 100, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "-1.0", "-0x1.0", "-2", 300, Exact, "Infinity", "Infinity", Equal,
    );
    test(
        "-1.0", "-0x1.0", "1000000", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-1.0", "-0x1.0", "1000000", 10, Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "-1.0", "-0x1.0", "1000000", 10, Ceiling, "0.0", "0x0.0", Equal,
    );
    test("-1.0", "-0x1.0", "1000000", 10, Down, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0", "1000000", 10, Up, "0.0", "0x0.0", Equal);
    test(
        "-1.0", "-0x1.0", "1000000", 10, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-1.0", "-0x1.0", "1000000", 100, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-1.0", "-0x1.0", "1000000", 300, Exact, "0.0", "0x0.0", Equal,
    );
}

#[test]
#[should_panic]
fn compound_prec_round_fail_1() {
    BigExponentFloat::from(5).compound_prec_round(3, 0, Floor);
}

#[test]
#[should_panic]
fn compound_prec_round_fail_2() {
    BigExponentFloat::from(core::f64::consts::PI).compound_prec_round(3, 10, Exact);
}

fn compound_prec_round_properties_helper(
    x: &BigExponentFloat,
    b: i64,
    prec: u64,
    rm: RoundingMode,
) {
    let r = compound_prec_round_all(x, b, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = compound_prec_round_all(x, b, prec, Floor);
        let ceiling = compound_prec_round_all(x, b, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(&compound_prec_round_all(x, b, prec, Exact), &r));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = compound_prec_round_all(x, b, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // For a moderate x > -1, (1 + x)^n is the nth power of the exact sum 1 + x. (At x = -1 the
    // conventions differ: (1 + x)^n is infinity for n < 0, while 0^n is NaN for odd n < 0, since
    // `BigExponentFloat` has no negative zero.)
    if x.is_finite() && exponent_is_moderate(x) && *x > -1i32 {
        let z = x + BigExponentFloat::ONE;
        assert!(same_pair(&z.pow_s_prec_round_ref(b, prec, rm), &r));
    }
}

#[test]
fn compound_prec_round_properties() {
    big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, b, prec, rm)| {
            compound_prec_round_properties_helper(&x, b, prec, rm);
        },
    );
}

#[test]
fn compound_prec_round_float_agreement_properties() {
    float_signed_unsigned_rounding_mode_quadruple_gen_var_17().test_properties(
        |(x, b, prec, rm)| {
            if rm == Exact {
                return;
            }
            let (f, fo) = x.compound_prec_round_ref(b, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = compound_prec_round_all(&float_to_big_exponent_float(&x), b, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
