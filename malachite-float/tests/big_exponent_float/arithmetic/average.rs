// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Average, AverageAssign};
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_power_of_2,
    float_to_big_exponent_float,
};
use malachite_float::test_util::generators::*;
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

fn p2(e: Integer) -> BigExponentFloat {
    big_exponent_float_power_of_2(e)
}

fn big() -> Integer {
    Integer::from_str("100000000000000000000").unwrap()
}

fn average_prec_round_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().average_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone().average_prec_round_val_ref(y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.average_prec_round_ref_val(y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(&x.average_prec_round_ref_ref(y, prec, rm), &r));
    let mut z = x.clone();
    let o = z.average_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = x.clone();
    let o = z.average_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().average_prec(y.clone(), prec), &r));
        assert!(same_pair(&x.clone().average_prec_val_ref(y, prec), &r));
        assert!(same_pair(&x.average_prec_ref_val(y.clone(), prec), &r));
        assert!(same_pair(&x.average_prec_ref_ref(y, prec), &r));
        let mut z = x.clone();
        let o = z.average_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = x.clone();
        let o = z.average_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn average_exact_all(x: &BigExponentFloat, y: &BigExponentFloat) -> BigExponentFloat {
    let r = x.clone().average(y.clone());
    assert!(r.is_valid());
    assert!(same(&x.clone().average(y), &r));
    assert!(same(&x.average(y.clone()), &r));
    assert!(same(&x.average(y), &r));
    let mut z = x.clone();
    z.average_assign(y.clone());
    assert!(same(&z, &r));
    let mut z = x.clone();
    z.average_assign(y);
    assert!(same(&z, &r));
    r
}

#[test]
fn test_average() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    assert!(same(&average_exact_all(&nan, &BigExponentFloat::ONE), &nan));
    assert!(same(&average_exact_all(&inf, &ninf), &nan));
    assert!(same(&average_exact_all(&inf, &BigExponentFloat::ONE), &inf));
    assert!(same(
        &average_exact_all(&BigExponentFloat::ZERO, &ninf),
        &ninf
    ));
    // (1.5 + 2.25) / 2 = 1.875
    assert!(same(
        &average_exact_all(&BigExponentFloat::from(1.5), &BigExponentFloat::from(2.25)),
        &BigExponentFloat::from(1.875)
    ));
    // (2^N + 2^N) / 2 = 2^N, beyond `Float`'s range, and (2^N + 2^-N) / 2 = 2^(N - 1) + 2^(-N - 1).
    assert!(same(&average_exact_all(&p2(big()), &p2(big())), &p2(big())));
    let r = average_prec_round_all(&p2(big()), &p2(-big()), 1, Floor);
    assert!(same_pair(&r, &(p2(big() - Integer::ONE), Less)));
    let r = average_prec_round_all(&p2(big()), &p2(-big()), 1, Ceiling);
    assert!(same_pair(&r, &(p2(big()), Greater)));
}

#[test]
#[should_panic]
fn average_prec_round_fail() {
    BigExponentFloat::ONE.average_prec_round(BigExponentFloat::ONE, 0, Floor);
}

#[test]
fn average_prec_round_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_7()
        .test_properties(|(x, y, prec, rm)| {
            let r = average_prec_round_all(&x, &y, prec, rm);
            assert!(same_pair(&average_prec_round_all(&y, &x, prec, rm), &r));
            // The average is the rounded sum, halved.
            let (s, o) = x.add_prec_round_ref_ref(&y, prec, rm);
            assert!(same(&r.0, &(s >> 1u32)));
            assert_eq!(r.1, o);
            if x.is_finite()
                && y.is_finite()
                && big_exponent_float_exponent_is_small(&x)
                && big_exponent_float_exponent_is_small(&y)
                && r.0.is_finite()
            {
                let exact = (Rational::exact_from(&x) + Rational::exact_from(&y)) >> 1u32;
                assert_eq!(Rational::exact_from(&r.0).cmp(&exact), r.1);
            }
        });
}

#[test]
fn average_properties() {
    big_exponent_float_pair_gen_var_2().test_properties(|(x, y)| {
        let r = average_exact_all(&x, &y);
        if r.is_finite() {
            // The exact average, doubled, is the exact sum.
            assert!(same(&(&r << 1u32), &(&x + &y)));
        }
    });
}

#[test]
fn average_prec_round_float_agreement_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_7_float()
        .test_properties(|((fx, fy, prec, rm), (x, y, _, _))| {
            let (f, fo) = fx.average_prec_round_ref_ref(&fy, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = average_prec_round_all(&x, &y, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        });
}
