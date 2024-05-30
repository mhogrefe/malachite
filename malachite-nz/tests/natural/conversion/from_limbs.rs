// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::slices::slice_test_zero;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_from_limbs_asc() {
    let test = |xs: &[Limb], out| {
        let x = Natural::from_limbs_asc(xs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = Natural::from_owned_limbs_asc(xs.to_vec());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(&[], "0");
    test(&[0], "0");
    test(&[0, 0, 0], "0");
    test(&[123], "123");
    test(&[123, 0], "123");
    test(&[123, 0, 0, 0], "123");
    test(&[3567587328, 232], "1000000000000");
    test(&[3567587328, 232, 0], "1000000000000");
    test(&[1, 2, 3, 4, 5], "1701411834921604967429270619762735448065");
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_from_limbs_desc() {
    let test = |xs: Vec<Limb>, out| {
        let x = Natural::from_limbs_desc(&xs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = Natural::from_owned_limbs_desc(xs.to_vec());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(vec![], "0");
    test(vec![0], "0");
    test(vec![0, 0, 0], "0");
    test(vec![123], "123");
    test(vec![0, 123], "123");
    test(vec![0, 0, 0, 123], "123");
    test(vec![232, 3567587328], "1000000000000");
    test(vec![0, 232, 3567587328], "1000000000000");
    test(
        vec![5, 4, 3, 2, 1],
        "1701411834921604967429270619762735448065",
    );
}

#[test]
fn from_limbs_asc_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen().test_properties_with_config(&config, |xs| {
        let x = Natural::from_limbs_asc(&xs);
        assert!(x.is_valid());
        assert_eq!(Natural::from_owned_limbs_asc(xs.clone()), x);
        let mut trimmed_limbs = xs
            .iter()
            .copied()
            .rev()
            .skip_while(|&limb| limb == 0)
            .collect_vec();
        trimmed_limbs.reverse();
        assert_eq!(x.to_limbs_asc(), trimmed_limbs);
        assert_eq!(
            Natural::from_limbs_desc(&xs.iter().copied().rev().collect_vec()),
            x
        );
        if !xs.is_empty() && *xs.last().unwrap() != 0 {
            assert_eq!(x.to_limbs_asc(), xs);
        }
        assert_eq!(slice_test_zero(&xs), x == 0);
    });
}

#[test]
fn from_limbs_desc_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen().test_properties_with_config(&config, |xs| {
        let x = Natural::from_limbs_desc(&xs);
        assert!(x.is_valid());
        assert_eq!(Natural::from_owned_limbs_desc(xs.clone()), x);
        assert_eq!(
            x.to_limbs_desc(),
            xs.iter()
                .copied()
                .skip_while(|&limb| limb == 0)
                .collect_vec()
        );
        assert_eq!(
            Natural::from_limbs_asc(&xs.iter().copied().rev().collect_vec()),
            x
        );
        if !xs.is_empty() && xs[0] != 0 {
            assert_eq!(x.to_limbs_desc(), xs);
        }
        assert_eq!(slice_test_zero(&xs), x == 0);
    });
}
