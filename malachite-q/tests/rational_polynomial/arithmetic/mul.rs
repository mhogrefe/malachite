// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{DivisibleBy, Square};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{Evaluate, Polynomial};
use malachite_nz::test_util::generators::integer_polynomial_pair_gen;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::rational_polynomial::arithmetic::mul::mul_divide_after;
use malachite_q::test_util::generators::{
    rational_polynomial_gen, rational_polynomial_pair_gen, rational_polynomial_rational_pair_gen,
    rational_polynomial_triple_gen,
};
use malachite_q::test_util::rational_polynomial::arithmetic::mul::{mul_naive, mul_then_reduce};

#[test]
fn test_mul() {
    let test = |s, t, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = RationalPolynomial::from_str(t).unwrap();
        // All four combinations of value and reference, and in place with both.
        let r = &p * &q;
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(&p * q.clone(), r);
        assert_eq!(p.clone() * &q, r);
        assert_eq!(p.clone() * q.clone(), r);
        let mut s = p.clone();
        s *= &q;
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s *= q.clone();
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mul_naive(&p, &q), r);
        assert_eq!(mul_then_reduce(&p, &q), r);
        assert_eq!(mul_divide_after(&p, &q), r);
    };
    // - self == Self::ZERO || other == Self::ZERO
    test("0", "0", "0");
    test("0", "1/2*x+1", "0");
    test("1/2*x+1", "0", "0");
    // - *b == 1u32
    // - *a == 1u32
    // - g == 1u32
    test("x+1", "x-1", "x^2-1");
    // - *b != 1u32
    test("x+1", "1/2*x", "1/2*x^2+1/2*x");
    // - *a != 1u32
    test("1/3*x+1/5", "x^2-1", "1/3*x^3+1/5*x^2-1/3*x-1/5");
    // - g != 1u32: the first numerator shares a factor with the second denominator.
    test("2*x+2", "1/2*x", "x^2+x");
    // The second numerator shares a factor with the first denominator.
    test("1/3*x", "3*x+6", "x^2+2*x");
    // Factors cancel both ways.
    test("2/3*x", "3/4*x+3/2", "1/2*x^2+x");
    // Constants.
    test("1/6", "3/4", "1/8");
    test("1/2*x+1/3", "x-1/2", "1/2*x^2+1/12*x-1/6");
    // Coefficients and denominators of many limbs.
    test(
        "1/1000000000000000000000*x",
        "3000000000000000000000*x+1",
        "3*x^2+1/1000000000000000000000*x",
    );
    test(
        "1000000000000000000000/7*x^2-1/1000000000000000000000",
        "7/1000000000000000000000*x",
        "x^3-7/1000000000000000000000000000000000000000000*x",
    );
}

#[test]
fn test_mul_self() {
    // A polynomial multiplied by itself, through the same reference.
    let test = |s, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let r = &p * &p;
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(mul_naive(&p, &p), r);
    };
    // - ptr::eq(self, other)
    test("0", "0");
    test("x-3", "x^2-6*x+9");
    test("1/2*x+1/3", "1/4*x^2+1/3*x+1/9");
}

#[test]
fn mul_properties() {
    rational_polynomial_pair_gen().test_properties(|(p, q)| {
        let r = &p * &q;
        assert!(r.is_valid());
        // The forms agree.
        assert_eq!(&p * q.clone(), r);
        assert_eq!(p.clone() * &q, r);
        assert_eq!(p.clone() * q.clone(), r);
        let mut s = p.clone();
        s *= &q;
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s *= q.clone();
        assert!(s.is_valid());
        assert_eq!(s, r);

        // The other algorithms agree.
        assert_eq!(mul_naive(&p, &q), r);
        assert_eq!(mul_then_reduce(&p, &q), r);
        assert_eq!(mul_divide_after(&p, &q), r);

        // Multiplication is commutative.
        assert_eq!(&q * &p, r);
        // The rationals have no zero divisors, so the degrees add.
        if p != RationalPolynomial::ZERO && q != RationalPolynomial::ZERO {
            assert_eq!(r.len(), p.len() + q.len() - 1);
        }
        // The denominator divides the product of the denominators.
        assert!((p.denominator_ref() * q.denominator_ref()).divisible_by(r.denominator_ref()));
    });

    rational_polynomial_gen().test_properties(|p| {
        assert_eq!(&p * RationalPolynomial::ZERO, RationalPolynomial::ZERO);
        assert_eq!(RationalPolynomial::ZERO * &p, RationalPolynomial::ZERO);
        let one = RationalPolynomial::from_str("1").unwrap();
        assert_eq!(&p * &one, p);
        assert_eq!(&one * &p, p);
        // Multiplying a polynomial by itself through one reference is squaring.
        let r = &p * &p;
        assert!(r.is_valid());
        assert_eq!(r, (&p).square());
        assert_eq!(r, &p * p.clone());
    });

    rational_polynomial_triple_gen().test_properties(|(p, q, r)| {
        // Multiplication is associative, and distributes over addition.
        assert_eq!(&(&p * &q) * &r, &p * &(&q * &r));
        assert_eq!(&p * &(&q + &r), &(&p * &q) + &(&p * &r));
    });

    rational_polynomial_rational_pair_gen().test_properties(|(p, x)| {
        // Evaluation is multiplicative; any fixed second polynomial will do.
        let q = RationalPolynomial::from_str("1/2*x^2-3").unwrap();
        assert_eq!(
            (&p * &q).evaluate(&x),
            (&p).evaluate(&x) * (&q).evaluate(&x)
        );
    });

    integer_polynomial_pair_gen().test_properties(|(p, q)| {
        // On polynomials with integer coefficients, this is the `IntegerPolynomial` operation.
        assert_eq!(
            RationalPolynomial::from(p.clone()) * RationalPolynomial::from(q.clone()),
            RationalPolynomial::from(p * q)
        );
    });
}
