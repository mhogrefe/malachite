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
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_gen, natural_polynomial_pair_gen, natural_polynomial_triple_gen,
};
use malachite_nz::test_util::natural_polynomial::comparison::cmp::*;

#[test]
fn test_cmp() {
    // In ascending order: the zero polynomial is least, then the constants in their own order, then
    // everything of degree 1, and so on.
    test_cmp_helper::<NaturalPolynomial>(&[
        "0", "1", "2", "100", "x", "x+1", "x+2", "2*x", "2*x+1", "100*x", "x^2", "x^2+x",
        "x^2+x+1", "x^2+2*x", "2*x^2", "100*x^2", "x^3",
    ]);
}

#[test]
fn test_cmp_degree_dominates() {
    // A higher degree wins however small its coefficients and however large the other's.
    let test = |s, t| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
        assert!(p > q, "{s} should be greater than {t}");
        assert!(q < p, "{t} should be less than {s}");
    };
    test("x", "1000000000000000000000000000000");
    test("x^2", "1000000000000000000000000000000*x");
    test(
        "x^10",
        "1000000000000000000000000000000*x^9+1000000000000000000000000000000",
    );
}

#[test]
fn cmp_properties() {
    natural_polynomial_pair_gen().test_properties(|(p, q)| {
        let c = p.cmp(&q);
        // Comparison is antisymmetric, and agrees with `Eq`.
        assert_eq!(q.cmp(&p), c.reverse());
        assert_eq!(p == q, c == Equal);

        // What the ordering says is what the polynomials eventually do: evaluating both past the
        // largest root of their difference gives the same answer.
        assert_eq!(natural_polynomial_cmp_evaluated(&p, &q), c);

        // The degrees decide first, the zero polynomial being below everything.
        match p.degree().cmp(&q.degree()) {
            Equal => {}
            d => assert_eq!(c, d),
        }
    });

    natural_polynomial_gen().test_properties(|p| {
        // Reflexivity, and the zero polynomial is the least of them all.
        assert_eq!(p.cmp(&p), Equal);
        assert!(p >= NaturalPolynomial::ZERO);
    });

    natural_polynomial_triple_gen().test_properties(|(p, q, r)| {
        // Transitivity, in the two forms that make the order a total one.
        if p < q && q < r {
            assert!(p < r);
        } else if p > q && q > r {
            assert!(p > r);
        }
    });
}

#[test]
fn test_cmp_constants_agree_with_naturals() {
    // Restricted to the constant polynomials, this is the order on the `Natural`s.
    let test = |x: u32, y: u32| {
        assert_eq!(
            NaturalPolynomial::from(x).cmp(&NaturalPolynomial::from(y)),
            x.cmp(&y)
        );
    };
    test(0, 0);
    test(0, 1);
    test(1, 0);
    test(5, 7);
    test(123, 122);
}
