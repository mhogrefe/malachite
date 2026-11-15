// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, FromStringBase};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_14, float_unsigned_pair_gen_var_1,
    rational_unsigned_rounding_mode_triple_gen_var_24,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
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

// Whether the exponent of `x`, if it has one, is at most 256 in absolute value, so that exact
// conversions and sums involving x stay small.
fn exponent_is_moderate(x: &BigExponentFloat) -> bool {
    x.get_exponent()
        .is_none_or(|e| e.unsigned_abs_ref() <= &256u32)
}

fn power_of_10_x_minus_1_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().power_of_10_x_minus_1_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.power_of_10_x_minus_1_prec_round_ref(prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.power_of_10_x_minus_1_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().power_of_10_x_minus_1_prec(prec), &r));
        assert!(same_pair(&x.power_of_10_x_minus_1_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.power_of_10_x_minus_1_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn power_of_10_x_minus_1_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::power_of_10_x_minus_1_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::power_of_10_x_minus_1_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::power_of_10_x_minus_1_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::power_of_10_x_minus_1_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_power_of_10_x_minus_1_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &power_of_10_x_minus_1_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &power_of_10_x_minus_1_prec_round_all(&inf, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &power_of_10_x_minus_1_prec_round_all(&ninf, 10, rm),
            &(BigExponentFloat::NEGATIVE_ONE, Equal)
        ));
        assert!(same_pair(
            &power_of_10_x_minus_1_prec_round_all(&zero, 10, rm),
            &(zero.clone(), Equal)
        ));
    }
    // The expected values agree with MPFR's wherever MPFR's exponent range allows.
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let r = power_of_10_x_minus_1_prec_round_all(&hex(x_hex), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        Nearest,
        "1.0e3",
        "0x4.0E+2",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Floor,
        "1384.0",
        "0x568.0",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Ceiling,
        "1386.0",
        "0x56a.0",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Down,
        "1384.0",
        "0x568.0",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Up,
        "1386.0",
        "0x56a.0",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Nearest,
        "1384.0",
        "0x568.0",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        100,
        Nearest,
        "1384.455731367010698462705619402",
        "0x568.74aacf951283c59a6deee4",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        1,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Floor,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Up,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Nearest,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        100,
        Nearest,
        "-0.99927821584092527205285444990127",
        "-0x0.ffd0b278a45a520df5dec92cf",
        Less,
    );
    test("0.50", "0x0.8", 1, Nearest, "2.0", "0x2.0", Less);
    test("0.50", "0x0.8", 10, Floor, "2.1602", "0x2.29", Less);
    test("0.50", "0x0.8", 10, Ceiling, "2.164", "0x2.2a", Greater);
    test("0.50", "0x0.8", 10, Down, "2.1602", "0x2.29", Less);
    test("0.50", "0x0.8", 10, Up, "2.164", "0x2.2a", Greater);
    test("0.50", "0x0.8", 10, Nearest, "2.164", "0x2.2a", Greater);
    test(
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "2.162277660168379331998893544431",
        "0x2.298b075b4b6a5240945790618",
        Less,
    );
    test("-0.50", "-0x0.8", 1, Nearest, "-0.50", "-0x0.8", Greater);
    test("-0.50", "-0x0.8", 10, Floor, "-0.68457", "-0x0.af4", Less);
    test(
        "-0.50", "-0x0.8", 10, Ceiling, "-0.6836", "-0x0.af", Greater,
    );
    test("-0.50", "-0x0.8", 10, Down, "-0.6836", "-0x0.af", Greater);
    test("-0.50", "-0x0.8", 10, Up, "-0.68457", "-0x0.af4", Less);
    test(
        "-0.50", "-0x0.8", 10, Nearest, "-0.6836", "-0x0.af", Greater,
    );
    test(
        "-0.50",
        "-0x0.8",
        100,
        Nearest,
        "-0.68377223398316206680011064555702",
        "-0x0.af0bb276dedbc493245da4c31",
        Less,
    );
    test("3.0", "0x3", 1, Nearest, "1.0e3", "0x4.0E+2", Greater);
    test("3.0", "0x3", 10, Floor, "999.00", "0x3e7.0", Equal);
    test("3.0", "0x3", 10, Ceiling, "999.00", "0x3e7.0", Equal);
    test("3.0", "0x3", 10, Down, "999.00", "0x3e7.0", Equal);
    test("3.0", "0x3", 10, Up, "999.00", "0x3e7.0", Equal);
    test("3.0", "0x3", 10, Nearest, "999.00", "0x3e7.0", Equal);
    test("3.0", "0x3", 100, Nearest, "999.00", "0x3e7.0", Equal);
    test("-3.0", "-0x3", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-3.0", "-0x3", 10, Floor, "-0.99902", "-0x0.ffc", Less);
    test("-3.0", "-0x3", 10, Ceiling, "-0.9980", "-0x0.ff8", Greater);
    test("-3.0", "-0x3", 10, Down, "-0.9980", "-0x0.ff8", Greater);
    test("-3.0", "-0x3", 10, Up, "-0.99902", "-0x0.ffc", Less);
    test("-3.0", "-0x3", 10, Nearest, "-0.99902", "-0x0.ffc", Less);
    test(
        "-3.0",
        "-0x3",
        100,
        Nearest,
        "-0.99900000000000000000000000000030",
        "-0x0.ffbe76c8b4395810624dd2f1b",
        Less,
    );
    test("100.0", "0x64", 1, Nearest, "8.7e99", "0x1.0E+83", Less);
    test("100.0", "0x64", 10, Floor, "9.9964e99", "0x1.248E+83", Less);
    test(
        "100.0",
        "0x64",
        10,
        Ceiling,
        "1.001e100",
        "0x1.25E+83",
        Greater,
    );
    test("100.0", "0x64", 10, Down, "9.9964e99", "0x1.248E+83", Less);
    test("100.0", "0x64", 10, Up, "1.001e100", "0x1.25E+83", Greater);
    test(
        "100.0",
        "0x64",
        10,
        Nearest,
        "9.9964e99",
        "0x1.248E+83",
        Less,
    );
    test(
        "100.0",
        "0x64",
        100,
        Nearest,
        "9.9999999999999999999999999999997e99",
        "0x1.249ad2594c37ceb0b2784c4ceE+83",
        Less,
    );
    test("-100.0", "-0x64", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-100.0", "-0x64", 10, Floor, "-1.0", "-0x1.0", Less);
    test(
        "-100.0", "-0x64", 10, Ceiling, "-0.99902", "-0x0.ffc", Greater,
    );
    test("-100.0", "-0x64", 10, Down, "-0.99902", "-0x0.ffc", Greater);
    test("-100.0", "-0x64", 10, Up, "-1.0", "-0x1.0", Less);
    test("-100.0", "-0x64", 10, Nearest, "-1.0", "-0x1.0", Less);
    test("-100.0", "-0x64", 100, Nearest, "-1.0", "-0x1.0", Less);
    test(
        "1.00000000e10",
        "0x2540be400.0",
        1,
        Nearest,
        "1.1e10000000000",
        "0x2.0E+8304820237",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Floor,
        "9.999e9999999999",
        "0x1.d5E+8304820237",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Ceiling,
        "1.0009e10000000000",
        "0x1.d58E+8304820237",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Down,
        "9.999e9999999999",
        "0x1.d5E+8304820237",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Up,
        "1.0009e10000000000",
        "0x1.d58E+8304820237",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Nearest,
        "9.999e9999999999",
        "0x1.d5E+8304820237",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        100,
        Nearest,
        "1.00000000000000000000000000000e10000000000",
        "0x1.d50eed9ae8ffa28314081c82E+8304820237",
        Less,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        1,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Floor,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Up,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        100,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        1,
        Nearest,
        "1.1e1208925819614629174706176",
        "0x1.0E+1003991161203142040635692",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Floor,
        "9.99e1208925819614629174706175",
        "0xe.8E+1003991161203142040635691",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Ceiling,
        "1.0001e1208925819614629174706176",
        "0xe.84E+1003991161203142040635691",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Down,
        "9.99e1208925819614629174706175",
        "0xe.8E+1003991161203142040635691",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Up,
        "1.0001e1208925819614629174706176",
        "0xe.84E+1003991161203142040635691",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Nearest,
        "1.0001e1208925819614629174706176",
        "0xe.84E+1003991161203142040635691",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        100,
        Nearest,
        "9.9999999999999999999999999999976e1208925819614629174706175",
        "0xe.837a9d8d97e34b183c65d829E+1003991161203142040635691",
        Less,
    );
    test("-1.2e24", "-0x1.0E+20", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-1.2e24", "-0x1.0E+20", 10, Floor, "-1.0", "-0x1.0", Less);
    test(
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test("-1.2e24", "-0x1.0E+20", 10, Up, "-1.0", "-0x1.0", Less);
    test("-1.2e24", "-0x1.0E+20", 10, Nearest, "-1.0", "-0x1.0", Less);
    test(
        "-1.2e24",
        "-0x1.0E+20",
        100,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        1,
        Nearest,
        "7.7e-121",
        "0x2.0E-100",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Floor,
        "8.9100e-121",
        "0x2.4dE-100",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Ceiling,
        "8.925e-121",
        "0x2.4eE-100",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Down,
        "8.9100e-121",
        "0x2.4dE-100",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Up,
        "8.925e-121",
        "0x2.4eE-100",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Nearest,
        "8.9100e-121",
        "0x2.4dE-100",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        100,
        Nearest,
        "8.916972414381306960272920066386e-121",
        "0x2.4d763776aaa2b05ba95b58aeE-100",
        Less,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        1,
        Nearest,
        "-7.7e-121",
        "-0x2.0E-100",
        Greater,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Floor,
        "-8.925e-121",
        "-0x2.4eE-100",
        Less,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Ceiling,
        "-8.9100e-121",
        "-0x2.4dE-100",
        Greater,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Down,
        "-8.9100e-121",
        "-0x2.4dE-100",
        Greater,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Up,
        "-8.925e-121",
        "-0x2.4eE-100",
        Less,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Nearest,
        "-8.9100e-121",
        "-0x2.4dE-100",
        Greater,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        100,
        Nearest,
        "-8.916972414381306960272920066386e-121",
        "-0x2.4d763776aaa2b05ba95b58aeE-100",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        1,
        Nearest,
        "6.4e-361235995",
        "0x4.0E-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Floor,
        "5.514e-361235995",
        "0x3.74E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Ceiling,
        "5.5198e-361235995",
        "0x3.75E-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Down,
        "5.514e-361235995",
        "0x3.74E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Up,
        "5.5198e-361235995",
        "0x3.75E-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Nearest,
        "5.514e-361235995",
        "0x3.74E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        100,
        Nearest,
        "5.514796782014766719755553645272e-361235995",
        "0x3.74315331fff408897e090505E-300000000",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        1,
        Nearest,
        "-6.4e-361235995",
        "-0x4.0E-300000000",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Floor,
        "-5.5198e-361235995",
        "-0x3.75E-300000000",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Ceiling,
        "-5.514e-361235995",
        "-0x3.74E-300000000",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Down,
        "-5.514e-361235995",
        "-0x3.74E-300000000",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Up,
        "-5.5198e-361235995",
        "-0x3.75E-300000000",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Nearest,
        "-5.514e-361235995",
        "-0x3.74E-300000000",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        100,
        Nearest,
        "-5.514796782014766719755553645272e-361235995",
        "-0x3.74315331fff408897e090505E-300000000",
        Greater,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        1,
        Nearest,
        "1.2e1342177280",
        "0x8.0E+1114654103",
        Greater,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Floor,
        "9.9887e1342177279",
        "0x6.baE+1114654103",
        Less,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Ceiling,
        "1.000e1342177280",
        "0x6.bcE+1114654103",
        Greater,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Down,
        "9.9887e1342177279",
        "0x6.baE+1114654103",
        Less,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Up,
        "1.000e1342177280",
        "0x6.bcE+1114654103",
        Greater,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Nearest,
        "1.000e1342177280",
        "0x6.bcE+1114654103",
        Greater,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        100,
        Nearest,
        "1.0000000000000000000000000000e1342177280",
        "0x6.bbf31a9152355ac13838208E+1114654103",
        Less,
    );
    test("-1.3e9", "-0x5.0E+7", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-1.3e9", "-0x5.0E+7", 10, Floor, "-1.0", "-0x1.0", Less);
    test(
        "-1.3e9",
        "-0x5.0E+7",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "-1.3e9",
        "-0x5.0E+7",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test("-1.3e9", "-0x5.0E+7", 10, Up, "-1.0", "-0x1.0", Less);
    test("-1.3e9", "-0x5.0E+7", 10, Nearest, "-1.0", "-0x1.0", Less);
    test("-1.3e9", "-0x5.0E+7", 100, Nearest, "-1.0", "-0x1.0", Less);
    test("3.0", "0x3", 300, Exact, "999.00", "0x3e7.0", Equal);
}

#[test]
fn test_power_of_10_x_minus_1_rational_prec_round() {
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &power_of_10_x_minus_1_rational_prec_round_all(&Rational::ZERO, 10, rm),
            &(BigExponentFloat::ZERO, Equal)
        ));
    }
    // The expected values agree with MPFR's wherever MPFR's exponent range allows.
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r = power_of_10_x_minus_1_rational_prec_round_all(
            &Rational::from_str(x).unwrap(),
            prec,
            rm,
        );
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("1/3", 1, Nearest, "1.0", "0x1.0", Less);
    test("1/3", 10, Floor, "1.1543", "0x1.278", Less);
    test("1/3", 10, Ceiling, "1.16", "0x1.28", Greater);
    test("1/3", 10, Down, "1.1543", "0x1.278", Less);
    test("1/3", 10, Up, "1.16", "0x1.28", Greater);
    test("1/3", 10, Nearest, "1.1543", "0x1.278", Less);
    test(
        "1/3",
        100,
        Nearest,
        "1.1544346900318837217592935665199",
        "0x1.278908270e09d951445ae49be",
        Greater,
    );
    test("-1/3", 1, Nearest, "-0.50", "-0x0.8", Greater);
    test("-1/3", 10, Floor, "-0.53613", "-0x0.894", Less);
    test("-1/3", 10, Ceiling, "-0.5352", "-0x0.89", Greater);
    test("-1/3", 10, Down, "-0.5352", "-0x0.89", Greater);
    test("-1/3", 10, Up, "-0.53613", "-0x0.894", Less);
    test("-1/3", 10, Nearest, "-0.53613", "-0x0.894", Less);
    test(
        "-1/3",
        100,
        Nearest,
        "-0.5358411166387221107589923649082",
        "-0x0.892ce227d0c0361d2af11ebea",
        Less,
    );
    test("3/5", 1, Nearest, "2.0", "0x2.0", Less);
    test("3/5", 10, Floor, "2.9805", "0x2.fb", Less);
    test("3/5", 10, Ceiling, "2.984", "0x2.fc", Greater);
    test("3/5", 10, Down, "2.9805", "0x2.fb", Less);
    test("3/5", 10, Up, "2.984", "0x2.fc", Greater);
    test("3/5", 10, Nearest, "2.9805", "0x2.fb", Less);
    test(
        "3/5",
        100,
        Nearest,
        "2.9810717055349725077025230508791",
        "0x2.fb2783ea4dbbf29ec0f12930c",
        Greater,
    );
    test("3", 1, Nearest, "1.0e3", "0x4.0E+2", Greater);
    test("3", 10, Floor, "999.00", "0x3e7.0", Equal);
    test("3", 10, Ceiling, "999.00", "0x3e7.0", Equal);
    test("3", 10, Down, "999.00", "0x3e7.0", Equal);
    test("3", 10, Up, "999.00", "0x3e7.0", Equal);
    test("3", 10, Nearest, "999.00", "0x3e7.0", Equal);
    test("3", 100, Nearest, "999.00", "0x3e7.0", Equal);
    test("-3", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-3", 10, Floor, "-0.99902", "-0x0.ffc", Less);
    test("-3", 10, Ceiling, "-0.9980", "-0x0.ff8", Greater);
    test("-3", 10, Down, "-0.9980", "-0x0.ff8", Greater);
    test("-3", 10, Up, "-0.99902", "-0x0.ffc", Less);
    test("-3", 10, Nearest, "-0.99902", "-0x0.ffc", Less);
    test(
        "-3",
        100,
        Nearest,
        "-0.99900000000000000000000000000030",
        "-0x0.ffbe76c8b4395810624dd2f1b",
        Less,
    );
    test("100", 1, Nearest, "8.7e99", "0x1.0E+83", Less);
    test("100", 10, Floor, "9.9964e99", "0x1.248E+83", Less);
    test("100", 10, Ceiling, "1.001e100", "0x1.25E+83", Greater);
    test("100", 10, Down, "9.9964e99", "0x1.248E+83", Less);
    test("100", 10, Up, "1.001e100", "0x1.25E+83", Greater);
    test("100", 10, Nearest, "9.9964e99", "0x1.248E+83", Less);
    test(
        "100",
        100,
        Nearest,
        "9.9999999999999999999999999999997e99",
        "0x1.249ad2594c37ceb0b2784c4ceE+83",
        Less,
    );
    test("-100", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-100", 10, Floor, "-1.0", "-0x1.0", Less);
    test("-100", 10, Ceiling, "-0.99902", "-0x0.ffc", Greater);
    test("-100", 10, Down, "-0.99902", "-0x0.ffc", Greater);
    test("-100", 10, Up, "-1.0", "-0x1.0", Less);
    test("-100", 10, Nearest, "-1.0", "-0x1.0", Less);
    test("-100", 100, Nearest, "-1.0", "-0x1.0", Less);
    test(
        "10000000000/3",
        1,
        Nearest,
        "2.8e3333333333",
        "0x4.0E+2768273412",
        Greater,
    );
    test(
        "10000000000/3",
        10,
        Floor,
        "2.1534e3333333333",
        "0x3.15E+2768273412",
        Less,
    );
    test(
        "10000000000/3",
        10,
        Ceiling,
        "2.156e3333333333",
        "0x3.16E+2768273412",
        Greater,
    );
    test(
        "10000000000/3",
        10,
        Down,
        "2.1534e3333333333",
        "0x3.15E+2768273412",
        Less,
    );
    test(
        "10000000000/3",
        10,
        Up,
        "2.156e3333333333",
        "0x3.16E+2768273412",
        Greater,
    );
    test(
        "10000000000/3",
        10,
        Nearest,
        "2.1534e3333333333",
        "0x3.15E+2768273412",
        Less,
    );
    test(
        "10000000000/3",
        100,
        Nearest,
        "2.154434690031883721759293566519e3333333333",
        "0x3.155c60bf26e2cfb7a37b6fb7E+2768273412",
        Less,
    );
    test("-10000000000/3", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-10000000000/3", 10, Floor, "-1.0", "-0x1.0", Less);
    test(
        "-10000000000/3",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test("-10000000000/3", 10, Down, "-0.99902", "-0x0.ffc", Greater);
    test("-10000000000/3", 10, Up, "-1.0", "-0x1.0", Less);
    test("-10000000000/3", 10, Nearest, "-1.0", "-0x1.0", Less);
    test("-10000000000/3", 100, Nearest, "-1.0", "-0x1.0", Less);
    test("1/10000000000", 1, Nearest, "2.3e-10", "0x1.0E-8", Greater);
    test("1/10000000000", 10, Floor, "2.301e-10", "0xf.dE-9", Less);
    test(
        "1/10000000000",
        10,
        Ceiling,
        "2.3033e-10",
        "0xf.d4E-9",
        Greater,
    );
    test("1/10000000000", 10, Down, "2.301e-10", "0xf.dE-9", Less);
    test("1/10000000000", 10, Up, "2.3033e-10", "0xf.d4E-9", Greater);
    test(
        "1/10000000000",
        10,
        Nearest,
        "2.3033e-10",
        "0xf.d4E-9",
        Greater,
    );
    test(
        "1/10000000000",
        100,
        Nearest,
        "2.3025850932591405895622581411361e-10",
        "0xf.d2c0230543963a2d457d7dcdE-9",
        Less,
    );
    test("-1/10000000000", 1, Nearest, "-2.3e-10", "-0x1.0E-8", Less);
    test(
        "-1/10000000000",
        10,
        Floor,
        "-2.3033e-10",
        "-0xf.d4E-9",
        Less,
    );
    test(
        "-1/10000000000",
        10,
        Ceiling,
        "-2.301e-10",
        "-0xf.dE-9",
        Greater,
    );
    test(
        "-1/10000000000",
        10,
        Down,
        "-2.301e-10",
        "-0xf.dE-9",
        Greater,
    );
    test("-1/10000000000", 10, Up, "-2.3033e-10", "-0xf.d4E-9", Less);
    test(
        "-1/10000000000",
        10,
        Nearest,
        "-2.3033e-10",
        "-0xf.d4E-9",
        Less,
    );
    test(
        "-1/10000000000",
        100,
        Nearest,
        "-2.302585092728950778514418340079e-10",
        "-0xf.d2c022f59d95fbf84d8993e8E-9",
        Less,
    );
    test(
        "1208925819614629174706176/3",
        1,
        Nearest,
        "2.2e402975273204876391568725",
        "0x1.0E+334663720401047346878564",
        Greater,
    );
    test(
        "1208925819614629174706176/3",
        10,
        Floor,
        "2.1539e402975273204876391568725",
        "0xf.7cE+334663720401047346878563",
        Less,
    );
    test(
        "1208925819614629174706176/3",
        10,
        Ceiling,
        "2.16e402975273204876391568725",
        "0xf.8E+334663720401047346878563",
        Greater,
    );
    test(
        "1208925819614629174706176/3",
        10,
        Down,
        "2.1539e402975273204876391568725",
        "0xf.7cE+334663720401047346878563",
        Less,
    );
    test(
        "1208925819614629174706176/3",
        10,
        Up,
        "2.16e402975273204876391568725",
        "0xf.8E+334663720401047346878563",
        Greater,
    );
    test(
        "1208925819614629174706176/3",
        10,
        Nearest,
        "2.1539e402975273204876391568725",
        "0xf.7cE+334663720401047346878563",
        Less,
    );
    test(
        "1208925819614629174706176/3",
        100,
        Nearest,
        "2.154434690031883721759293566519e402975273204876391568725",
        "0xf.7d03fec1b9adb755e29ca914E+334663720401047346878563",
        Less,
    );
    test("3", 300, Exact, "999.00", "0x3e7.0", Equal);
}

#[test]
#[should_panic]
fn power_of_10_x_minus_1_prec_round_fail_1() {
    BigExponentFloat::ONE.power_of_10_x_minus_1_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn power_of_10_x_minus_1_prec_round_fail_2() {
    BigExponentFloat::from(0.5).power_of_10_x_minus_1_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn power_of_10_x_minus_1_rational_prec_round_fail_1() {
    BigExponentFloat::power_of_10_x_minus_1_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn power_of_10_x_minus_1_rational_prec_round_fail_2() {
    BigExponentFloat::power_of_10_x_minus_1_rational_prec_round(
        Rational::from_unsigneds(1u32, 3),
        10,
        Exact,
    );
}

fn power_of_10_x_minus_1_prec_round_properties_helper(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) {
    let r = power_of_10_x_minus_1_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if !r.0.is_nan() {
        assert_eq!(r.0.partial_cmp(&0u32), x.partial_cmp(&0u32));
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else {
        // The result lies between the results rounded down and up.
        let floor = power_of_10_x_minus_1_prec_round_all(x, prec, Floor);
        let ceiling = power_of_10_x_minus_1_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0 || r.0.is_nan());
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(
                &power_of_10_x_minus_1_prec_round_all(x, prec, Exact),
                &r
            ));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = power_of_10_x_minus_1_prec_round_all(x, prec + 16, rm);
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
            &power_of_10_x_minus_1_rational_prec_round_all(&q, prec, rm),
            &r
        ));
    }
    // At an integer n, the exact value is a `Rational`.
    if let Ok(n) = Integer::try_from(x)
        && n.unsigned_abs_ref() <= &1000u32
    {
        let n = i64::exact_from(&n);
        let exact = Rational::from(10u32).pow(n) - Rational::ONE;
        assert!(same_pair(
            &BigExponentFloat::from_rational_prec_round(exact, prec, rm),
            &r
        ));
    }
}

#[test]
fn power_of_10_x_minus_1_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_14().test_properties(
        |(x, prec, rm)| {
            power_of_10_x_minus_1_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn power_of_10_x_minus_1_prec_round_float_agreement_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        for rm in exhaustive_rounding_modes() {
            if rm == Exact {
                continue;
            }
            let x = &x;
            let (f, fo) = x.clone().power_of_10_x_minus_1_prec_round(prec, rm);
            if float_result_comparable(&f, fo) {
                let r =
                    power_of_10_x_minus_1_prec_round_all(&float_to_big_exponent_float(x), prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        }
    });
}

#[test]
fn power_of_10_x_minus_1_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_24().test_properties(|(x, prec, rm)| {
        let r = power_of_10_x_minus_1_rational_prec_round_all(&x, prec, rm);
        if r.0.is_zero() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        if rm == Exact {
            assert_eq!(r.1, Equal);
        } else if rm != Nearest {
            let (y, o) = power_of_10_x_minus_1_rational_prec_round_all(&x, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    });
}

#[test]
fn power_of_10_x_minus_1_rational_prec_round_float_agreement_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_24().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let x = &x;
        let (f, fo) = Float::power_of_10_x_minus_1_rational_prec_round_ref(x, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = power_of_10_x_minus_1_rational_prec_round_all(x, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
