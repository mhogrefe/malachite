// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShr, DivRound, DivisibleByPowerOf2, ShlRound, ShrRound, ShrRoundAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    signed_rounding_mode_pair_gen, unsigned_pair_gen_var_37, unsigned_rounding_mode_pair_gen,
    unsigned_signed_rounding_mode_triple_gen_var_1,
    unsigned_unsigned_rounding_mode_triple_gen_var_4, unsigned_vec_unsigned_pair_gen_var_16,
    unsigned_vec_unsigned_pair_gen_var_20, unsigned_vec_unsigned_rounding_mode_triple_gen_var_2,
};
use malachite_nz::natural::arithmetic::shr_round::{
    limbs_shr_exact, limbs_shr_round, limbs_shr_round_nearest, limbs_shr_round_up,
    limbs_vec_shr_exact_in_place, limbs_vec_shr_round_in_place,
    limbs_vec_shr_round_nearest_in_place, limbs_vec_shr_round_up_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_rounding_mode_pair_gen, natural_signed_rounding_mode_triple_gen_var_2,
    natural_unsigned_pair_gen_var_10, natural_unsigned_pair_gen_var_4,
    natural_unsigned_rounding_mode_triple_gen_var_1,
};
use std::cmp::Ordering;
use std::ops::Shl;
use std::panic::catch_unwind;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_shr_round_up_and_limbs_vec_shr_round_up_in_place() {
    let test = |limbs: &[Limb], bits: u64, out: &[Limb], o: Ordering| {
        let (s, o_alt) = limbs_shr_round_up(limbs, bits);
        assert_eq!(s, out);
        assert_eq!(o_alt, o);

        let mut limbs = limbs.to_vec();
        assert_eq!(limbs_vec_shr_round_up_in_place(&mut limbs, bits), o);
        assert_eq!(limbs, out);
    };
    test(&[1], 0, &[1], Ordering::Equal);
    test(&[1], 1, &[1], Ordering::Greater);
    test(&[3], 1, &[2], Ordering::Greater);
    test(&[122, 456], 1, &[61, 228], Ordering::Equal);
    test(&[123, 456], 0, &[123, 456], Ordering::Equal);
    test(&[123, 456], 1, &[62, 228], Ordering::Greater);
    test(&[123, 455], 1, &[2147483710, 227], Ordering::Greater);
    test(&[123, 456], 31, &[913, 0], Ordering::Greater);
    test(&[123, 456], 32, &[457], Ordering::Greater);
    test(&[123, 456], 100, &[1], Ordering::Greater);
    test(&[256, 456], 8, &[3355443201, 1], Ordering::Equal);
    test(&[u32::MAX, 1], 1, &[0, 1], Ordering::Greater);
    test(&[u32::MAX, u32::MAX], 32, &[0, 1], Ordering::Greater);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_shr_round_nearest_and_limbs_vec_shr_round_nearest_in_place() {
    let test = |limbs: &[Limb], bits: u64, out: &[Limb], o: Ordering| {
        let (s, o_alt) = limbs_shr_round_nearest(limbs, bits);
        assert_eq!(s, out);
        assert_eq!(o_alt, o);

        let mut limbs = limbs.to_vec();
        assert_eq!(limbs_vec_shr_round_nearest_in_place(&mut limbs, bits), o);
        assert_eq!(limbs, out);
    };
    test(&[], 0, &[], Ordering::Equal);
    test(&[], 1, &[], Ordering::Equal);
    test(&[], 100, &[], Ordering::Equal);
    test(&[0, 0, 0], 0, &[0, 0, 0], Ordering::Equal);
    test(&[0, 0, 0], 1, &[0, 0, 0], Ordering::Equal);
    test(&[0, 0, 0], 100, &[], Ordering::Equal);
    test(&[1], 0, &[1], Ordering::Equal);
    test(&[1], 1, &[0], Ordering::Less);
    test(&[3], 1, &[2], Ordering::Greater);
    test(&[122, 456], 1, &[61, 228], Ordering::Equal);
    test(&[123, 456], 0, &[123, 456], Ordering::Equal);
    test(&[123, 456], 1, &[62, 228], Ordering::Greater);
    test(&[123, 455], 1, &[2147483710, 227], Ordering::Greater);
    test(&[123, 456], 31, &[912, 0], Ordering::Less);
    test(&[123, 456], 32, &[456], Ordering::Less);
    test(&[123, 456], 100, &[], Ordering::Less);
    test(&[256, 456], 8, &[3355443201, 1], Ordering::Equal);
    test(&[u32::MAX, 1], 1, &[0, 1], Ordering::Greater);
    test(&[u32::MAX, u32::MAX], 32, &[0, 1], Ordering::Greater);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_shr_exact_and_limbs_vec_shr_exact_in_place() {
    let test = |limbs: &[Limb], bits: u64, out: Option<Vec<Limb>>| {
        assert_eq!(limbs_shr_exact(limbs, bits), out);

        let mut limbs = limbs.to_vec();
        if limbs_vec_shr_exact_in_place(&mut limbs, bits) {
            assert_eq!(Some(limbs), out);
        } else {
            assert_eq!(None, out)
        }
    };
    test(&[1], 0, Some(vec![1]));
    test(&[1], 1, None);
    test(&[3], 1, None);
    test(&[122, 456], 1, Some(vec![61, 228]));
    test(&[123, 456], 0, Some(vec![123, 456]));
    test(&[123, 456], 1, None);
    test(&[123, 455], 1, None);
    test(&[123, 456], 31, None);
    test(&[123, 456], 32, None);
    test(&[123, 456], 100, None);
    test(&[256, 456], 8, Some(vec![3355443201, 1]));
    test(&[u32::MAX, 1], 1, None);
    test(&[u32::MAX, u32::MAX], 32, None);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_shr_round_and_limbs_vec_shr_round_in_place() {
    let test = |limbs: &[Limb], bits: u64, rm: RoundingMode, out: Option<&[Limb]>, o: Ordering| {
        if let Some((s, o_alt)) = limbs_shr_round(limbs, bits, rm) {
            assert_eq!(Some(s.as_slice()), out);
            assert_eq!(o_alt, o);
        } else {
            assert!(out.is_none());
        }

        let mut limbs = limbs.to_vec();
        let (b, o_alt) = limbs_vec_shr_round_in_place(&mut limbs, bits, rm);
        if b {
            assert_eq!(Some(limbs.as_slice()), out);
        } else {
            assert!(out.is_none())
        }
        assert_eq!(o_alt, o);
    };
    test(&[1], 0, RoundingMode::Nearest, Some(&[1]), Ordering::Equal);
    test(&[1], 1, RoundingMode::Up, Some(&[1]), Ordering::Greater);
    test(
        &[3],
        1,
        RoundingMode::Nearest,
        Some(&[2]),
        Ordering::Greater,
    );
    test(
        &[122, 456],
        1,
        RoundingMode::Floor,
        Some(&[61, 228]),
        Ordering::Equal,
    );
    test(
        &[123, 456],
        0,
        RoundingMode::Floor,
        Some(&[123, 456]),
        Ordering::Equal,
    );
    test(
        &[123, 456],
        1,
        RoundingMode::Down,
        Some(&[61, 228]),
        Ordering::Less,
    );
    test(
        &[123, 455],
        1,
        RoundingMode::Floor,
        Some(&[2147483709, 227]),
        Ordering::Less,
    );
    test(
        &[123, 456],
        31,
        RoundingMode::Ceiling,
        Some(&[913, 0]),
        Ordering::Greater,
    );
    test(
        &[123, 456],
        32,
        RoundingMode::Up,
        Some(&[457]),
        Ordering::Greater,
    );
    test(
        &[123, 456],
        100,
        RoundingMode::Down,
        Some(&[]),
        Ordering::Less,
    );
    test(
        &[256, 456],
        8,
        RoundingMode::Exact,
        Some(&[3355443201, 1]),
        Ordering::Equal,
    );
    test(
        &[u32::MAX, 1],
        1,
        RoundingMode::Exact,
        None,
        Ordering::Equal,
    );
    test(
        &[u32::MAX, u32::MAX],
        32,
        RoundingMode::Down,
        Some(&[u32::MAX]),
        Ordering::Less,
    );
}

fn test_shr_round_unsigned_helper<T: PrimitiveUnsigned>()
where
    Natural: ShrRound<T, Output = Natural> + ShrRoundAssign<T>,
    for<'a> &'a Natural: ShrRound<T, Output = Natural>,
{
    let test = |s, v: u8, rm: RoundingMode, out, o| {
        let u = Natural::from_str(s).unwrap();
        let v = T::from(v);

        let mut n = u.clone();
        assert_eq!(n.shr_round_assign(v, rm), o);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let (n, o_alt) = u.clone().shr_round(v, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert_eq!(o_alt, o);

        let (n, o_alt) = (&u).shr_round(v, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert_eq!(o_alt, o);
    };
    test("0", 0, RoundingMode::Down, "0", Ordering::Equal);
    test("0", 0, RoundingMode::Up, "0", Ordering::Equal);
    test("0", 0, RoundingMode::Floor, "0", Ordering::Equal);
    test("0", 0, RoundingMode::Ceiling, "0", Ordering::Equal);
    test("0", 0, RoundingMode::Nearest, "0", Ordering::Equal);
    test("0", 0, RoundingMode::Exact, "0", Ordering::Equal);

    test("0", 10, RoundingMode::Down, "0", Ordering::Equal);
    test("0", 10, RoundingMode::Up, "0", Ordering::Equal);
    test("0", 10, RoundingMode::Floor, "0", Ordering::Equal);
    test("0", 10, RoundingMode::Ceiling, "0", Ordering::Equal);
    test("0", 10, RoundingMode::Nearest, "0", Ordering::Equal);
    test("0", 10, RoundingMode::Exact, "0", Ordering::Equal);

    test("123", 0, RoundingMode::Down, "123", Ordering::Equal);
    test("123", 0, RoundingMode::Up, "123", Ordering::Equal);
    test("123", 0, RoundingMode::Floor, "123", Ordering::Equal);
    test("123", 0, RoundingMode::Ceiling, "123", Ordering::Equal);
    test("123", 0, RoundingMode::Nearest, "123", Ordering::Equal);
    test("123", 0, RoundingMode::Exact, "123", Ordering::Equal);

    test("245", 1, RoundingMode::Down, "122", Ordering::Less);
    test("245", 1, RoundingMode::Up, "123", Ordering::Greater);
    test("245", 1, RoundingMode::Floor, "122", Ordering::Less);
    test("245", 1, RoundingMode::Ceiling, "123", Ordering::Greater);
    test("245", 1, RoundingMode::Nearest, "122", Ordering::Less);

    test("246", 1, RoundingMode::Down, "123", Ordering::Equal);
    test("246", 1, RoundingMode::Up, "123", Ordering::Equal);
    test("246", 1, RoundingMode::Floor, "123", Ordering::Equal);
    test("246", 1, RoundingMode::Ceiling, "123", Ordering::Equal);
    test("246", 1, RoundingMode::Nearest, "123", Ordering::Equal);
    test("246", 1, RoundingMode::Exact, "123", Ordering::Equal);

    test("247", 1, RoundingMode::Down, "123", Ordering::Less);
    test("247", 1, RoundingMode::Up, "124", Ordering::Greater);
    test("247", 1, RoundingMode::Floor, "123", Ordering::Less);
    test("247", 1, RoundingMode::Ceiling, "124", Ordering::Greater);
    test("247", 1, RoundingMode::Nearest, "124", Ordering::Greater);

    test("491", 2, RoundingMode::Down, "122", Ordering::Less);
    test("491", 2, RoundingMode::Up, "123", Ordering::Greater);
    test("491", 2, RoundingMode::Floor, "122", Ordering::Less);
    test("491", 2, RoundingMode::Ceiling, "123", Ordering::Greater);
    test("491", 2, RoundingMode::Nearest, "123", Ordering::Greater);

    test("492", 2, RoundingMode::Down, "123", Ordering::Equal);
    test("492", 2, RoundingMode::Up, "123", Ordering::Equal);
    test("492", 2, RoundingMode::Floor, "123", Ordering::Equal);
    test("492", 2, RoundingMode::Ceiling, "123", Ordering::Equal);
    test("492", 2, RoundingMode::Nearest, "123", Ordering::Equal);
    test("492", 2, RoundingMode::Exact, "123", Ordering::Equal);

    test("493", 2, RoundingMode::Down, "123", Ordering::Less);
    test("493", 2, RoundingMode::Up, "124", Ordering::Greater);
    test("493", 2, RoundingMode::Floor, "123", Ordering::Less);
    test("493", 2, RoundingMode::Ceiling, "124", Ordering::Greater);
    test("493", 2, RoundingMode::Nearest, "123", Ordering::Less);

    test("4127195135", 25, RoundingMode::Down, "122", Ordering::Less);
    test("4127195135", 25, RoundingMode::Up, "123", Ordering::Greater);
    test("4127195135", 25, RoundingMode::Floor, "122", Ordering::Less);
    test(
        "4127195135",
        25,
        RoundingMode::Ceiling,
        "123",
        Ordering::Greater,
    );
    test(
        "4127195135",
        25,
        RoundingMode::Nearest,
        "123",
        Ordering::Greater,
    );

    test("4127195136", 25, RoundingMode::Down, "123", Ordering::Equal);
    test("4127195136", 25, RoundingMode::Up, "123", Ordering::Equal);
    test(
        "4127195136",
        25,
        RoundingMode::Floor,
        "123",
        Ordering::Equal,
    );
    test(
        "4127195136",
        25,
        RoundingMode::Ceiling,
        "123",
        Ordering::Equal,
    );
    test(
        "4127195136",
        25,
        RoundingMode::Nearest,
        "123",
        Ordering::Equal,
    );
    test(
        "4127195136",
        25,
        RoundingMode::Exact,
        "123",
        Ordering::Equal,
    );

    test("4127195137", 25, RoundingMode::Down, "123", Ordering::Less);
    test("4127195137", 25, RoundingMode::Up, "124", Ordering::Greater);
    test("4127195137", 25, RoundingMode::Floor, "123", Ordering::Less);
    test(
        "4127195137",
        25,
        RoundingMode::Ceiling,
        "124",
        Ordering::Greater,
    );
    test(
        "4127195137",
        25,
        RoundingMode::Nearest,
        "123",
        Ordering::Less,
    );

    test("8254390271", 26, RoundingMode::Down, "122", Ordering::Less);
    test("8254390271", 26, RoundingMode::Up, "123", Ordering::Greater);
    test("8254390271", 26, RoundingMode::Floor, "122", Ordering::Less);
    test(
        "8254390271",
        26,
        RoundingMode::Ceiling,
        "123",
        Ordering::Greater,
    );
    test(
        "8254390271",
        26,
        RoundingMode::Nearest,
        "123",
        Ordering::Greater,
    );

    test("8254390272", 26, RoundingMode::Down, "123", Ordering::Equal);
    test("8254390272", 26, RoundingMode::Up, "123", Ordering::Equal);
    test(
        "8254390272",
        26,
        RoundingMode::Floor,
        "123",
        Ordering::Equal,
    );
    test(
        "8254390272",
        26,
        RoundingMode::Ceiling,
        "123",
        Ordering::Equal,
    );
    test(
        "8254390272",
        26,
        RoundingMode::Nearest,
        "123",
        Ordering::Equal,
    );
    test(
        "8254390272",
        26,
        RoundingMode::Exact,
        "123",
        Ordering::Equal,
    );

    test("8254390273", 26, RoundingMode::Down, "123", Ordering::Less);
    test("8254390273", 26, RoundingMode::Up, "124", Ordering::Greater);
    test("8254390273", 26, RoundingMode::Floor, "123", Ordering::Less);
    test(
        "8254390273",
        26,
        RoundingMode::Ceiling,
        "124",
        Ordering::Greater,
    );
    test(
        "8254390273",
        26,
        RoundingMode::Nearest,
        "123",
        Ordering::Less,
    );

    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Down,
        "122",
        Ordering::Less,
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Up,
        "123",
        Ordering::Greater,
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Floor,
        "122",
        Ordering::Less,
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Ceiling,
        "123",
        Ordering::Greater,
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Nearest,
        "123",
        Ordering::Greater,
    );

    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Down,
        "123",
        Ordering::Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Up,
        "123",
        Ordering::Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Floor,
        "123",
        Ordering::Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Ceiling,
        "123",
        Ordering::Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Nearest,
        "123",
        Ordering::Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Exact,
        "123",
        Ordering::Equal,
    );

    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Down,
        "123",
        Ordering::Less,
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Up,
        "124",
        Ordering::Greater,
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Floor,
        "123",
        Ordering::Less,
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Ceiling,
        "124",
        Ordering::Greater,
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Nearest,
        "123",
        Ordering::Less,
    );

    test(
        "4294967295",
        1,
        RoundingMode::Down,
        "2147483647",
        Ordering::Less,
    );
    test(
        "4294967295",
        1,
        RoundingMode::Up,
        "2147483648",
        Ordering::Greater,
    );
    test(
        "4294967295",
        1,
        RoundingMode::Floor,
        "2147483647",
        Ordering::Less,
    );
    test(
        "4294967295",
        1,
        RoundingMode::Ceiling,
        "2147483648",
        Ordering::Greater,
    );
    test(
        "4294967295",
        1,
        RoundingMode::Nearest,
        "2147483648",
        Ordering::Greater,
    );

    test(
        "4294967296",
        1,
        RoundingMode::Down,
        "2147483648",
        Ordering::Equal,
    );
    test(
        "4294967296",
        1,
        RoundingMode::Up,
        "2147483648",
        Ordering::Equal,
    );
    test(
        "4294967296",
        1,
        RoundingMode::Floor,
        "2147483648",
        Ordering::Equal,
    );
    test(
        "4294967296",
        1,
        RoundingMode::Ceiling,
        "2147483648",
        Ordering::Equal,
    );
    test(
        "4294967296",
        1,
        RoundingMode::Nearest,
        "2147483648",
        Ordering::Equal,
    );
    test(
        "4294967296",
        1,
        RoundingMode::Exact,
        "2147483648",
        Ordering::Equal,
    );

    test(
        "4294967297",
        1,
        RoundingMode::Down,
        "2147483648",
        Ordering::Less,
    );
    test(
        "4294967297",
        1,
        RoundingMode::Up,
        "2147483649",
        Ordering::Greater,
    );
    test(
        "4294967297",
        1,
        RoundingMode::Floor,
        "2147483648",
        Ordering::Less,
    );
    test(
        "4294967297",
        1,
        RoundingMode::Ceiling,
        "2147483649",
        Ordering::Greater,
    );
    test(
        "4294967297",
        1,
        RoundingMode::Nearest,
        "2147483648",
        Ordering::Less,
    );

    test(
        "1000000000000",
        0,
        RoundingMode::Down,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        0,
        RoundingMode::Up,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        0,
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        0,
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        0,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        0,
        RoundingMode::Exact,
        "1000000000000",
        Ordering::Equal,
    );

    test(
        "7999999999999",
        3,
        RoundingMode::Down,
        "999999999999",
        Ordering::Less,
    );
    test(
        "7999999999999",
        3,
        RoundingMode::Up,
        "1000000000000",
        Ordering::Greater,
    );
    test(
        "7999999999999",
        3,
        RoundingMode::Floor,
        "999999999999",
        Ordering::Less,
    );
    test(
        "7999999999999",
        3,
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Greater,
    );
    test(
        "7999999999999",
        3,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Greater,
    );

    test(
        "8000000000000",
        3,
        RoundingMode::Down,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8000000000000",
        3,
        RoundingMode::Up,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8000000000000",
        3,
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8000000000000",
        3,
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8000000000000",
        3,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8000000000000",
        3,
        RoundingMode::Exact,
        "1000000000000",
        Ordering::Equal,
    );

    test(
        "8000000000001",
        3,
        RoundingMode::Down,
        "1000000000000",
        Ordering::Less,
    );
    test(
        "8000000000001",
        3,
        RoundingMode::Up,
        "1000000000001",
        Ordering::Greater,
    );
    test(
        "8000000000001",
        3,
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Less,
    );
    test(
        "8000000000001",
        3,
        RoundingMode::Ceiling,
        "1000000000001",
        Ordering::Greater,
    );
    test(
        "8000000000001",
        3,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Less,
    );

    test(
        "16777216000000000000",
        24,
        RoundingMode::Down,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Up,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Exact,
        "1000000000000",
        Ordering::Equal,
    );

    test(
        "33554432000000000000",
        25,
        RoundingMode::Down,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Up,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Exact,
        "1000000000000",
        Ordering::Equal,
    );

    test(
        "2147483648000000000000",
        31,
        RoundingMode::Down,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Up,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Exact,
        "1000000000000",
        Ordering::Equal,
    );

    test(
        "4294967296000000000000",
        32,
        RoundingMode::Down,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Up,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Exact,
        "1000000000000",
        Ordering::Equal,
    );

    test(
        "8589934592000000000000",
        33,
        RoundingMode::Down,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Up,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Exact,
        "1000000000000",
        Ordering::Equal,
    );

    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Down,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Up,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Exact,
        "1000000000000",
        Ordering::Equal,
    );

    test(
        "1000000000000",
        10,
        RoundingMode::Down,
        "976562500",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        10,
        RoundingMode::Up,
        "976562500",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        10,
        RoundingMode::Floor,
        "976562500",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        10,
        RoundingMode::Ceiling,
        "976562500",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        10,
        RoundingMode::Nearest,
        "976562500",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        10,
        RoundingMode::Exact,
        "976562500",
        Ordering::Equal,
    );

    test("980657949", 72, RoundingMode::Down, "0", Ordering::Less);
    test("980657949", 72, RoundingMode::Up, "1", Ordering::Greater);
    test("980657949", 72, RoundingMode::Floor, "0", Ordering::Less);
    test(
        "980657949",
        72,
        RoundingMode::Ceiling,
        "1",
        Ordering::Greater,
    );
    test("980657949", 72, RoundingMode::Nearest, "0", Ordering::Less);

    test("4294967295", 31, RoundingMode::Down, "1", Ordering::Less);
    test("4294967295", 31, RoundingMode::Up, "2", Ordering::Greater);
    test("4294967295", 31, RoundingMode::Floor, "1", Ordering::Less);
    test(
        "4294967295",
        31,
        RoundingMode::Ceiling,
        "2",
        Ordering::Greater,
    );
    test(
        "4294967295",
        31,
        RoundingMode::Nearest,
        "2",
        Ordering::Greater,
    );

    test("4294967295", 32, RoundingMode::Down, "0", Ordering::Less);
    test("4294967295", 32, RoundingMode::Up, "1", Ordering::Greater);
    test("4294967295", 32, RoundingMode::Floor, "0", Ordering::Less);
    test(
        "4294967295",
        32,
        RoundingMode::Ceiling,
        "1",
        Ordering::Greater,
    );
    test(
        "4294967295",
        32,
        RoundingMode::Nearest,
        "1",
        Ordering::Greater,
    );

    test("4294967296", 32, RoundingMode::Down, "1", Ordering::Equal);
    test("4294967296", 32, RoundingMode::Up, "1", Ordering::Equal);
    test("4294967296", 32, RoundingMode::Floor, "1", Ordering::Equal);
    test(
        "4294967296",
        32,
        RoundingMode::Ceiling,
        "1",
        Ordering::Equal,
    );
    test(
        "4294967296",
        32,
        RoundingMode::Nearest,
        "1",
        Ordering::Equal,
    );
    test("4294967296", 32, RoundingMode::Exact, "1", Ordering::Equal);

    test("4294967296", 33, RoundingMode::Down, "0", Ordering::Less);
    test("4294967296", 33, RoundingMode::Up, "1", Ordering::Greater);
    test("4294967296", 33, RoundingMode::Floor, "0", Ordering::Less);
    test(
        "4294967296",
        33,
        RoundingMode::Ceiling,
        "1",
        Ordering::Greater,
    );
    test("4294967296", 33, RoundingMode::Nearest, "0", Ordering::Less);
}

#[test]
fn test_shr_round_unsigned() {
    apply_fn_to_unsigneds!(test_shr_round_unsigned_helper);
}

macro_rules! shr_round_unsigned_fail_helper {
    ($t:ident) => {
        assert_panic!(Natural::from(123u32).shr_round_assign($t::ONE, RoundingMode::Exact));
        assert_panic!(
            Natural::from(123u32).shr_round_assign($t::exact_from(100), RoundingMode::Exact)
        );
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shr_round_assign($t::ONE, RoundingMode::Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shr_round_assign($t::exact_from(100), RoundingMode::Exact));
        assert_panic!(Natural::from(123u32).shr_round($t::ONE, RoundingMode::Exact));
        assert_panic!(Natural::from(123u32).shr_round($t::exact_from(100), RoundingMode::Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shr_round($t::ONE, RoundingMode::Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shr_round($t::exact_from(100), RoundingMode::Exact));
        assert_panic!((&Natural::from(123u32)).shr_round($t::ONE, RoundingMode::Exact));
        assert_panic!((&Natural::from(123u32)).shr_round($t::exact_from(100), RoundingMode::Exact));
        assert_panic!(
            (&Natural::from_str("1000000000001").unwrap()).shr_round($t::ONE, RoundingMode::Exact)
        );
        assert_panic!((&Natural::from_str("1000000000001").unwrap())
            .shr_round($t::exact_from(100), RoundingMode::Exact));
    };
}

#[test]
fn shr_round_unsigned_fail() {
    apply_to_unsigneds!(shr_round_unsigned_fail_helper);
}

fn test_shr_round_signed_helper<T: PrimitiveSigned>()
where
    Natural: ShrRound<T, Output = Natural> + ShrRoundAssign<T>,
    for<'a> &'a Natural: ShrRound<T, Output = Natural>,
{
    let test = |s, j: i8, rm: RoundingMode, out, o| {
        let u = Natural::from_str(s).unwrap();
        let j = T::from(j);

        let mut n = u.clone();
        assert_eq!(n.shr_round_assign(j, rm), o);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let (n, o_alt) = u.clone().shr_round(j, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert_eq!(o_alt, o);

        let (n, o_alt) = (&u).shr_round(j, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert_eq!(o_alt, o);
    };
    test("0", 0, RoundingMode::Down, "0", Ordering::Equal);
    test("0", 0, RoundingMode::Up, "0", Ordering::Equal);
    test("0", 0, RoundingMode::Floor, "0", Ordering::Equal);
    test("0", 0, RoundingMode::Ceiling, "0", Ordering::Equal);
    test("0", 0, RoundingMode::Nearest, "0", Ordering::Equal);
    test("0", 0, RoundingMode::Exact, "0", Ordering::Equal);

    test("0", 10, RoundingMode::Down, "0", Ordering::Equal);
    test("0", 10, RoundingMode::Up, "0", Ordering::Equal);
    test("0", 10, RoundingMode::Floor, "0", Ordering::Equal);
    test("0", 10, RoundingMode::Ceiling, "0", Ordering::Equal);
    test("0", 10, RoundingMode::Nearest, "0", Ordering::Equal);
    test("0", 10, RoundingMode::Exact, "0", Ordering::Equal);

    test("123", 0, RoundingMode::Down, "123", Ordering::Equal);
    test("123", 0, RoundingMode::Up, "123", Ordering::Equal);
    test("123", 0, RoundingMode::Floor, "123", Ordering::Equal);
    test("123", 0, RoundingMode::Ceiling, "123", Ordering::Equal);
    test("123", 0, RoundingMode::Nearest, "123", Ordering::Equal);
    test("123", 0, RoundingMode::Exact, "123", Ordering::Equal);

    test("245", 1, RoundingMode::Down, "122", Ordering::Less);
    test("245", 1, RoundingMode::Up, "123", Ordering::Greater);
    test("245", 1, RoundingMode::Floor, "122", Ordering::Less);
    test("245", 1, RoundingMode::Ceiling, "123", Ordering::Greater);
    test("245", 1, RoundingMode::Nearest, "122", Ordering::Less);

    test("246", 1, RoundingMode::Down, "123", Ordering::Equal);
    test("246", 1, RoundingMode::Up, "123", Ordering::Equal);
    test("246", 1, RoundingMode::Floor, "123", Ordering::Equal);
    test("246", 1, RoundingMode::Ceiling, "123", Ordering::Equal);
    test("246", 1, RoundingMode::Nearest, "123", Ordering::Equal);
    test("246", 1, RoundingMode::Exact, "123", Ordering::Equal);

    test("247", 1, RoundingMode::Down, "123", Ordering::Less);
    test("247", 1, RoundingMode::Up, "124", Ordering::Greater);
    test("247", 1, RoundingMode::Floor, "123", Ordering::Less);
    test("247", 1, RoundingMode::Ceiling, "124", Ordering::Greater);
    test("247", 1, RoundingMode::Nearest, "124", Ordering::Greater);

    test("491", 2, RoundingMode::Down, "122", Ordering::Less);
    test("491", 2, RoundingMode::Up, "123", Ordering::Greater);
    test("491", 2, RoundingMode::Floor, "122", Ordering::Less);
    test("491", 2, RoundingMode::Ceiling, "123", Ordering::Greater);
    test("491", 2, RoundingMode::Nearest, "123", Ordering::Greater);

    test("492", 2, RoundingMode::Down, "123", Ordering::Equal);
    test("492", 2, RoundingMode::Up, "123", Ordering::Equal);
    test("492", 2, RoundingMode::Floor, "123", Ordering::Equal);
    test("492", 2, RoundingMode::Ceiling, "123", Ordering::Equal);
    test("492", 2, RoundingMode::Nearest, "123", Ordering::Equal);
    test("492", 2, RoundingMode::Exact, "123", Ordering::Equal);

    test("493", 2, RoundingMode::Down, "123", Ordering::Less);
    test("493", 2, RoundingMode::Up, "124", Ordering::Greater);
    test("493", 2, RoundingMode::Floor, "123", Ordering::Less);
    test("493", 2, RoundingMode::Ceiling, "124", Ordering::Greater);
    test("493", 2, RoundingMode::Nearest, "123", Ordering::Less);

    test("4127195135", 25, RoundingMode::Down, "122", Ordering::Less);
    test("4127195135", 25, RoundingMode::Up, "123", Ordering::Greater);
    test("4127195135", 25, RoundingMode::Floor, "122", Ordering::Less);
    test(
        "4127195135",
        25,
        RoundingMode::Ceiling,
        "123",
        Ordering::Greater,
    );
    test(
        "4127195135",
        25,
        RoundingMode::Nearest,
        "123",
        Ordering::Greater,
    );

    test("4127195136", 25, RoundingMode::Down, "123", Ordering::Equal);
    test("4127195136", 25, RoundingMode::Up, "123", Ordering::Equal);
    test(
        "4127195136",
        25,
        RoundingMode::Floor,
        "123",
        Ordering::Equal,
    );
    test(
        "4127195136",
        25,
        RoundingMode::Ceiling,
        "123",
        Ordering::Equal,
    );
    test(
        "4127195136",
        25,
        RoundingMode::Nearest,
        "123",
        Ordering::Equal,
    );
    test(
        "4127195136",
        25,
        RoundingMode::Exact,
        "123",
        Ordering::Equal,
    );

    test("4127195137", 25, RoundingMode::Down, "123", Ordering::Less);
    test("4127195137", 25, RoundingMode::Up, "124", Ordering::Greater);
    test("4127195137", 25, RoundingMode::Floor, "123", Ordering::Less);
    test(
        "4127195137",
        25,
        RoundingMode::Ceiling,
        "124",
        Ordering::Greater,
    );
    test(
        "4127195137",
        25,
        RoundingMode::Nearest,
        "123",
        Ordering::Less,
    );

    test("8254390271", 26, RoundingMode::Down, "122", Ordering::Less);
    test("8254390271", 26, RoundingMode::Up, "123", Ordering::Greater);
    test("8254390271", 26, RoundingMode::Floor, "122", Ordering::Less);
    test(
        "8254390271",
        26,
        RoundingMode::Ceiling,
        "123",
        Ordering::Greater,
    );
    test(
        "8254390271",
        26,
        RoundingMode::Nearest,
        "123",
        Ordering::Greater,
    );

    test("8254390272", 26, RoundingMode::Down, "123", Ordering::Equal);
    test("8254390272", 26, RoundingMode::Up, "123", Ordering::Equal);
    test(
        "8254390272",
        26,
        RoundingMode::Floor,
        "123",
        Ordering::Equal,
    );
    test(
        "8254390272",
        26,
        RoundingMode::Ceiling,
        "123",
        Ordering::Equal,
    );
    test(
        "8254390272",
        26,
        RoundingMode::Nearest,
        "123",
        Ordering::Equal,
    );
    test(
        "8254390272",
        26,
        RoundingMode::Exact,
        "123",
        Ordering::Equal,
    );

    test("8254390273", 26, RoundingMode::Down, "123", Ordering::Less);
    test("8254390273", 26, RoundingMode::Up, "124", Ordering::Greater);
    test("8254390273", 26, RoundingMode::Floor, "123", Ordering::Less);
    test(
        "8254390273",
        26,
        RoundingMode::Ceiling,
        "124",
        Ordering::Greater,
    );
    test(
        "8254390273",
        26,
        RoundingMode::Nearest,
        "123",
        Ordering::Less,
    );

    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Down,
        "122",
        Ordering::Less,
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Up,
        "123",
        Ordering::Greater,
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Floor,
        "122",
        Ordering::Less,
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Ceiling,
        "123",
        Ordering::Greater,
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Nearest,
        "123",
        Ordering::Greater,
    );

    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Down,
        "123",
        Ordering::Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Up,
        "123",
        Ordering::Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Floor,
        "123",
        Ordering::Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Ceiling,
        "123",
        Ordering::Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Nearest,
        "123",
        Ordering::Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Exact,
        "123",
        Ordering::Equal,
    );

    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Down,
        "123",
        Ordering::Less,
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Up,
        "124",
        Ordering::Greater,
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Floor,
        "123",
        Ordering::Less,
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Ceiling,
        "124",
        Ordering::Greater,
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Nearest,
        "123",
        Ordering::Less,
    );

    test(
        "4294967295",
        1,
        RoundingMode::Down,
        "2147483647",
        Ordering::Less,
    );
    test(
        "4294967295",
        1,
        RoundingMode::Up,
        "2147483648",
        Ordering::Greater,
    );
    test(
        "4294967295",
        1,
        RoundingMode::Floor,
        "2147483647",
        Ordering::Less,
    );
    test(
        "4294967295",
        1,
        RoundingMode::Ceiling,
        "2147483648",
        Ordering::Greater,
    );
    test(
        "4294967295",
        1,
        RoundingMode::Nearest,
        "2147483648",
        Ordering::Greater,
    );

    test(
        "4294967296",
        1,
        RoundingMode::Down,
        "2147483648",
        Ordering::Equal,
    );
    test(
        "4294967296",
        1,
        RoundingMode::Up,
        "2147483648",
        Ordering::Equal,
    );
    test(
        "4294967296",
        1,
        RoundingMode::Floor,
        "2147483648",
        Ordering::Equal,
    );
    test(
        "4294967296",
        1,
        RoundingMode::Ceiling,
        "2147483648",
        Ordering::Equal,
    );
    test(
        "4294967296",
        1,
        RoundingMode::Nearest,
        "2147483648",
        Ordering::Equal,
    );
    test(
        "4294967296",
        1,
        RoundingMode::Exact,
        "2147483648",
        Ordering::Equal,
    );

    test(
        "4294967297",
        1,
        RoundingMode::Down,
        "2147483648",
        Ordering::Less,
    );
    test(
        "4294967297",
        1,
        RoundingMode::Up,
        "2147483649",
        Ordering::Greater,
    );
    test(
        "4294967297",
        1,
        RoundingMode::Floor,
        "2147483648",
        Ordering::Less,
    );
    test(
        "4294967297",
        1,
        RoundingMode::Ceiling,
        "2147483649",
        Ordering::Greater,
    );
    test(
        "4294967297",
        1,
        RoundingMode::Nearest,
        "2147483648",
        Ordering::Less,
    );

    test(
        "1000000000000",
        0,
        RoundingMode::Down,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        0,
        RoundingMode::Up,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        0,
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        0,
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        0,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        0,
        RoundingMode::Exact,
        "1000000000000",
        Ordering::Equal,
    );

    test(
        "7999999999999",
        3,
        RoundingMode::Down,
        "999999999999",
        Ordering::Less,
    );
    test(
        "7999999999999",
        3,
        RoundingMode::Up,
        "1000000000000",
        Ordering::Greater,
    );
    test(
        "7999999999999",
        3,
        RoundingMode::Floor,
        "999999999999",
        Ordering::Less,
    );
    test(
        "7999999999999",
        3,
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Greater,
    );
    test(
        "7999999999999",
        3,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Greater,
    );

    test(
        "8000000000000",
        3,
        RoundingMode::Down,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8000000000000",
        3,
        RoundingMode::Up,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8000000000000",
        3,
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8000000000000",
        3,
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8000000000000",
        3,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8000000000000",
        3,
        RoundingMode::Exact,
        "1000000000000",
        Ordering::Equal,
    );

    test(
        "8000000000001",
        3,
        RoundingMode::Down,
        "1000000000000",
        Ordering::Less,
    );
    test(
        "8000000000001",
        3,
        RoundingMode::Up,
        "1000000000001",
        Ordering::Greater,
    );
    test(
        "8000000000001",
        3,
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Less,
    );
    test(
        "8000000000001",
        3,
        RoundingMode::Ceiling,
        "1000000000001",
        Ordering::Greater,
    );
    test(
        "8000000000001",
        3,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Less,
    );

    test(
        "16777216000000000000",
        24,
        RoundingMode::Down,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Up,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Exact,
        "1000000000000",
        Ordering::Equal,
    );

    test(
        "33554432000000000000",
        25,
        RoundingMode::Down,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Up,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Exact,
        "1000000000000",
        Ordering::Equal,
    );

    test(
        "2147483648000000000000",
        31,
        RoundingMode::Down,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Up,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Exact,
        "1000000000000",
        Ordering::Equal,
    );

    test(
        "4294967296000000000000",
        32,
        RoundingMode::Down,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Up,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Exact,
        "1000000000000",
        Ordering::Equal,
    );

    test(
        "8589934592000000000000",
        33,
        RoundingMode::Down,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Up,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Exact,
        "1000000000000",
        Ordering::Equal,
    );

    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Down,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Up,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Floor,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Ceiling,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Nearest,
        "1000000000000",
        Ordering::Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Exact,
        "1000000000000",
        Ordering::Equal,
    );

    test(
        "1000000000000",
        10,
        RoundingMode::Down,
        "976562500",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        10,
        RoundingMode::Up,
        "976562500",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        10,
        RoundingMode::Floor,
        "976562500",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        10,
        RoundingMode::Ceiling,
        "976562500",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        10,
        RoundingMode::Nearest,
        "976562500",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        10,
        RoundingMode::Exact,
        "976562500",
        Ordering::Equal,
    );

    test("980657949", 72, RoundingMode::Down, "0", Ordering::Less);
    test("980657949", 72, RoundingMode::Up, "1", Ordering::Greater);
    test("980657949", 72, RoundingMode::Floor, "0", Ordering::Less);
    test(
        "980657949",
        72,
        RoundingMode::Ceiling,
        "1",
        Ordering::Greater,
    );
    test("980657949", 72, RoundingMode::Nearest, "0", Ordering::Less);

    test("4294967295", 31, RoundingMode::Down, "1", Ordering::Less);
    test("4294967295", 31, RoundingMode::Up, "2", Ordering::Greater);
    test("4294967295", 31, RoundingMode::Floor, "1", Ordering::Less);
    test(
        "4294967295",
        31,
        RoundingMode::Ceiling,
        "2",
        Ordering::Greater,
    );
    test(
        "4294967295",
        31,
        RoundingMode::Nearest,
        "2",
        Ordering::Greater,
    );

    test("4294967295", 32, RoundingMode::Down, "0", Ordering::Less);
    test("4294967295", 32, RoundingMode::Up, "1", Ordering::Greater);
    test("4294967295", 32, RoundingMode::Floor, "0", Ordering::Less);
    test(
        "4294967295",
        32,
        RoundingMode::Ceiling,
        "1",
        Ordering::Greater,
    );
    test(
        "4294967295",
        32,
        RoundingMode::Nearest,
        "1",
        Ordering::Greater,
    );

    test("4294967296", 32, RoundingMode::Down, "1", Ordering::Equal);
    test("4294967296", 32, RoundingMode::Up, "1", Ordering::Equal);
    test("4294967296", 32, RoundingMode::Floor, "1", Ordering::Equal);
    test(
        "4294967296",
        32,
        RoundingMode::Ceiling,
        "1",
        Ordering::Equal,
    );
    test(
        "4294967296",
        32,
        RoundingMode::Nearest,
        "1",
        Ordering::Equal,
    );
    test("4294967296", 32, RoundingMode::Exact, "1", Ordering::Equal);

    test("4294967296", 33, RoundingMode::Down, "0", Ordering::Less);
    test("4294967296", 33, RoundingMode::Up, "1", Ordering::Greater);
    test("4294967296", 33, RoundingMode::Floor, "0", Ordering::Less);
    test(
        "4294967296",
        33,
        RoundingMode::Ceiling,
        "1",
        Ordering::Greater,
    );
    test("4294967296", 33, RoundingMode::Nearest, "0", Ordering::Less);

    test("0", -10, RoundingMode::Exact, "0", Ordering::Equal);
    test("123", -1, RoundingMode::Exact, "246", Ordering::Equal);
    test("123", -2, RoundingMode::Exact, "492", Ordering::Equal);
    test(
        "123",
        -25,
        RoundingMode::Exact,
        "4127195136",
        Ordering::Equal,
    );
    test(
        "123",
        -26,
        RoundingMode::Exact,
        "8254390272",
        Ordering::Equal,
    );
    test(
        "123",
        -100,
        RoundingMode::Exact,
        "155921023828072216384094494261248",
        Ordering::Equal,
    );
    test(
        "2147483648",
        -1,
        RoundingMode::Exact,
        "4294967296",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        -3,
        RoundingMode::Exact,
        "8000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        -24,
        RoundingMode::Exact,
        "16777216000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        -25,
        RoundingMode::Exact,
        "33554432000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        -31,
        RoundingMode::Exact,
        "2147483648000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        -32,
        RoundingMode::Exact,
        "4294967296000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        -33,
        RoundingMode::Exact,
        "8589934592000000000000",
        Ordering::Equal,
    );
    test(
        "1000000000000",
        -100,
        RoundingMode::Exact,
        "1267650600228229401496703205376000000000000",
        Ordering::Equal,
    );
}

#[test]
fn test_shr_round_signed() {
    apply_fn_to_signeds!(test_shr_round_signed_helper);
}

macro_rules! shr_round_signed_fail_helper {
    ($t:ident) => {
        assert_panic!(Natural::from(123u32).shr_round_assign($t::ONE, RoundingMode::Exact));
        assert_panic!(
            Natural::from(123u32).shr_round_assign($t::exact_from(100), RoundingMode::Exact)
        );
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shr_round_assign($t::ONE, RoundingMode::Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shr_round_assign($t::exact_from(100), RoundingMode::Exact));
        assert_panic!(Natural::from(123u32).shr_round($t::ONE, RoundingMode::Exact));
        assert_panic!(Natural::from(123u32).shr_round($t::exact_from(100), RoundingMode::Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shr_round($t::ONE, RoundingMode::Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shr_round($t::exact_from(100), RoundingMode::Exact));
        assert_panic!((&Natural::from(123u32)).shr_round($t::ONE, RoundingMode::Exact));
        assert_panic!((&Natural::from(123u32)).shr_round($t::exact_from(100), RoundingMode::Exact));
        assert_panic!(
            (&Natural::from_str("1000000000001").unwrap()).shr_round($t::ONE, RoundingMode::Exact)
        );
        assert_panic!((&Natural::from_str("1000000000001").unwrap())
            .shr_round($t::exact_from(100), RoundingMode::Exact));
    };
}

#[test]
fn shr_round_signed_fail() {
    apply_to_signeds!(shr_round_signed_fail_helper);
}

#[test]
fn limbs_shr_round_up_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_20().test_properties_with_config(&config, |(xs, bits)| {
        let (s, o) = limbs_shr_round_up(&xs, bits);
        assert_eq!(
            (Natural::from_owned_limbs_asc(s), o),
            Natural::from_owned_limbs_asc(xs).shr_round(bits, RoundingMode::Up),
        );
    });
}

#[test]
fn limbs_shr_round_nearest_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(&config, |(xs, bits)| {
        let (s, o) = limbs_shr_round_nearest(&xs, bits);
        assert_eq!(
            (Natural::from_owned_limbs_asc(s), o),
            Natural::from_owned_limbs_asc(xs).shr_round(bits, RoundingMode::Nearest),
        );
    });
}

#[test]
fn limbs_shr_exact_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_20().test_properties_with_config(&config, |(xs, bits)| {
        let n = Natural::from_limbs_asc(&xs);
        limbs_shr_exact(&xs, bits).map_or_else(
            || {
                assert!(!n.divisible_by_power_of_2(bits));
            },
            |result_xs| {
                let (m, o) = (&n).shr_round(bits, RoundingMode::Exact);
                assert_eq!(Natural::from_owned_limbs_asc(result_xs), m);
                assert_eq!(m << bits, n);
                assert_eq!(o, Ordering::Equal);
            },
        );
    });
}

#[test]
fn limbs_shr_round_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_rounding_mode_triple_gen_var_2().test_properties_with_config(
        &config,
        |(xs, bits, rm)| {
            let n = Natural::from_limbs_asc(&xs);
            limbs_shr_round(&xs, bits, rm).map_or_else(
                || {
                    assert_eq!(rm, RoundingMode::Exact);
                    assert!(!n.divisible_by_power_of_2(bits));
                },
                |(result_xs, o)| {
                    let (m, o_alt) = (&n).shr_round(bits, rm);
                    assert_eq!(Natural::from_owned_limbs_asc(result_xs), m);
                    assert_eq!(o, o_alt);
                    if rm == RoundingMode::Exact {
                        assert_eq!(m << bits, n);
                    }
                },
            );
        },
    );
}

#[test]
fn limbs_vec_shr_round_up_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_20().test_properties_with_config(
        &config,
        |(mut xs, bits)| {
            let old_xs = xs.clone();
            let o = limbs_vec_shr_round_up_in_place(&mut xs, bits);
            let (n, o_alt) =
                Natural::from_owned_limbs_asc(old_xs).shr_round(bits, RoundingMode::Up);
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
            assert_eq!(o, o_alt);
        },
    );
}

#[test]
fn limbs_vec_shr_round_nearest_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(
        &config,
        |(mut xs, bits)| {
            let old_xs = xs.clone();
            let o = limbs_vec_shr_round_nearest_in_place(&mut xs, bits);
            let (n, o_alt) =
                Natural::from_owned_limbs_asc(old_xs).shr_round(bits, RoundingMode::Nearest);
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
            assert_eq!(o, o_alt);
        },
    );
}

#[test]
fn limbs_vec_shr_exact_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_20().test_properties_with_config(
        &config,
        |(mut xs, bits)| {
            let n = Natural::from_limbs_asc(&xs);
            if limbs_vec_shr_exact_in_place(&mut xs, bits) {
                let (m, o) = (&n).shr_round(bits, RoundingMode::Exact);
                assert_eq!(Natural::from_owned_limbs_asc(xs), m);
                assert_eq!(m << bits, n);
                assert_eq!(o, Ordering::Equal);
            } else {
                assert!(!n.divisible_by_power_of_2(bits));
            }
        },
    );
}

#[test]
fn limbs_vec_shr_round_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_rounding_mode_triple_gen_var_2().test_properties_with_config(
        &config,
        |(mut xs, bits, rm)| {
            let n = Natural::from_limbs_asc(&xs);
            let (b, o) = limbs_vec_shr_round_in_place(&mut xs, bits, rm);
            if b {
                let (m, o_alt) = (&n).shr_round(bits, rm);
                assert_eq!(Natural::from_owned_limbs_asc(xs), m);
                assert_eq!(o, o_alt);
                if rm == RoundingMode::Exact {
                    assert_eq!(m << bits, n);
                }
            } else {
                assert_eq!(rm, RoundingMode::Exact);
                assert!(!n.divisible_by_power_of_2(bits));
            }
        },
    );
}

fn unsigned_properties<T: PrimitiveUnsigned>()
where
    Natural: Shl<T, Output = Natural> + ShrRound<T, Output = Natural> + ShrRoundAssign<T>,
    for<'a> &'a Natural: Shl<T, Output = Natural> + ShrRound<T, Output = Natural>,
    Limb: ShrRound<T, Output = Limb>,
{
    natural_unsigned_rounding_mode_triple_gen_var_1::<T>().test_properties(|(n, u, rm)| {
        let mut mut_n = n.clone();
        let o = mut_n.shr_round_assign(u, rm);
        assert!(mut_n.is_valid());
        let shifted = mut_n;

        let (shifted_alt, o_alt) = (&n).shr_round(u, rm);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        assert_eq!(o, o_alt);

        let (shifted_alt, o_alt) = n.clone().shr_round(u, rm);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        assert_eq!(o, o_alt);

        assert!(shifted <= n);
        let (shifted_alt, o_alt) = (&n).div_round(Natural::ONE << u, rm);
        assert_eq!(shifted_alt, shifted);
        assert_eq!(o_alt, o);
        assert_eq!(
            n.divisible_by_power_of_2(u.exact_into()),
            o == Ordering::Equal
        );

        assert_eq!((shifted << u).cmp(&n), o);
        match rm {
            RoundingMode::Floor | RoundingMode::Down => assert_ne!(o, Ordering::Greater),
            RoundingMode::Ceiling | RoundingMode::Up => assert_ne!(o, Ordering::Less),
            RoundingMode::Exact => assert_eq!(o, Ordering::Equal),
            _ => {}
        }
    });

    natural_unsigned_pair_gen_var_4::<T>().test_properties(|(n, u)| {
        let left_shifted = &n << u;
        let no = (n, Ordering::Equal);
        assert_eq!((&left_shifted).shr_round(u, RoundingMode::Down), no);
        assert_eq!((&left_shifted).shr_round(u, RoundingMode::Up), no);
        assert_eq!((&left_shifted).shr_round(u, RoundingMode::Floor), no);
        assert_eq!((&left_shifted).shr_round(u, RoundingMode::Ceiling), no);
        assert_eq!((&left_shifted).shr_round(u, RoundingMode::Nearest), no);
        assert_eq!((&left_shifted).shr_round(u, RoundingMode::Exact), no);
    });

    natural_unsigned_pair_gen_var_10::<T>().test_properties(|(n, u)| {
        let down = (&n).shr_round(u, RoundingMode::Down);
        let up = (&down.0 + Natural::ONE, Ordering::Greater);
        assert_eq!((&n).shr_round(u, RoundingMode::Up), up);
        assert_eq!((&n).shr_round(u, RoundingMode::Floor), down);
        assert_eq!((&n).shr_round(u, RoundingMode::Ceiling), up);
        let nearest = n.shr_round(u, RoundingMode::Nearest);
        assert!(nearest == down || nearest == up);
    });

    unsigned_pair_gen_var_37::<Limb, T>().test_properties(|(u, v)| {
        if let Some(shift) = v.checked_add(T::exact_from(Limb::WIDTH)) {
            assert_eq!(
                Natural::from(u).shr_round(shift, RoundingMode::Down),
                (Natural::ZERO, Ordering::Less)
            );
            assert_eq!(
                Natural::from(u).shr_round(shift, RoundingMode::Floor),
                (Natural::ZERO, Ordering::Less)
            );
            assert_eq!(
                Natural::from(u).shr_round(shift, RoundingMode::Up),
                (Natural::ONE, Ordering::Greater)
            );
            assert_eq!(
                Natural::from(u).shr_round(shift, RoundingMode::Ceiling),
                (Natural::ONE, Ordering::Greater)
            );
            if let Some(extra_shift) = shift.checked_add(T::ONE) {
                assert_eq!(
                    Natural::from(u).shr_round(extra_shift, RoundingMode::Nearest),
                    (Natural::ZERO, Ordering::Less)
                );
            }
        }
    });

    natural_rounding_mode_pair_gen().test_properties(|(n, rm)| {
        assert_eq!((&n).shr_round(T::ZERO, rm), (n, Ordering::Equal));
    });

    unsigned_rounding_mode_pair_gen::<T>().test_properties(|(u, rm)| {
        assert_eq!(
            Natural::ZERO.shr_round(u, rm),
            (Natural::ZERO, Ordering::Equal)
        );
    });

    unsigned_unsigned_rounding_mode_triple_gen_var_4::<Limb, T>().test_properties(|(n, u, rm)| {
        let (s, o) = n.shr_round(u, rm);
        assert_eq!((Natural::from(s), o), Natural::from(n).shr_round(u, rm));
    });
}

fn signed_properties<T: PrimitiveSigned>()
where
    Natural: Shl<T, Output = Natural>
        + ShlRound<T, Output = Natural>
        + ShrRound<T, Output = Natural>
        + ShrRoundAssign<T>,
    for<'a> &'a Natural: ShlRound<T, Output = Natural> + ShrRound<T, Output = Natural>,
    Limb: ArithmeticCheckedShr<T, Output = Limb> + ShrRound<T, Output = Limb>,
{
    natural_signed_rounding_mode_triple_gen_var_2::<T>().test_properties(|(n, i, rm)| {
        let mut mut_n = n.clone();
        let o = mut_n.shr_round_assign(i, rm);
        assert!(mut_n.is_valid());
        let shifted = mut_n;

        let (shifted_alt, o_alt) = (&n).shr_round(i, rm);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        assert_eq!(o_alt, o);

        let (shifted_alt, o_alt) = n.clone().shr_round(i, rm);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        assert_eq!(o_alt, o);

        if i != T::MIN {
            let (shifted_alt, o_alt) = (&n).shl_round(-i, rm);
            assert_eq!(shifted_alt, shifted);
            assert_eq!(o_alt, o);
        }
        assert_eq!(
            i <= T::ZERO || n.divisible_by_power_of_2(i.exact_into()),
            o == Ordering::Equal
        );

        if i >= T::ZERO {
            assert_eq!((shifted << i).cmp(&n), o);
        }
        match rm {
            RoundingMode::Floor | RoundingMode::Down => assert_ne!(o, Ordering::Greater),
            RoundingMode::Ceiling | RoundingMode::Up => assert_ne!(o, Ordering::Less),
            RoundingMode::Exact => assert_eq!(o, Ordering::Equal),
            _ => {}
        }
    });

    natural_rounding_mode_pair_gen().test_properties(|(n, rm)| {
        assert_eq!((&n).shr_round(T::ZERO, rm), (n, Ordering::Equal));
    });

    signed_rounding_mode_pair_gen::<T>().test_properties(|(i, rm)| {
        assert_eq!(
            Natural::ZERO.shr_round(i, rm),
            (Natural::ZERO, Ordering::Equal)
        );
    });

    unsigned_signed_rounding_mode_triple_gen_var_1::<Limb, T>().test_properties(|(n, i, rm)| {
        if n.arithmetic_checked_shr(i).is_some() {
            let (s, o) = n.shr_round(i, rm);
            assert_eq!((Natural::from(s), o), Natural::from(n).shr_round(i, rm));
        }
    });
}

#[test]
fn shr_round_properties() {
    apply_fn_to_unsigneds!(unsigned_properties);
    apply_fn_to_signeds!(signed_properties);
}
