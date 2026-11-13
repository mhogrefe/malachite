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
use malachite_float::big_exponent_float::arithmetic::sum::sum_prec_round_generic;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_power_of_2,
    float_to_big_exponent_float,
};
use malachite_float::test_util::generators::{
    big_exponent_float_vec_gen_var_1,
    big_exponent_float_vec_unsigned_rounding_mode_triple_gen_var_1,
    float_vec_unsigned_rounding_mode_triple_gen_var_1,
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

fn sum_prec_round_all(
    xs: &[BigExponentFloat],
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::sum_prec_round(xs, prec, rm);
    assert!(r.0.is_valid());
    let r_alt = sum_prec_round_generic(xs, prec, rm);
    assert!(r_alt.0.is_valid());
    assert!(same_pair(&r_alt, &r));
    if rm == Nearest {
        assert!(same_pair(&BigExponentFloat::sum_prec(xs, prec), &r));
    }
    r
}

fn sum_all(xs: &[BigExponentFloat]) -> BigExponentFloat {
    let r: BigExponentFloat = xs.iter().cloned().sum();
    assert!(r.is_valid());
    assert!(same(&xs.iter().sum(), &r));
    r
}

#[test]
fn test_sum_prec_round() {
    let check =
        |xs: &[BigExponentFloat], prec: u64, rm: RoundingMode, out: &BigExponentFloat, o| {
            let r = sum_prec_round_all(xs, prec, rm);
            assert!(same(&r.0, out), "{:?} != {out:?}", r.0);
            assert_eq!(r.1, o);
        };
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let tiny = big_exponent_float_power_of_2(-big.clone());
    let one = BigExponentFloat::ONE;
    let neg = |x: &BigExponentFloat| -x;
    check(&[], 10, Floor, &BigExponentFloat::ZERO, Equal);
    check(
        &[BigExponentFloat::NAN, one.clone()],
        10,
        Floor,
        &BigExponentFloat::NAN,
        Equal,
    );
    check(
        &[BigExponentFloat::INFINITY, BigExponentFloat::NEGATIVE_INFINITY],
        10,
        Floor,
        &BigExponentFloat::NAN,
        Equal,
    );
    check(
        &[BigExponentFloat::INFINITY, one.clone()],
        10,
        Floor,
        &BigExponentFloat::INFINITY,
        Equal,
    );
    // The huge terms cancel, leaving the small ones, which are far apart.
    check(
        &[huge.clone(), one.clone(), neg(&huge)],
        10,
        Exact,
        &one,
        Equal,
    );
    check(
        &[huge.clone(), tiny.clone(), one.clone(), neg(&huge)],
        1,
        Floor,
        &one,
        Less,
    );
    check(
        &[huge.clone(), tiny.clone(), one.clone(), neg(&huge)],
        1,
        Ceiling,
        &BigExponentFloat::from(2.0),
        Greater,
    );
    // A tiny term decides a tie.
    check(
        &[one.clone(), big_exponent_float_power_of_2(Integer::from(-10)), tiny.clone()],
        10,
        Nearest,
        &BigExponentFloat::from(1.0 + 1.0 / 512.0),
        Greater,
    );
    check(
        &[one.clone(), big_exponent_float_power_of_2(Integer::from(-10)), neg(&tiny)],
        10,
        Nearest,
        &one,
        Less,
    );
    // The eight equal terms sum to 8 - 2^-7, which -8 almost cancels: they must be added exactly,
    // not rounded first.
    let mut xs = vec![BigExponentFloat::from(1.0 - 1.0 / 1024.0); 8];
    xs.push(BigExponentFloat::from(-8.0));
    check(&xs, 2, Exact, &BigExponentFloat::from(-1.0 / 128.0), Equal);
    // Two clusters of three terms each, 2^100000000000000000000 apart; the upper cluster's sum,
    // 2^(N - 1) + 2^(N - 1) + 2^(N - 3), is a midpoint at 3 bits, and the lower cluster's sign
    // decides the tie.
    let p2 = |e: &Integer| big_exponent_float_power_of_2(e.clone());
    let upper =
        [p2(&(&big - Integer::ONE)), p2(&(&big - Integer::ONE)), p2(&(&big - Integer::from(3)))];
    let mut xs = upper.to_vec();
    xs.extend([one.clone(), one.clone(), one.clone()]);
    check(
        &xs,
        3,
        Nearest,
        &(&huge + p2(&(&big - Integer::TWO))),
        Greater,
    );
    check(&xs, 3, Floor, &huge, Less);
    let mut xs = upper.to_vec();
    xs.extend([neg(&one), neg(&one), neg(&one)]);
    check(&xs, 3, Nearest, &huge, Less);
    check(
        &xs,
        3,
        Ceiling,
        &(&huge + p2(&(&big - Integer::TWO))),
        Greater,
    );
    // Terms in several clusters, all cancelling but the lowest.
    check(
        &[huge.clone(), neg(&huge), one.clone(), neg(&one), tiny.clone()],
        3,
        Exact,
        &tiny,
        Equal,
    );
}

#[test]
#[should_panic]
fn sum_prec_round_fail() {
    BigExponentFloat::sum_prec_round(&[BigExponentFloat::ONE], 0, Floor);
}

#[test]
fn sum_prec_round_properties() {
    big_exponent_float_vec_unsigned_rounding_mode_triple_gen_var_1().test_properties(
        |(xs, prec, rm)| {
            let r = sum_prec_round_all(&xs, prec, rm);
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // The order of the terms does not matter.
            let mut reversed = xs.clone();
            reversed.reverse();
            assert!(same_pair(
                &BigExponentFloat::sum_prec_round(&reversed, prec, rm),
                &r
            ));
            // Two terms are added by `add_prec_round`, which handles any gap between them.
            if xs.len() == 2 {
                assert!(same_pair(
                    &xs[0].add_prec_round_ref_ref(&xs[1], prec, rm),
                    &r
                ));
            }
            // Adding a copy of the terms far above them, where they form separate clusters, gives
            // the same result as the generic implementation. (The sum may be inexact even if the
            // original one is not.)
            if rm != Exact {
                let mut far = xs.clone();
                far.extend(xs.iter().map(|x| x << 0x1_0000_0000_u64));
                sum_prec_round_all(&far, prec, rm);
            }
            // Shifting every term shifts the sum.
            let shifted: Vec<BigExponentFloat> = xs.iter().map(|x| x << 1000u32).collect();
            let (s, o) = BigExponentFloat::sum_prec_round(&shifted, prec, rm);
            assert!(same(&s, &(&r.0 << 1000u32)));
            assert_eq!(o, r.1);
            if xs.iter().all(big_exponent_float_exponent_is_small) {
                // The exact sum, rounded once.
                let exact = sum_all(&xs);
                assert!(same_pair(&exact.round_to_prec_round_ref(prec, rm), &r));
                if exact.is_finite() {
                    let q: Rational = xs.iter().map(Rational::exact_from).sum();
                    assert_eq!(Rational::exact_from(&r.0).cmp(&q), r.1);
                }
            }
        },
    );
}

#[test]
fn sum_properties() {
    big_exponent_float_vec_gen_var_1().test_properties(|xs| {
        let s = sum_all(&xs);
        if s.is_finite() {
            let q: Rational = xs.iter().map(Rational::exact_from).sum();
            assert_eq!(Rational::exact_from(&s), q);
        }
    });
}

#[test]
fn sum_prec_round_float_agreement_properties() {
    float_vec_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(xs, prec, rm)| {
        let (f, fo) = Float::sum_prec_round(&xs, prec, rm);
        if float_result_comparable(&f, fo) {
            let bs: Vec<BigExponentFloat> = xs.iter().map(to_bef).collect();
            let r = sum_prec_round_all(&bs, prec, rm);
            assert!(same(&r.0, &to_bef(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
