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
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::{
    integer_polynomial_pair_gen, integer_polynomial_triple_gen,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::sub::sub_naive;

#[test]
fn test_sub() {
    let test = |s, t, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = IntegerPolynomial::from_str(t).unwrap();

        let r = p.clone() - q.clone();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);

        let r = p.clone() - &q;
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);

        let r = &p - q.clone();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);

        let r = &p - &q;
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);

        let mut r = p.clone();
        r -= q.clone();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);

        let mut r = p;
        r -= &q;
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "x-1", "-x+1");
    test("x^2+x", "x^2-1", "x+1");
    test("3*x+1", "x^2", "-x^2+3*x+1");
    test("x^2", "3*x+1", "x^2-3*x-1");
    test("x+1", "x+1", "0");
    test(
        "-1000000000000000000000*x+1",
        "1000000000000000000000*x+2",
        "-2000000000000000000000*x-1",
    );
}

#[test]
fn sub_properties() {
    integer_polynomial_pair_gen().test_properties(|(p, q)| {
        let r = &p - &q;
        assert!(r.is_valid());
        assert_eq!(p.clone() - q.clone(), r);
        assert_eq!(p.clone() - &q, r);
        assert_eq!(&p - q.clone(), r);
        let mut s = p.clone();
        s -= q.clone();
        assert_eq!(s, r);
        let mut s = p.clone();
        s -= &q;
        assert_eq!(s, r);

        assert_eq!(sub_naive(&p, &q), r);
        // The degree is at most the larger of the two degrees.
        assert!(r.len() <= p.len().max(q.len()));
        // Evaluation commutes with the operation.
        for x in [0i32, 1, 2, 3, 10] {
            let x = Integer::from(x);
            assert_eq!((&r).evaluate(&x), (&p).evaluate(&x) - (&q).evaluate(&x));
        }
        // p - q = p + (-q) = -(q - p), (p - q) + q = p, and p - p = 0.
        assert_eq!(&p + -&q, r);
        assert_eq!(-(&q - &p), r);
        assert_eq!(&r + &q, p);
        assert_eq!(&p - &p, IntegerPolynomial::ZERO);
        assert_eq!(&p - IntegerPolynomial::ZERO, p);
    });

    integer_polynomial_triple_gen().test_properties(|(p, q, r)| {
        // (p - q) - r = p - (q + r).
        assert_eq!((&p - &q) - &r, &p - (&q + &r));
    });
}
