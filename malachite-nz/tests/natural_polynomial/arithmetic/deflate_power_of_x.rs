// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::polynomial::{
    ComposePowerOfX, DeflatePowerOfX, DeflatePowerOfXAssign, ExponentGcd, Polynomial,
};
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_unsigned_pair_gen_var_1, natural_polynomial_unsigned_pair_gen_var_2,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::deflate_power_of_x::*;

#[test]
fn test_deflate_power_of_x() {
    let test = |s, n, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = (&p).deflate_power_of_x(n);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone().deflate_power_of_x(n), q);
        let mut r = p.clone();
        r.deflate_power_of_x_assign(n);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(deflate_power_of_x_naive(&p, n), Some(q));
    };
    // The zero polynomial and constants deflate to themselves.
    test("0", 1, "0");
    test("0", 5, "0");
    test("5", 3, "5");
    // Deflating by 1 changes nothing.
    test("x^2+3*x+2", 1, "x^2+3*x+2");
    // The coefficient of x^(in) moves to x^i.
    test("x^4+3*x^2+2", 2, "x^2+3*x+2");
    test("x^6+3*x^3+2", 3, "x^2+3*x+2");
    test("x^6+1", 2, "x^3+1");
    test("x^6+1", 3, "x^2+1");
    test("x^6+1", 6, "x+1");
    test("x^5", 5, "x");
    test(
        "1000000000000000000000*x^4+1",
        2,
        "1000000000000000000000*x^2+1",
    );
}

#[test]
#[should_panic]
fn deflate_power_of_x_fail_1() {
    // Deflating by 0
    NaturalPolynomial::from_str("x^2+1")
        .unwrap()
        .deflate_power_of_x(0);
}

#[test]
#[should_panic]
fn deflate_power_of_x_fail_2() {
    // Deflating a constant by 0
    (&NaturalPolynomial::from_str("5").unwrap()).deflate_power_of_x(0);
}

#[test]
#[should_panic]
fn deflate_power_of_x_fail_3() {
    // The leading exponent is not a multiple of n
    (&NaturalPolynomial::from_str("x^3+1").unwrap()).deflate_power_of_x(2);
}

#[test]
#[should_panic]
fn deflate_power_of_x_fail_4() {
    // A middle exponent is not a multiple of n
    let mut p = NaturalPolynomial::from_str("x^4+x+1").unwrap();
    p.deflate_power_of_x_assign(2);
}

#[test]
fn deflate_power_of_x_properties() {
    natural_polynomial_unsigned_pair_gen_var_2().test_properties(|(p, n)| {
        let q = (&p).deflate_power_of_x(n);
        assert!(q.is_valid());
        // The forms agree.
        assert_eq!(p.clone().deflate_power_of_x(n), q);
        let mut r = p.clone();
        r.deflate_power_of_x_assign(n);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(deflate_power_of_x_naive(&p, n), Some(q.clone()));

        // It inverts composition with x^n.
        assert_eq!((&q).compose_power_of_x(n), p);
        assert_eq!(q.degree(), p.degree().map(|d| d / n));
        // The exponent GCD is divided by n.
        let g = p.exponent_gcd();
        assert!(g.divisible_by(n));
        assert_eq!(q.exponent_gcd(), g / n);
        if g != 0 {
            // Deflating by the exponent GCD leaves an exponent GCD of 1.
            assert_eq!((&p).deflate_power_of_x(g).exponent_gcd(), 1);
        }
    });

    natural_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, k)| {
        let n = k + 1;
        // Deflation exists exactly when n divides the exponent GCD, or the polynomial is constant.
        let q = deflate_power_of_x_naive(&p, n);
        assert_eq!(
            q.is_some(),
            p.len() <= 1 || p.exponent_gcd().divisible_by(n)
        );
        if let Some(q) = q {
            assert_eq!((&p).deflate_power_of_x(n), q);
        }
        // Deflating by 1 changes nothing.
        assert_eq!((&p).deflate_power_of_x(1), p);
        // Deflating by n undoes composing with x^n.
        assert_eq!((&p).compose_power_of_x(n).deflate_power_of_x(n), p);
    });
}
