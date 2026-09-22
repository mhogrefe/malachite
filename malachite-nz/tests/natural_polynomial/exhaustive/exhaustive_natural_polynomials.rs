// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::iterators::prefix_to_string;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModPowerOf2IsReduced, PowerOf2};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::u64_polynomial::exhaustive::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural::exhaustive::{exhaustive_naturals, exhaustive_positive_naturals};
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::natural_polynomial::exhaustive::*;
use std::collections::HashSet;

fn strings<I: Iterator<Item = NaturalPolynomial>>(xs: I, n: usize) -> String {
    prefix_to_string(xs, n)
}

#[test]
fn test_exhaustive_natural_polynomials() {
    assert_eq!(
        strings(exhaustive_natural_polynomials(), 20),
        "[0, 1, 2, x, 3, 2*x, 4, x+1, x^2, x^3, 2*x^2, 2*x^3, x^2+x, x^3+x^2, 2*x^2+x, \
        2*x^3+x^2, 5, 2*x+1, 6, 3*x, ...]"
    );
    assert_eq!(
        strings(
            exhaustive_natural_polynomials_from_iterators(
                exhaustive_naturals(),
                exhaustive_positive_naturals()
            ),
            20
        ),
        strings(exhaustive_natural_polynomials(), 20)
    );
    assert_eq!(
        strings(
            exhaustive_natural_polynomials_from_iterators(
                exhaustive_positive_naturals(),
                exhaustive_positive_naturals()
            ),
            10
        ),
        "[0, 1, 2, x+1, 3, 2*x+1, 4, x+2, x^2+x+1, x^3+x^2+x+1, ...]"
    );
}

#[test]
fn test_exhaustive_natural_polynomials_with_degree() {
    assert_eq!(
        strings(exhaustive_natural_polynomials_with_degree(0), 10),
        "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, ...]"
    );
    assert_eq!(
        strings(exhaustive_natural_polynomials_with_degree(1), 10),
        "[x, 2*x, x+1, 2*x+1, 3*x, 4*x, 3*x+1, 4*x+1, x+2, 2*x+2, ...]"
    );
    assert_eq!(
        strings(exhaustive_natural_polynomials_with_degree(2), 10),
        "[x^2, 2*x^2, x^2+x, 2*x^2+x, x^2+1, 2*x^2+1, x^2+x+1, 2*x^2+x+1, 3*x^2, 4*x^2, ...]"
    );
}

#[test]
fn test_exhaustive_natural_polynomials_degree_bounds() {
    assert_eq!(
        strings(exhaustive_natural_polynomials_min_degree(0), 10),
        "[1, x, 2, 2*x, 3, x+1, 4, 2*x+1, x^2, x^3, ...]"
    );
    assert_eq!(
        strings(exhaustive_natural_polynomials_min_degree(2), 10),
        "[x^2, x^3, 2*x^2, 2*x^3, x^2+x, x^3+x^2, 2*x^2+x, 2*x^3+x^2, x^4, x^5, ...]"
    );
    assert_eq!(
        strings(exhaustive_natural_polynomials_degree_range(1, 3), 10),
        "[x, x^2, 2*x, 2*x^2, x+1, x^2+x, 2*x+1, 2*x^2+x, 3*x, x^2+1, ...]"
    );
    assert_eq!(
        strings(
            exhaustive_natural_polynomials_degree_inclusive_range(1, 2),
            10
        ),
        strings(exhaustive_natural_polynomials_degree_range(1, 3), 10)
    );
    // An empty range gives nothing.
    assert_eq!(
        strings(exhaustive_natural_polynomials_degree_range(3, 3), 10),
        "[]"
    );
    assert_eq!(
        strings(exhaustive_natural_polynomials_degree_range(3, 2), 10),
        "[]"
    );
    assert_eq!(
        strings(
            exhaustive_natural_polynomials_degree_inclusive_range(3, 2),
            10
        ),
        "[]"
    );
}

#[test]
fn exhaustive_natural_polynomials_properties() {
    // Everything generated is valid, and nothing is generated twice.
    let ps = exhaustive_natural_polynomials().take(500).collect_vec();
    assert!(ps.iter().all(NaturalPolynomial::is_valid));
    assert_eq!(ps.iter().collect::<HashSet<_>>().len(), ps.len());
    // The zero polynomial comes first, and only once.
    assert_eq!(ps[0], NaturalPolynomial::ZERO);
    assert_eq!(ps.iter().filter(|p| p.degree().is_none()).count(), 1);

    for d in 0..4 {
        let ps = exhaustive_natural_polynomials_with_degree(d)
            .take(200)
            .collect_vec();
        assert!(ps.iter().all(NaturalPolynomial::is_valid));
        assert!(ps.iter().all(|p| p.degree() == Some(d)));
        assert_eq!(ps.iter().collect::<HashSet<_>>().len(), ps.len());

        let ps = exhaustive_natural_polynomials_min_degree(d)
            .take(200)
            .collect_vec();
        assert!(ps.iter().all(|p| p.degree().unwrap() >= d));
        assert_eq!(ps.iter().collect::<HashSet<_>>().len(), ps.len());
    }

    for (a, b) in [(0u64, 1u64), (1, 3), (2, 5), (0, 4)] {
        let ps = exhaustive_natural_polynomials_degree_range(a, b)
            .take(200)
            .collect_vec();
        assert!(ps.iter().all(|p| {
            let d = p.degree().unwrap();
            d >= a && d < b
        }));
        assert_eq!(ps.iter().collect::<HashSet<_>>().len(), ps.len());

        let ps = exhaustive_natural_polynomials_degree_inclusive_range(a, b)
            .take(200)
            .collect_vec();
        assert!(ps.iter().all(|p| {
            let d = p.degree().unwrap();
            d >= a && d <= b
        }));
    }

    // A fixed degree is the inclusive range of that degree with itself.
    assert_eq!(
        exhaustive_natural_polynomials_with_degree(2)
            .take(50)
            .collect_vec(),
        exhaustive_natural_polynomials_degree_inclusive_range(2, 2)
            .take(50)
            .collect_vec()
    );
}

#[test]
fn test_exhaustive_natural_polynomials_reduced_mod_power_of_2() {
    // Modulo 2 the coefficients are all 0 or 1, so these are the polynomials over GF(2).
    assert_eq!(
        strings(exhaustive_natural_polynomials_reduced_mod_power_of_2(1), 10),
        "[0, 1, x, x^2, x+1, x^2+x, x^2+1, x^3, x^4, x^5, ...]"
    );
    assert_eq!(
        strings(exhaustive_natural_polynomials_reduced_mod_power_of_2(2), 10),
        "[0, 1, 2, x, 3, 2*x, x+1, x^2, x^3, x^4, ...]"
    );
}

#[test]
#[should_panic]
fn exhaustive_natural_polynomials_reduced_mod_power_of_2_fail() {
    exhaustive_natural_polynomials_reduced_mod_power_of_2(0);
}

#[test]
fn exhaustive_natural_polynomials_reduced_mod_power_of_2_properties() {
    for pow in [1, 2, 3, 8, 64, 100] {
        let ps = exhaustive_natural_polynomials_reduced_mod_power_of_2(pow)
            .take(200)
            .collect_vec();
        assert!(ps.iter().all(NaturalPolynomial::is_valid));
        // Everything generated is reduced, which is the whole point.
        assert!(ps.iter().all(|p| p.mod_power_of_2_is_reduced(pow)));
        // Nothing is generated twice, and the zero polynomial comes first.
        assert_eq!(ps.iter().collect::<HashSet<_>>().len(), ps.len());
        assert_eq!(ps[0], NaturalPolynomial::ZERO);
    }

    // The `u64` and `Natural` polynomials enumerate the same set in the same order: the restriction
    // is the same, and the coefficients agree.
    for pow in [1, 2, 3, 8] {
        assert_eq!(
            strings(
                exhaustive_natural_polynomials_reduced_mod_power_of_2(pow),
                50
            ),
            prefix_to_string(exhaustive_u64_polynomials_reduced_mod_power_of_2(pow), 50)
        );
    }
}

#[test]
fn test_exhaustive_natural_polynomials_reduced_mod() {
    assert_eq!(
        strings(
            exhaustive_natural_polynomials_reduced_mod(Natural::from(3u32)),
            10
        ),
        "[0, 1, 2, x, 2*x, x^2, x+1, 2*x^2, x^3, x^4, ...]"
    );
    // A modulus that is not a power of 2 admits coefficients a power of 2 would not.
    assert_eq!(
        strings(
            exhaustive_natural_polynomials_reduced_mod(Natural::from(5u32)),
            10
        ),
        "[0, 1, 2, x, 3, 2*x, 4, x+1, x^2, x^3, ...]"
    );
}

#[test]
#[should_panic]
fn exhaustive_natural_polynomials_reduced_mod_fail_zero() {
    exhaustive_natural_polynomials_reduced_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn exhaustive_natural_polynomials_reduced_mod_fail_one() {
    exhaustive_natural_polynomials_reduced_mod(Natural::ONE);
}

#[test]
fn exhaustive_natural_polynomials_reduced_mod_properties() {
    for m in [2u32, 3, 5, 10, 1000] {
        let m = Natural::from(m);
        let ps = exhaustive_natural_polynomials_reduced_mod(m.clone())
            .take(200)
            .collect_vec();
        assert!(ps.iter().all(NaturalPolynomial::is_valid));
        // Everything generated is reduced, which is the whole point.
        assert!(ps.iter().all(|p| p.mod_is_reduced(&m)));
        // Nothing is generated twice, and the zero polynomial comes first.
        assert_eq!(ps.iter().collect::<HashSet<_>>().len(), ps.len());
        assert_eq!(ps[0], NaturalPolynomial::ZERO);
    }

    // A power-of-2 modulus gives the same polynomials, in the same order, as the power-of-2
    // generator: both restrict the coefficients to the same range.
    for pow in [1, 2, 3, 8] {
        assert_eq!(
            strings(
                exhaustive_natural_polynomials_reduced_mod(Natural::power_of_2(pow)),
                50
            ),
            strings(
                exhaustive_natural_polynomials_reduced_mod_power_of_2(pow),
                50
            )
        );
    }

    // The `u64` and `Natural` polynomials enumerate the same set in the same order.
    for m in [2u64, 3, 5, 10, 1000] {
        assert_eq!(
            strings(
                exhaustive_natural_polynomials_reduced_mod(Natural::from(m)),
                50
            ),
            prefix_to_string(exhaustive_u64_polynomials_reduced_mod(m), 50)
        );
    }
}
