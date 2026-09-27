// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModAdd, ModAddAssign, ModIsReduced, ModNeg, ModPowerOf2Add, ModSub,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_add::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_mod_add() {
    fn test<T: PrimitiveUnsigned>(s: &str, t: &str, m: T, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(t).unwrap();
        // All four combinations of value and reference, and in place with both.
        let r = (&p).mod_add(&q, m);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).mod_add(q.clone(), m), r);
        assert_eq!(p.clone().mod_add(&q, m), r);
        assert_eq!(p.clone().mod_add(q.clone(), m), r);
        let mut s = p.clone();
        s.mod_add_assign(&q, m);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_add_assign(q.clone(), m);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mod_add_naive(&p, &q, m), r);
    }
    // With m = 1, only the zero polynomial is reduced.
    test::<u8>("0", "0", 1, "0");
    test::<u8>("0", "0", 7, "0");
    // Adding zero, either way round.
    test::<u8>("x+1", "0", 7, "x+1");
    test::<u8>("0", "x+1", 7, "x+1");
    // Wrapping around, with either operand the longer.
    test::<u8>("x+3", "x^2+6", 7, "x^2+x+2");
    test::<u8>("x^2+6", "x+3", 7, "x^2+x+2");
    // The leading coefficients cancel, and so do the linear ones.
    test::<u8>("5*x^2+x+3", "2*x^2+6*x+1", 7, "4");
    // Everything cancels.
    test::<u8>("6", "1", 7, "0");
    test::<u8>("1", "1", 2, "0");
    // Moduli near the top of the type.
    test::<u8>("254*x+200", "x+100", 255, "45");
    test::<u64>(
        "18446744073709551614*x",
        "1",
        18446744073709551615,
        "18446744073709551614*x+1",
    );
    test::<u64>("18446744073709551614", "1", 18446744073709551615, "0");
}

#[test]
#[should_panic]
fn mod_add_val_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_add(q, 7);
}

#[test]
#[should_panic]
fn mod_add_val_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = p.mod_add(q, 7);
}

#[test]
#[should_panic]
fn mod_add_val_val_zero_fail() {
    // The modulus is 0.
    let p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let _ = p.mod_add(q, 0);
}

#[test]
#[should_panic]
fn mod_add_val_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = p.mod_add(&q, 7);
}

#[test]
#[should_panic]
fn mod_add_val_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = p.mod_add(&q, 7);
}

#[test]
#[should_panic]
fn mod_add_val_ref_zero_fail() {
    // The modulus is 0.
    let p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let _ = p.mod_add(&q, 0);
}

#[test]
#[should_panic]
fn mod_add_ref_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_add(q, 7);
}

#[test]
#[should_panic]
fn mod_add_ref_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = (&p).mod_add(q, 7);
}

#[test]
#[should_panic]
fn mod_add_ref_val_zero_fail() {
    // The modulus is 0.
    let p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let _ = (&p).mod_add(q, 0);
}

#[test]
#[should_panic]
fn mod_add_ref_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let _ = (&p).mod_add(&q, 7);
}

#[test]
#[should_panic]
fn mod_add_ref_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let _ = (&p).mod_add(&q, 7);
}

#[test]
#[should_panic]
fn mod_add_ref_ref_zero_fail() {
    // The modulus is 0.
    let p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let _ = (&p).mod_add(&q, 0);
}

#[test]
#[should_panic]
fn mod_add_assign_val_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_add_assign(q, 7);
}

#[test]
#[should_panic]
fn mod_add_assign_val_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    p.mod_add_assign(q, 7);
}

#[test]
#[should_panic]
fn mod_add_assign_val_zero_fail() {
    // The modulus is 0.
    let mut p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    p.mod_add_assign(q, 0);
}

#[test]
#[should_panic]
fn mod_add_assign_ref_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    p.mod_add_assign(&q, 7);
}

#[test]
#[should_panic]
fn mod_add_assign_ref_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    p.mod_add_assign(&q, 7);
}

#[test]
#[should_panic]
fn mod_add_assign_ref_zero_fail() {
    // The modulus is 0.
    let mut p = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    p.mod_add_assign(&q, 0);
}

fn mod_add_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>().test_properties(
        |(p, q, m)| {
            let r = (&p).mod_add(&q, m);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!((&p).mod_add(q.clone(), m), r);
            assert_eq!(p.clone().mod_add(&q, m), r);
            assert_eq!(p.clone().mod_add(q.clone(), m), r);
            let mut s = p.clone();
            s.mod_add_assign(&q, m);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_add_assign(q.clone(), m);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // The result is reduced, and is the coefficient-wise sum.
            assert!(r.mod_is_reduced(&m));
            assert_eq!(mod_add_naive(&p, &q, m), r);
            // The degree is at most the larger of the two.
            assert!(r.len() <= p.len().max(q.len()));

            // Addition is commutative.
            assert_eq!((&q).mod_add(&p, m), r);
            // Adding zero changes nothing, and adding the negation gives zero.
            assert_eq!((&p).mod_add(&UnsignedPolynomial::ZERO, m), p);
            assert_eq!((&p).mod_add((&p).mod_neg(m), m), UnsignedPolynomial::ZERO);
            // Subtracting the second operand back gives the first.
            assert_eq!((&r).mod_sub(&q, m), p);
            // A power-of-2 modulus gives the same result as mod_power_of_2_add.
            if m.is_power_of_2() {
                assert_eq!((&p).mod_power_of_2_add(&q, m.trailing_zeros()), r);
            }
        },
    );
}

#[test]
fn mod_add_properties() {
    apply_fn_to_unsigneds!(mod_add_properties_helper);
}
