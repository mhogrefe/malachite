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
use malachite_float::big_exponent_float::arithmetic::add_rational::add_rational_prec_round_generic;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_power_of_2,
    float_to_big_exponent_float,
};
use malachite_float::test_util::generators::{
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_2,
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_2_float,
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

fn add_rational_prec_round_all(
    x: &BigExponentFloat,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().add_rational_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone().add_rational_prec_round_val_ref(y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.add_rational_prec_round_ref_val(y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.add_rational_prec_round_ref_ref(y, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.add_rational_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = x.clone();
    let o = z.add_rational_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().add_rational_prec(y.clone(), prec), &r));
        assert!(same_pair(&x.clone().add_rational_prec_val_ref(y, prec), &r));
        assert!(same_pair(&x.add_rational_prec_ref_val(y.clone(), prec), &r));
        assert!(same_pair(&x.add_rational_prec_ref_ref(y, prec), &r));
        let mut z = x.clone();
        let o = z.add_rational_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = x.clone();
        let o = z.add_rational_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

#[test]
fn test_add_rational_prec_round() {
    let check = |x: &BigExponentFloat,
                 y: &Rational,
                 prec: u64,
                 rm: RoundingMode,
                 out: &BigExponentFloat,
                 o| {
        let r = add_rational_prec_round_all(x, y, prec, rm);
        assert!(
            same(&r.0, out),
            "{x:?} {y} {prec} {rm}: {:?} != {out:?}",
            r.0
        );
        assert_eq!(r.1, o);
    };
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let tiny = big_exponent_float_power_of_2(-big.clone());
    let p2 = |e: Integer| big_exponent_float_power_of_2(e);
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let neg_inf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    let one = BigExponentFloat::ONE;
    let third = Rational::from_unsigneds(1u8, 3);
    let neg_third = -&third;
    let q_zero = Rational::ZERO;
    // 1/3 rounded to 10 bits.
    let third_floor = BigExponentFloat::from_rational_prec_round_ref(&third, 10, Floor).0;
    let third_ceiling = BigExponentFloat::from_rational_prec_round_ref(&third, 10, Ceiling).0;
    check(&nan, &third, 10, Floor, &nan, Equal);
    check(&inf, &third, 10, Floor, &inf, Equal);
    check(&neg_inf, &third, 10, Floor, &neg_inf, Equal);
    check(&zero, &q_zero, 10, Floor, &zero, Equal);
    check(&zero, &third, 10, Floor, &third_floor, Less);
    check(&one, &q_zero, 10, Floor, &one, Equal);
    check(
        &one,
        &Rational::ONE,
        1,
        Exact,
        &BigExponentFloat::TWO,
        Equal,
    );
    check(&one, &-Rational::ONE, 1, Exact, &zero, Equal);
    // 1/3 is far below 2^N: it decides only the rounding direction.
    check(&huge, &third, 10, Floor, &huge, Less);
    check(&huge, &third, 10, Nearest, &huge, Less);
    check(
        &huge,
        &third,
        10,
        Ceiling,
        &(&huge + p2(big.clone() - Integer::from(9))),
        Greater,
    );
    check(
        &huge,
        &neg_third,
        10,
        Floor,
        &(&huge - p2(big.clone() - Integer::from(10))),
        Less,
    );
    // 2^-N is far below 1/3: it decides only the rounding direction.
    check(&tiny, &third, 10, Floor, &third_floor, Less);
    check(&tiny, &third, 10, Ceiling, &third_ceiling, Greater);
    check(&-&tiny, &third, 10, Floor, &third_floor, Less);
    check(&-&tiny, &third, 10, Ceiling, &third_ceiling, Greater);
}

#[test]
#[should_panic]
fn add_rational_prec_round_fail_1() {
    BigExponentFloat::ONE.add_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn add_rational_prec_round_fail_2() {
    BigExponentFloat::ONE.add_rational_prec_round(Rational::from_unsigneds(1u8, 3), 10, Exact);
}

#[test]
fn add_rational_prec_round_properties() {
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_2().test_properties(
        |(x, y, prec, rm)| {
            let r = add_rational_prec_round_all(&x, &y, prec, rm);
            // The kernels agree with the kernel-free implementation, at any exponents.
            assert!(same_pair(
                &add_rational_prec_round_generic(&x, &y, prec, rm),
                &r
            ));
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // x + y = x - (-y).
            assert!(same_pair(
                &x.sub_rational_prec_round_ref_val(-&y, prec, rm),
                &r
            ));
            // (-x) + (-y) = -(x + y), with the rounding mode and the ordering reversed.
            let (s, o) = (-&x).add_rational_prec_round(-&y, prec, -rm);
            assert!(same(&-s, &r.0));
            assert_eq!(o.reverse(), r.1);
            // A dyadic y is added like a `BigExponentFloat`.
            if let Ok(y) = BigExponentFloat::try_from(&y) {
                assert!(same_pair(&x.add_prec_round_ref_ref(&y, prec, rm), &r));
            }
            if big_exponent_float_exponent_is_small(&x) && x.is_finite() {
                let q = Rational::exact_from(&x) + &y;
                assert_eq!(Rational::exact_from(&r.0).cmp(&q), r.1);
            }
        },
    );
}

#[test]
fn add_rational_prec_round_float_agreement_properties() {
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_2_float().test_properties(
        |((fx, y, prec, rm), (x, _, _, _))| {
            let (f, fo) = fx.add_rational_prec_round_ref_ref(&y, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = add_rational_prec_round_all(&x, &y, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
