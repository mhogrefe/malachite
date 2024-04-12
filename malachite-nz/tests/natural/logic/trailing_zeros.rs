// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::unsigned_vec_gen_var_2;
use malachite_nz::natural::logic::trailing_zeros::limbs_trailing_zeros;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_gen;
use malachite_nz::test_util::natural::logic::trailing_zeros::natural_trailing_zeros_alt;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_trailing_zeros() {
    let test = |xs, out| {
        assert_eq!(limbs_trailing_zeros(xs), out);
    };
    test(&[4], 2);
    test(&[0, 4], 34);
    test(&[1, 2, 3], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_trailing_zeros_fail_1() {
    limbs_trailing_zeros(&[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_trailing_zeros_fail_2() {
    limbs_trailing_zeros(&[0, 0, 0]);
}

#[test]
fn test_trailing_zeros() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().trailing_zeros(), out);
        assert_eq!(
            natural_trailing_zeros_alt(&Natural::from_str(n).unwrap()),
            out
        );
    };
    test("0", None);
    test("123", Some(0));
    test("1000000000000", Some(12));
    test("4294967295", Some(0));
    test("4294967296", Some(32));
    test("18446744073709551615", Some(0));
    test("18446744073709551616", Some(64));
}

#[test]
fn limbs_trailing_zeros_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen_var_2().test_properties_with_config(&config, |xs| {
        assert_eq!(
            Some(limbs_trailing_zeros(&xs)),
            Natural::from_owned_limbs_asc(xs).trailing_zeros()
        );
    });
}

#[allow(clippy::cmp_owned, clippy::useless_conversion)]
#[test]
fn trailing_zeros_properties() {
    natural_gen().test_properties(|x| {
        let trailing_zeros = x.trailing_zeros();
        assert_eq!(natural_trailing_zeros_alt(&x), trailing_zeros);
        assert_eq!(trailing_zeros.is_none(), x == 0);
        if x != 0 {
            let trailing_zeros = trailing_zeros.unwrap();
            if trailing_zeros <= u64::from(Limb::MAX) {
                assert!((&x >> trailing_zeros).odd());
                assert_eq!(&x >> trailing_zeros << trailing_zeros, x);
            }
        }
    });
}
