// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CheckedSub, CheckedSubMul, SubMul, SubMulAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_triple_gen_var_2, unsigned_vec_triple_gen_var_59,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12,
};
use malachite_nz::natural::arithmetic::sub_mul::{
    limbs_sub_mul, limbs_sub_mul_in_place_left, limbs_sub_mul_limb_greater,
    limbs_sub_mul_limb_greater_in_place_left, limbs_sub_mul_limb_greater_in_place_right,
    limbs_sub_mul_limb_same_length_in_place_left, limbs_sub_mul_limb_same_length_in_place_right,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_gen, natural_pair_gen, natural_pair_gen_var_10, natural_triple_gen_var_7,
};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_mul_limb_greater() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], limb: Limb, result: &[Limb], borrow| {
        let o_result = limbs_sub_mul_limb_greater(xs_before, ys_before, limb);
        if borrow == 0 {
            assert_eq!(o_result.unwrap(), result);
        } else {
            assert!(o_result.is_none());
        }
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_sub_mul_limb_greater_in_place_left(&mut xs, ys_before, limb),
            borrow
        );
        assert_eq!(xs, result);
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_sub_mul_limb_greater_in_place_right(xs_before, &mut ys, limb),
            borrow
        );
        assert_eq!(ys, result);
    };
    test(&[], &[], 4, &[], 0);
    test(&[123, 456], &[], 4, &[123, 456], 0);
    test(&[123, 456], &[123], 0, &[123, 456], 0);
    test(&[123, 456], &[123], 4, &[4294966927, 455], 0);
    test(&[123, 456], &[123], u32::MAX, &[246, 333], 0);
    test(&[123, 456], &[0, 123], u32::MAX, &[123, 579], 123);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_limb_greater_fail() {
    limbs_sub_mul_limb_greater(&[10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_limb_greater_in_place_left_fail() {
    limbs_sub_mul_limb_greater_in_place_left(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_limb_greater_in_place_right_fail() {
    limbs_sub_mul_limb_greater_in_place_right(&[10], &mut vec![10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_mul_limb_same_length() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], limb: Limb, result: &[Limb], borrow| {
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_sub_mul_limb_same_length_in_place_left(&mut xs, ys_before, limb),
            borrow
        );
        assert_eq!(xs, result);
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_sub_mul_limb_same_length_in_place_right(xs_before, &mut ys, limb),
            borrow
        );
        assert_eq!(ys, result);
    };
    test(&[], &[], 4, &[], 0);
    test(&[123, 456], &[0, 0], 4, &[123, 456], 0);
    test(&[123, 456], &[123, 0], 0, &[123, 456], 0);
    test(&[123, 456], &[123, 0], 4, &[4294966927, 455], 0);
    test(&[123, 456], &[123, 0], u32::MAX, &[246, 333], 0);
    test(&[123, 456], &[0, 123], u32::MAX, &[123, 579], 123);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_limb_same_length_in_place_left_fail() {
    limbs_sub_mul_limb_same_length_in_place_left(&mut [10, 10], &[10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_limb_same_length_in_place_right_fail() {
    limbs_sub_mul_limb_same_length_in_place_right(&[10, 10], &mut [10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_mul_and_limbs_sub_mul_in_place_left() {
    let test = |xs_before: &[Limb], ys: &[Limb], zs: &[Limb], result: Option<Vec<Limb>>| {
        assert_eq!(limbs_sub_mul(xs_before, ys, zs), result);
        let mut xs = xs_before.to_vec();
        let result_alt = if limbs_sub_mul_in_place_left(&mut xs, ys, zs) {
            None
        } else {
            Some(xs)
        };
        assert_eq!(result, result_alt);
    };
    test(&[123, 456, 789], &[123, 789], &[321, 654], None);
    test(
        &[123, 456, 789, 1],
        &[123, 789],
        &[321, 654],
        Some(vec![4294927936, 4294634040, 4294452078, 0]),
    );
    test(
        &[123, 456, 789, 987, 654],
        &[123, 789],
        &[321, 654],
        Some(vec![4294927936, 4294634040, 4294452078, 986, 654]),
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_fail_1() {
    limbs_sub_mul(&[10, 10, 10], &[10], &[10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_fail_2() {
    limbs_sub_mul(&[10, 10, 10], &[10, 10], &[10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_fail_3() {
    limbs_sub_mul(&[10, 10], &[10, 10], &[10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_in_place_left_fail_1() {
    let xs = &mut [10, 10, 10];
    limbs_sub_mul_in_place_left(xs, &[10], &[10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_in_place_left_fail_2() {
    let xs = &mut [10, 10, 10];
    limbs_sub_mul_in_place_left(xs, &[10, 10], &[10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_in_place_left_fail_3() {
    let xs = &mut [10, 10];
    limbs_sub_mul_in_place_left(xs, &[10, 10], &[10, 10]);
}

#[test]
fn test_sub_mul() {
    let test = |r, s, t, out: &str| {
        let u = Natural::from_str(r).unwrap();
        let v = Natural::from_str(s).unwrap();
        let w = Natural::from_str(t).unwrap();

        let mut n = u.clone();
        n.sub_mul_assign(v.clone(), w.clone());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = u.clone();
        n.sub_mul_assign(v.clone(), &w);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = u.clone();
        n.sub_mul_assign(&v, w.clone());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = u.clone();
        n.sub_mul_assign(&v, &w);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = u.clone().sub_mul(v.clone(), w.clone());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = u.clone().sub_mul(v.clone(), &w);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = u.clone().sub_mul(&v, w.clone());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = u.clone().sub_mul(&v, &w);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = (&u).sub_mul(&v, &w);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0", "0");
    test("0", "0", "123", "0");
    test("123", "0", "5", "123");
    test("123", "5", "1", "118");
    test("15", "3", "4", "3");
    test("1000000000000", "0", "123", "1000000000000");
    test("1000000000000", "1", "123", "999999999877");
    test("1000000000000", "123", "1", "999999999877");
    test("1000000000000", "123", "100", "999999987700");
    test("1000000000000", "100", "123", "999999987700");
    test("1000000000000", "65536", "65536", "995705032704");
    test("1000000000000", "1000000000000", "0", "1000000000000");
    test("1000000000000", "1000000000000", "1", "0");
    test("4294967296", "1", "1", "4294967295");
    test(
        "1000000000000000000000000",
        "1000000000000",
        "1000000000000",
        "0",
    );
    test(
        "1000000000001000000000000",
        "1000000000000",
        "1000000000000",
        "1000000000000",
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(
        Natural::from_str("5").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(
        Natural::from_str("1000000000000").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_val_ref_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(
        Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_val_ref_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(
        Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_ref_val_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(
        &Natural::from_str("5").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_ref_val_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(
        &Natural::from_str("1000000000000").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_ref_ref_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(
        &Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_ref_ref_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(
        &Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_fail_1() {
    Natural::from_str("123").unwrap().sub_mul(
        Natural::from_str("5").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_fail_2() {
    Natural::from_str("1000000000000").unwrap().sub_mul(
        Natural::from_str("1000000000000").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_val_ref_fail_1() {
    Natural::from_str("123").unwrap().sub_mul(
        Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_val_ref_fail_2() {
    Natural::from_str("1000000000000").unwrap().sub_mul(
        Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_ref_val_fail_1() {
    Natural::from_str("123").unwrap().sub_mul(
        &Natural::from_str("5").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_ref_val_fail_2() {
    Natural::from_str("1000000000000").unwrap().sub_mul(
        &Natural::from_str("1000000000000").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_ref_ref_fail_1() {
    Natural::from_str("123").unwrap().sub_mul(
        &Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_ref_ref_fail_2() {
    Natural::from_str("1000000000000").unwrap().sub_mul(
        &Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_ref_ref_ref_fail_1() {
    (&Natural::from_str("123").unwrap()).sub_mul(
        &Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_ref_ref_ref_fail_2() {
    (&Natural::from_str("1000000000000").unwrap()).sub_mul(
        &Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
fn limbs_sub_mul_limb_greater_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1().test_properties_with_config(
        &config,
        |(xs, ys, z)| {
            assert_eq!(
                limbs_sub_mul_limb_greater(&xs, &ys, z).map(Natural::from_owned_limbs_asc),
                Natural::from_owned_limbs_asc(xs)
                    .checked_sub_mul(Natural::from_owned_limbs_asc(ys), Natural::from(z))
            );
        },
    );
}

fn limbs_sub_mul_limb_in_place_left_helper(
    f: &mut dyn FnMut(&mut [Limb], &[Limb], Limb) -> Limb,
    mut xs: Vec<Limb>,
    ys: Vec<Limb>,
    z: Limb,
) {
    let xs_old = xs.clone();
    let borrow = f(&mut xs, &ys, z);
    if borrow == 0 {
        assert_eq!(
            Natural::from_owned_limbs_asc(xs),
            Natural::from_owned_limbs_asc(xs_old)
                .sub_mul(Natural::from_owned_limbs_asc(ys), Natural::from(z))
        );
    } else {
        let mut extended_xs = xs_old;
        extended_xs.push(0);
        extended_xs.push(1);
        let mut expected_xs = Natural::from_owned_limbs_asc(extended_xs)
            .sub_mul(Natural::from_owned_limbs_asc(ys), Natural::from(z))
            .into_limbs_asc();
        assert_eq!(expected_xs.pop().unwrap(), borrow.wrapping_neg());
        assert_eq!(xs, expected_xs);
    }
}

#[test]
fn limbs_sub_mul_limb_same_length_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12().test_properties_with_config(
        &config,
        |(xs, ys, z)| {
            limbs_sub_mul_limb_in_place_left_helper(
                &mut limbs_sub_mul_limb_same_length_in_place_left,
                xs,
                ys,
                z,
            );
        },
    );
}

#[test]
fn limbs_sub_mul_limb_greater_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1().test_properties_with_config(
        &config,
        |(xs, ys, z)| {
            limbs_sub_mul_limb_in_place_left_helper(
                &mut limbs_sub_mul_limb_greater_in_place_left,
                xs,
                ys,
                z,
            );
        },
    );
}

macro_rules! limbs_sub_mul_limb_in_place_right_helper {
    ($f: ident, $xs: ident, $ys: ident, $z: ident) => {{
        let ys_old = $ys.clone();
        let borrow = $f(&$xs, &mut $ys, $z);
        if borrow == 0 {
            assert_eq!(
                Natural::from_owned_limbs_asc($ys),
                Natural::from_owned_limbs_asc($xs)
                    .sub_mul(Natural::from_owned_limbs_asc(ys_old), Natural::from($z))
            );
        } else {
            let mut extended_xs = $xs.clone();
            extended_xs.push(0);
            extended_xs.push(1);
            let mut expected_xs = Natural::from_owned_limbs_asc(extended_xs)
                .sub_mul(Natural::from_owned_limbs_asc(ys_old), Natural::from($z))
                .into_limbs_asc();
            assert_eq!(expected_xs.pop().unwrap(), borrow.wrapping_neg());
            assert_eq!($ys, expected_xs);
        }
    }};
}

#[test]
fn limbs_sub_mul_limb_same_length_in_place_right_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12().test_properties_with_config(
        &config,
        |(xs, mut ys, z)| {
            limbs_sub_mul_limb_in_place_right_helper!(
                limbs_sub_mul_limb_same_length_in_place_right,
                xs,
                ys,
                z
            );
        },
    );
}

#[test]
fn limbs_sub_mul_limb_greater_in_place_right_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_12().test_properties_with_config(
        &config,
        |(xs, mut ys, z)| {
            limbs_sub_mul_limb_in_place_right_helper!(
                limbs_sub_mul_limb_greater_in_place_right,
                xs,
                ys,
                z
            );
        },
    );
}

#[test]
fn limbs_sub_mul_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_59().test_properties_with_config(&config, |(xs, ys, zs)| {
        let expected = limbs_sub_mul(&xs, &ys, &zs).map(Natural::from_owned_limbs_asc);
        assert_eq!(
            expected,
            Natural::from_owned_limbs_asc(xs).checked_sub_mul(
                Natural::from_owned_limbs_asc(ys),
                Natural::from_owned_limbs_asc(zs)
            )
        );
    });
}

#[test]
fn limbs_sub_mul_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_59().test_properties_with_config(&config, |(mut xs, ys, zs)| {
        let xs_old = xs.clone();
        let expected = if limbs_sub_mul_in_place_left(&mut xs, &ys, &zs) {
            None
        } else {
            Some(Natural::from_owned_limbs_asc(xs))
        };
        assert_eq!(
            expected,
            Natural::from_owned_limbs_asc(xs_old).checked_sub_mul(
                Natural::from_owned_limbs_asc(ys),
                Natural::from_owned_limbs_asc(zs)
            )
        );
    });
}

#[allow(clippy::useless_conversion)]
#[test]
fn sub_mul_properties() {
    natural_triple_gen_var_7().test_properties(|(a, b, c)| {
        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(&b, &c);
        assert!(mut_a.is_valid());
        let result = mut_a;

        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(&b, c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b.clone(), &c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b.clone(), c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let result_alt = (&a).sub_mul(&b, &c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().sub_mul(&b, &c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().sub_mul(&b, c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().sub_mul(b.clone(), &c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().sub_mul(b.clone(), c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!(&a - &b * &c, result);
        assert_eq!((&a).checked_sub(&b * &c), Some(result));
    });

    natural_gen().test_properties(|n| {
        assert_eq!((&n).sub_mul(&n, &Natural::ONE), 0);
    });

    natural_pair_gen().test_properties(|(a, b)| {
        assert_eq!((&a).sub_mul(&Natural::ZERO, &b), a);
        assert_eq!((&a).sub_mul(&b, &Natural::ZERO), a);
        assert_eq!((&a * &b).sub_mul(a, b), 0);
    });

    natural_pair_gen_var_10().test_properties(|(a, b)| {
        assert_eq!((&a).sub_mul(&Natural::ZERO, &b), a);
        assert_eq!((&a).sub_mul(&b, &Natural::ZERO), a);
        assert_eq!((&a * &b).sub_mul(a, b), 0);
    });

    unsigned_triple_gen_var_2::<Limb>().test_properties(|(x, y, z)| {
        assert_eq!(
            Limb::from(x).sub_mul(Limb::from(y), Limb::from(z)),
            Natural::from(x).sub_mul(Natural::from(y), Natural::from(z))
        );
    });
}
