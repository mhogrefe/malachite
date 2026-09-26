// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{Content, Evaluate, Polynomial};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_integer_pair_gen,
};

#[test]
fn test_neg() {
    let test = |s, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();

        let q = -p.clone();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);

        let q = -&p;
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);

        let mut q = p;
        q.neg_assign();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
    };
    test("0", "0");
    test("1", "-1");
    test("-1", "1");
    test("x", "-x");
    test("-x", "x");
    test("x^2-3*x+2", "-x^2+3*x-2");
    test(
        "-5*x^3+x-1000000000000000000000",
        "5*x^3-x+1000000000000000000000",
    );
}

#[test]
fn neg_properties() {
    integer_polynomial_gen().test_properties(|p| {
        let q = -&p;
        assert!(q.is_valid());
        assert_eq!(-p.clone(), q);
        let mut r = p.clone();
        r.neg_assign();
        assert_eq!(r, q);

        // Negating twice gives the polynomial back, and only the zero polynomial is its own
        // negation.
        assert_eq!(-&q, p);
        assert_eq!(q == p, p == IntegerPolynomial::ZERO);
        assert_eq!(q.len(), p.len());
        for i in 0..p.len() {
            assert_eq!(*q.coefficient(i), -p.coefficient(i));
        }
        // The content is unchanged.
        assert_eq!((&q).content(), (&p).content());
    });

    integer_polynomial_integer_pair_gen().test_properties(|(p, x)| {
        // Negation commutes with evaluation.
        assert_eq!((&-&p).evaluate(&x), -(&p).evaluate(&x));
    });
}
