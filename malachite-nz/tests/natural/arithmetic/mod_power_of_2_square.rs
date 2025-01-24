// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2Neg, ModPowerOf2Square,
    ModPowerOf2SquareAssign, ModSquare, PowerOf2, Square,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_gen_var_5, unsigned_pair_gen_var_17, unsigned_vec_pair_gen_var_1,
    unsigned_vec_pair_gen_var_20,
};
use malachite_nz::natural::arithmetic::mod_power_of_2_square::{
    limbs_mod_power_of_2_square, limbs_mod_power_of_2_square_ref, limbs_square_low,
    limbs_square_low_basecase, limbs_square_low_divide_and_conquer, limbs_square_low_scratch_len,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_natural_unsigned_triple_gen_var_4, natural_unsigned_pair_gen_var_11,
    unsigned_vec_pair_gen_var_21, unsigned_vec_unsigned_pair_gen_var_30,
};
use malachite_nz::test_util::natural::arithmetic::mod_power_of_2_square::*;
use std::panic::catch_unwind;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_square_low_basecase() {
    let test = |out_before: &[Limb], xs: &[Limb], out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        limbs_square_low_basecase(&mut out, xs);
        assert_eq!(out, out_after);

        let len = xs.len();
        let x = Natural::from_limbs_asc(xs);
        let pow = u64::exact_from(len) << Limb::LOG_WIDTH;
        let expected_square = (&x).square().mod_power_of_2(pow);
        assert_eq!(x.mod_power_of_2_square(pow), expected_square);
        let square = Natural::from_limbs_asc(&out_after[..len]);
        assert_eq!(square, expected_square);
        assert_eq!(&out_before[len..], &out_after[len..]);
    };
    // - n == 1
    test(&[10; 3], &[1], &[1, 10, 10]);
    // - n == 2
    test(&[10; 3], &[123, 456], &[15129, 112176, 10]);
    // - n > 2
    // - n.odd() in limbs_square_low_diagonal
    test(&[10; 4], &[123, 456, 789], &[15129, 112176, 402030, 10]);
    // - n.even() in limbs_square_low_diagonal
    test(
        &[10; 5],
        &[123, 456, 789, 987],
        &[15129, 112176, 402030, 962370, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_square_low_basecase_fail_1() {
    limbs_square_low_basecase(&mut [10], &[10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_square_low_basecase_fail_2() {
    limbs_square_low_basecase(&mut [10, 10], &[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_square_low_divide_and_conquer() {
    let test = |out_before: &[Limb], xs: &[Limb], out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_square_low_scratch_len(xs.len())];
        limbs_square_low_divide_and_conquer(&mut out, xs, &mut scratch);
        assert_eq!(out, out_after);

        let len = xs.len();
        let x = Natural::from_limbs_asc(xs);
        let pow = u64::exact_from(len) << Limb::LOG_WIDTH;
        let expected_square = (&x).square().mod_power_of_2(pow);
        assert_eq!(x.mod_power_of_2_square(pow), expected_square);
        let square = Natural::from_limbs_asc(&out_after[..len]);
        assert_eq!(square, expected_square);
        assert_eq!(&out_before[len..], &out_after[len..]);
    };
    test(&[10; 3], &[123, 456], &[15129, 112176, 10]);
    test(&[10; 4], &[123, 456, 789], &[15129, 112176, 402030, 10]);
    test(
        &[10; 5],
        &[123, 456, 789, 987],
        &[15129, 112176, 402030, 962370, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_square_low_divide_and_conquer_fail_1() {
    let mut scratch = vec![0; limbs_square_low_scratch_len(2)];
    limbs_square_low_divide_and_conquer(&mut [10], &[10, 10], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_square_low_divide_and_conquer_fail_2() {
    let mut scratch = vec![0; limbs_square_low_scratch_len(0)];
    limbs_square_low_divide_and_conquer(&mut [10, 10], &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_square_low_divide_and_conquer_fail_3() {
    let mut scratch = vec![0; limbs_square_low_scratch_len(1)];
    limbs_square_low_divide_and_conquer(&mut [10, 10], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_square_low() {
    let test = |out_before: &[Limb], xs: &[Limb], out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        limbs_square_low(&mut out, xs);
        assert_eq!(out, out_after);

        let len = xs.len();
        let x = Natural::from_limbs_asc(xs);
        let pow = u64::exact_from(len) << Limb::LOG_WIDTH;
        let expected_square = (&x).square().mod_power_of_2(pow);
        assert_eq!(x.mod_power_of_2_square(pow), expected_square);
        let square = Natural::from_limbs_asc(&out_after[..len]);
        assert_eq!(square, expected_square);
        assert_eq!(&out_before[len..], &out_after[len..]);
    };
    test(&[10; 3], &[1], &[1, 10, 10]);
    test(&[10; 3], &[123, 456], &[15129, 112176, 10]);
    test(&[10; 4], &[123, 456, 789], &[15129, 112176, 402030, 10]);
    test(
        &[10; 5],
        &[123, 456, 789, 987],
        &[15129, 112176, 402030, 962370, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_square_low_fail_1() {
    limbs_square_low(&mut [10], &[10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_square_low_fail_2() {
    limbs_square_low(&mut [10, 10], &[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_2_square() {
    let test = |xs, pow, out: &[Limb]| {
        assert_eq!(limbs_mod_power_of_2_square_ref(xs, pow), out);

        let mut mut_xs = xs.to_vec();
        assert_eq!(limbs_mod_power_of_2_square(&mut mut_xs, pow), out);

        let square = Natural::from_limbs_asc(out);
        assert_eq!(
            Natural::from_limbs_asc(xs).mod_power_of_2_square(pow),
            square
        );
        assert_eq!(
            Natural::from_limbs_asc(xs).square().mod_power_of_2(pow),
            square
        );
    };
    test(&[1], 1, &[1]);
    test(&[1], 1, &[1]);
    test(&[3], 2, &[1]);
    test(&[25], 5, &[17]);
    test(&[123, 456], 42, &[15129, 560]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_2_square_fail() {
    limbs_mod_power_of_2_square(&mut vec![], 2);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_2_square_ref_fail() {
    limbs_mod_power_of_2_square_ref(&[], 2);
}

#[test]
fn test_mod_power_of_2_square() {
    let test = |s, pow, out| {
        let u = Natural::from_str(s).unwrap();

        assert!(u.mod_power_of_2_is_reduced(pow));
        let n = u.clone().mod_power_of_2_square(pow);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
        assert!(n.mod_power_of_2_is_reduced(pow));

        let n = (&u).mod_power_of_2_square(pow);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = u;
        n.mod_power_of_2_square_assign(pow);
        assert_eq!(n.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 2, "0");
    test("1", 2, "1");
    test("2", 2, "0");
    test("2", 3, "4");
    test("5", 3, "1");
    test("100", 8, "16");
    test("12345678987654321", 64, "16556040056090124897");
}

#[test]
fn mod_power_of_2_square_fail() {
    assert_panic!(Natural::ONE.mod_power_of_2_square(0));
    assert_panic!((&Natural::ONE).mod_power_of_2_square(0));

    assert_panic!({
        let mut x = Natural::ONE;
        x.mod_power_of_2_square_assign(0);
    });
}

fn verify_limbs_square_low(out_before: &[Limb], xs: &[Limb], out_after: &[Limb]) {
    let len = xs.len();
    let x = Natural::from_limbs_asc(xs);
    let pow = u64::exact_from(len) << Limb::LOG_WIDTH;
    let expected_square = (&x).square().mod_power_of_2(pow);
    assert_eq!(x.mod_power_of_2_square(pow), expected_square);
    let square = Natural::from_limbs_asc(&out_after[..len]);
    assert_eq!(square, expected_square);
    assert_eq!(&out_before[len..], &out_after[len..]);
}

#[test]
fn limbs_square_low_basecase_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_21().test_properties_with_config(&config, |(mut out, xs)| {
        let out_old = out.clone();
        limbs_square_low_basecase(&mut out, &xs);
        verify_limbs_square_low(&out_old, &xs, &out);
        let expected_out = out;

        let mut out = out_old;
        limbs_square_low_basecase_unrestricted(&mut out, &xs);
        assert_eq!(out, expected_out);
    });
}

#[test]
fn limbs_square_low_divide_and_conquer_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_20().test_properties_with_config(&config, |(mut out, xs)| {
        let out_old = out.clone();
        let mut scratch = vec![0; limbs_square_low_scratch_len(xs.len())];
        limbs_square_low_divide_and_conquer(&mut out, &xs, &mut scratch);
        verify_limbs_square_low(&out_old, &xs, &out);
    });
}

#[test]
fn limbs_square_low_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_1().test_properties_with_config(&config, |(mut out, xs)| {
        let out_old = out.clone();
        limbs_square_low(&mut out, &xs);
        verify_limbs_square_low(&out_old, &xs, &out);
    });
}

#[test]
fn limbs_mod_power_of_2_square_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_30().test_properties_with_config(
        &config,
        |(mut xs, pow)| {
            let xs_old = xs.clone();
            let out = limbs_mod_power_of_2_square(&mut xs, pow);
            assert_eq!(limbs_mod_power_of_2_square_ref(&xs_old, pow), out);
            let expected_square = Natural::from_owned_limbs_asc(xs_old).mod_power_of_2_square(pow);
            assert_eq!(Natural::from_owned_limbs_asc(out), expected_square);
        },
    );
}

#[test]
fn mod_power_of_2_square_properties() {
    natural_unsigned_pair_gen_var_11().test_properties(|(n, pow)| {
        assert!(n.mod_power_of_2_is_reduced(pow));
        let square = (&n).mod_power_of_2_square(pow);
        assert!(square.is_valid());
        assert!(square.mod_power_of_2_is_reduced(pow));

        let square_alt = n.clone().mod_power_of_2_square(pow);
        assert!(square_alt.is_valid());
        assert_eq!(square_alt, square);

        let mut n_alt = n.clone();
        n_alt.mod_power_of_2_square_assign(pow);
        assert!(square_alt.is_valid());
        assert_eq!(square_alt, square);

        assert_eq!(square, (&n).square().mod_power_of_2(pow));
        assert_eq!(square, (&n).mod_square(Natural::power_of_2(pow)));
        assert_eq!(n.mod_power_of_2_neg(pow).mod_power_of_2_square(pow), square);
    });

    unsigned_gen_var_5().test_properties(|pow| {
        assert_eq!(Natural::ZERO.mod_power_of_2_square(pow), 0);
        if pow != 0 {
            assert_eq!(Natural::ONE.mod_power_of_2_square(pow), 1);
        }
    });

    natural_natural_unsigned_triple_gen_var_4().test_properties(|(x, y, pow)| {
        assert_eq!(
            (&x).mod_power_of_2_mul(&y, pow).mod_power_of_2_square(pow),
            x.mod_power_of_2_square(pow)
                .mod_power_of_2_mul(y.mod_power_of_2_square(pow), pow)
        );
    });

    unsigned_pair_gen_var_17::<Limb>().test_properties(|(n, pow)| {
        assert_eq!(
            n.mod_power_of_2_square(pow),
            Natural::from(n).mod_power_of_2_square(pow)
        );
    });
}
