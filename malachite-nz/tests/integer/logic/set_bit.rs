// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::{BitAccess, NotAssign};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::unsigned_vec_unsigned_pair_gen_var_20;
use malachite_nz::integer::logic::bit_access::limbs_set_bit_neg;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::integer_unsigned_pair_gen_var_2;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_set_bit_neg() {
    let test = |xs: &[Limb], index: u64, out: &[Limb]| {
        let mut mut_xs = xs.to_vec();
        limbs_set_bit_neg(&mut mut_xs, index);
        assert_eq!(mut_xs, out);
    };
    test(&[3, 2, 1], 100, &[3, 2, 1]);
    test(&[0, 0, 0b1101, 0b11], 96, &[0, 0, 0b1101, 0b10]);
    test(&[0, 0, 0b1101, 0b11], 66, &[0, 0, 0b1001, 0b11]);
    test(&[0, 0, 0b1100, 0b11], 64, &[0, 0, 0b1011, 0b11]);
    test(&[0, 0, 0b1101, 0b11], 32, &[0, u32::MAX, 0b1100, 0b11]);
}

#[test]
fn test_set_bit() {
    let test = |u, index, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.set_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 10, "1024");
    test("100", 0, "101");
    test("1000000000000", 10, "1000000001024");
    test("1000000000000", 100, "1267650600228229402496703205376");
    test("5", 100, "1267650600228229401496703205381");
    test("-1", 5, "-1");
    test("-1", 100, "-1");
    test("-33", 5, "-1");
    test("-1267650600228229401496703205377", 100, "-1");
    test("-32", 0, "-31");
    test("-1000000000000", 10, "-999999998976");
    test("-1000000000000", 100, "-1000000000000");
    test("-1267650600228229402496703205376", 100, "-1000000000000");
    test("-18446744078004518912", 0, "-18446744078004518911");
    test("-18446744078004518912", 32, "-18446744078004518912");
    test("-18446744078004518912", 33, "-18446744078004518912");
    test("-18446744078004518912", 64, "-4294967296");
    test("-18446744078004518912", 65, "-18446744078004518912");
    test("-4294967296", 0, "-4294967295");
}

#[test]
fn limbs_set_bit_neg_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_20().test_properties_with_config(
        &config,
        |(mut xs, index)| {
            let mut n = -Natural::from_limbs_asc(&xs);
            limbs_set_bit_neg(&mut xs, index);
            n.set_bit(index);
            assert_eq!(-Natural::from_owned_limbs_asc(xs), n);
        },
    );
}

#[test]
fn set_bit_properties() {
    integer_unsigned_pair_gen_var_2().test_properties(|(n, index)| {
        let mut mut_n = n.clone();
        mut_n.set_bit(index);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let mut mut_n = n.clone();
        mut_n.assign_bit(index, true);
        assert_eq!(mut_n, result);

        assert_eq!(&n | Integer::power_of_2(index), result);

        assert_ne!(result, 0);
        assert!(result >= n);
        if n.get_bit(index) {
            assert_eq!(result, n);
        } else {
            assert_ne!(result, n);
            let mut mut_result = result.clone();
            mut_result.clear_bit(index);
            assert_eq!(mut_result, n);
        }

        let mut mut_not_n = !n;
        mut_not_n.clear_bit(index);
        mut_not_n.not_assign();
        assert_eq!(mut_not_n, result);
    });
}
