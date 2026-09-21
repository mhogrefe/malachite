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
use malachite_nz::integer::random::{random_integers, random_nonzero_integers};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::random::*;

#[test]
fn test_random_integer_polynomials() {
    assert_eq!(
        prefix_to_string(random_integer_polynomials(EXAMPLE_SEED, 4, 1, 1, 1), 5),
        "[14, -2*x-497, 2*x-1, 122*x+1, 1, ...]"
    );
    // The convenience function is the general one with the usual iterators.
    assert_eq!(
        prefix_to_string(
            random_integer_polynomials_from_iterators(
                EXAMPLE_SEED,
                &|seed| random_integers(seed, 4, 1),
                &|seed| random_nonzero_integers(seed, 4, 1),
                1,
                1,
            ),
            5
        ),
        prefix_to_string(random_integer_polynomials(EXAMPLE_SEED, 4, 1, 1, 1), 5)
    );
}

#[test]
fn test_random_integer_polynomials_degree_bounds() {
    assert_eq!(
        prefix_to_string(
            random_integer_polynomials_with_degree(EXAMPLE_SEED, 2, 4, 1),
            5
        ),
        "[14*x^2-x-497, -2*x^2+19*x+1, 2*x^2+799*x+799, 122*x^2+66*x-1, x^2+334*x-10721, ...]"
    );
    assert_eq!(
        prefix_to_string(
            random_integer_polynomials_min_degree(EXAMPLE_SEED, 1, 4, 1, 3, 1),
            5
        ),
        "[14*x^2-x-497, -2*x^3+799*x^2+19*x+1, 2*x^3+66*x^2-x+799, \
        122*x^3+59*x^2+334*x-10721, x^2-5*x-119, ...]"
    );
    assert_eq!(
        prefix_to_string(
            random_integer_polynomials_degree_range(EXAMPLE_SEED, 1, 3, 4, 1),
            5
        ),
        "[14*x^2-x-497, -2*x+1, 2*x^2+799*x+19, 122*x^2-x+799, x+66, ...]"
    );
    assert_eq!(
        prefix_to_string(
            random_integer_polynomials_degree_inclusive_range(EXAMPLE_SEED, 1, 2, 4, 1),
            5
        ),
        prefix_to_string(
            random_integer_polynomials_degree_range(EXAMPLE_SEED, 1, 3, 4, 1),
            5
        )
    );
}

#[test]
fn test_striped_random_integer_polynomials() {
    assert_eq!(
        prefix_to_string(
            striped_random_integer_polynomials(EXAMPLE_SEED, 16, 1, 4, 1, 1, 1),
            5
        ),
        "[15, -2*x-496, 2*x-1, 65*x+1, 1, ...]"
    );
    assert_eq!(
        prefix_to_string(
            striped_random_integer_polynomials_with_degree(EXAMPLE_SEED, 2, 16, 1, 4, 1),
            5
        ),
        "[15*x^2-x-496, -2*x^2+16*x+1, 2*x^2+543*x+512, 65*x^2+127*x-1, x^2+383*x-10239, ...]"
    );
    assert_eq!(
        prefix_to_string(
            striped_random_integer_polynomials_min_degree(EXAMPLE_SEED, 1, 16, 1, 4, 1, 3, 1),
            5
        ),
        "[15*x^2-x-496, -2*x^3+512*x^2+16*x+1, 2*x^3+127*x^2-x+543, \
        65*x^3+36*x^2+383*x-10239, x^2-4*x-127, ...]"
    );
    assert_eq!(
        prefix_to_string(
            striped_random_integer_polynomials_degree_range(EXAMPLE_SEED, 1, 3, 16, 1, 4, 1),
            5
        ),
        "[15*x^2-x-496, -2*x+1, 2*x^2+512*x+16, 65*x^2-x+543, x+127, ...]"
    );
    assert_eq!(
        prefix_to_string(
            striped_random_integer_polynomials_degree_inclusive_range(
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
            striped_random_integer_polynomials_degree_range(EXAMPLE_SEED, 1, 3, 16, 1, 4, 1),
            5
        )
    );
}

#[test]
#[should_panic]
fn random_integer_polynomials_degree_range_fail() {
    random_integer_polynomials_degree_range(EXAMPLE_SEED, 3, 3, 4, 1);
}

#[test]
#[should_panic]
fn random_integer_polynomials_degree_inclusive_range_fail() {
    random_integer_polynomials_degree_inclusive_range(EXAMPLE_SEED, 3, 2, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_integer_polynomials_degree_range_fail() {
    striped_random_integer_polynomials_degree_range(EXAMPLE_SEED, 3, 2, 16, 1, 4, 1);
}

// The mean fraction of adjacent bit pairs that differ, across every coefficient. A striped
// generator's runs are long, so this is small; an unstriped one's bits are independent, so it is
// about a half.
fn transition_rate(ps: &[IntegerPolynomial]) -> f64 {
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
fn random_integer_polynomials_properties() {
    let ps = random_integer_polynomials(EXAMPLE_SEED, 32, 1, 3, 1)
        .take(2000)
        .collect_vec();
    assert!(ps.iter().all(IntegerPolynomial::is_valid));
    // The unbounded generator reaches the zero polynomial and a spread of degrees.
    assert!(ps.iter().any(|p| p.degree().is_none()));
    assert!(ps.iter().any(|p| p.degree() == Some(0)));
    assert!(ps.iter().any(|p| p.degree().is_some_and(|d| d > 5)));

    for d in 0..4u64 {
        let ps = random_integer_polynomials_with_degree(EXAMPLE_SEED, d, 32, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(IntegerPolynomial::is_valid));
        assert!(ps.iter().all(|p| p.degree() == Some(d)));

        let ps = random_integer_polynomials_min_degree(EXAMPLE_SEED, d, 32, 1, d + 2, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(|p| p.degree().unwrap() >= d));
    }

    for (a, b) in [(0u64, 1u64), (1, 3), (2, 5)] {
        let ps = random_integer_polynomials_degree_range(EXAMPLE_SEED, a, b, 32, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(|p| {
            let d = p.degree().unwrap();
            d >= a && d < b
        }));
        let ps = random_integer_polynomials_degree_inclusive_range(EXAMPLE_SEED, a, b, 32, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(|p| {
            let d = p.degree().unwrap();
            d >= a && d <= b
        }));
    }
}

#[test]
fn striped_random_integer_polynomials_properties() {
    let striped = striped_random_integer_polynomials(EXAMPLE_SEED, 16, 1, 32, 1, 3, 1)
        .take(2000)
        .collect_vec();
    assert!(striped.iter().all(IntegerPolynomial::is_valid));
    assert!(striped.iter().any(|p| p.degree().is_none()));
    assert!(striped.iter().any(|p| p.degree().is_some_and(|d| d > 5)));

    for d in 0..4u64 {
        let ps = striped_random_integer_polynomials_with_degree(EXAMPLE_SEED, d, 16, 1, 32, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(IntegerPolynomial::is_valid));
        assert!(ps.iter().all(|p| p.degree() == Some(d)));

        let ps =
            striped_random_integer_polynomials_min_degree(EXAMPLE_SEED, d, 16, 1, 32, 1, d + 2, 1)
                .take(300)
                .collect_vec();
        assert!(ps.iter().all(|p| p.degree().unwrap() >= d));
    }

    for (a, b) in [(0u64, 1u64), (1, 3), (2, 5)] {
        let ps = striped_random_integer_polynomials_degree_range(EXAMPLE_SEED, a, b, 16, 1, 32, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(|p| {
            let d = p.degree().unwrap();
            d >= a && d < b
        }));
    }

    // The striping reaches the coefficients: their bits change far less often than unstriped ones.
    let plain = random_integer_polynomials(EXAMPLE_SEED, 32, 1, 3, 1)
        .take(2000)
        .collect_vec();
    let s = transition_rate(&striped);
    let u = transition_rate(&plain);
    assert!(s < 0.15, "striped transition rate {s}");
    assert!(u > 0.4, "unstriped transition rate {u}");
}
