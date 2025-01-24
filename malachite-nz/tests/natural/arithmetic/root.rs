// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingRoot, CeilingRootAssign, CeilingSqrt, CheckedRoot, CheckedSqrt, FloorRoot,
    FloorRootAssign, FloorSqrt, Pow, RootAssignRem, RootRem, SqrtRem,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_pair_gen_var_32, unsigned_vec_unsigned_pair_gen_var_14,
};
use malachite_nz::natural::arithmetic::root::{limbs_floor_root, limbs_root_rem};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_gen, natural_unsigned_pair_gen_var_7};
use malachite_nz::test_util::natural::arithmetic::root::{
    ceiling_root_binary, checked_root_binary, floor_root_binary, root_rem_binary,
};
use num::BigUint;
use std::panic::catch_unwind;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_floor_root() {
    let test = |xs: &[Limb], exp: u64, root: &[Limb], inexact: bool| {
        let (actual_root, actual_inexact) = limbs_floor_root(xs, exp);
        assert_eq!(actual_root, root);
        assert_eq!(actual_inexact, inexact);
        let n = Natural::from_limbs_asc(xs);
        let r = Natural::from_limbs_asc(root);
        let pow = (&r).pow(exp);
        assert!(pow <= n);
        assert!((r + Natural::ONE).pow(exp) > n);
        assert_eq!(pow == n, !inexact);
    };
    // - (xs_len + 2) / 3 <= u_exp
    // - bit_count < exp in limbs_root_to_out_internal
    // - !out_rem_is_some in limbs_root_to_out_internal
    test(&[1], 3, &[1], false);
    // - bit_count >= exp in limbs_root_to_out_internal
    // - leading_zeros != Limb::WIDTH in limbs_root_to_out_internal
    // - bit_count.significant_bits() > LOGROOT_USED_BITS_COMP in log_based_root
    // - !approx || ss[0] <= 1 in limbs_root_to_out_internal
    // - need_adjust || qs_len != xs_len in limbs_root_to_out_internal
    // - rs_len == 0 || !out_rem_is_some in limbs_root_to_out_internal
    test(&[1000], 3, &[10], false);
    // - root_bits > log_exp in limbs_root_to_out_internal
    // - need_adjust || qs_len != rs_len in limbs_root_to_out_internal
    // - pow_cmp != Equal in limbs_root_to_out_internal
    // - carry == 0 first time in limbs_root_to_out_internal
    // - n_len - 1 <= next_len in limbs_root_to_out_internal
    // - carry == 0 second time in limbs_root_to_out_internal
    // - rs_len >= ws_len in limbs_root_to_out_internal
    // - qs_len <= b_rem in limbs_root_to_out_internal
    // - qs_len <= b_rem && ... in limbs_root_to_out_internal
    // - carry != 0 first time in limbs_root_to_out_internal
    // - n_len - 1 > next_len in limbs_root_to_out_internal
    test(&[123, 456, 789], 3, &[24415497], true);
    // - leading_zeros == Limb::WIDTH in limbs_root_to_out_internal
    test(&[0, 1], 3, &[1625], true);
    // - pow_cmp == Equal in limbs_root_to_out_internal
    // - rs_len < ws_len in limbs_root_to_out_internal
    test(&[0, 0, 1], 4, &[65536], false);
    // - !need_adjust && qs_len == rs_len in limbs_root_to_out_internal
    // - !need_adjust && qs_len == xs_len in limbs_root_to_out_internal
    // - carry != 0 second time in limbs_root_to_out_internal
    test(&[0, 0, 1], 5, &[7131], true);
    // - (xs_len + 2) / 3 > u_exp
    // - approx && ss[0] > 1 in limbs_root_to_out_internal
    test(
        &[
            10045114, 111940252, 2181719322, 1883679021, 2601294413, 1872079876, 578360935,
            2248016248, 1648448409, 589499551, 573051942, 3101629567, 486103882, 3213846717,
            2339835332, 2340868500, 3988971200,
        ],
        5,
        &[2416867165, 2555201003, 3891828300, 7026],
        true,
    );
    // - qs_len > b_rem || ... in limbs_root_to_out_internal
    test(
        &[
            2055929154, 2630529572, 271121346, 1501542260, 1183697298, 2075827756, 4275724366,
            1648161837, 3297263182, 4114641001, 1962106184, 3607497617, 561001103, 1137290806,
            2335506779, 1869248612,
        ],
        3,
        &[2524001878, 2377965049, 719885555, 160379071, 3624665804, 1231],
        true,
    );
    // - limbs_root_to_out_internal, root_bits <= log_exp
    test(
        &[
            1321882439, 1623785800, 3134073276, 2565564486, 2821610380, 2583585204, 3897897848,
            47587649, 2888164080, 1492585590, 1855797547, 1510761479, 3993330677, 2012682921,
            1836519625, 4236374717, 1223607044, 3596509294, 1741147226, 1412323213, 3811971203,
            1621563690, 3665246834, 1046970441, 99078224, 420931190, 2916287708, 1336157470,
            1469113083, 970862367, 3439357619, 2526884655, 1520990535, 3107383205, 3321150749,
            828096485, 938849804, 164343730, 2130891622, 3754519147, 2436884346, 2736885571,
            405986850, 1875306972, 3010233736, 3737129860, 2106807103, 419975711, 3145892129,
            1185575287, 4062252394, 543504740, 3340440476, 4196738733, 2082551138, 1234502144,
            3392112010, 3994477994, 1472445754, 934569550, 3244797218, 3086244200, 1419671372,
            933654118, 1800407464, 3858653268, 180317861, 1556428454, 3366377290, 1090143530,
            1147295140, 1271219922, 1096608888, 655592159, 1364844184, 1674114515, 1285632542,
            2458523712, 2081876810, 3935363261, 4174442908, 2398083509, 963931968, 4194789136,
            1729071894, 228036838, 2088259023, 2777007731, 2094146444, 888346074, 3199577164,
            2715723581, 3607954173, 3433473090, 3678701040, 1035050400, 2036582500, 3748434538,
            3152072052, 3413508577, 3191081341, 466218778, 4141778140, 2149717819, 2938658940,
            2989191274, 3671957666, 727865845, 568269125, 3615953546, 2956535711, 879106809,
            1582857293, 783679777, 604923241, 209547277, 3813482434, 1362157378, 1505311679,
            2420123937, 4156219100, 2338704513, 1016908906, 2362401070, 125533635,
        ],
        189,
        &[2306151],
        false,
    );
}

#[test]
fn limbs_floor_root_fail() {
    // - xs too short
    assert_panic!(limbs_floor_root(&[], 3));
    // - last element of xs zero
    assert_panic!(limbs_floor_root(&[1, 0], 3));
    // - exp is 0
    assert_panic!(limbs_floor_root(&[1, 1], 0));
    // - exp is 1
    assert_panic!(limbs_floor_root(&[1, 1], 1));
    // - exp is 2
    assert_panic!(limbs_floor_root(&[1, 1], 2));
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_root_rem() {
    let test = |xs: &[Limb], exp: u64, root: &[Limb], rem: &[Limb]| {
        let (actual_root, actual_rem) = limbs_root_rem(xs, exp);
        assert_eq!(actual_root, root);
        assert_eq!(actual_rem, rem);
        let n = Natural::from_limbs_asc(xs);
        let r_1 = Natural::from_limbs_asc(root);
        let r_2 = Natural::from_limbs_asc(rem);
        assert_eq!((&r_1).pow(exp) + r_2, n);
        assert!((r_1 + Natural::ONE).pow(exp) > n);
    };
    // - bit_count < exp in limbs_root_to_out_internal
    // - out_rem_is_some in limbs_root_to_out_internal
    test(&[1], 3, &[1], &[]);
    // - bit_count >= exp in limbs_root_to_out_internal
    // - leading_zeros != Limb::WIDTH in limbs_root_to_out_internal
    // - bit_count.significant_bits() <= LOGROOT_USED_BITS_COMP in log_based_root
    // - !approx || ss[0] <= 1 in limbs_root_to_out_internal
    // - !need_adjust && qs_len == xs_len in limbs_root_to_out_internal
    // - rs_len == 0 || !out_rem_is_some in limbs_root_to_out_internal
    test(&[1000], 3, &[10], &[]);
    // - root_bits > log_exp in limbs_root_to_out_internal
    // - need_adjust || qs_len != rs_len in limbs_root_to_out_internal
    // - pow_cmp != Equal in limbs_root_to_out_internal
    // - carry == 0 first time in limbs_root_to_out_internal
    // - n_len - 1 <= next_len in limbs_root_to_out_internal
    // - carry == 0 second time in limbs_root_to_out_internal
    // - rs_len >= ws_len in limbs_root_to_out_internal
    // - qs_len <= b_rem in limbs_root_to_out_internal
    // - qs_len <= b_rem && ... in limbs_root_to_out_internal
    // - carry != 0 first time in limbs_root_to_out_internal
    // - n_len - 1 > next_len in limbs_root_to_out_internal
    // - rs_len != 0 && out_rem_is_some in limbs_root_to_out_internal
    test(&[123, 456, 789], 3, &[24415497], &[1082861218, 142292]);
    // - leading_zeros == Limb::WIDTH in limbs_root_to_out_internal
    test(&[0, 1], 3, &[1625], &[3951671]);
    // - pow_cmp == Equal in limbs_root_to_out_internal
    // - rs_len < ws_len in limbs_root_to_out_internal
    test(&[0, 0, 1], 4, &[65536], &[]);
    // - !need_adjust && qs_len == rs_len in limbs_root_to_out_internal
    test(&[0, 0, 1], 5, &[7131], &[1889423061, 1656574]);
    // - carry != 0 second time in limbs_root_to_out_internal
    test(
        &[
            10045114, 111940252, 2181719322, 1883679021, 2601294413, 1872079876, 578360935,
            2248016248, 1648448409, 589499551, 573051942, 3101629567, 486103882, 3213846717,
            2339835332, 2340868500, 3988971200,
        ],
        5,
        &[2416867165, 2555201003, 3891828300, 7026],
        &[
            3289703629, 3644089536, 1993609161, 1739315193, 2220455044, 1795995908, 3261364903,
            2481515404, 3316729739, 227499169, 1205565253, 3882526697, 534818167, 1092514,
        ],
    );
    test(
        &[
            2055929154, 2630529572, 271121346, 1501542260, 1183697298, 2075827756, 4275724366,
            1648161837, 3297263182, 4114641001, 1962106184, 3607497617, 561001103, 1137290806,
            2335506779, 1869248612,
        ],
        3,
        &[2524001878, 2377965049, 719885555, 160379071, 3624665804, 1231],
        &[
            1938096298, 2492483757, 416851523, 4009456064, 358434376, 1470400066, 2808049667,
            1641457454, 3086626670, 2101663143, 3655678,
        ],
    );
    // - !need_adjust && qs_len == xs_len in limbs_root_to_out_internal
    test(&[0, 2], 4, &[304], &[49217536]);
    // - qs_len > b_rem || ... in limbs_root_to_out_internal
    test(&[1002403925, 303302627], 4, &[33783], &[188673716, 30155]);
}

#[test]
fn limbs_root_rem_fail() {
    // - xs too short
    assert_panic!(limbs_root_rem(&[], 3));
    // - last element of xs zero
    assert_panic!(limbs_root_rem(&[1, 0], 3));
    // - exp is 0
    assert_panic!(limbs_root_rem(&[1, 1], 0));
    // - exp is 1
    assert_panic!(limbs_root_rem(&[1, 1], 1));
    // - exp is 2
    assert_panic!(limbs_root_rem(&[1, 1], 2));
}

#[test]
fn test_floor_root() {
    let test = |s, exp, out| {
        let n = Natural::from_str(s).unwrap();
        assert_eq!(n.clone().floor_root(exp).to_string(), out);
        assert_eq!((&n).floor_root(exp).to_string(), out);
        assert_eq!(floor_root_binary(&n, exp).to_string(), out);

        let mut n = n;
        n.floor_root_assign(exp);
        assert_eq!(n.to_string(), out);
    };
    test("0", 1, "0");
    test("1", 1, "1");
    test("2", 1, "2");
    test("100", 1, "100");

    test("0", 2, "0");
    test("1", 2, "1");
    test("2", 2, "1");
    test("3", 2, "1");
    test("4", 2, "2");
    test("5", 2, "2");
    test("0", 3, "0");
    test("1", 3, "1");
    test("2", 3, "1");
    test("7", 3, "1");
    test("8", 3, "2");
    test("9", 3, "2");
    test("10", 2, "3");
    test("100", 2, "10");
    test("100", 3, "4");
    test("1000000000", 2, "31622");
    test("1000000000", 3, "1000");
    test("1000000000", 4, "177");
    test("1000000000", 5, "63");
    test("1000000000", 6, "31");
    test("1000000000", 7, "19");
    test("1000000000", 8, "13");
    test("1000000000", 9, "10");
    test("1000000000", 10, "7");
}

#[test]
#[should_panic]
fn floor_root_fail() {
    Natural::ONE.floor_root(0);
}

#[test]
#[should_panic]
fn floor_root_ref_fail() {
    (&Natural::ONE).floor_root(0);
}

#[test]
#[should_panic]
fn floor_root_assign_fail() {
    let mut x = Natural::ONE;
    x.floor_root_assign(0);
}

#[test]
fn test_ceiling_root() {
    let test = |s, exp, out| {
        let n = Natural::from_str(s).unwrap();
        assert_eq!(n.clone().ceiling_root(exp).to_string(), out);
        assert_eq!((&n).ceiling_root(exp).to_string(), out);
        assert_eq!(ceiling_root_binary(&n, exp).to_string(), out);

        let mut n = n;
        n.ceiling_root_assign(exp);
        assert_eq!(n.to_string(), out);
    };
    test("0", 1, "0");
    test("1", 1, "1");
    test("2", 1, "2");
    test("100", 1, "100");

    test("0", 2, "0");
    test("1", 2, "1");
    test("2", 2, "2");
    test("3", 2, "2");
    test("4", 2, "2");
    test("5", 2, "3");
    test("0", 3, "0");
    test("1", 3, "1");
    test("2", 3, "2");
    test("7", 3, "2");
    test("8", 3, "2");
    test("9", 3, "3");
    test("10", 2, "4");
    test("100", 2, "10");
    test("100", 3, "5");
    test("1000000000", 2, "31623");
    test("1000000000", 3, "1000");
    test("1000000000", 4, "178");
    test("1000000000", 5, "64");
    test("1000000000", 6, "32");
    test("1000000000", 7, "20");
    test("1000000000", 8, "14");
    test("1000000000", 9, "10");
    test("1000000000", 10, "8");
}

#[test]
#[should_panic]
fn ceiling_root_fail() {
    Natural::ONE.ceiling_root(0);
}

#[test]
#[should_panic]
fn ceiling_root_ref_fail() {
    (&Natural::ONE).ceiling_root(0);
}

#[test]
#[should_panic]
fn ceiling_root_assign_fail() {
    let mut x = Natural::ONE;
    x.ceiling_root_assign(0);
}

#[allow(clippy::redundant_closure_for_method_calls)]
#[test]
fn test_checked_root() {
    let test = |s, exp, out: Option<&str>| {
        let n = Natural::from_str(s).unwrap();
        let out = out.map(|s| s.to_string());

        assert_eq!(n.clone().checked_root(exp).map(|x| x.to_string()), out);
        assert_eq!((&n).checked_root(exp).map(|x| x.to_string()), out);
        assert_eq!(checked_root_binary(&n, exp).map(|x| x.to_string()), out);
    };
    test("0", 1, Some("0"));
    test("1", 1, Some("1"));
    test("2", 1, Some("2"));
    test("100", 1, Some("100"));

    test("0", 2, Some("0"));
    test("1", 2, Some("1"));
    test("2", 2, None);
    test("3", 2, None);
    test("4", 2, Some("2"));
    test("5", 2, None);
    test("0", 3, Some("0"));
    test("1", 3, Some("1"));
    test("2", 3, None);
    test("7", 3, None);
    test("8", 3, Some("2"));
    test("9", 3, None);
    test("10", 2, None);
    test("100", 2, Some("10"));
    test("100", 3, None);
    test("1000000000", 2, None);
    test("1000000000", 3, Some("1000"));
    test("1000000000", 4, None);
    test("1000000000", 5, None);
    test("1000000000", 6, None);
    test("1000000000", 7, None);
    test("1000000000", 8, None);
    test("1000000000", 9, Some("10"));
    test("1000000000", 10, None);
}

#[test]
#[should_panic]
fn checked_root_fail() {
    Natural::ONE.checked_root(0);
}

#[test]
#[should_panic]
fn checked_root_ref_fail() {
    (&Natural::ONE).checked_root(0);
}

#[test]
fn test_root_rem() {
    let test = |s, exp, root_out, rem_out| {
        let n = Natural::from_str(s).unwrap();

        let (root, rem) = n.clone().root_rem(exp);
        assert_eq!(root.to_string(), root_out);
        assert_eq!(rem.to_string(), rem_out);

        let (root, rem) = (&n).root_rem(exp);
        assert_eq!(root.to_string(), root_out);
        assert_eq!(rem.to_string(), rem_out);

        let (root, rem) = root_rem_binary(&n, exp);
        assert_eq!(root.to_string(), root_out);
        assert_eq!(rem.to_string(), rem_out);

        let mut n = n;
        assert_eq!(n.root_assign_rem(exp).to_string(), rem_out);
        assert_eq!(n.to_string(), root_out);
    };
    test("0", 1, "0", "0");
    test("1", 1, "1", "0");
    test("2", 1, "2", "0");
    test("100", 1, "100", "0");

    test("0", 2, "0", "0");
    test("1", 2, "1", "0");
    test("2", 2, "1", "1");
    test("3", 2, "1", "2");
    test("4", 2, "2", "0");
    test("5", 2, "2", "1");
    test("0", 3, "0", "0");
    test("1", 3, "1", "0");
    test("2", 3, "1", "1");
    test("7", 3, "1", "6");
    test("8", 3, "2", "0");
    test("9", 3, "2", "1");
    test("10", 2, "3", "1");
    test("100", 2, "10", "0");
    test("100", 3, "4", "36");
    test("1000000000", 2, "31622", "49116");
    test("1000000000", 3, "1000", "0");
    test("1000000000", 4, "177", "18493759");
    test("1000000000", 5, "63", "7563457");
    test("1000000000", 6, "31", "112496319");
    test("1000000000", 7, "19", "106128261");
    test("1000000000", 8, "13", "184269279");
    test("1000000000", 9, "10", "0");
    test("1000000000", 10, "7", "717524751");
}

#[test]
#[should_panic]
fn root_rem_fail() {
    Natural::ONE.root_rem(0);
}

#[test]
#[should_panic]
fn root_rem_ref_fail() {
    (&Natural::ONE).root_rem(0);
}

#[test]
#[should_panic]
fn root_assign_rem_fail() {
    let mut x = Natural::ONE;
    x.root_assign_rem(0);
}

#[test]
fn limbs_floor_root_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_14().test_properties_with_config(&config, |(xs, exp)| {
        let n = Natural::from_limbs_asc(&xs);
        let actual_root = (&n).floor_root(exp);
        let (root, inexact) = limbs_floor_root(&xs, exp);
        assert_eq!(Natural::from_owned_limbs_asc(root), actual_root);
        let pow = (&actual_root).pow(exp);
        assert_eq!(pow == n, !inexact);
        assert!(pow <= n);
        assert!((actual_root + Natural::ONE).pow(exp) > n);
    });
}

#[test]
fn limbs_root_rem_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_14().test_properties_with_config(&config, |(xs, exp)| {
        let n = Natural::from_limbs_asc(&xs);
        let (actual_root, actual_rem) = (&n).root_rem(exp);
        let (root, rem) = limbs_root_rem(&xs, exp);
        assert_eq!(Natural::from_owned_limbs_asc(root), actual_root);
        assert_eq!(Natural::from_owned_limbs_asc(rem), actual_rem);
        assert_eq!((&actual_root).pow(exp) + actual_rem, n);
        assert!((actual_root + Natural::ONE).pow(exp) > n);
    });
}

#[test]
fn floor_cbrt_properties() {
    natural_gen().test_properties(|n| {
        let cbrt = n.clone().floor_root(3);
        assert!(cbrt.is_valid());
        let cbrt_alt = (&n).floor_root(3);
        assert!(cbrt_alt.is_valid());
        assert_eq!(cbrt_alt, cbrt);
        let mut n_alt = n.clone();
        n_alt.floor_root_assign(3);
        assert!(cbrt_alt.is_valid());
        assert_eq!(n_alt, cbrt);
        assert_eq!(floor_root_binary(&n, 3), cbrt);
        assert_eq!(Natural::from(&BigUint::from(&n).nth_root(3)), cbrt);
        assert_eq!(Natural::exact_from(&rug::Integer::from(&n).root(3)), cbrt);

        let cube = (&cbrt).pow(3);
        let ceiling_cbrt = (&n).ceiling_root(3);
        if cube == n {
            assert_eq!(ceiling_cbrt, cbrt);
        } else {
            assert_eq!(ceiling_cbrt, &cbrt + Natural::ONE);
        }
        assert!(cube <= n);
        assert!((cbrt + Natural::ONE).pow(3) > n);
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert_eq!(u.floor_root(3), Natural::from(u).floor_root(3));
    });
}

#[test]
fn ceiling_cbrt_properties() {
    natural_gen().test_properties(|n| {
        let cbrt = n.clone().ceiling_root(3);
        assert!(cbrt.is_valid());
        let cbrt_alt = (&n).ceiling_root(3);
        assert!(cbrt_alt.is_valid());
        assert_eq!(cbrt_alt, cbrt);
        let mut n_alt = n.clone();
        n_alt.ceiling_root_assign(3);
        assert!(cbrt_alt.is_valid());
        assert_eq!(n_alt, cbrt);
        assert_eq!(ceiling_root_binary(&n, 3), cbrt);
        let cube = (&cbrt).pow(3);
        let floor_cbrt = (&n).floor_root(3);
        if cube == n {
            assert_eq!(floor_cbrt, cbrt);
        } else {
            assert_eq!(floor_cbrt, &cbrt - Natural::ONE);
        }
        assert!(cube >= n);
        if n != 0 {
            assert!((cbrt - Natural::ONE).pow(3) < n);
        }
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert_eq!(u.ceiling_root(3), Natural::from(u).ceiling_root(3));
    });
}

#[test]
fn checked_cbrt_properties() {
    natural_gen().test_properties(|n| {
        let cbrt = n.clone().checked_root(3);
        assert!(cbrt.as_ref().map_or(true, Natural::is_valid));
        let cbrt_alt = (&n).checked_root(3);
        assert!(cbrt_alt.as_ref().map_or(true, Natural::is_valid));
        assert_eq!(cbrt_alt, cbrt);
        assert_eq!(checked_root_binary(&n, 3), cbrt);
        if let Some(cbrt) = cbrt {
            assert_eq!((&cbrt).pow(3), n);
            assert_eq!((&n).floor_root(3), cbrt);
            assert_eq!(n.ceiling_root(3), cbrt);
        }
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert_eq!(
            u.checked_root(3).map(Natural::from),
            Natural::from(u).checked_root(3)
        );
    });
}

#[test]
fn cbrt_rem_properties() {
    natural_gen().test_properties(|n| {
        let (cbrt, rem) = n.clone().root_rem(3);
        assert!(cbrt.is_valid());
        assert!(rem.is_valid());
        let (cbrt_alt, rem_alt) = (&n).root_rem(3);
        assert!(cbrt_alt.is_valid());
        assert!(rem_alt.is_valid());
        assert_eq!(cbrt_alt, cbrt_alt);
        assert_eq!(rem_alt, rem);
        let mut n_alt = n.clone();
        let rem_alt = n_alt.root_assign_rem(3);
        assert!(n_alt.is_valid());
        assert!(rem_alt.is_valid());
        assert_eq!(n_alt, cbrt);
        assert_eq!(rem_alt, rem);
        assert_eq!(root_rem_binary(&n, 3), (cbrt.clone(), rem.clone()));
        let (rug_cbrt, rug_rem) = rug::Integer::from(&n).root_rem(rug::Integer::new(), 3);
        assert_eq!(Natural::exact_from(&rug_cbrt), cbrt);
        assert_eq!(Natural::exact_from(&rug_rem), rem);

        assert_eq!((&n).floor_root(3), cbrt);
        assert_eq!(cbrt.pow(3) + rem, n);
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        let (cbrt, rem) = u.root_rem(3);
        assert_eq!(
            (Natural::from(cbrt), Natural::from(rem)),
            Natural::from(u).root_rem(3)
        );
    });
}

#[test]
fn floor_root_properties() {
    natural_unsigned_pair_gen_var_7().test_properties(|(n, exp)| {
        let root = n.clone().floor_root(exp);
        assert!(root.is_valid());
        let root_alt = (&n).floor_root(exp);
        assert!(root_alt.is_valid());
        assert_eq!(root_alt, root);
        let mut n_alt = n.clone();
        n_alt.floor_root_assign(exp);
        assert!(root_alt.is_valid());
        assert_eq!(n_alt, root);
        assert_eq!(floor_root_binary(&n, exp), root);
        assert_eq!(
            Natural::from(&BigUint::from(&n).nth_root(u32::exact_from(exp))),
            root
        );
        assert_eq!(
            Natural::exact_from(&rug::Integer::from(&n).root(u32::exact_from(exp))),
            root
        );

        let pow = (&root).pow(exp);
        let ceiling_root = (&n).ceiling_root(exp);
        if pow == n {
            assert_eq!(ceiling_root, root);
        } else {
            assert_eq!(ceiling_root, &root + Natural::ONE);
        }
        assert!(pow <= n);
        assert!((root + Natural::ONE).pow(exp) > n);
    });

    natural_gen().test_properties(|n| {
        assert_eq!((&n).floor_root(2), (&n).floor_sqrt());
        assert_eq!((&n).floor_root(1), n);
    });

    unsigned_pair_gen_var_32::<Limb, u64>().test_properties(|(u, exp)| {
        assert_eq!(u.floor_root(exp), Natural::from(u).floor_root(exp));
    });
}

#[test]
fn ceiling_root_properties() {
    natural_unsigned_pair_gen_var_7().test_properties(|(n, exp)| {
        let root = n.clone().ceiling_root(exp);
        assert!(root.is_valid());
        let root_alt = (&n).ceiling_root(exp);
        assert!(root_alt.is_valid());
        assert_eq!(root_alt, root);
        let mut n_alt = n.clone();
        n_alt.ceiling_root_assign(exp);
        assert!(root_alt.is_valid());
        assert_eq!(n_alt, root);
        assert_eq!(ceiling_root_binary(&n, exp), root);
        let pow = (&root).pow(exp);
        let floor_root = (&n).floor_root(exp);
        if pow == n {
            assert_eq!(floor_root, root);
        } else {
            assert_eq!(floor_root, &root - Natural::ONE);
        }
        assert!(pow >= n);
        if n != 0 {
            assert!((root - Natural::ONE).pow(exp) < n);
        }
    });

    natural_gen().test_properties(|n| {
        assert_eq!((&n).ceiling_root(2), (&n).ceiling_sqrt());
        assert_eq!((&n).ceiling_root(1), n);
    });

    unsigned_pair_gen_var_32::<Limb, u64>().test_properties(|(u, exp)| {
        assert_eq!(u.ceiling_root(exp), Natural::from(u).ceiling_root(exp));
    });
}

#[test]
fn checked_root_properties() {
    natural_unsigned_pair_gen_var_7().test_properties(|(n, exp)| {
        let root = n.clone().checked_root(exp);
        assert!(root.as_ref().map_or(true, Natural::is_valid));
        let root_alt = (&n).checked_root(exp);
        assert!(root_alt.as_ref().map_or(true, Natural::is_valid));
        assert_eq!(root_alt, root);
        assert_eq!(checked_root_binary(&n, exp), root);
        if let Some(root) = root {
            assert_eq!((&root).pow(exp), n);
            assert_eq!((&n).floor_root(exp), root);
            assert_eq!(n.ceiling_root(exp), root);
        }
    });

    natural_gen().test_properties(|n| {
        assert_eq!((&n).checked_root(2), (&n).checked_sqrt());
        assert_eq!((&n).checked_root(1), Some(n));
    });

    unsigned_pair_gen_var_32::<Limb, u64>().test_properties(|(u, exp)| {
        assert_eq!(
            u.checked_root(exp).map(Natural::from),
            Natural::from(u).checked_root(exp)
        );
    });
}

#[test]
fn root_rem_properties() {
    natural_unsigned_pair_gen_var_7().test_properties(|(n, exp)| {
        let (root, rem) = n.clone().root_rem(exp);
        assert!(root.is_valid());
        assert!(rem.is_valid());
        let (root_alt, rem_alt) = (&n).root_rem(exp);
        assert!(root_alt.is_valid());
        assert!(rem_alt.is_valid());
        assert_eq!(root_alt, root);
        assert_eq!(rem_alt, rem);
        let mut n_alt = n.clone();
        let rem_alt = n_alt.root_assign_rem(exp);
        assert!(root_alt.is_valid());
        assert!(rem_alt.is_valid());
        assert_eq!(n_alt, root);
        assert_eq!(rem_alt, rem);
        assert_eq!(root_rem_binary(&n, exp), (root.clone(), rem.clone()));
        let (rug_root, rug_rem) =
            rug::Integer::from(&n).root_rem(rug::Integer::new(), u32::exact_from(exp));
        assert_eq!(Natural::exact_from(&rug_root), root);
        assert_eq!(Natural::exact_from(&rug_rem), rem);

        assert_eq!((&n).floor_root(exp), root);
        assert_eq!(root.pow(exp) + rem, n);
    });

    natural_gen().test_properties(|n| {
        assert_eq!((&n).root_rem(2), (&n).sqrt_rem());
        assert_eq!((&n).root_rem(1), (n, Natural::ZERO));
    });

    unsigned_pair_gen_var_32::<Limb, u64>().test_properties(|(u, exp)| {
        let (root, rem) = u.root_rem(exp);
        assert_eq!(
            (Natural::from(root), Natural::from(rem)),
            Natural::from(u).root_rem(exp)
        );
    });
}
