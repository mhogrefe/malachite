// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::iterators::prefix_to_string;
use malachite_base::num::logic::traits::BitIterable;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural::random::{random_naturals, random_positive_naturals};
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::natural_polynomial::random::*;

#[test]
fn test_random_natural_polynomials() {
    assert_eq!(
        prefix_to_string(random_natural_polynomials(EXAMPLE_SEED, 4, 1, 1, 1), 5),
        "[14, x+3, 4*x, x+13235, 30, ...]"
    );
    // The convenience function is the general one with the usual iterators.
    assert_eq!(
        prefix_to_string(
            random_natural_polynomials_from_iterators(
                EXAMPLE_SEED,
                &|seed| random_naturals(seed, 4, 1),
                &|seed| random_positive_naturals(seed, 4, 1),
                1,
                1,
            ),
            5
        ),
        prefix_to_string(random_natural_polynomials(EXAMPLE_SEED, 4, 1, 1, 1), 5)
    );
}

#[test]
fn test_random_natural_polynomials_degree_bounds() {
    assert_eq!(
        prefix_to_string(
            random_natural_polynomials_with_degree(EXAMPLE_SEED, 2, 4, 1),
            5
        ),
        "[14*x^2+3, x^2+x+13235, 4*x^2+x+7, x^2+15*x, 30*x^2+2*x+13, ...]"
    );
    assert_eq!(
        prefix_to_string(
            random_natural_polynomials_min_degree(EXAMPLE_SEED, 1, 4, 1, 3, 1),
            5
        ),
        "[14*x^2+3, x^3+7*x^2+x+13235, 4*x^3+15*x^2+1, x^3+x^2+2*x+13, 30*x^2+x, ...]"
    );
    assert_eq!(
        prefix_to_string(
            random_natural_polynomials_degree_range(EXAMPLE_SEED, 1, 3, 4, 1),
            5
        ),
        "[14*x^2+3, x+13235, 4*x^2+7*x+1, x^2+1, 30*x+15, ...]"
    );
    assert_eq!(
        prefix_to_string(
            random_natural_polynomials_degree_inclusive_range(EXAMPLE_SEED, 1, 2, 4, 1),
            5
        ),
        prefix_to_string(
            random_natural_polynomials_degree_range(EXAMPLE_SEED, 1, 3, 4, 1),
            5
        )
    );
}

#[test]
fn test_striped_random_natural_polynomials() {
    assert_eq!(
        prefix_to_string(
            striped_random_natural_polynomials(EXAMPLE_SEED, 16, 1, 4, 1, 1, 1),
            5
        ),
        "[15, x+2, 6*x, x+16376, 30, ...]"
    );
    assert_eq!(
        prefix_to_string(
            striped_random_natural_polynomials_with_degree(EXAMPLE_SEED, 2, 16, 1, 4, 1),
            5
        ),
        "[15*x^2+2, x^2+x+16376, 6*x^2+x+4, x^2+15*x, 30*x^2+3*x+15, ...]"
    );
    assert_eq!(
        prefix_to_string(
            striped_random_natural_polynomials_min_degree(EXAMPLE_SEED, 1, 16, 1, 4, 1, 3, 1),
            5
        ),
        "[15*x^2+2, x^3+4*x^2+x+16376, 6*x^3+15*x^2+1, x^3+x^2+3*x+15, 30*x^2+x, ...]"
    );
    assert_eq!(
        prefix_to_string(
            striped_random_natural_polynomials_degree_range(EXAMPLE_SEED, 1, 3, 16, 1, 4, 1),
            5
        ),
        "[15*x^2+2, x+16376, 6*x^2+4*x+1, x^2+1, 30*x+15, ...]"
    );
    assert_eq!(
        prefix_to_string(
            striped_random_natural_polynomials_degree_inclusive_range(
                EXAMPLE_SEED,
                1,
                2,
                16,
                1,
                4,
                1
            ),
            5
        ),
        prefix_to_string(
            striped_random_natural_polynomials_degree_range(EXAMPLE_SEED, 1, 3, 16, 1, 4, 1),
            5
        )
    );
}

#[test]
#[should_panic]
fn random_natural_polynomials_degree_range_fail() {
    random_natural_polynomials_degree_range(EXAMPLE_SEED, 3, 3, 4, 1);
}

#[test]
#[should_panic]
fn random_natural_polynomials_degree_inclusive_range_fail() {
    random_natural_polynomials_degree_inclusive_range(EXAMPLE_SEED, 3, 2, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_polynomials_degree_range_fail() {
    striped_random_natural_polynomials_degree_range(EXAMPLE_SEED, 3, 2, 16, 1, 4, 1);
}

// The mean fraction of adjacent bit pairs that differ, across every coefficient. A striped
// generator's runs are long, so this is small; an unstriped one's bits are independent, so it is
// about a half.
fn transition_rate(ps: &[NaturalPolynomial]) -> f64 {
    let (mut changes, mut bits) = (0u64, 0u64);
    for p in ps {
        for c in p.coefficients_asc() {
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
fn random_natural_polynomials_properties() {
    let ps = random_natural_polynomials(EXAMPLE_SEED, 32, 1, 3, 1)
        .take(2000)
        .collect_vec();
    assert!(ps.iter().all(NaturalPolynomial::is_valid));
    // The unbounded generator reaches the zero polynomial and a spread of degrees.
    assert!(ps.iter().any(|p| p.degree().is_none()));
    assert!(ps.iter().any(|p| p.degree() == Some(0)));
    assert!(ps.iter().any(|p| p.degree().is_some_and(|d| d > 5)));

    for d in 0..4u64 {
        let ps = random_natural_polynomials_with_degree(EXAMPLE_SEED, d, 32, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(NaturalPolynomial::is_valid));
        assert!(ps.iter().all(|p| p.degree() == Some(d)));

        let ps = random_natural_polynomials_min_degree(EXAMPLE_SEED, d, 32, 1, d + 2, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(|p| p.degree().unwrap() >= d));
    }

    for (a, b) in [(0u64, 1u64), (1, 3), (2, 5)] {
        let ps = random_natural_polynomials_degree_range(EXAMPLE_SEED, a, b, 32, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(|p| {
            let d = p.degree().unwrap();
            d >= a && d < b
        }));
        let ps = random_natural_polynomials_degree_inclusive_range(EXAMPLE_SEED, a, b, 32, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(|p| {
            let d = p.degree().unwrap();
            d >= a && d <= b
        }));
    }
}

#[test]
fn striped_random_natural_polynomials_properties() {
    let striped = striped_random_natural_polynomials(EXAMPLE_SEED, 16, 1, 32, 1, 3, 1)
        .take(2000)
        .collect_vec();
    assert!(striped.iter().all(NaturalPolynomial::is_valid));
    assert!(striped.iter().any(|p| p.degree().is_none()));
    assert!(striped.iter().any(|p| p.degree().is_some_and(|d| d > 5)));

    for d in 0..4u64 {
        let ps = striped_random_natural_polynomials_with_degree(EXAMPLE_SEED, d, 16, 1, 32, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(NaturalPolynomial::is_valid));
        assert!(ps.iter().all(|p| p.degree() == Some(d)));

        let ps =
            striped_random_natural_polynomials_min_degree(EXAMPLE_SEED, d, 16, 1, 32, 1, d + 2, 1)
                .take(300)
                .collect_vec();
        assert!(ps.iter().all(|p| p.degree().unwrap() >= d));
    }

    for (a, b) in [(0u64, 1u64), (1, 3), (2, 5)] {
        let ps = striped_random_natural_polynomials_degree_range(EXAMPLE_SEED, a, b, 16, 1, 32, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(|p| {
            let d = p.degree().unwrap();
            d >= a && d < b
        }));
    }

    // The striping reaches the coefficients: their bits change far less often than unstriped ones.
    let plain = random_natural_polynomials(EXAMPLE_SEED, 32, 1, 3, 1)
        .take(2000)
        .collect_vec();
    let s = transition_rate(&striped);
    let u = transition_rate(&plain);
    assert!(s < 0.15, "striped transition rate {s}");
    assert!(u > 0.4, "unstriped transition rate {u}");
}
