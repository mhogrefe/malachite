// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Mod, ModIsReduced};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{Derivative, ModDerivative, ModDerivativeAssign, Polynomial};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_2;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_natural_natural_triple_gen_var_1;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_derivative::mod_derivative_naive;

#[test]
fn test_mod_derivative() {
    let test = |s, m, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let q = (&p).mod_derivative(&m);
        assert!(q.is_valid());
        assert!(q.mod_is_reduced(&m));
        assert_eq!(q.to_string(), out);
        assert_eq!((&p).mod_derivative(m.clone()), q);
        assert_eq!(p.clone().mod_derivative(&m), q);
        assert_eq!(p.clone().mod_derivative(m.clone()), q);
        let mut r = p.clone();
        r.mod_derivative_assign(&m);
        assert!(r.is_valid());
        assert_eq!(r, q);
        let mut r = p.clone();
        r.mod_derivative_assign(m.clone());
        assert_eq!(r, q);
        assert_eq!(mod_derivative_naive(&p, &m), q);
    };
    // A constant polynomial, including zero, has derivative zero.
    test("0", "1", "0");
    test("5", "7", "0");
    test("x^3+3*x^2+2*x+5", "1000000000000000000000", "3*x^2+6*x+2");
    // Coefficients can vanish, and the derivative can lose more than one degree.
    test("x^3+3*x^2+2*x+5", "6", "3*x^2+2");
    test("x^3+2*x+1", "3", "2");
    test("2*x^2+x", "4", "1");
    test("x^2", "2", "0");
}

#[test]
#[should_panic]
fn mod_derivative_fail_1() {
    NaturalPolynomial::from_str("10*x+1")
        .unwrap()
        .mod_derivative(Natural::from(10u32));
}

#[test]
#[should_panic]
fn mod_derivative_fail_2() {
    (&NaturalPolynomial::from_str("10*x+1").unwrap()).mod_derivative(&Natural::from(10u32));
}

#[test]
#[should_panic]
fn mod_derivative_fail_3() {
    let mut p = NaturalPolynomial::from_str("10*x+1").unwrap();
    p.mod_derivative_assign(&Natural::from(10u32));
}

#[test]
#[should_panic]
fn mod_derivative_fail_4() {
    NaturalPolynomial::ZERO.mod_derivative(Natural::ZERO);
}

#[test]
fn mod_derivative_properties() {
    natural_polynomial_natural_natural_triple_gen_var_1().test_properties(|(p, _, m)| {
        let q = (&p).mod_derivative(&m);
        assert!(q.is_valid());
        assert!(q.mod_is_reduced(&m));
        // The forms agree.
        assert_eq!((&p).mod_derivative(m.clone()), q);
        assert_eq!(p.clone().mod_derivative(&m), q);
        assert_eq!(p.clone().mod_derivative(m.clone()), q);
        let mut r = p.clone();
        r.mod_derivative_assign(&m);
        assert!(r.is_valid());
        assert_eq!(r, q);
        let mut r = p.clone();
        r.mod_derivative_assign(m.clone());
        assert_eq!(r, q);
        assert_eq!(mod_derivative_naive(&p, &m), q);

        // It is the derivative over the naturals, reduced.
        assert_eq!((&p).derivative().mod_op(&m), q);
        // The degree drops by at least one.
        assert!(q.len() < p.len() || p.len() == 0);
    });

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<u64>().test_properties(|(p, _, m)| {
        // The u64 and Natural versions agree.
        assert_eq!(
            NaturalPolynomial::from((&p).mod_derivative(m)),
            NaturalPolynomial::from(p).mod_derivative(Natural::from(m))
        );
    });
}
