// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::basic::traits::{Two, Zero};
use malachite_base::polynomial::{ComposePowerOfX, ComposePowerOfXAssign, Evaluate, Polynomial};
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::rational_polynomial_unsigned_pair_gen_var_1;
use malachite_q::test_util::rational_polynomial::arithmetic::compose_power_of_x::*;

#[test]
fn test_compose_power_of_x() {
    let test = |s, k, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = (&p).compose_power_of_x(k);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone().compose_power_of_x(k), q);
        let mut r = p.clone();
        r.compose_power_of_x_assign(k);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(compose_power_of_x_naive(&p, k), q);
    };
    // The zero polynomial stays zero. With k = 0, this is the constant p(1).
    test("0", 0, "0");
    test("0", 3, "0");
    // Composing with x, or composing a constant, changes nothing.
    test("1/2*x^2-1/3", 1, "1/2*x^2-1/3");
    test("-5/3", 4, "-5/3");
    // The coefficient of x^i moves to x^(ik).
    test("1/2*x^2-1/3", 2, "1/2*x^4-1/3");
    test("1/2*x^2-1/3*x", 3, "1/2*x^6-1/3*x^3");
    test("1/2*x^2+1/3*x+1/6", 0, "1");
    test("1/2*x^2-1/3", 0, "1/6");
    test("1/2*x-1/2", 0, "0");
}

#[test]
fn compose_power_of_x_properties() {
    rational_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, k)| {
        let q = (&p).compose_power_of_x(k);
        assert!(q.is_valid());
        // The forms agree.
        assert_eq!(p.clone().compose_power_of_x(k), q);
        let mut r = p.clone();
        r.compose_power_of_x_assign(k);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(compose_power_of_x_naive(&p, k), q);

        if k == 0 {
            // The result is a constant.
            assert!(q.len() <= 1);
        } else {
            // The coefficient of x^i moves to x^(ik), and the others are zero.
            if let Some(d) = p.degree() {
                assert_eq!(q.degree(), Some(d * k));
            } else {
                assert_eq!(q, RationalPolynomial::ZERO);
            }
            for i in 0..q.len() {
                if i % k == 0 {
                    assert_eq!(q.coefficient(i), p.coefficient(i / k));
                } else {
                    assert!(q.coefficient(i) == 0u32);
                }
            }
            // Composing with x^k and then x^j is composing with x^(jk).
            assert_eq!((&q).compose_power_of_x(k), (&p).compose_power_of_x(k * k));
        }
        // Composing with x changes nothing.
        assert_eq!((&p).compose_power_of_x(1), p);
        // Evaluation: q(2) = p(2^k).
        let two = Rational::TWO;
        assert_eq!((&q).evaluate(&two), (&p).evaluate(&(&two).pow(k)));
    });
}
