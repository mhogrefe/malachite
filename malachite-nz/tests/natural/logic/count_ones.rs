// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::CountOnes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{unsigned_gen, unsigned_vec_gen};
use malachite_nz::natural::logic::count_ones::limbs_count_ones;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_gen;
use malachite_nz::test_util::natural::logic::count_ones::{
    natural_count_ones_alt_1, natural_count_ones_alt_2,
};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_count_ones() {
    let test = |xs, out| {
        assert_eq!(limbs_count_ones(xs), out);
    };
    test(&[], 0);
    test(&[0, 1, 2], 2);
    test(&[1, u32::MAX], 33);
}

#[test]
fn test_count_ones() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().count_ones(), out);
        assert_eq!(
            natural_count_ones_alt_1(&Natural::from_str(n).unwrap()),
            out
        );
        assert_eq!(
            natural_count_ones_alt_2(&Natural::from_str(n).unwrap()),
            out
        );
    };
    test("0", 0);
    test("105", 4);
    test("1000000000000", 13);
    test("4294967295", 32);
    test("4294967296", 1);
    test("18446744073709551615", 64);
    test("18446744073709551616", 1);
}

#[test]
fn limbs_count_ones_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen().test_properties_with_config(&config, |xs| {
        assert_eq!(
            limbs_count_ones(&xs),
            Natural::from_owned_limbs_asc(xs).count_ones()
        );
    });
}

#[test]
fn count_ones_properties() {
    natural_gen().test_properties(|x| {
        let ones = x.count_ones();
        assert_eq!(natural_count_ones_alt_1(&x), ones);
        assert_eq!(natural_count_ones_alt_2(&x), ones);
        assert_eq!(ones == 0, x == 0);
        assert_eq!(ones == 1, x.is_power_of_2());
        assert_eq!((!x).checked_count_zeros(), Some(ones));
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert_eq!(Natural::from(u).count_ones(), CountOnes::count_ones(u));
    });
}
