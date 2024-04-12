// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeOne, Zero};
use malachite_base::num::logic::traits::{CheckedHammingDistance, HammingDistance};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    signed_pair_gen, unsigned_vec_pair_gen_var_8, unsigned_vec_unsigned_pair_gen_var_19,
};
use malachite_nz::integer::logic::checked_hamming_distance::{
    limbs_hamming_distance_limb_neg, limbs_hamming_distance_neg,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{
    integer_gen, integer_pair_gen, integer_triple_gen_var_1, natural_pair_gen,
};
use malachite_nz::test_util::integer::logic::checked_hamming_distance::{
    integer_checked_hamming_distance_alt_1, integer_checked_hamming_distance_alt_2,
    rug_checked_hamming_distance,
};
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_hamming_distance_limb_neg() {
    let test = |xs, y, out| {
        assert_eq!(limbs_hamming_distance_limb_neg(xs, y), out);
    };
    test(&[2], 2, 0);
    test(&[1, 1, 1], 1, 2);
    test(&[1, 1, 1], 2, 3);
    test(&[1, 2, 3], 3, 4);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_hamming_distance_limb_neg_fail() {
    limbs_hamming_distance_limb_neg(&[], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_hamming_distance_neg() {
    let test = |xs, ys, out| {
        assert_eq!(limbs_hamming_distance_neg(xs, ys), out);
    };
    test(&[2], &[3], 2);
    test(&[1, 1, 1], &[1, 2, 3], 3);
    test(&[1, 1, 1], &[1, 2, 3, 4], 4);
    test(&[1, 2, 3, 4], &[1, 1, 1], 4);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_hamming_distance_neg_fail_1() {
    limbs_hamming_distance_neg(&[0, 0], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_hamming_distance_neg_fail_2() {
    limbs_hamming_distance_neg(&[1, 2, 3], &[0, 0]);
}

#[test]
fn test_checked_hamming_distance() {
    let test = |s, t, out| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        assert_eq!(u.checked_hamming_distance(&v), out);
        assert_eq!(integer_checked_hamming_distance_alt_1(&u, &v), out);
        assert_eq!(integer_checked_hamming_distance_alt_2(&u, &v), out);
        assert_eq!(
            rug_checked_hamming_distance(
                &rug::Integer::from_str(s).unwrap(),
                &rug::Integer::from_str(t).unwrap(),
            ),
            out
        );
    };
    test("105", "123", Some(2));
    test("1000000000000", "0", Some(13));
    test("4294967295", "0", Some(32));
    test("4294967295", "4294967295", Some(0));
    test("4294967295", "4294967296", Some(33));
    test("1000000000000", "1000000000001", Some(1));
    test("-105", "-123", Some(2));
    test("-1000000000000", "-1", Some(24));
    test("-4294967295", "-1", Some(31));
    test("-4294967295", "-4294967295", Some(0));
    test("-4294967295", "-4294967296", Some(1));
    test("-1000000000000", "-1000000000001", Some(13));
    test("-105", "123", None);
    test("-1000000000000", "0", None);
    test("-4294967295", "0", None);
    test("-4294967295", "4294967295", None);
    test("-4294967295", "4294967296", None);
    test("-1000000000000", "1000000000001", None);
    test("105", "-123", None);
    test("1000000000000", "-1", None);
    test("4294967295", "-1", None);
    test("4294967295", "-4294967295", None);
    test("4294967295", "-4294967296", None);
    test("1000000000000", "-1000000000001", None);
}

#[test]
fn limbs_hamming_distance_limb_neg_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_19().test_properties_with_config(&config, |(xs, y)| {
        assert_eq!(
            Some(limbs_hamming_distance_limb_neg(&xs, y)),
            (-Natural::from_owned_limbs_asc(xs)).checked_hamming_distance(&-Natural::from(y)),
        );
    });
}

#[test]
fn limbs_hamming_distance_neg_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(xs, ys)| {
        assert_eq!(
            Some(limbs_hamming_distance_neg(&xs, &ys)),
            (-Natural::from_owned_limbs_asc(xs))
                .checked_hamming_distance(&-Natural::from_owned_limbs_asc(ys)),
        );
    });
}

#[test]
fn checked_hamming_distance_properties() {
    integer_pair_gen().test_properties(|(x, y)| {
        let distance = x.checked_hamming_distance(&y);
        assert_eq!(
            rug_checked_hamming_distance(&rug::Integer::from(&x), &rug::Integer::from(&y)),
            distance
        );
        assert_eq!(y.checked_hamming_distance(&x), distance);
        assert_eq!(integer_checked_hamming_distance_alt_1(&x, &y), distance);
        assert_eq!(integer_checked_hamming_distance_alt_2(&x, &y), distance);
        assert_eq!(distance == Some(0), x == y);
        assert_eq!((&x ^ &y).checked_count_ones(), distance);
        assert_eq!((!x).checked_hamming_distance(&!y), distance);
    });

    integer_triple_gen_var_1().test_properties(|(x, y, z)| {
        assert!(
            x.checked_hamming_distance(&z).unwrap()
                <= x.checked_hamming_distance(&y).unwrap()
                    + y.checked_hamming_distance(&z).unwrap()
        );
        let x = !x;
        let y = !y;
        let z = !z;
        assert!(
            x.checked_hamming_distance(&z).unwrap()
                <= x.checked_hamming_distance(&y).unwrap()
                    + y.checked_hamming_distance(&z).unwrap()
        );
    });

    integer_gen().test_properties(|n| {
        assert_eq!(n.checked_hamming_distance(&n), Some(0));
        assert_eq!(
            n.checked_hamming_distance(&Integer::ZERO),
            n.checked_count_ones()
        );
        assert_eq!(
            n.checked_hamming_distance(&Integer::NEGATIVE_ONE),
            n.checked_count_zeros()
        );
        assert_eq!(
            Integer::ZERO.checked_hamming_distance(&n),
            n.checked_count_ones()
        );
        assert_eq!(
            Integer::NEGATIVE_ONE.checked_hamming_distance(&n),
            n.checked_count_zeros()
        );
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(
            Some(x.hamming_distance(&y)),
            Integer::from(x).checked_hamming_distance(&Integer::from(y)),
        );
    });

    signed_pair_gen::<SignedLimb>().test_properties(|(x, y)| {
        assert_eq!(
            Integer::from(x).checked_hamming_distance(&Integer::from(y)),
            x.checked_hamming_distance(y)
        );
    });
}
