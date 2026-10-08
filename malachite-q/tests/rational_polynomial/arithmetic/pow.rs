// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Content, Parity, Pow, PowAssign, Square};
use malachite_base::num::basic::traits::OneHalf;
use malachite_base::polynomial::{Evaluate, Polynomial};
use malachite_nz::test_util::generators::integer_polynomial_unsigned_pair_gen_var_5;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_gen, rational_polynomial_unsigned_pair_gen_var_1,
};
use malachite_q::test_util::rational_polynomial::arithmetic::pow::pow_naive;

#[test]
fn test_pow() {
    let test = |s, e: u64, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let power = p.clone().pow(e);
        assert!(power.is_valid());
        assert_eq!(power.to_string(), out);
        assert_eq!((&p).pow(e).to_string(), out);
        let mut q = p.clone();
        q.pow_assign(e);
        assert_eq!(q.to_string(), out);
        assert_eq!(pow_naive(&p, e).to_string(), out);
    };
    // - e == 0
    test("0", 0, "1");
    test("1/2*x+1/3", 0, "1");
    // - the polynomial is zero
    test("0", 5, "0");
    // - the polynomial is a constant
    test("-2/3", 3, "-8/27");
    // - the polynomial is a monomial
    test("1/2*x^2", 4, "1/16*x^8");
    // - e == 1
    test("1/2*x+1/3", 1, "1/2*x+1/3");
    // - e == 2
    test("1/2*x+1/3", 2, "1/4*x^2+1/3*x+1/9");
    // - e >= 3
    test("1/2*x+1/3", 3, "1/8*x^3+1/4*x^2+1/6*x+1/27");
    test("1/2*x-1", 4, "1/16*x^4-1/2*x^3+3/2*x^2-2*x+1");
    test(
        "1/3*x^2+1/2*x+1",
        5,
        "1/243*x^10+5/162*x^9+25/162*x^8+55/108*x^7+565/432*x^6+81/32*x^5+565/144*x^4+55/12*x^3+\
        25/6*x^2+5/2*x+1",
    );
    test("3/2*x^3-x", 3, "27/8*x^9-27/4*x^7+9/2*x^5-x^3");
    // - an integral polynomial
    test("x+1", 5, "x^5+5*x^4+10*x^3+10*x^2+5*x+1");
    test(
        "-1/2*x+2/3",
        7,
        "-1/128*x^7+7/96*x^6-7/24*x^5+35/54*x^4-70/81*x^3+56/81*x^2-224/729*x+128/2187",
    );
}

#[test]
fn pow_properties() {
    rational_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, e)| {
        let power = p.clone().pow(e);
        assert!(power.is_valid());
        assert_eq!((&p).pow(e), power);
        let mut q = p.clone();
        q.pow_assign(e);
        assert_eq!(q, power);
        // The naive power is slow for large exponents; the integer kernels it would check are
        // cross-checked against it in malachite-nz.
        if e <= 6 {
            assert_eq!(pow_naive(&p, e), power);
        }
        if p != 0u32 {
            assert_eq!(power.degree(), p.degree().map(|d| d * e));
            assert_eq!(power.leading_coefficient(), p.leading_coefficient().pow(e));
            // Gauss's lemma, over the rationals
            assert_eq!((&power).content(), (&p).content().pow(e));
        }
        let neg_power = (-&p).pow(e);
        assert_eq!(if e.even() { neg_power } else { -neg_power }, power);
        for x in [Rational::ONE_HALF, Rational::from(-3)] {
            assert_eq!((&power).evaluate(&x), (&p).evaluate(&x).pow(e));
        }
        let half = e >> 1;
        assert_eq!((&p).pow(half) * (&p).pow(e - half), power);
    });

    rational_polynomial_gen().test_properties(|p| {
        assert_eq!((&p).pow(0), RationalPolynomial::one());
        assert_eq!((&p).pow(1), p);
        assert_eq!((&p).pow(2), (&p).square());
    });

    integer_polynomial_unsigned_pair_gen_var_5().test_properties(|(p, e)| {
        // agreement with the integer polynomial power
        assert_eq!(
            RationalPolynomial::from(p.clone()).pow(e),
            RationalPolynomial::from((&p).pow(e))
        );
    });
}
