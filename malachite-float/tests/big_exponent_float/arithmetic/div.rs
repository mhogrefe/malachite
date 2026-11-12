// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::div::div_prec_round_generic;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_power_of_2,
    float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_5,
    float_float_unsigned_rounding_mode_quadruple_gen_var_4,
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
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let tiny = big_exponent_float_power_of_2(-big.clone());
    let one = BigExponentFloat::ONE;
    let two = BigExponentFloat::TWO;
    let neg_two = BigExponentFloat::from(-2.0);
    let pi = parse_big_exponent_float_hex("0x3.243f6a8885a30#53");
    let e = parse_big_exponent_float_hex("0x2.b7e151628aed2#53");
    let check = |x: &BigExponentFloat,
                 y: &BigExponentFloat,
                 prec: u64,
                 rm: RoundingMode,
                 out: &BigExponentFloat,
                 o| {
        let r = div_prec_round_all(x, y, prec, rm);
        assert!(
            same(&r.0, out),
            "{x:?} {y:?} {prec} {rm}: {:?} != {out:?}",
            r.0
        );
        assert_eq!(r.1, o);
    };
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let neg_inf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    check(&nan, &one, 10, Floor, &nan, Equal);
    check(&one, &nan, 10, Floor, &nan, Equal);
    check(&one, &zero, 10, Floor, &nan, Equal);
    check(&zero, &zero, 10, Floor, &nan, Equal);
    check(&inf, &zero, 10, Floor, &nan, Equal);
    check(&inf, &neg_inf, 10, Floor, &nan, Equal);
    check(&inf, &two, 10, Floor, &inf, Equal);
    check(&inf, &neg_two, 10, Floor, &neg_inf, Equal);
    check(&neg_inf, &neg_two, 10, Floor, &inf, Equal);
    check(&zero, &two, 10, Floor, &zero, Equal);
    check(&zero, &inf, 10, Floor, &zero, Equal);
    check(&two, &neg_inf, 10, Floor, &zero, Equal);
    check(
        &one,
        &BigExponentFloat::from(4.0),
        1,
        Exact,
        &BigExponentFloat::from(0.25),
        Equal,
    );
    check(
        &neg_two,
        &BigExponentFloat::from(0.5),
        1,
        Exact,
        &BigExponentFloat::from(-4.0),
        Equal,
    );
    check(
        &huge,
        &tiny,
        1,
        Exact,
        &big_exponent_float_power_of_2(big.clone() * Integer::TWO),
        Equal,
    );
    check(
        &tiny,
        &huge,
        1,
        Exact,
        &big_exponent_float_power_of_2(-big * Integer::TWO),
        Equal,
    );
    check(
        &pi,
        &e,
        5,
        Floor,
        &parse_big_exponent_float_hex("0x1.2#5"),
        Less,
    );
    check(
        &pi,
        &e,
        20,
        Nearest,
        &parse_big_exponent_float_hex("0x1.27ddc#20"),
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
