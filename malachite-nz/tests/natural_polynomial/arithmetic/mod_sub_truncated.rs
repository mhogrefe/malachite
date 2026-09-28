// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{CheckedLogBase2, ModIsReduced, ModNeg, ModSub};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{
    EqTruncated, ModAddTruncated, ModPowerOf2SubTruncated, ModSubTruncated, ModSubTruncatedAssign,
    Polynomial,
};
use malachite_base::test_util::generators::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_sub_truncated::*;

#[test]
fn test_mod_sub_truncated() {
    let test = |s, t, len, m, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
        let m = Natural::from_str(m).unwrap();
        // Every combination of value and reference, and in place with all of them.
        let r = (&p).mod_sub_truncated(&q, len, &m);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(p.clone().mod_sub_truncated(q.clone(), len, m.clone()), r);
        assert_eq!(p.clone().mod_sub_truncated(q.clone(), len, &m), r);
        assert_eq!(p.clone().mod_sub_truncated(&q, len, m.clone()), r);
        assert_eq!(p.clone().mod_sub_truncated(&q, len, &m), r);
        assert_eq!((&p).mod_sub_truncated(q.clone(), len, m.clone()), r);
        assert_eq!((&p).mod_sub_truncated(q.clone(), len, &m), r);
        assert_eq!((&p).mod_sub_truncated(&q, len, m.clone()), r);
        let mut s = p.clone();
        s.mod_sub_truncated_assign(q.clone(), len, m.clone());
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_sub_truncated_assign(q.clone(), len, &m);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_sub_truncated_assign(&q, len, m.clone());
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_sub_truncated_assign(&q, len, &m);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mod_sub_truncated_naive(&p, &q, len, &m), r);
    };
    // A zero length, or m = 1, keeps nothing.
    test("0", "0", 0, "1", "0");
    test("0", "0", 3, "1", "0");
    test("x+1", "0", 0, "7", "0");
    // Truncation drops the high coefficients of either operand.
    test("x^2+1", "0", 2, "7", "1");
    test("0", "x^2+1", 3, "7", "6*x^2+6");
    // The quadratic and linear coefficients wrap around.
    test("x^3+2*x^2+x+5", "4*x^2+6*x+2", 3, "7", "5*x^2+2*x+3");
    test("x^3+2*x^2+x+5", "4*x^2+6*x+2", 1, "7", "3");
    // A length past both degrees is the whole difference.
    test("x^3+2*x^2+x+5", "4*x^2+6*x+2", 100, "7", "x^3+5*x^2+2*x+3");
    // Everything cancels.
    test("x^2+x+1", "x^2+x+1", 3, "7", "0");
    // The cut leading terms would have cancelled anyway.
    test("x^3+x", "x^3+2*x", 3, "7", "6*x");
    // Coefficients and a modulus of two limbs.
    test(
        "x",
        "18446744073709551616*x^2+18446744073709551616*x",
        3,
        "18446744073709551617",
        "x^2+2*x",
    );
}

#[test]
#[should_panic]
fn mod_sub_truncated_val_val_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_sub_truncated(q, 3, m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_val_val_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_sub_truncated(q, 3, m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_val_val_val_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = p.mod_sub_truncated(q, 3, m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_val_val_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_sub_truncated(q, 3, &m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_val_val_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_sub_truncated(q, 3, &m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_val_val_ref_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = p.mod_sub_truncated(q, 3, &m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_val_ref_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_sub_truncated(&q, 3, m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_val_ref_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_sub_truncated(&q, 3, m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_val_ref_val_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = p.mod_sub_truncated(&q, 3, m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_val_ref_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_sub_truncated(&q, 3, &m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_val_ref_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = p.mod_sub_truncated(&q, 3, &m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_val_ref_ref_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = p.mod_sub_truncated(&q, 3, &m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_ref_val_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_sub_truncated(q, 3, m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_ref_val_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_sub_truncated(q, 3, m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_ref_val_val_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = (&p).mod_sub_truncated(q, 3, m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_ref_val_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_sub_truncated(q, 3, &m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_ref_val_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_sub_truncated(q, 3, &m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_ref_val_ref_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = (&p).mod_sub_truncated(q, 3, &m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_ref_ref_val_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_sub_truncated(&q, 3, m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_ref_ref_val_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_sub_truncated(&q, 3, m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_ref_ref_val_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = (&p).mod_sub_truncated(&q, 3, m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_ref_ref_ref_self_fail() {
    // A coefficient of self is not reduced.
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_sub_truncated(&q, 3, &m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_ref_ref_ref_other_fail() {
    // A coefficient of other is not reduced.
    let p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    let _ = (&p).mod_sub_truncated(&q, 3, &m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_ref_ref_ref_zero_fail() {
    // The modulus is 0.
    let p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    let _ = (&p).mod_sub_truncated(&q, 3, &m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_assign_val_val_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_sub_truncated_assign(q, 3, m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_assign_val_val_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_sub_truncated_assign(q, 3, m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_assign_val_val_zero_fail() {
    // The modulus is 0.
    let mut p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    p.mod_sub_truncated_assign(q, 3, m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_assign_val_ref_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_sub_truncated_assign(q, 3, &m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_assign_val_ref_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_sub_truncated_assign(q, 3, &m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_assign_val_ref_zero_fail() {
    // The modulus is 0.
    let mut p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    p.mod_sub_truncated_assign(q, 3, &m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_assign_ref_val_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_sub_truncated_assign(&q, 3, m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_assign_ref_val_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_sub_truncated_assign(&q, 3, m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_assign_ref_val_zero_fail() {
    // The modulus is 0.
    let mut p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    p.mod_sub_truncated_assign(&q, 3, m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_assign_ref_ref_self_fail() {
    // A coefficient of self is not reduced.
    let mut p = NaturalPolynomial::from_str("8*x+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_sub_truncated_assign(&q, 3, &m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_assign_ref_ref_other_fail() {
    // A coefficient of other is not reduced.
    let mut p = NaturalPolynomial::from_str("x").unwrap();
    let q = NaturalPolynomial::from_str("8*x+1").unwrap();
    let m = Natural::from_str("7").unwrap();
    p.mod_sub_truncated_assign(&q, 3, &m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_assign_ref_ref_zero_fail() {
    // The modulus is 0.
    let mut p = NaturalPolynomial::from_str("0").unwrap();
    let q = NaturalPolynomial::from_str("0").unwrap();
    let m = Natural::from_str("0").unwrap();
    p.mod_sub_truncated_assign(&q, 3, &m);
}

#[test]
#[should_panic]
fn mod_sub_truncated_beyond_len_fail() {
    // A coefficient past the kept length is not reduced; the whole operand must be reduced.
    let p = NaturalPolynomial::from_str("8*x^2+1").unwrap();
    let q = NaturalPolynomial::from_str("x").unwrap();
    let _ = (&p).mod_sub_truncated(&q, 1, &Natural::from(7u32));
}

#[test]
fn mod_sub_truncated_properties() {
    natural_polynomial_pair_unsigned_natural_quadruple_gen_var_1().test_properties(
        |(p, q, len, m)| {
            let r = (&p).mod_sub_truncated(&q, len, &m);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!(p.clone().mod_sub_truncated(q.clone(), len, m.clone()), r);
            assert_eq!(p.clone().mod_sub_truncated(q.clone(), len, &m), r);
            assert_eq!(p.clone().mod_sub_truncated(&q, len, m.clone()), r);
            assert_eq!(p.clone().mod_sub_truncated(&q, len, &m), r);
            assert_eq!((&p).mod_sub_truncated(q.clone(), len, m.clone()), r);
            assert_eq!((&p).mod_sub_truncated(q.clone(), len, &m), r);
            assert_eq!((&p).mod_sub_truncated(&q, len, m.clone()), r);
            let mut s = p.clone();
            s.mod_sub_truncated_assign(q.clone(), len, m.clone());
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_sub_truncated_assign(q.clone(), len, &m);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_sub_truncated_assign(&q, len, m.clone());
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_sub_truncated_assign(&q, len, &m);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // The result is reduced, and is the truncation of the whole modular result and the
            // modular result of the truncations.
            assert!(r.mod_is_reduced(&m));
            assert_eq!(mod_sub_truncated_naive(&p, &q, len, &m), r);
            assert_eq!((&p).mod_sub(&q, &m).truncate(len), r);
            assert_eq!(p.truncate(len).mod_sub(q.truncate(len), &m), r);
            // It agrees with the whole modular result below the cut, and has nothing above it.
            assert!(r.eq_truncated(&(&p).mod_sub(&q, &m), len));
            assert!(r.len() <= len);
            // Swapping the operands negates the difference.
            assert_eq!((&q).mod_sub_truncated(&p, len, &m), (&r).mod_neg(&m));
            // A polynomial minus itself is zero.
            assert_eq!((&p).mod_sub_truncated(&p, len, &m), NaturalPolynomial::ZERO);
            // It is adding the negation.
            assert_eq!((&p).mod_add_truncated((&q).mod_neg(&m), len, &m), r);
            // A power-of-2 modulus gives the same result as mod_power_of_2_sub_truncated.
            if let Some(pow) = m.checked_log_base_2() {
                assert_eq!((&p).mod_power_of_2_sub_truncated(&q, len, pow), r);
            }
        },
    );

    natural_polynomial_natural_polynomial_natural_triple_gen_var_1().test_properties(
        |(p, q, m)| {
            // A zero length keeps nothing, and a length past both degrees keeps everything.
            assert_eq!((&p).mod_sub_truncated(&q, 0, &m), NaturalPolynomial::ZERO);
            let len = p.len().max(q.len());
            assert_eq!((&p).mod_sub_truncated(&q, len, &m), (&p).mod_sub(&q, &m));
            assert_eq!(
                (&p).mod_sub_truncated(&q, u64::MAX, &m),
                (&p).mod_sub(&q, &m)
            );
        },
    );

    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<u64>().test_properties(
        |(p, q, len, m)| {
            // The u64 and Natural versions agree.
            assert_eq!(
                NaturalPolynomial::from((&p).mod_sub_truncated(&q, len, m)),
                NaturalPolynomial::from(p).mod_sub_truncated(
                    NaturalPolynomial::from(q),
                    len,
                    Natural::from(m)
                )
            );
        },
    );
}
