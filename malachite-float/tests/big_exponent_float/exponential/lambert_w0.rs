// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, FromStringBase};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    assert_lambert_w_correctly_rounded, float_to_big_exponent_float,
};
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_24, float_unsigned_pair_gen_var_1,
    rational_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

fn same_pair(x: &(BigExponentFloat, Ordering), y: &(BigExponentFloat, Ordering)) -> bool {
    same(&x.0, &y.0) && x.1 == y.1
}

fn float_result_comparable(x: &Float, o: Ordering) -> bool {
    x.get_exponent().map_or_else(
        || !x.is_infinite() && (!x.is_zero() || o == Equal),
        |e| e > Float::MIN_EXPONENT + 1 && e < Float::MAX_EXPONENT - 1,
    )
}

fn hex(s: &str) -> BigExponentFloat {
    BigExponentFloat::from_string_base(16, s).unwrap()
}

fn exponent_is_moderate(x: &BigExponentFloat) -> bool {
    x.get_exponent()
        .is_none_or(|e| e.unsigned_abs_ref() <= &256u32)
}

fn lambert_w0_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().lambert_w0_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.lambert_w0_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.lambert_w0_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().lambert_w0_prec(prec), &r));
        assert!(same_pair(&x.lambert_w0_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.lambert_w0_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn lambert_w0_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::lambert_w0_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::lambert_w0_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::lambert_w0_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::lambert_w0_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_lambert_w0_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &lambert_w0_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &lambert_w0_prec_round_all(&inf, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &lambert_w0_prec_round_all(&ninf, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &lambert_w0_prec_round_all(&zero, 10, rm),
            &(zero.clone(), Equal)
        ));
        // Below -1/e
        assert!(same_pair(
            &lambert_w0_prec_round_all(&BigExponentFloat::NEGATIVE_ONE, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &lambert_w0_rational_prec_round_all(&Rational::ZERO, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &lambert_w0_rational_prec_round_all(&Rational::NEGATIVE_ONE, 10, rm),
            &(nan.clone(), Equal)
        ));
    }
    // The expected values agree with Azurite's verified Lambert W functions.
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let r = lambert_w0_prec_round_all(&hex(x_hex), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("1.0", "0x1.0", 1, Nearest, "0.50", "0x0.8", Less);
    test(
        "1.0",
        "0x1.0",
        64,
        Nearest,
        "0.567143290409783873017",
        "0x0.91304d7c74b2ba5f",
        Greater,
    );
    test("1.0", "0x1.0", 10, Floor, "0.5664", "0x0.91", Less);
    test("1.0", "0x1.0", 10, Ceiling, "0.56738", "0x0.914", Greater);
    test("1.0", "0x1.0", 10, Down, "0.5664", "0x0.91", Less);
    test("1.0", "0x1.0", 10, Up, "0.56738", "0x0.914", Greater);
    test("1.0", "0x1.0", 10, Nearest, "0.56738", "0x0.914", Greater);
    test("0.50", "0x0.8", 1, Nearest, "0.25", "0x0.4", Less);
    test(
        "0.50",
        "0x0.8",
        64,
        Nearest,
        "0.35173371124919582603",
        "0x0.5a0b3872b74bff59",
        Greater,
    );
    test("0.50", "0x0.8", 10, Floor, "0.352", "0x0.5a", Less);
    test("0.50", "0x0.8", 10, Ceiling, "0.35205", "0x0.5a2", Greater);
    test("0.50", "0x0.8", 10, Down, "0.352", "0x0.5a", Less);
    test("0.50", "0x0.8", 10, Up, "0.35205", "0x0.5a2", Greater);
    test("0.50", "0x0.8", 10, Nearest, "0.352", "0x0.5a", Less);
    test("2.0", "0x2.0", 1, Nearest, "1.0", "0x1.0", Greater);
    test(
        "2.0",
        "0x2.0",
        64,
        Nearest,
        "0.85260550201372549135",
        "0x0.da445aab89e28ccc",
        Greater,
    );
    test("2.0", "0x2.0", 10, Floor, "0.85254", "0x0.da4", Less);
    test("2.0", "0x2.0", 10, Ceiling, "0.8535", "0x0.da8", Greater);
    test("2.0", "0x2.0", 10, Down, "0.85254", "0x0.da4", Less);
    test("2.0", "0x2.0", 10, Up, "0.8535", "0x0.da8", Greater);
    test("2.0", "0x2.0", 10, Nearest, "0.85254", "0x0.da4", Less);
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        64,
        Nearest,
        "0.999999999999999973383",
        "0x0.fffffffffffffe15",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Floor,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Ceiling,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Down,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Up,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test("10.0", "0xa.0", 1, Nearest, "2.0", "0x2.0", Greater);
    test(
        "10.0",
        "0xa.0",
        64,
        Nearest,
        "1.7455280027406993831",
        "0x1.bedaec5606043dcc",
        Greater,
    );
    test("10.0", "0xa.0", 10, Floor, "1.7441", "0x1.be8", Less);
    test("10.0", "0xa.0", 10, Ceiling, "1.746", "0x1.bf", Greater);
    test("10.0", "0xa.0", 10, Down, "1.7441", "0x1.be8", Less);
    test("10.0", "0xa.0", 10, Up, "1.746", "0x1.bf", Greater);
    test("10.0", "0xa.0", 10, Nearest, "1.746", "0x1.bf", Greater);
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        1,
        Nearest,
        "16.0",
        "0x1.0E+1",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        64,
        Nearest,
        "20.0286854133049507815",
        "0x14.0757ed60045a902",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Floor,
        "20.0",
        "0x14.0",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Ceiling,
        "20.031",
        "0x14.08",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Down,
        "20.0",
        "0x14.0",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Up,
        "20.031",
        "0x14.08",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Nearest,
        "20.031",
        "0x14.08",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788",
        1,
        Nearest,
        "7.6e-6",
        "0x0.00008",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788",
        64,
        Nearest,
        "9.99990000149997415179e-6",
        "0x0.0000a7c53e5407f5c8d5",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788",
        10,
        Floor,
        "9.9987e-6",
        "0x0.0000a7c",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788",
        10,
        Ceiling,
        "0.0000100",
        "0x0.0000a8",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788",
        10,
        Down,
        "9.9987e-6",
        "0x0.0000a7c",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788",
        10,
        Up,
        "0.0000100",
        "0x0.0000a8",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788",
        10,
        Nearest,
        "9.9987e-6",
        "0x0.0000a7c",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        1,
        Nearest,
        "2.6e2",
        "0x1.0E+2",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        64,
        Nearest,
        "224.84310644511850155",
        "0xe0.d7d5d2f0d4ae32",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Floor,
        "224.75",
        "0xe0.c",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Ceiling,
        "225.0",
        "0xe1.0",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Down,
        "224.75",
        "0xe0.c",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Up,
        "225.0",
        "0xe1.0",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Nearest,
        "224.75",
        "0xe0.c",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84",
        1,
        Nearest,
        "1.1e-100",
        "0x1.0E-83",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84",
        64,
        Nearest,
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84",
        10,
        Floor,
        "9.9900e-101",
        "0xd.fcE-84",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84",
        10,
        Ceiling,
        "1.0e-100",
        "0xe.0E-84",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84",
        10,
        Down,
        "9.9900e-101",
        "0xd.fcE-84",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84",
        10,
        Up,
        "1.0e-100",
        "0xe.0E-84",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84",
        10,
        Nearest,
        "1.0e-100",
        "0xe.0E-84",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        1,
        Nearest,
        "4.0",
        "0x4.0",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        64,
        Nearest,
        "3.54917081446780319459",
        "0x3.8c9675600e90e42c",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Floor,
        "3.547",
        "0x3.8c",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Ceiling,
        "3.5508",
        "0x3.8d",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Down,
        "3.547",
        "0x3.8c",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Up,
        "3.5508",
        "0x3.8d",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Nearest,
        "3.5508",
        "0x3.8d",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        1,
        Nearest,
        "-0.12",
        "-0x0.2",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        64,
        Nearest,
        "-0.111832559158962971823",
        "-0x0.1ca10f00373a296e6",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Floor,
        "-0.11194",
        "-0x0.1ca8",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Ceiling,
        "-0.1118",
        "-0x0.1ca",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Down,
        "-0.1118",
        "-0x0.1ca",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Up,
        "-0.11194",
        "-0x0.1ca8",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Nearest,
        "-0.1118",
        "-0x0.1ca",
        Greater,
    );
    test("-0.25", "-0x0.4", 1, Nearest, "-0.25", "-0x0.4", Greater);
    test(
        "-0.25",
        "-0x0.4",
        64,
        Nearest,
        "-0.357402956181388903057",
        "-0x0.5b7ec2984af40c618",
        Greater,
    );
    test("-0.25", "-0x0.4", 10, Floor, "-0.3574", "-0x0.5b8", Less);
    test(
        "-0.25", "-0x0.4", 10, Ceiling, "-0.35693", "-0x0.5b6", Greater,
    );
    test("-0.25", "-0x0.4", 10, Down, "-0.35693", "-0x0.5b6", Greater);
    test("-0.25", "-0x0.4", 10, Up, "-0.3574", "-0x0.5b8", Less);
    test("-0.25", "-0x0.4", 10, Nearest, "-0.3574", "-0x0.5b8", Less);
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        1,
        Nearest,
        "-0.50",
        "-0x0.8",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        64,
        Nearest,
        "-0.48940222718021493358",
        "-0x0.7d4976e0541e2cde",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Floor,
        "-0.48975",
        "-0x0.7d6",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Ceiling,
        "-0.4893",
        "-0x0.7d4",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Down,
        "-0.4893",
        "-0x0.7d4",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Up,
        "-0.48975",
        "-0x0.7d6",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Nearest,
        "-0.4893",
        "-0x0.7d4",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        1,
        Nearest,
        "-7.6e-6",
        "-0x0.00008",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        64,
        Nearest,
        "-0.0000100001000015000274852",
        "-0x0.0000a7c61a3b06c5829f",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        10,
        Floor,
        "-0.0000100",
        "-0x0.0000a8",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        10,
        Ceiling,
        "-9.9987e-6",
        "-0x0.0000a7c",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        10,
        Down,
        "-9.9987e-6",
        "-0x0.0000a7c",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        10,
        Up,
        "-0.0000100",
        "-0x0.0000a8",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        10,
        Nearest,
        "-9.9987e-6",
        "-0x0.0000a7c",
        Greater,
    );
    test(
        "-1.000000000000000e-100",
        "-0xd.ff9772470298E-84",
        1,
        Nearest,
        "-1.1e-100",
        "-0x1.0E-83",
        Less,
    );
    test(
        "-1.000000000000000e-100",
        "-0xd.ff9772470298E-84",
        64,
        Nearest,
        "-1.000000000000000e-100",
        "-0xd.ff9772470298E-84",
        Greater,
    );
    test(
        "-1.000000000000000e-100",
        "-0xd.ff9772470298E-84",
        10,
        Floor,
        "-1.0e-100",
        "-0xe.0E-84",
        Less,
    );
    test(
        "-1.000000000000000e-100",
        "-0xd.ff9772470298E-84",
        10,
        Ceiling,
        "-9.9900e-101",
        "-0xd.fcE-84",
        Greater,
    );
    test(
        "-1.000000000000000e-100",
        "-0xd.ff9772470298E-84",
        10,
        Down,
        "-9.9900e-101",
        "-0xd.fcE-84",
        Greater,
    );
    test(
        "-1.000000000000000e-100",
        "-0xd.ff9772470298E-84",
        10,
        Up,
        "-1.0e-100",
        "-0xe.0E-84",
        Less,
    );
    test(
        "-1.000000000000000e-100",
        "-0xd.ff9772470298E-84",
        10,
        Nearest,
        "-1.0e-100",
        "-0xe.0E-84",
        Less,
    );
    test(
        "-0.36787944117144233",
        "-0x0.5e2d58d8b3bce",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.36787944117144228",
        "-0x0.5e2d58d8b3bcdc",
        1,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-0.36787944117144228",
        "-0x0.5e2d58d8b3bcdc",
        64,
        Nearest,
        "-0.999999984695745871525",
        "-0x0.ffffffbe44cb6cf5",
        Less,
    );
    test(
        "-0.36787944117144228",
        "-0x0.5e2d58d8b3bcdc",
        10,
        Floor,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-0.36787944117144228",
        "-0x0.5e2d58d8b3bcdc",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "-0.36787944117144228",
        "-0x0.5e2d58d8b3bcdc",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "-0.36787944117144228",
        "-0x0.5e2d58d8b3bcdc",
        10,
        Up,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-0.36787944117144228",
        "-0x0.5e2d58d8b3bcdc",
        10,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test("-0.38", "-0x0.6", 10, Nearest, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        1,
        Nearest,
        "0.25",
        "0x0.4",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        64,
        Nearest,
        "0.257627653049736704284",
        "0x0.41f3e2c864d39a618",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Floor,
        "0.25732",
        "0x0.41e",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Ceiling,
        "0.258",
        "0x0.42",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Down,
        "0.25732",
        "0x0.41e",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Up,
        "0.258",
        "0x0.42",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Nearest,
        "0.258",
        "0x0.42",
        Greater,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558",
        1,
        Nearest,
        "-0.50",
        "-0x0.8",
        Greater,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558",
        64,
        Nearest,
        "-0.61906128673594511215",
        "-0x0.9e7accecc024e2f6",
        Greater,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558",
        10,
        Floor,
        "-0.6191",
        "-0x0.9e8",
        Less,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558",
        10,
        Ceiling,
        "-0.61816",
        "-0x0.9e4",
        Greater,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558",
        10,
        Down,
        "-0.61816",
        "-0x0.9e4",
        Greater,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558",
        10,
        Up,
        "-0.6191",
        "-0x0.9e8",
        Less,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558",
        10,
        Nearest,
        "-0.6191",
        "-0x0.9e8",
        Less,
    );
    test(
        "-0.0010000000000000000000000000000000000000000000000000000000000006",
        "-0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608",
        1,
        Nearest,
        "-0.00098",
        "-0x0.004",
        Greater,
    );
    test(
        "-0.0010000000000000000000000000000000000000000000000000000000000006",
        "-0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608",
        64,
        Nearest,
        "-0.0010010015026718858234",
        "-0x0.00419a04b79be695b6",
        Less,
    );
    test(
        "-0.0010000000000000000000000000000000000000000000000000000000000006",
        "-0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608",
        10,
        Floor,
        "-0.0010014",
        "-0x0.0041a",
        Less,
    );
    test(
        "-0.0010000000000000000000000000000000000000000000000000000000000006",
        "-0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608",
        10,
        Ceiling,
        "-0.0009995",
        "-0x0.00418",
        Greater,
    );
    test(
        "-0.0010000000000000000000000000000000000000000000000000000000000006",
        "-0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608",
        10,
        Down,
        "-0.0009995",
        "-0x0.00418",
        Greater,
    );
    test(
        "-0.0010000000000000000000000000000000000000000000000000000000000006",
        "-0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608",
        10,
        Up,
        "-0.0010014",
        "-0x0.0041a",
        Less,
    );
    test(
        "-0.0010000000000000000000000000000000000000000000000000000000000006",
        "-0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608",
        10,
        Nearest,
        "-0.0010014",
        "-0x0.0041a",
        Less,
    );
    test(
        "9.6e301029995663",
        "0x1.0E+250000000000",
        1,
        Nearest,
        "5.5e11",
        "0x8.0E+9",
        Less,
    );
    test(
        "9.6e301029995663",
        "0x1.0E+250000000000",
        64,
        Nearest,
        "693147180532.680801213",
        "0xa162ca9df4.ae48fd",
        Less,
    );
    test(
        "9.6e301029995663",
        "0x1.0E+250000000000",
        10,
        Floor,
        "6.9256e11",
        "0xa.14E+9",
        Less,
    );
    test(
        "9.6e301029995663",
        "0x1.0E+250000000000",
        10,
        Ceiling,
        "6.936e11",
        "0xa.18E+9",
        Greater,
    );
    test(
        "9.6e301029995663",
        "0x1.0E+250000000000",
        10,
        Down,
        "6.9256e11",
        "0xa.14E+9",
        Less,
    );
    test(
        "9.6e301029995663",
        "0x1.0E+250000000000",
        10,
        Up,
        "6.936e11",
        "0xa.18E+9",
        Greater,
    );
    test(
        "9.6e301029995663",
        "0x1.0E+250000000000",
        10,
        Nearest,
        "6.936e11",
        "0xa.18E+9",
        Greater,
    );
    test(
        "1.0e-301029995664",
        "0x1.0E-250000000000",
        1,
        Nearest,
        "1.0e-301029995664",
        "0x1.0E-250000000000",
        Greater,
    );
    test(
        "1.0e-301029995664",
        "0x1.0E-250000000000",
        64,
        Nearest,
        "1.0e-301029995664",
        "0x1.0E-250000000000",
        Greater,
    );
    test(
        "1.0e-301029995664",
        "0x1.0E-250000000000",
        10,
        Floor,
        "1.0432e-301029995664",
        "0xf.fcE-250000000001",
        Less,
    );
    test(
        "1.0e-301029995664",
        "0x1.0E-250000000000",
        10,
        Ceiling,
        "1.0e-301029995664",
        "0x1.0E-250000000000",
        Greater,
    );
    test(
        "1.0e-301029995664",
        "0x1.0E-250000000000",
        10,
        Down,
        "1.0432e-301029995664",
        "0xf.fcE-250000000001",
        Less,
    );
    test(
        "1.0e-301029995664",
        "0x1.0E-250000000000",
        10,
        Up,
        "1.0e-301029995664",
        "0x1.0E-250000000000",
        Greater,
    );
    test(
        "1.0e-301029995664",
        "0x1.0E-250000000000",
        10,
        Nearest,
        "1.0e-301029995664",
        "0x1.0E-250000000000",
        Greater,
    );
    test(
        "-1.0e-301029995664",
        "-0x1.0E-250000000000",
        1,
        Nearest,
        "-1.0e-301029995664",
        "-0x1.0E-250000000000",
        Greater,
    );
    test(
        "-1.0e-301029995664",
        "-0x1.0E-250000000000",
        64,
        Nearest,
        "-1.0e-301029995664",
        "-0x1.0E-250000000000",
        Greater,
    );
    test(
        "-1.0e-301029995664",
        "-0x1.0E-250000000000",
        10,
        Floor,
        "-1.0463e-301029995664",
        "-0x1.008E-250000000000",
        Less,
    );
    test(
        "-1.0e-301029995664",
        "-0x1.0E-250000000000",
        10,
        Ceiling,
        "-1.0e-301029995664",
        "-0x1.0E-250000000000",
        Greater,
    );
    test(
        "-1.0e-301029995664",
        "-0x1.0E-250000000000",
        10,
        Down,
        "-1.0e-301029995664",
        "-0x1.0E-250000000000",
        Greater,
    );
    test(
        "-1.0e-301029995664",
        "-0x1.0E-250000000000",
        10,
        Up,
        "-1.0463e-301029995664",
        "-0x1.008E-250000000000",
        Less,
    );
    test(
        "-1.0e-301029995664",
        "-0x1.0E-250000000000",
        10,
        Nearest,
        "-1.0e-301029995664",
        "-0x1.0E-250000000000",
        Greater,
    );
    test(
        "2.6e355393490465494856466",
        "0x3.0E+295147905179352825856",
        1,
        Nearest,
        "5.9e20",
        "0x2.0E+17",
        Less,
    );
    test(
        "2.6e355393490465494856466",
        "0x3.0E+295147905179352825856",
        64,
        Nearest,
        "818323753292969962176.0",
        "0x2.c5c85fdf473de6acE+17",
        Less,
    );
    test(
        "2.6e355393490465494856466",
        "0x3.0E+295147905179352825856",
        10,
        Floor,
        "8.1742e20",
        "0x2.c5E+17",
        Less,
    );
    test(
        "2.6e355393490465494856466",
        "0x3.0E+295147905179352825856",
        10,
        Ceiling,
        "8.186e20",
        "0x2.c6E+17",
        Greater,
    );
    test(
        "2.6e355393490465494856466",
        "0x3.0E+295147905179352825856",
        10,
        Down,
        "8.1742e20",
        "0x2.c5E+17",
        Less,
    );
    test(
        "2.6e355393490465494856466",
        "0x3.0E+295147905179352825856",
        10,
        Up,
        "8.186e20",
        "0x2.c6E+17",
        Greater,
    );
    test(
        "2.6e355393490465494856466",
        "0x3.0E+295147905179352825856",
        10,
        Nearest,
        "8.186e20",
        "0x2.c6E+17",
        Greater,
    );
    test(
        "3.4e-355393490465494856466",
        "0x3.0E-295147905179352825856",
        1,
        Nearest,
        "2.3e-355393490465494856466",
        "0x2.0E-295147905179352825856",
        Less,
    );
    test(
        "3.4e-355393490465494856466",
        "0x3.0E-295147905179352825856",
        64,
        Nearest,
        "3.4e-355393490465494856466",
        "0x3.0E-295147905179352825856",
        Greater,
    );
    test(
        "3.4e-355393490465494856466",
        "0x3.0E-295147905179352825856",
        10,
        Floor,
        "3.4237e-355393490465494856466",
        "0x2.ffE-295147905179352825856",
        Less,
    );
    test(
        "3.4e-355393490465494856466",
        "0x3.0E-295147905179352825856",
        10,
        Ceiling,
        "3.4e-355393490465494856466",
        "0x3.0E-295147905179352825856",
        Greater,
    );
    test(
        "3.4e-355393490465494856466",
        "0x3.0E-295147905179352825856",
        10,
        Down,
        "3.4237e-355393490465494856466",
        "0x2.ffE-295147905179352825856",
        Less,
    );
    test(
        "3.4e-355393490465494856466",
        "0x3.0E-295147905179352825856",
        10,
        Up,
        "3.4e-355393490465494856466",
        "0x3.0E-295147905179352825856",
        Greater,
    );
    test(
        "3.4e-355393490465494856466",
        "0x3.0E-295147905179352825856",
        10,
        Nearest,
        "3.4e-355393490465494856466",
        "0x3.0E-295147905179352825856",
        Greater,
    );
    test(
        "-3.4e-355393490465494856466",
        "-0x3.0E-295147905179352825856",
        1,
        Nearest,
        "-4.6e-355393490465494856466",
        "-0x4.0E-295147905179352825856",
        Less,
    );
    test(
        "-3.4e-355393490465494856466",
        "-0x3.0E-295147905179352825856",
        64,
        Nearest,
        "-3.4e-355393490465494856466",
        "-0x3.0E-295147905179352825856",
        Greater,
    );
    test(
        "-3.4e-355393490465494856466",
        "-0x3.0E-295147905179352825856",
        10,
        Floor,
        "-3.4326e-355393490465494856466",
        "-0x3.01E-295147905179352825856",
        Less,
    );
    test(
        "-3.4e-355393490465494856466",
        "-0x3.0E-295147905179352825856",
        10,
        Ceiling,
        "-3.4e-355393490465494856466",
        "-0x3.0E-295147905179352825856",
        Greater,
    );
    test(
        "-3.4e-355393490465494856466",
        "-0x3.0E-295147905179352825856",
        10,
        Down,
        "-3.4e-355393490465494856466",
        "-0x3.0E-295147905179352825856",
        Greater,
    );
    test(
        "-3.4e-355393490465494856466",
        "-0x3.0E-295147905179352825856",
        10,
        Up,
        "-3.4326e-355393490465494856466",
        "-0x3.01E-295147905179352825856",
        Less,
    );
    test(
        "-3.4e-355393490465494856466",
        "-0x3.0E-295147905179352825856",
        10,
        Nearest,
        "-3.4e-355393490465494856466",
        "-0x3.0E-295147905179352825856",
        Greater,
    );
    // - apriori_w0_pos x < 3
    // - deriv_lower_exp c > 0
    // - lambert_bracket mean-value rounds reach the target
    // - lambert_w0_tiny not tiny
    // - mul_bracket_any y >= 0
    // - mvt_round dist > 0
    // - rounding_possible yes
    // - seed_float W_0 x >= 1
    test("1.0", "0x1.0", 1, Down, "0.50", "0x0.8", Less);
    // - lambert_w_neg below -1/e
    // - q_at q < 0
    test("-1.0", "-0x1.0", 1, Down, "NaN", "NaN", Equal);
    // - seed_float W_0 0 <= x < 1
    test("0.50", "0x0.8", 1, Down, "0.25", "0x0.4", Less);
    // - apriori_w0_pos x >= 3
    test("4.0", "0x4.0", 1, Down, "1.0", "0x1.0", Less);
    // - mul_bracket_any y < 0
    // - q_at q > 0
    // - q_tight recomputed
    // - seed_float W_0 x < 0
    test("-0.25", "-0x0.4", 1, Down, "-0.25", "-0x0.4", Greater);
    // - q_tight c == 0
    test("-0.12", "-0x0.2", 1, Down, "-0.12", "-0x0.2", Greater);
    // - halley_refine doubling step
    test(
        "1.04226364758062811487679885e63",
        "0x2.889a2dba3978ccd56c826E+52",
        68,
        Down,
        "140.161460741571157303",
        "0x8c.29557dbca2d2bfc",
        Less,
    );
    // - lambert_w0_tiny applies
    test(
        "-5.7963690926625326932e-77",
        "-0x6.b634633de2c9d88E-64",
        49,
        Down,
        "-5.796369092662527e-77",
        "-0x6.b634633de2c8E-64",
        Greater,
    );
    // - rounding_possible no
    // - ziv_loop_certified retry
    test(
        "-7.9e-31",
        "-0x1.0E-25",
        108,
        Down,
        "-7.8886090522101180541172856528341e-31",
        "-0x1.0000000000000000000000001E-25",
        Greater,
    );
    // - log_seed Newton
    // - seed_float x outside the seed range
    test(
        "2.3281685280997921493337341223879518220630854242143616671023085e4835",
        "0x6.b014710df6d8d0fb1901206ed24e1e002b4411ac77d2348fd2E+4015",
        84,
        Ceiling,
        "11124.527098947084218027227",
        "0x2b74.86eff4e37b95918b48",
        Greater,
    );
    test(
        "8.0125492416043474e10778525511745053023",
        "0x4.f38a550e9d4aE+8951371679731519077",
        13,
        Up,
        "2.4819e19",
        "0x1.587E+16",
        Greater,
    );
    test(
        "3.10569581155512148431517955643542829240984581403e126798795034194091995639",
        "0x2.5612de736f2a6162903822483d6e28914a04384E+105304119905488380329451",
        3,
        Floor,
        "2.6e23",
        "0x3.8E+19",
        Less,
    );
    // - log_seed tiny x
    test(
        "-5.23251650861762432115354588328405238488825239041728142627591170763504415653525497179673\
        97863042145123729598440408081506482147992687884152425e-436",
        "-0x4.133d7aa33db8f1de4c4c3c887864ea740dd2897391893e3fb30a336df838d823026e931c42d5eb6aa70b\
        58c9816c3f6fa0430543a8dce4e0fbE-362",
        1519,
        Down,
        "-5.23251650861762432115354588328405238488825239041728142627591170763504415653525497179673\
        978630421451237295984404080815064821479926878841524247788863533770842947706580354917465273\
        308019799363188503081042417322134473703434248921087411489287054006671026861750538598497896\
        575653820376464406687519215309131697965712528741728046193308899292723127715242450054792012\
        388585310355955162183206649252009696186355787712021959576522458432778949044518621949990784\
        641358988804e-436",
        "-0x4.133d7aa33db8f1de4c4c3c887864ea740dd2897391893e3fb30a336df838d823026e931c42d5eb6aa70b\
        58c9816c3f6fa0430543a8dce4e0fb000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000109b5e0411d57c6be89E-362",
        Greater,
    );
    // - halley_candidate series seed
    test(
        "-0.36787944117144228",
        "-0x0.5e2d58d8b3bcdc",
        1,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    // - q_at undecided
    // - q_encl retry at a higher precision
    test(
        "-0.367879441171442321595523770161460867445811131031767834507837",
        "-0x0.5e2d58d8b3bcdf1abadec7829054f90dda9805aab56c77333",
        10,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-0.3678794411714423215955237701614608674458111310317678345078373",
        "-0x0.5e2d58d8b3bcdf1abadec7829054f90dda9805aab56c773331",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
}

#[test]
fn test_lambert_w0_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r = lambert_w0_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test(
        "1",
        53,
        Nearest,
        "0.56714329040978384",
        "0x0.91304d7c74b2b8",
        Less,
    );
    test("1", 10, Floor, "0.5664", "0x0.91", Less);
    test("1", 10, Ceiling, "0.56738", "0x0.914", Greater);
    test("1", 10, Down, "0.5664", "0x0.91", Less);
    test("1", 10, Up, "0.56738", "0x0.914", Greater);
    test("1", 10, Nearest, "0.56738", "0x0.914", Greater);
    test(
        "1/3",
        53,
        Nearest,
        "0.25762765304973673",
        "0x0.41f3e2c864d39c",
        Greater,
    );
    test("1/3", 10, Floor, "0.25732", "0x0.41e", Less);
    test("1/3", 10, Ceiling, "0.258", "0x0.42", Greater);
    test("1/3", 10, Down, "0.25732", "0x0.41e", Less);
    test("1/3", 10, Up, "0.258", "0x0.42", Greater);
    test("1/3", 10, Nearest, "0.258", "0x0.42", Greater);
    test(
        "-1/3",
        53,
        Nearest,
        "-0.61906128673594507",
        "-0x0.9e7accecc024e",
        Greater,
    );
    test("-1/3", 10, Floor, "-0.6191", "-0x0.9e8", Less);
    test("-1/3", 10, Ceiling, "-0.61816", "-0x0.9e4", Greater);
    test("-1/3", 10, Down, "-0.61816", "-0x0.9e4", Greater);
    test("-1/3", 10, Up, "-0.6191", "-0x0.9e8", Less);
    test("-1/3", 10, Nearest, "-0.6191", "-0x0.9e8", Less);
    test(
        "22/7",
        53,
        Nearest,
        "1.0738665609038538",
        "0x1.12e8eb3f599c8",
        Less,
    );
    test("22/7", 10, Floor, "1.0723", "0x1.128", Less);
    test("22/7", 10, Ceiling, "1.074", "0x1.13", Greater);
    test("22/7", 10, Down, "1.0723", "0x1.128", Less);
    test("22/7", 10, Up, "1.074", "0x1.13", Greater);
    test("22/7", 10, Nearest, "1.074", "0x1.13", Greater);
    test(
        "-1/4",
        53,
        Nearest,
        "-0.35740295618138890",
        "-0x0.5b7ec2984af40c",
        Greater,
    );
    test("-1/4", 10, Floor, "-0.3574", "-0x0.5b8", Less);
    test("-1/4", 10, Ceiling, "-0.35693", "-0x0.5b6", Greater);
    test("-1/4", 10, Down, "-0.35693", "-0x0.5b6", Greater);
    test("-1/4", 10, Up, "-0.3574", "-0x0.5b8", Less);
    test("-1/4", 10, Nearest, "-0.3574", "-0x0.5b8", Less);
    test(
        "-1/1000",
        53,
        Nearest,
        "-0.0010010015026718857",
        "-0x0.00419a04b79be694",
        Greater,
    );
    test("-1/1000", 10, Floor, "-0.0010014", "-0x0.0041a", Less);
    test("-1/1000", 10, Ceiling, "-0.0009995", "-0x0.00418", Greater);
    test("-1/1000", 10, Down, "-0.0009995", "-0x0.00418", Greater);
    test("-1/1000", 10, Up, "-0.0010014", "-0x0.0041a", Less);
    test("-1/1000", 10, Nearest, "-0.0010014", "-0x0.0041a", Less);
    test(
        "1000000000000000000000000000000",
        53,
        Nearest,
        "64.904633770046118",
        "0x40.e7961429348c",
        Less,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Floor,
        "64.875",
        "0x40.e",
        Less,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Ceiling,
        "65.00",
        "0x41.0",
        Greater,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Down,
        "64.875",
        "0x40.e",
        Less,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Up,
        "65.00",
        "0x41.0",
        Greater,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Nearest,
        "64.875",
        "0x40.e",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        53,
        Nearest,
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Floor,
        "9.9994e-31",
        "0x1.448E-25",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Ceiling,
        "1.001e-30",
        "0x1.45E-25",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Down,
        "9.9994e-31",
        "0x1.448E-25",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Up,
        "1.001e-30",
        "0x1.45E-25",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Nearest,
        "9.9994e-31",
        "0x1.448E-25",
        Less,
    );
    test(
        "-1/1000000000000000000000000000000",
        53,
        Nearest,
        "-1.000000000000000e-30",
        "-0x1.4484bfeebc2aE-25",
        Less,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Floor,
        "-1.001e-30",
        "-0x1.45E-25",
        Less,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Ceiling,
        "-9.9994e-31",
        "-0x1.448E-25",
        Greater,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Down,
        "-9.9994e-31",
        "-0x1.448E-25",
        Greater,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Up,
        "-1.001e-30",
        "-0x1.45E-25",
        Less,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Nearest,
        "-9.9994e-31",
        "-0x1.448E-25",
        Greater,
    );
    test(
        "-11/30",
        53,
        Nearest,
        "-0.92091999699760863",
        "-0x0.ebc169b5564f48",
        Greater,
    );
    test("-11/30", 10, Floor, "-0.922", "-0x0.ec", Less);
    test("-11/30", 10, Ceiling, "-0.92090", "-0x0.ebc", Greater);
    test("-11/30", 10, Down, "-0.92090", "-0x0.ebc", Greater);
    test("-11/30", 10, Up, "-0.922", "-0x0.ec", Less);
    test("-11/30", 10, Nearest, "-0.92090", "-0x0.ebc", Greater);
    test("-3/8", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "-4598493014643029/12500000000000000",
        53,
        Nearest,
        "-0.99999999705480624",
        "-0x0.fffffff359b9a8",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Floor,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Up,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-36787944117144233/100000000000000000",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    // - lambert_w0_rational_helper tiny guard
    test(
        "-843157938172414828993517099075/777067556890291628367784762729407562656962735620855808500\
        7249638955617140820833992704",
        31,
        Nearest,
        "-1.0850510111e-55",
        "-0x2.a918f998E-46",
        Less,
    );
}

#[test]
#[should_panic]
fn lambert_w0_prec_round_fail_1() {
    BigExponentFloat::ONE.lambert_w0_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn lambert_w0_prec_round_fail_2() {
    BigExponentFloat::from(0.5).lambert_w0_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn lambert_w0_rational_prec_round_fail_1() {
    BigExponentFloat::lambert_w0_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn lambert_w0_rational_prec_round_fail_2() {
    BigExponentFloat::lambert_w0_rational_prec_round(Rational::from_unsigneds(1u32, 3), 10, Exact);
}

fn lambert_w0_prec_round_properties_helper(x: &BigExponentFloat, prec: u64, rm: RoundingMode) {
    let r = lambert_w0_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if *x >= 0u32 {
        assert_eq!(r.0.partial_cmp(&0u32), x.partial_cmp(&0u32));
    } else if !r.0.is_nan() {
        // W_0 maps (-1/e, 0) to (-1, 0).
        assert!(r.0 >= -1i32 && r.0 <= 0u32);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else {
        if r.0.is_finite() && !r.0.is_zero() {
            // The result is correctly rounded, by a check that is independent of the algorithm.
            assert_lambert_w_correctly_rounded(
                true,
                &|f| f.partial_cmp(x).unwrap(),
                Some(x),
                &r.0,
                r.1,
                prec,
                rm,
            );
        }
        // The result lies between the results rounded down and up.
        let floor = lambert_w0_prec_round_all(x, prec, Floor);
        let ceiling = lambert_w0_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0 || r.0.is_nan());
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(&lambert_w0_prec_round_all(x, prec, Exact), &r));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = lambert_w0_prec_round_all(x, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // A `BigExponentFloat` that is not too large or too small gives the same result as the equal
    // `Rational`.
    if x.is_finite() && exponent_is_moderate(x) {
        let q = Rational::exact_from(x);
        assert!(same_pair(
            &lambert_w0_rational_prec_round_all(&q, prec, rm),
            &r
        ));
    }
}

#[test]
fn lambert_w0_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_24().test_properties(
        |(x, prec, rm)| {
            lambert_w0_prec_round_properties_helper(&x, prec, rm);
        },
    );

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 128);
    config.insert("mean_small_n", 512);
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_24().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            lambert_w0_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn lambert_w0_prec_round_float_agreement_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        for rm in exhaustive_rounding_modes() {
            if rm == Exact {
                continue;
            }
            let x = &x;
            let (f, fo) = x.lambert_w0_prec_round_ref(prec, rm);
            if float_result_comparable(&f, fo) {
                let r = lambert_w0_prec_round_all(&float_to_big_exponent_float(x), prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        }
    });
}

#[test]
fn lambert_w0_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        let r = lambert_w0_rational_prec_round_all(&x, prec, rm);
        if !r.0.is_finite() || r.0.is_zero() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        if x >= 0u32 {
            assert_eq!(r.0.partial_cmp(&0u32), x.partial_cmp(&0u32));
        } else if !r.0.is_nan() {
            assert!(r.0 >= -1i32 && r.0 <= 0u32);
        }
        if rm == Exact {
            assert_eq!(r.1, Equal);
        } else {
            if r.0.is_finite() && !r.0.is_zero() {
                assert_lambert_w_correctly_rounded(
                    true,
                    &|f| f.partial_cmp(&x).unwrap(),
                    None,
                    &r.0,
                    r.1,
                    prec,
                    rm,
                );
            }
            if rm != Nearest {
                let (y, o) = lambert_w0_rational_prec_round_all(&x, prec + 16, rm);
                let (z, oz) = y.round_to_prec_round(prec, rm);
                assert!(same(&z, &r.0));
                assert_eq!(if o == Equal { oz } else { o }, r.1);
            }
        }
    });
}

#[test]
fn lambert_w0_rational_prec_round_float_agreement_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let x = &x;
        let (f, fo) = Float::lambert_w0_rational_prec_round_ref(x, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = lambert_w0_rational_prec_round_all(x, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
