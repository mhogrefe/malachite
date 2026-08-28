// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::gaussian_integer::random::striped_random_real_gaussian_integers;

fn striped_random_real_gaussian_integers_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_real_gaussian_integers(
            EXAMPLE_SEED,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator
        )
        .take(20)
        .map(|x| x.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_striped_random_real_gaussian_integers() {
    // mean stripe = 2, mean bits = 2
    striped_random_real_gaussian_integers_helper(
        2,
        1,
        2,
        1,
        &[
            "1", "0", "-28", "28", "-4", "-22", "7", "2", "0", "-3", "-1", "-1", "-9", "-40", "-4",
            "-12", "0", "0", "1", "6",
        ],
    );
    // mean stripe = 4, mean bits = 32
    striped_random_real_gaussian_integers_helper(
        4,
        1,
        32,
        1,
        &[
            "122880",
            "75540121799304929871856",
            "44",
            "-1907242564614",
            "-66605579808",
            "-1017",
            "0",
            "-112",
            "114673",
            "4604",
            "155889",
            "-262271",
            "-8192",
            "-107255954944",
            "-7548692088",
            "1823",
            "28",
            "-545",
            "-56952",
            "-32",
        ],
    );
    // mean stripe = 16, mean bits = 32
    striped_random_real_gaussian_integers_helper(
        16,
        1,
        32,
        1,
        &[
            "65536",
            "75521006248971741167616",
            "32",
            "-2199023255520",
            "-68719468544",
            "-527",
            "0",
            "-112",
            "131071",
            "4152",
            "262143",
            "-262145",
            "-8192",
            "-137405429760",
            "-4294967296",
            "1219",
            "16",
            "-1023",
            "-32768",
            "-32",
        ],
    );
    // mean stripe = 32, mean bits = 64
    striped_random_real_gaussian_integers_helper(
        32,
        1,
        64,
        1,
        &[
            "8192",
            "178427569518544464724715670468776264076361728",
            "8176",
            "-262144",
            "-268435456",
            "-226655146685469074391039",
            "4294967296",
            "-67108863",
            "-19807040628566083848630173696",
            "45671926166590716193865150952632647489410830335",
            "43978334404607",
            "25217283965692466698592647766367652888868773818546142944566019485979788718647436525711\
            32639068666062843684114535546880",
            "-1728806579227565766676057273846916536097145074328900789155504620306432",
            "-4194304",
            "-16777215",
            "-1",
            "43556142803623322374103370143943282917375",
            "31742",
            "-4123168604160",
            "-129703669268270284799",
        ],
    );
}

#[test]
fn striped_random_real_gaussian_integers_axis() {
    assert!(
        striped_random_real_gaussian_integers(EXAMPLE_SEED, 16, 1, 32, 1)
            .take(100)
            .all(|x| x.imaginary == 0)
    );
}

#[test]
#[should_panic]
fn striped_random_real_gaussian_integers_fail_1() {
    let _ = striped_random_real_gaussian_integers(EXAMPLE_SEED, 1, 2, 32, 1);
}

#[test]
#[should_panic]
fn striped_random_real_gaussian_integers_fail_2() {
    let _ = striped_random_real_gaussian_integers(EXAMPLE_SEED, 2, 1, 1, 0);
}
