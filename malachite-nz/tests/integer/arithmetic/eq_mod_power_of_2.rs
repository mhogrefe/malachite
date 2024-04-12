// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivisibleByPowerOf2, EqModPowerOf2, ModPowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    signed_signed_unsigned_triple_gen_var_2, unsigned_vec_unsigned_unsigned_triple_gen_var_8,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_9,
};
use malachite_nz::integer::arithmetic::eq_mod_power_of_2::{
    limbs_eq_mod_power_of_2_neg_limb, limbs_eq_mod_power_of_2_neg_pos,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{
    integer_integer_integer_unsigned_quadruple_gen_var_1,
    integer_integer_unsigned_triple_gen_var_1, integer_integer_unsigned_triple_gen_var_2,
    integer_integer_unsigned_triple_gen_var_3, integer_pair_gen, integer_unsigned_pair_gen_var_2,
    natural_natural_unsigned_triple_gen_var_1,
};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_mod_power_of_2_neg_limb() {
    let test = |xs, y, pow, out| {
        assert_eq!(limbs_eq_mod_power_of_2_neg_limb(xs, y, pow), out);
    };
    let width = Limb::WIDTH;
    test(&[1, 1], 3, 0, true);
    test(&[1, 1], 3, 1, true);
    test(&[1, 1], 3, 2, true);
    test(&[1, 1], 3, 3, false);
    test(&[1, 1], u32::MAX, 0, true);
    test(&[1, 1], u32::MAX, 1, true);
    test(&[1, 1], u32::MAX, width, true);
    test(&[1, 1], u32::MAX, width + 1, true);
    test(&[1, 2], u32::MAX, width + 1, false);
    test(&[1, u32::MAX, u32::MAX], u32::MAX, width + 1, true);
    test(&[1, u32::MAX, u32::MAX], u32::MAX, 2 * width, true);
    test(&[1, u32::MAX, u32::MAX], u32::MAX, 3 * width - 1, true);
    test(&[1, u32::MAX, u32::MAX], u32::MAX, 3 * width, true);
    test(&[1, u32::MAX, u32::MAX], u32::MAX, 3 * width + 1, false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_mod_power_of_2_neg_pos() {
    let test = |xs, ys, pow, out| {
        assert_eq!(limbs_eq_mod_power_of_2_neg_pos(xs, ys, pow), out);
    };
    test(&[0b1111011, 0b111001000], &[0b10101], 4, true);
    test(&[0b1111011, 0b111001000], &[0b10101], 5, false);
    test(
        &[0b1111011, 0b111001000],
        &[0b11111111111111111111111110000101, 0b1111],
        35,
        true,
    );
    test(
        &[0b1111011, 0b111001000],
        &[0b11111111111111111111111110000101, 0b1111],
        36,
        false,
    );
    test(
        &[0b1111011, 0b111001000],
        &[0b11111111111111111111111110000101, 0b1111],
        100,
        false,
    );
    test(
        &[0b1111011, 0b111001000],
        &[0b11111111111111111111111110000101, 0b10111],
        37,
        true,
    );
    test(
        &[0b1111011, 0b111001000],
        &[0b11111111111111111111111110000101, 0b10111],
        38,
        false,
    );
    test(
        &[0b1111011, 0b111001000],
        &[0b11111111111111111111111110000101, 0b10111],
        100,
        false,
    );

    test(
        &[0xabcdabcd, 0x12341234],
        &[0x54325433, 0xedcbedcb],
        64,
        true,
    );
    test(&[0xabcdabcd, 0x12341234], &[0, 0xedcbedcb], 64, false);
    test(
        &[0xabcdabcd, 0x12341234],
        &[0x54325433, 0xedcbedcb],
        65,
        false,
    );
    test(
        &[0xabcdabcd, 0x12341234],
        &[0x54325433, 0xedcbedcb],
        128,
        false,
    );
    test(&[0, 0, 0x12341234], &[0, 0, 0x1234edcc], 80, true);

    test(
        &[0x54325433, 0xedcbedcb],
        &[0xabcdabcd, 0x12341234],
        64,
        true,
    );
    test(&[0, 0xedcbedcb], &[0xabcdabcd, 0x12341234], 64, false);
    test(
        &[0x54325433, 0xedcbedcb],
        &[0xabcdabcd, 0x12341234],
        65,
        false,
    );
    test(
        &[0x54325433, 0xedcbedcb],
        &[0xabcdabcd, 0x12341234],
        128,
        false,
    );
    test(&[0, 0, 0x1234edcc], &[0, 0, 0x12341234], 80, true);
}

#[test]
fn test_eq_mod_power_of_2() {
    let test = |x, y, pow, out| {
        assert_eq!(
            Integer::from_str(x)
                .unwrap()
                .eq_mod_power_of_2(&Integer::from_str(y).unwrap(), pow),
            out
        );
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(
            rug::Integer::from_str(x)
                .unwrap()
                .is_congruent_2pow(&rug::Integer::from_str(y).unwrap(), Limb::exact_from(pow)),
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

    test("0", "-256", 8, true);
    test("0", "-256", 9, false);
    test("-13", "27", 0, true);
    test("-13", "27", 1, true);
    test("-13", "27", 2, true);
    test("-13", "27", 3, true);
    test("-13", "27", 4, false);
    test("-13", "27", 100, false);
    test("13", "-27", 0, true);
    test("13", "-27", 1, true);
    test("13", "-27", 2, true);
    test("13", "-27", 3, true);
    test("13", "-27", 4, false);
    test("13", "-27", 100, false);
    test("-1000000000001", "4095", 13, true);
    test("-1000000000001", "4095", 14, false);
    test("1000000000001", "-4095", 13, true);
    test("1000000000001", "-4095", 14, false);
    test("4294967295", "-1", 32, true);
    test("-1", "4294967295", 32, true);

    test("-13", "-21", 0, true);
    test("-13", "-21", 1, true);
    test("-13", "-21", 2, true);
    test("-13", "-21", 3, true);
    test("-13", "-21", 4, false);
    test("-13", "-21", 100, false);
    test("-1000000000001", "-1", 12, true);
    test("-1000000000001", "-1", 13, false);
    test("-4294967295", "-4294967295", 32, true);
    test("-281474976710672", "-844424930131984", 49, true);
    test("-281474976710672", "-844424930131984", 50, false);

    test("1311693408901639117", "-17135050664807912499", 64, true);
    test("1311693408901639117", "-17135050663395328000", 64, false);
    test("1311693408901639117", "-17135050664807912499", 65, false);
    test("1311693408901639117", "-17135050664807912499", 128, false);
    test(
        "5633680281231555440641310720",
        "-5634717283396403096794955776",
        80,
        true,
    );

    test("-1311693408901639117", "17135050664807912499", 64, true);
    test("-1311693408901639117", "17135050663395328000", 64, false);
    test("-1311693408901639117", "17135050664807912499", 65, false);
    test("-1311693408901639117", "17135050664807912499", 128, false);
    test(
        "-5633680281231555440641310720",
        "5634717283396403096794955776",
        80,
        true,
    );
    test("18446744073709541007", "-10609", 64, true);
    test("18446744073709541007", "-10609", 65, false);
    test("79228162514264337589248972431", "-4294977905", 96, true);
    test("79228162514264337589248972431", "-4294977905", 97, false);
}

#[test]
fn limbs_eq_mod_power_of_2_neg_limb_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_unsigned_triple_gen_var_8().test_properties_with_config(
        &config,
        |(xs, y, pow)| {
            assert_eq!(
                limbs_eq_mod_power_of_2_neg_limb(&xs, y, pow),
                (-Natural::from_owned_limbs_asc(xs)).eq_mod_power_of_2(&Integer::from(y), pow)
            );
        },
    );
}

#[test]
fn limbs_eq_mod_power_of_2_neg_pos_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_9().test_properties_with_config(
        &config,
        |(xs, ys, pow)| {
            assert_eq!(
                limbs_eq_mod_power_of_2_neg_pos(&xs, &ys, pow),
                (-Natural::from_owned_limbs_asc(xs))
                    .eq_mod_power_of_2(&Integer::from(Natural::from_owned_limbs_asc(ys)), pow)
            );
        },
    );
}

#[test]
fn eq_mod_power_of_2_properties() {
    integer_integer_unsigned_triple_gen_var_1().test_properties(|(x, y, pow)| {
        let eq_mod_power_of_2 = x.eq_mod_power_of_2(&y, pow);
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(
            rug::Integer::from(&x)
                .is_congruent_2pow(&rug::Integer::from(&y), Limb::exact_from(pow)),
            eq_mod_power_of_2
        );
        assert_eq!(y.eq_mod_power_of_2(&x, pow), eq_mod_power_of_2);
        assert_eq!(
            x.mod_power_of_2(pow) == y.mod_power_of_2(pow),
            eq_mod_power_of_2,
        );
    });

    integer_integer_unsigned_triple_gen_var_2().test_properties(|(x, y, pow)| {
        assert!(x.eq_mod_power_of_2(&y, pow), "{x} {y} {pow}");
        #[cfg(feature = "32_bit_limbs")]
        assert!(rug::Integer::from(&x)
            .is_congruent_2pow(&rug::Integer::from(&y), Limb::exact_from(pow)));
        assert!(y.eq_mod_power_of_2(&x, pow));
        assert_eq!(x.mod_power_of_2(pow), y.mod_power_of_2(pow));
    });

    integer_integer_unsigned_triple_gen_var_3().test_properties(|(x, y, pow)| {
        assert!(!x.eq_mod_power_of_2(&y, pow));
        #[cfg(feature = "32_bit_limbs")]
        assert!(!rug::Integer::from(&x)
            .is_congruent_2pow(&rug::Integer::from(&y), Limb::exact_from(pow)));
        assert!(!y.eq_mod_power_of_2(&x, pow));
        assert_ne!(x.mod_power_of_2(pow), y.mod_power_of_2(pow));
    });

    integer_unsigned_pair_gen_var_2().test_properties(|(n, pow)| {
        assert!(n.eq_mod_power_of_2(&n, pow));
        assert_eq!(
            n.eq_mod_power_of_2(&Integer::ZERO, pow),
            n.divisible_by_power_of_2(pow)
        );
        assert_eq!(
            Integer::ZERO.eq_mod_power_of_2(&n, pow),
            n.divisible_by_power_of_2(pow)
        );
    });

    integer_integer_integer_unsigned_quadruple_gen_var_1().test_properties(|(x, y, z, pow)| {
        if x.eq_mod_power_of_2(&y, pow) && y.eq_mod_power_of_2(&z, pow) {
            assert!(x.eq_mod_power_of_2(&z, pow));
        }
    });

    integer_pair_gen().test_properties(|(x, y)| {
        assert!(x.eq_mod_power_of_2(&y, 0));
    });

    natural_natural_unsigned_triple_gen_var_1().test_properties(|(x, y, pow)| {
        assert_eq!(
            x.eq_mod_power_of_2(&y, pow),
            Integer::from(x).eq_mod_power_of_2(&Integer::from(y), pow),
        );
    });

    signed_signed_unsigned_triple_gen_var_2::<SignedLimb, u64>().test_properties(|(x, y, pow)| {
        assert_eq!(
            x.eq_mod_power_of_2(y, pow),
            Integer::from(x).eq_mod_power_of_2(&Integer::from(y), pow),
        );
    });
}
