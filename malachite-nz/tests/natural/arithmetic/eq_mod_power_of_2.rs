// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivisibleByPowerOf2, EqModPowerOf2, ModPowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_triple_gen_var_4, unsigned_vec_unsigned_unsigned_triple_gen_var_8,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_9,
};
use malachite_nz::natural::arithmetic::eq_mod_power_of_2::{
    limbs_eq_limb_mod_power_of_2, limbs_eq_mod_power_of_2,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_natural_natural_unsigned_quadruple_gen_var_1,
    natural_natural_unsigned_triple_gen_var_1, natural_natural_unsigned_triple_gen_var_2,
    natural_natural_unsigned_triple_gen_var_3, natural_pair_gen, natural_unsigned_pair_gen_var_4,
};
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_limb_mod_power_of_2() {
    let test = |xs, y, pow, out| {
        assert_eq!(limbs_eq_limb_mod_power_of_2(xs, y, pow), out);
    };
    test(&[0b1111011, 0b111001000], 0b101011, 4, true);
    test(&[0b1111011, 0b111001000], 0b101011, 5, false);
    test(&[0b1111011, 0b111001000], 0b1111011, 35, true);
    test(&[0b1111011, 0b111001000], 0b1111011, 36, false);
    test(&[0b1111011, 0b111001000], 0b1111011, 100, false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_mod_power_of_2() {
    let test = |xs, ys, pow, out| {
        assert_eq!(limbs_eq_mod_power_of_2(xs, ys, pow), out);
    };
    test(&[0b1111011, 0b111001000], &[0b101011], 4, true);
    test(&[0b1111011, 0b111001000], &[0b101011], 5, false);
    test(&[0b1111011, 0b111001000], &[0b1111011], 35, true);
    test(&[0b1111011, 0b111001000], &[0b1111011], 36, false);
    test(&[0b1111011, 0b111001000], &[0b1111011], 100, false);
    test(
        &[0b1111011, 0b111001000],
        &[0b1111011, 0b111101000],
        37,
        true,
    );
    test(
        &[0b1111011, 0b111001000],
        &[0b1111011, 0b111101000],
        38,
        false,
    );
    test(
        &[0b1111011, 0b111001000],
        &[0b1111011, 0b111101000],
        100,
        false,
    );
}

#[test]
fn test_eq_mod_power_of_2() {
    let test = |x, y, pow, out| {
        assert_eq!(
            Natural::from_str(x)
                .unwrap()
                .eq_mod_power_of_2(&Natural::from_str(y).unwrap(), pow),
            out
        );
        assert_eq!(
            rug::Integer::from_str(x)
                .unwrap()
                .is_congruent_2pow(&rug::Integer::from_str(y).unwrap(), u32::exact_from(pow)),
            out
        );
    };
    test("0", "256", 8, true);
    test("0", "256", 9, false);
    test("13", "21", 0, true);
    test("13", "21", 1, true);
    test("13", "21", 2, true);
    test("13", "21", 3, true);
    test("13", "21", 4, false);
    test("13", "21", 100, false);
    test("1000000000001", "1", 12, true);
    test("1000000000001", "1", 13, false);
    test("4294967295", "4294967295", 32, true);
    test("281474976710672", "844424930131984", 49, true);
    test("281474976710672", "844424930131984", 50, false);
}

#[test]
fn limbs_eq_limb_mod_power_of_2_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_unsigned_triple_gen_var_8().test_properties_with_config(
        &config,
        |(xs, y, pow)| {
            assert_eq!(
                limbs_eq_limb_mod_power_of_2(&xs, y, pow),
                Natural::from_owned_limbs_asc(xs).eq_mod_power_of_2(&Natural::from(y), pow)
            );
        },
    );
}

#[test]
fn limbs_eq_mod_power_of_2_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_9().test_properties_with_config(
        &config,
        |(xs, ys, pow)| {
            assert_eq!(
                limbs_eq_mod_power_of_2(&xs, &ys, pow),
                Natural::from_owned_limbs_asc(xs)
                    .eq_mod_power_of_2(&Natural::from_owned_limbs_asc(ys), pow)
            );
        },
    );
}

#[test]
fn eq_mod_power_of_2_properties() {
    natural_natural_unsigned_triple_gen_var_1().test_properties(|(x, y, pow)| {
        let eq_mod_power_of_2 = x.eq_mod_power_of_2(&y, pow);
        assert_eq!(
            rug::Integer::from(&x).is_congruent_2pow(&rug::Integer::from(&y), u32::exact_from(pow)),
            eq_mod_power_of_2
        );
        assert_eq!(y.eq_mod_power_of_2(&x, pow), eq_mod_power_of_2);
        assert_eq!(
            x.mod_power_of_2(pow) == y.mod_power_of_2(pow),
            eq_mod_power_of_2
        );
    });

    natural_natural_unsigned_triple_gen_var_2().test_properties(|(x, y, pow)| {
        assert!(x.eq_mod_power_of_2(&y, pow));
        assert!(
            rug::Integer::from(&x).is_congruent_2pow(&rug::Integer::from(&y), u32::exact_from(pow))
        );
        assert!(y.eq_mod_power_of_2(&x, pow));
        assert_eq!(x.mod_power_of_2(pow), y.mod_power_of_2(pow));
    });

    natural_natural_unsigned_triple_gen_var_3().test_properties(|(x, y, pow)| {
        assert!(!x.eq_mod_power_of_2(&y, pow));
        assert!(!rug::Integer::from(&x)
            .is_congruent_2pow(&rug::Integer::from(&y), u32::exact_from(pow)));
        assert!(!y.eq_mod_power_of_2(&x, pow));
        assert_ne!(x.mod_power_of_2(pow), y.mod_power_of_2(pow));
    });

    natural_unsigned_pair_gen_var_4().test_properties(|(n, pow)| {
        assert!(n.eq_mod_power_of_2(&n, pow));
        assert_eq!(
            n.eq_mod_power_of_2(&Natural::ZERO, pow),
            n.divisible_by_power_of_2(pow)
        );
        assert_eq!(
            Natural::ZERO.eq_mod_power_of_2(&n, pow),
            n.divisible_by_power_of_2(pow)
        );
    });

    natural_natural_natural_unsigned_quadruple_gen_var_1().test_properties(|(x, y, z, pow)| {
        if x.eq_mod_power_of_2(&y, pow) && y.eq_mod_power_of_2(&z, pow) {
            assert!(x.eq_mod_power_of_2(&z, pow));
        }
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert!(x.eq_mod_power_of_2(&y, 0));
    });

    unsigned_triple_gen_var_4::<Limb, u64>().test_properties(|(x, y, pow)| {
        assert_eq!(
            x.eq_mod_power_of_2(y, pow),
            Natural::from(x).eq_mod_power_of_2(&Natural::from(y), pow)
        );
    });
}
