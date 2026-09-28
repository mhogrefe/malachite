// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::polynomial::{DivPowerOfX, DivPowerOfXAssign, MulPowerOfX, Polynomial};
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::rational_polynomial_unsigned_pair_gen_var_1;
use malachite_q::test_util::rational_polynomial::arithmetic::div_power_of_x::*;

#[test]
fn test_div_power_of_x() {
    let test = |s, n, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = (&p).div_power_of_x(n);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone().div_power_of_x(n), q);
        let mut r = p.clone();
        r.div_power_of_x_assign(n);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(div_power_of_x_naive(&p, n), q);
    };
    // The zero polynomial stays zero.
    test("0", 0, "0");
    test("0", 5, "0");
    // Dividing by x^0 changes nothing.
    test("1/2*x^3-1/3*x^2+x", 0, "1/2*x^3-1/3*x^2+x");
    test("1/2*x^3-1/3*x^2+x", 1, "1/2*x^2-1/3*x+1");
    test("1/2*x^3-1/3*x^2+x", 2, "1/2*x-1/3");
    // Dividing by a power at least the length gives zero.
    test("1/2*x^3-1/3*x^2+x", 4, "0");
    // Dropping the constant term 3/5 leaves (10x + 5)/5, which reduces to 2x + 1.
    test("2*x^2+x+3/5", 1, "2*x+1");
    // Dropping the constant term leaves x/6, still in lowest terms.
    test("1/6*x^2+1/3", 1, "1/6*x");
    test(
        "1/1000000000000000000000*x^2+1/3",
        2,
        "1/1000000000000000000000",
    );
}

#[test]
fn div_power_of_x_properties() {
    rational_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, n)| {
        let q = (&p).div_power_of_x(n);
        assert!(q.is_valid());
        // The forms agree.
        assert_eq!(p.clone().div_power_of_x(n), q);
        let mut r = p.clone();
        r.div_power_of_x_assign(n);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(div_power_of_x_naive(&p, n), q);

        // The coefficients move down by n places.
        assert_eq!(q.len(), p.len().saturating_sub(n));
        for i in 0..q.len() {
            assert_eq!(q.coefficient(i), p.coefficient(i + n));
        }
        // Dividing after multiplying by x^n gives the polynomial back.
        assert_eq!((&p).mul_power_of_x(n).div_power_of_x(n), p);
        // Multiplying back by x^n and adding the dropped low part gives the polynomial back.
        assert_eq!((&q).mul_power_of_x(n) + p.truncate(n), p);
        // Dividing by x^n twice is dividing by x^(2n).
        assert_eq!((&q).div_power_of_x(n), (&p).div_power_of_x(2 * n));
        // Dividing by x^0 changes nothing.
        assert_eq!((&p).div_power_of_x(0), p);
        // On polynomials with integer coefficients, this is the IntegerPolynomial operation.
        if *p.denominator_ref() == 1u32 {
            assert_eq!(
                q,
                RationalPolynomial::from(p.numerator_ref().div_power_of_x(n))
            );
        }
    });
}
