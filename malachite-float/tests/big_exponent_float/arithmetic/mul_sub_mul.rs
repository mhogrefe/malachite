// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{MulSubMul, MulSubMulAssign};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::mul_add_mul::mul_add_mul_prec_round_generic;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::*;
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

fn mul_sub_mul_prec_round_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    z: &BigExponentFloat,
    w: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x
        .clone()
        .mul_sub_mul_prec_round(y.clone(), z.clone(), w.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone()
            .mul_sub_mul_prec_round_val_val_val_ref(y.clone(), z.clone(), w, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.clone()
            .mul_sub_mul_prec_round_val_val_ref_val(y.clone(), z, w.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.clone()
            .mul_sub_mul_prec_round_val_val_ref_ref(y.clone(), z, w, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.clone()
            .mul_sub_mul_prec_round_val_ref_val_val(y, z.clone(), w.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.clone()
            .mul_sub_mul_prec_round_val_ref_val_ref(y, z.clone(), w, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.clone()
            .mul_sub_mul_prec_round_val_ref_ref_val(y, z, w.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.clone()
            .mul_sub_mul_prec_round_val_ref_ref_ref(y, z, w, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.mul_sub_mul_prec_round_ref_ref_ref_ref(y, z, w, prec, rm),
        &r
    ));
    let mut t = x.clone();
    let o = t.mul_sub_mul_prec_round_assign(y.clone(), z.clone(), w.clone(), prec, rm);
    assert!(same_pair(&(t, o), &r));
    let mut t = x.clone();
    let o = t.mul_sub_mul_prec_round_assign_val_val_ref(y.clone(), z.clone(), w, prec, rm);
    assert!(same_pair(&(t, o), &r));
    let mut t = x.clone();
    let o = t.mul_sub_mul_prec_round_assign_val_ref_val(y.clone(), z, w.clone(), prec, rm);
    assert!(same_pair(&(t, o), &r));
    let mut t = x.clone();
    let o = t.mul_sub_mul_prec_round_assign_val_ref_ref(y.clone(), z, w, prec, rm);
    assert!(same_pair(&(t, o), &r));
    let mut t = x.clone();
    let o = t.mul_sub_mul_prec_round_assign_ref_val_val(y, z.clone(), w.clone(), prec, rm);
    assert!(same_pair(&(t, o), &r));
    let mut t = x.clone();
    let o = t.mul_sub_mul_prec_round_assign_ref_val_ref(y, z.clone(), w, prec, rm);
    assert!(same_pair(&(t, o), &r));
    let mut t = x.clone();
    let o = t.mul_sub_mul_prec_round_assign_ref_ref_val(y, z, w.clone(), prec, rm);
    assert!(same_pair(&(t, o), &r));
    let mut t = x.clone();
    let o = t.mul_sub_mul_prec_round_assign_ref_ref_ref(y, z, w, prec, rm);
    assert!(same_pair(&(t, o), &r));
    if rm == Nearest {
        assert!(same_pair(
            &x.clone()
                .mul_sub_mul_prec(y.clone(), z.clone(), w.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.clone()
                .mul_sub_mul_prec_val_val_val_ref(y.clone(), z.clone(), w, prec),
            &r
        ));
        assert!(same_pair(
            &x.clone()
                .mul_sub_mul_prec_val_val_ref_val(y.clone(), z, w.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.clone()
                .mul_sub_mul_prec_val_val_ref_ref(y.clone(), z, w, prec),
            &r
        ));
        assert!(same_pair(
            &x.clone()
                .mul_sub_mul_prec_val_ref_val_val(y, z.clone(), w.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.clone()
                .mul_sub_mul_prec_val_ref_val_ref(y, z.clone(), w, prec),
            &r
        ));
        assert!(same_pair(
            &x.clone()
                .mul_sub_mul_prec_val_ref_ref_val(y, z, w.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.clone().mul_sub_mul_prec_val_ref_ref_ref(y, z, w, prec),
            &r
        ));
        assert!(same_pair(
            &x.mul_sub_mul_prec_ref_ref_ref_ref(y, z, w, prec),
            &r
        ));
        let mut t = x.clone();
        let o = t.mul_sub_mul_prec_assign(y.clone(), z.clone(), w.clone(), prec);
        assert!(same_pair(&(t, o), &r));
        let mut t = x.clone();
        let o = t.mul_sub_mul_prec_assign_val_val_ref(y.clone(), z.clone(), w, prec);
        assert!(same_pair(&(t, o), &r));
        let mut t = x.clone();
        let o = t.mul_sub_mul_prec_assign_val_ref_val(y.clone(), z, w.clone(), prec);
        assert!(same_pair(&(t, o), &r));
        let mut t = x.clone();
        let o = t.mul_sub_mul_prec_assign_val_ref_ref(y.clone(), z, w, prec);
        assert!(same_pair(&(t, o), &r));
        let mut t = x.clone();
        let o = t.mul_sub_mul_prec_assign_ref_val_val(y, z.clone(), w.clone(), prec);
        assert!(same_pair(&(t, o), &r));
        let mut t = x.clone();
        let o = t.mul_sub_mul_prec_assign_ref_val_ref(y, z.clone(), w, prec);
        assert!(same_pair(&(t, o), &r));
        let mut t = x.clone();
        let o = t.mul_sub_mul_prec_assign_ref_ref_val(y, z, w.clone(), prec);
        assert!(same_pair(&(t, o), &r));
        let mut t = x.clone();
        let o = t.mul_sub_mul_prec_assign_ref_ref_ref(y, z, w, prec);
        assert!(same_pair(&(t, o), &r));
    }
    r
}

fn mul_sub_mul_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    z: &BigExponentFloat,
    w: &BigExponentFloat,
) -> BigExponentFloat {
    let r = x.clone().mul_sub_mul(y.clone(), z.clone(), w.clone());
    assert!(r.is_valid());
    assert!(same(&x.clone().mul_sub_mul(y.clone(), z.clone(), w), &r));
    assert!(same(&x.clone().mul_sub_mul(y.clone(), z, w.clone()), &r));
    assert!(same(&x.clone().mul_sub_mul(y.clone(), z, w), &r));
    assert!(same(&x.clone().mul_sub_mul(y, z.clone(), w.clone()), &r));
    assert!(same(&x.clone().mul_sub_mul(y, z.clone(), w), &r));
    assert!(same(&x.clone().mul_sub_mul(y, z, w.clone()), &r));
    assert!(same(&x.clone().mul_sub_mul(y, z, w), &r));
    assert!(same(&x.mul_sub_mul(y, z, w), &r));
    let mut t = x.clone();
    t.mul_sub_mul_assign(y.clone(), z.clone(), w.clone());
    assert!(same(&t, &r));
    let mut t = x.clone();
    t.mul_sub_mul_assign(y.clone(), z.clone(), w);
    assert!(same(&t, &r));
    let mut t = x.clone();
    t.mul_sub_mul_assign(y.clone(), z, w.clone());
    assert!(same(&t, &r));
    let mut t = x.clone();
    t.mul_sub_mul_assign(y.clone(), z, w);
    assert!(same(&t, &r));
    let mut t = x.clone();
    t.mul_sub_mul_assign(y, z.clone(), w.clone());
    assert!(same(&t, &r));
    let mut t = x.clone();
    t.mul_sub_mul_assign(y, z.clone(), w);
    assert!(same(&t, &r));
    let mut t = x.clone();
    t.mul_sub_mul_assign(y, z, w.clone());
    assert!(same(&t, &r));
    let mut t = x.clone();
    t.mul_sub_mul_assign(y, z, w);
    assert!(same(&t, &r));
    r
}

#[test]
fn test_mul_sub_mul_prec_round() {
    let check = |x: &str,
                 x_hex: &str,
                 y: &str,
                 y_hex: &str,
                 z: &str,
                 z_hex: &str,
                 w: &str,
                 w_hex: &str,
                 prec: u64,
                 rm: RoundingMode,
                 out: &str,
                 out_hex: &str,
                 o| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(parse_big_exponent_float_hex(y_hex).to_string(), y);
        let y = &parse_big_exponent_float_hex(y_hex);
        assert_eq!(parse_big_exponent_float_hex(z_hex).to_string(), z);
        let z = &parse_big_exponent_float_hex(z_hex);
        assert_eq!(parse_big_exponent_float_hex(w_hex).to_string(), w);
        let w = &parse_big_exponent_float_hex(w_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        let r = mul_sub_mul_prec_round_all(x, y, z, w, prec, rm);
        assert!(same(&r.0, out), "{:?} != {out:?}", r.0);
        assert_eq!(r.1, o);
    };

    // The products cancel exactly.
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "1.0",
        "0x1.0",
        "1.0",
        "0x1.0",
        10,
        Exact,
        "0.0",
        "0x0.0",
        Equal,
    );
    // The second product is far below the first.
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        10,
        Ceiling,
        "5.6e60205999132796239042",
        "0x1.0E+50000000000000000000",
        Greater,
    );
    check(
        "NaN", "NaN", "1.0", "0x1.0", "1.0", "0x1.0", "1.0", "0x1.0", 10, Floor, "NaN", "NaN",
        Equal,
    );
    check(
        "1.0", "0x1.0", "1.0", "0x1.0", "Infinity", "Infinity", "0.0", "0x0.0", 10, Floor, "NaN",
        "NaN", Equal,
    );
    check(
        "1.5", "0x1.8", "2.0", "0x2.0", "0.50", "0x0.8", "3.0", "0x3.0", 4, Exact, "1.5", "0x1.8",
        Equal,
    );
}

#[test]
#[should_panic]
fn mul_sub_mul_prec_round_fail_1() {
    BigExponentFloat::ONE.mul_sub_mul_prec_round(
        BigExponentFloat::ONE,
        BigExponentFloat::ONE,
        BigExponentFloat::ONE,
        0,
        Floor,
    );
}

#[test]
#[should_panic]
fn mul_sub_mul_prec_round_fail_2() {
    BigExponentFloat::from(3.0).mul_sub_mul_prec_round(
        BigExponentFloat::from(3.0),
        BigExponentFloat::from(1.0),
        BigExponentFloat::from(2.0),
        1,
        Exact,
    );
}

#[test]
fn mul_sub_mul_prec_round_properties() {
    let generator = big_exponent_float_big_exponent_float_big_exponent_float_big_exponent_float_unsigned_rounding_mode_sextuple_gen_var_2;
    generator().test_properties(|(x, y, z, w, prec, rm)| {
        let r = mul_sub_mul_prec_round_all(&x, &y, &z, &w, prec, rm);
        // The kernels agree with the kernel-free implementation, at any exponents.
        assert!(same_pair(
            &mul_add_mul_prec_round_generic(&x, &y, &z, &w, true, prec, rm),
            &r
        ));
        if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        // The products' factors commute.
        assert!(same_pair(
            &y.mul_sub_mul_prec_round_ref_ref_ref_ref(&x, &w, &z, prec, rm),
            &r
        ));
        // xy - zw = xy + (-z)w.
        assert!(same_pair(
            &x.mul_add_mul_prec_round_ref_ref_ref_ref(&y, &-&z, &w, prec, rm),
            &r
        ));
        if [&x, &y, &z, &w]
            .iter()
            .all(|v| big_exponent_float_exponent_is_small(v))
            && [&x, &y, &z, &w].iter().all(|v| v.is_finite())
        {
            let q = Rational::exact_from(&x) * Rational::exact_from(&y)
                - Rational::exact_from(&z) * Rational::exact_from(&w);
            assert_eq!(Rational::exact_from(&r.0).cmp(&q), r.1);
        }
    });
}

#[test]
fn mul_sub_mul_properties() {
    big_exponent_float_quadruple_gen_var_1().test_properties(|(x, y, z, w)| {
        assert!(same(
            &mul_sub_mul_all(&x, &y, &z, &w),
            &(&(&x * &y) - &(&z * &w))
        ));
    });
}

#[test]
fn mul_sub_mul_prec_round_float_agreement_properties() {
    float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3().test_properties(
        |(x, y, z, w, prec, rm)| {
            let (f, fo) = x.mul_sub_mul_prec_round_ref_ref_ref_ref(&y, &z, &w, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = mul_sub_mul_prec_round_all(
                    &to_bef(&x),
                    &to_bef(&y),
                    &to_bef(&z),
                    &to_bef(&w),
                    prec,
                    rm,
                );
                assert!(same(&r.0, &to_bef(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
