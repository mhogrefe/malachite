// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModInverse, ModMul, ModPow, ModPowerOf2, PowerOf2};
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Two, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_power_of_2,
    float_to_big_exponent_float,
};
use malachite_float::test_util::generators::*;
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
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

fn rem_prec_round_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().rem_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone().rem_prec_round_val_ref(y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.rem_prec_round_ref_val(y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(&x.rem_prec_round_ref_ref(y, prec, rm), &r));
    let mut z = x.clone();
    let o = z.rem_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = x.clone();
    let o = z.rem_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().rem_prec(y.clone(), prec), &r));
        assert!(same_pair(&x.clone().rem_prec_val_ref(y, prec), &r));
        assert!(same_pair(&x.rem_prec_ref_val(y.clone(), prec), &r));
        assert!(same_pair(&x.rem_prec_ref_ref(y, prec), &r));
        let mut z = x.clone();
        let o = z.rem_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = x.clone();
        let o = z.rem_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn rem_exact_all(x: &BigExponentFloat, y: &BigExponentFloat) -> BigExponentFloat {
    let r = x.clone() % y.clone();
    assert!(r.is_valid());
    assert!(same(&(x.clone() % y), &r));
    assert!(same(&(x % y.clone()), &r));
    assert!(same(&(x % y), &r));
    let mut z = x.clone();
    z %= y.clone();
    assert!(same(&z, &r));
    let mut z = x.clone();
    z %= y;
    assert!(same(&z, &r));
    assert!(same(&rem_and_quotient_bits_exact_all(x, y).0, &r));
    r
}

fn ieee_remainder_prec_round_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().ieee_remainder_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone().ieee_remainder_prec_round_val_ref(y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.ieee_remainder_prec_round_ref_val(y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.ieee_remainder_prec_round_ref_ref(y, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.ieee_remainder_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = x.clone();
    let o = z.ieee_remainder_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(
            &x.clone().ieee_remainder_prec(y.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.clone().ieee_remainder_prec_val_ref(y, prec),
            &r
        ));
        assert!(same_pair(
            &x.ieee_remainder_prec_ref_val(y.clone(), prec),
            &r
        ));
        assert!(same_pair(&x.ieee_remainder_prec_ref_ref(y, prec), &r));
        let mut z = x.clone();
        let o = z.ieee_remainder_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = x.clone();
        let o = z.ieee_remainder_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn ieee_remainder_exact_all(x: &BigExponentFloat, y: &BigExponentFloat) -> BigExponentFloat {
    let r = x.clone().ieee_remainder(y.clone());
    assert!(r.is_valid());
    assert!(same(&x.clone().ieee_remainder_val_ref(y), &r));
    assert!(same(&x.ieee_remainder_ref_val(y.clone()), &r));
    assert!(same(&x.ieee_remainder_ref_ref(y), &r));
    let mut z = x.clone();
    z.ieee_remainder_assign(y.clone());
    assert!(same(&z, &r));
    let mut z = x.clone();
    z.ieee_remainder_assign_ref(y);
    assert!(same(&z, &r));
    r
}

fn rem_and_quotient_bits_prec_round_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering, i64) {
    let r = x
        .clone()
        .rem_and_quotient_bits_prec_round(y.clone(), prec, rm);
    let same3 =
        |a: &(BigExponentFloat, Ordering, i64)| same(&a.0, &r.0) && a.1 == r.1 && a.2 == r.2;
    assert!(same3(
        &x.clone()
            .rem_and_quotient_bits_prec_round_val_ref(y, prec, rm)
    ));
    assert!(same3(&x.rem_and_quotient_bits_prec_round_ref_val(
        y.clone(),
        prec,
        rm
    )));
    assert!(same3(
        &x.rem_and_quotient_bits_prec_round_ref_ref(y, prec, rm)
    ));
    if rm == Nearest {
        assert!(same3(
            &x.clone().rem_and_quotient_bits_prec(y.clone(), prec)
        ));
        assert!(same3(
            &x.clone().rem_and_quotient_bits_prec_val_ref(y, prec)
        ));
        assert!(same3(
            &x.rem_and_quotient_bits_prec_ref_val(y.clone(), prec)
        ));
        assert!(same3(&x.rem_and_quotient_bits_prec_ref_ref(y, prec)));
    }
    assert!(same_pair(
        &(r.0.clone(), r.1),
        &rem_prec_round_all(x, y, prec, rm)
    ));
    r
}

fn rem_and_quotient_bits_exact_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
) -> (BigExponentFloat, i64) {
    let r = x.clone().rem_and_quotient_bits(y.clone());
    let same2 = |a: &(BigExponentFloat, i64)| same(&a.0, &r.0) && a.1 == r.1;
    assert!(same2(&x.clone().rem_and_quotient_bits_val_ref(y)));
    assert!(same2(&x.rem_and_quotient_bits_ref_val(y.clone())));
    assert!(same2(&x.rem_and_quotient_bits_ref_ref(y)));
    r
}

#[test]
fn test_rem() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    let one = BigExponentFloat::ONE;
    let three = BigExponentFloat::from(3);
    assert!(same(&rem_exact_all(&nan, &one), &nan));
    assert!(same(&rem_exact_all(&inf, &one), &nan));
    assert!(same(&rem_exact_all(&one, &zero), &nan));
    assert!(same(&rem_exact_all(&one, &ninf), &one));
    assert!(same(&rem_exact_all(&zero, &one), &zero));
    // 10.5 = 3 * 3 + 1.5, and 10.5 = 3 * 4 - 1.5 with 3.5 rounding to the even 4.
    let x = BigExponentFloat::from(10.5);
    assert!(same(
        &rem_exact_all(&x, &three),
        &BigExponentFloat::from(1.5)
    ));
    assert!(same(
        &rem_exact_all(&-&x, &three),
        &BigExponentFloat::from(-1.5)
    ));
    assert!(same(
        &ieee_remainder_exact_all(&x, &three),
        &BigExponentFloat::from(-1.5)
    ));
    let q = rem_and_quotient_bits_exact_all(&x, &three).1;
    assert_eq!(q, 3);
    let q = rem_and_quotient_bits_exact_all(&x, &-&three).1;
    assert_eq!(q, -3);
    // 7.5 = 3 * 2.5, which rounds to the even 2.
    assert!(same(
        &ieee_remainder_exact_all(&BigExponentFloat::from(7.5), &three),
        &BigExponentFloat::from(1.5)
    ));
    // Exponent gaps beyond `Float`'s range. 2^k mod 3 is 2 for odd k and 1 for even k, and 2^k mod
    // 5 is 1 for k divisible by 4.
    let n = big();
    assert!(same(&rem_exact_all(&p2(n.clone()), &three), &one));
    assert!(same(
        &rem_exact_all(&p2(n.clone() + Integer::ONE), &three),
        &BigExponentFloat::TWO
    ));
    assert!(same(
        &rem_exact_all(&p2(n.clone()), &BigExponentFloat::from(5)),
        &one
    ));
    // For even N, the quotient q = (2^N - 1) / 3, so 3q = -1 modulo 2^63.
    let q = rem_and_quotient_bits_exact_all(&p2(n.clone()), &three).1;
    assert_eq!(
        (u64::exact_from(q).wrapping_mul(3).wrapping_add(1)).mod_power_of_2(63),
        0
    );
    // 2^N mod 3 = 1 rounds the quotient down, so the IEEE remainder is 1 as well; 2^(N + 1) mod 3 =
    // 2 > 3/2 rounds it up, giving 2 - 3 = -1.
    assert!(same(
        &ieee_remainder_exact_all(&p2(n.clone()), &three),
        &one
    ));
    assert!(same(
        &ieee_remainder_exact_all(&p2(n.clone() + Integer::ONE), &three),
        &BigExponentFloat::from(-1)
    ));
    // A dividend far below the divisor is its own remainder.
    assert!(same(
        &rem_exact_all(&p2(-n.clone()), &three),
        &p2(-n.clone())
    ));
    // ... unless it is more than half of it, for the IEEE remainder: 3 * 2^N / 2^(N + 2) = 3/4
    // rounds to 1, giving 3 * 2^N - 2^(N + 2) = -2^N. Half of it, 2^N / 2^(N + 1), is a tie, which
    // rounds to the even 0.
    assert!(same(
        &ieee_remainder_exact_all(&(p2(n.clone()) * &three), &p2(n.clone() + Integer::TWO)),
        &-p2(n.clone())
    ));
    assert!(same(
        &ieee_remainder_exact_all(&p2(n.clone()), &p2(n.clone() + Integer::ONE)),
        &p2(n.clone())
    ));
    // The remainder of a power of 2 by an odd number 2^k * m with a huge k: 2^(N + 3) mod (2^N * 3)
    // = 2^N * (8 mod 3) = 2^(N + 1).
    assert!(same(
        &rem_exact_all(&p2(n.clone() + Integer::from(3)), &(p2(n.clone()) * &three)),
        &p2(n.clone() + Integer::ONE)
    ));
    let q = rem_and_quotient_bits_exact_all(
        &p2(n.clone() + Integer::from(3)),
        &(p2(n.clone()) * &three),
    )
    .1;
    assert_eq!(q, 2);
    let r = rem_prec_round_all(&p2(n + Integer::ONE), &BigExponentFloat::from(7), 1, Exact);
    // 2^k mod 7 is 1, 2, 4 for k mod 3 = 0, 1, 2; 10^20 + 1 = 2 mod 3, since 10 = 1 mod 3, so the
    // remainder is 4.
    assert!(same_pair(&r, &(BigExponentFloat::from(4), Equal)));
}

#[test]
#[should_panic]
fn rem_prec_round_fail_1() {
    BigExponentFloat::ONE.rem_prec_round(BigExponentFloat::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn rem_prec_round_fail_2() {
    BigExponentFloat::from(10.5).rem_prec_round(BigExponentFloat::from(3), 1, Exact);
}

// The exact quotient and remainder of x / y, with the quotient rounded toward zero, or to nearest
// with ties to even, for small exponents.
fn exact_rem(x: &BigExponentFloat, y: &BigExponentFloat, nearest: bool) -> (Rational, Integer) {
    let qx = Rational::exact_from(x);
    let qy = Rational::exact_from(y);
    let q = Integer::rounding_from(&qx / &qy, if nearest { Nearest } else { Down }).0;
    (qx - qy * Rational::from(&q), q)
}

#[test]
fn rem_prec_round_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_9()
        .test_properties(|(x, y, prec, rm)| {
            let r = rem_and_quotient_bits_prec_round_all(&x, &y, prec, rm);
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            }
            if x.is_finite()
                && !x.is_zero()
                && y.is_finite()
                && !y.is_zero()
                && big_exponent_float_exponent_is_small(&x)
                && big_exponent_float_exponent_is_small(&y)
            {
                let (exact, q) = exact_rem(&x, &y, false);
                assert_eq!(Rational::exact_from(&r.0).cmp(&exact), r.1);
                let low = i64::exact_from(&q.unsigned_abs_ref().mod_power_of_2(63));
                assert_eq!(r.2, if q < 0u32 { -low } else { low });
            }
        });
}

#[test]
fn ieee_remainder_prec_round_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_10()
        .test_properties(|(x, y, prec, rm)| {
            let r = ieee_remainder_prec_round_all(&x, &y, prec, rm);
            if x.is_finite()
                && !x.is_zero()
                && y.is_finite()
                && !y.is_zero()
                && big_exponent_float_exponent_is_small(&x)
                && big_exponent_float_exponent_is_small(&y)
            {
                let exact = exact_rem(&x, &y, true).0;
                assert_eq!(Rational::exact_from(&r.0).cmp(&exact), r.1);
            }
        });
}

#[test]
fn rem_properties() {
    big_exponent_float_pair_gen().test_properties(|(x, y)| {
        let r = rem_exact_all(&x, &y);
        let ieee = ieee_remainder_exact_all(&x, &y);
        if r.is_finite() && !r.is_zero() {
            // The remainder has the sign of x and is smaller than y in magnitude.
            assert_eq!(r.is_sign_positive(), x.is_sign_positive());
            assert!(r.lt_abs(&y));
            // Multiplying both arguments by 2 doubles the remainders.
            assert!(same(&((&x << 1u32) % (&y << 1u32)), &(&r << 1u32)));
        }
        if ieee.is_finite() && y.is_finite() {
            // The IEEE remainder is at most |y| / 2 in magnitude, and differs from the remainder by
            // 0 or y in magnitude.
            assert!(!(&ieee << 1u32).gt_abs(&y));
        }
    });
}

#[test]
fn rem_prec_round_float_agreement_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_9_float()
        .test_properties(|((fx, fy, prec, rm), (x, y, _, _))| {
            let (f, fo, fq) = fx.rem_and_quotient_bits_prec_round_ref_ref(&fy, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = rem_and_quotient_bits_prec_round_all(&x, &y, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
                assert_eq!(r.2, fq);
            }
        });
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_10_float()
        .test_properties(|((fx, fy, prec, rm), (x, y, _, _))| {
            let (f, fo) = fx.ieee_remainder_prec_round_ref_ref(&fy, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = ieee_remainder_prec_round_all(&x, &y, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        });
}

fn same_triple(
    x: &(BigExponentFloat, Ordering, i64),
    y: &(BigExponentFloat, Ordering, i64),
) -> bool {
    same(&x.0, &y.0) && x.1 == y.1 && x.2 == y.2
}

fn ieee_remainder_and_quotient_bits_prec_round_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering, i64) {
    let r = x
        .clone()
        .ieee_remainder_and_quotient_bits_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_triple(
        &x.clone()
            .ieee_remainder_and_quotient_bits_prec_round_val_ref(y, prec, rm),
        &r
    ));
    assert!(same_triple(
        &x.ieee_remainder_and_quotient_bits_prec_round_ref_val(y.clone(), prec, rm),
        &r
    ));
    assert!(same_triple(
        &x.ieee_remainder_and_quotient_bits_prec_round_ref_ref(y, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_triple(
            &x.clone().ieee_remainder_and_quotient_bits_prec(y.clone(), prec),
            &r
        ));
        assert!(same_triple(
            &x.clone().ieee_remainder_and_quotient_bits_prec_val_ref(y, prec),
            &r
        ));
        assert!(same_triple(
            &x.ieee_remainder_and_quotient_bits_prec_ref_val(y.clone(), prec),
            &r
        ));
        assert!(same_triple(
            &x.ieee_remainder_and_quotient_bits_prec_ref_ref(y, prec),
            &r
        ));
    }
    r
}

fn ieee_remainder_and_quotient_bits_exact_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
) -> (BigExponentFloat, i64) {
    let r = x.clone().ieee_remainder_and_quotient_bits(y.clone());
    assert!(r.0.is_valid());
    let same2 = |a: &(BigExponentFloat, i64)| same(&a.0, &r.0) && a.1 == r.1;
    assert!(same2(&x.clone().ieee_remainder_and_quotient_bits_val_ref(y)));
    assert!(same2(&x.ieee_remainder_and_quotient_bits_ref_val(y.clone())));
    assert!(same2(&x.ieee_remainder_and_quotient_bits_ref_ref(y)));
    r
}

fn rem_unsigned_prec_round_all(
    x: &BigExponentFloat,
    n: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().rem_unsigned_prec_round(n, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.rem_unsigned_prec_round_ref(n, prec, rm), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().rem_unsigned_prec(n, prec), &r));
        assert!(same_pair(&x.rem_unsigned_prec_ref(n, prec), &r));
    }
    r
}

fn rem_unsigned_exact_all(x: &BigExponentFloat, n: u64) -> BigExponentFloat {
    let r = x.clone().rem_unsigned(n);
    assert!(r.is_valid());
    assert!(same(&x.rem_unsigned_ref(n), &r));
    r
}

fn rem_rational_all(
    x: &BigExponentFloat,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering, i64) {
    let r = x.clone().rem_rational_and_quotient_bits_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_triple(&x.clone().rem_rational_and_quotient_bits_prec_round_val_ref(y, prec, rm), &r));
    assert!(same_triple(&x.rem_rational_and_quotient_bits_prec_round_ref_val(y.clone(), prec, rm), &r));
    assert!(same_triple(&x.rem_rational_and_quotient_bits_prec_round_ref_ref(y, prec, rm), &r));
    let p = (r.0.clone(), r.1);
    assert!(same_pair(&x.clone().rem_rational_prec_round(y.clone(), prec, rm), &p));
    assert!(same_pair(&x.clone().rem_rational_prec_round_val_ref(y, prec, rm), &p));
    assert!(same_pair(&x.rem_rational_prec_round_ref_val(y.clone(), prec, rm), &p));
    assert!(same_pair(&x.rem_rational_prec_round_ref_ref(y, prec, rm), &p));
    let mut z = x.clone();
    let o = z.rem_rational_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &p));
    let mut z = x.clone();
    let o = z.rem_rational_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &p));
    if rm == Nearest {
        assert!(same_triple(&x.clone().rem_rational_and_quotient_bits_prec(y.clone(), prec), &r));
        assert!(same_triple(&x.clone().rem_rational_and_quotient_bits_prec_val_ref(y, prec), &r));
        assert!(same_triple(&x.rem_rational_and_quotient_bits_prec_ref_val(y.clone(), prec), &r));
        assert!(same_triple(&x.rem_rational_and_quotient_bits_prec_ref_ref(y, prec), &r));
        assert!(same_pair(&x.clone().rem_rational_prec(y.clone(), prec), &p));
        assert!(same_pair(&x.clone().rem_rational_prec_val_ref(y, prec), &p));
        assert!(same_pair(&x.rem_rational_prec_ref_val(y.clone(), prec), &p));
        assert!(same_pair(&x.rem_rational_prec_ref_ref(y, prec), &p));
        let mut z = x.clone();
        let o = z.rem_rational_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &p));
        let mut z = x.clone();
        let o = z.rem_rational_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &p));
    }
    r
}

fn ieee_remainder_rational_all(
    x: &BigExponentFloat,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering, i64) {
    let r = x.clone().ieee_remainder_rational_and_quotient_bits_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_triple(&x.clone().ieee_remainder_rational_and_quotient_bits_prec_round_val_ref(y, prec, rm), &r));
    assert!(same_triple(&x.ieee_remainder_rational_and_quotient_bits_prec_round_ref_val(y.clone(), prec, rm), &r));
    assert!(same_triple(&x.ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(y, prec, rm), &r));
    let p = (r.0.clone(), r.1);
    assert!(same_pair(&x.clone().ieee_remainder_rational_prec_round(y.clone(), prec, rm), &p));
    assert!(same_pair(&x.clone().ieee_remainder_rational_prec_round_val_ref(y, prec, rm), &p));
    assert!(same_pair(&x.ieee_remainder_rational_prec_round_ref_val(y.clone(), prec, rm), &p));
    assert!(same_pair(&x.ieee_remainder_rational_prec_round_ref_ref(y, prec, rm), &p));
    let mut z = x.clone();
    let o = z.ieee_remainder_rational_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &p));
    let mut z = x.clone();
    let o = z.ieee_remainder_rational_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &p));
    if rm == Nearest {
        assert!(same_triple(&x.clone().ieee_remainder_rational_and_quotient_bits_prec(y.clone(), prec), &r));
        assert!(same_triple(&x.clone().ieee_remainder_rational_and_quotient_bits_prec_val_ref(y, prec), &r));
        assert!(same_triple(&x.ieee_remainder_rational_and_quotient_bits_prec_ref_val(y.clone(), prec), &r));
        assert!(same_triple(&x.ieee_remainder_rational_and_quotient_bits_prec_ref_ref(y, prec), &r));
        assert!(same_pair(&x.clone().ieee_remainder_rational_prec(y.clone(), prec), &p));
        assert!(same_pair(&x.clone().ieee_remainder_rational_prec_val_ref(y, prec), &p));
        assert!(same_pair(&x.ieee_remainder_rational_prec_ref_val(y.clone(), prec), &p));
        assert!(same_pair(&x.ieee_remainder_rational_prec_ref_ref(y, prec), &p));
        let mut z = x.clone();
        let o = z.ieee_remainder_rational_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &p));
        let mut z = x.clone();
        let o = z.ieee_remainder_rational_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &p));
    }
    r
}

fn rational_rem_all(
    x: &Rational,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering, i64) {
    let r = BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round(x.clone(), y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_triple(&BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_val_ref(x.clone(), y, prec, rm), &r));
    assert!(same_triple(&BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_val(x, y.clone(), prec, rm), &r));
    assert!(same_triple(&BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_ref(x, y, prec, rm), &r));
    let p = (r.0.clone(), r.1);
    assert!(same_pair(&BigExponentFloat::rational_rem_big_exponent_float_prec_round(x.clone(), y.clone(), prec, rm), &p));
    assert!(same_pair(&BigExponentFloat::rational_rem_big_exponent_float_prec_round_val_ref(x.clone(), y, prec, rm), &p));
    assert!(same_pair(&BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_val(x, y.clone(), prec, rm), &p));
    assert!(same_pair(&BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_ref(x, y, prec, rm), &p));
    if rm == Nearest {
        assert!(same_triple(&BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec(x.clone(), y.clone(), prec), &r));
        assert!(same_triple(&BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_val_ref(x.clone(), y, prec), &r));
        assert!(same_triple(&BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_ref_val(x, y.clone(), prec), &r));
        assert!(same_triple(&BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_ref_ref(x, y, prec), &r));
        assert!(same_pair(&BigExponentFloat::rational_rem_big_exponent_float_prec(x.clone(), y.clone(), prec), &p));
        assert!(same_pair(&BigExponentFloat::rational_rem_big_exponent_float_prec_val_ref(x.clone(), y, prec), &p));
        assert!(same_pair(&BigExponentFloat::rational_rem_big_exponent_float_prec_ref_val(x, y.clone(), prec), &p));
        assert!(same_pair(&BigExponentFloat::rational_rem_big_exponent_float_prec_ref_ref(x, y, prec), &p));
    }
    r
}

fn rational_ieee_remainder_all(
    x: &Rational,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering, i64) {
    let r = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round(x.clone(), y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_triple(&BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_val_ref(x.clone(), y, prec, rm), &r));
    assert!(same_triple(&BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_val(x, y.clone(), prec, rm), &r));
    assert!(same_triple(&BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_ref(x, y, prec, rm), &r));
    let p = (r.0.clone(), r.1);
    assert!(same_pair(&BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round(x.clone(), y.clone(), prec, rm), &p));
    assert!(same_pair(&BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_val_ref(x.clone(), y, prec, rm), &p));
    assert!(same_pair(&BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_val(x, y.clone(), prec, rm), &p));
    assert!(same_pair(&BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_ref(x, y, prec, rm), &p));
    if rm == Nearest {
        assert!(same_triple(&BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec(x.clone(), y.clone(), prec), &r));
        assert!(same_triple(&BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_val_ref(x.clone(), y, prec), &r));
        assert!(same_triple(&BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_ref_val(x, y.clone(), prec), &r));
        assert!(same_triple(&BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_ref_ref(x, y, prec), &r));
        assert!(same_pair(&BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec(x.clone(), y.clone(), prec), &p));
        assert!(same_pair(&BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_val_ref(x.clone(), y, prec), &p));
        assert!(same_pair(&BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_ref_val(x, y.clone(), prec), &p));
        assert!(same_pair(&BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_ref_ref(x, y, prec), &p));
    }
    r
}

// The exact remainder of x / y and the quotient, for `Rational`s, with the quotient rounded toward
// zero, or to nearest with ties to even.
fn exact_rational_rem(x: &Rational, y: &Rational, nearest: bool) -> (Rational, Integer) {
    let q = Integer::rounding_from(x / y, if nearest { Nearest } else { Down }).0;
    (x - y * Rational::from(&q), q)
}

// ±(|q| mod 2^63), with q's sign.
fn low_quotient_bits(q: &Integer) -> i64 {
    let low = i64::exact_from(&q.unsigned_abs_ref().mod_power_of_2(63));
    if *q < 0u32 { -low } else { low }
}

// Checks a remainder, its `Ordering`, and its quotient bits against the exact `Rational` values.
fn check_rational_rem(
    x: &Rational,
    y: &Rational,
    nearest: bool,
    prec: u64,
    rm: RoundingMode,
    r: &(BigExponentFloat, Ordering, i64),
) {
    let (exact, q) = exact_rational_rem(x, y, nearest);
    let expected = BigExponentFloat::from_rational_prec_round(exact, prec, rm);
    assert!(same_pair(&(r.0.clone(), r.1), &expected));
    assert_eq!(r.2, low_quotient_bits(&q));
}

// -2 * 3^-1 mod 2^63, as a signed value: the low quotient bits of (2^k - 2) / 3 for k >= 63.
fn minus_two_thirds_mod_2_63() -> i64 {
    let m = Natural::power_of_2(63);
    let inv = Natural::from(3u32).mod_inverse(&m).unwrap();
    i64::exact_from(&(&m - Natural::TWO).mod_mul(inv, &m))
}

#[test]
fn test_rem_rational() {
    let huge_odd = Integer::from(i64::MAX);
    let third = Rational::from_unsigneds(1u8, 3);
    // 2^k mod 3 is 2 for odd k, so (2^k - 2) / 3 is the quotient.
    let r = rem_rational_all(&p2(huge_odd.clone()), &Rational::from(3), 10, Nearest);
    assert!(same(&r.0, &BigExponentFloat::from(2)));
    assert_eq!(r.1, Equal);
    assert_eq!(r.2, minus_two_thirds_mod_2_63());
    // The nearest quotient rounds up, since 2 > 3 / 2, leaving 2 - 3.
    let r = ieee_remainder_rational_all(&p2(huge_odd.clone()), &Rational::from(3), 10, Nearest);
    assert!(same(&r.0, &BigExponentFloat::from(-1)));
    assert_eq!(r.2, minus_two_thirds_mod_2_63() + 1);
    // 2^k mod 3/2 = (2^(k + 1) mod 3) / 2, and k + 1 is even.
    let r = rem_rational_all(&p2(huge_odd.clone()), &Rational::from_unsigneds(3u8, 2), 10, Nearest);
    assert!(same(&r.0, &BigExponentFloat::from(0.5)));
    // 2^N mod 5 for N = 10^20, computed independently.
    let expected =
        Natural::TWO.mod_pow(Natural::exact_from(&big()), Natural::from(5u32));
    let r = rem_rational_all(&p2(big()), &Rational::from(5), 10, Nearest);
    assert!(same(&r.0, &BigExponentFloat::from(&expected)));
    // A tiny x is its own truncated and nearest remainder.
    let tiny = p2(-big());
    let r = rem_rational_all(&tiny, &third, 10, Nearest);
    assert!(same(&r.0, &tiny));
    assert_eq!((r.1, r.2), (Equal, 0));
    let r = ieee_remainder_rational_all(&-&tiny, &third, 10, Nearest);
    assert!(same(&r.0, &-&tiny));
    // Signs: the remainder has x's sign, and the quotient bits have the quotient's sign.
    let r = rem_rational_all(&BigExponentFloat::from(-10.5), &Rational::from(3), 10, Nearest);
    assert!(same(&r.0, &BigExponentFloat::from(-1.5)));
    assert_eq!(r.2, -3);
    let r = rem_rational_all(&BigExponentFloat::from(10.5), &Rational::from(-3), 10, Nearest);
    assert!(same(&r.0, &BigExponentFloat::from(1.5)));
    assert_eq!(r.2, -3);
    let r = ieee_remainder_rational_all(&BigExponentFloat::from(10.5), &Rational::from(3), 10, Nearest);
    assert!(same(&r.0, &BigExponentFloat::from(-1.5)));
    assert_eq!(r.2, 4);
    // Special cases
    for x in [BigExponentFloat::NAN, BigExponentFloat::INFINITY, BigExponentFloat::NEGATIVE_INFINITY] {
        let r = rem_rational_all(&x, &third, 10, Nearest);
        assert!(r.0.is_nan());
        assert_eq!((r.1, r.2), (Equal, 0));
    }
    let r = rem_rational_all(&BigExponentFloat::ONE, &Rational::ZERO, 10, Nearest);
    assert!(r.0.is_nan());
    let r = rem_rational_all(&BigExponentFloat::ZERO, &third, 10, Nearest);
    assert!(same(&r.0, &BigExponentFloat::ZERO));
}

#[test]
fn test_rational_rem() {
    let t = Integer::from(i64::MAX);
    let tiny = p2(-t.clone());
    let third = Rational::from_unsigneds(1u8, 3);
    // 1/3 mod 2^-t = ((2^t mod 3) / 3) * 2^-t = (2/3) * 2^-t for odd t, and the quotient is
    // (2^t - 2) / 3.
    for (prec, rm) in [(10, Nearest), (5, Floor), (5, Ceiling)] {
        let r = rational_rem_all(&third, &tiny, prec, rm);
        let (e, o) =
            BigExponentFloat::from_rational_prec_round(Rational::from_unsigneds(2u8, 3), prec, rm);
        assert!(same(&r.0, &(e * &tiny)));
        assert_eq!(r.1, o);
        assert_eq!(r.2, minus_two_thirds_mod_2_63());
    }
    // The nearest quotient rounds up, since 2/3 > 1/2, leaving -(1/3) * 2^-t.
    let r = rational_ieee_remainder_all(&third, &tiny, 10, Nearest);
    let e = BigExponentFloat::from_rational_prec_round(Rational::from_signeds(-1i8, 3), 10, Nearest).0;
    assert!(same(&r.0, &(e * &tiny)));
    assert_eq!(r.2, minus_two_thirds_mod_2_63() + 1);
    // 1/5 mod 2^-N = ((2^N mod 5) / 5) * 2^-N for N = 10^20.
    let k = Natural::TWO.mod_pow(Natural::exact_from(&big()), Natural::from(5u32));
    let r = rational_rem_all(&Rational::from_unsigneds(1u8, 5), &p2(-big()), 10, Nearest);
    let e = BigExponentFloat::from_rational_prec_round(Rational::from_naturals(k, Natural::from(5u32)), 10, Nearest).0;
    assert!(same(&r.0, &(e * p2(-big()))));
    // A huge y leaves x.
    let r = rational_rem_all(&third, &p2(big()), 10, Nearest);
    assert!(same_pair(&(r.0, r.1), &BigExponentFloat::from_rational_prec_round(third.clone(), 10, Nearest)));
    // Special cases
    for y in [BigExponentFloat::NAN, BigExponentFloat::ZERO] {
        let r = rational_rem_all(&third, &y, 10, Nearest);
        assert!(r.0.is_nan());
        assert_eq!((r.1, r.2), (Equal, 0));
    }
    for y in [BigExponentFloat::INFINITY, BigExponentFloat::NEGATIVE_INFINITY] {
        let r = rational_rem_all(&third, &y, 10, Nearest);
        assert!(same_pair(&(r.0, r.1), &BigExponentFloat::from_rational_prec_round(third.clone(), 10, Nearest)));
        assert_eq!(r.2, 0);
    }
    let r = rational_rem_all(&Rational::ZERO, &BigExponentFloat::ONE, 10, Nearest);
    assert!(same(&r.0, &BigExponentFloat::ZERO));
}

#[test]
fn test_rem_unsigned_and_ieee_quotient_bits() {
    assert!(same(&rem_unsigned_exact_all(&BigExponentFloat::from(10.5), 3), &BigExponentFloat::from(1.5)));
    assert!(same(&rem_unsigned_exact_all(&BigExponentFloat::from(-10.5), 3), &BigExponentFloat::from(-1.5)));
    assert!(same(&rem_unsigned_exact_all(&p2(Integer::from(i64::MAX)), 3), &BigExponentFloat::from(2)));
    assert!(rem_unsigned_exact_all(&BigExponentFloat::from(10.5), 0).is_nan());
    assert!(rem_unsigned_exact_all(&BigExponentFloat::INFINITY, 3).is_nan());
    assert!(rem_unsigned_exact_all(&BigExponentFloat::NAN, 3).is_nan());
    assert!(same(&rem_unsigned_exact_all(&BigExponentFloat::ZERO, 3), &BigExponentFloat::ZERO));
    let (r, q) = ieee_remainder_and_quotient_bits_exact_all(&p2(Integer::from(i64::MAX)), &BigExponentFloat::from(3));
    assert!(same(&r, &BigExponentFloat::from(-1)));
    assert_eq!(q, minus_two_thirds_mod_2_63() + 1);
    let (r, q) = ieee_remainder_and_quotient_bits_exact_all(&BigExponentFloat::from(-10.5), &BigExponentFloat::from(3));
    assert!(same(&r, &BigExponentFloat::from(1.5)));
    assert_eq!(q, -4);
}

#[test]
fn rem_rational_prec_round_properties() {
    for nearest in [false, true] {
        let generator = if nearest {
            big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_10()
        } else {
            big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_9()
        };
        generator.test_properties(|(x, y, prec, rm)| {
            let r = if nearest {
                ieee_remainder_rational_all(&x, &y, prec, rm)
            } else {
                rem_rational_all(&x, &y, prec, rm)
            };
            if r.0.is_nan() || r.0.is_zero() {
                assert_eq!(r.1, Equal);
            }
            if x.is_finite() && !x.is_zero() && y != 0u32 && big_exponent_float_exponent_is_small(&x) {
                check_rational_rem(&Rational::exact_from(&x), &y, nearest, prec, rm, &r);
            }
        });
    }
}

#[test]
fn rational_rem_prec_round_properties() {
    for nearest in [false, true] {
        let generator = if nearest {
            big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_12()
        } else {
            big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_11()
        };
        generator.test_properties(|(y, x, prec, rm)| {
            let r = if nearest {
                rational_ieee_remainder_all(&x, &y, prec, rm)
            } else {
                rational_rem_all(&x, &y, prec, rm)
            };
            if r.0.is_nan() || r.0.is_zero() {
                assert_eq!(r.1, Equal);
            }
            if y.is_finite() && !y.is_zero() && big_exponent_float_exponent_is_small(&y) {
                check_rational_rem(&x, &Rational::exact_from(&y), nearest, prec, rm, &r);
            }
        });
    }
}

#[test]
fn rem_unsigned_prec_round_properties() {
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_3().test_properties(
        |(x, n, prec, rm)| {
            let r = rem_unsigned_prec_round_all(&x, n, prec, rm);
            assert!(same_pair(&r, &x.rem_prec_round_ref_val(BigExponentFloat::from(n), prec, rm)));
        },
    );
    big_exponent_float_unsigned_pair_gen::<u64>().test_properties(|(x, n)| {
        let r = rem_unsigned_exact_all(&x, n);
        assert!(same(&r, &(&x % BigExponentFloat::from(n))));
    });
}

#[test]
fn ieee_remainder_and_quotient_bits_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_10()
        .test_properties(|(x, y, prec, rm)| {
            let r = ieee_remainder_and_quotient_bits_prec_round_all(&x, &y, prec, rm);
            assert!(same_pair(&(r.0.clone(), r.1), &x.ieee_remainder_prec_round_ref_ref(&y, prec, rm)));
            if x.is_finite()
                && !x.is_zero()
                && y.is_finite()
                && !y.is_zero()
                && big_exponent_float_exponent_is_small(&x)
                && big_exponent_float_exponent_is_small(&y)
            {
                assert_eq!(r.2, low_quotient_bits(&exact_rem(&x, &y, true).1));
            }
        });
    big_exponent_float_pair_gen().test_properties(|(x, y)| {
        let (r, q) = ieee_remainder_and_quotient_bits_exact_all(&x, &y);
        assert!(same(&r, &x.ieee_remainder_ref_ref(&y)));
        // The nearest quotient is the truncated one, or one more in magnitude.
        let (_, tq) = x.rem_and_quotient_bits_ref_ref(&y);
        let d = q.wrapping_sub(tq);
        assert!(d == 0 || d.unsigned_abs() == 1 || d.unsigned_abs() == (1 << 63) - 1);
    });
}

#[test]
fn rem_rational_float_agreement_properties() {
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_9_float().test_properties(
        |((fx, y, prec, rm), (x, _, _, _))| {
            let (f, fo, fq) = fx.rem_rational_and_quotient_bits_prec_round_ref_ref(&y, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = rem_rational_all(&x, &y, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!((r.1, r.2), (fo, fq));
            }
        },
    );
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_10_float().test_properties(
        |((fx, y, prec, rm), (x, _, _, _))| {
            let (f, fo, fq) =
                fx.ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(&y, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = ieee_remainder_rational_all(&x, &y, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!((r.1, r.2), (fo, fq));
            }
        },
    );
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_11_float().test_properties(
        |((fy, x, prec, rm), (y, _, _, _))| {
            let (f, fo, fq) =
                Float::rational_rem_float_and_quotient_bits_prec_round_ref_ref(&x, &fy, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = rational_rem_all(&x, &y, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!((r.1, r.2), (fo, fq));
            }
        },
    );
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_12_float().test_properties(
        |((fy, x, prec, rm), (y, _, _, _))| {
            let (f, fo, fq) = Float::rational_ieee_remainder_float_and_quotient_bits_prec_round_ref_ref(
                &x, &fy, prec, rm,
            );
            if float_result_comparable(&f, fo) {
                let r = rational_ieee_remainder_all(&x, &y, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!((r.1, r.2), (fo, fq));
            }
        },
    );
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_3_float().test_properties(
        |((fx, n, prec, rm), (x, _, _, _))| {
            let (f, fo) = fx.rem_unsigned_prec_round_ref(n, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = rem_unsigned_prec_round_all(&x, n, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_10_float()
        .test_properties(|((fx, fy, prec, rm), (x, y, _, _))| {
            let (f, fo, fq) = fx.ieee_remainder_and_quotient_bits_prec_round_ref_ref(&fy, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = ieee_remainder_and_quotient_bits_prec_round_all(&x, &y, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!((r.1, r.2), (fo, fq));
            }
        });
}
