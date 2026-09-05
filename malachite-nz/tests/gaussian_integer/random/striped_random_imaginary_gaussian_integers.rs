// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::gaussian_integer::random::striped_random_imaginary_gaussian_integers;

fn striped_random_imaginary_gaussian_integers_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_imaginary_gaussian_integers(
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
fn test_striped_random_imaginary_gaussian_integers() {
    // mean stripe = 2, mean bits = 2
    striped_random_imaginary_gaussian_integers_helper(
        2,
        1,
        2,
        1,
        &[
            "i", "0", "-28i", "28i", "-4i", "-22i", "7i", "2i", "0", "-3i", "-i", "-i", "-9i",
            "-40i", "-4i", "-12i", "0", "0", "i", "6i",
        ],
    );
    // mean stripe = 4, mean bits = 32
    striped_random_imaginary_gaussian_integers_helper(
        4,
        1,
        32,
        1,
        &[
            "122880i",
            "75540121799304929871856i",
            "44i",
            "-1907242564614i",
            "-66605579808i",
            "-1017i",
            "0",
            "-112i",
            "114673i",
            "4604i",
            "155889i",
            "-262271i",
            "-8192i",
            "-107255954944i",
            "-7548692088i",
            "1823i",
            "28i",
            "-545i",
            "-56952i",
            "-32i",
        ],
    );
    // mean stripe = 16, mean bits = 32
    striped_random_imaginary_gaussian_integers_helper(
        16,
        1,
        32,
        1,
        &[
            "65536i",
            "75521006248971741167616i",
            "32i",
            "-2199023255520i",
            "-68719468544i",
            "-527i",
            "0",
            "-112i",
            "131071i",
            "4152i",
            "262143i",
            "-262145i",
            "-8192i",
            "-137405429760i",
            "-4294967296i",
            "1219i",
            "16i",
            "-1023i",
            "-32768i",
            "-32i",
        ],
    );
    // mean stripe = 32, mean bits = 64
    striped_random_imaginary_gaussian_integers_helper(
        32,
        1,
        64,
        1,
        &[
            "8192i",
            "178427569518544464724715670468776264076361728i",
            "8176i",
            "-262144i",
            "-268435456i",
            "-226655146685469074391039i",
            "4294967296i",
            "-67108863i",
            "-19807040628566083848630173696i",
            "45671926166590716193865150952632647489410830335i",
            "43978334404607i",
            "25217283965692466698592647766367652888868773818546142944566019485979788718647436525711\
            32639068666062843684114535546880i",
            "-1728806579227565766676057273846916536097145074328900789155504620306432i",
            "-4194304i",
            "-16777215i",
            "-i",
            "43556142803623322374103370143943282917375i",
            "31742i",
            "-4123168604160i",
            "-129703669268270284799i",
        ],
    );
}

#[test]
fn striped_random_imaginary_gaussian_integers_axis() {
    assert!(
        striped_random_imaginary_gaussian_integers(EXAMPLE_SEED, 16, 1, 32, 1)
            .take(100)
            .all(|x| x.real == 0u32)
    );
}

#[test]
#[should_panic]
fn striped_random_imaginary_gaussian_integers_fail_1() {
    let _ = striped_random_imaginary_gaussian_integers(EXAMPLE_SEED, 1, 2, 32, 1);
}

#[test]
#[should_panic]
fn striped_random_imaginary_gaussian_integers_fail_2() {
    let _ = striped_random_imaginary_gaussian_integers(EXAMPLE_SEED, 2, 1, 1, 0);
}
