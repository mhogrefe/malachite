// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{CheckedLogBase2, Mod, ModIsReduced, ModMul};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{
    ModMulTruncated, ModMulTruncatedAssign, ModPowerOf2MulTruncated, MulTruncated, Polynomial,
};
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_mul_truncated::*;

// Checks every form of `mod_mul_truncated` and `mod_mul_truncated_assign` against `r`.
fn check_forms(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    len: u64,
    m: &Natural,
    r: &NaturalPolynomial,
) {
    assert_eq!(&p.clone().mod_mul_truncated(q.clone(), len, m.clone()), r);
    assert_eq!(&p.clone().mod_mul_truncated(q.clone(), len, m), r);
    assert_eq!(&p.clone().mod_mul_truncated(q, len, m.clone()), r);
    assert_eq!(&p.clone().mod_mul_truncated(q, len, m), r);
    assert_eq!(&p.mod_mul_truncated(q.clone(), len, m.clone()), r);
    assert_eq!(&p.mod_mul_truncated(q.clone(), len, m), r);
    assert_eq!(&p.mod_mul_truncated(q, len, m.clone()), r);
    assert_eq!(&p.mod_mul_truncated(q, len, m), r);
    let mut s = p.clone();
    s.mod_mul_truncated_assign(q.clone(), len, m.clone());
    assert!(s.is_valid());
    assert_eq!(&s, r);
    let mut s = p.clone();
    s.mod_mul_truncated_assign(q.clone(), len, m);
    assert_eq!(&s, r);
    let mut s = p.clone();
    s.mod_mul_truncated_assign(q, len, m.clone());
    assert_eq!(&s, r);
    let mut s = p.clone();
    s.mod_mul_truncated_assign(q, len, m);
    assert_eq!(&s, r);
}

#[test]
fn test_mod_mul_truncated() {
    let test = |s, t, len: u64, m: u32, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
        let m = Natural::from(m);
        let r = (&p).mod_mul_truncated(&q, len, &m);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        check_forms(&p, &q, len, &m, &r);
        assert_eq!(mod_mul_truncated_naive(&p, &q, len, &m), r);
    };
    test("x^2+3*x+2", "2*x+5", 0, 7, "0");
    test("x^2+3*x+2", "2*x+5", 2, 7, "5*x+3");
    test("x^2+3*x+2", "2*x+5", 10, 7, "2*x^3+4*x^2+5*x+3");
    // A constant, which the forms taking the other polynomial by value multiply in place.
    test("x^2+3*x+2", "4", 2, 7, "5*x+1");
    // The linear coefficient vanishes, and is trimmed.
    test("x+6", "x+1", 2, 7, "6");
}

#[test]
#[should_panic]
fn mod_mul_truncated_fail_1() {
    NaturalPolynomial::from_str("x+7")
        .unwrap()
        .mod_mul_truncated(
            NaturalPolynomial::from_str("x+1").unwrap(),
            2,
            Natural::from(7u32),
        );
}

#[test]
#[should_panic]
fn mod_mul_truncated_fail_2() {
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_mul_truncated(
            NaturalPolynomial::from_str("x+1").unwrap(),
            2,
            Natural::ZERO,
        );
}

#[test]
fn mod_mul_truncated_properties() {
    natural_polynomial_pair_unsigned_natural_quadruple_gen_var_1().test_properties(
        |(p, q, len, m)| {
            let r = (&p).mod_mul_truncated(&q, len, &m);
            assert!(r.is_valid());
            assert!(r.mod_is_reduced(&m));
            check_forms(&p, &q, len, &m, &r);
            assert_eq!(mod_mul_truncated_naive(&p, &q, len, &m), r);
            assert_eq!((&p).mul_truncated(&q, len).mod_op(&m), r);
            assert_eq!((&p).mod_mul(&q, &m).truncate(len), r);
            assert_eq!((&q).mod_mul_truncated(&p, len, &m), r);
            if let Some(pow) = (&m).checked_log_base_2() {
                assert_eq!((&p).mod_power_of_2_mul_truncated(&q, len, pow), r);
            }
            assert!(r.len() <= len);
        },
    );
}
