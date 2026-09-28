// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{L2NormSquared, Polynomial};
use malachite_nz::test_util::generators::integer_polynomial_gen;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_gen, rational_polynomial_rational_pair_gen,
};
use malachite_q::test_util::rational_polynomial::arithmetic::l2_norm_squared::*;

#[test]
fn test_l2_norm_squared() {
    let test = |s, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let n = (&p).l2_norm_squared();
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
        assert_eq!(l2_norm_squared_naive(&p), n);
    };
    test("0", "0");
    test("-5", "25");
    test("x-3", "10");
    test("1/2*x^2-1/3", "13/36");
    // The sum of the squares of the numerator's coefficients shares a factor with the square of the
    // denominator.
    test("1/2*x+1/2", "1/2");
    test(
        "1/1000000000000000000000*x",
        "1/1000000000000000000000000000000000000000000",
    );
}

#[test]
fn l2_norm_squared_properties() {
    rational_polynomial_gen().test_properties(|p| {
        let n = (&p).l2_norm_squared();
        assert!(n.is_valid());
        assert_eq!(l2_norm_squared_naive(&p), n);
        assert!(n >= 0u32);
        assert_eq!((-&p).l2_norm_squared(), n);
        // It is zero exactly for the zero polynomial.
        assert_eq!(n == 0u32, p == RationalPolynomial::ZERO);
    });

    rational_polynomial_rational_pair_gen().test_properties(|(p, c)| {
        // Scaling the polynomial by c scales the square of the norm by c^2.
        let scaled = RationalPolynomial::from_coefficients_asc(
            p.to_coefficients_asc()
                .into_iter()
                .map(|x| x * &c)
                .collect::<Vec<_>>(),
        );
        assert_eq!(
            (&scaled).l2_norm_squared(),
            (&p).l2_norm_squared() * &c * &c
        );
    });

    integer_polynomial_gen().test_properties(|p| {
        // On polynomials with integer coefficients, this is the IntegerPolynomial operation.
        assert_eq!(
            (&RationalPolynomial::from(p.clone())).l2_norm_squared(),
            Rational::from((&p).l2_norm_squared())
        );
    });
}
