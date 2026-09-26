// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{Evaluate, Polynomial};
use malachite_nz::test_util::generators::integer_polynomial_pair_gen;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_gen, rational_polynomial_pair_gen, rational_polynomial_rational_pair_gen,
    rational_polynomial_triple_gen,
};
use malachite_q::test_util::rational_polynomial::arithmetic::sub::{sub_cross_multiply, sub_naive};

#[test]
fn test_sub() {
    let test = |s, t, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = RationalPolynomial::from_str(t).unwrap();
        // All four combinations of value and reference, and in place with both.
        let r = &p - &q;
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(&p - q.clone(), r);
        assert_eq!(p.clone() - &q, r);
        assert_eq!(p.clone() - q.clone(), r);
        let mut s = p.clone();
        s -= &q;
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s -= q.clone();
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(sub_naive(&p, &q), r);
        assert_eq!(sub_cross_multiply(&p, &q), r);
    };
    // - a == *b
    // - denominator == 1u32
    test("0", "0", "0");
    // - a != *b
    // - a == 1u32 || *b == 1u32
    // - g == 1u32
    test("0", "1/2*x", "-1/2*x");
    test("1/2*x", "0", "1/2*x");
    test("x+1", "-x+1", "2*x");
    // The leading coefficients cancel.
    test("x^2+1", "x^2-x", "x+1");
    test("x", "1/2", "x-1/2");
    test("-1/5", "-3*x^2", "3*x^2-1/5");
    // - denominator != 1u32
    // - g == 1u32 when a == *b
    test("1/2*x", "-1/2", "1/2*x+1/2");
    // - g != 1u32 when a == *b
    test("1/4*x+1/4", "-1/4*x+3/4", "1/2*x-1/2");
    // Equal polynomials, held separately, reduce to 0/1.
    test("1/3*x", "1/3*x", "0");
    // The content of the difference's numerator is coprime to the denominator, found before every
    // coefficient is read.
    test(
        "1/6*x^3+1/6*x^2+1/6*x+1/6",
        "-1/3*x^3-5/6*x^2-5/6*x-1/6",
        "1/2*x^3+x^2+x+1/3",
    );
    // The content of the difference's numerator shares a factor with the denominator, found after
    // reading every coefficient.
    test(
        "1/4*x^4+1/4*x^3+1/4*x^2+1/4*x+1/4",
        "-1/4*x^4-1/4*x^3-1/4*x^2-1/4*x-1/4",
        "1/2*x^4+1/2*x^3+1/2*x^2+1/2*x+1/2",
    );
    // - a != 1u32 && *b != 1u32
    test("1/2*x", "1/3", "1/2*x-1/3");
    test("1/2*x^3", "-1/3*x", "1/2*x^3+1/3*x");
    test("1/3", "1/2*x^2+x", "-1/2*x^2-x+1/3");
    // - g != 1u32
    // - e == 1u32
    test("1/6*x", "-1/4", "1/6*x+1/4");
    // - e != 1u32
    test("1/6*x+1/6", "-1/10*x-3/10", "4/15*x+7/15");
    // A constant difference.
    test("1/6", "-1/3", "1/2");
    // The leading coefficients cancel, with different denominators.
    test("1/2*x^2+1/3", "1/2*x^2-1/5*x", "1/5*x+1/3");
    // Coefficients and denominators of many limbs.
    test(
        "1/1000000000000000000000*x",
        "1/3000000000000000000000",
        "1/1000000000000000000000*x-1/3000000000000000000000",
    );
    test(
        "1000000000000000000000/7*x^2-1/1000000000000000000000",
        "1000000000000000000000/7*x^2-1/1000000000000000000000*x",
        "1/1000000000000000000000*x-1/1000000000000000000000",
    );
}

#[test]
fn test_sub_self() {
    // A polynomial subtracted from itself, through the same reference.
    // - ptr::eq(self, other)
    let test = |s| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let r = &p - &p;
        assert!(r.is_valid());
        assert_eq!(r, RationalPolynomial::ZERO);
    };
    test("0");
    test("x-3");
    test("1/2*x+1/4");
}

#[test]
fn sub_properties() {
    rational_polynomial_pair_gen().test_properties(|(p, q)| {
        let r = &p - &q;
        assert!(r.is_valid());
        // The forms agree.
        assert_eq!(&p - q.clone(), r);
        assert_eq!(p.clone() - &q, r);
        assert_eq!(p.clone() - q.clone(), r);
        let mut s = p.clone();
        s -= &q;
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s -= q.clone();
        assert!(s.is_valid());
        assert_eq!(s, r);

        // The other algorithms agree.
        assert_eq!(sub_naive(&p, &q), r);
        assert_eq!(sub_cross_multiply(&p, &q), r);

        // Subtracting is adding the negation.
        assert_eq!(&p + &(-&q), r);
        // Swapping the operands negates the difference.
        assert_eq!(&q - &p, -&r);
        // Adding back the subtrahend gives back the minuend.
        assert_eq!(&r + &q, p);
        // The degree is at most the larger of the two.
        assert!(r.len() <= p.len().max(q.len()));
    });

    rational_polynomial_gen().test_properties(|p| {
        assert_eq!(&p - RationalPolynomial::ZERO, p);
        assert_eq!(RationalPolynomial::ZERO - &p, -&p);
        // Through one reference or through two, a polynomial minus itself is zero.
        assert_eq!(&p - &p, RationalPolynomial::ZERO);
        assert_eq!(&p - p.clone(), RationalPolynomial::ZERO);
    });

    rational_polynomial_triple_gen().test_properties(|(p, q, r)| {
        assert_eq!(&(&p - &q) - &r, &p - &(&q + &r));
    });

    rational_polynomial_rational_pair_gen().test_properties(|(p, x)| {
        // Evaluation is additive; any fixed second polynomial will do.
        let q = RationalPolynomial::from_str("1/2*x^2-3").unwrap();
        assert_eq!(
            (&p - &q).evaluate(&x),
            (&p).evaluate(&x) - (&q).evaluate(&x)
        );
    });

    integer_polynomial_pair_gen().test_properties(|(p, q)| {
        // On polynomials with integer coefficients, this is the `IntegerPolynomial` operation.
        assert_eq!(
            RationalPolynomial::from(p.clone()) - RationalPolynomial::from(q.clone()),
            RationalPolynomial::from(p - q)
        );
    });
}
