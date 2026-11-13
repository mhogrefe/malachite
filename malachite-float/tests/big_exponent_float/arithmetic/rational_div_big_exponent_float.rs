// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::rational_div_big_exponent_float::*;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_power_of_2,
    float_to_big_exponent_float,
};
use malachite_float::test_util::generators::{
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_6,
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_6_float,
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

fn rational_div_big_exponent_float_prec_round_all(
    x: &Rational,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::rational_div_big_exponent_float_prec_round(
        x.clone(),
        y.clone(),
        prec,
        rm,
    );
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::rational_div_big_exponent_float_prec_round_val_ref(
            x.clone(),
            y,
            prec,
            rm
        ),
        &r
    ));
    assert!(same_pair(
        &BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_val(
            x,
            y.clone(),
            prec,
            rm
        ),
        &r
    ));
    assert!(same_pair(
        &BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_ref(x, y, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::rational_div_big_exponent_float_prec(x.clone(), y.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::rational_div_big_exponent_float_prec_val_ref(x.clone(), y, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::rational_div_big_exponent_float_prec_ref_val(x, y.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::rational_div_big_exponent_float_prec_ref_ref(x, y, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_rational_div_big_exponent_float_prec_round() {
    let check = |x: &Rational,
                 y: &BigExponentFloat,
                 prec: u64,
                 rm: RoundingMode,
                 out: &BigExponentFloat,
                 o| {
        let r = rational_div_big_exponent_float_prec_round_all(x, y, prec, rm);
        assert!(
            same(&r.0, out),
            "{x} {y:?} {prec} {rm}: {:?} != {out:?}",
            r.0
        );
        assert_eq!(r.1, o);
    };
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let tiny = big_exponent_float_power_of_2(-big.clone());
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let neg_inf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    let one = BigExponentFloat::ONE;
    let third = Rational::from_unsigneds(1u8, 3);
    let q_zero = Rational::ZERO;
    // 1/3 rounded to 10 bits.
    let third_floor = BigExponentFloat::from_rational_prec_round_ref(&third, 10, Floor).0;
    let third_ceiling = BigExponentFloat::from_rational_prec_round_ref(&third, 10, Ceiling).0;
    check(&third, &nan, 10, Floor, &nan, Equal);
    check(&third, &zero, 10, Floor, &nan, Equal);
    check(&q_zero, &zero, 10, Floor, &nan, Equal);
    check(&third, &inf, 10, Floor, &zero, Equal);
    check(&third, &neg_inf, 10, Floor, &zero, Equal);
    check(&q_zero, &one, 10, Floor, &zero, Equal);
    check(&third, &one, 10, Floor, &third_floor, Less);
    check(
        &Rational::from(3u32),
        &BigExponentFloat::from(3u32),
        1,
        Exact,
        &one,
        Equal,
    );
    check(&third, &huge, 10, Floor, &(&third_floor * &tiny), Less);
    check(
        &third,
        &tiny,
        10,
        Ceiling,
        &(&third_ceiling * &huge),
        Greater,
    );
}

#[test]
#[should_panic]
fn rational_div_big_exponent_float_prec_round_fail_1() {
    BigExponentFloat::rational_div_big_exponent_float_prec_round(
        Rational::ONE,
        BigExponentFloat::ONE,
        0,
        Floor,
    );
}

#[test]
#[should_panic]
fn rational_div_big_exponent_float_prec_round_fail_2() {
    BigExponentFloat::rational_div_big_exponent_float_prec_round(
        Rational::ONE,
        BigExponentFloat::from(3u32),
        10,
        Exact,
    );
}

#[test]
fn rational_div_big_exponent_float_prec_round_properties() {
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_6().test_properties(
        |(y, x, prec, rm)| {
            let r = rational_div_big_exponent_float_prec_round_all(&x, &y, prec, rm);
            // The kernels agree with the kernel-free implementation, at any exponents.
            assert!(same_pair(
                &rational_div_big_exponent_float_prec_round_generic(&x, &y, prec, rm),
                &r
            ));
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // (-x) / y = x / (-y) = -(x / y), with the rounding mode and the ordering reversed.
            let (q, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round_val_ref(
                -&x, &y, prec, -rm,
            );
            assert!(same(&-q, &r.0));
            assert_eq!(o.reverse(), r.1);
            // A dyadic x divides like a `BigExponentFloat`.
            if let Ok(x) = BigExponentFloat::try_from(&x) {
                assert!(same_pair(&x.div_prec_round_ref_ref(&y, prec, rm), &r));
            }
            if big_exponent_float_exponent_is_small(&y) && y.is_finite() && y != 0u32 {
                let q = &x / Rational::exact_from(&y);
                assert_eq!(Rational::exact_from(&r.0).cmp(&q), r.1);
            }
        },
    );
}

#[test]
fn rational_div_big_exponent_float_prec_round_float_agreement_properties() {
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_6_float().test_properties(
        |((fy, x, prec, rm), (y, _, _, _))| {
            let (f, fo) = Float::rational_div_float_prec_round_ref_ref(&x, &fy, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = rational_div_big_exponent_float_prec_round_all(&x, &y, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
