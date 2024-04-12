// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::{CheckedHammingDistance, CountOnes, HammingDistance};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::unsigned_pair_gen_var_27;
use malachite_base::test_util::generators::{
    unsigned_vec_pair_gen_var_6, unsigned_vec_pair_gen_var_7, unsigned_vec_unsigned_pair_gen_var_15,
};
use malachite_nz::natural::logic::hamming_distance::{
    limbs_hamming_distance, limbs_hamming_distance_limb, limbs_hamming_distance_same_length,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_gen, natural_pair_gen, natural_triple_gen};
use malachite_nz::test_util::natural::logic::hamming_distance::{
    natural_hamming_distance_alt_1, natural_hamming_distance_alt_2, rug_hamming_distance,
};
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_hamming_distance_limb() {
    let test = |xs, y, out| {
        assert_eq!(limbs_hamming_distance_limb(xs, y), out);
    };
    test(&[2], 3, 1);
    test(&[1, 1, 1], 1, 2);
    test(&[1, 1, 1], 2, 4);
    test(&[1, 2, 3], 0, 4);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_hamming_distance_limb_fail() {
    limbs_hamming_distance_limb(&[], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_hamming_distance_same_length() {
    let test = |xs, ys, out| {
        assert_eq!(limbs_hamming_distance_same_length(xs, ys), out);
    };
    test(&[], &[], 0);
    test(&[2], &[3], 1);
    test(&[1, 1, 1], &[1, 2, 3], 3);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_hamming_distance_limb_same_length_fail() {
    limbs_hamming_distance_same_length(&[1], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_hamming_distance() {
    let test = |xs, ys, out| {
        assert_eq!(limbs_hamming_distance(xs, ys), out);
    };
    test(&[], &[], 0);
    test(&[2], &[3], 1);
    test(&[1, 1, 1], &[1, 2, 3], 3);
    test(&[], &[1, 2, 3], 4);
    test(&[1, 2, 3], &[], 4);
    test(&[1, 1, 1], &[1, 2, 3, 4], 4);
    test(&[1, 2, 3, 4], &[1, 1, 1], 4);
}

#[test]
fn test_hamming_distance() {
    let test = |x, y, out| {
        assert_eq!(
            Natural::from_str(x)
                .unwrap()
                .hamming_distance(&Natural::from_str(y).unwrap()),
            out
        );
        assert_eq!(
            natural_hamming_distance_alt_1(
                &Natural::from_str(x).unwrap(),
                &Natural::from_str(y).unwrap(),
            ),
            out
        );
        assert_eq!(
            natural_hamming_distance_alt_2(
                &Natural::from_str(x).unwrap(),
                &Natural::from_str(y).unwrap(),
            ),
            out
        );
        assert_eq!(
            rug_hamming_distance(
                &rug::Integer::from_str(x).unwrap(),
                &rug::Integer::from_str(y).unwrap(),
            ),
            out
        );
    };
    test("105", "123", 2);
    test("1000000000000", "0", 13);
    test("4294967295", "0", 32);
    test("4294967295", "4294967295", 0);
    test("4294967295", "4294967296", 33);
    test("1000000000000", "1000000000001", 1);
}

#[test]
fn limbs_hamming_distance_limb_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_15().test_properties_with_config(&config, |(xs, y)| {
        assert_eq!(
            limbs_hamming_distance_limb(&xs, y),
            Natural::from_owned_limbs_asc(xs).hamming_distance(&Natural::from(y))
        );
    });
}

#[test]
fn limbs_hamming_distance_same_length_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_6().test_properties_with_config(&config, |(xs, ys)| {
        assert_eq!(
            limbs_hamming_distance_same_length(&xs, &ys),
            Natural::from_owned_limbs_asc(xs).hamming_distance(&Natural::from_owned_limbs_asc(ys))
        );
    });
}

#[test]
fn limbs_hamming_distance_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_7().test_properties_with_config(&config, |(xs, ys)| {
        assert_eq!(
            limbs_hamming_distance(&xs, &ys),
            Natural::from_owned_limbs_asc(xs).hamming_distance(&Natural::from_owned_limbs_asc(ys))
        );
    });
}

#[test]
fn hamming_distance_properties() {
    natural_pair_gen().test_properties(|(x, y)| {
        let distance = x.hamming_distance(&y);
        assert_eq!(
            rug_hamming_distance(&rug::Integer::from(&x), &rug::Integer::from(&y)),
            distance
        );
        assert_eq!(y.hamming_distance(&x), distance);
        assert_eq!(natural_hamming_distance_alt_1(&x, &y), distance);
        assert_eq!(natural_hamming_distance_alt_2(&x, &y), distance);
        assert_eq!(distance == 0, x == y);
        assert_eq!((&x ^ &y).count_ones(), distance);
        assert_eq!((!x).checked_hamming_distance(&!y), Some(distance));
    });

    natural_triple_gen().test_properties(|(x, y, z)| {
        assert!(x.hamming_distance(&z) <= x.hamming_distance(&y) + y.hamming_distance(&z));
    });

    natural_gen().test_properties(|n| {
        assert_eq!(n.hamming_distance(&n), 0);
        assert_eq!(n.hamming_distance(&Natural::ZERO), n.count_ones());
        assert_eq!(Natural::ZERO.hamming_distance(&n), n.count_ones());
    });

    unsigned_pair_gen_var_27::<Limb>().test_properties(|(x, y)| {
        assert_eq!(
            Natural::from(x).hamming_distance(&Natural::from(y)),
            x.hamming_distance(y)
        );
    });
}
