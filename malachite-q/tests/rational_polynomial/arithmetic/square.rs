// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::polynomial::Evaluate;
use malachite_nz::test_util::generators::integer_polynomial_gen;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_gen, rational_polynomial_rational_pair_gen,
};
use malachite_q::test_util::rational_polynomial::arithmetic::mul::mul_then_reduce;
use malachite_q::test_util::rational_polynomial::arithmetic::square::square_naive;

#[test]
fn test_square() {
    let test = |s, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let r = (&p).square();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(p.clone().square(), r);
        let mut s = p.clone();
        s.square_assign();
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(square_naive(&p), r);
    };
    test("0", "0");
    test("x-3", "x^2-6*x+9");
    test("1/2*x+1/3", "1/4*x^2+1/3*x+1/9");
    test("2/3*x^2-3/5", "4/9*x^4-4/5*x^2+9/25");
    // Coefficients and denominators of many limbs.
    test(
        "1/1000000000000000000000*x+1",
        "1/1000000000000000000000000000000000000000000*x^2+1/500000000000000000000*x+1",
    );
}

#[test]
fn square_properties() {
    rational_polynomial_gen().test_properties(|p| {
        let r = (&p).square();
        assert!(r.is_valid());
        // The forms agree.
        assert_eq!(p.clone().square(), r);
        let mut s = p.clone();
        s.square_assign();
        assert!(s.is_valid());
        assert_eq!(s, r);

        // The other algorithms agree, and it is the product with itself.
        assert_eq!(square_naive(&p), r);
        assert_eq!(mul_then_reduce(&p, &p), r);
        assert_eq!(&p * p.clone(), r);
        // The denominator is the square of the denominator.
        assert_eq!(*r.denominator_ref(), p.denominator_ref().square());
        // The square of the negation is the same.
        assert_eq!((-&p).square(), r);
    });

    rational_polynomial_rational_pair_gen().test_properties(|(p, x)| {
        // Evaluation commutes with squaring.
        assert_eq!((&p).square().evaluate(&x), (&p).evaluate(&x).square());
    });

    integer_polynomial_gen().test_properties(|p| {
        // On polynomials with integer coefficients, this is the `IntegerPolynomial` operation.
        assert_eq!(
            RationalPolynomial::from(p.clone()).square(),
            RationalPolynomial::from(p.square())
        );
    });
}
