// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::num::logic::traits::BitConvertible;
use malachite_base::test_util::generators::bool_vec_gen;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::num::logic::bit_convertible::{
    from_bits_asc_alt, from_bits_desc_alt,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::natural::logic::from_bits::{
    from_bits_asc_naive, from_bits_desc_naive,
};

#[test]
fn test_from_bits_asc() {
    let test = |bits: &[bool], out| {
        let x = Natural::from_bits_asc(bits.iter().copied());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test(&[], "0");
    test(&[false], "0");
    test(&[false, false, false], "0");
    test(&[true], "1");
    test(&[false, true, true], "6");
    test(&[true, false, false, true, false, true, true], "105");
    test(&[true, false, false, true, false, true, true, false], "105");
    test(
        &[true, false, false, true, false, true, true, false, false, false],
        "105",
    );
    test(
        &[
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, false, false, false, true, false, true, false, false, true, false, true, false,
            false, true, false, true, false, true, true, false, false, false, true, false, true,
            true, true,
        ],
        "1000000000000",
    );
    test(
        &[
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, false, false, false, true, false, true, false, false, true, false, true, false,
            false, true, false, true, false, true, true, false, false, false, true, false, true,
            true, true, false,
        ],
        "1000000000000",
    );
}

#[test]
fn test_from_bits_desc() {
    let test = |bits: &[bool], out| {
        let x = Natural::from_bits_desc(bits.iter().copied());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test(&[], "0");
    test(&[false], "0");
    test(&[false, false, false], "0");
    test(&[true], "1");
    test(&[true, true, false], "6");
    test(&[true, true, false, true, false, false, true], "105");
    test(&[false, true, true, false, true, false, false, true], "105");
    test(
        &[false, false, false, true, true, false, true, false, false, true],
        "105",
    );
    test(
        &[
            true, true, true, false, true, false, false, false, true, true, false, true, false,
            true, false, false, true, false, true, false, false, true, false, true, false, false,
            false, true, false, false, false, false, false, false, false, false, false, false,
            false, false,
        ],
        "1000000000000",
    );
    test(
        &[
            false, true, true, true, false, true, false, false, false, true, true, false, true,
            false, true, false, false, true, false, true, false, false, true, false, true, false,
            false, false, true, false, false, false, false, false, false, false, false, false,
            false, false, false,
        ],
        "1000000000000",
    );
}

#[test]
fn from_bits_asc_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 1024);
    config.insert("mean_stripe_n", 512);
    bool_vec_gen().test_properties_with_config(&config, |bits| {
        let x = Natural::from_bits_asc(bits.iter().copied());
        assert!(x.is_valid());
        assert_eq!(from_bits_asc_naive(bits.iter().copied()), x);
        assert_eq!(from_bits_asc_alt::<Natural, _>(bits.iter().copied()), x);
        let mut trimmed_bits = bits
            .iter()
            .copied()
            .rev()
            .skip_while(|&bit| !bit)
            .collect_vec();
        trimmed_bits.reverse();
        assert_eq!(x.to_bits_asc(), trimmed_bits);
        assert_eq!(Natural::from_bits_desc(bits.iter().copied().rev()), x);
        if !bits.is_empty() && *bits.last().unwrap() {
            assert_eq!(x.to_bits_asc(), *bits);
        }
        assert_eq!(bits.iter().all(|b| !b), x == 0);
        if Limb::convertible_from(&x) {
            assert_eq!(Limb::from_bits_asc(bits.into_iter()), x);
        }
    });
}

#[test]
fn from_bits_desc_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 1024);
    config.insert("mean_stripe_n", 512);
    bool_vec_gen().test_properties_with_config(&config, |bits| {
        let x = Natural::from_bits_desc(bits.iter().copied());
        assert!(x.is_valid());
        assert_eq!(from_bits_desc_naive(bits.iter().copied()), x);
        assert_eq!(from_bits_desc_alt::<Natural, _>(bits.iter().copied()), x);
        assert_eq!(
            x.to_bits_desc(),
            bits.iter().copied().skip_while(|&b| !b).collect_vec()
        );
        assert_eq!(Natural::from_bits_asc(bits.iter().copied().rev()), x);
        if !bits.is_empty() && bits[0] {
            assert_eq!(x.to_bits_desc(), *bits);
        }
        assert_eq!(bits.iter().all(|b| !b), x == 0);
        if Limb::convertible_from(&x) {
            assert_eq!(Limb::from_bits_desc(bits.iter().copied()), x);
        }
    });
}
