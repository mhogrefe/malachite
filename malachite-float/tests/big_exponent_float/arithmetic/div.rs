// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{One, Two};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::div::div_prec_round_generic;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_5,
    float_float_unsigned_rounding_mode_quadruple_gen_var_4,
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

fn div_prec_round_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().div_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone().div_prec_round_val_ref(y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.div_prec_round_ref_val(y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(&x.div_prec_round_ref_ref(y, prec, rm), &r));
    let mut z = x.clone();
    let o = z.div_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = x.clone();
    let o = z.div_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().div_prec(y.clone(), prec), &r));
        assert!(same_pair(&x.clone().div_prec_val_ref(y, prec), &r));
        assert!(same_pair(&x.div_prec_ref_val(y.clone(), prec), &r));
        assert!(same_pair(&x.div_prec_ref_ref(y, prec), &r));
        let mut z = x.clone();
        let o = z.div_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = x.clone();
        let o = z.div_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

#[test]
fn test_div_prec_round() {
    let check = |x: &str,
                 x_hex: &str,
                 y: &str,
                 y_hex: &str,
                 prec: u64,
                 rm: RoundingMode,
                 out: &str,
                 out_hex: &str,
                 o| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(parse_big_exponent_float_hex(y_hex).to_string(), y);
        let y = &parse_big_exponent_float_hex(y_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        let r = div_prec_round_all(x, y, prec, rm);
        assert!(
            same(&r.0, out),
            "{x:?} {y:?} {prec} {rm}: {:?} != {out:?}",
            r.0
        );
        assert_eq!(r.1, o);
    };
    check("NaN", "NaN", "1.0", "0x1.0", 10, Floor, "NaN", "NaN", Equal);
    check("1.0", "0x1.0", "NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    check(
        "1.0", "0x1.0", "0.0", "0x0.0", 10, Floor, "NaN", "NaN", Equal,
    );
    check(
        "0.0", "0x0.0", "0.0", "0x0.0", 10, Floor, "NaN", "NaN", Equal,
    );
    check(
        "Infinity", "Infinity", "0.0", "0x0.0", 10, Floor, "NaN", "NaN", Equal,
    );
    check(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    check(
        "Infinity", "Infinity", "2.0", "0x2.0", 10, Floor, "Infinity", "Infinity", Equal,
    );
    check(
        "Infinity",
        "Infinity",
        "-2.0",
        "-0x2.0",
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    check(
        "-Infinity",
        "-Infinity",
        "-2.0",
        "-0x2.0",
        10,
        Floor,
        "Infinity",
        "Infinity",
        Equal,
    );
    check(
        "0.0", "0x0.0", "2.0", "0x2.0", 10, Floor, "0.0", "0x0.0", Equal,
    );
    check(
        "0.0", "0x0.0", "Infinity", "Infinity", 10, Floor, "0.0", "0x0.0", Equal,
    );
    check(
        "2.0",
        "0x2.0",
        "-Infinity",
        "-Infinity",
        10,
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    check(
        "1.0", "0x1.0", "4.0", "0x4.0", 1, Exact, "0.25", "0x0.4", Equal,
    );
    check(
        "-2.0", "-0x2.0", "0.50", "0x0.8", 1, Exact, "-4.0", "-0x4.0", Equal,
    );
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        1,
        Exact,
        "5.6e60205999132796239042",
        "0x1.0E+50000000000000000000",
        Equal,
    );
    check(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        1,
        Exact,
        "1.8e-60205999132796239043",
        "0x1.0E-50000000000000000000",
        Equal,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        5,
        Floor,
        "1.12",
        "0x1.2",
        Less,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        20,
        Nearest,
        "1.155727",
        "0x1.27ddc",
        Greater,
    );
}

#[test]
#[should_panic]
fn div_prec_round_fail_1() {
    BigExponentFloat::ONE.div_prec_round(BigExponentFloat::TWO, 0, Floor);
}

#[test]
#[should_panic]
fn div_prec_round_fail_2() {
    BigExponentFloat::ONE.div_prec_round(BigExponentFloat::from(3.0), 10, Exact);
}

#[test]
fn div_prec_round_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_5()
        .test_properties(|(x, y, prec, rm)| {
            let r = div_prec_round_all(&x, &y, prec, rm);
            // The kernels agree with the kernel-free implementation, at any exponents.
            assert!(same_pair(&div_prec_round_generic(&x, &y, prec, rm), &r));
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // (-x)/y = x/(-y) = -(x/y), with the rounding mode and the ordering reversed.
            let (q, o) = (-&x).div_prec_round_val_ref(&y, prec, -rm);
            assert!(same(&-q, &r.0));
            assert_eq!(o.reverse(), r.1);
            let (q, o) = x.div_prec_round_ref_val(-&y, prec, -rm);
            assert!(same(&-q, &r.0));
            assert_eq!(o.reverse(), r.1);
            // x/y = x * (1/y) when 1/y is exact.
            if y.is_finite() && !y.is_zero() && y.get_prec() == Some(1) {
                let (reciprocal, o) = y.reciprocal_prec_round_ref(1, Exact);
                assert_eq!(o, Equal);
                assert!(same_pair(
                    &x.mul_prec_round_ref_ref(&reciprocal, prec, rm),
                    &r
                ));
            }
            if big_exponent_float_exponent_is_small(&x)
                && big_exponent_float_exponent_is_small(&y)
                && x.is_finite()
                && y.is_finite()
                && !y.is_zero()
            {
                let q = Rational::exact_from(&x) / Rational::exact_from(&y);
                assert_eq!(Rational::exact_from(&r.0).cmp(&q), r.1);
            }
        });
}

#[test]
fn div_prec_round_float_agreement_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_4().test_properties(|(x, y, prec, rm)| {
        let (f, fo) = x.div_prec_round_ref_ref(&y, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = div_prec_round_all(
                &float_to_big_exponent_float(&x),
                &float_to_big_exponent_float(&y),
                prec,
                rm,
            );
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
