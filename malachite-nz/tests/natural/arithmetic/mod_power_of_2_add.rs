// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModAdd, ModPowerOf2, ModPowerOf2Add, ModPowerOf2AddAssign, ModPowerOf2IsReduced,
    ModPowerOf2Neg, ModPowerOf2Shl, ModPowerOf2Sub, PowerOf2,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::unsigned_triple_gen_var_11;
use malachite_nz::natural::arithmetic::mod_power_of_2_add::{
    limbs_mod_power_of_2_add, limbs_mod_power_of_2_add_greater,
    limbs_mod_power_of_2_add_in_place_either, limbs_mod_power_of_2_add_limb,
    limbs_slice_mod_power_of_2_add_greater_in_place_left,
    limbs_slice_mod_power_of_2_add_limb_in_place, limbs_vec_mod_power_of_2_add_in_place_left,
    limbs_vec_mod_power_of_2_add_limb_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_natural_natural_unsigned_quadruple_gen_var_2,
    natural_natural_unsigned_triple_gen_var_4, natural_unsigned_pair_gen_var_11,
    unsigned_vec_unsigned_unsigned_triple_gen_var_14,
    unsigned_vec_unsigned_unsigned_triple_gen_var_15,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_19,
};
use std::panic::catch_unwind;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_2_add_limb() {
    let test = |xs: &[Limb], y: Limb, pow: u64, out: &[Limb]| {
        assert_eq!(limbs_mod_power_of_2_add_limb(xs, y, pow), out);
    };
    test(&[], 0, 0, &[]);
    test(&[], 0, 5, &[]);
    test(&[], 5, 3, &[5]);
    test(&[123, 456], 789, 41, &[912, 456]);
    test(&[u32::MAX], 2, 33, &[1, 1]);
    test(&[u32::MAX], 2, 32, &[1]);
    test(&[u32::MAX, 3], 2, 34, &[1, 0]);
    test(&[u32::MAX, 3], 2, 35, &[1, 4]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_mod_power_of_2_add_limb_in_place() {
    let test = |xs: &[Limb], y: Limb, pow: u64, out: &[Limb], carry: bool| {
        let mut xs = xs.to_vec();
        assert_eq!(
            limbs_slice_mod_power_of_2_add_limb_in_place(&mut xs, y, pow),
            carry
        );
        assert_eq!(xs, out);
    };
    test(&[], 0, 0, &[], false);
    test(&[], 0, 5, &[], false);
    test(&[], 5, 3, &[], true);
    test(&[123, 456], 789, 41, &[912, 456], false);
    test(&[u32::MAX], 2, 33, &[1], true);
    test(&[u32::MAX], 2, 32, &[1], false);
    test(&[u32::MAX, 3], 2, 34, &[1, 0], false);
    test(&[u32::MAX, 3], 2, 35, &[1, 4], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_vec_mod_power_of_2_add_limb_in_place() {
    let test = |xs: &[Limb], y: Limb, pow: u64, out: &[Limb]| {
        let mut xs = xs.to_vec();
        limbs_vec_mod_power_of_2_add_limb_in_place(&mut xs, y, pow);
        assert_eq!(xs, out);
    };
    test(&[123, 456], 789, 41, &[912, 456]);
    test(&[u32::MAX], 2, 33, &[1, 1]);
    test(&[u32::MAX], 2, 32, &[1]);
    test(&[u32::MAX, 3], 2, 34, &[1, 0]);
    test(&[u32::MAX, 3], 2, 35, &[1, 4]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_mod_power_of_2_add_limb_in_place_fail() {
    limbs_vec_mod_power_of_2_add_limb_in_place(&mut vec![], 10, 4);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_2_add_greater() {
    let test = |xs, ys, pow, out: &[Limb]| {
        assert_eq!(limbs_mod_power_of_2_add_greater(xs, ys, pow), out);
    };
    test(&[], &[], 0, &[]);
    test(&[], &[], 5, &[]);
    test(&[2], &[], 3, &[2]);
    test(&[2], &[3], 2, &[1]);
    test(&[1, 2, 3], &[6, 7], 100, &[7, 9, 3]);
    test(&[100, 101, u32::MAX], &[102, 101, 2], 97, &[202, 202, 1, 1]);
    test(&[100, 101, u32::MAX], &[102, 101, 2], 96, &[202, 202, 1]);
    test(&[u32::MAX], &[2], 33, &[1, 1]);
    test(&[u32::MAX], &[2], 32, &[1]);
    test(&[u32::MAX, 3], &[2], 34, &[1, 0]);
    test(&[u32::MAX, 3], &[2], 35, &[1, 4]);
    test(&[u32::MAX, u32::MAX], &[2], 65, &[1, 0, 1]);
    test(&[u32::MAX, u32::MAX], &[2], 64, &[1, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn test_limbs_mod_power_of_2_add_greater_fail() {
    limbs_mod_power_of_2_add_greater(&[6, 7], &[1, 2, 3], 4);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_2_add_and_limbs_vec_mod_power_of_2_add_in_place_left() {
    let test = |xs_before, ys, pow, out: &[Limb]| {
        assert_eq!(limbs_mod_power_of_2_add(xs_before, ys, pow), out);

        let mut xs = xs_before.to_vec();
        limbs_vec_mod_power_of_2_add_in_place_left(&mut xs, ys, pow);
        assert_eq!(xs, out);
    };
    test(&[], &[], 0, &[]);
    test(&[], &[], 5, &[]);
    test(&[2], &[], 3, &[2]);
    test(&[], &[2], 3, &[2]);
    test(&[2], &[3], 2, &[1]);
    test(&[1, 2, 3], &[6, 7], 100, &[7, 9, 3]);
    test(&[6, 7], &[1, 2, 3], 100, &[7, 9, 3]);
    test(&[100, 101, u32::MAX], &[102, 101, 2], 97, &[202, 202, 1, 1]);
    test(&[100, 101, u32::MAX], &[102, 101, 2], 96, &[202, 202, 1]);
    test(&[u32::MAX], &[2], 33, &[1, 1]);
    test(&[u32::MAX], &[2], 32, &[1]);
    test(&[u32::MAX, 3], &[2], 34, &[1, 0]);
    test(&[u32::MAX, 3], &[2], 35, &[1, 4]);
    test(&[u32::MAX, u32::MAX], &[2], 65, &[1, 0, 1]);
    test(&[u32::MAX, u32::MAX], &[2], 64, &[1, 0]);
    test(&[2], &[u32::MAX, u32::MAX], 65, &[1, 0, 1]);
    test(&[2], &[u32::MAX, u32::MAX], 64, &[1, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_mod_power_of_2_add_greater_in_place_left() {
    let test = |xs_before: &[Limb], ys, pow, xs_after: &[Limb], carry| {
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_slice_mod_power_of_2_add_greater_in_place_left(&mut xs, ys, pow),
            carry
        );
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], 0, &[], false);
    test(&[], &[], 5, &[], false);
    test(&[2], &[], 3, &[2], false);
    test(&[2], &[3], 2, &[1], false);
    test(&[1, 2, 3], &[6, 7], 100, &[7, 9, 3], false);
    test(
        &[100, 101, u32::MAX],
        &[102, 101, 2],
        97,
        &[202, 202, 1],
        true,
    );
    test(
        &[100, 101, u32::MAX],
        &[102, 101, 2],
        96,
        &[202, 202, 1],
        false,
    );
    test(&[u32::MAX], &[2], 33, &[1], true);
    test(&[u32::MAX], &[2], 32, &[1], false);
    test(&[u32::MAX, 3], &[2], 34, &[1, 0], false);
    test(&[u32::MAX, 3], &[2], 35, &[1, 4], false);
    test(&[u32::MAX, u32::MAX], &[2], 65, &[1, 0], true);
    test(&[u32::MAX, u32::MAX], &[2], 64, &[1, 0], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_mod_power_of_2_add_greater_in_place_left_fail() {
    let mut xs = vec![6, 7];
    limbs_slice_mod_power_of_2_add_greater_in_place_left(&mut xs, &[1, 2, 3], 4);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_2_add_in_place_either() {
    let test = |xs_before: &[Limb],
                ys_before: &[Limb],
                pow,
                right,
                xs_after: &[Limb],
                ys_after: &[Limb]| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_mod_power_of_2_add_in_place_either(&mut xs, &mut ys, pow),
            right
        );
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[], &[], 0, false, &[], &[]);
    test(&[], &[], 5, false, &[], &[]);
    test(&[2], &[], 3, false, &[2], &[]);
    test(&[], &[2], 3, true, &[], &[2]);
    test(&[2], &[3], 2, false, &[1], &[3]);
    test(&[1, 2, 3], &[6, 7], 100, false, &[7, 9, 3], &[6, 7]);
    test(&[6, 7], &[1, 2, 3], 100, true, &[6, 7], &[7, 9, 3]);
    test(
        &[100, 101, u32::MAX],
        &[102, 101, 2],
        97,
        false,
        &[202, 202, 1, 1],
        &[102, 101, 2],
    );
    test(
        &[100, 101, u32::MAX],
        &[102, 101, 2],
        96,
        false,
        &[202, 202, 1],
        &[102, 101, 2],
    );
    test(&[u32::MAX], &[2], 33, false, &[1, 1], &[2]);
    test(&[u32::MAX], &[2], 32, false, &[1], &[2]);
    test(&[u32::MAX, 3], &[2], 34, false, &[1, 0], &[2]);
    test(&[u32::MAX, 3], &[2], 35, false, &[1, 4], &[2]);
    test(&[u32::MAX, u32::MAX], &[2], 65, false, &[1, 0, 1], &[2]);
    test(&[u32::MAX, u32::MAX], &[2], 64, false, &[1, 0], &[2]);
    test(&[2], &[u32::MAX, u32::MAX], 65, true, &[2], &[1, 0, 1]);
    test(&[2], &[u32::MAX, u32::MAX], 64, true, &[2], &[1, 0]);
}

#[test]
fn test_mod_power_of_2_add() {
    let test = |s, t, pow, out| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        assert!(u.mod_power_of_2_is_reduced(pow));
        assert!(v.mod_power_of_2_is_reduced(pow));

        let mut n = u.clone();
        n.mod_power_of_2_add_assign(v.clone(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_power_of_2_is_reduced(pow));

        let mut n = u.clone();
        n.mod_power_of_2_add_assign(&v, pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_power_of_2_add(v.clone(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_power_of_2_add(v.clone(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_power_of_2_add(&v, pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_power_of_2_add(&v, pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", 0, "0");
    test("0", "0", 5, "0");
    test("0", "2", 5, "2");
    test("10", "14", 4, "8");
    test("0", "123", 7, "123");
    test("123", "0", 7, "123");
    test("123", "456", 9, "67");
    test("1267650600228229401496703205375", "3", 100, "2");
    test("3", "1267650600228229401496703205375", 100, "2");
}

#[test]
fn mod_power_of_2_add_fail() {
    assert_panic!(Natural::ZERO.mod_power_of_2_add(Natural::ONE, 0));
    assert_panic!(Natural::ONE.mod_power_of_2_add(Natural::ZERO, 0));

    assert_panic!(Natural::ZERO.mod_power_of_2_add(&Natural::ONE, 0));
    assert_panic!(Natural::ONE.mod_power_of_2_add(&Natural::ZERO, 0));

    assert_panic!((&Natural::ZERO).mod_power_of_2_add(Natural::ONE, 0));
    assert_panic!((&Natural::ONE).mod_power_of_2_add(Natural::ZERO, 0));

    assert_panic!((&Natural::ZERO).mod_power_of_2_add(Natural::ONE, 0));
    assert_panic!((&Natural::ONE).mod_power_of_2_add(Natural::ZERO, 0));

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_power_of_2_add_assign(Natural::ONE, 0);
    });
    assert_panic!({
        let mut x = Natural::ONE;
        x.mod_power_of_2_add_assign(Natural::ZERO, 0);
    });

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_power_of_2_add_assign(&Natural::ONE, 0);
    });
    assert_panic!({
        let mut x = Natural::ONE;
        x.mod_power_of_2_add_assign(&Natural::ZERO, 0);
    });
}

#[test]
fn limbs_mod_power_of_2_add_limb_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_unsigned_triple_gen_var_14().test_properties_with_config(
        &config,
        |(xs, y, pow)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_mod_power_of_2_add_limb(&xs, y, pow)),
                Natural::from_owned_limbs_asc(xs).mod_power_of_2_add(Natural::from(y), pow),
            );
        },
    );
}

#[test]
fn limbs_slice_mod_power_of_2_add_limb_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_unsigned_triple_gen_var_14().test_properties_with_config(
        &config,
        |(mut xs, y, pow)| {
            let old_xs = xs.clone();
            let carry = limbs_slice_mod_power_of_2_add_limb_in_place(&mut xs, y, pow);
            let n = Natural::from_limbs_asc(&old_xs).mod_power_of_2_add(Natural::from(y), pow);
            let mut expected_limbs = n.into_limbs_asc();
            assert_eq!(carry, expected_limbs.len() == xs.len() + 1);
            expected_limbs.resize(xs.len(), 0);
            assert_eq!(xs, expected_limbs);
        },
    );
}

#[test]
fn limbs_vec_mod_power_of_2_add_limb_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_unsigned_triple_gen_var_15().test_properties_with_config(
        &config,
        |(mut xs, y, pow)| {
            let old_xs = xs.clone();
            limbs_vec_mod_power_of_2_add_limb_in_place(&mut xs, y, pow);
            let n = Natural::from_owned_limbs_asc(old_xs).mod_power_of_2_add(Natural::from(y), pow);
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
        },
    );
}

fn limbs_mod_power_of_2_add_helper(
    f: &dyn Fn(&[Limb], &[Limb], u64) -> Vec<Limb>,
    xs: Vec<Limb>,
    ys: Vec<Limb>,
    pow: u64,
) {
    assert_eq!(
        Natural::from_owned_limbs_asc(f(&xs, &ys, pow)),
        Natural::from_owned_limbs_asc(xs)
            .mod_power_of_2_add(Natural::from_owned_limbs_asc(ys), pow)
    );
}

#[test]
fn limbs_mod_power_of_2_add_greater_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_19().test_properties_with_config(
        &config,
        |(xs, ys, pow)| {
            limbs_mod_power_of_2_add_helper(&limbs_mod_power_of_2_add_greater, xs, ys, pow);
        },
    );
}

#[test]
fn limbs_mod_power_of_2_add_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18().test_properties_with_config(
        &config,
        |(xs, ys, pow)| {
            limbs_mod_power_of_2_add_helper(&limbs_mod_power_of_2_add, xs, ys, pow);
        },
    );
}

#[test]
fn limbs_slice_mod_power_of_2_add_greater_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_19().test_properties_with_config(
        &config,
        |(mut xs, ys, pow)| {
            let xs_old = xs.clone();
            let carry = limbs_slice_mod_power_of_2_add_greater_in_place_left(&mut xs, &ys, pow);
            let n = Natural::from_owned_limbs_asc(xs_old)
                .mod_power_of_2_add(Natural::from_owned_limbs_asc(ys), pow);
            let len = xs.len();
            let mut limbs = n.into_limbs_asc();
            assert_eq!(carry, limbs.len() == len + 1);
            limbs.resize(len, 0);
            assert_eq!(limbs, xs);
        },
    );
}

#[test]
fn limbs_vec_mod_power_of_2_add_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18().test_properties_with_config(
        &config,
        |(mut xs, ys, pow)| {
            let xs_old = xs.clone();
            limbs_vec_mod_power_of_2_add_in_place_left(&mut xs, &ys, pow);
            assert_eq!(
                Natural::from_owned_limbs_asc(xs),
                Natural::from_owned_limbs_asc(xs_old)
                    .mod_power_of_2_add(Natural::from_owned_limbs_asc(ys), pow)
            );
        },
    );
}

#[test]
fn limbs_mod_power_of_2_add_in_place_either_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18().test_properties_with_config(
        &config,
        |(mut xs, mut ys, pow)| {
            let xs_old = xs.clone();
            let ys_old = ys.clone();
            let right = limbs_mod_power_of_2_add_in_place_either(&mut xs, &mut ys, pow);
            let n = Natural::from_limbs_asc(&xs_old)
                .mod_power_of_2_add(Natural::from_limbs_asc(&ys_old), pow);
            if right {
                assert_eq!(xs, xs_old);
                assert_eq!(Natural::from_owned_limbs_asc(ys), n);
            } else {
                assert_eq!(Natural::from_owned_limbs_asc(xs), n);
                assert_eq!(ys, ys_old);
            }
        },
    );
}

#[test]
fn mod_power_of_2_add_properties() {
    natural_natural_unsigned_triple_gen_var_4().test_properties(|(x, y, pow)| {
        assert!(x.mod_power_of_2_is_reduced(pow));
        assert!(y.mod_power_of_2_is_reduced(pow));
        let sum_val_val = x.clone().mod_power_of_2_add(y.clone(), pow);
        let sum_val_ref = x.clone().mod_power_of_2_add(&y, pow);
        let sum_ref_val = (&x).mod_power_of_2_add(y.clone(), pow);
        let sum = (&x).mod_power_of_2_add(&y, pow);
        assert!(sum_val_val.is_valid());
        assert!(sum_val_ref.is_valid());
        assert!(sum_ref_val.is_valid());
        assert!(sum.is_valid());
        assert!(sum.mod_power_of_2_is_reduced(pow));
        assert_eq!(sum_val_val, sum);
        assert_eq!(sum_val_ref, sum);
        assert_eq!(sum_ref_val, sum);

        assert_eq!((&x + &y).mod_power_of_2(pow), sum);
        let mut sum_alt = &x + &y;
        sum_alt.clear_bit(pow);
        assert_eq!(sum_alt, sum);
        assert_eq!((&x).mod_add(&y, Natural::power_of_2(pow)), sum);

        let mut mut_x = x.clone();
        mut_x.mod_power_of_2_add_assign(y.clone(), pow);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, sum);
        let mut mut_x = x.clone();
        mut_x.mod_power_of_2_add_assign(&y, pow);
        assert_eq!(mut_x, sum);
        assert!(mut_x.is_valid());

        assert_eq!((&y).mod_power_of_2_add(&x, pow), sum);
        assert_eq!(
            (&x).mod_power_of_2_sub((&y).mod_power_of_2_neg(pow), pow),
            sum
        );
        assert_eq!((&sum).mod_power_of_2_sub(&x, pow), y);
        assert_eq!(sum.mod_power_of_2_sub(y, pow), x);
    });

    natural_unsigned_pair_gen_var_11().test_properties(|(x, pow)| {
        assert_eq!((&x).mod_power_of_2_add(Natural::ZERO, pow), x);
        assert_eq!(Natural::ZERO.mod_power_of_2_add(&x, pow), x);
        assert_eq!(
            (&x).mod_power_of_2_add(&x, pow),
            x.mod_power_of_2_shl(1, pow)
        );
    });

    natural_natural_natural_unsigned_quadruple_gen_var_2().test_properties(|(x, y, z, pow)| {
        assert_eq!(
            (&x).mod_power_of_2_add(&y, pow).mod_power_of_2_add(&z, pow),
            x.mod_power_of_2_add(y.mod_power_of_2_add(z, pow), pow)
        );
    });

    unsigned_triple_gen_var_11::<Limb>().test_properties(|(x, y, pow)| {
        assert_eq!(
            x.mod_power_of_2_add(y, pow),
            Natural::from(x).mod_power_of_2_add(Natural::from(y), pow)
        );
    });
}
