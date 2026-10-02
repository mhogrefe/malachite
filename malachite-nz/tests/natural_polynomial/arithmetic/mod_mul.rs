// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    CheckedLogBase2, Mod, ModIsReduced, ModMul, ModMulAssign, ModPowerOf2Mul,
};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_mul::mod_mul_naive;

// Checks every form of `mod_mul` and `mod_mul_assign` against `r`.
fn check_forms(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural, r: &NaturalPolynomial) {
    assert_eq!(&p.clone().mod_mul(q.clone(), m.clone()), r);
    assert_eq!(&p.clone().mod_mul(q.clone(), m), r);
    assert_eq!(&p.clone().mod_mul(q, m.clone()), r);
    assert_eq!(&p.clone().mod_mul(q, m), r);
    assert_eq!(&p.mod_mul(q.clone(), m.clone()), r);
    assert_eq!(&p.mod_mul(q.clone(), m), r);
    assert_eq!(&p.mod_mul(q, m.clone()), r);
    assert_eq!(&p.mod_mul(q, m), r);
    let mut s = p.clone();
    s.mod_mul_assign(q.clone(), m.clone());
    assert!(s.is_valid());
    assert_eq!(&s, r);
    let mut s = p.clone();
    s.mod_mul_assign(q.clone(), m);
    assert_eq!(&s, r);
    let mut s = p.clone();
    s.mod_mul_assign(q, m.clone());
    assert_eq!(&s, r);
    let mut s = p.clone();
    s.mod_mul_assign(q, m);
    assert_eq!(&s, r);
}

#[test]
fn test_mod_mul() {
    let test = |s, t, m: u32, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
        let m = Natural::from(m);
        let r = (&p).mod_mul(&q, &m);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        check_forms(&p, &q, &m, &r);
        assert_eq!(mod_mul_naive(&p, &q, &m), r);
    };
    test("0", "x+1", 7, "0");
    test("x^2+3*x+2", "2*x+5", 7, "2*x^3+4*x^2+5*x+3");
    // A constant, which the forms taking the other polynomial by value multiply in place.
    test("x^2+3*x+2", "4", 7, "4*x^2+5*x+1");
    // Modulo 6, the leading coefficient vanishes and the degree drops.
    test("2*x+1", "3*x+1", 6, "5*x+1");
    // Every coefficient vanishes.
    test("2*x+2", "3", 6, "0");
    test("0", "0", 1, "0");
}

#[test]
#[should_panic]
fn mod_mul_fail_1() {
    NaturalPolynomial::from_str("x+7").unwrap().mod_mul(
        NaturalPolynomial::from_str("x+1").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_mul_fail_2() {
    NaturalPolynomial::from_str("x+1").unwrap().mod_mul(
        NaturalPolynomial::from_str("7*x+1").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_mul_fail_3() {
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_mul(NaturalPolynomial::from_str("x+1").unwrap(), Natural::ZERO);
}

#[test]
fn mod_mul_properties() {
    natural_polynomial_natural_polynomial_natural_triple_gen_var_1().test_properties(
        |(p, q, m)| {
            let r = (&p).mod_mul(&q, &m);
            assert!(r.is_valid());
            assert!(r.mod_is_reduced(&m));
            check_forms(&p, &q, &m, &r);
            assert_eq!(mod_mul_naive(&p, &q, &m), r);
            assert_eq!((&p * &q).mod_op(&m), r);
            // Multiplication is commutative.
            assert_eq!((&q).mod_mul(&p, &m), r);
            // Modulo a power of 2, it agrees with `mod_power_of_2_mul`.
            if let Some(pow) = (&m).checked_log_base_2() {
                assert_eq!((&p).mod_power_of_2_mul(&q, pow), r);
            }
        },
    );
}
