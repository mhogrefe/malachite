// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    CheckedLogBase2, ModAdd, ModAddAssign, ModIsReduced, ModNeg, ModPowerOf2Add, ModSub,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_add::*;

#[test]
fn test_mod_add() {
    let test = |s, t, m, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
        let m = Natural::from_str(m).unwrap();
        // Every combination of value and reference, and in place with all of them.
        let r = (&p).mod_add(&q, &m);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(p.clone().mod_add(q.clone(), m.clone()), r);
        assert_eq!(p.clone().mod_add(q.clone(), &m), r);
        assert_eq!(p.clone().mod_add(&q, m.clone()), r);
        assert_eq!(p.clone().mod_add(&q, &m), r);
        assert_eq!((&p).mod_add(q.clone(), m.clone()), r);
        assert_eq!((&p).mod_add(q.clone(), &m), r);
        assert_eq!((&p).mod_add(&q, m.clone()), r);
        let mut s = p.clone();
        s.mod_add_assign(q.clone(), m.clone());
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_add_assign(q.clone(), &m);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_add_assign(&q, m.clone());
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_add_assign(&q, &m);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mod_add_naive(&p, &q, &m), r);
    };
    // With m = 1, only the zero polynomial is reduced.
    test("0", "0", "1", "0");
    test("0", "0", "7", "0");
    // Adding zero, either way round.
    test("x+1", "0", "7", "x+1");
    test("0", "x+1", "7", "x+1");
    // Wrapping around, with either operand the longer.
    test("x+3", "x^2+6", "7", "x^2+x+2");
    test("x^2+6", "x+3", "7", "x^2+x+2");
    // The leading coefficients cancel, and so do the linear ones.
    test("5*x^2+x+3", "2*x^2+6*x+1", "7", "4");
    // Everything cancels.
    test("6", "1", "7", "0");
    test("1", "1", "2", "0");
    test("1", "1", "3", "2");
    // Coefficients and a modulus of two limbs.
    test(
        "18446744073709551616*x",
        "1",
        "18446744073709551617",
        "18446744073709551616*x+1",
    );
    test("18446744073709551616", "1", "18446744073709551617", "0");
    test(
        "18446744073709551616*x^2",
        "18446744073709551616*x^2+x",
        "18446744073709551617",
        "18446744073709551615*x^2+x",
    );
}

#[test]
#[should_panic]
fn mod_add_val_val_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_add(q, m);
}

#[test]
#[should_panic]
fn mod_add_val_val_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_add(q, m);
}

#[test]
#[should_panic]
fn mod_add_val_val_val_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = p.mod_add(q, m);
}

#[test]
#[should_panic]
fn mod_add_val_val_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_add(q, &m);
}

#[test]
#[should_panic]
fn mod_add_val_val_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_add(q, &m);
}

#[test]
#[should_panic]
fn mod_add_val_val_ref_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = p.mod_add(q, &m);
}

#[test]
#[should_panic]
fn mod_add_val_ref_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_add(&q, m);
}

#[test]
#[should_panic]
fn mod_add_val_ref_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_add(&q, m);
}

#[test]
#[should_panic]
fn mod_add_val_ref_val_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = p.mod_add(&q, m);
}

#[test]
#[should_panic]
fn mod_add_val_ref_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_add(&q, &m);
}

#[test]
#[should_panic]
fn mod_add_val_ref_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_add(&q, &m);
}

#[test]
#[should_panic]
fn mod_add_val_ref_ref_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = p.mod_add(&q, &m);
}

#[test]
#[should_panic]
fn mod_add_ref_val_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_add(q, m);
}

#[test]
#[should_panic]
fn mod_add_ref_val_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_add(q, m);
}

#[test]
#[should_panic]
fn mod_add_ref_val_val_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = (&p).mod_add(q, m);
}

#[test]
#[should_panic]
fn mod_add_ref_val_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_add(q, &m);
}

#[test]
#[should_panic]
fn mod_add_ref_val_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_add(q, &m);
}

#[test]
#[should_panic]
fn mod_add_ref_val_ref_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = (&p).mod_add(q, &m);
}

#[test]
#[should_panic]
fn mod_add_ref_ref_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_add(&q, m);
}

#[test]
#[should_panic]
fn mod_add_ref_ref_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_add(&q, m);
}

#[test]
#[should_panic]
fn mod_add_ref_ref_val_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = (&p).mod_add(&q, m);
}

#[test]
#[should_panic]
fn mod_add_ref_ref_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_add(&q, &m);
}

#[test]
#[should_panic]
fn mod_add_ref_ref_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_add(&q, &m);
}

#[test]
#[should_panic]
fn mod_add_ref_ref_ref_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = (&p).mod_add(&q, &m);
}

#[test]
#[should_panic]
fn mod_add_assign_val_val_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_add_assign(q, m);
}

#[test]
#[should_panic]
fn mod_add_assign_val_val_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_add_assign(q, m);
}

#[test]
#[should_panic]
fn mod_add_assign_val_val_zero_fail() {
    // The modulus is 0.
    let mut p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    p.mod_add_assign(q, m);
}

#[test]
#[should_panic]
fn mod_add_assign_val_ref_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_add_assign(q, &m);
}

#[test]
#[should_panic]
fn mod_add_assign_val_ref_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_add_assign(q, &m);
}

#[test]
#[should_panic]
fn mod_add_assign_val_ref_zero_fail() {
    // The modulus is 0.
    let mut p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    p.mod_add_assign(q, &m);
}

#[test]
#[should_panic]
fn mod_add_assign_ref_val_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_add_assign(&q, m);
}

#[test]
#[should_panic]
fn mod_add_assign_ref_val_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_add_assign(&q, m);
}

#[test]
#[should_panic]
fn mod_add_assign_ref_val_zero_fail() {
    // The modulus is 0.
    let mut p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    p.mod_add_assign(&q, m);
}

#[test]
#[should_panic]
fn mod_add_assign_ref_ref_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_add_assign(&q, &m);
}

#[test]
#[should_panic]
fn mod_add_assign_ref_ref_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_add_assign(&q, &m);
}

#[test]
#[should_panic]
fn mod_add_assign_ref_ref_zero_fail() {
    // The modulus is 0.
    let mut p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    p.mod_add_assign(&q, &m);
}

#[test]
fn mod_add_properties() {
    natural_polynomial_natural_polynomial_natural_triple_gen_var_1().test_properties(
        |(p, q, m)| {
            let r = (&p).mod_add(&q, &m);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!(p.clone().mod_add(q.clone(), m.clone()), r);
            assert_eq!(p.clone().mod_add(q.clone(), &m), r);
            assert_eq!(p.clone().mod_add(&q, m.clone()), r);
            assert_eq!(p.clone().mod_add(&q, &m), r);
            assert_eq!((&p).mod_add(q.clone(), m.clone()), r);
            assert_eq!((&p).mod_add(q.clone(), &m), r);
            assert_eq!((&p).mod_add(&q, m.clone()), r);
            let mut s = p.clone();
            s.mod_add_assign(q.clone(), m.clone());
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_add_assign(q.clone(), &m);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_add_assign(&q, m.clone());
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_add_assign(&q, &m);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // The result is reduced, and is the integer sum, reduced.
            assert!(r.mod_is_reduced(&m));
            assert_eq!(mod_add_naive(&p, &q, &m), r);
            // The degree is at most the larger of the two.
            assert!(r.len() <= p.len().max(q.len()));

            // Addition is commutative.
            assert_eq!((&q).mod_add(&p, &m), r);
            // Adding zero changes nothing, and adding the negation gives zero.
            assert_eq!((&p).mod_add(&NaturalPolynomial::ZERO, &m), p);
            assert_eq!((&p).mod_add((&p).mod_neg(&m), &m), NaturalPolynomial::ZERO);
            // Subtracting the second operand back gives the first.
            assert_eq!((&r).mod_sub(&q, &m), p);
            // A power-of-2 modulus gives the same result as mod_power_of_2_add.
            if let Some(pow) = m.checked_log_base_2() {
                assert_eq!((&p).mod_power_of_2_add(&q, pow), r);
            }
        },
    );

    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<u64>().test_properties(
        |(p, q, m)| {
            // The u64 and Natural versions agree.
            assert_eq!(
                NaturalPolynomial::from((&p).mod_add(&q, m)),
                NaturalPolynomial::from(p).mod_add(NaturalPolynomial::from(q), Natural::from(m))
            );
        },
    );
}
