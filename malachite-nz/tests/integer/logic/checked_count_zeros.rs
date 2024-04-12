// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::CountZeros;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    signed_gen_var_4, unsigned_vec_gen_var_2, unsigned_vec_gen_var_4,
};
use malachite_nz::integer::logic::checked_count_zeros::limbs_count_zeros_neg;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::integer_gen;
use malachite_nz::test_util::integer::logic::checked_count_zeros::{
    integer_checked_count_zeros_alt_1, integer_checked_count_zeros_alt_2,
};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_count_zeros_neg() {
    let test = |xs, out| {
        assert_eq!(limbs_count_zeros_neg(xs), out);
    };
    test(&[0, 1, 2], 33);
    test(&[1, u32::MAX], 32);
}

#[test]
fn test_checked_count_zeros() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();

        assert_eq!(u.checked_count_zeros(), out);
        assert_eq!(integer_checked_count_zeros_alt_1(&u), out);
        assert_eq!(integer_checked_count_zeros_alt_2(&u), out);
    };
    test("0", None);
    test("105", None);
    test("-105", Some(3));
    test("1000000000000", None);
    test("-1000000000000", Some(24));
    test("4294967295", None);
    test("-4294967295", Some(31));
    test("4294967296", None);
    test("-4294967296", Some(32));
    test("18446744073709551615", None);
    test("-18446744073709551615", Some(63));
    test("18446744073709551616", None);
    test("-18446744073709551616", Some(64));
}

#[test]
fn limbs_count_zeros_neg_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen_var_4().test_properties_with_config(&config, |xs| {
        limbs_count_zeros_neg(&xs);
    });

    unsigned_vec_gen_var_2().test_properties_with_config(&config, |xs| {
        assert_eq!(
            Some(limbs_count_zeros_neg(&xs)),
            (-Natural::from_owned_limbs_asc(xs)).checked_count_zeros()
        );
    });
}

#[test]
fn checked_count_zeros_properties() {
    integer_gen().test_properties(|x| {
        let zeros = x.checked_count_zeros();
        assert_eq!(integer_checked_count_zeros_alt_1(&x), zeros);
        assert_eq!(integer_checked_count_zeros_alt_2(&x), zeros);
        assert_eq!((!x).checked_count_ones(), zeros);
    });

    signed_gen_var_4::<SignedLimb>().test_properties(|i| {
        assert_eq!(
            Integer::from(i).checked_count_zeros(),
            Some(CountZeros::count_zeros(i))
        );
    });
}
