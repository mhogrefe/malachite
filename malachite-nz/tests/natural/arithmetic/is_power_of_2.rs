// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{unsigned_gen, unsigned_vec_gen_var_1};
use malachite_nz::natural::arithmetic::is_power_of_2::limbs_is_power_of_2;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_gen, natural_gen_var_2};
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_is_power_of_2() {
    let test = |xs, out| {
        assert_eq!(limbs_is_power_of_2(xs), out);
    };
    test(&[1], true);
    test(&[2], true);
    test(&[3], false);
    test(&[4], true);
    test(&[256], true);
    test(&[0, 0, 0, 256], true);
    test(&[1, 0, 0, 256], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_is_power_of_2_fail() {
    limbs_is_power_of_2(&[]);
}

#[test]
fn test_is_power_of_2() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().is_power_of_2(), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().is_power_of_two(), out);
    };
    test("0", false);
    test("1", true);
    test("2", true);
    test("3", false);
    test("4", true);
    test("5", false);
    test("6", false);
    test("7", false);
    test("8", true);
    test("1024", true);
    test("1025", false);
    test("1000000000000", false);
    test("1099511627776", true);
}

#[test]
fn limbs_is_power_of_2_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen_var_1().test_properties_with_config(&config, |xs| {
        assert_eq!(
            limbs_is_power_of_2(&xs),
            Natural::from_owned_limbs_asc(xs).is_power_of_2()
        );
    });
}

#[allow(clippy::cmp_owned, clippy::useless_conversion)]
#[test]
fn is_power_of_2_properties() {
    natural_gen().test_properties(|x| {
        let is_power_of_2 = x.is_power_of_2();
        assert_eq!(rug::Integer::from(&x).is_power_of_two(), is_power_of_2);
    });

    natural_gen_var_2().test_properties(|x| {
        let is_power_of_2 = x.is_power_of_2();
        let trailing_zeros = x.trailing_zeros().unwrap();
        assert_eq!(trailing_zeros == x.significant_bits() - 1, is_power_of_2);
        if trailing_zeros <= u64::from(Limb::MAX) {
            assert_eq!(x >> trailing_zeros == 1, is_power_of_2);
        }
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert_eq!(u.is_power_of_2(), Natural::from(u).is_power_of_2());
    });
}
