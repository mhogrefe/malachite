// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::*;
use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::common::test_cmp_helper;
use malachite_base::test_util::generators::{
    unsigned_polynomial_gen, unsigned_polynomial_pair_gen, unsigned_polynomial_triple_gen,
};
use malachite_base::test_util::unsigned_polynomial::comparison::cmp::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_cmp() {
    // In ascending order: the zero polynomial is least, then the constants in their own order, then
    // everything of degree 1, and so on.
    test_cmp_helper::<UnsignedPolynomial<u64>>(&[
        "0", "1", "2", "100", "x", "x+1", "x+2", "2*x", "2*x+1", "100*x", "x^2", "x^2+x",
        "x^2+x+1", "x^2+2*x", "2*x^2", "100*x^2", "x^3",
    ]);
}

#[test]
fn test_cmp_degree_dominates() {
    // A higher degree wins however small its coefficients and however large the other's, up to the
    // largest coefficient a `UnsignedPolynomial` can hold.
    let test = |s, t| {
        let p = UnsignedPolynomial::<u64>::from_str(s).unwrap();
        let q = UnsignedPolynomial::<u64>::from_str(t).unwrap();
        assert!(p > q, "{s} should be greater than {t}");
        assert!(q < p, "{t} should be less than {s}");
    };
    test("x", "18446744073709551615");
    test("x^2", "18446744073709551615*x");
    test("x^10", "18446744073709551615*x^9+18446744073709551615");
}

#[test]
fn cmp_properties() {
    unsigned_polynomial_pair_gen().test_properties(|(p, q)| {
        let c = p.cmp(&q);
        // Comparison is antisymmetric, and agrees with `Eq`.
        assert_eq!(q.cmp(&p), c.reverse());
        assert_eq!(p == q, c == Equal);

        // The degrees decide first, the zero polynomial being below everything.
        match p.degree().cmp(&q.degree()) {
            Equal => {}
            d => assert_eq!(c, d),
        }

        // What the ordering says is what the polynomials eventually do, wherever the values are
        // small enough to evaluate at all.
        if let Some(evaluated) = unsigned_polynomial_cmp_evaluated(&p, &q) {
            assert_eq!(evaluated, c);
        }
    });

    unsigned_polynomial_gen().test_properties(|p| {
        // Reflexivity, and the zero polynomial is the least of them all.
        assert_eq!(p.cmp(&p), Equal);
        assert!(p >= UnsignedPolynomial::<u64>::ZERO);
    });

    unsigned_polynomial_triple_gen().test_properties(|(p, q, r)| {
        // Transitivity, in the two forms that make the order a total one.
        if p < q && q < r {
            assert!(p < r);
        } else if p > q && q > r {
            assert!(p > r);
        }
    });
}

#[test]
fn test_cmp_constants_agree_with_u64s() {
    // Restricted to the constant polynomials, this is the order on the `u64`s.
    let test = |x: u64, y: u64| {
        assert_eq!(
            UnsignedPolynomial::<u64>::from(x).cmp(&UnsignedPolynomial::<u64>::from(y)),
            x.cmp(&y)
        );
    };
    test(0, 0);
    test(0, 1);
    test(1, 0);
    test(5, 7);
    test(123, 122);
    test(u64::MAX, u64::MAX - 1);
}
