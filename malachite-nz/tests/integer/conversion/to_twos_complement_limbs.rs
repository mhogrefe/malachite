// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    signed_gen, unsigned_gen_var_5, unsigned_vec_gen, unsigned_vec_gen_var_2,
};
use malachite_nz::integer::conversion::to_twos_complement_limbs::*;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{
    integer_bool_vec_pair_gen_var_1, integer_gen, integer_unsigned_pair_gen_var_2,
};
use malachite_nz::test_util::integer::conversion::to_twos_complement_limbs::{
    limbs_twos_complement_in_place_alt_1, limbs_twos_complement_in_place_alt_2,
};
use std::cmp::Ordering::*;
#[cfg(feature = "32_bit_limbs")]
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_twos_complement() {
    let test = |xs: &[Limb], out: &[Limb]| {
        assert_eq!(limbs_twos_complement(xs), out);
    };
    test(&[1, 2, 3], &[u32::MAX, 0xfffffffd, 0xfffffffc]);
    test(&[u32::MAX, 0xfffffffd, 0xfffffffc], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_maybe_sign_extend_non_negative_in_place() {
    let test = |xs: &[Limb], out: &[Limb]| {
        let mut mut_xs = xs.to_vec();
        limbs_maybe_sign_extend_non_negative_in_place(&mut mut_xs);
        assert_eq!(mut_xs, out);
    };
    test(&[], &[]);
    test(&[1, 2, 3], &[1, 2, 3]);
    test(&[1, 2, u32::MAX], &[1, 2, u32::MAX, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_twos_complement_in_place() {
    let test = |xs: &[Limb], out: &[Limb], carry: bool| {
        let mut mut_xs = xs.to_vec();
        assert_eq!(limbs_twos_complement_in_place(&mut mut_xs), carry);
        assert_eq!(mut_xs, out);

        let mut mut_xs = xs.to_vec();
        assert_eq!(limbs_twos_complement_in_place_alt_1(&mut mut_xs), carry);
        assert_eq!(mut_xs, out);

        let mut mut_xs = xs.to_vec();
        assert_eq!(limbs_twos_complement_in_place_alt_2(&mut mut_xs), carry);
        assert_eq!(mut_xs, out);
    };
    test(&[], &[], true);
    test(&[1, 2, 3], &[u32::MAX, 0xfffffffd, 0xfffffffc], false);
    test(&[u32::MAX, 0xfffffffd, 0xfffffffc], &[1, 2, 3], false);
    test(&[0, 0, 0], &[0, 0, 0], true);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_twos_complement_and_maybe_sign_extend_negative_in_place() {
    let test = |xs: &[Limb], out: &[Limb]| {
        let mut mut_xs = xs.to_vec();
        limbs_twos_complement_and_maybe_sign_extend_negative_in_place(&mut mut_xs);
        assert_eq!(mut_xs, out);
    };
    test(&[1, 2, 3], &[u32::MAX, 0xfffffffd, 0xfffffffc]);
    test(&[u32::MAX, 0xfffffffd, 0xfffffffc], &[1, 2, 3, u32::MAX]);
    test(&[0, u32::MAX], &[0, 1, u32::MAX]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_twos_complement_and_maybe_sign_extend_negative_in_place_fail() {
    let mut mut_xs = vec![0, 0, 0];
    limbs_twos_complement_and_maybe_sign_extend_negative_in_place(&mut mut_xs);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_twos_complement_limbs_asc() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();

        assert_eq!(u.twos_complement_limbs().collect_vec(), out);
        assert_eq!(u.to_twos_complement_limbs_asc(), out);
        assert_eq!(u.into_twos_complement_limbs_asc(), out);
    };
    test("0", vec![]);
    test("123", vec![123]);
    test("-123", vec![4294967173]);
    test("1000000000000", vec![3567587328, 232]);
    test("-1000000000000", vec![727379968, 4294967063]);
    test(
        "1701411834921604967429270619762735448065",
        vec![1, 2, 3, 4, 5],
    );
    test(
        "-1701411834921604967429270619762735448065",
        vec![u32::MAX, u32::MAX - 2, u32::MAX - 3, u32::MAX - 4, u32::MAX - 5],
    );
    test("2147483647", vec![2147483647]);
    test("-2147483647", vec![2147483649]);
    test("2147483648", vec![2147483648, 0]);
    test("-2147483648", vec![2147483648]);
    test("2147483649", vec![2147483649, 0]);
    test("-2147483649", vec![2147483647, 4294967295]);
    test("4294967294", vec![u32::MAX - 1, 0]);
    test("-4294967294", vec![2, u32::MAX]);
    test("4294967295", vec![u32::MAX, 0]);
    test("-4294967295", vec![1, u32::MAX]);
    test("4294967296", vec![0, 1]);
    test("-4294967296", vec![0, u32::MAX]);
    test("18446744073709551615", vec![u32::MAX, u32::MAX, 0]);
    test("-18446744073709551615", vec![1, 0, u32::MAX]);
    test("18446744073709551616", vec![0, 0, 1]);
    test("-18446744073709551616", vec![0, 0, u32::MAX]);

    let n = Integer::from_str("-1701411834921604967429270619762735448065").unwrap();
    let mut limbs = n.twos_complement_limbs();
    assert_eq!(Some(u32::MAX), limbs.next());
    assert_eq!(Some(u32::MAX - 5), limbs.next_back());
    assert_eq!(Some(u32::MAX - 4), limbs.next_back());
    assert_eq!(Some(u32::MAX - 2), limbs.next());
    assert_eq!(Some(u32::MAX - 3), limbs.next());
    assert_eq!(None, limbs.next());
    assert_eq!(None, limbs.next_back());

    let limbs = n.twos_complement_limbs();
    assert_eq!(limbs.get(0), u32::MAX);
    assert_eq!(limbs.get(1), u32::MAX - 2);
    assert_eq!(limbs.get(2), u32::MAX - 3);
    assert_eq!(limbs.get(3), u32::MAX - 4);
    assert_eq!(limbs.get(4), u32::MAX - 5);
    assert_eq!(limbs.get(5), u32::MAX);

    let mut limbs = n.twos_complement_limbs();
    assert_eq!(Some(u32::MAX), limbs.next());
    assert_eq!(Some(u32::MAX - 2), limbs.next());
    assert_eq!(Some(u32::MAX - 3), limbs.next());
    assert_eq!(Some(u32::MAX - 5), limbs.next_back());
    assert_eq!(Some(u32::MAX - 4), limbs.next_back());
    assert_eq!(None, limbs.next());
    assert_eq!(None, limbs.next_back());
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_twos_complement_limbs_desc() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();

        assert_eq!(u.to_twos_complement_limbs_desc(), out);
        assert_eq!(u.into_twos_complement_limbs_desc(), out);
    };
    test("0", vec![]);
    test("123", vec![123]);
    test("-123", vec![4294967173]);
    test("1000000000000", vec![232, 3567587328]);
    test("-1000000000000", vec![4294967063, 727379968]);
    test(
        "1701411834921604967429270619762735448065",
        vec![5, 4, 3, 2, 1],
    );
    test(
        "-1701411834921604967429270619762735448065",
        vec![u32::MAX - 5, u32::MAX - 4, u32::MAX - 3, u32::MAX - 2, u32::MAX],
    );
    test("4294967295", vec![0, u32::MAX]);
    test("-4294967295", vec![u32::MAX, 1]);
    test("4294967296", vec![1, 0]);
    test("-4294967296", vec![u32::MAX, 0]);
    test("18446744073709551615", vec![0, u32::MAX, u32::MAX]);
    test("-18446744073709551615", vec![u32::MAX, 0, 1]);
    test("18446744073709551616", vec![1, 0, 0]);
    test("-18446744073709551616", vec![u32::MAX, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_twos_complement_limb_count() {
    let test = |n, out| {
        assert_eq!(
            Integer::from_str(n).unwrap().twos_complement_limb_count(),
            out
        );
    };
    test("0", 0);
    test("123", 1);
    test("-123", 1);
    test("1000000000000", 2);
    test("-1000000000000", 2);
    test("1701411834921604967429270619762735448065", 5);
    test("-1701411834921604967429270619762735448065", 5);
    test("2147483647", 1);
    test("-2147483647", 1);
    test("2147483648", 2);
    test("-2147483648", 1);
    test("2147483649", 2);
    test("-2147483649", 2);
    test("4294967294", 2);
    test("-4294967294", 2);
    test("4294967295", 2);
    test("-4294967295", 2);
    test("4294967296", 2);
    test("-4294967296", 2);
    test("18446744073709551615", 3);
    test("-18446744073709551615", 3);
    test("18446744073709551616", 3);
    test("-18446744073709551616", 3);
}

#[test]
fn limbs_twos_complement_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen_var_2().test_properties_with_config(&config, |xs| {
        let out_xs = limbs_twos_complement(&xs);
        if *xs.last().unwrap() != 0 && out_xs.last().unwrap().get_highest_bit() {
            let n = -Natural::from_limbs_asc(&xs);
            assert_eq!(n.to_twos_complement_limbs_asc(), out_xs);
        }
    });
}

#[test]
fn limbs_maybe_sign_extend_non_negative_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen().test_properties_with_config(&config, |xs| {
        let mut mut_xs = xs.clone();
        limbs_maybe_sign_extend_non_negative_in_place(&mut mut_xs);
        if !xs.is_empty() && *xs.last().unwrap() != 0 {
            let n = Integer::from(Natural::from_owned_limbs_asc(xs));
            assert_eq!(n.to_twos_complement_limbs_asc(), mut_xs);
        }
    });
}

#[test]
fn limbs_twos_complement_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen().test_properties_with_config(&config, |xs| {
        let mut mut_xs = xs.clone();
        limbs_twos_complement_in_place(&mut mut_xs);
        let mut mut_xs_alt = xs.clone();
        limbs_twos_complement_in_place_alt_1(&mut mut_xs_alt);
        assert_eq!(mut_xs_alt, mut_xs);
        let mut mut_xs_alt = xs.clone();
        limbs_twos_complement_in_place_alt_2(&mut mut_xs_alt);
        assert_eq!(mut_xs_alt, mut_xs);
        if !xs.is_empty() && *xs.last().unwrap() != 0 && mut_xs.last().unwrap().get_highest_bit() {
            let n = -Natural::from_owned_limbs_asc(xs);
            assert_eq!(n.to_twos_complement_limbs_asc(), mut_xs);
        }
    });
}

#[test]
fn limbs_twos_complement_and_maybe_sign_extend_negative_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen_var_2().test_properties_with_config(&config, |xs| {
        let mut mut_xs = xs.clone();
        limbs_twos_complement_and_maybe_sign_extend_negative_in_place(&mut mut_xs);
        if !xs.is_empty() && *xs.last().unwrap() != 0 {
            let n = -Natural::from_owned_limbs_asc(xs);
            assert_eq!(n.to_twos_complement_limbs_asc(), mut_xs);
        }
    });
}

#[test]
fn to_twos_complement_limbs_asc_properties() {
    integer_gen().test_properties(|x| {
        let xs = x.to_twos_complement_limbs_asc();
        assert_eq!(xs.len(), usize::exact_from(x.twos_complement_limb_count()));
        assert_eq!(x.clone().into_twos_complement_limbs_asc(), xs);
        assert_eq!(x.twos_complement_limbs().collect_vec(), xs);
        assert_eq!(Integer::from_twos_complement_limbs_asc(&xs), x);
        assert_eq!(
            x.to_twos_complement_limbs_desc(),
            xs.iter().copied().rev().collect_vec()
        );
        match x.sign() {
            Equal => assert!(xs.is_empty()),
            Greater => {
                let last = *xs.last().unwrap();
                assert!(!last.get_highest_bit());
                if last == 0 {
                    assert!(xs[xs.len() - 2].get_highest_bit());
                }
            }
            Less => {
                let last = *xs.last().unwrap();
                assert!(last.get_highest_bit());
                if last == !0 && xs.len() > 1 {
                    assert!(!xs[xs.len() - 2].get_highest_bit());
                }
            }
        }
    });
}

#[test]
fn to_twos_complement_limbs_desc_properties() {
    integer_gen().test_properties(|x| {
        let xs = x.to_twos_complement_limbs_desc();
        assert_eq!(xs.len(), usize::exact_from(x.twos_complement_limb_count()));
        assert_eq!(x.clone().into_twos_complement_limbs_desc(), xs);
        assert_eq!(x.twos_complement_limbs().rev().collect_vec(), xs);
        assert_eq!(Integer::from_twos_complement_limbs_desc(&xs), x);
        assert_eq!(
            x.to_twos_complement_limbs_asc(),
            xs.iter().copied().rev().collect_vec()
        );
        match x.sign() {
            Equal => assert!(xs.is_empty()),
            Greater => {
                let first = xs[0];
                assert!(!first.get_highest_bit());
                if first == 0 {
                    assert!(xs[1].get_highest_bit());
                }
            }
            Less => {
                let first = xs[0];
                assert!(first.get_highest_bit());
                if first == !0 && xs.len() > 1 {
                    assert!(!xs[1].get_highest_bit());
                }
            }
        }
    });
}

#[test]
fn twos_complement_limbs_properties() {
    integer_bool_vec_pair_gen_var_1().test_properties(|(n, bs)| {
        let mut limbs = n.twos_complement_limbs();
        let mut xs = Vec::new();
        let mut i = 0;
        for b in bs {
            if b {
                xs.insert(i, limbs.next().unwrap());
                i += 1;
            } else {
                xs.insert(i, limbs.next_back().unwrap());
            }
        }
        assert!(limbs.next().is_none());
        assert!(limbs.next_back().is_none());
        assert_eq!(n.to_twos_complement_limbs_asc(), xs);
    });

    integer_unsigned_pair_gen_var_2().test_properties(|(n, u)| {
        if u < n.unsigned_abs_ref().limb_count() {
            assert_eq!(
                n.twos_complement_limbs().get(u),
                n.to_twos_complement_limbs_asc()[usize::exact_from(u)]
            );
        } else {
            assert_eq!(
                n.twos_complement_limbs().get(u),
                if n >= 0 { 0 } else { Limb::MAX }
            );
        }
    });

    unsigned_gen_var_5().test_properties(|u| {
        assert_eq!(Integer::ZERO.twos_complement_limbs().get(u), 0);
    });
}

const LIMB_HIGH_BIT: Limb = 1 << (Limb::WIDTH - 1);

#[test]
fn twos_complement_limb_count_properties() {
    integer_gen().test_properties(|x| {
        let n = x.twos_complement_limb_count();
        assert_eq!(
            (x >= 0 && x < LIMB_HIGH_BIT) || (x < 0 && x >= -Integer::from(LIMB_HIGH_BIT)),
            n <= 1
        );
    });

    signed_gen::<SignedLimb>().test_properties(|i| {
        assert!(Integer::from(i).twos_complement_limb_count() <= 1);
    });
}
