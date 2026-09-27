// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModAdd, ModIsReduced, ModNeg, ModPowerOf2Sub, ModSub, ModSubAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_sub::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_mod_sub() {
    fn test<T: PrimitiveUnsigned>(s: &str, t: &str, m: T, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(t).unwrap();
        // All four combinations of value and reference, and in place with both.
        let r = (&p).mod_sub(&q, m);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).mod_sub(q.clone(), m), r);
        assert_eq!(p.clone().mod_sub(&q, m), r);
        assert_eq!(p.clone().mod_sub(q.clone(), m), r);
        let mut s = p.clone();
        s.mod_sub_assign(&q, m);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_sub_assign(q.clone(), m);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mod_sub_naive(&p, &q, m), r);
    }
    // With m = 1, only the zero polynomial is reduced.
    test::<u8>("0", "0", 1, "0");
    test::<u8>("0", "0", 7, "0");
    // Subtracting from zero negates.
    test::<u8>("x+1", "0", 7, "x+1");
    test::<u8>("0", "x+1", 7, "6*x+6");
    // Wrapping around, with either operand the longer.
    test::<u8>("x", "x^2+1", 7, "6*x^2+x+6");
    test::<u8>("x^2+1", "x", 7, "x^2+6*x+1");
    // Every coefficient but the leading one wraps around.
    test::<u8>("5*x^2+x+3", "2*x^2+6*x+1", 7, "3*x^2+2*x+2");
    // The leading coefficients cancel.
    test::<u8>("5*x^2+x", "5*x^2+3", 7, "x+4");
    // Everything cancels.
    test::<u8>("3", "3", 5, "0");
    // Moduli near the top of the type.
    test::<u8>("x", "254*x+1", 255, "2*x+254");
    test::<u64>("0", "1", 18446744073709551615, "18446744073709551614");
    test::<u64>(
        "x",
        "18446744073709551614*x^2",
        18446744073709551615,
        "x^2+x",
    );
}

#[test]
#[should_panic]
fn mod_sub_val_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_sub(q, 7);
}

#[test]
#[should_panic]
fn mod_sub_val_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = p.mod_sub(q, 7);
}

#[test]
#[should_panic]
fn mod_sub_val_val_zero_fail() {
    // The modulus is 0.
    let p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let _ = p.mod_sub(q, 0);
}

#[test]
#[should_panic]
fn mod_sub_val_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_sub(&q, 7);
}

#[test]
#[should_panic]
fn mod_sub_val_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = p.mod_sub(&q, 7);
}

#[test]
#[should_panic]
fn mod_sub_val_ref_zero_fail() {
    // The modulus is 0.
    let p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let _ = p.mod_sub(&q, 0);
}

#[test]
#[should_panic]
fn mod_sub_ref_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_sub(q, 7);
}

#[test]
#[should_panic]
fn mod_sub_ref_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = (&p).mod_sub(q, 7);
}

#[test]
#[should_panic]
fn mod_sub_ref_val_zero_fail() {
    // The modulus is 0.
    let p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let _ = (&p).mod_sub(q, 0);
}

#[test]
#[should_panic]
fn mod_sub_ref_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_sub(&q, 7);
}

#[test]
#[should_panic]
fn mod_sub_ref_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = (&p).mod_sub(&q, 7);
}

#[test]
#[should_panic]
fn mod_sub_ref_ref_zero_fail() {
    // The modulus is 0.
    let p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let _ = (&p).mod_sub(&q, 0);
}

#[test]
#[should_panic]
fn mod_sub_assign_val_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_sub_assign(q, 7);
}

#[test]
#[should_panic]
fn mod_sub_assign_val_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    p.mod_sub_assign(q, 7);
}

#[test]
#[should_panic]
fn mod_sub_assign_val_zero_fail() {
    // The modulus is 0.
    let mut p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    p.mod_sub_assign(q, 0);
}

#[test]
#[should_panic]
fn mod_sub_assign_ref_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_sub_assign(&q, 7);
}

#[test]
#[should_panic]
fn mod_sub_assign_ref_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    p.mod_sub_assign(&q, 7);
}

#[test]
#[should_panic]
fn mod_sub_assign_ref_zero_fail() {
    // The modulus is 0.
    let mut p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    p.mod_sub_assign(&q, 0);
}

fn mod_sub_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>().test_properties(
        |(p, q, m)| {
            let r = (&p).mod_sub(&q, m);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!((&p).mod_sub(q.clone(), m), r);
            assert_eq!(p.clone().mod_sub(&q, m), r);
            assert_eq!(p.clone().mod_sub(q.clone(), m), r);
            let mut s = p.clone();
            s.mod_sub_assign(&q, m);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_sub_assign(q.clone(), m);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // The result is reduced, and is the coefficient-wise difference.
            assert!(r.mod_is_reduced(&m));
            assert_eq!(mod_sub_naive(&p, &q, m), r);
            // The degree is at most the larger of the two.
            assert!(r.len() <= p.len().max(q.len()));

            // Subtracting is adding the negation.
            assert_eq!((&p).mod_add((&q).mod_neg(m), m), r);
            // Swapping the operands negates the difference.
            assert_eq!((&q).mod_sub(&p, m), (&r).mod_neg(m));
            // Adding the second operand back gives the first.
            assert_eq!((&r).mod_add(&q, m), p);
            // Subtracting zero changes nothing, and a polynomial minus itself is zero.
            assert_eq!((&p).mod_sub(&UnsignedPolynomial::ZERO, m), p);
            assert_eq!((&p).mod_sub(&p, m), UnsignedPolynomial::ZERO);
            // A power-of-2 modulus gives the same result as mod_power_of_2_sub.
            if m.is_power_of_2() {
                assert_eq!((&p).mod_power_of_2_sub(&q, m.trailing_zeros()), r);
            }
        },
    );
}

#[test]
fn mod_sub_properties() {
    apply_fn_to_unsigneds!(mod_sub_properties_helper);
}
