// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Two, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, FromStringBase};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    float_to_big_exponent_float, inverse_gudermannian_prec_round_naive,
    inverse_gudermannian_rational_prec_round_naive,
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

// Whether the reference, which would take as many bits as a tiny argument has leading zeros, is run
// on `x`.
fn reference_is_cheap(x: &BigExponentFloat) -> bool {
    x.get_exponent().is_some_and(|e| *e >= -1000i32)
}

fn exponent_is_moderate(x: &BigExponentFloat) -> bool {
    x.get_exponent()
        .is_none_or(|e| e.unsigned_abs_ref() <= &256u32)
}

fn inverse_gudermannian_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().inverse_gudermannian_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.inverse_gudermannian_prec_round_ref(prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.inverse_gudermannian_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().inverse_gudermannian_prec(prec), &r));
        assert!(same_pair(&x.inverse_gudermannian_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.inverse_gudermannian_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn inverse_gudermannian_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::inverse_gudermannian_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::inverse_gudermannian_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::inverse_gudermannian_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::inverse_gudermannian_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_inverse_gudermannian_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &inverse_gudermannian_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &inverse_gudermannian_prec_round_all(&zero, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &inverse_gudermannian_rational_prec_round_all(&Rational::ZERO, 10, rm),
            &(zero.clone(), Equal)
        ));
        // Outside the domain
        assert!(same_pair(
            &inverse_gudermannian_prec_round_all(&inf, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &inverse_gudermannian_prec_round_all(&ninf, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &inverse_gudermannian_prec_round_all(&BigExponentFloat::TWO, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &inverse_gudermannian_rational_prec_round_all(&Rational::TWO, 10, rm),
            &(nan.clone(), Equal)
        ));
    }
    // The expected values agree with Azurite's verified inverse Gudermannian function.
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let r = inverse_gudermannian_prec_round_all(&hex(x_hex), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("1.0", "0x1.0", 1, Nearest, "1.0", "0x1.0", Less);
    test(
        "1.0",
        "0x1.0",
        64,
        Nearest,
        "1.22619117088351707080",
        "0x1.39e7aa2196b1faa2",
        Less,
    );
    test("1.0", "0x1.0", 10, Floor, "1.2246", "0x1.398", Less);
    test("1.0", "0x1.0", 10, Ceiling, "1.227", "0x1.3a", Greater);
    test("1.0", "0x1.0", 10, Down, "1.2246", "0x1.398", Less);
    test("1.0", "0x1.0", 10, Up, "1.227", "0x1.3a", Greater);
    test("1.0", "0x1.0", 10, Nearest, "1.227", "0x1.3a", Greater);
    test("0.50", "0x0.8", 1, Nearest, "0.50", "0x0.8", Less);
    test(
        "0.50",
        "0x0.8",
        64,
        Nearest,
        "0.5222381032784403302",
        "0x0.85b165764e5427a",
        Greater,
    );
    test("0.50", "0x0.8", 10, Floor, "0.5215", "0x0.858", Less);
    test("0.50", "0x0.8", 10, Ceiling, "0.52246", "0x0.85c", Greater);
    test("0.50", "0x0.8", 10, Down, "0.5215", "0x0.858", Less);
    test("0.50", "0x0.8", 10, Up, "0.52246", "0x0.85c", Greater);
    test("0.50", "0x0.8", 10, Nearest, "0.52246", "0x0.85c", Greater);
    test("2.0", "0x2.0", 1, Nearest, "NaN", "NaN", Equal);
    test("2.0", "0x2.0", 64, Nearest, "NaN", "NaN", Equal);
    test("2.0", "0x2.0", 10, Floor, "NaN", "NaN", Equal);
    test("2.0", "0x2.0", 10, Ceiling, "NaN", "NaN", Equal);
    test("2.0", "0x2.0", 10, Down, "NaN", "NaN", Equal);
    test("2.0", "0x2.0", 10, Up, "NaN", "NaN", Equal);
    test("2.0", "0x2.0", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        64,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test("10.0", "0xa.0", 1, Nearest, "NaN", "NaN", Equal);
    test("10.0", "0xa.0", 64, Nearest, "NaN", "NaN", Equal);
    test("10.0", "0xa.0", 10, Floor, "NaN", "NaN", Equal);
    test("10.0", "0xa.0", 10, Ceiling, "NaN", "NaN", Equal);
    test("10.0", "0xa.0", 10, Down, "NaN", "NaN", Equal);
    test("10.0", "0xa.0", 10, Up, "NaN", "NaN", Equal);
    test("10.0", "0xa.0", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        64,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
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
        "0.0000100000000001666674850",
        "0x0.0000a7c5ac472749fd15",
        Greater,
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
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        64,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
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
        Less,
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
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        64,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
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
        "-0.100167084547480188073",
        "-0x0.19a48cd044604be32",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Floor,
        "-0.10022",
        "-0x0.19a8",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Ceiling,
        "-0.1001",
        "-0x0.19a",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Down,
        "-0.1001",
        "-0x0.19a",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Up,
        "-0.10022",
        "-0x0.19a8",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Nearest,
        "-0.10022",
        "-0x0.19a8",
        Less,
    );
    test("-0.25", "-0x0.4", 1, Nearest, "-0.25", "-0x0.4", Greater);
    test(
        "-0.25",
        "-0x0.4",
        64,
        Nearest,
        "-0.25264561035786753829",
        "-0x0.40ad61f9f70a3c52",
        Greater,
    );
    test("-0.25", "-0x0.4", 10, Floor, "-0.2529", "-0x0.40c", Less);
    test(
        "-0.25", "-0x0.4", 10, Ceiling, "-0.25244", "-0x0.40a", Greater,
    );
    test("-0.25", "-0x0.4", 10, Down, "-0.25244", "-0x0.40a", Greater);
    test("-0.25", "-0x0.4", 10, Up, "-0.2529", "-0x0.40c", Less);
    test(
        "-0.25", "-0x0.4", 10, Nearest, "-0.25244", "-0x0.40a", Greater,
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
        "-0.304603974401704092164",
        "-0x0.4dfa86ac4974ea6d8",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Floor,
        "-0.305",
        "-0x0.4e",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Ceiling,
        "-0.30420",
        "-0x0.4de",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Down,
        "-0.30420",
        "-0x0.4de",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Up,
        "-0.305",
        "-0x0.4e",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Nearest,
        "-0.305",
        "-0x0.4e",
        Less,
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
        "-0.0000100000000001666674850",
        "-0x0.0000a7c5ac472749fd15",
        Less,
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
    test("1.0", "0x1.0", 1, Down, "1.0", "0x1.0", Less);
    test("0.50", "0x0.8", 1, Down, "0.50", "0x0.8", Less);
    test("-0.25", "-0x0.4", 1, Down, "-0.25", "-0x0.4", Greater);
    test(
        "2.3281685280997921493337341223879518220630854242143616671023085e4835",
        "0x6.b014710df6d8d0fb1901206ed24e1e002b4411ac77d2348fd2E+4015",
        84,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "8.0125492416043474e10778525511745053023",
        "0x4.f38a550e9d4aE+8951371679731519077",
        13,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.367879441171442321595523770161460867445811131031767834507837",
        "-0x0.5e2d58d8b3bcdf1abadec7829054f90dda9805aab56c77333",
        10,
        Nearest,
        "-0.37646",
        "-0x0.606",
        Greater,
    );
    test(
        "-0.3678794411714423215955237701614608674458111310317678345078373",
        "-0x0.5e2d58d8b3bcdf1abadec7829054f90dda9805aab56c773331",
        10,
        Nearest,
        "-0.37646",
        "-0x0.606",
        Greater,
    );
    test("1.5", "0x1.8", 10, Nearest, "3.3398", "0x3.57", Less);
    test("1.2", "0x1.4", 20, Floor, "1.8214664", "0x1.d24ba", Less);
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18",
        53,
        Up,
        "38.025003373828874",
        "0x26.06669f00e27a",
        Greater,
    );
    test(
        "1.5707963267948968",
        "0x1.921fb54442d19",
        53,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.5707963267948966",
        "-0x1.921fb54442d18",
        100,
        Ceiling,
        "-38.025003373828868061802405161167",
        "-0x26.06669f00e27876a3a88273cc",
        Greater,
    );
    // - gd_inv_start e < 0
    // - inverse_gudermannian_helper exponent < 1
    // - odd_function_near_identity not tiny
    // - odd_function_near_identity positive
    // - ziv_loop_certified_from retry
    test(
        "5.7e-14",
        "0x1.0E-11",
        8,
        Up,
        "5.729e-14",
        "0x1.02E-11",
        Greater,
    );
    // - gd_inv_start e >= 0
    // - inverse_gudermannian_helper exponent 1 and below pi / 2
    // - odd_function_near_identity negative
    test("-1.0", "-0x1.0", 1, Up, "-2.0", "-0x2.0", Less);
    // - inverse_gudermannian_helper exponent 1 and above pi / 2
    test("1.62", "0x1.a", 1, Up, "NaN", "NaN", Equal);
    // - inverse_gudermannian_helper exponent > 1
    test("2.0", "0x2.0", 1, Up, "NaN", "NaN", Equal);
    // - odd_function_near_identity tiny argument
    test(
        "3.2e-27",
        "0x1.0E-22",
        1,
        Up,
        "6.5e-27",
        "0x2.0E-22",
        Greater,
    );
}

#[test]
fn test_inverse_gudermannian_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r =
            inverse_gudermannian_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test(
        "1",
        53,
        Nearest,
        "1.226191170883517",
        "0x1.39e7aa2196b2",
        Greater,
    );
    test("1", 10, Floor, "1.2246", "0x1.398", Less);
    test("1", 10, Ceiling, "1.227", "0x1.3a", Greater);
    test("1", 10, Down, "1.2246", "0x1.398", Less);
    test("1", 10, Up, "1.227", "0x1.3a", Greater);
    test("1", 10, Nearest, "1.227", "0x1.3a", Greater);
    test(
        "1/3",
        53,
        Nearest,
        "0.33968337608418653",
        "0x0.56f57d5f46c568",
        Greater,
    );
    test("1/3", 10, Floor, "0.33936", "0x0.56e", Less);
    test("1/3", 10, Ceiling, "0.3398", "0x0.57", Greater);
    test("1/3", 10, Down, "0.33936", "0x0.56e", Less);
    test("1/3", 10, Up, "0.3398", "0x0.57", Greater);
    test("1/3", 10, Nearest, "0.3398", "0x0.57", Greater);
    test(
        "-1/3",
        53,
        Nearest,
        "-0.33968337608418653",
        "-0x0.56f57d5f46c568",
        Less,
    );
    test("-1/3", 10, Floor, "-0.3398", "-0x0.57", Less);
    test("-1/3", 10, Ceiling, "-0.33936", "-0x0.56e", Greater);
    test("-1/3", 10, Down, "-0.33936", "-0x0.56e", Greater);
    test("-1/3", 10, Up, "-0.3398", "-0x0.57", Less);
    test("-1/3", 10, Nearest, "-0.3398", "-0x0.57", Less);
    test("22/7", 53, Nearest, "NaN", "NaN", Equal);
    test("22/7", 10, Floor, "NaN", "NaN", Equal);
    test("22/7", 10, Ceiling, "NaN", "NaN", Equal);
    test("22/7", 10, Down, "NaN", "NaN", Equal);
    test("22/7", 10, Up, "NaN", "NaN", Equal);
    test("22/7", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "-1/4",
        53,
        Nearest,
        "-0.25264561035786753",
        "-0x0.40ad61f9f70a3c",
        Greater,
    );
    test("-1/4", 10, Floor, "-0.2529", "-0x0.40c", Less);
    test("-1/4", 10, Ceiling, "-0.25244", "-0x0.40a", Greater);
    test("-1/4", 10, Down, "-0.25244", "-0x0.40a", Greater);
    test("-1/4", 10, Up, "-0.2529", "-0x0.40c", Less);
    test("-1/4", 10, Nearest, "-0.25244", "-0x0.40a", Greater);
    test(
        "-1/1000",
        53,
        Nearest,
        "-0.0010000001666667082",
        "-0x0.00418938030729f",
        Greater,
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
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
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
        "-0.37517012967630636",
        "-0x0.600b264d65542",
        Less,
    );
    test("-11/30", 10, Floor, "-0.37549", "-0x0.602", Less);
    test("-11/30", 10, Ceiling, "-0.38", "-0x0.6", Greater);
    test("-11/30", 10, Down, "-0.38", "-0x0.6", Greater);
    test("-11/30", 10, Up, "-0.37549", "-0x0.602", Less);
    test("-11/30", 10, Nearest, "-0.38", "-0x0.6", Greater);
    test("-3/8", 10, Nearest, "-0.38428", "-0x0.626", Less);
    test(
        "-4598493014643029/12500000000000000",
        53,
        Nearest,
        "-0.37646956354321420",
        "-0x0.60604f2f5b850c",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Floor,
        "-0.3770",
        "-0x0.608",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Ceiling,
        "-0.37646",
        "-0x0.606",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Down,
        "-0.37646",
        "-0x0.606",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Up,
        "-0.3770",
        "-0x0.608",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Nearest,
        "-0.37646",
        "-0x0.606",
        Greater,
    );
    test(
        "-36787944117144233/100000000000000000",
        10,
        Nearest,
        "-0.37646",
        "-0x0.606",
        Greater,
    );
    test("3/2", 10, Nearest, "3.3398", "0x3.57", Less);
    test(
        "157/100",
        30,
        Ceiling,
        "7.8286480382",
        "0x7.d422472",
        Greater,
    );
    test("11/7", 10, Nearest, "NaN", "NaN", Equal);
    test("2", 10, Floor, "NaN", "NaN", Equal);
    test("-3/2", 10, Down, "-3.3398", "-0x3.57", Greater);
    // - gd_inv_start e < 0
    // - inverse_gudermannian_rational_helper q <= 1 in absolute value
    // - odd_function_near_identity negative
    // - odd_function_near_identity not tiny
    // - ziv_loop_certified_from retry
    test("-1/274877906944", 1, Up, "-7.3e-12", "-0x8.0E-10", Less);
    // - gd_inv_start e >= 0
    // - inverse_gudermannian_rational_helper 1 < q < 2 in absolute value and below pi / 2
    // - odd_function_near_identity positive
    test("3/2", 1, Up, "4.0", "0x4.0", Greater);
    // - inverse_gudermannian_rational_helper q >= 2 in absolute value
    test("2", 1, Up, "NaN", "NaN", Equal);
    // - inverse_gudermannian_rational_helper 1 < q < 2 in absolute value and above pi / 2
    test("5/3", 1, Up, "NaN", "NaN", Equal);
    // - odd_function_near_identity tiny argument
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
fn inverse_gudermannian_prec_round_fail_1() {
    BigExponentFloat::ONE.inverse_gudermannian_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn inverse_gudermannian_prec_round_fail_2() {
    BigExponentFloat::from(0.5).inverse_gudermannian_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn inverse_gudermannian_rational_prec_round_fail_1() {
    BigExponentFloat::inverse_gudermannian_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn inverse_gudermannian_rational_prec_round_fail_2() {
    BigExponentFloat::inverse_gudermannian_rational_prec_round(
        Rational::from_unsigneds(1u32, 3),
        10,
        Exact,
    );
}

fn inverse_gudermannian_prec_round_properties_helper(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) {
    let r = inverse_gudermannian_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if r.0.is_nan() {
        // Outside the domain: |x| > pi / 2.
        assert!(x.is_nan() || x.ge_abs(&1u32));
    } else {
        // The function is odd, and its domain is (-pi / 2, pi / 2).
        assert_eq!(r.0.partial_cmp(&0u32), x.partial_cmp(&0u32));
        assert!(x.lt_abs(&2u32));
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else {
        // The reference computes atanh(sin(x)).
        if r.0.is_finite() && !r.0.is_zero() && reference_is_cheap(x) {
            assert!(same_pair(
                &inverse_gudermannian_prec_round_naive(x, prec, rm),
                &r
            ));
        }
        // The function is odd.
        let (neg, neg_o) = inverse_gudermannian_prec_round_all(&-x, prec, -rm);
        assert!(same_pair(&(-neg, neg_o.reverse()), &r));
        // The result lies between the results rounded down and up.
        let floor = inverse_gudermannian_prec_round_all(x, prec, Floor);
        let ceiling = inverse_gudermannian_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0 || r.0.is_nan());
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(
                &inverse_gudermannian_prec_round_all(x, prec, Exact),
                &r
            ));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = inverse_gudermannian_prec_round_all(x, prec + 16, rm);
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
            &inverse_gudermannian_rational_prec_round_all(&q, prec, rm),
            &r
        ));
    }
}

#[test]
fn inverse_gudermannian_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties(
        |(x, prec, rm)| {
            inverse_gudermannian_prec_round_properties_helper(&x, prec, rm);
        },
    );

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 128);
    config.insert("mean_small_n", 512);
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            inverse_gudermannian_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn inverse_gudermannian_prec_round_float_agreement_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        for rm in exhaustive_rounding_modes() {
            if rm == Exact {
                continue;
            }
            let x = &x;
            let (f, fo) = x.inverse_gudermannian_prec_round_ref(prec, rm);
            if float_result_comparable(&f, fo) {
                let r =
                    inverse_gudermannian_prec_round_all(&float_to_big_exponent_float(x), prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        }
    });
}

#[test]
fn inverse_gudermannian_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        let r = inverse_gudermannian_rational_prec_round_all(&x, prec, rm);
        if !r.0.is_finite() || r.0.is_zero() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        if r.0.is_nan() {
            assert!(x.ge_abs(&1u32));
        } else {
            assert_eq!(r.0.partial_cmp(&0u32), x.partial_cmp(&0u32));
            assert!(x.lt_abs(&2u32));
        }
        if rm == Exact {
            assert_eq!(r.1, Equal);
        } else {
            if r.0.is_finite() && !r.0.is_zero() {
                assert!(same_pair(
                    &inverse_gudermannian_rational_prec_round_naive(&x, prec, rm),
                    &r
                ));
            }
            if rm != Nearest {
                let (y, o) = inverse_gudermannian_rational_prec_round_all(&x, prec + 16, rm);
                let (z, oz) = y.round_to_prec_round(prec, rm);
                assert!(same(&z, &r.0));
                assert_eq!(if o == Equal { oz } else { o }, r.1);
            }
        }
    });
}

#[test]
fn inverse_gudermannian_rational_prec_round_float_agreement_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let x = &x;
        let (f, fo) = Float::inverse_gudermannian_rational_prec_round_ref(x, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = inverse_gudermannian_rational_prec_round_all(x, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
