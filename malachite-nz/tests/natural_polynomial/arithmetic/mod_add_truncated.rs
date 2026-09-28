// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{CheckedLogBase2, ModAdd, ModIsReduced, ModNeg};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{
    EqTruncated, ModAddTruncated, ModAddTruncatedAssign, ModPowerOf2AddTruncated, Polynomial,
};
use malachite_base::test_util::generators::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_add_truncated::*;

#[test]
fn test_mod_add_truncated() {
    let test = |s, t, len, m, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
        let m = Natural::from_str(m).unwrap();
        // Every combination of value and reference, and in place with all of them.
        let r = (&p).mod_add_truncated(&q, len, &m);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(p.clone().mod_add_truncated(q.clone(), len, m.clone()), r);
        assert_eq!(p.clone().mod_add_truncated(q.clone(), len, &m), r);
        assert_eq!(p.clone().mod_add_truncated(&q, len, m.clone()), r);
        assert_eq!(p.clone().mod_add_truncated(&q, len, &m), r);
        assert_eq!((&p).mod_add_truncated(q.clone(), len, m.clone()), r);
        assert_eq!((&p).mod_add_truncated(q.clone(), len, &m), r);
        assert_eq!((&p).mod_add_truncated(&q, len, m.clone()), r);
        let mut s = p.clone();
        s.mod_add_truncated_assign(q.clone(), len, m.clone());
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_add_truncated_assign(q.clone(), len, &m);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_add_truncated_assign(&q, len, m.clone());
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_add_truncated_assign(&q, len, &m);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mod_add_truncated_naive(&p, &q, len, &m), r);
    };
    // A zero length, or m = 1, keeps nothing.
    test("0", "0", 0, "1", "0");
    test("0", "0", 3, "1", "0");
    test("x+1", "0", 0, "7", "0");
    // Truncation drops the high coefficients of either operand.
    test("x^2+1", "0", 2, "7", "1");
    test("0", "x^2+1", 2, "7", "1");
    // The linear and constant coefficients wrap around to 0.
    test("x^3+2*x^2+x+5", "4*x^2+6*x+2", 3, "7", "6*x^2");
    test("x^3+2*x^2+x+5", "4*x^2+6*x+2", 1, "7", "0");
    // A length past both degrees is the whole sum.
    test("x^3+2*x^2+x+5", "4*x^2+6*x+2", 100, "7", "x^3+6*x^2");
    // Everything cancels.
    test("6*x^2+1", "x^2+6", 3, "7", "0");
    // Coefficients and a modulus of two limbs.
    test(
        "18446744073709551616*x^2+x",
        "18446744073709551616*x^2+18446744073709551616*x",
        2,
        "18446744073709551617",
        "0",
    );
}

#[test]
#[should_panic]
fn mod_add_truncated_val_val_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_add_truncated(q, 3, m);
}

#[test]
#[should_panic]
fn mod_add_truncated_val_val_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_add_truncated(q, 3, m);
}

#[test]
#[should_panic]
fn mod_add_truncated_val_val_val_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = p.mod_add_truncated(q, 3, m);
}

#[test]
#[should_panic]
fn mod_add_truncated_val_val_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_add_truncated(q, 3, &m);
}

#[test]
#[should_panic]
fn mod_add_truncated_val_val_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_add_truncated(q, 3, &m);
}

#[test]
#[should_panic]
fn mod_add_truncated_val_val_ref_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = p.mod_add_truncated(q, 3, &m);
}

#[test]
#[should_panic]
fn mod_add_truncated_val_ref_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_add_truncated(&q, 3, m);
}

#[test]
#[should_panic]
fn mod_add_truncated_val_ref_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_add_truncated(&q, 3, m);
}

#[test]
#[should_panic]
fn mod_add_truncated_val_ref_val_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = p.mod_add_truncated(&q, 3, m);
}

#[test]
#[should_panic]
fn mod_add_truncated_val_ref_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_add_truncated(&q, 3, &m);
}

#[test]
#[should_panic]
fn mod_add_truncated_val_ref_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_add_truncated(&q, 3, &m);
}

#[test]
#[should_panic]
fn mod_add_truncated_val_ref_ref_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = p.mod_add_truncated(&q, 3, &m);
}

#[test]
#[should_panic]
fn mod_add_truncated_ref_val_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_add_truncated(q, 3, m);
}

#[test]
#[should_panic]
fn mod_add_truncated_ref_val_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_add_truncated(q, 3, m);
}

#[test]
#[should_panic]
fn mod_add_truncated_ref_val_val_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = (&p).mod_add_truncated(q, 3, m);
}

#[test]
#[should_panic]
fn mod_add_truncated_ref_val_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_add_truncated(q, 3, &m);
}

#[test]
#[should_panic]
fn mod_add_truncated_ref_val_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_add_truncated(q, 3, &m);
}

#[test]
#[should_panic]
fn mod_add_truncated_ref_val_ref_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = (&p).mod_add_truncated(q, 3, &m);
}

#[test]
#[should_panic]
fn mod_add_truncated_ref_ref_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_add_truncated(&q, 3, m);
}

#[test]
#[should_panic]
fn mod_add_truncated_ref_ref_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_add_truncated(&q, 3, m);
}

#[test]
#[should_panic]
fn mod_add_truncated_ref_ref_val_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = (&p).mod_add_truncated(&q, 3, m);
}

#[test]
#[should_panic]
fn mod_add_truncated_ref_ref_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_add_truncated(&q, 3, &m);
}

#[test]
#[should_panic]
fn mod_add_truncated_ref_ref_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_add_truncated(&q, 3, &m);
}

#[test]
#[should_panic]
fn mod_add_truncated_ref_ref_ref_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = (&p).mod_add_truncated(&q, 3, &m);
}

#[test]
#[should_panic]
fn mod_add_truncated_assign_val_val_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_add_truncated_assign(q, 3, m);
}

#[test]
#[should_panic]
fn mod_add_truncated_assign_val_val_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_add_truncated_assign(q, 3, m);
}

#[test]
#[should_panic]
fn mod_add_truncated_assign_val_val_zero_fail() {
    // The modulus is 0.
    let mut p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    p.mod_add_truncated_assign(q, 3, m);
}

#[test]
#[should_panic]
fn mod_add_truncated_assign_val_ref_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_add_truncated_assign(q, 3, &m);
}

#[test]
#[should_panic]
fn mod_add_truncated_assign_val_ref_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_add_truncated_assign(q, 3, &m);
}

#[test]
#[should_panic]
fn mod_add_truncated_assign_val_ref_zero_fail() {
    // The modulus is 0.
    let mut p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    p.mod_add_truncated_assign(q, 3, &m);
}

#[test]
#[should_panic]
fn mod_add_truncated_assign_ref_val_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_add_truncated_assign(&q, 3, m);
}

#[test]
#[should_panic]
fn mod_add_truncated_assign_ref_val_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_add_truncated_assign(&q, 3, m);
}

#[test]
#[should_panic]
fn mod_add_truncated_assign_ref_val_zero_fail() {
    // The modulus is 0.
    let mut p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    p.mod_add_truncated_assign(&q, 3, m);
}

#[test]
#[should_panic]
fn mod_add_truncated_assign_ref_ref_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_add_truncated_assign(&q, 3, &m);
}

#[test]
#[should_panic]
fn mod_add_truncated_assign_ref_ref_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_add_truncated_assign(&q, 3, &m);
}

#[test]
#[should_panic]
fn mod_add_truncated_assign_ref_ref_zero_fail() {
    // The modulus is 0.
    let mut p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    p.mod_add_truncated_assign(&q, 3, &m);
}

#[test]
#[should_panic]
fn mod_add_truncated_beyond_len_fail() {
    // A coefficient past the kept length is not reduced; the whole operand must be reduced.
    let p = NaturalPolynomial::from_str("8*x^2+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let _ = (&p).mod_add_truncated(&q, 1, &Natural::from(7u32));
}

#[test]
fn mod_add_truncated_properties() {
    natural_polynomial_pair_unsigned_natural_quadruple_gen_var_1().test_properties(
        |(p, q, len, m)| {
            let r = (&p).mod_add_truncated(&q, len, &m);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!(p.clone().mod_add_truncated(q.clone(), len, m.clone()), r);
            assert_eq!(p.clone().mod_add_truncated(q.clone(), len, &m), r);
            assert_eq!(p.clone().mod_add_truncated(&q, len, m.clone()), r);
            assert_eq!(p.clone().mod_add_truncated(&q, len, &m), r);
            assert_eq!((&p).mod_add_truncated(q.clone(), len, m.clone()), r);
            assert_eq!((&p).mod_add_truncated(q.clone(), len, &m), r);
            assert_eq!((&p).mod_add_truncated(&q, len, m.clone()), r);
            let mut s = p.clone();
            s.mod_add_truncated_assign(q.clone(), len, m.clone());
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_add_truncated_assign(q.clone(), len, &m);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_add_truncated_assign(&q, len, m.clone());
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_add_truncated_assign(&q, len, &m);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // The result is reduced, and is the truncation of the whole modular result and the
            // modular result of the truncations.
            assert!(r.mod_is_reduced(&m));
            assert_eq!(mod_add_truncated_naive(&p, &q, len, &m), r);
            assert_eq!((&p).mod_add(&q, &m).truncate(len), r);
            assert_eq!(p.truncate(len).mod_add(q.truncate(len), &m), r);
            // It agrees with the whole modular result below the cut, and has nothing above it.
            assert!(r.eq_truncated(&(&p).mod_add(&q, &m), len));
            assert!(r.len() <= len);
            // Addition is commutative.
            assert_eq!((&q).mod_add_truncated(&p, len, &m), r);
            // Adding the negation gives zero.
            assert_eq!(
                (&p).mod_add_truncated((&p).mod_neg(&m), len, &m),
                NaturalPolynomial::ZERO
            );
            // A power-of-2 modulus gives the same result as mod_power_of_2_add_truncated.
            if let Some(pow) = m.checked_log_base_2() {
                assert_eq!((&p).mod_power_of_2_add_truncated(&q, len, pow), r);
            }
        },
    );

    natural_polynomial_natural_polynomial_natural_triple_gen_var_1().test_properties(
        |(p, q, m)| {
            // A zero length keeps nothing, and a length past both degrees keeps everything.
            assert_eq!((&p).mod_add_truncated(&q, 0, &m), NaturalPolynomial::ZERO);
            let len = p.len().max(q.len());
            assert_eq!((&p).mod_add_truncated(&q, len, &m), (&p).mod_add(&q, &m));
            assert_eq!(
                (&p).mod_add_truncated(&q, u64::MAX, &m),
                (&p).mod_add(&q, &m)
            );
        },
    );

    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<u64>().test_properties(
        |(p, q, len, m)| {
            // The u64 and Natural versions agree.
            assert_eq!(
                NaturalPolynomial::from((&p).mod_add_truncated(&q, len, m)),
                NaturalPolynomial::from(p).mod_add_truncated(
                    NaturalPolynomial::from(q),
                    len,
                    Natural::from(m)
                )
            );
        },
    );
}
