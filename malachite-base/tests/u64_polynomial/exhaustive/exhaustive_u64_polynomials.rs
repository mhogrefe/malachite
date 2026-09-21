// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::iterators::prefix_to_string;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModPowerOf2IsReduced};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::exhaustive::{exhaustive_positive_primitive_ints, exhaustive_unsigneds};
use malachite_base::u64_polynomial::U64Polynomial;
use malachite_base::u64_polynomial::exhaustive::*;
use std::collections::HashSet;

fn strings<I: Iterator<Item = U64Polynomial>>(xs: I, n: usize) -> String {
    prefix_to_string(xs, n)
}

#[test]
fn test_exhaustive_u64_polynomials() {
    assert_eq!(
        strings(exhaustive_u64_polynomials(), 20),
        "[0, 1, 2, x, 3, 2*x, 4, x+1, x^2, x^3, 2*x^2, 2*x^3, x^2+x, x^3+x^2, 2*x^2+x, \
        2*x^3+x^2, 5, 2*x+1, 6, 3*x, ...]"
    );
    assert_eq!(
        strings(
            exhaustive_u64_polynomials_from_iterators(
                exhaustive_unsigneds::<u64>(),
                exhaustive_positive_primitive_ints::<u64>()
            ),
            20
        ),
        strings(exhaustive_u64_polynomials(), 20)
    );
    assert_eq!(
        strings(
            exhaustive_u64_polynomials_from_iterators(
                exhaustive_positive_primitive_ints::<u64>(),
                exhaustive_positive_primitive_ints::<u64>()
            ),
            10
        ),
        "[0, 1, 2, x+1, 3, 2*x+1, 4, x+2, x^2+x+1, x^3+x^2+x+1, ...]"
    );
}

#[test]
fn test_exhaustive_u64_polynomials_with_degree() {
    assert_eq!(
        strings(exhaustive_u64_polynomials_with_degree(0), 10),
        "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, ...]"
    );
    assert_eq!(
        strings(exhaustive_u64_polynomials_with_degree(1), 10),
        "[x, 2*x, x+1, 2*x+1, 3*x, 4*x, 3*x+1, 4*x+1, x+2, 2*x+2, ...]"
    );
    assert_eq!(
        strings(exhaustive_u64_polynomials_with_degree(2), 10),
        "[x^2, 2*x^2, x^2+x, 2*x^2+x, x^2+1, 2*x^2+1, x^2+x+1, 2*x^2+x+1, 3*x^2, 4*x^2, ...]"
    );
}

#[test]
fn test_exhaustive_u64_polynomials_degree_bounds() {
    assert_eq!(
        strings(exhaustive_u64_polynomials_min_degree(0), 10),
        "[1, x, 2, 2*x, 3, x+1, 4, 2*x+1, x^2, x^3, ...]"
    );
    assert_eq!(
        strings(exhaustive_u64_polynomials_min_degree(2), 10),
        "[x^2, x^3, 2*x^2, 2*x^3, x^2+x, x^3+x^2, 2*x^2+x, 2*x^3+x^2, x^4, x^5, ...]"
    );
    assert_eq!(
        strings(exhaustive_u64_polynomials_degree_range(1, 3), 10),
        "[x, x^2, 2*x, 2*x^2, x+1, x^2+x, 2*x+1, 2*x^2+x, 3*x, x^2+1, ...]"
    );
    assert_eq!(
        strings(exhaustive_u64_polynomials_degree_inclusive_range(1, 2), 10),
        strings(exhaustive_u64_polynomials_degree_range(1, 3), 10)
    );
    // An empty range gives nothing.
    assert_eq!(
        strings(exhaustive_u64_polynomials_degree_range(3, 3), 10),
        "[]"
    );
    assert_eq!(
        strings(exhaustive_u64_polynomials_degree_range(3, 2), 10),
        "[]"
    );
    assert_eq!(
        strings(exhaustive_u64_polynomials_degree_inclusive_range(3, 2), 10),
        "[]"
    );
}

#[test]
fn exhaustive_u64_polynomials_properties() {
    // Everything generated is valid, and nothing is generated twice.
    let ps = exhaustive_u64_polynomials().take(500).collect_vec();
    assert!(ps.iter().all(U64Polynomial::is_valid));
    assert_eq!(ps.iter().collect::<HashSet<_>>().len(), ps.len());
    // The zero polynomial comes first, and only once.
    assert_eq!(ps[0], U64Polynomial::ZERO);
    assert_eq!(ps.iter().filter(|p| p.degree().is_none()).count(), 1);

    for d in 0..4 {
        let ps = exhaustive_u64_polynomials_with_degree(d)
            .take(200)
            .collect_vec();
        assert!(ps.iter().all(U64Polynomial::is_valid));
        assert!(ps.iter().all(|p| p.degree() == Some(d)));
        assert_eq!(ps.iter().collect::<HashSet<_>>().len(), ps.len());

        let ps = exhaustive_u64_polynomials_min_degree(d)
            .take(200)
            .collect_vec();
        assert!(ps.iter().all(|p| p.degree().unwrap() >= d));
        assert_eq!(ps.iter().collect::<HashSet<_>>().len(), ps.len());
    }

    for (a, b) in [(0u64, 1u64), (1, 3), (2, 5), (0, 4)] {
        let ps = exhaustive_u64_polynomials_degree_range(a, b)
            .take(200)
            .collect_vec();
        assert!(ps.iter().all(|p| {
            let d = p.degree().unwrap();
            d >= a && d < b
        }));
        assert_eq!(ps.iter().collect::<HashSet<_>>().len(), ps.len());

        let ps = exhaustive_u64_polynomials_degree_inclusive_range(a, b)
            .take(200)
            .collect_vec();
        assert!(ps.iter().all(|p| {
            let d = p.degree().unwrap();
            d >= a && d <= b
        }));
    }

    // A fixed degree is the inclusive range of that degree with itself.
    assert_eq!(
        exhaustive_u64_polynomials_with_degree(2)
            .take(50)
            .collect_vec(),
        exhaustive_u64_polynomials_degree_inclusive_range(2, 2)
            .take(50)
            .collect_vec()
    );
}

#[test]
fn test_exhaustive_u64_polynomials_reduced_mod_power_of_2() {
    // Modulo 2 the coefficients are all 0 or 1, so these are the polynomials over GF(2).
    assert_eq!(
        strings(exhaustive_u64_polynomials_reduced_mod_power_of_2(1), 10),
        "[0, 1, x, x^2, x+1, x^2+x, x^2+1, x^3, x^4, x^5, ...]"
    );
    assert_eq!(
        strings(exhaustive_u64_polynomials_reduced_mod_power_of_2(2), 10),
        "[0, 1, 2, x, 3, 2*x, x+1, x^2, x^3, x^4, ...]"
    );
    // Modulo 2^64 nothing is excluded, so this is every polynomial.
    assert_eq!(
        strings(exhaustive_u64_polynomials_reduced_mod_power_of_2(64), 20),
        strings(exhaustive_u64_polynomials(), 20)
    );
}

#[test]
#[should_panic]
fn exhaustive_u64_polynomials_reduced_mod_power_of_2_fail_zero() {
    exhaustive_u64_polynomials_reduced_mod_power_of_2(0);
}

#[test]
#[should_panic]
fn exhaustive_u64_polynomials_reduced_mod_power_of_2_fail_too_wide() {
    exhaustive_u64_polynomials_reduced_mod_power_of_2(65);
}

#[test]
fn exhaustive_u64_polynomials_reduced_mod_power_of_2_properties() {
    for pow in [1, 2, 3, 8, 63, 64] {
        let ps = exhaustive_u64_polynomials_reduced_mod_power_of_2(pow)
            .take(200)
            .collect_vec();
        assert!(ps.iter().all(U64Polynomial::is_valid));
        // Everything generated is reduced, which is the whole point.
        assert!(ps.iter().all(|p| p.mod_power_of_2_is_reduced(pow)));
        // Nothing is generated twice, and the zero polynomial comes first.
        assert_eq!(ps.iter().collect::<HashSet<_>>().len(), ps.len());
        assert_eq!(ps[0], U64Polynomial::ZERO);
        // Restricting further can only remove polynomials, never add them.
        if pow > 1 {
            let narrower: HashSet<_> = exhaustive_u64_polynomials_reduced_mod_power_of_2(pow - 1)
                .take(200)
                .collect();
            let wider: HashSet<_> = exhaustive_u64_polynomials_reduced_mod_power_of_2(pow)
                .take(20000)
                .collect();
            assert!(narrower.is_subset(&wider));
        }
    }
}

#[test]
fn test_exhaustive_u64_polynomials_reduced_mod() {
    // Modulo 2 this is the same set as modulo 2^1, the polynomials over GF(2).
    assert_eq!(
        strings(exhaustive_u64_polynomials_reduced_mod(2), 10),
        strings(exhaustive_u64_polynomials_reduced_mod_power_of_2(1), 10)
    );
    assert_eq!(
        strings(exhaustive_u64_polynomials_reduced_mod(3), 10),
        "[0, 1, 2, x, 2*x, x^2, x+1, 2*x^2, x^3, x^4, ...]"
    );
    // A modulus that is not a power of 2 admits coefficients a power of 2 would not.
    assert_eq!(
        strings(exhaustive_u64_polynomials_reduced_mod(5), 10),
        "[0, 1, 2, x, 3, 2*x, 4, x+1, x^2, x^3, ...]"
    );
}

#[test]
#[should_panic]
fn exhaustive_u64_polynomials_reduced_mod_fail_zero() {
    exhaustive_u64_polynomials_reduced_mod(0);
}

#[test]
#[should_panic]
fn exhaustive_u64_polynomials_reduced_mod_fail_one() {
    exhaustive_u64_polynomials_reduced_mod(1);
}

#[test]
fn exhaustive_u64_polynomials_reduced_mod_properties() {
    for m in [2, 3, 5, 10, 1000, u64::MAX] {
        let ps = exhaustive_u64_polynomials_reduced_mod(m)
            .take(200)
            .collect_vec();
        assert!(ps.iter().all(U64Polynomial::is_valid));
        // Everything generated is reduced, which is the whole point.
        assert!(ps.iter().all(|p| p.mod_is_reduced(&m)));
        // Nothing is generated twice, and the zero polynomial comes first.
        assert_eq!(ps.iter().collect::<HashSet<_>>().len(), ps.len());
        assert_eq!(ps[0], U64Polynomial::ZERO);
    }

    // A power-of-2 modulus gives the same polynomials as the power-of-2 generator, in the same
    // order: both restrict the coefficients to the same range.
    for pow in [1, 2, 3, 8] {
        assert_eq!(
            strings(exhaustive_u64_polynomials_reduced_mod(1 << pow), 50),
            strings(exhaustive_u64_polynomials_reduced_mod_power_of_2(pow), 50)
        );
    }

    // A larger modulus admits everything a smaller one does.
    let narrower: HashSet<_> = exhaustive_u64_polynomials_reduced_mod(3)
        .take(200)
        .collect();
    let wider: HashSet<_> = exhaustive_u64_polynomials_reduced_mod(5)
        .take(20000)
        .collect();
    assert!(narrower.is_subset(&wider));
}
