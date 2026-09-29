// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{Derivative, NthDerivative, NthDerivativeAssign, Polynomial};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_gen, natural_polynomial_unsigned_pair_gen_var_1,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::nth_derivative::nth_derivative_naive;

#[test]
fn test_nth_derivative() {
    let test = |s, n, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = (&p).nth_derivative(n);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone().nth_derivative(n), q);
        let mut r = p.clone();
        r.nth_derivative_assign(n);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(nth_derivative_naive(&p, n), q);
    };
    // The zeroth derivative is the polynomial itself.
    test("x^4+3*x^3+2*x+5", 0, "x^4+3*x^3+2*x+5");
    test("x^4+3*x^3+2*x+5", 1, "4*x^3+9*x^2+2");
    test("x^4+3*x^3+2*x+5", 2, "12*x^2+18*x");
    test("x^4+3*x^3+2*x+5", 3, "24*x+18");
    test("x^4+3*x^3+2*x+5", 4, "24");
    // A polynomial of degree less than n has nth derivative zero.
    test("x^4+3*x^3+2*x+5", 5, "0");
    test("0", 3, "0");
    test("x^10", 10, "3628800");
    test("1000000000000000000000*x^3", 2, "6000000000000000000000*x");
}

#[test]
fn nth_derivative_properties() {
    natural_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, n)| {
        let q = (&p).nth_derivative(n);
        assert!(q.is_valid());
        // The forms agree.
        assert_eq!(p.clone().nth_derivative(n), q);
        let mut r = p.clone();
        r.nth_derivative_assign(n);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(nth_derivative_naive(&p, n), q);

        // The degree drops by n, and a polynomial of degree less than n has nth derivative zero.
        match p.degree() {
            Some(d) if d >= n => assert_eq!(q.degree(), Some(d - n)),
            _ => assert_eq!(q, NaturalPolynomial::ZERO),
        }
        // Differentiating once more is the (n + 1)th derivative.
        assert_eq!((&q).derivative(), (&p).nth_derivative(n + 1));
        // It agrees with the nth derivative of the same polynomial as an IntegerPolynomial.
        assert_eq!(
            IntegerPolynomial::from(p.clone()).nth_derivative(n),
            IntegerPolynomial::from(q.clone())
        );
    });

    natural_polynomial_gen().test_properties(|p| {
        // The zeroth derivative is the polynomial itself, and the first is the derivative.
        assert_eq!((&p).nth_derivative(0), p);
        assert_eq!((&p).nth_derivative(1), (&p).derivative());
    });
}
