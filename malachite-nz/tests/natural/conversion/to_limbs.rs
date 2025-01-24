// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::common::test_double_ended_iterator_size_hint;
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    natural_bool_vec_pair_gen_var_1, natural_gen, natural_unsigned_pair_gen_var_4,
};
#[cfg(feature = "32_bit_limbs")]
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_to_limbs_asc() {
    let test = |n, out| {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(n.limbs().collect_vec(), out);
        assert_eq!(n.to_limbs_asc(), out);
        assert_eq!(n.into_limbs_asc(), out);
    };
    test("0", vec![]);
    test("123", vec![123]);
    test("1000000000000", vec![3567587328, 232]);
    test(
        "1701411834921604967429270619762735448065",
        vec![1, 2, 3, 4, 5],
    );
    test("4294967295", vec![u32::MAX]);
    test("4294967296", vec![0, 1]);
    test("18446744073709551615", vec![u32::MAX, u32::MAX]);
    test("18446744073709551616", vec![0, 0, 1]);

    let n = Natural::from_str("1701411834921604967429270619762735448065").unwrap();
    let mut limbs = n.limbs();
    assert_eq!(Some(1), limbs.next());
    assert_eq!(Some(5), limbs.next_back());
    assert_eq!(Some(4), limbs.next_back());
    assert_eq!(Some(2), limbs.next());
    assert_eq!(Some(3), limbs.next());
    assert_eq!(None, limbs.next());
    assert_eq!(None, limbs.next_back());

    assert_eq!(limbs[0], 1);
    assert_eq!(limbs[1], 2);
    assert_eq!(limbs[2], 3);
    assert_eq!(limbs[3], 4);
    assert_eq!(limbs[4], 5);
    assert_eq!(limbs[5], 0);

    let mut limbs = n.limbs();
    assert_eq!(Some(1), limbs.next());
    assert_eq!(Some(2), limbs.next());
    assert_eq!(Some(3), limbs.next());
    assert_eq!(Some(5), limbs.next_back());
    assert_eq!(Some(4), limbs.next_back());
    assert_eq!(None, limbs.next());
    assert_eq!(None, limbs.next_back());
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_to_limbs_desc() {
    let test = |n, out| {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(n.limbs().rev().collect_vec(), out);
        assert_eq!(n.to_limbs_desc(), out);
        assert_eq!(n.into_limbs_desc(), out);
    };
    test("0", vec![]);
    test("123", vec![123]);
    test("1000000000000", vec![232, 3567587328]);
    test(
        "1701411834921604967429270619762735448065",
        vec![5, 4, 3, 2, 1],
    );
    test("4294967295", vec![u32::MAX]);
    test("4294967296", vec![1, 0]);
    test("18446744073709551615", vec![u32::MAX, u32::MAX]);
    test("18446744073709551616", vec![1, 0, 0]);
}

#[test]
fn to_limbs_asc_properties() {
    natural_gen().test_properties(|x| {
        let limbs = x.to_limbs_asc();
        assert_eq!(x.clone().into_limbs_asc(), limbs);
        assert_eq!(x.limbs().collect_vec(), limbs);
        assert_eq!(Natural::from_limbs_asc(&limbs), x);
        if x != 0 {
            assert_ne!(*limbs.last().unwrap(), 0);
        }
    });
}

#[test]
fn to_limbs_desc_properties() {
    natural_gen().test_properties(|x| {
        let limbs = x.to_limbs_desc();
        assert_eq!(x.clone().into_limbs_desc(), limbs);
        assert_eq!(x.limbs().rev().collect_vec(), limbs);
        assert_eq!(Natural::from_limbs_desc(&limbs), x);
        if x != 0 {
            assert_ne!(limbs[0], 0);
        }
    });
}

#[test]
fn limbs_properties() {
    natural_gen().test_properties(|n| {
        test_double_ended_iterator_size_hint(n.limbs(), usize::exact_from(n.limb_count()));
    });

    natural_bool_vec_pair_gen_var_1().test_properties(|(n, bs)| {
        let mut limbs = n.limbs();
        let mut limb_vec = Vec::new();
        let mut i = 0;
        for b in bs {
            if b {
                limb_vec.insert(i, limbs.next().unwrap());
                i += 1;
            } else {
                limb_vec.insert(i, limbs.next_back().unwrap());
            }
        }
        assert!(limbs.next().is_none());
        assert!(limbs.next_back().is_none());
        assert_eq!(n.to_limbs_asc(), limb_vec);
    });

    natural_unsigned_pair_gen_var_4().test_properties(|(n, u)| {
        if u < usize::exact_from(n.limb_count()) {
            assert_eq!(n.limbs()[u], n.to_limbs_asc()[u]);
        } else {
            assert_eq!(n.limbs()[u], 0);
        }
    });

    unsigned_gen_var_5().test_properties(|u| {
        assert_eq!(Natural::ZERO.limbs()[u], 0);
    });
}
