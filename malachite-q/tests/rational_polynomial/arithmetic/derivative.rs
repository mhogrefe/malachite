// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{Derivative, DerivativeAssign, MulPowerOfX, Polynomial};
use malachite_nz::test_util::generators::integer_polynomial_gen;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{rational_polynomial_gen, rational_polynomial_pair_gen};
use malachite_q::test_util::rational_polynomial::arithmetic::derivative::derivative_naive;

#[test]
fn test_derivative() {
    let test = |s, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = (&p).derivative();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone().derivative(), q);
        let mut r = p.clone();
        r.derivative_assign();
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(derivative_naive(&p), q);
    };
    // A constant polynomial, including zero, has derivative zero.
    test("0", "0");
    test("5/3", "0");
    test("x", "1");
    test("1/2*x^2+1/3*x+1", "x+1/3");
    // The multipliers can cancel factors of the denominator.
    test("1/2*x^2+1/2", "x");
    test("1/3*x^3", "x^2");
    test("1/6*x^3-1/4*x^2", "1/2*x^2-1/2*x");
    test("-1/10*x^5+x", "-1/2*x^4+1");
}

#[test]
fn derivative_properties() {
    rational_polynomial_gen().test_properties(|p| {
        let q = (&p).derivative();
        assert!(q.is_valid());
        // The forms agree.
        assert_eq!(p.clone().derivative(), q);
        let mut r = p.clone();
        r.derivative_assign();
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(derivative_naive(&p), q);

        // The degree drops by one, and a constant has derivative zero.
        match p.degree() {
            Some(d) if d > 0 => assert_eq!(q.degree(), Some(d - 1)),
            _ => assert_eq!(q, RationalPolynomial::ZERO),
        }
        // The product rule for x * p: (xp)' = p + xp'.
        assert_eq!(
            (&p).mul_power_of_x(1).derivative(),
            &p + (&q).mul_power_of_x(1)
        );
        // It commutes with negation.
        assert_eq!((-&p).derivative(), -&q);
    });

    rational_polynomial_pair_gen().test_properties(|(p, q)| {
        // Differentiation is linear.
        assert_eq!((&p + &q).derivative(), p.derivative() + q.derivative());
    });

    integer_polynomial_gen().test_properties(|p| {
        // It agrees with the derivative of the same polynomial as an IntegerPolynomial.
        assert_eq!(
            RationalPolynomial::from(p.clone()).derivative(),
            RationalPolynomial::from(p.derivative())
        );
    });
}
