// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::add_mul::add_mul_prec_round_generic;
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

fn to_bef(x: &Float) -> BigExponentFloat {
    float_to_big_exponent_float(x)
}

fn add_mul_prec_round_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    z: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().add_mul_prec_round(y.clone(), z.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone()
            .add_mul_prec_round_val_val_ref(y.clone(), z, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.clone()
            .add_mul_prec_round_val_ref_val(y, z.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.clone().add_mul_prec_round_val_ref_ref(y, z, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.add_mul_prec_round_ref_val_val(y.clone(), z.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.add_mul_prec_round_ref_val_ref(y.clone(), z, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.add_mul_prec_round_ref_ref_val(y, z.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.add_mul_prec_round_ref_ref_ref(y, z, prec, rm),
        &r
    ));
    let mut t = x.clone();
    let o = t.add_mul_prec_round_assign(y.clone(), z.clone(), prec, rm);
    assert!(same_pair(&(t, o), &r));
    let mut t = x.clone();
    let o = t.add_mul_prec_round_assign_val_ref(y.clone(), z, prec, rm);
    assert!(same_pair(&(t, o), &r));
    let mut t = x.clone();
    let o = t.add_mul_prec_round_assign_ref_val(y, z.clone(), prec, rm);
    assert!(same_pair(&(t, o), &r));
    let mut t = x.clone();
    let o = t.add_mul_prec_round_assign_ref_ref(y, z, prec, rm);
    assert!(same_pair(&(t, o), &r));
    if rm == Nearest {
        assert!(same_pair(
            &x.clone().add_mul_prec(y.clone(), z.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.clone().add_mul_prec_val_val_ref(y.clone(), z, prec),
            &r
        ));
        assert!(same_pair(
            &x.clone().add_mul_prec_val_ref_val(y, z.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.clone().add_mul_prec_val_ref_ref(y, z, prec),
            &r
        ));
        assert!(same_pair(
            &x.add_mul_prec_ref_val_val(y.clone(), z.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.add_mul_prec_ref_val_ref(y.clone(), z, prec),
            &r
        ));
        assert!(same_pair(
            &x.add_mul_prec_ref_ref_val(y, z.clone(), prec),
            &r
        ));
        assert!(same_pair(&x.add_mul_prec_ref_ref_ref(y, z, prec), &r));
        let mut t = x.clone();
        let o = t.add_mul_prec_assign(y.clone(), z.clone(), prec);
        assert!(same_pair(&(t, o), &r));
        let mut t = x.clone();
        let o = t.add_mul_prec_assign_val_ref(y.clone(), z, prec);
        assert!(same_pair(&(t, o), &r));
        let mut t = x.clone();
        let o = t.add_mul_prec_assign_ref_val(y, z.clone(), prec);
        assert!(same_pair(&(t, o), &r));
        let mut t = x.clone();
        let o = t.add_mul_prec_assign_ref_ref(y, z, prec);
        assert!(same_pair(&(t, o), &r));
    }
    r
}

fn add_mul_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    z: &BigExponentFloat,
) -> BigExponentFloat {
    let r = x.clone().add_mul(y.clone(), z.clone());
    assert!(r.is_valid());
    assert!(same(&x.clone().add_mul(y.clone(), z), &r));
    assert!(same(&x.clone().add_mul(y, z.clone()), &r));
    assert!(same(&x.clone().add_mul(y, z), &r));
    assert!(same(&x.add_mul(y.clone(), z.clone()), &r));
    assert!(same(&x.add_mul(y.clone(), z), &r));
    assert!(same(&x.add_mul(y, z.clone()), &r));
    assert!(same(&x.add_mul(y, z), &r));
    let mut t = x.clone();
    t.add_mul_assign(y.clone(), z.clone());
    assert!(same(&t, &r));
    let mut t = x.clone();
    t.add_mul_assign(y.clone(), z);
    assert!(same(&t, &r));
    let mut t = x.clone();
    t.add_mul_assign(y, z.clone());
    assert!(same(&t, &r));
    let mut t = x.clone();
    t.add_mul_assign(y, z);
    assert!(same(&t, &r));
    r
}

#[test]
fn test_add_mul_prec_round() {
    let check = |x: &BigExponentFloat,
                 y: &BigExponentFloat,
                 z: &BigExponentFloat,
                 prec: u64,
                 rm: RoundingMode,
                 out: &BigExponentFloat,
                 o| {
        let r = add_mul_prec_round_all(x, y, z, prec, rm);
        assert!(same(&r.0, out), "{:?} != {out:?}", r.0);
        assert_eq!(r.1, o);
    };
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let tiny = big_exponent_float_power_of_2(-big);
    let one = BigExponentFloat::ONE;
    let neg_one = BigExponentFloat::from(-1.0);
    // The huge terms cancel exactly.
    check(
        &huge,
        &neg_one,
        &huge,
        10,
        Exact,
        &BigExponentFloat::ZERO,
        Equal,
    );
    check(
        &one,
        &huge,
        &tiny,
        1,
        Exact,
        &BigExponentFloat::from(2.0),
        Equal,
    );
    // A tiny x is far below yz.
    check(&tiny, &huge, &huge, 10, Floor, &(&huge * &huge), Less);
    check(
        &BigExponentFloat::NAN,
        &one,
        &one,
        10,
        Floor,
        &BigExponentFloat::NAN,
        Equal,
    );
    check(
        &one,
        &BigExponentFloat::INFINITY,
        &BigExponentFloat::ZERO,
        10,
        Floor,
        &BigExponentFloat::NAN,
        Equal,
    );
    check(
        &BigExponentFloat::INFINITY,
        &BigExponentFloat::INFINITY,
        &neg_one,
        10,
        Floor,
        &BigExponentFloat::NAN,
        Equal,
    );
    check(
        &BigExponentFloat::from(1.5),
        &BigExponentFloat::from(0.5),
        &BigExponentFloat::from(3.0),
        2,
        Exact,
        &BigExponentFloat::from(3.0),
        Equal,
    );
}

#[test]
#[should_panic]
fn add_mul_prec_round_fail_1() {
    BigExponentFloat::ONE.add_mul_prec_round(
        BigExponentFloat::ONE,
        BigExponentFloat::ONE,
        0,
        Floor,
    );
}

#[test]
#[should_panic]
fn add_mul_prec_round_fail_2() {
    BigExponentFloat::from(3.0).add_mul_prec_round(
        BigExponentFloat::from(3.0),
        BigExponentFloat::from(3.0),
        1,
        Exact,
    );
}

#[test]
fn add_mul_prec_round_properties() {
    let generator = big_exponent_float_big_exponent_float_big_exponent_float_unsigned_rounding_mode_quintuple_gen_var_1;
    generator().test_properties(|(x, y, z, prec, rm)| {
        let r = add_mul_prec_round_all(&x, &y, &z, prec, rm);
        // The kernels agree with the kernel-free implementation, at any exponents.
        assert!(same_pair(
            &add_mul_prec_round_generic(&x, &y, &z, false, prec, rm),
            &r
        ));
        if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        // The product's factors commute.
        assert!(same_pair(
            &x.add_mul_prec_round_ref_ref_ref(&z, &y, prec, rm),
            &r
        ));
        if [&x, &y, &z]
            .iter()
            .all(|v| big_exponent_float_exponent_is_small(v))
            && [&x, &y, &z].iter().all(|v| v.is_finite())
        {
            let q = Rational::exact_from(&x) + Rational::exact_from(&y) * Rational::exact_from(&z);
            assert_eq!(Rational::exact_from(&r.0).cmp(&q), r.1);
        }
    });
}

#[test]
fn add_mul_properties() {
    big_exponent_float_triple_gen_var_1().test_properties(|(x, y, z)| {
        assert!(same(&add_mul_all(&x, &y, &z), &(&x + &(&y * &z))));
    });
}

#[test]
fn add_mul_prec_round_float_agreement_properties() {
    float_float_float_unsigned_rounding_mode_quintuple_gen_var_1().test_properties(
        |(x, y, z, prec, rm)| {
            let (f, fo) = x.add_mul_prec_round_ref_ref_ref(&y, &z, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = add_mul_prec_round_all(&to_bef(&x), &to_bef(&y), &to_bef(&z), prec, rm);
                assert!(same(&r.0, &to_bef(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
