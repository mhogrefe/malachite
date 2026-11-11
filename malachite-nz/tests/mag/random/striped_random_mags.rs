// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::mag::random::striped_random_mags;

fn striped_random_mags_helper(
    mean_exponent_abs_numerator: u64,
    mean_exponent_abs_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    zero_p_numerator: u64,
    zero_p_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_mags(
            EXAMPLE_SEED,
            mean_exponent_abs_numerator,
            mean_exponent_abs_denominator,
            mean_stripe_numerator,
            mean_stripe_denominator,
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
fn test_striped_random_mags() {
    // mean |exponent| = 1, mean stripe = 2, P(0) = 1/10
    striped_random_mags_helper(
        1,
        1,
        2,
        1,
        1,
        10,
        &[
            "77894653*2^-24",
            "993992167*2^-30",
            "948475287*2^-31",
            "0",
            "392483451*2^-27",
            "601275521*2^-29",
            "321524681*2^-29",
            "578575479*2^-29",
            "400352421*2^-30",
            "731242021*2^-31",
            "764889221*2^-29",
            "818697489*2^-25",
            "551891771*2^-31",
            "0",
            "629962725*2^-30",
            "773419433*2^-29",
            "740056439*2^-31",
            "0",
            "787813141*2^-30",
            "74598185*2^-30",
        ],
    );
    // mean |exponent| = 10, mean stripe = 4, P(0) = 1/10
    striped_random_mags_helper(
        10,
        1,
        4,
        1,
        1,
        10,
        &[
            "641663007*2^-14",
            "254951199*2^-34",
            "1053031297*2^-7",
            "0",
            "669056973*2^-37",
            "397463143*2^-35",
            "1051647*2^-20",
            "167518145*2^-27",
            "269320449*2^-30",
            "545047555*2^-17",
            "17819659*2^-22",
            "15735751*2^-30",
            "596050205*2^-22",
            "0",
            "658534435*2^-20",
            "98828223*2^-38",
            "778074883*2^-15",
            "0",
            "541590079*2^-45",
            "150626531*2^-24",
        ],
    );
    // mean |exponent| = 10, mean stripe = 32, P(0) = 1/2
    striped_random_mags_helper(
        10,
        1,
        32,
        1,
        1,
        2,
        &[
            "0",
            "0",
            "0",
            "1*2^15",
            "0",
            "939528191*2^-36",
            "0",
            "0",
            "0",
            "1040187519*2^-7",
            "536871039*2^-37",
            "536870927*2^-36",
            "0",
            "0",
            "570425343*2^-29",
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
fn striped_random_mags_fail_1() {
    striped_random_mags(EXAMPLE_SEED, 0, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_mags_fail_2() {
    striped_random_mags(EXAMPLE_SEED, 1, 1, 1, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_mags_fail_3() {
    striped_random_mags(EXAMPLE_SEED, 1, 1, 2, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_mags_fail_4() {
    striped_random_mags(EXAMPLE_SEED, 1, 1, 2, 1, 2, 1);
}
