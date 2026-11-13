// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, float_to_big_exponent_float,
};
use malachite_float::test_util::generators::{
    big_exponent_float_vec_pair_gen_var_1,
    big_exponent_float_vec_pair_unsigned_rounding_mode_quadruple_gen_var_1,
    float_vec_pair_unsigned_rounding_mode_quadruple_gen_var_1,
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

fn to_bef(x: &Float) -> BigExponentFloat {
    float_to_big_exponent_float(x)
}

fn dot_prec_round_all(
    xs: &[BigExponentFloat],
    ys: &[BigExponentFloat],
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::dot_prec_round(xs, ys, prec, rm);
    assert!(r.0.is_valid());
    if rm == Nearest {
        assert!(same_pair(&BigExponentFloat::dot_prec(xs, ys, prec), &r));
    }
    r
}

#[test]
fn test_dot_prec_round() {
    let check = |xs: &[BigExponentFloat],
                 ys: &[BigExponentFloat],
                 prec: u64,
                 rm: RoundingMode,
                 out: &BigExponentFloat,
                 o| {
        let r = dot_prec_round_all(xs, ys, prec, rm);
        assert!(same(&r.0, out), "{:?} != {out:?}", r.0);
        assert_eq!(r.1, o);
    };
    let one = BigExponentFloat::ONE;
    check(&[], &[], 10, Floor, &BigExponentFloat::ZERO, Equal);
    check(
        &[BigExponentFloat::INFINITY],
        &[BigExponentFloat::ZERO],
        10,
        Floor,
        &BigExponentFloat::NAN,
        Equal,
    );
    check(
        &[BigExponentFloat::INFINITY, BigExponentFloat::INFINITY],
        &[one.clone(), BigExponentFloat::from(-1.0)],
        10,
        Floor,
        &BigExponentFloat::NAN,
        Equal,
    );
    check(
        &[BigExponentFloat::from(1.5), BigExponentFloat::from(0.25)],
        &[BigExponentFloat::from(3.0), BigExponentFloat::from(-0.5)],
        6,
        Exact,
        &BigExponentFloat::from(4.375),
        Equal,
    );
}

#[test]
#[should_panic]
fn dot_prec_round_fail_1() {
    BigExponentFloat::dot_prec_round(&[BigExponentFloat::ONE], &[], 10, Floor);
}

#[test]
#[should_panic]
fn dot_prec_round_fail_2() {
    BigExponentFloat::dot_prec_round(&[BigExponentFloat::ONE], &[BigExponentFloat::ONE], 0, Floor);
}

#[test]
fn dot_prec_round_properties() {
    big_exponent_float_vec_pair_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(xs, ys, prec, rm)| {
            let r = dot_prec_round_all(&xs, &ys, prec, rm);
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // The dot product is the sum of the exact products, and the slices commute.
            let products: Vec<BigExponentFloat> = xs.iter().zip(&ys).map(|(x, y)| x * y).collect();
            assert!(same_pair(
                &BigExponentFloat::sum_prec_round(&products, prec, rm),
                &r
            ));
            assert!(same_pair(
                &BigExponentFloat::dot_prec_round(&ys, &xs, prec, rm),
                &r
            ));
            if xs.len() == 2 {
                assert!(same_pair(
                    &xs[0].mul_add_mul_prec_round_ref_ref_ref_ref(&ys[0], &xs[1], &ys[1], prec, rm),
                    &r
                ));
            }
            if xs
                .iter()
                .chain(&ys)
                .all(big_exponent_float_exponent_is_small)
                && xs.iter().chain(&ys).all(BigExponentFloat::is_finite)
            {
                let q: Rational = xs
                    .iter()
                    .zip(&ys)
                    .map(|(x, y)| Rational::exact_from(x) * Rational::exact_from(y))
                    .sum();
                assert_eq!(Rational::exact_from(&r.0).cmp(&q), r.1);
            }
        },
    );
}

#[test]
fn dot_properties() {
    big_exponent_float_vec_pair_gen_var_1().test_properties(|(xs, ys)| {
        let d = BigExponentFloat::dot(&xs, &ys);
        assert!(d.is_valid());
        if d.is_finite() {
            let q: Rational = xs
                .iter()
                .zip(&ys)
                .map(|(x, y)| Rational::exact_from(x) * Rational::exact_from(y))
                .sum();
            assert_eq!(Rational::exact_from(&d), q);
        }
    });
}

#[test]
fn dot_prec_round_float_agreement_properties() {
    float_vec_pair_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(xs, ys, prec, rm)| {
            let (f, fo) = Float::dot_prec_round(&xs, &ys, prec, rm);
            if float_result_comparable(&f, fo) {
                let bxs: Vec<BigExponentFloat> = xs.iter().map(to_bef).collect();
                let bys: Vec<BigExponentFloat> = ys.iter().map(to_bef).collect();
                let r = dot_prec_round_all(&bxs, &bys, prec, rm);
                assert!(same(&r.0, &to_bef(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
