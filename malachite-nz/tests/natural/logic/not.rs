// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{unsigned_vec_gen, unsigned_vec_pair_gen_var_1};
use malachite_nz::integer::Integer;
use malachite_nz::natural::logic::not::{limbs_not, limbs_not_in_place, limbs_not_to_out};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_gen;
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_not_and_limbs_not_in_place() {
    let test = |xs: &[Limb], out: &[Limb]| {
        assert_eq!(limbs_not(xs), out);

        let mut mut_xs = xs.to_vec();
        limbs_not_in_place(&mut mut_xs);
        assert_eq!(mut_xs, out);
    };
    test(&[], &[]);
    test(&[0, 1, 2], &[u32::MAX, u32::MAX - 1, u32::MAX - 2]);
    test(&[u32::MAX, u32::MAX - 1, u32::MAX - 2], &[0, 1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_not_to_out() {
    let test = |xs: &[Limb], out_before: &[Limb], out_after: &[Limb]| {
        let mut mut_out = out_before.to_vec();
        limbs_not_to_out(&mut mut_out, xs);
        assert_eq!(mut_out, out_after);
    };
    test(&[], &[], &[]);
    test(&[0x11111111], &[5], &[0xeeeeeeee]);
    test(
        &[0xffff0000, 0xf0f0f0f0],
        &[0, 1, 2],
        &[0xffff, 0xf0f0f0f, 2],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_not_to_out_fail() {
    let mut out = vec![1, 2];
    limbs_not_to_out(&mut out, &[1, 2, 3]);
}

#[test]
fn test_not() {
    let test = |s, out| {
        let not = !Natural::from_str(s).unwrap();
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        let not = !(&Natural::from_str(s).unwrap());
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        assert_eq!((!rug::Integer::from_str(s).unwrap()).to_string(), out);
    };
    test("0", "-1");
    test("123", "-124");
    test("1000000000000", "-1000000000001");
    test("2147483647", "-2147483648");
}

#[test]
fn limbs_not_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen().test_properties_with_config(&config, |xs| {
        let not_xs = limbs_not(&xs);
        assert_eq!(limbs_not(&not_xs), xs);
    });
}

#[test]
fn limbs_not_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_1().test_properties_with_config(&config, |(mut out, xs)| {
        let out_old = out.clone();
        limbs_not_to_out(&mut out, &xs);
        limbs_not_in_place(&mut out[..xs.len()]);
        assert_eq!(&out[..xs.len()], xs);
        assert_eq!(&out[xs.len()..], &out_old[xs.len()..]);
    });
}

#[test]
fn limbs_not_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen().test_properties_with_config(&config, |mut xs| {
        let xs_old = xs.clone();
        limbs_not_in_place(&mut xs);
        limbs_not_in_place(&mut xs);
        assert_eq!(xs, xs_old);
    });
}

#[test]
fn not_properties() {
    natural_gen().test_properties(|x| {
        let not = !x.clone();
        assert!(not.is_valid());

        let rug_not = !rug::Integer::from(&x);
        assert_eq!(Integer::from(&rug_not), not);

        let not_alt = !&x;
        assert!(not_alt.is_valid());
        assert_eq!(not_alt, not);

        assert!(not < 0);
        assert_eq!(not, -(&x + Natural::ONE));
        assert_ne!(not, x);
        assert_eq!(!not, x);
    });
}
