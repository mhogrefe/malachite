// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::vecs::vec_delete_left;
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;
use std::cmp::Ordering::*;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_from_twos_complement_limbs_asc() {
    let test = |xs: &[Limb], out| {
        let x = Integer::from_twos_complement_limbs_asc(xs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        let x = Integer::from_owned_twos_complement_limbs_asc(xs.to_vec());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(&[], "0");
    test(&[0], "0");
    test(&[0, 0, 0], "0");
    test(&[123], "123");
    test(&[123, 0], "123");
    test(&[4294967173], "-123");
    test(&[4294967173, u32::MAX], "-123");
    test(&[3567587328, 232], "1000000000000");
    test(&[727379968, 4294967063], "-1000000000000");
    test(&[1, 2, 3, 4, 5], "1701411834921604967429270619762735448065");
    test(
        &[u32::MAX, u32::MAX - 2, u32::MAX - 3, u32::MAX - 4, u32::MAX - 5],
        "-1701411834921604967429270619762735448065",
    );
    test(&[u32::MAX, 0], "4294967295");
    test(&[1, u32::MAX], "-4294967295");
    test(&[0, 1], "4294967296");
    test(&[0, u32::MAX], "-4294967296");
    test(&[u32::MAX, u32::MAX, 0], "18446744073709551615");
    test(&[1, 0, u32::MAX], "-18446744073709551615");
    test(&[0, 0, 1], "18446744073709551616");
    test(&[0, 0, u32::MAX], "-18446744073709551616");
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_from_twos_complement_limbs_desc() {
    let test = |xs: &[Limb], out| {
        let x = Integer::from_twos_complement_limbs_desc(xs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        let x = Integer::from_owned_twos_complement_limbs_desc(xs.to_vec());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(&[], "0");
    test(&[0], "0");
    test(&[0, 0, 0], "0");
    test(&[123], "123");
    test(&[0, 123], "123");
    test(&[4294967173], "-123");
    test(&[u32::MAX, 4294967173], "-123");
    test(&[232, 3567587328], "1000000000000");
    test(&[4294967063, 727379968], "-1000000000000");
    test(&[5, 4, 3, 2, 1], "1701411834921604967429270619762735448065");
    test(
        &[u32::MAX - 5, u32::MAX - 4, u32::MAX - 3, u32::MAX - 2, u32::MAX],
        "-1701411834921604967429270619762735448065",
    );
    test(&[0, u32::MAX], "4294967295");
    test(&[u32::MAX, 1], "-4294967295");
    test(&[1, 0], "4294967296");
    test(&[u32::MAX, 0], "-4294967296");
    test(&[0, u32::MAX, u32::MAX], "18446744073709551615");
    test(&[u32::MAX, 0, 1], "-18446744073709551615");
    test(&[1, 0, 0], "18446744073709551616");
    test(&[u32::MAX, 0, 0], "-18446744073709551616");
}

fn trim_be_limbs(xs: &mut Vec<Limb>) {
    if xs.is_empty() {
        return;
    }
    if xs[0].get_highest_bit() {
        match xs.iter().position(|&limb| limb != Limb::MAX) {
            None => *xs = vec![Limb::MAX],
            Some(i) => {
                let i = if xs[i].get_highest_bit() { i } else { i - 1 };
                vec_delete_left(xs, i);
            }
        }
    } else {
        match xs.iter().position(|&limb| limb != 0) {
            None => xs.clear(),
            Some(i) => {
                let i = if xs[i].get_highest_bit() { i - 1 } else { i };
                vec_delete_left(xs, i);
            }
        }
    }
}

#[test]
fn from_twos_complement_limbs_asc_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen().test_properties_with_config(&config, |xs| {
        let x = Integer::from_twos_complement_limbs_asc(&xs);
        assert_eq!(Integer::from_owned_twos_complement_limbs_asc(xs.clone()), x);
        let mut trimmed_xs = xs.iter().copied().rev().collect_vec();
        trim_be_limbs(&mut trimmed_xs);
        trimmed_xs.reverse();
        assert_eq!(x.to_twos_complement_limbs_asc(), trimmed_xs);
        assert_eq!(
            Integer::from_owned_twos_complement_limbs_desc(xs.iter().copied().rev().collect()),
            x
        );
        if match x.sign() {
            Equal => xs.is_empty(),
            Greater => {
                let last = *xs.last().unwrap();
                !last.get_highest_bit() && (last != 0 || xs[xs.len() - 2].get_highest_bit())
            }
            Less => {
                let last = *xs.last().unwrap();
                last.get_highest_bit()
                    && (last != Limb::MAX || xs.len() <= 1 || !xs[xs.len() - 2].get_highest_bit())
            }
        } {
            assert_eq!(x.to_twos_complement_limbs_asc(), xs);
        }
    });
}

#[test]
fn from_twos_complement_limbs_desc_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen().test_properties_with_config(&config, |xs| {
        let x = Integer::from_twos_complement_limbs_desc(&xs);
        assert_eq!(
            Integer::from_owned_twos_complement_limbs_desc(xs.clone()),
            x
        );
        let mut trimmed_xs = xs.clone();
        trim_be_limbs(&mut trimmed_xs);
        assert_eq!(x.to_twos_complement_limbs_desc(), trimmed_xs);
        assert_eq!(
            Integer::from_owned_twos_complement_limbs_asc(xs.iter().copied().rev().collect()),
            x
        );
        if match x.sign() {
            Equal => xs.is_empty(),
            Greater => {
                let first = xs[0];
                !first.get_highest_bit() && (first != 0 || xs[1].get_highest_bit())
            }
            Less => {
                let first = xs[0];
                first.get_highest_bit()
                    && (first != Limb::MAX || xs.len() <= 1 || !xs[1].get_highest_bit())
            }
        } {
            assert_eq!(x.to_twos_complement_limbs_desc(), xs);
        }
    });
}
