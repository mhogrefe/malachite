// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::iterators::prefix_to_string;
use malachite_base::num::arithmetic::traits::{Height, ModIsReduced, ModPowerOf2IsReduced};
use malachite_base::num::logic::traits::{BitIterable, LowMask};
use malachite_base::num::random::{random_positive_unsigneds, random_primitive_ints};
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::u64_polynomial::U64Polynomial;
use malachite_base::u64_polynomial::random::*;

#[test]
fn test_random_u64_polynomials() {
    assert_eq!(
        prefix_to_string(random_u64_polynomials(EXAMPLE_SEED, 1, 1), 5),
        "[6282517168718784610, 3854918945212287108*x+16126131237969988437, 3848495687584076941*x+16\
        908237734149745446, 8242875068444962379*x+10938355129926736414, 33570146165392012, ...]"
    );
    // The convenience function is the general one with the usual iterators.
    assert_eq!(
        prefix_to_string(
            random_u64_polynomials_from_iterators(
                EXAMPLE_SEED,
                &|seed| random_primitive_ints(seed),
                &|seed| random_positive_unsigneds(seed),
                1,
                1,
            ),
            5
        ),
        prefix_to_string(random_u64_polynomials(EXAMPLE_SEED, 1, 1), 5)
    );
}

#[test]
fn test_random_u64_polynomials_degree_bounds() {
    assert_eq!(
        prefix_to_string(random_u64_polynomials_with_degree(EXAMPLE_SEED, 2), 5),
        "[6282517168718784610*x^2+16908237734149745446*x+16126131237969988437, 3854918945212287108*\
        x^2+12663883950309859797*x+10938355129926736414, 3848495687584076941*x^2+160309163093886283\
        38*x+14328508029084493994, 8242875068444962379*x^2+6855165495190718789*x+527496784918977578\
        9, 33570146165392012*x^2+5364743571823285937*x+12452306358869796714, ...]"
    );
    assert_eq!(
        prefix_to_string(random_u64_polynomials_min_degree(EXAMPLE_SEED, 1, 3, 1), 5),
        "[6282517168718784610*x^2+16908237734149745446*x+16126131237969988437, 3854918945212287108*\
        x^3+14328508029084493994*x^2+12663883950309859797*x+10938355129926736414, 38484956875840769\
        41*x^3+6855165495190718789*x^2+5274967849189775789*x+16030916309388628338, 8242875068444962\
        379*x^3+18084098515246349065*x^2+5364743571823285937*x+12452306358869796714, 33570146165392\
        012*x^2+8082601913180739774*x+4929296619887363376, ...]"
    );
    assert_eq!(
        prefix_to_string(random_u64_polynomials_degree_range(EXAMPLE_SEED, 1, 3), 5),
        "[6282517168718784610*x^2+16908237734149745446*x+16126131237969988437, 3854918945212287108*\
        x+10938355129926736414, 3848495687584076941*x^2+14328508029084493994*x+12663883950309859797\
        , 8242875068444962379*x^2+5274967849189775789*x+16030916309388628338, 33570146165392012*x+6\
        855165495190718789, ...]"
    );
    assert_eq!(
        prefix_to_string(
            random_u64_polynomials_degree_inclusive_range(EXAMPLE_SEED, 1, 2),
            5
        ),
        prefix_to_string(random_u64_polynomials_degree_range(EXAMPLE_SEED, 1, 3), 5)
    );
}

#[test]
fn test_striped_random_u64_polynomials() {
    assert_eq!(
        prefix_to_string(striped_random_u64_polynomials(EXAMPLE_SEED, 16, 1, 1, 1), 5),
        "[271656550527, 27127151148662784*x+18302682203357708288, 8866461766451184*x+18446744005015\
        272960, 18446181398633971712*x+9727775212300075008, 18446708889362628608, ...]"
    );
    assert_eq!(
        prefix_to_string(
            striped_random_u64_polynomials_with_degree(EXAMPLE_SEED, 2, 16, 1),
            5
        ),
        "[271656550527*x^2+18446744005015272960*x+18302682203357708288, 27127151148662784*x^2+22517\
        99813816318*x+9727775212300075008, 8866461766451184*x^2+79164805742588*x+4398046510592, 184\
        46181398633971712*x^2+13835058055549583359*x+31525223161659391, 18446708889362628608*x^2+90\
        07199254740543*x+9223652962075148288, ...]"
    );
    assert_eq!(
        prefix_to_string(
            striped_random_u64_polynomials_min_degree(EXAMPLE_SEED, 1, 16, 1, 3, 1),
            5
        ),
        "[271656550527*x^2+18446744005015272960*x+18302682203357708288, 27127151148662784*x^3+43980\
        46510592*x^2+2251799813816318*x+9727775212300075008, 8866461766451184*x^3+13835058055549583\
        359*x^2+31525223161659391*x+79164805742588, 18446181398633971712*x^3+4398046446591*x^2+9007\
        199254740543*x+9223652962075148288, 18446708889362628608*x^2+35047000244217*x, ...]"
    );
    assert_eq!(
        prefix_to_string(
            striped_random_u64_polynomials_degree_range(EXAMPLE_SEED, 1, 3, 16, 1),
            5
        ),
        "[271656550527*x^2+18446744005015272960*x+18302682203357708288, 27127151148662784*x+9727775\
        212300075008, 8866461766451184*x^2+4398046510592*x+2251799813816318, 18446181398633971712*x\
        ^2+31525223161659391*x+79164805742588, 18446708889362628608*x+13835058055549583359, ...]"
    );
    assert_eq!(
        prefix_to_string(
            striped_random_u64_polynomials_degree_inclusive_range(EXAMPLE_SEED, 1, 2, 16, 1),
            5
        ),
        prefix_to_string(
            striped_random_u64_polynomials_degree_range(EXAMPLE_SEED, 1, 3, 16, 1),
            5
        )
    );
}

#[test]
#[should_panic]
fn random_u64_polynomials_degree_range_fail() {
    random_u64_polynomials_degree_range(EXAMPLE_SEED, 3, 3);
}

#[test]
#[should_panic]
fn random_u64_polynomials_degree_inclusive_range_fail() {
    random_u64_polynomials_degree_inclusive_range(EXAMPLE_SEED, 3, 2);
}

#[test]
#[should_panic]
fn striped_random_u64_polynomials_degree_range_fail() {
    striped_random_u64_polynomials_degree_range(EXAMPLE_SEED, 3, 2, 16, 1);
}

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
fn test_random_u64_polynomials_reduced_mod_power_of_2() {
    assert_eq!(
        prefix_to_string(
            random_u64_polynomials_reduced_mod_power_of_2(EXAMPLE_SEED, 4, 2, 1),
            3
        ),
        "[3*x^5+8*x^4+11*x^2+5*x+5, 7, 9*x^7+8*x^6+6*x^5+14*x^4+11*x^3+12*x^2+8*x+8, ...]"
    );
    assert_eq!(
        prefix_to_string(
            striped_random_u64_polynomials_reduced_mod_power_of_2(EXAMPLE_SEED, 8, 8, 1, 2, 1),
            3
        ),
        "[248*x^5+7*x^4+31*x^3+248*x^2+220*x+255, 224, \
        111*x^7+112*x^4+255*x^3+255*x^2+x+3, ...]"
    );
}

#[test]
#[should_panic]
fn random_u64_polynomials_reduced_mod_power_of_2_fail_zero() {
    random_u64_polynomials_reduced_mod_power_of_2(EXAMPLE_SEED, 0, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_u64_polynomials_reduced_mod_power_of_2_fail_too_wide() {
    striped_random_u64_polynomials_reduced_mod_power_of_2(EXAMPLE_SEED, 65, 8, 1, 2, 1);
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
fn test_random_u64_polynomials_reduced_mod() {
    assert_eq!(
        prefix_to_string(
            random_u64_polynomials_reduced_mod(EXAMPLE_SEED, 10, 2, 1),
            3
        ),
        "[3*x^5+8*x^4+8*x^3+5*x+5, 7, 9*x^7+x^6+9*x^5+2*x^4+6*x^3+8*x^2+6*x+8, ...]"
    );
    assert_eq!(
        prefix_to_string(
            striped_random_u64_polynomials_reduced_mod(EXAMPLE_SEED, 1000, 8, 1, 2, 1),
            3
        ),
        "[x^5+3*x^4+x^3+62*x^2+952*x+999, 1, x^7+511*x^3+7*x^2+999*x+7, ...]"
    );
}

#[test]
#[should_panic]
fn random_u64_polynomials_reduced_mod_fail_one() {
    random_u64_polynomials_reduced_mod(EXAMPLE_SEED, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_u64_polynomials_reduced_mod_fail_zero() {
    striped_random_u64_polynomials_reduced_mod(EXAMPLE_SEED, 0, 8, 1, 2, 1);
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
