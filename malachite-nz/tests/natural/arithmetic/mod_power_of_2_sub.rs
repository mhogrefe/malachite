// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Neg, ModPowerOf2Sub,
    ModPowerOf2SubAssign, ModSub, PowerOf2,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::unsigned_triple_gen_var_11;
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::mod_power_of_2_sub::{
    limbs_mod_power_of_2_limb_sub_limbs, limbs_mod_power_of_2_limb_sub_limbs_in_place,
    limbs_mod_power_of_2_sub, limbs_mod_power_of_2_sub_in_place_either,
    limbs_mod_power_of_2_sub_in_place_left, limbs_mod_power_of_2_sub_in_place_right,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_natural_unsigned_triple_gen_var_4, natural_unsigned_pair_gen_var_11,
    unsigned_vec_unsigned_unsigned_triple_gen_var_16,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_20,
};
use std::panic::catch_unwind;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_2_limb_sub_limbs_and_limbs_mod_power_of_2_limb_sub_limbs_in_place() {
    let test = |x: Limb, ys: &[Limb], pow: u64, out: &[Limb]| {
        assert_eq!(limbs_mod_power_of_2_limb_sub_limbs(x, ys, pow), out);

        let mut ys = ys.to_vec();
        limbs_mod_power_of_2_limb_sub_limbs_in_place(x, &mut ys, pow);
        assert_eq!(ys, out);
    };
    test(3, &[2], 4, &[1]);
    test(3, &[10], 4, &[9]);
    test(3, &[1, 2, 3], 70, &[2, u32::MAX - 1, 60]);
    test(
        3,
        &[1, 2, 3],
        200,
        &[2, u32::MAX - 1, 0xfffffffc, u32::MAX, u32::MAX, u32::MAX, 255],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_2_limb_sub_limbs_fail() {
    limbs_mod_power_of_2_limb_sub_limbs(3, &[10], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_2_limb_sub_limbs_in_place_fail() {
    let mut ys = vec![10];
    limbs_mod_power_of_2_limb_sub_limbs_in_place(3, &mut ys, 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_2_sub() {
    let test = |xs_before, ys, pow, out: &[Limb]| {
        assert_eq!(limbs_mod_power_of_2_sub(xs_before, ys, pow), out);

        let mut xs = xs_before.to_vec();
        limbs_mod_power_of_2_sub_in_place_left(&mut xs, ys, pow);
        assert_eq!(xs, out);
    };
    test(&[], &[], 0, &[]);
    test(&[], &[], 5, &[]);
    test(&[2], &[], 3, &[2]);
    test(&[], &[2], 3, &[6]);
    test(&[2], &[3], 2, &[3]);
    test(&[1, 2, 3], &[6, 7], 100, &[0xfffffffb, 0xfffffffa, 2]);
    test(&[6, 7], &[1, 2, 3], 100, &[5, 5, 0xfffffffd, 15]);
    test(&[6, 7], &[1, 2], 100, &[5, 5]);
    test(
        &[1, 2],
        &[6, 7],
        100,
        &[0xfffffffb, 0xfffffffa, u32::MAX, 15],
    );
    test(&[6, 7], &[2, 3, 0], 100, &[4, 4, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_2_sub_in_place_right() {
    let test = |xs, ys_before: &[Limb], pow, out: &[Limb]| {
        let mut ys = ys_before.to_vec();
        limbs_mod_power_of_2_sub_in_place_right(xs, &mut ys, pow);
        assert_eq!(ys, out);
    };
    test(&[], &[], 0, &[]);
    test(&[], &[], 5, &[]);
    test(&[2], &[], 3, &[2]);
    test(&[], &[2], 3, &[6]);
    test(&[2], &[3], 2, &[3]);
    test(&[1, 2, 3], &[6, 7], 100, &[0xfffffffb, 0xfffffffa, 2]);
    test(&[6, 7], &[1, 2, 3], 100, &[5, 5, 0xfffffffd, 15]);
    test(&[6, 7], &[1, 2], 100, &[5, 5]);
    test(
        &[1, 2],
        &[6, 7],
        100,
        &[0xfffffffb, 0xfffffffa, u32::MAX, 15],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_2_sub_in_place_either() {
    let test = |xs_before: &[Limb],
                ys_before: &[Limb],
                pow,
                right,
                xs_after: &[Limb],
                ys_after: &[Limb]| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_mod_power_of_2_sub_in_place_either(&mut xs, &mut ys, pow),
            right
        );
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[], &[], 0, false, &[], &[]);
    test(&[], &[], 5, false, &[], &[]);
    test(&[2], &[], 3, false, &[2], &[]);
    test(&[], &[2], 3, true, &[], &[6]);
    test(&[2], &[3], 2, false, &[3], &[3]);
    test(
        &[1, 2, 3],
        &[6, 7],
        100,
        false,
        &[0xfffffffb, 0xfffffffa, 2],
        &[6, 7],
    );
    test(
        &[6, 7],
        &[1, 2, 3],
        100,
        true,
        &[6, 7],
        &[5, 5, 0xfffffffd, 15],
    );
    test(&[6, 7], &[1, 2], 100, false, &[5, 5], &[1, 2]);
    test(
        &[1, 2],
        &[6, 7],
        100,
        false,
        &[0xfffffffb, 0xfffffffa, u32::MAX, 15],
        &[6, 7],
    );
}

#[test]
fn test_mod_power_of_2_sub() {
    let test = |s, t, pow, out| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        assert!(u.mod_power_of_2_is_reduced(pow));
        assert!(v.mod_power_of_2_is_reduced(pow));

        let mut n = u.clone();
        n.mod_power_of_2_sub_assign(v.clone(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_power_of_2_is_reduced(pow));

        let mut n = u.clone();
        n.mod_power_of_2_sub_assign(&v, pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_power_of_2_sub(v.clone(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_power_of_2_sub(v.clone(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_power_of_2_sub(&v, pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_power_of_2_sub(&v, pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", 0, "0");
    test("0", "0", 5, "0");
    test("0", "27", 5, "5");
    test("10", "2", 4, "8");
    test("2", "10", 4, "8");
    test("0", "5", 7, "123");
    test("123", "0", 7, "123");
    test("123", "56", 9, "67");
    test("56", "123", 9, "445");
    test("3", "1267650600228229401496703205375", 100, "4");
    test(
        "10970645355953595821",
        "19870830162202579837",
        65,
        "27993303341170119216",
    );
    test(
        "14424295573283161220",
        "2247489031103704789",
        66,
        "12176806542179456431",
    );
    test(
        "2247489031103704789",
        "14424295573283161220",
        66,
        "61610169752658750033",
    );
    test(
        "340279770772528537691305857201098194975",
        "5708990430541473157891818604560539975629668416",
        165,
        "46762343404498631680132551366007801946215309901791",
    );
}

#[test]
fn mod_power_of_2_sub_fail() {
    assert_panic!(Natural::ZERO.mod_power_of_2_sub(Natural::ONE, 0));
    assert_panic!(Natural::ONE.mod_power_of_2_sub(Natural::ZERO, 0));

    assert_panic!(Natural::ZERO.mod_power_of_2_sub(&Natural::ONE, 0));
    assert_panic!(Natural::ONE.mod_power_of_2_sub(&Natural::ZERO, 0));

    assert_panic!((&Natural::ZERO).mod_power_of_2_sub(Natural::ONE, 0));
    assert_panic!((&Natural::ONE).mod_power_of_2_sub(Natural::ZERO, 0));

    assert_panic!((&Natural::ZERO).mod_power_of_2_sub(Natural::ONE, 0));
    assert_panic!((&Natural::ONE).mod_power_of_2_sub(Natural::ZERO, 0));

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_power_of_2_sub_assign(Natural::ONE, 0);
    });
    assert_panic!({
        let mut x = Natural::ONE;
        x.mod_power_of_2_sub_assign(Natural::ZERO, 0);
    });

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_power_of_2_sub_assign(&Natural::ONE, 0);
    });
    assert_panic!({
        let mut x = Natural::ONE;
        x.mod_power_of_2_sub_assign(&Natural::ZERO, 0);
    });
}

#[test]
fn limbs_mod_power_of_2_limb_sub_limbs_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_unsigned_triple_gen_var_16().test_properties_with_config(
        &config,
        |(ys, x, pow)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_mod_power_of_2_limb_sub_limbs(x, &ys, pow)),
                Natural::from(x).mod_power_of_2_sub(Natural::from_owned_limbs_asc(ys), pow),
            );
        },
    );
}

#[test]
fn limbs_mod_power_of_2_limb_sub_limbs_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_unsigned_triple_gen_var_16().test_properties_with_config(
        &config,
        |(mut ys, x, pow)| {
            let old_ys = ys.clone();
            limbs_mod_power_of_2_limb_sub_limbs_in_place(x, &mut ys, pow);
            let n = Natural::from(x).mod_power_of_2_sub(Natural::from_owned_limbs_asc(old_ys), pow);
            let mut expected_limbs = n.into_limbs_asc();
            expected_limbs.resize(ys.len(), 0);
            assert_eq!(ys, expected_limbs);
        },
    );
}

#[test]
fn limbs_mod_power_of_2_sub_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18().test_properties_with_config(
        &config,
        |(xs, ys, pow)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_mod_power_of_2_sub(&xs, &ys, pow)),
                Natural::from_owned_limbs_asc(xs)
                    .mod_power_of_2_sub(Natural::from_owned_limbs_asc(ys), pow)
            );
        },
    );
}

#[test]
fn limbs_mod_power_of_2_sub_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18().test_properties_with_config(
        &config,
        |(mut xs, ys, pow)| {
            let xs_old = xs.clone();
            limbs_mod_power_of_2_sub_in_place_left(&mut xs, &ys, pow);
            assert_eq!(
                Natural::from_owned_limbs_asc(xs),
                Natural::from_owned_limbs_asc(xs_old)
                    .mod_power_of_2_sub(Natural::from_owned_limbs_asc(ys), pow)
            );
        },
    );
}

#[test]
fn limbs_mod_power_of_2_sub_in_place_right_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_20().test_properties_with_config(
        &config,
        |(xs, mut ys, pow)| {
            let ys_old = ys.clone();
            limbs_mod_power_of_2_sub_in_place_right(&xs, &mut ys, pow);
            assert_eq!(
                Natural::from_owned_limbs_asc(ys),
                Natural::from_owned_limbs_asc(xs)
                    .mod_power_of_2_sub(Natural::from_owned_limbs_asc(ys_old), pow)
            );
        },
    );
}

#[test]
fn limbs_mod_power_of_2_sub_in_place_either_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_20().test_properties_with_config(
        &config,
        |(mut xs, mut ys, pow)| {
            let xs_old = xs.clone();
            let ys_old = ys.clone();
            let right = limbs_mod_power_of_2_sub_in_place_either(&mut xs, &mut ys, pow);
            let n = Natural::from_limbs_asc(&xs_old)
                .mod_power_of_2_sub(Natural::from_limbs_asc(&ys_old), pow);
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
fn mod_power_of_2_sub_properties() {
    natural_natural_unsigned_triple_gen_var_4().test_properties(|(x, y, pow)| {
        assert!(x.mod_power_of_2_is_reduced(pow));
        assert!(y.mod_power_of_2_is_reduced(pow));
        let diff_val_val = x.clone().mod_power_of_2_sub(y.clone(), pow);
        let diff_val_ref = x.clone().mod_power_of_2_sub(&y, pow);
        let diff_ref_val = (&x).mod_power_of_2_sub(y.clone(), pow);
        let diff = (&x).mod_power_of_2_sub(&y, pow);
        assert!(diff_val_val.is_valid());
        assert!(diff_val_ref.is_valid());
        assert!(diff_ref_val.is_valid());
        assert!(diff.is_valid());
        assert!(diff.mod_power_of_2_is_reduced(pow));
        assert_eq!(diff_val_val, diff);
        assert_eq!(diff_val_ref, diff);
        assert_eq!(diff_ref_val, diff);

        assert_eq!(
            (Integer::from(&x) - Integer::from(&y)).mod_power_of_2(pow),
            diff
        );
        let diff_alt = if x >= y {
            &x - &y
        } else {
            let mut x = x.clone();
            x.set_bit(pow);
            &x - &y
        };
        assert_eq!(diff_alt, diff);
        assert_eq!((&x).mod_sub(&y, Natural::power_of_2(pow)), diff);

        let mut mut_x = x.clone();
        mut_x.mod_power_of_2_sub_assign(y.clone(), pow);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, diff);
        let mut mut_x = x.clone();
        mut_x.mod_power_of_2_sub_assign(&y, pow);
        assert_eq!(mut_x, diff);
        assert!(mut_x.is_valid());

        assert_eq!(
            (&y).mod_power_of_2_sub(&x, pow),
            (&diff).mod_power_of_2_neg(pow),
        );
        assert_eq!(
            (&x).mod_power_of_2_add((&y).mod_power_of_2_neg(pow), pow),
            diff
        );
        assert_eq!((&diff).mod_power_of_2_add(&y, pow), x);
        assert_eq!(diff.mod_power_of_2_sub(x, pow), y.mod_power_of_2_neg(pow));
    });

    natural_unsigned_pair_gen_var_11().test_properties(|(x, pow)| {
        assert_eq!((&x).mod_power_of_2_sub(Natural::ZERO, pow), x);
        assert_eq!(
            Natural::ZERO.mod_power_of_2_sub(&x, pow),
            (&x).mod_power_of_2_neg(pow)
        );
        assert_eq!((&x).mod_power_of_2_sub(&x, pow), 0);
    });

    unsigned_triple_gen_var_11::<Limb>().test_properties(|(x, y, pow)| {
        assert_eq!(
            x.mod_power_of_2_sub(y, pow),
            Natural::from(x).mod_power_of_2_sub(Natural::from(y), pow)
        );
    });
}
