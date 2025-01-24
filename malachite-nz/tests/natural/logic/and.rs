// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::CountOnes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_27, unsigned_vec_pair_gen, unsigned_vec_pair_gen_var_6,
    unsigned_vec_triple_gen_var_31, unsigned_vec_triple_gen_var_32,
    unsigned_vec_unsigned_pair_gen_var_15,
};
use malachite_nz::natural::logic::and::{
    limbs_and, limbs_and_in_place_either, limbs_and_limb, limbs_and_same_length_to_out,
    limbs_and_to_out, limbs_slice_and_in_place_left, limbs_slice_and_same_length_in_place_left,
    limbs_vec_and_in_place_left,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_gen, natural_pair_gen, natural_triple_gen};
use malachite_nz::test_util::natural::logic::and::{natural_and_alt_1, natural_and_alt_2};
use num::BigUint;
use rug;
use std::cmp::max;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_and_limb() {
    let test = |xs: &[Limb], y: Limb, out: Limb| {
        assert_eq!(limbs_and_limb(xs, y), out);
    };
    test(&[6, 7], 2, 2);
    test(&[100, 101, 102], 10, 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
const fn limbs_and_limb_fail() {
    limbs_and_limb(&[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_and_and_limbs_vec_and_in_place_left() {
    let test = |xs_before, ys, out| {
        assert_eq!(limbs_and(xs_before, ys), out);

        let mut xs = xs_before.to_vec();
        limbs_vec_and_in_place_left(&mut xs, ys);
        assert_eq!(xs, out);
    };
    test(&[], &[], vec![]);
    test(&[2], &[3], vec![2]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 0, 1]);
    test(&[6, 7], &[1, 2, 3], vec![0, 2]);
    test(&[1, 2, 3], &[6, 7], vec![0, 2]);
    test(&[100, 101, 102], &[102, 101, 100], vec![100, 101, 100]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_and_same_length_to_out() {
    let test = |xs, ys, out_before: &[Limb], out_after| {
        let mut out = out_before.to_vec();
        limbs_and_same_length_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[], &[], &[0, 0], vec![0, 0]);
    test(&[1, 1, 1], &[1, 2, 3], &[5, 5, 5, 5], vec![1, 0, 1, 5]);
    test(&[6, 7], &[1, 2], &[0, 0], vec![0, 2]);
    test(&[6, 7], &[1, 2], &[10, 10, 10, 10], vec![0, 2, 10, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![100, 101, 100, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_same_length_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    limbs_and_same_length_to_out(&mut out, &[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_same_length_to_out_fail_2() {
    let mut out = vec![10];
    limbs_and_same_length_to_out(&mut out, &[6, 7], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_and_to_out() {
    let test = |xs, ys, out_before: &[Limb], out_after| {
        let mut out = out_before.to_vec();
        limbs_and_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[], &[], &[0, 0], vec![0, 0]);
    test(&[1, 1, 1], &[1, 2, 3], &[5, 5, 5, 5], vec![1, 0, 1, 5]);
    test(&[6, 7], &[1, 2, 3], &[10, 10, 10, 10], vec![0, 2, 0, 10]);
    test(&[1, 2, 3], &[6, 7], &[10, 10, 10, 10], vec![0, 2, 0, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![100, 101, 100, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_to_out_fail() {
    let mut out = vec![10, 10];
    limbs_and_to_out(&mut out, &[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_and_same_length_in_place_left() {
    let test = |xs_before: &[Limb], ys, xs_after| {
        let mut xs = xs_before.to_vec();
        limbs_slice_and_same_length_in_place_left(&mut xs, ys);
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], vec![]);
    test(&[6, 7], &[1, 2], vec![0, 2]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 0, 1]);
    test(&[100, 101, 102], &[102, 101, 100], vec![100, 101, 100]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_and_same_length_in_place_left_fail() {
    let mut out = vec![6, 7];
    limbs_slice_and_same_length_in_place_left(&mut out, &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_and_in_place_left() {
    let test = |xs_before: &[Limb], ys, truncate_length, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(limbs_slice_and_in_place_left(&mut xs, ys), truncate_length);
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], None, vec![]);
    test(&[6, 7], &[1, 2], None, vec![0, 2]);
    test(&[6, 7], &[1, 2, 3], None, vec![0, 2]);
    test(&[1, 2, 3], &[6, 7], Some(2), vec![0, 2, 3]);
    test(&[], &[1, 2, 3], None, vec![]);
    test(&[1, 2, 3], &[], Some(0), vec![1, 2, 3]);
    test(&[1, 1, 1], &[1, 2, 3], None, vec![1, 0, 1]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        None,
        vec![100, 101, 100],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_and_in_place_either() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], right, xs_after, ys_after| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_and_in_place_either(&mut xs, &mut ys), right);
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[], &[], false, vec![], vec![]);
    test(&[6, 7], &[1, 2], false, vec![0, 2], vec![1, 2]);
    test(&[6, 7], &[1, 2, 3], false, vec![0, 2], vec![1, 2, 3]);
    test(&[1, 2, 3], &[6, 7], true, vec![1, 2, 3], vec![0, 2]);
    test(&[], &[1, 2, 3], false, vec![], vec![1, 2, 3]);
    test(&[1, 2, 3], &[], true, vec![1, 2, 3], vec![]);
    test(&[1, 1, 1], &[1, 2, 3], false, vec![1, 0, 1], vec![1, 2, 3]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![100, 101, 100],
        vec![102, 101, 100],
    );
}

#[test]
fn test_and() {
    let test = |u, v, out| {
        let mut n = Natural::from_str(u).unwrap();
        n &= Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n &= &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() & Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() & Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() & &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() & &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(
            natural_and_alt_1(
                &Natural::from_str(u).unwrap(),
                &Natural::from_str(v).unwrap(),
            )
            .to_string(),
            out
        );
        assert_eq!(
            natural_and_alt_2(
                &Natural::from_str(u).unwrap(),
                &Natural::from_str(v).unwrap(),
            )
            .to_string(),
            out
        );

        let n = BigUint::from_str(u).unwrap() & BigUint::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() & rug::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "0");
    test("123", "0", "0");
    test("123", "456", "72");
    test("1000000000000", "123", "0");
    test("123", "1000000000000", "0");
    test("1000000000001", "123", "1");
    test("12345678987654321", "987654321", "579887281");
    test("1000000000000", "999999999999", "999999995904");
    test("12345678987654321", "314159265358979", "312331665941633");
}

#[test]
fn limbs_and_limb_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_15().test_properties_with_config(&config, |(xs, y)| {
        assert_eq!(
            limbs_and_limb(&xs, y),
            Natural::from_owned_limbs_asc(xs) & Natural::from(y)
        );
    });
}

#[test]
fn limbs_and_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen().test_properties_with_config(&config, |(xs, ys)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_and(&xs, &ys)),
            Natural::from_owned_limbs_asc(xs) & Natural::from_owned_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_and_same_length_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_31().test_properties_with_config(&config, |(mut out, xs, ys)| {
        let out_old = out.clone();
        limbs_and_same_length_to_out(&mut out, &xs, &ys);
        let len = xs.len();
        assert_eq!(
            Natural::from_limbs_asc(&out[..len]),
            Natural::from_owned_limbs_asc(xs) & Natural::from_owned_limbs_asc(ys)
        );
        assert_eq!(&out[len..], &out_old[len..]);
    });
}

#[test]
fn limbs_and_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_32().test_properties_with_config(&config, |(mut out, xs, ys)| {
        let out_old = out.clone();
        limbs_and_to_out(&mut out, &xs, &ys);
        let len = max(xs.len(), ys.len());
        assert_eq!(
            Natural::from_limbs_asc(&out[..len]),
            Natural::from_owned_limbs_asc(xs) & Natural::from_owned_limbs_asc(ys)
        );
        assert_eq!(&out[len..], &out_old[len..]);
    });
}

#[test]
fn limbs_slice_and_same_length_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_6().test_properties_with_config(&config, |(mut xs, ys)| {
        let xs_old = xs.clone();
        limbs_slice_and_same_length_in_place_left(&mut xs, &ys);
        assert_eq!(
            Natural::from_owned_limbs_asc(xs),
            Natural::from_owned_limbs_asc(xs_old) & Natural::from_owned_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_slice_and_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen().test_properties_with_config(&config, |(mut xs, ys)| {
        let xs_old = xs.clone();
        let truncate_size = limbs_slice_and_in_place_left(&mut xs, &ys);
        let n = Natural::from_limbs_asc(&xs_old) & Natural::from_owned_limbs_asc(ys);
        if let Some(truncate_size) = truncate_size {
            assert_eq!(Natural::from_limbs_asc(&xs[..truncate_size]), n);
            assert_eq!(&xs[truncate_size..], &xs_old[truncate_size..]);
        } else {
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
        }
    });
}

#[test]
fn limbs_vec_and_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen().test_properties_with_config(&config, |(mut xs, ys)| {
        let xs_old = xs.clone();
        limbs_vec_and_in_place_left(&mut xs, &ys);
        assert_eq!(
            Natural::from_owned_limbs_asc(xs),
            Natural::from_owned_limbs_asc(xs_old) & Natural::from_owned_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_and_in_place_either_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen().test_properties_with_config(&config, |(mut xs, mut ys)| {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let right = limbs_and_in_place_either(&mut xs, &mut ys);
        let n = Natural::from_limbs_asc(&xs_old) & Natural::from_limbs_asc(&ys_old);
        if right {
            assert_eq!(xs, xs_old);
            assert_eq!(Natural::from_owned_limbs_asc(ys), n);
        } else {
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
            assert_eq!(ys, ys_old);
        }
    });
}

#[allow(clippy::eq_op)]
#[test]
fn and_properties() {
    natural_pair_gen().test_properties(|(x, y)| {
        let result_val_val = x.clone() & y.clone();
        let result_val_ref = x.clone() & &y;
        let result_ref_val = &x & y.clone();
        let result = &x & &y;
        assert!(result_val_val.is_valid());
        assert!(result_val_ref.is_valid());
        assert!(result_ref_val.is_valid());
        assert!(result.is_valid());
        assert_eq!(result_val_val, result);
        assert_eq!(result_val_ref, result);
        assert_eq!(result_ref_val, result);

        let mut mut_x = x.clone();
        mut_x &= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, result);
        let mut mut_x = x.clone();
        mut_x &= &y;
        assert_eq!(mut_x, result);
        assert!(mut_x.is_valid());

        let mut mut_x = rug::Integer::from(&x);
        mut_x &= rug::Integer::from(&y);
        assert_eq!(Natural::exact_from(&mut_x), result);

        assert_eq!(
            Natural::from(&(BigUint::from(&x) & BigUint::from(&y))),
            result
        );
        assert_eq!(
            Natural::exact_from(&(rug::Integer::from(&x) & rug::Integer::from(&y))),
            result
        );

        assert_eq!(natural_and_alt_1(&x, &y), result);
        assert_eq!(natural_and_alt_2(&x, &y), result);

        assert_eq!(&y & &x, result);
        assert_eq!(&result & &x, result);
        assert_eq!(&result & &y, result);

        assert!(result <= x);
        assert!(result <= y);

        let ones = result.count_ones();
        assert!(ones <= x.count_ones());
        assert!(ones <= y.count_ones());
    });

    natural_gen().test_properties(|x| {
        assert_eq!(&x & Natural::ZERO, 0);
        assert_eq!(Natural::ZERO & &x, 0);
        assert_eq!(&x & &x, x);
    });

    natural_triple_gen().test_properties(|(x, y, z)| {
        assert_eq!((&x & &y) & &z, x & (y & z));
    });

    unsigned_pair_gen_var_27::<Limb>().test_properties(|(x, y)| {
        assert_eq!(Natural::from(x) & Natural::from(y), x & y);
    });
}
