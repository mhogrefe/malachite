// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::mag::random::random_positive_mags;

fn random_positive_mags_helper(
    mean_exponent_abs_numerator: u64,
    mean_exponent_abs_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_positive_mags(
            EXAMPLE_SEED,
            mean_exponent_abs_numerator,
            mean_exponent_abs_denominator
        )
        .take(20)
        .map(|x| x.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_positive_mags() {
    // mean |exponent| = 1
    random_positive_mags_helper(
        1,
        1,
        &[
            "440583171*2^-29",
            "727019109*2^-30",
            "573448095*2^-29",
            "448791547*2^-29",
            "569778313*2^-30",
            "54392383*2^-25",
            "219847365*2^-27",
            "125337359*2^-27",
            "226494179*2^-27",
            "539626793*2^-30",
            "926315763*2^-29",
            "290623559*2^-28",
            "640129751*2^-28",
            "225395267*2^-28",
            "436086901*2^-29",
            "838455799*2^-29",
            "605557435*2^-33",
            "289093679*2^-29",
            "1038961681*2^-32",
            "163367143*2^-28",
        ],
    );
    // mean |exponent| = 10
    random_positive_mags_helper(
        10,
        1,
        &[
            "440583171*2^-30",
            "727019109*2^-50",
            "573448095*2^-37",
            "448791547*2^-30",
            "569778313*2^-14",
            "54392383*2^-33",
            "219847365*2^-34",
            "125337359*2^-26",
            "226494179*2^-26",
            "539626793*2^-45",
            "926315763*2^-32",
            "290623559*2^-15",
            "640129751*2^-15",
            "225395267*2^-28",
            "436086901*2^-24",
            "838455799*2^-32",
            "605557435*2^-40",
            "289093679*2^-19",
            "1038961681*2^-33",
            "163367143*2^-38",
        ],
    );
    // mean |exponent| = 100
    random_positive_mags_helper(
        100,
        1,
        &[
            "440583171*2^-226",
            "727019109*2^-214",
            "573448095*2^-57",
            "448791547*2^-249",
            "569778313*2^46",
            "54392383*2^-69",
            "219847365*2^-4",
            "125337359*2^-35",
            "226494179*2^-56",
            "539626793*2^51",
            "926315763*2^151",
            "290623559*2^-88",
            "640129751*2^-55",
            "225395267*2^25",
            "436086901*2^143",
            "838455799*2^-37",
            "605557435*2^-54",
            "289093679*2^71",
            "1038961681*2^-34",
            "163367143*2^-31",
        ],
    );
}

#[test]
#[should_panic]
fn random_positive_mags_fail_1() {
    random_positive_mags(EXAMPLE_SEED, 0, 1);
}

#[test]
#[should_panic]
fn random_positive_mags_fail_2() {
    random_positive_mags(EXAMPLE_SEED, 1, 0);
}
