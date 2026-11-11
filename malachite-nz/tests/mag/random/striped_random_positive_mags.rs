// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::mag::random::striped_random_positive_mags;

fn striped_random_positive_mags_helper(
    mean_exponent_abs_numerator: u64,
    mean_exponent_abs_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_positive_mags(
            EXAMPLE_SEED,
            mean_exponent_abs_numerator,
            mean_exponent_abs_denominator,
            mean_stripe_numerator,
            mean_stripe_denominator
        )
        .take(20)
        .map(|x| x.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_striped_random_positive_mags() {
    // mean |exponent| = 1, mean stripe = 2
    striped_random_positive_mags_helper(
        1,
        1,
        2,
        1,
        &[
            "328720909*2^-29",
            "1012714141*2^-30",
            "993460883*2^-29",
            "680006147*2^-30",
            "728128611*2^-30",
            "439021051*2^-28",
            "710971743*2^-29",
            "779787735*2^-30",
            "206055909*2^-27",
            "200672379*2^-28",
            "629562893*2^-29",
            "861749051*2^-29",
            "860865579*2^-28",
            "626458823*2^-30",
            "1070420357*2^-30",
            "680186953*2^-29",
            "226447323*2^-31",
            "497794327*2^-29",
            "1018040923*2^-32",
            "388681805*2^-29",
        ],
    );
    // mean |exponent| = 10, mean stripe = 4
    striped_random_positive_mags_helper(
        10,
        1,
        4,
        1,
        &[
            "570394577*2^-31",
            "33038751*2^-45",
            "217583263*2^-35",
            "168787897*2^-29",
            "568264703*2^-14",
            "941591039*2^-37",
            "603909119*2^-36",
            "543142467*2^-29",
            "931394791*2^-28",
            "285409991*2^-44",
            "134316599*2^-30",
            "1073710071*2^-16",
            "1048071*2^-5",
            "305671175*2^-29",
            "805306421*2^-25",
            "667332353*2^-32",
            "1070071555*2^-40",
            "259915839*2^-18",
            "3989231*2^-25",
            "570352639*2^-40",
        ],
    );
    // mean |exponent| = 10, mean stripe = 32
    striped_random_positive_mags_helper(
        10,
        1,
        32,
        1,
        &[
            "537395199*2^-31",
            "939556863*2^-50",
            "524287*2^-26",
            "536936447*2^-31",
            "1*2^15",
            "534839295*2^-36",
            "17039359*2^-31",
            "1",
            "1073740927*2^-28",
            "1*2^-16",
            "536870919*2^-32",
            "1073741823*2^-16",
            "1065861119*2^-15",
            "1*2^-1",
            "1073479683*2^-25",
            "1*2^-3",
            "65535*2^-26",
            "1073741823*2^-20",
            "127*2^-10",
            "1*2^-11",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_positive_mags_fail_1() {
    striped_random_positive_mags(EXAMPLE_SEED, 0, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_positive_mags_fail_2() {
    striped_random_positive_mags(EXAMPLE_SEED, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_positive_mags_fail_3() {
    striped_random_positive_mags(EXAMPLE_SEED, 1, 1, 1, 1);
}

#[test]
#[should_panic]
fn striped_random_positive_mags_fail_4() {
    striped_random_positive_mags(EXAMPLE_SEED, 1, 1, 2, 0);
}
