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
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::*;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_sub::*;

#[test]
fn test_mod_power_of_2_sub() {
    let test = |s, t, pow, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
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
    };
    // With pow 0, only the zero polynomial is reduced.
    test("0", "0", 0, "0");
    test("0", "0", 5, "0");
    // Subtracting from zero negates.
    test("x+1", "0", 3, "x+1");
    test("0", "x+1", 3, "7*x+7");
    // Wrapping around, with either operand the longer.
    test("x", "x^2+1", 3, "7*x^2+x+7");
    test("x^2+1", "x", 3, "x^2+7*x+1");
    // Every coefficient wraps around.
    test("5*x^2+x+3", "3*x^2+7*x+1", 3, "2*x^2+2*x+2");
    // The leading coefficients cancel.
    test("5*x^2+x", "5*x^2+3", 3, "x+5");
    // Everything cancels.
    test("3", "3", 2, "0");
    // Coefficients of one and two limbs.
    test("0", "1", 64, "18446744073709551615");
    test("x", "1267650600228229401496703205375*x^2", 100, "x^2+x");
    test(
        "1267650600228229401496703205375*x^2+x",
        "1267650600228229401496703205375*x^2",
        100,
        "x",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_self_fail() {
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let _ = p.mod_power_of_2_sub(q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_other_fail() {
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let _ = p.mod_power_of_2_sub(q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_val_ref_self_fail() {
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let _ = p.mod_power_of_2_sub(&q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_val_ref_other_fail() {
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let _ = p.mod_power_of_2_sub(&q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_ref_val_self_fail() {
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_sub(q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_ref_val_other_fail() {
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let _ = (&p).mod_power_of_2_sub(q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_ref_ref_self_fail() {
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let _ = (&p).mod_power_of_2_sub(&q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_ref_ref_other_fail() {
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let _ = (&p).mod_power_of_2_sub(&q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_assign_self_fail() {
    let mut p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    p.mod_power_of_2_sub_assign(q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_assign_other_fail() {
    let mut p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    p.mod_power_of_2_sub_assign(q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_assign_ref_self_fail() {
    let mut p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    p.mod_power_of_2_sub_assign(&q, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_assign_ref_other_fail() {
    let mut p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    p.mod_power_of_2_sub_assign(&q, 3);
}

#[test]
fn mod_power_of_2_sub_properties() {
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1().test_properties(
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

            // The result is reduced, and is the integer difference, reduced.
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
            assert_eq!((&p).mod_power_of_2_sub(&NaturalPolynomial::ZERO, pow), p);
            assert_eq!((&p).mod_power_of_2_sub(&p, pow), NaturalPolynomial::ZERO);
        },
    );

    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<u64>().test_properties(
        |(p, q, pow)| {
            // The u64 and Natural versions agree.
            assert_eq!(
                NaturalPolynomial::from((&p).mod_power_of_2_sub(&q, pow)),
                NaturalPolynomial::from(p).mod_power_of_2_sub(NaturalPolynomial::from(q), pow)
            );
        },
    );
}
