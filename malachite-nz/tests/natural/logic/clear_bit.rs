// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_2, unsigned_vec_unsigned_pair_gen_var_16,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::logic::bit_access::limbs_clear_bit;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_4;
#[cfg(feature = "32_bit_limbs")]
use rug;
#[cfg(feature = "32_bit_limbs")]
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_clear_bit() {
    let test = |xs: &[Limb], index: u64, out: &[Limb]| {
        let mut xs = xs.to_vec();
        limbs_clear_bit(&mut xs, index);
        assert_eq!(xs, out);
    };
    test(&[3, 3], 33, &[3, 1]);
    test(&[3, 1], 1, &[1, 1]);
    test(&[3, 3], 100, &[3, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_clear_bit() {
    let test = |u, index, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.clear_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n.set_bit(u32::exact_from(index), false);
        assert_eq!(n.to_string(), out);
    };
    test("0", 10, "0");
    test("0", 100, "0");
    test("1024", 10, "0");
    test("101", 0, "100");
    test("1000000001024", 10, "1000000000000");
    test("1000000001024", 100, "1000000001024");
    test("1267650600228229402496703205376", 100, "1000000000000");
    test("1267650600228229401496703205381", 100, "5");
}

#[test]
fn limbs_clear_bit_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(&config, |(xs, index)| {
        let mut mut_xs = xs.clone();
        let mut n = Natural::from_limbs_asc(&xs);
        limbs_clear_bit(&mut mut_xs, index);
        n.clear_bit(index);
        assert_eq!(Natural::from_limbs_asc(&mut_xs), n);
    });
}

#[test]
fn clear_bit_properties() {
    natural_unsigned_pair_gen_var_4().test_properties(|(n, index)| {
        let mut mut_n = n.clone();
        mut_n.clear_bit(index);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let mut rug_n = rug::Integer::from(&n);
        rug_n.set_bit(u32::exact_from(index), false);
        assert_eq!(Natural::exact_from(&rug_n), result);

        let mut mut_n = n.clone();
        mut_n.assign_bit(index, false);
        assert_eq!(mut_n, result);

        assert_eq!(Integer::from(&n) & !Natural::power_of_2(index), result);

        assert!(result <= n);
        if n.get_bit(index) {
            assert_ne!(result, n);
            let mut mut_result = result;
            mut_result.set_bit(index);
            assert_eq!(mut_result, n);
        } else {
            assert_eq!(result, n);
        }
    });

    unsigned_pair_gen_var_2::<Limb, u64>().test_properties(|(u, index)| {
        let mut mut_u = u;
        mut_u.clear_bit(index);
        let mut n = Natural::from(u);
        n.clear_bit(index);
        assert_eq!(n, mut_u);
    });
}
