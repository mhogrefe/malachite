// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    DivisibleByPowerOf2, ModPowerOf2, ModPowerOf2Assign, ModPowerOf2IsReduced, NegModPowerOf2,
    NegModPowerOf2Assign, PowerOf2, RemPowerOf2, RemPowerOf2Assign, ShrRound,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::LowMask;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_pair_gen_var_2, unsigned_pair_gen_var_20,
    unsigned_vec_unsigned_pair_gen_var_16,
};
use malachite_nz::natural::arithmetic::mod_power_of_2::{
    limbs_mod_power_of_2, limbs_neg_mod_power_of_2, limbs_neg_mod_power_of_2_in_place,
    limbs_slice_mod_power_of_2_in_place, limbs_vec_mod_power_of_2_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_gen, natural_natural_unsigned_triple_gen_var_1, natural_unsigned_pair_gen_var_10,
    natural_unsigned_pair_gen_var_4, natural_unsigned_pair_gen_var_9,
    natural_unsigned_unsigned_triple_gen_var_5,
};
use std::cmp::min;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_2_and_limbs_vec_mod_power_of_2_in_place() {
    let test = |xs: &[Limb], pow: u64, out: &[Limb]| {
        assert_eq!(limbs_mod_power_of_2(xs, pow), out);

        let mut xs = xs.to_vec();
        limbs_vec_mod_power_of_2_in_place(&mut xs, pow);
        assert_eq!(xs, out);
    };
    test(&[], 0, &[]);
    test(&[], 5, &[]);
    test(&[], 100, &[]);
    test(&[6, 7], 2, &[2]);
    test(&[100, 101, 102], 10, &[100]);
    test(&[123, 456], 0, &[]);
    test(&[123, 456], 1, &[1]);
    test(&[123, 456], 10, &[123]);
    test(&[123, 456], 33, &[123, 0]);
    test(&[123, 456], 40, &[123, 200]);
    test(&[123, 456], 100, &[123, 456]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_mod_power_of_2_in_place() {
    let test = |xs: &[Limb], pow: u64, out: &[Limb]| {
        let mut xs = xs.to_vec();
        limbs_slice_mod_power_of_2_in_place(&mut xs, pow);
        assert_eq!(xs, out);
    };
    test(&[], 0, &[]);
    test(&[], 5, &[]);
    test(&[], 100, &[]);
    test(&[6, 7], 2, &[2, 0]);
    test(&[100, 101, 102], 10, &[100, 0, 0]);
    test(&[123, 456], 0, &[0, 0]);
    test(&[123, 456], 1, &[1, 0]);
    test(&[123, 456], 10, &[123, 0]);
    test(&[123, 456], 33, &[123, 0]);
    test(&[123, 456], 40, &[123, 200]);
    test(&[123, 456], 100, &[123, 456]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_mod_power_of_2_and_limbs_neg_mod_power_of_2_in_place() {
    let test = |xs: &[Limb], pow: u64, out: &[Limb]| {
        assert_eq!(limbs_neg_mod_power_of_2(xs, pow), out);

        let mut xs = xs.to_vec();
        limbs_neg_mod_power_of_2_in_place(&mut xs, pow);
        assert_eq!(xs, out);
    };
    test(&[], 0, &[]);
    test(&[], 5, &[0]);
    test(&[], 100, &[0, 0, 0, 0]);
    test(&[6, 7], 2, &[2]);
    test(&[100, 101, 102], 10, &[924]);
    test(&[123, 456], 0, &[]);
    test(&[123, 456], 1, &[1]);
    test(&[123, 456], 10, &[901]);
    test(&[123, 456], 33, &[4294967173, 1]);
    test(&[123, 456], 40, &[4294967173, 55]);
    test(&[123, 456], 100, &[4294967173, 4294966839, u32::MAX, 15]);
}

#[test]
fn test_mod_power_of_2_and_rem_power_of_2() {
    let test = |s, v: u64, out| {
        let u = Natural::from_str(s).unwrap();

        let mut n = u.clone();
        n.mod_power_of_2_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_power_of_2_is_reduced(v));

        let n = u.clone().mod_power_of_2(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_power_of_2(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n.rem_power_of_2_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().rem_power_of_2(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).rem_power_of_2(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 0, "0");
    test("260", 8, "4");
    test("1611", 4, "11");
    test("123", 100, "123");
    test("1000000000000", 0, "0");
    test("1000000000000", 12, "0");
    test("1000000000001", 12, "1");
    test("999999999999", 12, "4095");
    test("1000000000000", 15, "4096");
    test("1000000000000", 100, "1000000000000");
    test("1000000000000000000000000", 40, "1020608380928");
    test("1000000000000000000000000", 64, "2003764205206896640");
    test("4294967295", 31, "2147483647");
    test("4294967295", 32, "4294967295");
    test("4294967295", 33, "4294967295");
    test("4294967296", 31, "0");
    test("4294967296", 32, "0");
    test("4294967296", 33, "4294967296");
    test("4294967297", 31, "1");
    test("4294967297", 32, "1");
    test("4294967297", 33, "4294967297");
}

#[test]
fn test_neg_mod_power_of_2() {
    let test = |s, v: u64, out| {
        let u = Natural::from_str(s).unwrap();

        let mut n = u.clone();
        n.neg_mod_power_of_2_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_power_of_2_is_reduced(v));

        let n = u.clone().neg_mod_power_of_2(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).neg_mod_power_of_2(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };

    test("0", 0, "0");
    test("260", 8, "252");
    test("1611", 4, "5");
    test("123", 100, "1267650600228229401496703205253");
    test("1000000000000", 0, "0");
    test("1000000000000", 12, "0");
    test("1000000000001", 12, "4095");
    test("999999999999", 12, "1");
    test("1000000000000", 15, "28672");
    test("1000000000000", 100, "1267650600228229400496703205376");
    test("1000000000000000000000000", 40, "78903246848");
    test("1000000000000000000000000", 64, "16442979868502654976");
    test("4294967295", 31, "1");
    test("4294967295", 32, "1");
    test("4294967295", 33, "4294967297");
    test("4294967296", 31, "0");
    test("4294967296", 32, "0");
    test("4294967296", 33, "4294967296");
    test("4294967297", 31, "2147483647");
    test("4294967297", 32, "4294967295");
    test("4294967297", 33, "4294967295");
}

#[test]
fn limbs_mod_power_of_2_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(&config, |(xs, pow)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_mod_power_of_2(&xs, pow)),
            Natural::from_owned_limbs_asc(xs).mod_power_of_2(pow),
        );
    });
}

macro_rules! limbs_slice_mod_power_of_2_in_place_helper {
    ($f: ident, $xs: ident, $pow: ident) => {
        let old_xs = $xs.clone();
        $f(&mut $xs, $pow);
        let n = Natural::from_limbs_asc(&old_xs).mod_power_of_2($pow);
        assert_eq!(Natural::from_owned_limbs_asc($xs), n);
    };
}

#[test]
fn limbs_slice_mod_power_of_2_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(
        &config,
        |(mut xs, pow)| {
            limbs_slice_mod_power_of_2_in_place_helper!(
                limbs_slice_mod_power_of_2_in_place,
                xs,
                pow
            );
        },
    );
}

#[test]
fn limbs_vec_mod_power_of_2_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(
        &config,
        |(mut xs, pow)| {
            limbs_slice_mod_power_of_2_in_place_helper!(limbs_vec_mod_power_of_2_in_place, xs, pow);
        },
    );
}

#[test]
fn limbs_neg_mod_power_of_2_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(&config, |(xs, pow)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_neg_mod_power_of_2(&xs, pow)),
            Natural::from_owned_limbs_asc(xs).neg_mod_power_of_2(pow),
        );
    });
}

#[test]
fn limbs_neg_mod_power_of_2_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(
        &config,
        |(mut xs, pow)| {
            let old_xs = xs.clone();
            limbs_neg_mod_power_of_2_in_place(&mut xs, pow);
            let n = Natural::from_limbs_asc(&old_xs).neg_mod_power_of_2(pow);
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
        },
    );
}

#[test]
fn mod_power_of_2_and_rem_power_of_2_properties() {
    natural_unsigned_pair_gen_var_4().test_properties(|(n, u)| {
        let mut mut_n = n.clone();
        mut_n.mod_power_of_2_assign(u);
        assert!(mut_n.is_valid());
        let result = mut_n;
        assert!(result.mod_power_of_2_is_reduced(u));

        let result_alt = (&n).mod_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = n.clone().mod_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let mut mut_n = n.clone();
        mut_n.rem_power_of_2_assign(u);
        assert!(mut_n.is_valid());
        let result_alt = mut_n;
        assert_eq!(result_alt, result);

        let result_alt = (&n).rem_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = n.clone().rem_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert!(result <= n);
        assert_eq!((&n >> u << u) + &result, n);
        assert!(result < Natural::power_of_2(u));
        assert_eq!(result == 0, n.divisible_by_power_of_2(u));
        assert_eq!((&result).mod_power_of_2(u), result);
        assert_eq!(n & Natural::low_mask(u), result);
    });

    natural_natural_unsigned_triple_gen_var_1().test_properties(|(x, y, u)| {
        assert_eq!(
            (&x + &y).mod_power_of_2(u),
            ((&x).mod_power_of_2(u) + (&y).mod_power_of_2(u)).mod_power_of_2(u)
        );
        assert_eq!(
            (&x * &y).mod_power_of_2(u),
            (x.mod_power_of_2(u) * y.mod_power_of_2(u)).mod_power_of_2(u)
        );
    });

    natural_unsigned_pair_gen_var_9().test_properties(|(n, u)| {
        assert_eq!(n.mod_power_of_2(u), 0);
    });

    natural_unsigned_pair_gen_var_10().test_properties(|(n, u)| {
        assert_ne!((&n).mod_power_of_2(u), 0);
        assert_eq!(
            (&n).mod_power_of_2(u) + n.neg_mod_power_of_2(u),
            Natural::power_of_2(u)
        );
    });

    natural_unsigned_unsigned_triple_gen_var_5().test_properties(|(n, u, v)| {
        assert_eq!(
            (&n).mod_power_of_2(u).mod_power_of_2(v),
            n.mod_power_of_2(min(u, v))
        );
    });

    natural_gen().test_properties(|n| {
        assert_eq!(n.mod_power_of_2(0), 0);
    });

    unsigned_gen().test_properties(|u| {
        assert_eq!(Natural::ZERO.mod_power_of_2(u), 0);
    });

    unsigned_pair_gen_var_2::<Limb, u64>().test_properties(|(u, pow)| {
        assert_eq!(u.mod_power_of_2(pow), Natural::from(u).mod_power_of_2(pow));
        assert_eq!(u.rem_power_of_2(pow), Natural::from(u).rem_power_of_2(pow));
    });
}

#[test]
fn neg_mod_power_of_2_properties() {
    natural_unsigned_pair_gen_var_4().test_properties(|(n, u)| {
        let mut mut_n = n.clone();
        mut_n.neg_mod_power_of_2_assign(u);
        assert!(mut_n.is_valid());
        let result = mut_n;
        assert!(result.mod_power_of_2_is_reduced(u));

        let result_alt = (&n).neg_mod_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = n.clone().neg_mod_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!(((&n).shr_round(u, Ceiling).0 << u) - &result, n);
        assert!(result < Natural::power_of_2(u));
        assert_eq!(result == 0, n.divisible_by_power_of_2(u));
        assert_eq!((&result).neg_mod_power_of_2(u), (&n).mod_power_of_2(u));
        assert_eq!((-n).mod_power_of_2(u), result);
    });

    natural_natural_unsigned_triple_gen_var_1().test_properties(|(x, y, u)| {
        assert_eq!(
            (&x + &y).neg_mod_power_of_2(u),
            ((&x).mod_power_of_2(u) + (&y).mod_power_of_2(u)).neg_mod_power_of_2(u)
        );
        assert_eq!(
            (&x * &y).neg_mod_power_of_2(u),
            (x.mod_power_of_2(u) * y.mod_power_of_2(u)).neg_mod_power_of_2(u)
        );
    });

    natural_unsigned_pair_gen_var_9().test_properties(|(n, u)| {
        assert_eq!(n.neg_mod_power_of_2(u), 0);
    });

    natural_unsigned_pair_gen_var_10().test_properties(|(n, u)| {
        let m = (&n).neg_mod_power_of_2(u);
        assert_ne!(m, 0);
        assert_eq!((((&n >> u) + Natural::ONE) << u) - &m, n);
        assert_eq!(n.mod_power_of_2(u) + m, Natural::power_of_2(u));
    });

    natural_gen().test_properties(|n| {
        assert_eq!(n.neg_mod_power_of_2(0), 0);
    });

    unsigned_gen().test_properties(|u| {
        assert_eq!(Natural::ZERO.neg_mod_power_of_2(u), 0);
    });

    unsigned_pair_gen_var_20::<Limb>().test_properties(|(u, pow)| {
        assert_eq!(
            u.neg_mod_power_of_2(pow),
            Natural::from(u).neg_mod_power_of_2(pow)
        );
    });
}
