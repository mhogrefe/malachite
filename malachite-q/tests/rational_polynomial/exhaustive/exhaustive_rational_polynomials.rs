// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::iterators::prefix_to_string;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_q::rational::exhaustive::{exhaustive_nonzero_rationals, exhaustive_rationals};
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::rational_polynomial::exhaustive::*;
use std::collections::HashSet;

fn strings<I: Iterator<Item = RationalPolynomial>>(xs: I, n: usize) -> String {
    prefix_to_string(xs, n)
}

#[test]
fn test_exhaustive_rational_polynomials() {
    assert_eq!(
        strings(exhaustive_rational_polynomials(), 20),
        "[0, 1, -1, x, 1/2, -x, -1/2, x+1, x^2, x^3, -x^2, -x^3, x^2+x, x^3+x^2, -x^2+x, -x^3+x^2, \
        2, -x+1, -2, 1/2*x, ...]"
    );
    assert_eq!(
        strings(
            exhaustive_rational_polynomials_from_iterators(
                exhaustive_rationals(),
                exhaustive_nonzero_rationals()
            ),
            20
        ),
        strings(exhaustive_rational_polynomials(), 20)
    );
    assert_eq!(
        strings(
            exhaustive_rational_polynomials_from_iterators(
                exhaustive_nonzero_rationals(),
                exhaustive_nonzero_rationals()
            ),
            10
        ),
        "[0, 1, -1, x+1, 1/2, -x+1, -1/2, x-1, x^2+x+1, x^3+x^2+x+1, ...]"
    );
}

#[test]
fn test_exhaustive_rational_polynomials_with_degree() {
    assert_eq!(
        strings(exhaustive_rational_polynomials_with_degree(0), 10),
        "[1, -1, 1/2, -1/2, 2, -2, 1/3, -1/3, 3/2, -3/2, ...]"
    );
    assert_eq!(
        strings(exhaustive_rational_polynomials_with_degree(1), 10),
        "[x, -x, x+1, -x+1, 1/2*x, -1/2*x, 1/2*x+1, -1/2*x+1, x-1, -x-1, ...]"
    );
    assert_eq!(
        strings(exhaustive_rational_polynomials_with_degree(2), 10),
        "[x^2, -x^2, x^2+x, -x^2+x, x^2+1, -x^2+1, x^2+x+1, -x^2+x+1, 1/2*x^2, -1/2*x^2, ...]"
    );
}

#[test]
fn test_exhaustive_rational_polynomials_degree_bounds() {
    assert_eq!(
        strings(exhaustive_rational_polynomials_min_degree(0), 10),
        "[1, x, -1, -x, 1/2, x+1, -1/2, -x+1, x^2, x^3, ...]"
    );
    assert_eq!(
        strings(exhaustive_rational_polynomials_min_degree(2), 10),
        "[x^2, x^3, -x^2, -x^3, x^2+x, x^3+x^2, -x^2+x, -x^3+x^2, x^4, x^5, ...]"
    );
    assert_eq!(
        strings(exhaustive_rational_polynomials_degree_range(1, 3), 10),
        "[x, x^2, -x, -x^2, x+1, x^2+x, -x+1, -x^2+x, 1/2*x, x^2+1, ...]"
    );
    assert_eq!(
        strings(
            exhaustive_rational_polynomials_degree_inclusive_range(1, 2),
            10
        ),
        strings(exhaustive_rational_polynomials_degree_range(1, 3), 10)
    );
    // An empty range gives nothing.
    assert_eq!(
        strings(exhaustive_rational_polynomials_degree_range(3, 3), 10),
        "[]"
    );
    assert_eq!(
        strings(exhaustive_rational_polynomials_degree_range(3, 2), 10),
        "[]"
    );
    assert_eq!(
        strings(
            exhaustive_rational_polynomials_degree_inclusive_range(3, 2),
            10
        ),
        "[]"
    );
}

#[test]
fn exhaustive_rational_polynomials_properties() {
    // Everything generated is valid, and nothing is generated twice.
    let ps = exhaustive_rational_polynomials().take(500).collect_vec();
    assert!(ps.iter().all(RationalPolynomial::is_valid));
    assert_eq!(ps.iter().collect::<HashSet<_>>().len(), ps.len());
    // The zero polynomial comes first, and only once.
    assert_eq!(ps[0], RationalPolynomial::ZERO);
    assert_eq!(ps.iter().filter(|p| p.degree().is_none()).count(), 1);

    for d in 0..4 {
        let ps = exhaustive_rational_polynomials_with_degree(d)
            .take(200)
            .collect_vec();
        assert!(ps.iter().all(RationalPolynomial::is_valid));
        assert!(ps.iter().all(|p| p.degree() == Some(d)));
        assert_eq!(ps.iter().collect::<HashSet<_>>().len(), ps.len());

        let ps = exhaustive_rational_polynomials_min_degree(d)
            .take(200)
            .collect_vec();
        assert!(ps.iter().all(|p| p.degree().unwrap() >= d));
        assert_eq!(ps.iter().collect::<HashSet<_>>().len(), ps.len());
    }

    for (a, b) in [(0u64, 1u64), (1, 3), (2, 5), (0, 4)] {
        let ps = exhaustive_rational_polynomials_degree_range(a, b)
            .take(200)
            .collect_vec();
        assert!(ps.iter().all(|p| {
            let d = p.degree().unwrap();
            d >= a && d < b
        }));
        assert_eq!(ps.iter().collect::<HashSet<_>>().len(), ps.len());

        let ps = exhaustive_rational_polynomials_degree_inclusive_range(a, b)
            .take(200)
            .collect_vec();
        assert!(ps.iter().all(|p| {
            let d = p.degree().unwrap();
            d >= a && d <= b
        }));
    }

    // A fixed degree is the inclusive range of that degree with itself.
    assert_eq!(
        exhaustive_rational_polynomials_with_degree(2)
            .take(50)
            .collect_vec(),
        exhaustive_rational_polynomials_degree_inclusive_range(2, 2)
            .take(50)
            .collect_vec()
    );
}
