// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// The pinned outputs of each generator live beside it, one file per generator. These are the
// contracts the generators promise but that no list of values can establish: that everything a
// reduced generator produces really is reduced, that the degree-bounded ones stay in their bounds,
// and that striping reaches the coefficients.

use itertools::Itertools;
use malachite_base::num::arithmetic::traits::{Height, ModIsReduced, ModPowerOf2IsReduced};
use malachite_base::num::logic::traits::{BitIterable, LowMask};
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::u64_polynomial::U64Polynomial;
use malachite_base::u64_polynomial::random::*;

// The mean fraction of adjacent bit pairs that differ, across every coefficient. A striped
// generator's runs are long, so this is small; an unstriped one's bits are independent, so it is
// about a half.
fn transition_rate(ps: &[U64Polynomial]) -> f64 {
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
fn random_u64_polynomials_properties() {
    let ps = random_u64_polynomials(EXAMPLE_SEED, 3, 1)
        .take(2000)
        .collect_vec();
    assert!(ps.iter().all(U64Polynomial::is_valid));
    // The unbounded generator reaches the zero polynomial and a spread of degrees.
    assert!(ps.iter().any(|p| p.degree().is_none()));
    assert!(ps.iter().any(|p| p.degree() == Some(0)));
    assert!(ps.iter().any(|p| p.degree().is_some_and(|d| d > 5)));

    for d in 0..4u64 {
        let ps = random_u64_polynomials_with_degree(EXAMPLE_SEED, d)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(U64Polynomial::is_valid));
        assert!(ps.iter().all(|p| p.degree() == Some(d)));

        let ps = random_u64_polynomials_min_degree(EXAMPLE_SEED, d, d + 2, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(|p| p.degree().unwrap() >= d));
    }

    for (a, b) in [(0u64, 1u64), (1, 3), (2, 5)] {
        let ps = random_u64_polynomials_degree_range(EXAMPLE_SEED, a, b)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(|p| {
            let d = p.degree().unwrap();
            d >= a && d < b
        }));
        let ps = random_u64_polynomials_degree_inclusive_range(EXAMPLE_SEED, a, b)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(|p| {
            let d = p.degree().unwrap();
            d >= a && d <= b
        }));
    }
}
#[test]
fn striped_random_u64_polynomials_properties() {
    let striped = striped_random_u64_polynomials(EXAMPLE_SEED, 16, 1, 3, 1)
        .take(2000)
        .collect_vec();
    assert!(striped.iter().all(U64Polynomial::is_valid));
    assert!(striped.iter().any(|p| p.degree().is_none()));
    assert!(striped.iter().any(|p| p.degree().is_some_and(|d| d > 5)));

    for d in 0..4u64 {
        let ps = striped_random_u64_polynomials_with_degree(EXAMPLE_SEED, d, 16, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(U64Polynomial::is_valid));
        assert!(ps.iter().all(|p| p.degree() == Some(d)));

        let ps = striped_random_u64_polynomials_min_degree(EXAMPLE_SEED, d, 16, 1, d + 2, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(|p| p.degree().unwrap() >= d));
    }

    for (a, b) in [(0u64, 1u64), (1, 3), (2, 5)] {
        let ps = striped_random_u64_polynomials_degree_range(EXAMPLE_SEED, a, b, 16, 1)
            .take(300)
            .collect_vec();
        assert!(ps.iter().all(|p| {
            let d = p.degree().unwrap();
            d >= a && d < b
        }));
    }

    // The striping reaches the coefficients: their bits change far less often than unstriped ones.
    let plain = random_u64_polynomials(EXAMPLE_SEED, 3, 1)
        .take(2000)
        .collect_vec();
    let s = transition_rate(&striped);
    let u = transition_rate(&plain);
    assert!(s < 0.15, "striped transition rate {s}");
    assert!(u > 0.4, "unstriped transition rate {u}");
}
#[test]
fn random_u64_polynomials_reduced_mod_power_of_2_properties() {
    for pow in [1, 2, 3, 8, 63, 64] {
        let ps = random_u64_polynomials_reduced_mod_power_of_2(EXAMPLE_SEED, pow, 4, 1)
            .take(1000)
            .collect_vec();
        assert!(ps.iter().all(U64Polynomial::is_valid));
        // Everything generated is reduced, which is the whole point.
        assert!(ps.iter().all(|p| p.mod_power_of_2_is_reduced(pow)));
        // The restriction is on the coefficients, not the degree, so degrees still spread out, and
        // the zero polynomial is still reachable.
        assert!(ps.iter().any(|p| p.degree().is_none()));
        assert!(ps.iter().any(|p| p.degree().is_some_and(|d| d > 5)));
        // Every coefficient below the modulus is reachable, so the largest one shows up.
        if pow <= 3 {
            assert!(ps.iter().any(|p| p.to_height() == u64::low_mask(pow)));
        }

        let ps =
            striped_random_u64_polynomials_reduced_mod_power_of_2(EXAMPLE_SEED, pow, 8, 1, 4, 1)
                .take(1000)
                .collect_vec();
        assert!(ps.iter().all(U64Polynomial::is_valid));
        assert!(ps.iter().all(|p| p.mod_power_of_2_is_reduced(pow)));
        assert!(ps.iter().any(|p| p.degree().is_none()));
    }

    // Striping still reaches the coefficients: their bits change far less often than unstriped ones
    // of the same width.
    let striped =
        striped_random_u64_polynomials_reduced_mod_power_of_2(EXAMPLE_SEED, 64, 16, 1, 3, 1)
            .take(2000)
            .collect_vec();
    let plain = random_u64_polynomials_reduced_mod_power_of_2(EXAMPLE_SEED, 64, 3, 1)
        .take(2000)
        .collect_vec();
    let s = transition_rate(&striped);
    let u = transition_rate(&plain);
    assert!(s < 0.15, "striped transition rate {s}");
    assert!(u > 0.4, "unstriped transition rate {u}");
}
#[test]
fn random_u64_polynomials_reduced_mod_properties() {
    for m in [2, 3, 5, 10, 1000, u64::MAX, u64::MAX - 1] {
        let ps = random_u64_polynomials_reduced_mod(EXAMPLE_SEED, m, 4, 1)
            .take(1000)
            .collect_vec();
        assert!(ps.iter().all(U64Polynomial::is_valid));
        // Everything generated is reduced, which is the whole point.
        assert!(ps.iter().all(|p| p.mod_is_reduced(&m)));
        // The restriction is on the coefficients, not the degree.
        assert!(ps.iter().any(|p| p.degree().is_none()));
        assert!(ps.iter().any(|p| p.degree().is_some_and(|d| d > 5)));
        // Every coefficient below the modulus is reachable, so the largest one shows up.
        if m <= 5 {
            assert!(ps.iter().any(|p| p.to_height() == m - 1));
        }

        let ps = striped_random_u64_polynomials_reduced_mod(EXAMPLE_SEED, m, 8, 1, 4, 1)
            .take(1000)
            .collect_vec();
        assert!(ps.iter().all(U64Polynomial::is_valid));
        assert!(ps.iter().all(|p| p.mod_is_reduced(&m)));
        assert!(ps.iter().any(|p| p.degree().is_none()));
    }

    // Striping still reaches the coefficients, with a modulus that is not a bit-width boundary.
    let striped = striped_random_u64_polynomials_reduced_mod(EXAMPLE_SEED, u64::MAX, 16, 1, 3, 1)
        .take(2000)
        .collect_vec();
    let plain = random_u64_polynomials_reduced_mod(EXAMPLE_SEED, u64::MAX, 3, 1)
        .take(2000)
        .collect_vec();
    let s = transition_rate(&striped);
    let u = transition_rate(&plain);
    assert!(s < 0.15, "striped transition rate {s}");
    assert!(u > 0.4, "unstriped transition rate {u}");
}
