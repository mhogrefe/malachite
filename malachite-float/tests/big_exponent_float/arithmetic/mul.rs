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
use malachite_float::big_exponent_float::arithmetic::mul::mul_prec_round_generic;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_negate, big_exponent_float_power_of_2,
    float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_4,
    big_exponent_float_pair_gen, float_float_unsigned_rounding_mode_quadruple_gen_var_3,
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

// Whether a `Float` result is one that a `BigExponentFloat` result must match: not an overflow or
// underflow, which a `BigExponentFloat` never has.
fn float_result_comparable(x: &Float, o: Ordering) -> bool {
    x.get_exponent().map_or_else(
        || !x.is_infinite() && (!x.is_zero() || o == Equal),
        |e| e > Float::MIN_EXPONENT + 1 && e < Float::MAX_EXPONENT - 1,
    )
}

// Every by-value, by-reference, and in-place form of `mul_prec_round` and `mul_prec` agrees, and
// the result is valid.
fn mul_prec_round_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().mul_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone().mul_prec_round_val_ref(y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.mul_prec_round_ref_val(y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(&x.mul_prec_round_ref_ref(y, prec, rm), &r));
    let mut z = x.clone();
    let o = z.mul_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = x.clone();
    let o = z.mul_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().mul_prec(y.clone(), prec), &r));
        assert!(same_pair(&x.clone().mul_prec_val_ref(y, prec), &r));
        assert!(same_pair(&x.mul_prec_ref_val(y.clone(), prec), &r));
        assert!(same_pair(&x.mul_prec_ref_ref(y, prec), &r));
        let mut z = x.clone();
        let o = z.mul_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = x.clone();
        let o = z.mul_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

// Every form of the exact `*` agrees, and the result is valid.
fn mul_all(x: &BigExponentFloat, y: &BigExponentFloat) -> BigExponentFloat {
    let r = x.clone() * y.clone();
    assert!(r.is_valid());
    assert!(same(&(x.clone() * y), &r));
    assert!(same(&(x * y.clone()), &r));
    assert!(same(&(x * y), &r));
    let mut z = x.clone();
    z *= y.clone();
    assert!(same(&z, &r));
    let mut z = x.clone();
    z *= y;
    assert!(same(&z, &r));
    r
}

#[test]
fn test_mul() {
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let tiny = big_exponent_float_power_of_2(-big.clone());
    let one = BigExponentFloat::ONE;
    let pi = parse_big_exponent_float_hex("0x3.243f6a8885a30#53");
    let e = parse_big_exponent_float_hex("0x2.b7e151628aed2#53");
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    let check = |x: &BigExponentFloat,
                 y: &BigExponentFloat,
                 prec: u64,
                 rm: RoundingMode,
                 out: &BigExponentFloat,
                 o: Ordering| {
        let r = mul_prec_round_all(x, y, prec, rm);
        assert!(
            same(&r.0, out),
            "{x:?} {y:?} {prec} {rm}: {:?} != {out:?}",
            r.0
        );
        assert_eq!(r.1, o);
    };

    check(&nan, &one, 10, Floor, &nan, Equal);
    check(&inf, &zero, 10, Floor, &nan, Equal);
    check(&zero, &ninf, 10, Floor, &nan, Equal);
    check(
        &inf,
        &big_exponent_float_negate(one.clone()),
        10,
        Floor,
        &ninf,
        Equal,
    );
    check(&ninf, &ninf, 10, Floor, &inf, Equal);
    check(&zero, &pi, 10, Floor, &zero, Equal);
    // Exponents add without overflow or underflow.
    check(&huge, &tiny, 1, Exact, &one, Equal);
    check(
        &huge,
        &huge,
        1,
        Exact,
        &big_exponent_float_power_of_2(big.clone() * Integer::TWO),
        Equal,
    );
    check(
        &pi,
        &e,
        5,
        Floor,
        &parse_big_exponent_float_hex("0x8.8#5"),
        Less,
    );
    check(
        &pi,
        &e,
        5,
        Ceiling,
        &parse_big_exponent_float_hex("0x9.0#5"),
        Greater,
    );
    check(
        &pi,
        &e,
        20,
        Nearest,
        &parse_big_exponent_float_hex("0x8.8a2c#20"),
        Less,
    );
    let p = mul_all(&pi, &e);
    assert_eq!(
        Rational::exact_from(&p),
        Rational::exact_from(&pi) * Rational::exact_from(&e)
    );
    assert!(same(&mul_all(&huge, &tiny), &one));
    assert!(same(&mul_all(&inf, &zero), &nan));
}

#[test]
#[should_panic]
fn mul_prec_round_fail_1() {
    BigExponentFloat::ONE.mul_prec_round(BigExponentFloat::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn mul_prec_round_fail_2() {
    let x = BigExponentFloat::from(1.5);
    let y = BigExponentFloat::from(1.0 / 3.0);
    x.mul_prec_round(y, 1, Exact);
}

#[test]
#[should_panic]
fn mul_prec_fail() {
    BigExponentFloat::ONE.mul_prec(BigExponentFloat::ONE, 0);
}

#[test]
fn mul_prec_round_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_4()
        .test_properties(|(x, y, prec, rm)| {
            let r = mul_prec_round_all(&x, &y, prec, rm);
            // The kernels agree with the kernel-free implementation, at any exponents.
            assert!(same_pair(&mul_prec_round_generic(&x, &y, prec, rm), &r));
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // (-x)y = -(xy), with the rounding mode and the ordering reversed, and xy = yx.
            let (s, o) = big_exponent_float_negate(x.clone()).mul_prec_round_val_ref(&y, prec, -rm);
            assert!(same(&big_exponent_float_negate(s), &r.0));
            assert_eq!(o.reverse(), r.1);
            assert!(same_pair(&y.mul_prec_round_ref_ref(&x, prec, rm), &r));
            if big_exponent_float_exponent_is_small(&x) && big_exponent_float_exponent_is_small(&y)
            {
                // Rounding the exact result gives the same value and ordering.
                let exact = mul_all(&x, &y);
                assert!(same_pair(&exact.round_to_prec_round_ref(prec, rm), &r));
                if exact.is_finite() {
                    let q = Rational::exact_from(&x) * Rational::exact_from(&y);
                    assert_eq!(Rational::exact_from(&exact), q);
                    assert_eq!(Rational::exact_from(&r.0).cmp(&q), r.1);
                }
            }
        });
}

#[test]
fn mul_properties() {
    big_exponent_float_pair_gen().test_properties(|(x, y)| {
        if big_exponent_float_exponent_is_small(&x) && big_exponent_float_exponent_is_small(&y) {
            let r = mul_all(&x, &y);
            if let (Ok(qx), Ok(qy)) = (Rational::try_from(&x), Rational::try_from(&y)) {
                assert_eq!(Rational::exact_from(&r), qx * qy);
            }
        }
    });
}

#[test]
fn mul_prec_round_float_agreement_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_3().test_properties(|(x, y, prec, rm)| {
        let (f, fo) = x.mul_prec_round_ref_ref(&y, prec, rm);
        if float_result_comparable(&f, fo) {
            let bx = float_to_big_exponent_float(&x);
            let by = float_to_big_exponent_float(&y);
            let r = mul_prec_round_all(&bx, &by, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
