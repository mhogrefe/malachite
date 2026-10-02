// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{DivisibleBy, LcmAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::{Derivative, Integral, IntegralAssign, Polynomial};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_polynomial_gen;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{rational_polynomial_gen, rational_polynomial_pair_gen};
use malachite_q::test_util::rational_polynomial::arithmetic::integral::integral_naive;

#[test]
fn test_integral() {
    let test = |s, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let r = (&p).integral();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(p.clone().integral(), r);
        let mut q = p.clone();
        q.integral_assign();
        assert!(q.is_valid());
        assert_eq!(q, r);
        assert_eq!(integral_naive(&p), r);
    };
    // - *self == RationalPolynomial::ZERO
    test("0", "0");
    // - *self != RationalPolynomial::ZERO
    // - a constant, so the only coefficient moves to x without a divisor
    test("5/7", "5/7*x");
    // - g == k: the divisor is absorbed by the coefficient
    test("2*x", "x^2");
    // - g != k
    // - g == 1: the whole divisor goes into the denominator
    // - d != divisor: the denominator grows
    // - t != 1u32
    test("x", "1/2*x^2");
    // - g != 1: part of the divisor is absorbed
    test("6*x^3", "3/2*x^4");
    // The least common multiple of the leftover divisors, 6, scales both coefficients.
    test("x^2+x", "1/3*x^3+1/2*x^2");
    // - d == divisor: the leftover divisor 2 already divides t = 4
    test("x^3+x", "1/4*x^4+1/2*x^2");
    // A denominator to begin with.
    test("1/3*x^2+2*x+1", "1/9*x^3+x^2+x");
    test("3*x^2+x+1/2", "x^3+1/2*x^2+1/2*x");
    test("-12*x^5+5/2*x^4-9/7", "-2*x^6+1/2*x^5-9/7*x");
    // Coefficients of many limbs.
    test(
        "1000000000000000000000*x^2+1/7",
        "1000000000000000000000/3*x^3+1/7*x",
    );
    // Many divisors, whose least common multiple is 2520.
    test(
        "x^9+x^8+x^7+x^6+x^5+x^4+x^3+x^2+x+1",
        "1/10*x^10+1/9*x^9+1/8*x^8+1/7*x^7+1/6*x^6+1/5*x^5+1/4*x^4+1/3*x^3+1/2*x^2+x",
    );
}

#[test]
fn integral_properties() {
    rational_polynomial_gen().test_properties(|p| {
        let r = (&p).integral();
        assert!(r.is_valid());
        // The forms agree.
        assert_eq!(p.clone().integral(), r);
        let mut q = p.clone();
        q.integral_assign();
        assert!(q.is_valid());
        assert_eq!(q, r);

        // It is the coefficient-wise integral.
        assert_eq!(integral_naive(&p), r);
        // Its constant term is zero, and it is one coefficient longer, unless it is zero.
        assert_eq!(r.coefficient(0), Rational::ZERO);
        if p != RationalPolynomial::ZERO {
            assert_eq!(r.len(), p.len() + 1);
        }
        // Differentiating it gives back the polynomial, and integrating the derivative gives back
        // the polynomial without its constant term.
        assert_eq!((&r).derivative(), p);
        let constant = RationalPolynomial::from_str(&p.coefficient(0).to_string()).unwrap();
        assert_eq!((&p).derivative().integral(), &p - constant);
    });

    rational_polynomial_pair_gen().test_properties(|(p, q)| {
        // Integration is additive.
        assert_eq!((&p + &q).integral(), (&p).integral() + (&q).integral());
    });

    integer_polynomial_gen().test_properties(|p| {
        // The denominator of the integral of a polynomial with integer coefficients divides the
        // least common multiple of 1, 2, ..., n, where n is the length of the polynomial.
        let r = RationalPolynomial::from(p.clone()).integral();
        assert!(r.is_valid());
        let mut lcm = Natural::ONE;
        for i in 1..=p.len() {
            lcm.lcm_assign(Natural::from(i));
        }
        assert!(lcm.divisible_by(r.denominator_ref()));
        assert_eq!(integral_naive(&RationalPolynomial::from(p)), r);
    });
}
