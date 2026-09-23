// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// The pinned outputs of each generator live beside it, one file per generator. These are the
// contracts the generators promise but that no list of values can establish: that the
// degree-bounded generators stay in their bounds, and that striping reaches the coefficients.

use itertools::Itertools;
use malachite_base::num::logic::traits::BitIterable;
use malachite_base::polynomial::Polynomial;
use malachite_base::random::EXAMPLE_SEED;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::rational_polynomial::random::*;

// The mean fraction of adjacent bit pairs that differ, across every coefficient. A striped
// generator's runs are long, so this is small; an unstriped one's bits are independent, so it is
// about a half.
fn transition_rate(ps: &[RationalPolynomial]) -> f64 {
    let (mut changes, mut bits) = (0u64, 0u64);
    for p in ps {
        for c in p
            .to_coefficients_asc()
            .iter()
            .flat_map(|q| [q.to_numerator(), q.to_denominator()])
        {
            let mut prev = None;
            for b in c.bits() {
                if prev.is_some_and(|q| q != b) {
                    changes += 1;
                }
                prev = Some(b);
                bits += 1;
            }
        }
    }
    changes as f64 / bits as f64
}

#[test]
fn random_rational_polynomials_properties() {
    let ps = random_rational_polynomials(EXAMPLE_SEED, 32, 1, 3, 1)
        .take(2000)
        .collect_vec();
    assert!(ps.iter().all(RationalPolynomial::is_valid));
    // The unbounded generator reaches the zero polynomial and a spread of degrees.
    assert!(ps.iter().any(|p| p.degree().is_none()));
    assert!(ps.iter().any(|p| p.degree() == Some(0)));
    assert!(ps.iter().any(|p| p.degree().is_some_and(|d| d > 5)));

    for d in 0..4u64 {
        let ps = random_rational_polynomials_with_degree(EXAMPLE_SEED, d, 32, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(RationalPolynomial::is_valid));
        assert!(ps.iter().all(|p| p.degree() == Some(d)));

        let ps = random_rational_polynomials_min_degree(EXAMPLE_SEED, d, 32, 1, d + 2, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(|p| p.degree().unwrap() >= d));
    }

    for (a, b) in [(0u64, 1u64), (1, 3), (2, 5)] {
        let ps = random_rational_polynomials_degree_range(EXAMPLE_SEED, a, b, 32, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(|p| {
            let d = p.degree().unwrap();
            d >= a && d < b
        }));
        let ps = random_rational_polynomials_degree_inclusive_range(EXAMPLE_SEED, a, b, 32, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(|p| {
            let d = p.degree().unwrap();
            d >= a && d <= b
        }));
    }
}

#[test]
fn striped_random_rational_polynomials_properties() {
    let striped = striped_random_rational_polynomials(EXAMPLE_SEED, 16, 1, 32, 1, 3, 1)
        .take(2000)
        .collect_vec();
    assert!(striped.iter().all(RationalPolynomial::is_valid));
    assert!(striped.iter().any(|p| p.degree().is_none()));
    assert!(striped.iter().any(|p| p.degree().is_some_and(|d| d > 5)));

    for d in 0..4u64 {
        let ps = striped_random_rational_polynomials_with_degree(EXAMPLE_SEED, d, 16, 1, 32, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(RationalPolynomial::is_valid));
        assert!(ps.iter().all(|p| p.degree() == Some(d)));

        let ps =
            striped_random_rational_polynomials_min_degree(EXAMPLE_SEED, d, 16, 1, 32, 1, d + 2, 1)
                .take(300)
                .collect_vec();
        assert!(ps.iter().all(|p| p.degree().unwrap() >= d));
    }

    for (a, b) in [(0u64, 1u64), (1, 3), (2, 5)] {
        let ps = striped_random_rational_polynomials_degree_range(EXAMPLE_SEED, a, b, 16, 1, 32, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(|p| {
            let d = p.degree().unwrap();
            d >= a && d < b
        }));
    }

    // The striping reaches the coefficients: their bits change far less often than unstriped ones.
    let plain = random_rational_polynomials(EXAMPLE_SEED, 32, 1, 3, 1)
        .take(2000)
        .collect_vec();
    let s = transition_rate(&striped);
    let u = transition_rate(&plain);
    assert!(s < 0.15, "striped transition rate {s}");
    assert!(u > 0.4, "unstriped transition rate {u}");
}
