// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, FromStringBase};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    float_to_big_exponent_float, gudermannian_prec_round_naive,
    gudermannian_rational_prec_round_naive,
};
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23, float_unsigned_pair_gen_var_1,
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

// Whether the reference, whose time grows with the argument, is run on `x`.
fn reference_is_cheap(x: &BigExponentFloat) -> bool {
    x.get_exponent()
        .is_some_and(|e| *e >= -1000i32 && *e <= 8u32)
}

fn exponent_is_moderate(x: &BigExponentFloat) -> bool {
    x.get_exponent()
        .is_none_or(|e| e.unsigned_abs_ref() <= &256u32)
}

fn gudermannian_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().gudermannian_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.gudermannian_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.gudermannian_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().gudermannian_prec(prec), &r));
        assert!(same_pair(&x.gudermannian_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.gudermannian_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn gudermannian_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::gudermannian_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::gudermannian_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::gudermannian_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::gudermannian_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_gudermannian_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &gudermannian_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &gudermannian_prec_round_all(&zero, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &gudermannian_rational_prec_round_all(&Rational::ZERO, 10, rm),
            &(zero.clone(), Equal)
        ));
        // gd(±infinity) = ±pi/2, rounded
        if rm != Exact {
            assert!(same_pair(
                &gudermannian_prec_round_all(&inf, 10, rm),
                &BigExponentFloat::pi_over_2_prec_round(10, rm)
            ));
            let (p, o) = BigExponentFloat::pi_over_2_prec_round(10, -rm);
            assert!(same_pair(
                &gudermannian_prec_round_all(&ninf, 10, rm),
                &(-p, o.reverse())
            ));
        }
    }
    // The expected values agree with Azurite's verified Gudermannian function.
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let r = gudermannian_prec_round_all(&hex(x_hex), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("1.0", "0x1.0", 1, Nearest, "1.0", "0x1.0", Greater);
    test(
        "1.0",
        "0x1.0",
        64,
        Nearest,
        "0.86576948323965862429",
        "0x0.dda311a0639f9766",
        Less,
    );
    test("1.0", "0x1.0", 10, Floor, "0.8652", "0x0.dd8", Less);
    test("1.0", "0x1.0", 10, Ceiling, "0.86621", "0x0.ddc", Greater);
    test("1.0", "0x1.0", 10, Down, "0.8652", "0x0.dd8", Less);
    test("1.0", "0x1.0", 10, Up, "0.86621", "0x0.ddc", Greater);
    test("1.0", "0x1.0", 10, Nearest, "0.86621", "0x0.ddc", Greater);
    test("0.50", "0x0.8", 1, Nearest, "0.50", "0x0.8", Greater);
    test(
        "0.50",
        "0x0.8",
        64,
        Nearest,
        "0.480381079133729448609",
        "0x0.7afa41207f1e4b238",
        Greater,
    );
    test("0.50", "0x0.8", 10, Floor, "0.47998", "0x0.7ae", Less);
    test("0.50", "0x0.8", 10, Ceiling, "0.4805", "0x0.7b", Greater);
    test("0.50", "0x0.8", 10, Down, "0.47998", "0x0.7ae", Less);
    test("0.50", "0x0.8", 10, Up, "0.4805", "0x0.7b", Greater);
    test("0.50", "0x0.8", 10, Nearest, "0.4805", "0x0.7b", Greater);
    test("2.0", "0x2.0", 1, Nearest, "1.0", "0x1.0", Less);
    test(
        "2.0",
        "0x2.0",
        64,
        Nearest,
        "1.3017603360460150998",
        "0x1.4d402a568c2fd418",
        Less,
    );
    test("2.0", "0x2.0", 10, Floor, "1.301", "0x1.4d", Less);
    test("2.0", "0x2.0", 10, Ceiling, "1.3027", "0x1.4d8", Greater);
    test("2.0", "0x2.0", 10, Down, "1.301", "0x1.4d", Less);
    test("2.0", "0x2.0", 10, Up, "1.3027", "0x1.4d8", Greater);
    test("2.0", "0x2.0", 10, Nearest, "1.3027", "0x1.4d8", Greater);
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        64,
        Nearest,
        "1.43901131596367510819",
        "0x1.70630baca34de4ca",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Floor,
        "1.44",
        "0x1.7",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Ceiling,
        "1.4395",
        "0x1.708",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Down,
        "1.44",
        "0x1.7",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Up,
        "1.4395",
        "0x1.708",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Nearest,
        "1.4395",
        "0x1.708",
        Greater,
    );
    test("10.0", "0xa.0", 1, Nearest, "2.0", "0x2.0", Greater);
    test(
        "10.0",
        "0xa.0",
        64,
        Nearest,
        "1.5707055269354340337",
        "0x1.9219c1e5d589f484",
        Greater,
    );
    test("10.0", "0xa.0", 10, Floor, "1.570", "0x1.92", Less);
    test("10.0", "0xa.0", 10, Ceiling, "1.5723", "0x1.928", Greater);
    test("10.0", "0xa.0", 10, Down, "1.570", "0x1.92", Less);
    test("10.0", "0xa.0", 10, Up, "1.5723", "0x1.928", Greater);
    test("10.0", "0xa.0", 10, Nearest, "1.570", "0x1.92", Less);
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        1,
        Nearest,
        "2.0",
        "0x2.0",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        64,
        Nearest,
        "1.57079632679489661926",
        "0x1.921fb54442d1846a",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Floor,
        "1.570",
        "0x1.92",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Ceiling,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Down,
        "1.570",
        "0x1.92",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Up,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Nearest,
        "1.570",
        "0x1.92",
        Less,
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
        "9.99999999983333415104e-6",
        "0x0.0000a7c5ac470f4512eb",
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
        "2.0",
        "0x2.0",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        64,
        Nearest,
        "1.57079632679489661926",
        "0x1.921fb54442d1846a",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Floor,
        "1.570",
        "0x1.92",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Ceiling,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Down,
        "1.570",
        "0x1.92",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Up,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Nearest,
        "1.570",
        "0x1.92",
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
        "2.0",
        "0x2.0",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        64,
        Nearest,
        "1.57079632679489661926",
        "0x1.921fb54442d1846a",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Floor,
        "1.570",
        "0x1.92",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Ceiling,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Down,
        "1.570",
        "0x1.92",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Up,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Nearest,
        "1.570",
        "0x1.92",
        Less,
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
        "-0.099833748793486620595",
        "-0x0.198eb45e1ae820c94",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Floor,
        "-0.09985",
        "-0x0.199",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Ceiling,
        "-0.099731",
        "-0x0.1988",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Down,
        "-0.099731",
        "-0x0.1988",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Up,
        "-0.09985",
        "-0x0.199",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Nearest,
        "-0.09985",
        "-0x0.199",
        Less,
    );
    test("-0.25", "-0x0.4", 1, Nearest, "-0.25", "-0x0.4", Less);
    test(
        "-0.25",
        "-0x0.4",
        64,
        Nearest,
        "-0.247435798982431482760",
        "-0x0.3f57f3d87d3a7610c",
        Greater,
    );
    test("-0.25", "-0x0.4", 10, Floor, "-0.2476", "-0x0.3f6", Less);
    test(
        "-0.25", "-0x0.4", 10, Ceiling, "-0.24731", "-0x0.3f5", Greater,
    );
    test("-0.25", "-0x0.4", 10, Down, "-0.24731", "-0x0.3f5", Greater);
    test("-0.25", "-0x0.4", 10, Up, "-0.2476", "-0x0.3f6", Less);
    test(
        "-0.25", "-0x0.4", 10, Nearest, "-0.24731", "-0x0.3f5", Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        1,
        Nearest,
        "-0.25",
        "-0x0.4",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        64,
        Nearest,
        "-0.295598675984604127069",
        "-0x0.4bac5ad61843321a8",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Floor,
        "-0.2959",
        "-0x0.4bc",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Ceiling,
        "-0.29541",
        "-0x0.4ba",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Down,
        "-0.29541",
        "-0x0.4ba",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Up,
        "-0.2959",
        "-0x0.4bc",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Nearest,
        "-0.29541",
        "-0x0.4ba",
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
        "-9.99999999983333415104e-6",
        "-0x0.0000a7c5ac470f4512eb",
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
    test("1.0", "0x1.0", 1, Down, "0.50", "0x0.8", Less);
    test("0.50", "0x0.8", 1, Down, "0.25", "0x0.4", Less);
    test("-0.25", "-0x0.4", 1, Down, "-0.12", "-0x0.2", Greater);
    test(
        "2.3281685280997921493337341223879518220630854242143616671023085e4835",
        "0x6.b014710df6d8d0fb1901206ed24e1e002b4411ac77d2348fd2E+4015",
        84,
        Ceiling,
        "1.57079632679489661923132178",
        "0x1.921fb54442d18469898ce",
        Greater,
    );
    test(
        "8.0125492416043474e10778525511745053023",
        "0x4.f38a550e9d4aE+8951371679731519077",
        13,
        Up,
        "1.5708",
        "0x1.922",
        Greater,
    );
    test(
        "-0.367879441171442321595523770161460867445811131031767834507837",
        "-0x0.5e2d58d8b3bcdf1abadec7829054f90dda9805aab56c77333",
        10,
        Nearest,
        "-0.35986",
        "-0x0.5c2",
        Less,
    );
    test(
        "-0.3678794411714423215955237701614608674458111310317678345078373",
        "-0x0.5e2d58d8b3bcdf1abadec7829054f90dda9805aab56c773331",
        10,
        Nearest,
        "-0.35986",
        "-0x0.5c2",
        Less,
    );
    // - atan_alt an even number of terms
    // - atan_alt an odd number of terms
    // - atan_alt term below the threshold
    // - encl_narrow series accepted
    // - float_abs_floor e < 1
    // - gd_bracket small series tried
    // - gudermannian_helper not tiny
    // - ziv_loop_certified retry
    test(
        "2.1e-22",
        "0x1.0E-18",
        6,
        Up,
        "2.1e-22",
        "0x1.0E-18",
        Greater,
    );
    // - float_abs_floor 1 <= e < 62
    // - gd_bracket general path
    // - gd_bracket large series not tried
    test("1.0", "0x1.0", 21, Up, "0.86576986", "0x0.dda318", Greater);
    // - float_abs_floor e >= 62
    // - gd_bracket huge
    test("1.8e19", "0x1.0E+16", 3, Up, "1.8", "0x1.c", Greater);
    // - gd_bracket small series not tried
    test("0.50", "0x0.8", 1, Up, "0.50", "0x0.8", Greater);
    // - gd_bracket large series tried
    test("1.0", "0x1.0", 1, Up, "1.0", "0x1.0", Greater);
    // - gudermannian_helper tiny
    test(
        "3.2e-27",
        "0x1.0E-22",
        1,
        Up,
        "3.2e-27",
        "0x1.0E-22",
        Greater,
    );
}

#[test]
fn test_gudermannian_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r = gudermannian_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test(
        "1",
        53,
        Nearest,
        "0.86576948323965863",
        "0x0.dda311a0639f98",
        Greater,
    );
    test("1", 10, Floor, "0.8652", "0x0.dd8", Less);
    test("1", 10, Ceiling, "0.86621", "0x0.ddc", Greater);
    test("1", 10, Down, "0.8652", "0x0.dd8", Less);
    test("1", 10, Up, "0.86621", "0x0.ddc", Greater);
    test("1", 10, Nearest, "0.86621", "0x0.ddc", Greater);
    test(
        "1/3",
        53,
        Nearest,
        "0.32732661447167716",
        "0x0.53cbad50442934",
        Greater,
    );
    test("1/3", 10, Floor, "0.3271", "0x0.53c", Less);
    test("1/3", 10, Ceiling, "0.32764", "0x0.53e", Greater);
    test("1/3", 10, Down, "0.3271", "0x0.53c", Less);
    test("1/3", 10, Up, "0.32764", "0x0.53e", Greater);
    test("1/3", 10, Nearest, "0.3271", "0x0.53c", Less);
    test(
        "-1/3",
        53,
        Nearest,
        "-0.32732661447167716",
        "-0x0.53cbad50442934",
        Less,
    );
    test("-1/3", 10, Floor, "-0.32764", "-0x0.53e", Less);
    test("-1/3", 10, Ceiling, "-0.3271", "-0x0.53c", Greater);
    test("-1/3", 10, Down, "-0.3271", "-0x0.53c", Greater);
    test("-1/3", 10, Up, "-0.32764", "-0x0.53e", Less);
    test("-1/3", 10, Nearest, "-0.3271", "-0x0.53c", Greater);
    test(
        "22/7",
        53,
        Nearest,
        "1.4845312444284462",
        "0x1.7c0a3d58b5da1",
        Less,
    );
    test("22/7", 10, Floor, "1.484", "0x1.7c", Less);
    test("22/7", 10, Ceiling, "1.4863", "0x1.7c8", Greater);
    test("22/7", 10, Down, "1.484", "0x1.7c", Less);
    test("22/7", 10, Up, "1.4863", "0x1.7c8", Greater);
    test("22/7", 10, Nearest, "1.484", "0x1.7c", Less);
    test(
        "-1/4",
        53,
        Nearest,
        "-0.24743579898243148",
        "-0x0.3f57f3d87d3a76",
        Greater,
    );
    test("-1/4", 10, Floor, "-0.2476", "-0x0.3f6", Less);
    test("-1/4", 10, Ceiling, "-0.24731", "-0x0.3f5", Greater);
    test("-1/4", 10, Down, "-0.24731", "-0x0.3f5", Greater);
    test("-1/4", 10, Up, "-0.2476", "-0x0.3f6", Less);
    test("-1/4", 10, Nearest, "-0.24731", "-0x0.3f5", Greater);
    test(
        "-1/1000",
        53,
        Nearest,
        "-0.00099999983333337508",
        "-0x0.0041893694862bf",
        Less,
    );
    test("-1/1000", 10, Floor, "-0.0010014", "-0x0.0041a", Less);
    test("-1/1000", 10, Ceiling, "-0.0009995", "-0x0.00418", Greater);
    test("-1/1000", 10, Down, "-0.0009995", "-0x0.00418", Greater);
    test("-1/1000", 10, Up, "-0.0010014", "-0x0.0041a", Less);
    test("-1/1000", 10, Nearest, "-0.0009995", "-0x0.00418", Greater);
    test(
        "1000000000000000000000000000000",
        53,
        Nearest,
        "1.5707963267948966",
        "0x1.921fb54442d18",
        Less,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Floor,
        "1.570",
        "0x1.92",
        Less,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Ceiling,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Down,
        "1.570",
        "0x1.92",
        Less,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Up,
        "1.5723",
        "0x1.928",
        Greater,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Nearest,
        "1.570",
        "0x1.92",
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
        "-0.35871642108764440",
        "-0x0.5bd4d6e11c0e94",
        Greater,
    );
    test("-11/30", 10, Floor, "-0.35889", "-0x0.5be", Less);
    test("-11/30", 10, Ceiling, "-0.3584", "-0x0.5bc", Greater);
    test("-11/30", 10, Down, "-0.3584", "-0x0.5bc", Greater);
    test("-11/30", 10, Up, "-0.35889", "-0x0.5be", Less);
    test("-11/30", 10, Nearest, "-0.35889", "-0x0.5be", Less);
    test("-3/8", 10, Nearest, "-0.36670", "-0x0.5de", Less);
    test(
        "-4598493014643029/12500000000000000",
        53,
        Nearest,
        "-0.35985175838320388",
        "-0x0.5c1f3eada9f8ec",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Floor,
        "-0.35986",
        "-0x0.5c2",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Ceiling,
        "-0.359",
        "-0x0.5c",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Down,
        "-0.359",
        "-0x0.5c",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Up,
        "-0.35986",
        "-0x0.5c2",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Nearest,
        "-0.35986",
        "-0x0.5c2",
        Less,
    );
    test(
        "-36787944117144233/100000000000000000",
        10,
        Nearest,
        "-0.35986",
        "-0x0.5c2",
        Less,
    );
    // - atan_alt an even number of terms
    // - atan_alt an odd number of terms
    // - atan_alt term below the threshold
    // - encl_narrow series accepted
    // - gd_rational_bracket small series tried
    // - gudermannian_rational_helper not tiny
    // - ziv_loop_certified retry
    test(
        "1/147573952589676412928",
        6,
        Up,
        "6.8e-21",
        "0x2.0E-17",
        Greater,
    );
    // - gd_rational_bracket general path
    // - gd_rational_bracket large series not tried
    test("1", 21, Up, "0.86576986", "0x0.dda318", Greater);
    // - gd_rational_bracket large series tried
    test("1", 1, Up, "1.0", "0x1.0", Greater);
    // - gd_rational_bracket small series not tried
    test("1/2", 1, Up, "0.50", "0x0.8", Greater);
    // - gd_rational_bracket huge
    test("128", 5, Up, "1.62", "0x1.a", Greater);
    // - gudermannian_rational_helper tiny
    test(
        "1/75557863725914324467712",
        2,
        Up,
        "1.3e-23",
        "0x1.0E-19",
        Greater,
    );
}

#[test]
#[should_panic]
fn gudermannian_prec_round_fail_1() {
    BigExponentFloat::ONE.gudermannian_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn gudermannian_prec_round_fail_2() {
    BigExponentFloat::from(0.5).gudermannian_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn gudermannian_prec_round_fail_3() {
    BigExponentFloat::INFINITY.gudermannian_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn gudermannian_rational_prec_round_fail_1() {
    BigExponentFloat::gudermannian_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn gudermannian_rational_prec_round_fail_2() {
    BigExponentFloat::gudermannian_rational_prec_round(
        Rational::from_unsigneds(1u32, 3),
        10,
        Exact,
    );
}

fn gudermannian_prec_round_properties_helper(x: &BigExponentFloat, prec: u64, rm: RoundingMode) {
    let r = gudermannian_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if !x.is_nan() {
        // gd is odd, and |gd(x)| < pi / 2.
        assert_eq!(r.0.partial_cmp(&0u32), x.partial_cmp(&0u32));
        assert!(r.0.le_abs(&2u32));
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else {
        // The reference computes arctan(sinh(x)), in time that grows with x.
        if x.is_finite() && !x.is_zero() && reference_is_cheap(x) {
            assert!(same_pair(&gudermannian_prec_round_naive(x, prec, rm), &r));
        }
        // gd(-x) = -gd(x)
        let (neg, neg_o) = gudermannian_prec_round_all(&-x, prec, -rm);
        assert!(same_pair(&(-neg, neg_o.reverse()), &r));
        // The result lies between the results rounded down and up.
        let floor = gudermannian_prec_round_all(x, prec, Floor);
        let ceiling = gudermannian_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0 || r.0.is_nan());
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(&gudermannian_prec_round_all(x, prec, Exact), &r));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = gudermannian_prec_round_all(x, prec + 16, rm);
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
            &gudermannian_rational_prec_round_all(&q, prec, rm),
            &r
        ));
    }
}

#[test]
fn gudermannian_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties(
        |(x, prec, rm)| {
            gudermannian_prec_round_properties_helper(&x, prec, rm);
        },
    );

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 128);
    config.insert("mean_small_n", 512);
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            gudermannian_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn gudermannian_prec_round_float_agreement_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        for rm in exhaustive_rounding_modes() {
            if rm == Exact {
                continue;
            }
            let x = &x;
            let (f, fo) = x.gudermannian_prec_round_ref(prec, rm);
            if float_result_comparable(&f, fo) {
                let r = gudermannian_prec_round_all(&float_to_big_exponent_float(x), prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        }
    });
}

#[test]
fn gudermannian_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        let r = gudermannian_rational_prec_round_all(&x, prec, rm);
        if !r.0.is_finite() || r.0.is_zero() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        assert_eq!(r.0.partial_cmp(&0u32), x.partial_cmp(&0u32));
        assert!(r.0.le_abs(&2u32));
        if rm == Exact {
            assert_eq!(r.1, Equal);
        } else {
            if x != 0u32 && x.le_abs(&256u32) {
                assert!(same_pair(
                    &gudermannian_rational_prec_round_naive(&x, prec, rm),
                    &r
                ));
            }
            if rm != Nearest {
                let (y, o) = gudermannian_rational_prec_round_all(&x, prec + 16, rm);
                let (z, oz) = y.round_to_prec_round(prec, rm);
                assert!(same(&z, &r.0));
                assert_eq!(if o == Equal { oz } else { o }, r.1);
            }
        }
    });
}

#[test]
fn gudermannian_rational_prec_round_float_agreement_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let x = &x;
        let (f, fo) = Float::gudermannian_rational_prec_round_ref(x, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = gudermannian_rational_prec_round_all(x, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
