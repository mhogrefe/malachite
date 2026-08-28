// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::gaussian_integer::random::random_real_gaussian_integers;

fn random_real_gaussian_integers_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_real_gaussian_integers(EXAMPLE_SEED, mean_bits_numerator, mean_bits_denominator)
            .take(20)
            .map(|x| x.to_string())
            .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_real_gaussian_integers() {
    // mean bits = 65/64
    random_real_gaussian_integers_helper(
        65,
        64,
        &[
            "2", "0", "-1", "0", "-6", "0", "-2", "98", "1", "0", "0", "1", "-15", "3", "-18",
            "-1", "0", "-67", "1", "1",
        ],
    );
    // mean bits = 2
    random_real_gaussian_integers_helper(
        2,
        1,
        &[
            "1", "0", "-24", "18", "-6", "-18", "4", "2", "0", "-3", "-1", "-1", "-9", "-35", "-6",
            "-8", "0", "0", "1", "6",
        ],
    );
    // mean bits = 32
    random_real_gaussian_integers_helper(
        32,
        1,
        &[
            "89270",
            "69403499476962893258904",
            "62",
            "-1848070042786",
            "-64671510460",
            "-696",
            "0",
            "-79",
            "70819",
            "7330",
            "215441",
            "-424643",
            "-11858",
            "-84146163512",
            "-7212822200",
            "1518",
            "23",
            "-909",
            "-60054",
            "-46",
        ],
    );
    // mean bits = 64
    random_real_gaussian_integers_helper(
        64,
        1,
        &[
            "15542",
            "204354108892664954266560767940941860034994328",
            "5282",
            "-323516",
            "-400812728",
            "-248570628312176883893327",
            "5606382754",
            "-63523217",
            "-15024295498724618356672330435",
            "25408382788335305673841323624499957642146385720",
            "70153184455655",
            "33157733495351097449766897571769262785295460456592996025656609489115364170390153697558\
            40712936487655650300919339856269",
            "-2179070834703641056854463566957970466590674233219693760530182904389383",
            "-5826316",
            "-8647284",
            "-1",
            "43088412843029635753589496830104451113312",
            "18608",
            "-3946823889925",
            "-114916707179919722397",
        ],
    );
}

#[test]
fn random_real_gaussian_integers_axis() {
    assert!(
        random_real_gaussian_integers(EXAMPLE_SEED, 32, 1)
            .take(100)
            .all(|x| x.imaginary == 0)
    );
}

#[test]
#[should_panic]
fn random_real_gaussian_integers_fail_1() {
    let _ = random_real_gaussian_integers(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn random_real_gaussian_integers_fail_2() {
    let _ = random_real_gaussian_integers(EXAMPLE_SEED, u64::MAX, 1);
}
