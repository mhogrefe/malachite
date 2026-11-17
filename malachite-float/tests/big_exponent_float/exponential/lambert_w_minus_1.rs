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

fn lambert_w_minus_1_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().lambert_w_minus_1_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.lambert_w_minus_1_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.lambert_w_minus_1_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().lambert_w_minus_1_prec(prec), &r));
        assert!(same_pair(&x.lambert_w_minus_1_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.lambert_w_minus_1_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn lambert_w_minus_1_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::lambert_w_minus_1_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::lambert_w_minus_1_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::lambert_w_minus_1_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::lambert_w_minus_1_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_lambert_w_minus_1_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &lambert_w_minus_1_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &lambert_w_minus_1_prec_round_all(&inf, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &lambert_w_minus_1_prec_round_all(&ninf, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &lambert_w_minus_1_prec_round_all(&zero, 10, rm),
            &(ninf.clone(), Equal)
        ));
        // Positive, and below -1/e
        assert!(same_pair(
            &lambert_w_minus_1_prec_round_all(&BigExponentFloat::ONE, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &lambert_w_minus_1_prec_round_all(&BigExponentFloat::NEGATIVE_ONE, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &lambert_w_minus_1_rational_prec_round_all(&Rational::ZERO, 10, rm),
            &(ninf.clone(), Equal)
        ));
        assert!(same_pair(
            &lambert_w_minus_1_rational_prec_round_all(&Rational::ONE, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &lambert_w_minus_1_rational_prec_round_all(&Rational::NEGATIVE_ONE, 10, rm),
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
        let r = lambert_w_minus_1_prec_round_all(&hex(x_hex), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("1.0", "0x1.0", 10, Nearest, "NaN", "NaN", Equal);
    test("0.50", "0x0.8", 10, Nearest, "NaN", "NaN", Equal);
    test("2.0", "0x2.0", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test("10.0", "0xa.0", 10, Nearest, "NaN", "NaN", Equal);
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
        10,
        Nearest,
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
        10,
        Nearest,
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
        "-4.0",
        "-0x4.0",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        64,
        Nearest,
        "-3.57715206395729714141",
        "-0x3.93c03cd783f7414c",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Floor,
        "-3.578",
        "-0x3.94",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Ceiling,
        "-3.5742",
        "-0x3.93",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Down,
        "-3.5742",
        "-0x3.93",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Up,
        "-3.578",
        "-0x3.94",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a",
        10,
        Nearest,
        "-3.578",
        "-0x3.94",
        Less,
    );
    test("-0.25", "-0x0.4", 1, Nearest, "-2.0", "-0x2.0", Greater);
    test(
        "-0.25",
        "-0x0.4",
        64,
        Nearest,
        "-2.1532923641103496491",
        "-0x2.273e2b1a949a113",
        Greater,
    );
    test("-0.25", "-0x0.4", 10, Floor, "-2.156", "-0x2.28", Less);
    test(
        "-0.25", "-0x0.4", 10, Ceiling, "-2.1523", "-0x2.27", Greater,
    );
    test("-0.25", "-0x0.4", 10, Down, "-2.1523", "-0x2.27", Greater);
    test("-0.25", "-0x0.4", 10, Up, "-2.156", "-0x2.28", Less);
    test(
        "-0.25", "-0x0.4", 10, Nearest, "-2.1523", "-0x2.27", Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        1,
        Nearest,
        "-2.0",
        "-0x2.0",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        64,
        Nearest,
        "-1.7813370234216276964",
        "-0x1.c805b402bff7effc",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Floor,
        "-1.7832",
        "-0x1.c88",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Ceiling,
        "-1.78",
        "-0x1.c8",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Down,
        "-1.78",
        "-0x1.c8",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Up,
        "-1.7832",
        "-0x1.c88",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc",
        10,
        Nearest,
        "-1.78",
        "-0x1.c8",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        1,
        Nearest,
        "-16.0",
        "-0x1.0E+1",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        64,
        Nearest,
        "-14.163600815810182921",
        "-0xe.29e1be3980ef8f4",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        10,
        Floor,
        "-14.172",
        "-0xe.2c",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        10,
        Ceiling,
        "-14.16",
        "-0xe.28",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        10,
        Down,
        "-14.16",
        "-0xe.28",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        10,
        Up,
        "-14.172",
        "-0xe.2c",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788",
        10,
        Nearest,
        "-14.16",
        "-0xe.28",
        Greater,
    );
    test(
        "-1.000000000000000e-100",
        "-0xd.ff9772470298E-84",
        1,
        Nearest,
        "-2.6e2",
        "-0x1.0E+2",
        Less,
    );
    test(
        "-1.000000000000000e-100",
        "-0xd.ff9772470298E-84",
        64,
        Nearest,
        "-235.72115887568531364",
        "-0xeb.b89dde3a49de86",
        Less,
    );
    test(
        "-1.000000000000000e-100",
        "-0xd.ff9772470298E-84",
        10,
        Floor,
        "-235.75",
        "-0xeb.c",
        Less,
    );
    test(
        "-1.000000000000000e-100",
        "-0xd.ff9772470298E-84",
        10,
        Ceiling,
        "-235.5",
        "-0xeb.8",
        Greater,
    );
    test(
        "-1.000000000000000e-100",
        "-0xd.ff9772470298E-84",
        10,
        Down,
        "-235.5",
        "-0xeb.8",
        Greater,
    );
    test(
        "-1.000000000000000e-100",
        "-0xd.ff9772470298E-84",
        10,
        Up,
        "-235.75",
        "-0xeb.c",
        Less,
    );
    test(
        "-1.000000000000000e-100",
        "-0xd.ff9772470298E-84",
        10,
        Nearest,
        "-235.75",
        "-0xeb.c",
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
        Greater,
    );
    test(
        "-0.36787944117144228",
        "-0x0.5e2d58d8b3bcdc",
        64,
        Nearest,
        "-1.0000000153042542847",
        "-0x1.00000041bb349e4c",
        Less,
    );
    test(
        "-0.36787944117144228",
        "-0x0.5e2d58d8b3bcdc",
        10,
        Floor,
        "-1.0020",
        "-0x1.008",
        Less,
    );
    test(
        "-0.36787944117144228",
        "-0x0.5e2d58d8b3bcdc",
        10,
        Ceiling,
        "-1.0",
        "-0x1.0",
        Greater,
    );
    test(
        "-0.36787944117144228",
        "-0x0.5e2d58d8b3bcdc",
        10,
        Down,
        "-1.0",
        "-0x1.0",
        Greater,
    );
    test(
        "-0.36787944117144228",
        "-0x0.5e2d58d8b3bcdc",
        10,
        Up,
        "-1.0020",
        "-0x1.008",
        Less,
    );
    test(
        "-0.36787944117144228",
        "-0x0.5e2d58d8b3bcdc",
        10,
        Nearest,
        "-1.0",
        "-0x1.0",
        Greater,
    );
    test("-0.38", "-0x0.6", 10, Nearest, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558",
        1,
        Nearest,
        "-2.0",
        "-0x2.0",
        Less,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558",
        64,
        Nearest,
        "-1.51213455165784247389",
        "-0x1.831b3ffe85a57666",
        Greater,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558",
        10,
        Floor,
        "-1.5137",
        "-0x1.838",
        Less,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558",
        10,
        Ceiling,
        "-1.512",
        "-0x1.83",
        Greater,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558",
        10,
        Down,
        "-1.512",
        "-0x1.83",
        Greater,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558",
        10,
        Up,
        "-1.5137",
        "-0x1.838",
        Less,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558",
        10,
        Nearest,
        "-1.512",
        "-0x1.83",
        Greater,
    );
    test(
        "-0.0010000000000000000000000000000000000000000000000000000000000006",
        "-0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608",
        1,
        Nearest,
        "-8.0",
        "-0x8.0",
        Greater,
    );
    test(
        "-0.0010000000000000000000000000000000000000000000000000000000000006",
        "-0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608",
        64,
        Nearest,
        "-9.118006470402740121",
        "-0x9.1e35ac0b189dfe",
        Greater,
    );
    test(
        "-0.0010000000000000000000000000000000000000000000000000000000000006",
        "-0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608",
        10,
        Floor,
        "-9.125",
        "-0x9.2",
        Less,
    );
    test(
        "-0.0010000000000000000000000000000000000000000000000000000000000006",
        "-0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608",
        10,
        Ceiling,
        "-9.1094",
        "-0x9.1c",
        Greater,
    );
    test(
        "-0.0010000000000000000000000000000000000000000000000000000000000006",
        "-0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608",
        10,
        Down,
        "-9.1094",
        "-0x9.1c",
        Greater,
    );
    test(
        "-0.0010000000000000000000000000000000000000000000000000000000000006",
        "-0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608",
        10,
        Up,
        "-9.125",
        "-0x9.2",
        Less,
    );
    test(
        "-0.0010000000000000000000000000000000000000000000000000000000000006",
        "-0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608",
        10,
        Nearest,
        "-9.125",
        "-0x9.2",
        Less,
    );
    test(
        "9.6e301029995663",
        "0x1.0E+250000000000",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e-301029995664",
        "0x1.0E-250000000000",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0e-301029995664",
        "-0x1.0E-250000000000",
        1,
        Nearest,
        "-5.5e11",
        "-0x8.0E+9",
        Greater,
    );
    test(
        "-1.0e-301029995664",
        "-0x1.0E-250000000000",
        64,
        Nearest,
        "-693147180587.209817588",
        "-0xa162ca9e2b.35b69b",
        Greater,
    );
    test(
        "-1.0e-301029995664",
        "-0x1.0E-250000000000",
        10,
        Floor,
        "-6.936e11",
        "-0xa.18E+9",
        Less,
    );
    test(
        "-1.0e-301029995664",
        "-0x1.0E-250000000000",
        10,
        Ceiling,
        "-6.9256e11",
        "-0xa.14E+9",
        Greater,
    );
    test(
        "-1.0e-301029995664",
        "-0x1.0E-250000000000",
        10,
        Down,
        "-6.9256e11",
        "-0xa.14E+9",
        Greater,
    );
    test(
        "-1.0e-301029995664",
        "-0x1.0E-250000000000",
        10,
        Up,
        "-6.936e11",
        "-0xa.18E+9",
        Less,
    );
    test(
        "-1.0e-301029995664",
        "-0x1.0E-250000000000",
        10,
        Nearest,
        "-6.936e11",
        "-0xa.18E+9",
        Less,
    );
    test(
        "2.6e355393490465494856466",
        "0x3.0E+295147905179352825856",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.4e-355393490465494856466",
        "0x3.0E-295147905179352825856",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-3.4e-355393490465494856466",
        "-0x3.0E-295147905179352825856",
        1,
        Nearest,
        "-5.9e20",
        "-0x2.0E+17",
        Greater,
    );
    test(
        "-3.4e-355393490465494856466",
        "-0x3.0E-295147905179352825856",
        64,
        Nearest,
        "-818323753292969962304.0",
        "-0x2.c5c85fdf473de6b4E+17",
        Less,
    );
    test(
        "-3.4e-355393490465494856466",
        "-0x3.0E-295147905179352825856",
        10,
        Floor,
        "-8.186e20",
        "-0x2.c6E+17",
        Less,
    );
    test(
        "-3.4e-355393490465494856466",
        "-0x3.0E-295147905179352825856",
        10,
        Ceiling,
        "-8.1742e20",
        "-0x2.c5E+17",
        Greater,
    );
    test(
        "-3.4e-355393490465494856466",
        "-0x3.0E-295147905179352825856",
        10,
        Down,
        "-8.1742e20",
        "-0x2.c5E+17",
        Greater,
    );
    test(
        "-3.4e-355393490465494856466",
        "-0x3.0E-295147905179352825856",
        10,
        Up,
        "-8.186e20",
        "-0x2.c6E+17",
        Less,
    );
    test(
        "-3.4e-355393490465494856466",
        "-0x3.0E-295147905179352825856",
        10,
        Nearest,
        "-8.186e20",
        "-0x2.c6E+17",
        Less,
    );
    // - lambert_w_neg below -1/e
    // - q_at q < 0
    test("-1.0", "-0x1.0", 1, Down, "NaN", "NaN", Equal);
    // - apriori_wm1 q_hi > 1/4
    // - deriv_lower_exp c > 0
    // - lambert_bracket mean-value rounds reach the target
    // - mul_bracket_any y < 0
    // - mvt_round dist > 0
    // - q_at q > 0
    // - q_tight recomputed
    // - rounding_possible yes
    // - seed_float W_-1 q >= 0.3
    test("-0.25", "-0x0.4", 1, Down, "-2.0", "-0x2.0", Greater);
    // - q_tight c == 0
    test("-0.12", "-0x0.2", 1, Down, "-2.0", "-0x2.0", Greater);
    // - apriori_wm1 q_hi <= 1/4
    // - seed_float W_-1 q < 0.3
    test("-0.31", "-0x0.5", 1, Down, "-1.0", "-0x1.0", Greater);
    // - halley_refine doubling step
    test(
        "-0.00450557310251921752663362551670784",
        "-0x0.012746f92003c13df434710df9978",
        126,
        Down,
        "-7.40453249301213627085586025163763751860",
        "-0x7.678f7103a80eaafb9eb3e95d248056a",
        Greater,
    );
    // - log_seed Newton
    // - seed_float x outside the seed range
    test(
        "-1.832633e-2921",
        "-0x2.df2eE-2426",
        10,
        Nearest,
        "-6736.0",
        "-0x1.a5E+3",
        Less,
    );
    test(
        "-2.18109122535176717417819822953324775017318e-8202822486",
        "-0xb.601df43c6dc28418ddfd96a8f1779829dE-6812296619",
        62,
        Ceiling,
        "-18887696799.621910267",
        "-0x465cb619f.9f3582e",
        Greater,
    );
    test(
        "-1.2210906945354240e-322",
        "-0x6.2dc507e392e14E-268",
        93,
        Up,
        "-747.84985770564796280249651250",
        "-0x2eb.d990464c02f7df37e3bb4",
        Less,
    );
    // - halley_candidate series seed
    test(
        "-0.36787944117144228",
        "-0x0.5e2d58d8b3bcdc",
        1,
        Nearest,
        "-1.0",
        "-0x1.0",
        Greater,
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
        Greater,
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
    // - deriv_lower_exp c <= 0
    test(
        "-2.1e-363922934236666733021125",
        "-0x1.0E-302231454903657293676544",
        3,
        Floor,
        "-9.1e23",
        "-0xc.0E+19",
        Less,
    );
}

#[test]
fn test_lambert_w_minus_1_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r =
            lambert_w_minus_1_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("1", 10, Nearest, "NaN", "NaN", Equal);
    test("1/3", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "-1/3",
        53,
        Nearest,
        "-1.5121345516578424",
        "-0x1.831b3ffe85a57",
        Greater,
    );
    test("-1/3", 10, Floor, "-1.5137", "-0x1.838", Less);
    test("-1/3", 10, Ceiling, "-1.512", "-0x1.83", Greater);
    test("-1/3", 10, Down, "-1.512", "-0x1.83", Greater);
    test("-1/3", 10, Up, "-1.5137", "-0x1.838", Less);
    test("-1/3", 10, Nearest, "-1.512", "-0x1.83", Greater);
    test("22/7", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "-1/4",
        53,
        Nearest,
        "-2.1532923641103499",
        "-0x2.273e2b1a949a2",
        Less,
    );
    test("-1/4", 10, Floor, "-2.156", "-0x2.28", Less);
    test("-1/4", 10, Ceiling, "-2.1523", "-0x2.27", Greater);
    test("-1/4", 10, Down, "-2.1523", "-0x2.27", Greater);
    test("-1/4", 10, Up, "-2.156", "-0x2.28", Less);
    test("-1/4", 10, Nearest, "-2.1523", "-0x2.27", Greater);
    test(
        "-1/1000",
        53,
        Nearest,
        "-9.1180064704027401",
        "-0x9.1e35ac0b189e",
        Less,
    );
    test("-1/1000", 10, Floor, "-9.125", "-0x9.2", Less);
    test("-1/1000", 10, Ceiling, "-9.1094", "-0x9.1c", Greater);
    test("-1/1000", 10, Down, "-9.1094", "-0x9.1c", Greater);
    test("-1/1000", 10, Up, "-9.125", "-0x9.2", Less);
    test("-1/1000", 10, Nearest, "-9.125", "-0x9.2", Less);
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
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1/1000000000000000000000000000000",
        53,
        Nearest,
        "-73.373110313822977",
        "-0x49.5f842853ab84",
        Less,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Floor,
        "-73.375",
        "-0x49.6",
        Less,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Ceiling,
        "-73.25",
        "-0x49.4",
        Greater,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Down,
        "-73.25",
        "-0x49.4",
        Greater,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Up,
        "-73.375",
        "-0x49.6",
        Less,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Nearest,
        "-73.375",
        "-0x49.6",
        Less,
    );
    test(
        "-11/30",
        53,
        Nearest,
        "-1.0834824917754593",
        "-0x1.155f1bcbf6d4f",
        Greater,
    );
    test("-11/30", 10, Floor, "-1.0840", "-0x1.158", Less);
    test("-11/30", 10, Ceiling, "-1.082", "-0x1.15", Greater);
    test("-11/30", 10, Down, "-1.082", "-0x1.15", Greater);
    test("-11/30", 10, Up, "-1.0840", "-0x1.158", Less);
    test("-11/30", 10, Nearest, "-1.0840", "-0x1.158", Less);
    test("-3/8", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "-4598493014643029/12500000000000000",
        53,
        Nearest,
        "-1.0000000029451939",
        "-0x1.0000000ca6466",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Floor,
        "-1.0020",
        "-0x1.008",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Ceiling,
        "-1.0",
        "-0x1.0",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Down,
        "-1.0",
        "-0x1.0",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Up,
        "-1.0020",
        "-0x1.008",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Nearest,
        "-1.0",
        "-0x1.0",
        Greater,
    );
    test(
        "-36787944117144233/100000000000000000",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
}

#[test]
#[should_panic]
fn lambert_w_minus_1_prec_round_fail_1() {
    BigExponentFloat::ONE.lambert_w_minus_1_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn lambert_w_minus_1_prec_round_fail_2() {
    BigExponentFloat::from(-0.25).lambert_w_minus_1_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn lambert_w_minus_1_rational_prec_round_fail_1() {
    BigExponentFloat::lambert_w_minus_1_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn lambert_w_minus_1_rational_prec_round_fail_2() {
    BigExponentFloat::lambert_w_minus_1_rational_prec_round(
        Rational::from_signeds(-1, 3),
        10,
        Exact,
    );
}

fn lambert_w_minus_1_prec_round_properties_helper(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) {
    let r = lambert_w_minus_1_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if *x >= 0u32 {
        assert!(r.0.is_nan() || x.is_zero() && r.0 == BigExponentFloat::NEGATIVE_INFINITY);
    } else if !r.0.is_nan() {
        // W_{-1} maps (-1/e, 0) to (-∞, -1).
        assert!(r.0 <= -1i32);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else {
        if r.0.is_finite() && !r.0.is_zero() {
            // The result is correctly rounded, by a check that is independent of the algorithm.
            assert_lambert_w_correctly_rounded(
                false,
                &|f| f.partial_cmp(x).unwrap(),
                Some(x),
                &r.0,
                r.1,
                prec,
                rm,
            );
        }
        // The result lies between the results rounded down and up.
        let floor = lambert_w_minus_1_prec_round_all(x, prec, Floor);
        let ceiling = lambert_w_minus_1_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0 || r.0.is_nan());
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(
                &lambert_w_minus_1_prec_round_all(x, prec, Exact),
                &r
            ));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = lambert_w_minus_1_prec_round_all(x, prec + 16, rm);
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
            &lambert_w_minus_1_rational_prec_round_all(&q, prec, rm),
            &r
        ));
    }
}

#[test]
fn lambert_w_minus_1_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_24().test_properties(
        |(x, prec, rm)| {
            lambert_w_minus_1_prec_round_properties_helper(&x, prec, rm);
        },
    );

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 128);
    config.insert("mean_small_n", 512);
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_24().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            lambert_w_minus_1_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn lambert_w_minus_1_prec_round_float_agreement_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        for rm in exhaustive_rounding_modes() {
            if rm == Exact {
                continue;
            }
            let x = &x;
            let (f, fo) = x.lambert_w_minus_1_prec_round_ref(prec, rm);
            if float_result_comparable(&f, fo) {
                let r = lambert_w_minus_1_prec_round_all(&float_to_big_exponent_float(x), prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        }
    });
}

#[test]
fn lambert_w_minus_1_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        let r = lambert_w_minus_1_rational_prec_round_all(&x, prec, rm);
        if !r.0.is_finite() || r.0.is_zero() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        if x >= 0u32 {
            assert!(r.0.is_nan() || x == 0u32 && r.0 == BigExponentFloat::NEGATIVE_INFINITY);
        } else if !r.0.is_nan() {
            assert!(r.0 <= -1i32);
        }
        if rm == Exact {
            assert_eq!(r.1, Equal);
        } else {
            if r.0.is_finite() && !r.0.is_zero() {
                assert_lambert_w_correctly_rounded(
                    false,
                    &|f| f.partial_cmp(&x).unwrap(),
                    None,
                    &r.0,
                    r.1,
                    prec,
                    rm,
                );
            }
            if rm != Nearest {
                let (y, o) = lambert_w_minus_1_rational_prec_round_all(&x, prec + 16, rm);
                let (z, oz) = y.round_to_prec_round(prec, rm);
                assert!(same(&z, &r.0));
                assert_eq!(if o == Equal { oz } else { o }, r.1);
            }
        }
    });
}

#[test]
fn lambert_w_minus_1_rational_prec_round_float_agreement_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let x = &x;
        let (f, fo) = Float::lambert_w_minus_1_rational_prec_round_ref(x, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = lambert_w_minus_1_rational_prec_round_all(x, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
