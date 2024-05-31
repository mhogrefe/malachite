// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(not(feature = "32_bit_limbs"))]
use core::cmp::Ordering::*;
#[cfg(feature = "32_bit_limbs")]
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    Abs, DivisibleByPowerOf2, PowerOf2, RoundToMultiple, RoundToMultipleOfPowerOf2,
    RoundToMultipleOfPowerOf2Assign, ShrRound,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_rounding_mode_pair_gen, unsigned_unsigned_rounding_mode_triple_gen_var_3,
    unsigned_vec_unsigned_pair_gen_var_16, unsigned_vec_unsigned_pair_gen_var_20,
    unsigned_vec_unsigned_rounding_mode_triple_gen_var_2,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::round_to_multiple_of_power_of_2::{
    limbs_round_to_multiple_of_power_of_2, limbs_round_to_multiple_of_power_of_2_down,
    limbs_round_to_multiple_of_power_of_2_down_in_place,
    limbs_round_to_multiple_of_power_of_2_in_place, limbs_round_to_multiple_of_power_of_2_nearest,
    limbs_round_to_multiple_of_power_of_2_nearest_in_place,
    limbs_round_to_multiple_of_power_of_2_up, limbs_round_to_multiple_of_power_of_2_up_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_rounding_mode_pair_gen, natural_unsigned_pair_gen_var_10,
    natural_unsigned_pair_gen_var_13, natural_unsigned_pair_gen_var_4,
    natural_unsigned_rounding_mode_triple_gen_var_1,
};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_round_to_multiple_of_power_of_2_down() {
    let test = |xs: &[Limb], pow: u64, out: &[Limb], o: Ordering| {
        let (r, o_alt) = limbs_round_to_multiple_of_power_of_2_down(xs, pow);
        assert_eq!(r, out);
        assert_eq!(o_alt, o);

        let mut xs = xs.to_vec();
        assert_eq!(
            limbs_round_to_multiple_of_power_of_2_down_in_place(&mut xs, pow),
            o
        );
        assert_eq!(xs, out);
    };
    test(&[], 0, &[], Equal);
    test(&[], 1, &[], Equal);
    test(&[], 100, &[], Equal);
    test(&[0, 0, 0], 0, &[0, 0, 0], Equal);
    test(&[0, 0, 0], 1, &[0, 0, 0], Equal);
    test(&[0, 0, 0], 100, &[], Equal);
    test(&[1], 0, &[1], Equal);
    test(&[1], 1, &[0], Less);
    test(&[3], 1, &[2], Less);
    test(&[122, 456], 1, &[122, 456], Equal);
    test(&[123, 456], 0, &[123, 456], Equal);
    test(&[123, 456], 1, &[122, 456], Less);
    test(&[123, 455], 1, &[122, 455], Less);
    test(&[123, 456], 31, &[0, 456], Less);
    test(&[123, 456], 32, &[0, 456], Less);
    test(&[123, 456], 100, &[], Less);
    test(&[256, 456], 8, &[256, 456], Equal);
    test(&[u32::MAX, 1], 1, &[u32::MAX - 1, 1], Less);
    test(&[u32::MAX, u32::MAX], 32, &[0, u32::MAX], Less);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_round_to_multiple_of_power_of_2_up_in_place() {
    let test = |xs: &[Limb], pow: u64, out: &[Limb], o: Ordering| {
        let (r, o_alt) = limbs_round_to_multiple_of_power_of_2_up(xs, pow);
        assert_eq!(r, out);
        assert_eq!(o_alt, o);

        let mut xs = xs.to_vec();
        assert_eq!(
            limbs_round_to_multiple_of_power_of_2_up_in_place(&mut xs, pow),
            o
        );
        assert_eq!(xs, out);
    };
    test(&[1], 0, &[1], Equal);
    test(&[1], 1, &[2], Greater);
    test(&[3], 1, &[4], Greater);
    test(&[122, 456], 1, &[122, 456], Equal);
    test(&[123, 456], 0, &[123, 456], Equal);
    test(&[123, 456], 1, &[124, 456], Greater);
    test(&[123, 455], 1, &[124, 455], Greater);
    test(&[123, 456], 31, &[0x80000000, 456], Greater);
    test(&[123, 456], 32, &[0, 457], Greater);
    test(&[123, 456], 100, &[0, 0, 0, 16], Greater);
    test(&[256, 456], 8, &[256, 456], Equal);
    test(&[u32::MAX, 1], 1, &[0, 2], Greater);
    test(&[u32::MAX, u32::MAX], 32, &[0, 0, 1], Greater);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_round_to_multiple_of_power_of_2_nearest() {
    let test = |xs: &[Limb], pow: u64, out: &[Limb], o: Ordering| {
        let (r, o_alt) = limbs_round_to_multiple_of_power_of_2_nearest(xs, pow);
        assert_eq!(r, out);
        assert_eq!(o_alt, o);

        let mut xs = xs.to_vec();
        assert_eq!(
            limbs_round_to_multiple_of_power_of_2_nearest_in_place(&mut xs, pow),
            o
        );
        assert_eq!(xs, out);
    };
    test(&[], 0, &[], Equal);
    test(&[], 1, &[], Equal);
    test(&[], 100, &[], Equal);
    test(&[0, 0, 0], 0, &[0, 0, 0], Equal);
    test(&[0, 0, 0], 1, &[0, 0, 0], Equal);
    test(&[0, 0, 0], 100, &[], Equal);
    test(&[1], 0, &[1], Equal);
    test(&[1], 1, &[0], Less);
    test(&[3], 1, &[4], Greater);
    test(&[122, 456], 1, &[122, 456], Equal);
    test(&[123, 456], 0, &[123, 456], Equal);
    test(&[123, 456], 1, &[124, 456], Greater);
    test(&[123, 455], 1, &[124, 455], Greater);
    test(&[123, 456], 31, &[0, 456], Less);
    test(&[123, 456], 32, &[0, 456], Less);
    test(&[123, 456], 100, &[], Less);
    test(&[256, 456], 8, &[256, 456], Equal);
    test(&[u32::MAX, 1], 1, &[0, 2], Greater);
    test(&[u32::MAX, u32::MAX], 32, &[0, 0, 1], Greater);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_round_to_multiple_of_power_of_2() {
    let test = |xs: &[Limb], pow: u64, rm: RoundingMode, out: Option<(&[Limb], Ordering)>| {
        let result = limbs_round_to_multiple_of_power_of_2(xs, pow, rm);
        if let Some((r, o_alt)) = result {
            let out = out.unwrap();
            assert_eq!(r, out.0);
            assert_eq!(o_alt, out.1);
        } else {
            assert!(out.is_none());
        }

        let mut xs = xs.to_vec();
        limbs_round_to_multiple_of_power_of_2_in_place(&mut xs, pow, rm).map_or_else(
            || assert!(out.is_none()),
            |o_alt| {
                let out = out.unwrap();
                assert_eq!(xs, out.0);
                assert_eq!(o_alt, out.1);
            },
        );
    };
    test(&[1], 0, Nearest, Some((&[1], Equal)));
    test(&[1], 1, Up, Some((&[2], Greater)));
    test(&[3], 1, Nearest, Some((&[4], Greater)));
    test(&[122, 456], 1, Floor, Some((&[122, 456], Equal)));
    test(&[123, 456], 0, Floor, Some((&[123, 456], Equal)));
    test(&[123, 456], 1, Down, Some((&[122, 456], Less)));
    test(&[123, 455], 1, Floor, Some((&[122, 455], Less)));
    test(
        &[123, 456],
        31,
        Ceiling,
        Some((&[0x80000000, 456], Greater)),
    );
    test(&[123, 456], 32, Up, Some((&[0, 457], Greater)));
    test(&[123, 456], 100, Down, Some((&[], Less)));
    test(&[256, 456], 8, Exact, Some((&[256, 456], Equal)));
    test(&[u32::MAX, 1], 1, Exact, None);
    test(
        &[u32::MAX, u32::MAX],
        32,
        Down,
        Some((&[0, u32::MAX], Less)),
    );
}

#[test]
fn test_round_to_multiple_of_power_of_2() {
    let test = |s, v: u64, rm: RoundingMode, out, o| {
        let u = Natural::from_str(s).unwrap();

        let mut n = u.clone();
        assert_eq!(n.round_to_multiple_of_power_of_2_assign(v, rm), o);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let (n, o_alt) = u.clone().round_to_multiple_of_power_of_2(v, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert_eq!(o_alt, o);

        let (n, o_alt) = (&u).round_to_multiple_of_power_of_2(v, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert_eq!(o_alt, o);

        let (n, o_alt) = u.shr_round(v, rm);
        assert_eq!((n << v).to_string(), out);
        assert_eq!(o_alt, o);
    };
    test("0", 0, Down, "0", Equal);
    test("0", 0, Up, "0", Equal);
    test("0", 0, Floor, "0", Equal);
    test("0", 0, Ceiling, "0", Equal);
    test("0", 0, Nearest, "0", Equal);
    test("0", 0, Exact, "0", Equal);

    test("0", 10, Down, "0", Equal);
    test("0", 10, Up, "0", Equal);
    test("0", 10, Floor, "0", Equal);
    test("0", 10, Ceiling, "0", Equal);
    test("0", 10, Nearest, "0", Equal);
    test("0", 10, Exact, "0", Equal);

    test("123", 0, Down, "123", Equal);
    test("123", 0, Up, "123", Equal);
    test("123", 0, Floor, "123", Equal);
    test("123", 0, Ceiling, "123", Equal);
    test("123", 0, Nearest, "123", Equal);
    test("123", 0, Exact, "123", Equal);

    test("245", 1, Down, "244", Less);
    test("245", 1, Up, "246", Greater);
    test("245", 1, Floor, "244", Less);
    test("245", 1, Ceiling, "246", Greater);
    test("245", 1, Nearest, "244", Less);

    test("246", 1, Down, "246", Equal);
    test("246", 1, Up, "246", Equal);
    test("246", 1, Floor, "246", Equal);
    test("246", 1, Ceiling, "246", Equal);
    test("246", 1, Nearest, "246", Equal);
    test("246", 1, Exact, "246", Equal);

    test("247", 1, Down, "246", Less);
    test("247", 1, Up, "248", Greater);
    test("247", 1, Floor, "246", Less);
    test("247", 1, Ceiling, "248", Greater);
    test("247", 1, Nearest, "248", Greater);

    test("491", 2, Down, "488", Less);
    test("491", 2, Up, "492", Greater);
    test("491", 2, Floor, "488", Less);
    test("491", 2, Ceiling, "492", Greater);
    test("491", 2, Nearest, "492", Greater);

    test("492", 2, Down, "492", Equal);
    test("492", 2, Up, "492", Equal);
    test("492", 2, Floor, "492", Equal);
    test("492", 2, Ceiling, "492", Equal);
    test("492", 2, Nearest, "492", Equal);
    test("492", 2, Exact, "492", Equal);

    test("493", 2, Down, "492", Less);
    test("493", 2, Up, "496", Greater);
    test("493", 2, Floor, "492", Less);
    test("493", 2, Ceiling, "496", Greater);
    test("493", 2, Nearest, "492", Less);

    test("4127195135", 25, Down, "4093640704", Less);
    test("4127195135", 25, Up, "4127195136", Greater);
    test("4127195135", 25, Floor, "4093640704", Less);
    test("4127195135", 25, Ceiling, "4127195136", Greater);
    test("4127195135", 25, Nearest, "4127195136", Greater);

    test("4127195136", 25, Down, "4127195136", Equal);
    test("4127195136", 25, Up, "4127195136", Equal);
    test("4127195136", 25, Floor, "4127195136", Equal);
    test("4127195136", 25, Ceiling, "4127195136", Equal);
    test("4127195136", 25, Nearest, "4127195136", Equal);
    test("4127195136", 25, Exact, "4127195136", Equal);

    test("4127195137", 25, Down, "4127195136", Less);
    test("4127195137", 25, Up, "4160749568", Greater);
    test("4127195137", 25, Floor, "4127195136", Less);
    test("4127195137", 25, Ceiling, "4160749568", Greater);
    test("4127195137", 25, Nearest, "4127195136", Less);

    test("8254390271", 26, Down, "8187281408", Less);
    test("8254390271", 26, Up, "8254390272", Greater);
    test("8254390271", 26, Floor, "8187281408", Less);
    test("8254390271", 26, Ceiling, "8254390272", Greater);
    test("8254390271", 26, Nearest, "8254390272", Greater);

    test("8254390272", 26, Down, "8254390272", Equal);
    test("8254390272", 26, Up, "8254390272", Equal);
    test("8254390272", 26, Floor, "8254390272", Equal);
    test("8254390272", 26, Ceiling, "8254390272", Equal);
    test("8254390272", 26, Nearest, "8254390272", Equal);
    test("8254390272", 26, Exact, "8254390272", Equal);

    test("8254390273", 26, Down, "8254390272", Less);
    test("8254390273", 26, Up, "8321499136", Greater);
    test("8254390273", 26, Floor, "8254390272", Less);
    test("8254390273", 26, Ceiling, "8321499136", Greater);
    test("8254390273", 26, Nearest, "8254390272", Less);
    test(
        "155921023828072216384094494261247",
        100,
        Down,
        "154653373227843986982597791055872",
        Less,
    );
    test(
        "155921023828072216384094494261247",
        100,
        Up,
        "155921023828072216384094494261248",
        Greater,
    );
    test(
        "155921023828072216384094494261247",
        100,
        Floor,
        "154653373227843986982597791055872",
        Less,
    );
    test(
        "155921023828072216384094494261247",
        100,
        Ceiling,
        "155921023828072216384094494261248",
        Greater,
    );
    test(
        "155921023828072216384094494261247",
        100,
        Nearest,
        "155921023828072216384094494261248",
        Greater,
    );

    test(
        "155921023828072216384094494261248",
        100,
        Down,
        "155921023828072216384094494261248",
        Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        Up,
        "155921023828072216384094494261248",
        Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        Floor,
        "155921023828072216384094494261248",
        Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        Ceiling,
        "155921023828072216384094494261248",
        Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        Nearest,
        "155921023828072216384094494261248",
        Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        Exact,
        "155921023828072216384094494261248",
        Equal,
    );

    test(
        "155921023828072216384094494261249",
        100,
        Down,
        "155921023828072216384094494261248",
        Less,
    );
    test(
        "155921023828072216384094494261249",
        100,
        Up,
        "157188674428300445785591197466624",
        Greater,
    );
    test(
        "155921023828072216384094494261249",
        100,
        Floor,
        "155921023828072216384094494261248",
        Less,
    );
    test(
        "155921023828072216384094494261249",
        100,
        Ceiling,
        "157188674428300445785591197466624",
        Greater,
    );
    test(
        "155921023828072216384094494261249",
        100,
        Nearest,
        "155921023828072216384094494261248",
        Less,
    );

    test("4294967295", 1, Down, "4294967294", Less);
    test("4294967295", 1, Up, "4294967296", Greater);
    test("4294967295", 1, Floor, "4294967294", Less);
    test("4294967295", 1, Ceiling, "4294967296", Greater);
    test("4294967295", 1, Nearest, "4294967296", Greater);

    test("4294967296", 1, Down, "4294967296", Equal);
    test("4294967296", 1, Up, "4294967296", Equal);
    test("4294967296", 1, Floor, "4294967296", Equal);
    test("4294967296", 1, Ceiling, "4294967296", Equal);
    test("4294967296", 1, Nearest, "4294967296", Equal);
    test("4294967296", 1, Exact, "4294967296", Equal);

    test("4294967297", 1, Down, "4294967296", Less);
    test("4294967297", 1, Up, "4294967298", Greater);
    test("4294967297", 1, Floor, "4294967296", Less);
    test("4294967297", 1, Ceiling, "4294967298", Greater);
    test("4294967297", 1, Nearest, "4294967296", Less);

    test("1000000000000", 0, Down, "1000000000000", Equal);
    test("1000000000000", 0, Up, "1000000000000", Equal);
    test("1000000000000", 0, Floor, "1000000000000", Equal);
    test("1000000000000", 0, Ceiling, "1000000000000", Equal);
    test("1000000000000", 0, Nearest, "1000000000000", Equal);
    test("1000000000000", 0, Exact, "1000000000000", Equal);

    test("7999999999999", 3, Down, "7999999999992", Less);
    test("7999999999999", 3, Up, "8000000000000", Greater);
    test("7999999999999", 3, Floor, "7999999999992", Less);
    test("7999999999999", 3, Ceiling, "8000000000000", Greater);
    test("7999999999999", 3, Nearest, "8000000000000", Greater);

    test("8000000000000", 3, Down, "8000000000000", Equal);
    test("8000000000000", 3, Up, "8000000000000", Equal);
    test("8000000000000", 3, Floor, "8000000000000", Equal);
    test("8000000000000", 3, Ceiling, "8000000000000", Equal);
    test("8000000000000", 3, Nearest, "8000000000000", Equal);
    test("8000000000000", 3, Exact, "8000000000000", Equal);

    test("8000000000001", 3, Down, "8000000000000", Less);
    test("8000000000001", 3, Up, "8000000000008", Greater);
    test("8000000000001", 3, Floor, "8000000000000", Less);
    test("8000000000001", 3, Ceiling, "8000000000008", Greater);
    test("8000000000001", 3, Nearest, "8000000000000", Less);

    test(
        "16777216000000000000",
        24,
        Down,
        "16777216000000000000",
        Equal,
    );
    test(
        "16777216000000000000",
        24,
        Up,
        "16777216000000000000",
        Equal,
    );
    test(
        "16777216000000000000",
        24,
        Floor,
        "16777216000000000000",
        Equal,
    );
    test(
        "16777216000000000000",
        24,
        Ceiling,
        "16777216000000000000",
        Equal,
    );
    test(
        "16777216000000000000",
        24,
        Nearest,
        "16777216000000000000",
        Equal,
    );
    test(
        "16777216000000000000",
        24,
        Exact,
        "16777216000000000000",
        Equal,
    );

    test(
        "33554432000000000000",
        25,
        Down,
        "33554432000000000000",
        Equal,
    );
    test(
        "33554432000000000000",
        25,
        Up,
        "33554432000000000000",
        Equal,
    );
    test(
        "33554432000000000000",
        25,
        Floor,
        "33554432000000000000",
        Equal,
    );
    test(
        "33554432000000000000",
        25,
        Ceiling,
        "33554432000000000000",
        Equal,
    );
    test(
        "33554432000000000000",
        25,
        Nearest,
        "33554432000000000000",
        Equal,
    );
    test(
        "33554432000000000000",
        25,
        Exact,
        "33554432000000000000",
        Equal,
    );

    test(
        "2147483648000000000000",
        31,
        Down,
        "2147483648000000000000",
        Equal,
    );
    test(
        "2147483648000000000000",
        31,
        Up,
        "2147483648000000000000",
        Equal,
    );
    test(
        "2147483648000000000000",
        31,
        Floor,
        "2147483648000000000000",
        Equal,
    );
    test(
        "2147483648000000000000",
        31,
        Ceiling,
        "2147483648000000000000",
        Equal,
    );
    test(
        "2147483648000000000000",
        31,
        Nearest,
        "2147483648000000000000",
        Equal,
    );
    test(
        "2147483648000000000000",
        31,
        Exact,
        "2147483648000000000000",
        Equal,
    );

    test(
        "4294967296000000000000",
        32,
        Down,
        "4294967296000000000000",
        Equal,
    );
    test(
        "4294967296000000000000",
        32,
        Up,
        "4294967296000000000000",
        Equal,
    );
    test(
        "4294967296000000000000",
        32,
        Floor,
        "4294967296000000000000",
        Equal,
    );
    test(
        "4294967296000000000000",
        32,
        Ceiling,
        "4294967296000000000000",
        Equal,
    );
    test(
        "4294967296000000000000",
        32,
        Nearest,
        "4294967296000000000000",
        Equal,
    );
    test(
        "4294967296000000000000",
        32,
        Exact,
        "4294967296000000000000",
        Equal,
    );

    test(
        "8589934592000000000000",
        33,
        Down,
        "8589934592000000000000",
        Equal,
    );
    test(
        "8589934592000000000000",
        33,
        Up,
        "8589934592000000000000",
        Equal,
    );
    test(
        "8589934592000000000000",
        33,
        Floor,
        "8589934592000000000000",
        Equal,
    );
    test(
        "8589934592000000000000",
        33,
        Ceiling,
        "8589934592000000000000",
        Equal,
    );
    test(
        "8589934592000000000000",
        33,
        Nearest,
        "8589934592000000000000",
        Equal,
    );
    test(
        "8589934592000000000000",
        33,
        Exact,
        "8589934592000000000000",
        Equal,
    );

    test(
        "1267650600228229401496703205376000000000000",
        100,
        Down,
        "1267650600228229401496703205376000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Up,
        "1267650600228229401496703205376000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Floor,
        "1267650600228229401496703205376000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Ceiling,
        "1267650600228229401496703205376000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Nearest,
        "1267650600228229401496703205376000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Exact,
        "1267650600228229401496703205376000000000000",
        Equal,
    );

    test("1000000000000", 10, Down, "1000000000000", Equal);
    test("1000000000000", 10, Up, "1000000000000", Equal);
    test("1000000000000", 10, Floor, "1000000000000", Equal);
    test("1000000000000", 10, Ceiling, "1000000000000", Equal);
    test("1000000000000", 10, Nearest, "1000000000000", Equal);
    test("1000000000000", 10, Exact, "1000000000000", Equal);

    test("980657949", 72, Down, "0", Less);
    test("980657949", 72, Up, "4722366482869645213696", Greater);
    test("980657949", 72, Floor, "0", Less);
    test("980657949", 72, Ceiling, "4722366482869645213696", Greater);
    test("980657949", 72, Nearest, "0", Less);

    test("4294967295", 31, Down, "2147483648", Less);
    test("4294967295", 31, Up, "4294967296", Greater);
    test("4294967295", 31, Floor, "2147483648", Less);
    test("4294967295", 31, Ceiling, "4294967296", Greater);
    test("4294967295", 31, Nearest, "4294967296", Greater);

    test("4294967295", 32, Down, "0", Less);
    test("4294967295", 32, Up, "4294967296", Greater);
    test("4294967295", 32, Floor, "0", Less);
    test("4294967295", 32, Ceiling, "4294967296", Greater);
    test("4294967295", 32, Nearest, "4294967296", Greater);

    test("4294967296", 32, Down, "4294967296", Equal);
    test("4294967296", 32, Up, "4294967296", Equal);
    test("4294967296", 32, Floor, "4294967296", Equal);
    test("4294967296", 32, Ceiling, "4294967296", Equal);
    test("4294967296", 32, Nearest, "4294967296", Equal);
    test("4294967296", 32, Exact, "4294967296", Equal);

    test("4294967296", 33, Down, "0", Less);
    test("4294967296", 33, Up, "8589934592", Greater);
    test("4294967296", 33, Floor, "0", Less);
    test("4294967296", 33, Ceiling, "8589934592", Greater);
    test("4294967296", 33, Nearest, "0", Less);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_1() {
    Natural::from(123u32).round_to_multiple_of_power_of_2_assign(1, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_2() {
    Natural::from(123u32).round_to_multiple_of_power_of_2_assign(100, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_3() {
    Natural::from_str("1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2_assign(1, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_4() {
    Natural::from_str("1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2_assign(100, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_1() {
    Natural::from(123u32).round_to_multiple_of_power_of_2(1, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_2() {
    Natural::from(123u32).round_to_multiple_of_power_of_2(100, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_3() {
    Natural::from_str("1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2(1, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_4() {
    Natural::from_str("1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2(100, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_1() {
    (&Natural::from(123u32)).round_to_multiple_of_power_of_2(1, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_2() {
    (&Natural::from(123u32)).round_to_multiple_of_power_of_2(100, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_3() {
    (&Natural::from_str("1000000000001").unwrap()).round_to_multiple_of_power_of_2(1, Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_4() {
    (&Natural::from_str("1000000000001").unwrap()).round_to_multiple_of_power_of_2(100, Exact);
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_down_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(&config, |(xs, pow)| {
        let (r, o) = limbs_round_to_multiple_of_power_of_2_down(&xs, pow);
        let x = Natural::from_owned_limbs_asc(xs);
        let r = Natural::from_owned_limbs_asc(r);
        assert_eq!(r, &x >> pow << pow);
        assert_eq!(r.cmp(&x), o);
    });
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_up_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_20().test_properties_with_config(&config, |(xs, pow)| {
        let (r, o) = limbs_round_to_multiple_of_power_of_2_up(&xs, pow);
        let x = Natural::from_owned_limbs_asc(xs);
        let r = Natural::from_owned_limbs_asc(r);
        let (r_alt, o_alt) = (&x).round_to_multiple_of_power_of_2(pow, Up);
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);
        assert_eq!(r.cmp(&x), o);
    });
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_nearest_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(&config, |(xs, pow)| {
        let (r, o) = limbs_round_to_multiple_of_power_of_2_nearest(&xs, pow);
        let r = Natural::from_owned_limbs_asc(r);
        let x = Natural::from_owned_limbs_asc(xs);
        let (r_alt, o_alt) = (&x).round_to_multiple_of_power_of_2(pow, Nearest);
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);
        assert_eq!(r.cmp(&x), o);
    });
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_rounding_mode_triple_gen_var_2().test_properties_with_config(
        &config,
        |(xs, pow, rm)| {
            let n = Natural::from_limbs_asc(&xs);
            limbs_round_to_multiple_of_power_of_2(&xs, pow, rm).map_or_else(
                || {
                    assert_eq!(rm, Exact);
                    assert!(!n.divisible_by_power_of_2(pow));
                },
                |(result_limbs, o)| {
                    let (m, o_alt) = (&n).round_to_multiple_of_power_of_2(pow, rm);
                    assert_eq!(Natural::from_owned_limbs_asc(result_limbs), m);
                    assert_eq!(o_alt, o);
                    if rm == Exact {
                        assert_eq!(m, n);
                    }
                },
            );
        },
    );
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_down_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(
        &config,
        |(mut xs, pow)| {
            let old_xs = xs.clone();
            limbs_round_to_multiple_of_power_of_2_down_in_place(&mut xs, pow);
            let n = Natural::from_owned_limbs_asc(old_xs) >> pow << pow;
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
        },
    );
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_up_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_20().test_properties_with_config(
        &config,
        |(mut xs, pow)| {
            let old_xs = xs.clone();
            let o = limbs_round_to_multiple_of_power_of_2_up_in_place(&mut xs, pow);
            let (n, o_alt) =
                Natural::from_owned_limbs_asc(old_xs).round_to_multiple_of_power_of_2(pow, Up);
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
            assert_eq!(o_alt, o);
        },
    );
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_nearest_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(
        &config,
        |(mut xs, pow)| {
            let old_xs = xs.clone();
            let o = limbs_round_to_multiple_of_power_of_2_nearest_in_place(&mut xs, pow);
            let (n, o_alt) =
                Natural::from_owned_limbs_asc(old_xs).round_to_multiple_of_power_of_2(pow, Nearest);
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
            assert_eq!(o_alt, o);
        },
    );
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_rounding_mode_triple_gen_var_2().test_properties_with_config(
        &config,
        |(mut xs, pow, rm)| {
            let n = Natural::from_limbs_asc(&xs);
            limbs_round_to_multiple_of_power_of_2_in_place(&mut xs, pow, rm).map_or_else(
                || {
                    assert_eq!(rm, Exact);
                    assert!(!n.divisible_by_power_of_2(pow));
                },
                |o| {
                    let (m, o_alt) = (&n).round_to_multiple_of_power_of_2(pow, rm);
                    assert_eq!(Natural::from_owned_limbs_asc(xs), m);
                    if rm == Exact {
                        assert_eq!(m, n);
                    }
                    assert_eq!(o_alt, o);
                },
            );
        },
    );
}

#[test]
fn round_to_multiple_of_power_of_2_properties() {
    natural_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(n, pow, rm)| {
        let (r, o) = (&n).round_to_multiple_of_power_of_2(pow, rm);
        assert!(r.is_valid());

        let (r_alt, o_alt) = n.clone().round_to_multiple_of_power_of_2(pow, rm);
        assert!(r_alt.is_valid());
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);

        let mut mut_n = n.clone();
        assert_eq!(mut_n.round_to_multiple_of_power_of_2_assign(pow, rm), o);
        assert!(mut_n.is_valid());
        assert_eq!(mut_n, r);

        assert!(r.divisible_by_power_of_2(pow));
        assert_eq!(r.cmp(&n), o);
        match rm {
            Floor | Down => assert_ne!(o, Greater),
            Ceiling | Up => assert_ne!(o, Less),
            Exact => assert_eq!(o, Equal),
            _ => {}
        }
        let (s, o_alt) = (&n).shr_round(pow, rm);
        assert_eq!(s << pow, r);
        assert_eq!(o_alt, o);
        assert!((Integer::from(&r) - Integer::from(&n)).abs() <= Natural::power_of_2(pow));
        let (r_alt, o_alt) = (&n).round_to_multiple(Natural::power_of_2(pow), rm);
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);
        match rm {
            Floor | Down => assert!(r <= n),
            Ceiling | Up => assert!(r >= n),
            Exact => assert_eq!(r, n),
            Nearest => {
                let k = Natural::power_of_2(pow);
                let closest;
                let second_closest;
                if r <= n {
                    closest = &n - &r;
                    second_closest = &r + k - n;
                } else {
                    closest = &r - &n;
                    second_closest = n + k - &r;
                }
                assert!(closest <= second_closest);
                if closest == second_closest {
                    assert!(!r.get_bit(pow));
                }
            }
        }
    });

    natural_unsigned_pair_gen_var_4::<u64>().test_properties(|(n, pow)| {
        let shifted = n << pow;
        let so = (shifted.clone(), Equal);
        assert_eq!((&shifted).round_to_multiple_of_power_of_2(pow, Down), so);
        assert_eq!((&shifted).round_to_multiple_of_power_of_2(pow, Up), so);
        assert_eq!((&shifted).round_to_multiple_of_power_of_2(pow, Floor), so);
        assert_eq!((&shifted).round_to_multiple_of_power_of_2(pow, Ceiling), so);
        assert_eq!((&shifted).round_to_multiple_of_power_of_2(pow, Nearest), so);
        assert_eq!((&shifted).round_to_multiple_of_power_of_2(pow, Exact), so);
    });

    natural_unsigned_pair_gen_var_10().test_properties(|(n, pow)| {
        let down = (&n).round_to_multiple_of_power_of_2(pow, Down);
        assert_eq!(down.1, Less);
        let up = (&down.0 + Natural::power_of_2(pow), Greater);
        assert_eq!((&n).round_to_multiple_of_power_of_2(pow, Up), up);
        assert_eq!((&n).round_to_multiple_of_power_of_2(pow, Floor), down);
        assert_eq!((&n).round_to_multiple_of_power_of_2(pow, Ceiling), up);
        let nearest = n.round_to_multiple_of_power_of_2(pow, Nearest);
        assert!(nearest == down || nearest == up);
    });

    natural_unsigned_pair_gen_var_13::<u64>().test_properties(|(n, pow)| {
        if let Some(shift) = pow.checked_add(n.significant_bits()) {
            assert_eq!(
                (&n).round_to_multiple_of_power_of_2(shift, Down),
                (Natural::ZERO, Less)
            );
            assert_eq!(
                (&n).round_to_multiple_of_power_of_2(shift, Floor),
                (Natural::ZERO, Less)
            );
            if let Some(extra_shift) = shift.checked_add(1) {
                assert_eq!(
                    n.round_to_multiple_of_power_of_2(extra_shift, Nearest),
                    (Natural::ZERO, Less)
                );
            }
        }
    });

    natural_rounding_mode_pair_gen().test_properties(|(n, rm)| {
        assert_eq!((&n).round_to_multiple_of_power_of_2(0, rm), (n, Equal));
    });

    unsigned_rounding_mode_pair_gen().test_properties(|(pow, rm)| {
        assert_eq!(
            Natural::ZERO.round_to_multiple_of_power_of_2(pow, rm),
            (Natural::ZERO, Equal)
        );
    });

    unsigned_unsigned_rounding_mode_triple_gen_var_3::<Limb>().test_properties(|(n, pow, rm)| {
        let (r, o) = Natural::from(n).round_to_multiple_of_power_of_2(pow, rm);
        let (r_alt, o_alt) = n.round_to_multiple_of_power_of_2(pow, rm);
        assert_eq!(r_alt, r);
        assert_eq!(o_alt, o);
    });
}
