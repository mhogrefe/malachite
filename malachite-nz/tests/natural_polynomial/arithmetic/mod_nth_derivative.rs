// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Factorial, Mod, ModIsReduced};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{
    ModDerivative, ModNthDerivative, ModNthDerivativeAssign, NthDerivative,
};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_5;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_unsigned_natural_triple_gen_var_2;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_nth_derivative::*;

#[test]
fn test_mod_nth_derivative() {
    let test = |s, n, m, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let q = (&p).mod_nth_derivative(n, &m);
        assert!(q.is_valid());
        assert!(q.mod_is_reduced(&m));
        assert_eq!(q.to_string(), out);
        assert_eq!((&p).mod_nth_derivative(n, m.clone()), q);
        assert_eq!(p.clone().mod_nth_derivative(n, &m), q);
        assert_eq!(p.clone().mod_nth_derivative(n, m.clone()), q);
        let mut r = p.clone();
        r.mod_nth_derivative_assign(n, &m);
        assert!(r.is_valid());
        assert_eq!(r, q);
        let mut r = p.clone();
        r.mod_nth_derivative_assign(n, m.clone());
        assert_eq!(r, q);
        assert_eq!(mod_nth_derivative_naive(&p, n, &m), q);
    };
    test("0", 3, "1", "0");
    test("x^4+3*x^3+2*x+4", 0, "5", "x^4+3*x^3+2*x+4");
    test("x^4+3*x^3+2*x+4", 1, "5", "4*x^3+4*x^2+2");
    test("x^4+3*x^3+2*x+4", 2, "5", "2*x^2+3*x");
    test("x^4+3*x^3+2*x+4", 4, "5", "4");
    test("x^4+3*x^3+2*x+4", 5, "5", "0");
    // 5 divides 5!.
    test("x^9+x^5", 5, "5", "0");
    // 6 divides 3!.
    test("x^4+x^3", 3, "6", "0");
    // 8 does not divide 3!, but it divides 4 * 3 * 2.
    test("x^4+x^3", 3, "8", "6");
    test(
        "x^4+3*x^3+2*x+4",
        2,
        "1000000000000000000000",
        "12*x^2+18*x",
    );
}

#[test]
#[should_panic]
fn mod_nth_derivative_fail_1() {
    NaturalPolynomial::from_str("10*x+1")
        .unwrap()
        .mod_nth_derivative(1, Natural::from(10u32));
}

#[test]
#[should_panic]
fn mod_nth_derivative_fail_2() {
    (&NaturalPolynomial::from_str("10*x+1").unwrap()).mod_nth_derivative(1, &Natural::from(10u32));
}

#[test]
#[should_panic]
fn mod_nth_derivative_fail_3() {
    let mut p = NaturalPolynomial::from_str("10*x+1").unwrap();
    p.mod_nth_derivative_assign(1, &Natural::from(10u32));
}

#[test]
#[should_panic]
fn mod_nth_derivative_fail_4() {
    NaturalPolynomial::ZERO.mod_nth_derivative(1, Natural::ZERO);
}

#[test]
fn mod_nth_derivative_properties() {
    natural_polynomial_unsigned_natural_triple_gen_var_2().test_properties(|(p, n, m)| {
        let q = (&p).mod_nth_derivative(n, &m);
        assert!(q.is_valid());
        assert!(q.mod_is_reduced(&m));
        // The forms agree.
        assert_eq!((&p).mod_nth_derivative(n, m.clone()), q);
        assert_eq!(p.clone().mod_nth_derivative(n, &m), q);
        assert_eq!(p.clone().mod_nth_derivative(n, m.clone()), q);
        let mut r = p.clone();
        r.mod_nth_derivative_assign(n, &m);
        assert!(r.is_valid());
        assert_eq!(r, q);
        let mut r = p.clone();
        r.mod_nth_derivative_assign(n, m.clone());
        assert_eq!(r, q);
        assert_eq!(mod_nth_derivative_naive(&p, n, &m), q);

        // It is the nth derivative over the naturals, reduced.
        assert_eq!((&p).nth_derivative(n).mod_op(&m), q);
        // Differentiating once more is the (n + 1)th derivative.
        assert_eq!((&q).mod_derivative(&m), (&p).mod_nth_derivative(n + 1, &m));
        // If m divides n!, the result is zero.
        if Natural::factorial(n) % &m == 0u32 {
            assert_eq!(q, NaturalPolynomial::ZERO);
        }
    });

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_5::<u64>().test_properties(|(p, n, m)| {
        // The u64 and Natural versions agree.
        assert_eq!(
            NaturalPolynomial::from((&p).mod_nth_derivative(n, m)),
            NaturalPolynomial::from(p).mod_nth_derivative(n, Natural::from(m))
        );
    });
}
