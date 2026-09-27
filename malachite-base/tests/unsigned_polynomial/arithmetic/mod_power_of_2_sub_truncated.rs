// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2Neg, ModPowerOf2Sub,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{
    EqTruncated, ModPowerOf2AddTruncated, ModPowerOf2SubTruncated, ModPowerOf2SubTruncatedAssign,
    Polynomial,
};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_sub_truncated::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_mod_power_of_2_sub_truncated() {
    fn test<T: PrimitiveUnsigned>(s: &str, t: &str, len: u64, pow: u64, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(t).unwrap();
        // All four combinations of value and reference, and in place with both.
        let r = (&p).mod_power_of_2_sub_truncated(&q, len, pow);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).mod_power_of_2_sub_truncated(q.clone(), len, pow), r);
        assert_eq!(p.clone().mod_power_of_2_sub_truncated(&q, len, pow), r);
        assert_eq!(
            p.clone().mod_power_of_2_sub_truncated(q.clone(), len, pow),
            r
        );
        let mut s = p.clone();
        s.mod_power_of_2_sub_truncated_assign(&q, len, pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_power_of_2_sub_truncated_assign(q.clone(), len, pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mod_power_of_2_sub_truncated_naive(&p, &q, len, pow), r);
    }
    // A zero length or pow 0 keeps nothing.
    test::<u8>("0", "0", 0, 0, "0");
    test::<u8>("0", "0", 3, 0, "0");
    test::<u8>("x+1", "0", 0, 3, "0");
    // Truncation drops the high coefficients of either operand.
    test::<u8>("x^2+1", "0", 2, 3, "1");
    test::<u8>("0", "x^2+1", 3, 3, "7*x^2+7");
    // The quadratic and linear coefficients wrap around.
    test::<u8>("x^3+2*x^2+x+5", "4*x^2+7*x+2", 3, 3, "6*x^2+2*x+3");
    test::<u8>("x^3+2*x^2+x+5", "4*x^2+7*x+2", 1, 3, "3");
    // A length past both degrees is the whole difference.
    test::<u8>("x^3+2*x^2+x+5", "4*x^2+7*x+2", 100, 3, "x^3+6*x^2+2*x+3");
    // Everything cancels.
    test::<u8>("x^2+x+1", "x^2+x+1", 3, 3, "0");
    // The cut leading terms would have cancelled anyway.
    test::<u8>("x^3+x", "x^3+2*x", 3, 3, "7*x");
    // The full width of the type.
    test::<u8>("x", "255*x^2+x+1", 3, 8, "x^2+255");
    test::<u64>("0", "1", 1, 64, "18446744073709551615");
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_truncated_val_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_power_of_2_sub_truncated(q, 3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_truncated_val_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = p.mod_power_of_2_sub_truncated(q, 3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_truncated_val_val_pow_fail() {
    // pow is wider than a u8.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_power_of_2_sub_truncated(q, 3, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_truncated_val_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_power_of_2_sub_truncated(&q, 3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_truncated_val_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = p.mod_power_of_2_sub_truncated(&q, 3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_truncated_val_ref_pow_fail() {
    // pow is wider than a u8.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_power_of_2_sub_truncated(&q, 3, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_truncated_ref_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_sub_truncated(q, 3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_truncated_ref_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = (&p).mod_power_of_2_sub_truncated(q, 3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_truncated_ref_val_pow_fail() {
    // pow is wider than a u8.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_sub_truncated(q, 3, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_truncated_ref_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_sub_truncated(&q, 3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_truncated_ref_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = (&p).mod_power_of_2_sub_truncated(&q, 3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_truncated_ref_ref_pow_fail() {
    // pow is wider than a u8.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_sub_truncated(&q, 3, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_truncated_assign_val_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_power_of_2_sub_truncated_assign(q, 3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_truncated_assign_val_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    p.mod_power_of_2_sub_truncated_assign(q, 3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_truncated_assign_val_pow_fail() {
    // pow is wider than a u8.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_power_of_2_sub_truncated_assign(q, 3, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_truncated_assign_ref_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_power_of_2_sub_truncated_assign(&q, 3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_truncated_assign_ref_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    p.mod_power_of_2_sub_truncated_assign(&q, 3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_truncated_assign_ref_pow_fail() {
    // pow is wider than a u8.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_power_of_2_sub_truncated_assign(&q, 3, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_truncated_beyond_len_fail() {
    // A coefficient past the kept length is not reduced; the whole operand must be reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x^2+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_sub_truncated(&q, 1, 3);
}

fn mod_power_of_2_sub_truncated_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1::<T>().test_properties(
        |(p, q, len, pow)| {
            let r = (&p).mod_power_of_2_sub_truncated(&q, len, pow);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!((&p).mod_power_of_2_sub_truncated(q.clone(), len, pow), r);
            assert_eq!(p.clone().mod_power_of_2_sub_truncated(&q, len, pow), r);
            assert_eq!(
                p.clone().mod_power_of_2_sub_truncated(q.clone(), len, pow),
                r
            );
            let mut s = p.clone();
            s.mod_power_of_2_sub_truncated_assign(&q, len, pow);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_power_of_2_sub_truncated_assign(q.clone(), len, pow);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // The result is reduced, and is the truncation of the whole modular result and the
            // modular result of the truncations.
            assert!(r.mod_power_of_2_is_reduced(pow));
            assert_eq!(mod_power_of_2_sub_truncated_naive(&p, &q, len, pow), r);
            assert_eq!((&p).mod_power_of_2_sub(&q, pow).truncate(len), r);
            assert_eq!(p.truncate(len).mod_power_of_2_sub(q.truncate(len), pow), r);
            // It agrees with the whole modular result below the cut, and has nothing above it.
            assert!(r.eq_truncated(&(&p).mod_power_of_2_sub(&q, pow), len));
            assert!(r.len() <= len);
            // Swapping the operands negates the difference.
            assert_eq!(
                (&q).mod_power_of_2_sub_truncated(&p, len, pow),
                (&r).mod_power_of_2_neg(pow)
            );
            // A polynomial minus itself is zero.
            assert_eq!(
                (&p).mod_power_of_2_sub_truncated(&p, len, pow),
                UnsignedPolynomial::ZERO
            );
            // It is adding the negation.
            assert_eq!(
                (&p).mod_power_of_2_add_truncated((&q).mod_power_of_2_neg(pow), len, pow),
                r
            );
        },
    );
}

#[test]
fn mod_power_of_2_sub_truncated_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_sub_truncated_properties_helper);
}
