// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::mag::random::random_mags;

fn random_mags_helper(
    mean_exponent_abs_numerator: u64,
    mean_exponent_abs_denominator: u64,
    zero_p_numerator: u64,
    zero_p_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_mags(
            EXAMPLE_SEED,
            mean_exponent_abs_numerator,
            mean_exponent_abs_denominator,
            zero_p_numerator,
            zero_p_denominator
        )
        .take(20)
        .map(|x| x.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_mags() {
    // mean |exponent| = 1, P(0) = 1/10
    random_mags_helper(
        1,
        1,
        1,
        10,
        &[
            "943339659*2^-27",
            "562936945*2^-30",
            "704202107*2^-31",
            "0",
            "314890179*2^-27",
            "150491401*2^-27",
            "571808423*2^-30",
            "507002221*2^-28",
            "788563677*2^-31",
            "904270201*2^-31",
            "567944497*2^-29",
            "411458535*2^-24",
            "960042541*2^-31",
            "0",
            "68636089*2^-27",
            "1048163055*2^-29",
            "221623897*2^-29",
            "0",
            "79341665*2^-27",
            "301925027*2^-32",
        ],
    );
    // mean |exponent| = 10, P(0) = 1/10
    random_mags_helper(
        10,
        1,
        1,
        10,
        &[
            "943339659*2^-14",
            "562936945*2^-36",
            "704202107*2^-7",
            "0",
            "314890179*2^-36",
            "150491401*2^-34",
            "571808423*2^-29",
            "507002221*2^-28",
            "788563677*2^-31",
            "904270201*2^-17",
            "567944497*2^-27",
            "411458535*2^-35",
            "960042541*2^-22",
            "0",
            "68636089*2^-17",
            "1048163055*2^-41",
            "221623897*2^-13",
            "0",
            "79341665*2^-42",
            "301925027*2^-25",
        ],
    );
    // mean |exponent| = 10, P(0) = 1/2
    random_mags_helper(
        10,
        1,
        1,
        2,
        &[
            "0",
            "0",
            "0",
            "943339659*2^-14",
            "0",
            "562936945*2^-36",
            "0",
            "0",
            "0",
            "704202107*2^-7",
            "314890179*2^-36",
            "150491401*2^-34",
            "0",
            "0",
            "571808423*2^-29",
            "0",
            "0",
            "0",
            "0",
            "0",
        ],
    );
}

#[test]
#[should_panic]
fn random_mags_fail_1() {
    random_mags(EXAMPLE_SEED, 0, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_mags_fail_2() {
    random_mags(EXAMPLE_SEED, 1, 0, 1, 10);
}

#[test]
#[should_panic]
fn random_mags_fail_3() {
    random_mags(EXAMPLE_SEED, 1, 1, 1, 0);
}

#[test]
#[should_panic]
fn random_mags_fail_4() {
    random_mags(EXAMPLE_SEED, 1, 1, 2, 1);
}
