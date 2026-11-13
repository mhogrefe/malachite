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
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_power_of_2,
    float_to_big_exponent_float,
};
use malachite_float::test_util::generators::{
    big_exponent_float_vec_gen_var_1,
    big_exponent_float_vec_unsigned_rounding_mode_triple_gen_var_2,
    float_vec_unsigned_rounding_mode_triple_gen_var_3,
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

fn to_bef(x: &Float) -> BigExponentFloat {
    float_to_big_exponent_float(x)
}

fn product_prec_round_all(
    xs: &[BigExponentFloat],
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::product_prec_round(xs, prec, rm);
    assert!(r.0.is_valid());
    if rm == Nearest {
        assert!(same_pair(&BigExponentFloat::product_prec(xs, prec), &r));
    }
    r
}

fn product_all(xs: &[BigExponentFloat]) -> BigExponentFloat {
    let r: BigExponentFloat = xs.iter().cloned().product();
    assert!(r.is_valid());
    assert!(same(&xs.iter().product(), &r));
    r
}

#[test]
fn test_product_prec_round() {
    let check =
        |xs: &[BigExponentFloat], prec: u64, rm: RoundingMode, out: &BigExponentFloat, o| {
            let r = product_prec_round_all(xs, prec, rm);
            assert!(same(&r.0, out), "{:?} != {out:?}", r.0);
            assert_eq!(r.1, o);
        };
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let tiny = big_exponent_float_power_of_2(-big.clone());
    let one = BigExponentFloat::ONE;
    check(&[], 10, Floor, &one, Equal);
    check(
        &[BigExponentFloat::NAN, one.clone()],
        10,
        Floor,
        &BigExponentFloat::NAN,
        Equal,
    );
    check(
        &[BigExponentFloat::INFINITY, BigExponentFloat::ZERO],
        10,
        Floor,
        &BigExponentFloat::NAN,
        Equal,
    );
    check(
        &[BigExponentFloat::INFINITY, BigExponentFloat::from(-2.0)],
        10,
        Floor,
        &BigExponentFloat::NEGATIVE_INFINITY,
        Equal,
    );
    check(
        &[BigExponentFloat::ZERO, huge.clone()],
        10,
        Floor,
        &BigExponentFloat::ZERO,
        Equal,
    );
    check(
        &[huge.clone(), tiny.clone(), BigExponentFloat::from(3.0)],
        2,
        Exact,
        &BigExponentFloat::from(3.0),
        Equal,
    );
    check(
        &[huge.clone(), huge.clone()],
        1,
        Exact,
        &big_exponent_float_power_of_2(big * Integer::TWO),
        Equal,
    );
    check(
        &[BigExponentFloat::from(3.0), BigExponentFloat::from(3.0), BigExponentFloat::from(3.0)],
        3,
        Floor,
        &BigExponentFloat::from(24.0),
        Less,
    );
}

#[test]
#[should_panic]
fn product_prec_round_fail() {
    BigExponentFloat::product_prec_round(&[BigExponentFloat::ONE], 0, Floor);
}

#[test]
fn product_prec_round_properties() {
    big_exponent_float_vec_unsigned_rounding_mode_triple_gen_var_2().test_properties(
        |(xs, prec, rm)| {
            let r = product_prec_round_all(&xs, prec, rm);
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // The exact product, rounded once; the exponents may be huge, since the product of the
            // significands does not depend on them.
            assert!(same_pair(
                &product_all(&xs).round_to_prec_round(prec, rm),
                &r
            ));
            let mut reversed = xs.clone();
            reversed.reverse();
            assert!(same_pair(
                &BigExponentFloat::product_prec_round(&reversed, prec, rm),
                &r
            ));
            if xs.len() == 2 {
                assert!(same_pair(
                    &xs[0].mul_prec_round_ref_ref(&xs[1], prec, rm),
                    &r
                ));
            }
            if xs.iter().all(big_exponent_float_exponent_is_small) && r.0.is_finite() {
                let q: Rational = xs.iter().map(Rational::exact_from).product();
                assert_eq!(Rational::exact_from(&r.0).cmp(&q), r.1);
            }
        },
    );
}

#[test]
fn product_properties() {
    big_exponent_float_vec_gen_var_1().test_properties(|xs| {
        let p = product_all(&xs);
        if p.is_finite() {
            let q: Rational = xs.iter().map(Rational::exact_from).product();
            assert_eq!(Rational::exact_from(&p), q);
        }
    });
}

#[test]
fn product_prec_round_float_agreement_properties() {
    float_vec_unsigned_rounding_mode_triple_gen_var_3().test_properties(|(xs, prec, rm)| {
        let (f, fo) = Float::product_prec_round(&xs, prec, rm);
        if float_result_comparable(&f, fo) {
            let bs: Vec<BigExponentFloat> = xs.iter().map(to_bef).collect();
            let r = product_prec_round_all(&bs, prec, rm);
            assert!(same(&r.0, &to_bef(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
