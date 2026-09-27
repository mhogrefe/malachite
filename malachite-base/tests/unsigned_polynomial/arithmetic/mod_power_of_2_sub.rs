// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Neg, ModPowerOf2Sub, ModPowerOf2SubAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_sub::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_mod_power_of_2_sub() {
    fn test<T: PrimitiveUnsigned>(s: &str, t: &str, pow: u64, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(t).unwrap();
        // All four combinations of value and reference, and in place with both.
        let r = (&p).mod_power_of_2_sub(&q, pow);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).mod_power_of_2_sub(q.clone(), pow), r);
        assert_eq!(p.clone().mod_power_of_2_sub(&q, pow), r);
        assert_eq!(p.clone().mod_power_of_2_sub(q.clone(), pow), r);
        let mut s = p.clone();
        s.mod_power_of_2_sub_assign(&q, pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_power_of_2_sub_assign(q.clone(), pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mod_power_of_2_sub_naive(&p, &q, pow), r);
    }
    // With pow 0, only the zero polynomial is reduced.
    test::<u8>("0", "0", 0, "0");
    test::<u8>("0", "0", 5, "0");
    // Subtracting from zero negates.
    test::<u8>("x+1", "0", 3, "x+1");
    test::<u8>("0", "x+1", 3, "7*x+7");
    // Wrapping around, with either operand the longer.
    test::<u8>("x", "x^2+1", 3, "7*x^2+x+7");
    test::<u8>("x^2+1", "x", 3, "x^2+7*x+1");
    // Every coefficient wraps around.
    test::<u8>("5*x^2+x+3", "3*x^2+7*x+1", 3, "2*x^2+2*x+2");
    // The leading coefficients cancel.
    test::<u8>("5*x^2+x", "5*x^2+3", 3, "x+5");
    // Everything cancels.
    test::<u8>("3", "3", 2, "0");
    // The full width of the type.
    test::<u8>("x", "255*x+1", 8, "2*x+255");
    test::<u64>("0", "1", 64, "18446744073709551615");
    test::<u64>("x", "18446744073709551615*x^2", 64, "x^2+x");
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_power_of_2_sub(q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = p.mod_power_of_2_sub(q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_pow_fail() {
    // pow is wider than a u8.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_power_of_2_sub(q, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_val_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_power_of_2_sub(&q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_val_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = p.mod_power_of_2_sub(&q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_val_ref_pow_fail() {
    // pow is wider than a u8.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_power_of_2_sub(&q, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_ref_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_sub(q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_ref_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = (&p).mod_power_of_2_sub(q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_ref_val_pow_fail() {
    // pow is wider than a u8.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_sub(q, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_ref_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_sub(&q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_ref_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = (&p).mod_power_of_2_sub(&q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_ref_ref_pow_fail() {
    // pow is wider than a u8.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_sub(&q, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_assign_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_power_of_2_sub_assign(q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_assign_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    p.mod_power_of_2_sub_assign(q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_assign_pow_fail() {
    // pow is wider than a u8.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_power_of_2_sub_assign(q, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_assign_ref_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_power_of_2_sub_assign(&q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_assign_ref_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    p.mod_power_of_2_sub_assign(&q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_assign_ref_pow_fail() {
    // pow is wider than a u8.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_power_of_2_sub_assign(&q, 9);
}

fn mod_power_of_2_sub_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().test_properties(
        |(p, q, pow)| {
            let r = (&p).mod_power_of_2_sub(&q, pow);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!((&p).mod_power_of_2_sub(q.clone(), pow), r);
            assert_eq!(p.clone().mod_power_of_2_sub(&q, pow), r);
            assert_eq!(p.clone().mod_power_of_2_sub(q.clone(), pow), r);
            let mut s = p.clone();
            s.mod_power_of_2_sub_assign(&q, pow);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_power_of_2_sub_assign(q.clone(), pow);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // The result is reduced, and is the coefficient-wise sub.
            assert!(r.mod_power_of_2_is_reduced(pow));
            assert_eq!(mod_power_of_2_sub_naive(&p, &q, pow), r);
            // The degree is at most the larger of the two.
            assert!(r.len() <= p.len().max(q.len()));

            // Subtracting is adding the negation.
            assert_eq!(
                (&p).mod_power_of_2_add((&q).mod_power_of_2_neg(pow), pow),
                r
            );
            // Swapping the operands negates the difference.
            assert_eq!(
                (&q).mod_power_of_2_sub(&p, pow),
                (&r).mod_power_of_2_neg(pow)
            );
            // Adding the second operand back gives the first.
            assert_eq!((&r).mod_power_of_2_add(&q, pow), p);
            // Subtracting zero changes nothing, and a polynomial minus itself is zero.
            assert_eq!((&p).mod_power_of_2_sub(&UnsignedPolynomial::ZERO, pow), p);
            assert_eq!((&p).mod_power_of_2_sub(&p, pow), UnsignedPolynomial::ZERO);
        },
    );
}

#[test]
fn mod_power_of_2_sub_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_sub_properties_helper);
}
