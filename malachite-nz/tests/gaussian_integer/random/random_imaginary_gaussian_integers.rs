// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::gaussian_integer::random::random_imaginary_gaussian_integers;

fn random_imaginary_gaussian_integers_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_imaginary_gaussian_integers(
            EXAMPLE_SEED,
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
fn test_random_imaginary_gaussian_integers() {
    // mean bits = 65/64
    random_imaginary_gaussian_integers_helper(
        65,
        64,
        &[
            "2i", "0", "-i", "0", "-6i", "0", "-2i", "98i", "i", "0", "0", "i", "-15i", "3i",
            "-18i", "-i", "0", "-67i", "i", "i",
        ],
    );
    // mean bits = 2
    random_imaginary_gaussian_integers_helper(
        2,
        1,
        &[
            "i", "0", "-24i", "18i", "-6i", "-18i", "4i", "2i", "0", "-3i", "-i", "-i", "-9i",
            "-35i", "-6i", "-8i", "0", "0", "i", "6i",
        ],
    );
    // mean bits = 32
    random_imaginary_gaussian_integers_helper(
        32,
        1,
        &[
            "89270i",
            "69403499476962893258904i",
            "62i",
            "-1848070042786i",
            "-64671510460i",
            "-696i",
            "0",
            "-79i",
            "70819i",
            "7330i",
            "215441i",
            "-424643i",
            "-11858i",
            "-84146163512i",
            "-7212822200i",
            "1518i",
            "23i",
            "-909i",
            "-60054i",
            "-46i",
        ],
    );
    // mean bits = 64
    random_imaginary_gaussian_integers_helper(
        64,
        1,
        &[
            "15542i",
            "204354108892664954266560767940941860034994328i",
            "5282i",
            "-323516i",
            "-400812728i",
            "-248570628312176883893327i",
            "5606382754i",
            "-63523217i",
            "-15024295498724618356672330435i",
            "25408382788335305673841323624499957642146385720i",
            "70153184455655i",
            "33157733495351097449766897571769262785295460456592996025656609489115364170390153697558\
            40712936487655650300919339856269i",
            "-2179070834703641056854463566957970466590674233219693760530182904389383i",
            "-5826316i",
            "-8647284i",
            "-i",
            "43088412843029635753589496830104451113312i",
            "18608i",
            "-3946823889925i",
            "-114916707179919722397i",
        ],
    );
}

#[test]
fn random_imaginary_gaussian_integers_axis() {
    assert!(
        random_imaginary_gaussian_integers(EXAMPLE_SEED, 32, 1)
            .take(100)
            .all(|x| x.real == 0u32)
    );
}

#[test]
#[should_panic]
fn random_imaginary_gaussian_integers_fail_1() {
    let _ = random_imaginary_gaussian_integers(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn random_imaginary_gaussian_integers_fail_2() {
    let _ = random_imaginary_gaussian_integers(EXAMPLE_SEED, u64::MAX, 1);
}
