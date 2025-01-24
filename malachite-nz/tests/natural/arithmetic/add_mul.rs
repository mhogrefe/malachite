// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign, CheckedAddMul};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_triple_gen_var_1, unsigned_triple_gen_var_19, unsigned_vec_triple_gen_var_41,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12,
};
use malachite_nz::natural::arithmetic::add_mul::{
    limbs_add_mul, limbs_add_mul_in_place_left, limbs_add_mul_limb,
    limbs_slice_add_mul_limb_same_length_in_place_left,
    limbs_slice_add_mul_limb_same_length_in_place_right, limbs_vec_add_mul_limb_in_place_either,
    limbs_vec_add_mul_limb_in_place_left, limbs_vec_add_mul_limb_in_place_right,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_pair_gen, natural_triple_gen};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_mul_limb() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], z: Limb, result: &[Limb]| {
        assert_eq!(limbs_add_mul_limb(xs_before, ys_before, z), result);
        let mut xs = xs_before.to_vec();
        limbs_vec_add_mul_limb_in_place_left(&mut xs, ys_before, z);
        assert_eq!(xs, result);
        let mut ys = ys_before.to_vec();
        limbs_vec_add_mul_limb_in_place_right(xs_before, &mut ys, z);
        assert_eq!(ys, result);
    };
    test(&[123, 456], &[123], 4, &[615, 456]);
    test(&[123, 456], &[123], u32::MAX, &[0, 579]);
    test(&[123], &[0, 123], 4, &[123, 492]);
    test(&[123, 456], &[0, 123], u32::MAX, &[123, 333, 123]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_add_mul_limb_same_length_in_place_left() {
    let test = |xs_before: &[Limb], ys: &[Limb], z: Limb, xs_after: &[Limb], carry: Limb| {
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_slice_add_mul_limb_same_length_in_place_left(&mut xs, ys, z),
            carry
        );
        assert_eq!(xs, xs_after);
    };
    test(&[123], &[123], 4, &[615], 0);
    test(&[123], &[123], u32::MAX, &[0], 123);
    test(&[123, 0], &[0, 123], 4, &[123, 492], 0);
    test(&[123, 456], &[0, 123], u32::MAX, &[123, 333], 123);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_add_mul_limb_same_length_in_place_left_fail() {
    limbs_slice_add_mul_limb_same_length_in_place_left(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_add_mul_limb_same_length_in_place_right() {
    let test = |xs: &[Limb], ys_before: &[Limb], z: Limb, ys_after: &[Limb], carry: Limb| {
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_slice_add_mul_limb_same_length_in_place_right(xs, &mut ys, z),
            carry
        );
        assert_eq!(ys, ys_after);
    };
    test(&[123, 456], &[123, 0], 4, &[615, 456], 0);
    test(&[123, 456], &[123, 0], u32::MAX, &[0, 579], 0);
    test(&[123, 0], &[0, 123], 4, &[123, 492], 0);
    test(&[123, 456], &[0, 123], u32::MAX, &[123, 333], 123);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_add_mul_limb_same_length_in_place_right_fail() {
    limbs_slice_add_mul_limb_same_length_in_place_right(&[10, 10], &mut [10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_vec_add_mul_limb_in_place_either() {
    let test = |xs_before: &[Limb],
                ys_before: &[Limb],
                z: Limb,
                right: bool,
                xs_after: &[Limb],
                ys_after: &[Limb]| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_vec_add_mul_limb_in_place_either(&mut xs, &mut ys, z),
            right
        );
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[123, 456], &[123], 4, false, &[615, 456], &[123]);
    test(
        &[123, 456],
        &[123, 0],
        u32::MAX,
        false,
        &[0, 579],
        &[123, 0],
    );
    test(&[123], &[0, 123], 4, true, &[123], &[123, 492]);
    test(
        &[123, 456],
        &[0, 123],
        u32::MAX,
        false,
        &[123, 333, 123],
        &[0, 123],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_mul_and_limbs_add_mul_in_place_left() {
    let test = |xs_before: &[Limb], ys: &[Limb], zs: &[Limb], result: &[Limb]| {
        assert_eq!(limbs_add_mul(xs_before, ys, zs), result);
        let mut xs = xs_before.to_vec();
        limbs_add_mul_in_place_left(&mut xs, ys, zs);
        assert_eq!(xs, result);
    };
    test(
        &[123, 456],
        &[123, 789],
        &[321, 654],
        &[39606, 334167, 516006],
    );
    test(
        &[123, 456, 789, 987, 654],
        &[123, 789],
        &[321, 654],
        &[39606, 334167, 516795, 987, 654],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_mul_fail_1() {
    limbs_add_mul(&[10, 10], &[], &[10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_mul_fail_2() {
    limbs_add_mul(&[10, 10], &[10, 10], &[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_mul_in_place_left_fail_1() {
    let mut xs = vec![10, 10];
    limbs_add_mul_in_place_left(&mut xs, &[], &[10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_mul_in_place_left_fail_2() {
    let mut xs = vec![10, 10];
    limbs_add_mul_in_place_left(&mut xs, &[10, 10], &[]);
}

#[test]
fn test_add_mul() {
    let test = |r, s, t, out| {
        let u = Natural::from_str(r).unwrap();
        let v = Natural::from_str(s).unwrap();
        let w = Natural::from_str(t).unwrap();

        let mut a = u.clone();
        a.add_mul_assign(v.clone(), w.clone());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = u.clone();
        a.add_mul_assign(v.clone(), &w);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = u.clone();
        a.add_mul_assign(&v, w.clone());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = u.clone();
        a.add_mul_assign(&v, &w);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = u.clone().add_mul(v.clone(), w.clone());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = u.clone().add_mul(v.clone(), &w);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = u.clone().add_mul(&v, w.clone());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = u.clone().add_mul(&v, &w);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&u).add_mul(&v, &w);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());
    };
    test("0", "0", "0", "0");
    test("0", "0", "123", "0");
    test("123", "0", "5", "123");
    test("123", "5", "1", "128");
    test("123", "5", "100", "623");
    test("10", "3", "4", "22");
    test("1000000000000", "0", "123", "1000000000000");
    test("1000000000000", "1", "123", "1000000000123");
    test("1000000000000", "123", "1", "1000000000123");
    test("1000000000000", "123", "100", "1000000012300");
    test("1000000000000", "100", "123", "1000000012300");
    test("1000000000000", "65536", "65536", "1004294967296");
    test("1000000000000", "1000000000000", "0", "1000000000000");
    test("1000000000000", "1000000000000", "1", "2000000000000");
    test("1000000000000", "1000000000000", "100", "101000000000000");
    test("0", "1000000000000", "100", "100000000000000");
    test(
        "1000000000000",
        "65536",
        "1000000000000",
        "65537000000000000",
    );
    test(
        "1000000000000",
        "1000000000000",
        "1000000000000",
        "1000000000001000000000000",
    );
    test(
        "0",
        "1000000000000",
        "1000000000000",
        "1000000000000000000000000",
    );
    test(
        "18446744073583722494",
        "2",
        "4033876984",
        "18446744081651476462",
    );
}

#[test]
fn limbs_add_mul_limb_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10().test_properties_with_config(
        &config,
        |(a, b, c)| {
            assert_eq!(
                limbs_add_mul_limb(&a, &b, c),
                Natural::from_owned_limbs_asc(a)
                    .add_mul(Natural::from_owned_limbs_asc(b), Natural::from(c))
                    .into_limbs_asc()
            );
        },
    );
}

#[test]
fn limbs_slice_add_mul_limb_same_length_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12().test_properties_with_config(
        &config,
        |(a, b, c)| {
            let a_old = a;
            let mut a = a_old.to_vec();
            let carry = limbs_slice_add_mul_limb_same_length_in_place_left(&mut a, &b, c);
            let len = b.len();
            let mut result = a[..len].to_vec();
            result.push(carry);
            assert_eq!(
                Natural::from_owned_limbs_asc(result),
                Natural::from_limbs_asc(&a_old[..len])
                    .add_mul(Natural::from_owned_limbs_asc(b), Natural::from(c))
            );
            assert_eq!(&a[len..], &a_old[len..]);
        },
    );
}

#[test]
fn limbs_slice_add_mul_limb_same_length_in_place_right_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12().test_properties_with_config(
        &config,
        |(a, mut b, c)| {
            let b_old = b.clone();
            let carry = limbs_slice_add_mul_limb_same_length_in_place_right(&a, &mut b, c);
            b.push(carry);
            assert_eq!(
                Natural::from_owned_limbs_asc(b),
                Natural::from_owned_limbs_asc(a)
                    .add_mul(Natural::from_owned_limbs_asc(b_old), Natural::from(c))
            );
        },
    );
}

#[test]
fn limbs_vec_add_mul_limb_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10().test_properties_with_config(
        &config,
        |(mut a, b, c)| {
            let a_old = a.clone();
            limbs_vec_add_mul_limb_in_place_left(&mut a, &b, c);
            assert_eq!(
                a,
                Natural::from_owned_limbs_asc(a_old)
                    .add_mul(Natural::from_owned_limbs_asc(b), Natural::from(c))
                    .into_limbs_asc()
            );
        },
    );
}

#[test]
fn limbs_vec_add_mul_limb_in_place_right_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10().test_properties_with_config(
        &config,
        |(a, mut b, c)| {
            let b_old = b.clone();
            limbs_vec_add_mul_limb_in_place_right(&a, &mut b, c);
            assert_eq!(
                b,
                Natural::from_owned_limbs_asc(a)
                    .add_mul(Natural::from_owned_limbs_asc(b_old), Natural::from(c))
                    .into_limbs_asc()
            );
        },
    );
}

#[test]
fn limbs_vec_add_mul_limb_in_place_either_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_10().test_properties_with_config(
        &config,
        |(mut a, mut b, c)| {
            let a_old = a.clone();
            let b_old = b.clone();
            let result = if limbs_vec_add_mul_limb_in_place_either(&mut a, &mut b, c) {
                assert_eq!(a_old, a);
                b
            } else {
                assert_eq!(b_old, b);
                a
            };
            assert_eq!(
                result,
                Natural::from_owned_limbs_asc(a_old)
                    .add_mul(Natural::from_owned_limbs_asc(b_old), Natural::from(c))
                    .into_limbs_asc()
            );
        },
    );
}

#[test]
fn limbs_add_mul_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_41().test_properties_with_config(&config, |(a, b, c)| {
        assert_eq!(
            limbs_add_mul(&a, &b, &c),
            Natural::from_owned_limbs_asc(a)
                .add_mul(
                    Natural::from_owned_limbs_asc(b),
                    Natural::from_owned_limbs_asc(c)
                )
                .into_limbs_asc()
        );
    });
}

#[test]
fn limbs_add_mul_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_41().test_properties_with_config(&config, |(mut a, b, c)| {
        let a_old = a.clone();
        limbs_add_mul_in_place_left(&mut a, &b, &c);
        assert_eq!(
            a,
            Natural::from_owned_limbs_asc(a_old)
                .add_mul(
                    Natural::from_owned_limbs_asc(b),
                    Natural::from_owned_limbs_asc(c)
                )
                .into_limbs_asc()
        );
    });
}

#[test]
fn add_mul_properties() {
    natural_triple_gen().test_properties(|(a, b, c)| {
        let mut mut_a = a.clone();
        mut_a.add_mul_assign(b.clone(), c.clone());
        assert!(mut_a.is_valid());
        let result = mut_a;

        let mut mut_a = a.clone();
        mut_a.add_mul_assign(b.clone(), &c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.add_mul_assign(&b, c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.add_mul_assign(&b, &c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let result_alt = a.clone().add_mul(b.clone(), c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().add_mul(b.clone(), &c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().add_mul(&b, c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().add_mul(&b, &c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = (&a).add_mul(&b, &c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!(&a + &b * &c, result);
        assert_eq!(a.add_mul(c, b), result);
    });

    natural_pair_gen().test_properties(|(a, b)| {
        let a = &a;
        let b = &b;
        assert_eq!(a.add_mul(&Natural::ZERO, b), *a);
        assert_eq!(a.add_mul(&Natural::ONE, b), a + b);
        assert_eq!(Natural::ZERO.add_mul(a, b), a * b);
        assert_eq!(a.add_mul(b, &Natural::ZERO), *a);
        assert_eq!(a.add_mul(b, &Natural::ONE), a + b);
    });

    unsigned_triple_gen_var_19::<Limb>().test_properties(|(x, y, z)| {
        let result = Natural::from(x).add_mul(Natural::from(y), Natural::from(z));
        assert_eq!(
            x.checked_add_mul(y, z).is_some(),
            Limb::convertible_from(&result)
        );
    });

    unsigned_triple_gen_var_1::<Limb>().test_properties(|(x, y, z)| {
        assert_eq!(
            x.add_mul(y, z),
            Natural::from(x).add_mul(Natural::from(y), Natural::from(z))
        );
    });
}
