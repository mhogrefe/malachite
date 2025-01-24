// Copyright © 2025 Mikhail Hogrefe
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
    ArithmeticCheckedShr, DivRound, DivisibleByPowerOf2, ShlRound, ShrRound, ShrRoundAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
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
    test(&[1], 0, &[1], Equal);
    test(&[1], 1, &[1], Greater);
    test(&[3], 1, &[2], Greater);
    test(&[122, 456], 1, &[61, 228], Equal);
    test(&[123, 456], 0, &[123, 456], Equal);
    test(&[123, 456], 1, &[62, 228], Greater);
    test(&[123, 455], 1, &[2147483710, 227], Greater);
    test(&[123, 456], 31, &[913, 0], Greater);
    test(&[123, 456], 32, &[457], Greater);
    test(&[123, 456], 100, &[1], Greater);
    test(&[256, 456], 8, &[3355443201, 1], Equal);
    test(&[u32::MAX, 1], 1, &[0, 1], Greater);
    test(&[u32::MAX, u32::MAX], 32, &[0, 1], Greater);
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
    test(&[], 0, &[], Equal);
    test(&[], 1, &[], Equal);
    test(&[], 100, &[], Equal);
    test(&[0, 0, 0], 0, &[0, 0, 0], Equal);
    test(&[0, 0, 0], 1, &[0, 0, 0], Equal);
    test(&[0, 0, 0], 100, &[], Equal);
    test(&[1], 0, &[1], Equal);
    test(&[1], 1, &[0], Less);
    test(&[3], 1, &[2], Greater);
    test(&[122, 456], 1, &[61, 228], Equal);
    test(&[123, 456], 0, &[123, 456], Equal);
    test(&[123, 456], 1, &[62, 228], Greater);
    test(&[123, 455], 1, &[2147483710, 227], Greater);
    test(&[123, 456], 31, &[912, 0], Less);
    test(&[123, 456], 32, &[456], Less);
    test(&[123, 456], 100, &[], Less);
    test(&[256, 456], 8, &[3355443201, 1], Equal);
    test(&[u32::MAX, 1], 1, &[0, 1], Greater);
    test(&[u32::MAX, u32::MAX], 32, &[0, 1], Greater);
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
            assert_eq!(None, out);
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
            assert!(out.is_none());
        }
        assert_eq!(o_alt, o);
    };
    test(&[1], 0, Nearest, Some(&[1]), Equal);
    test(&[1], 1, Up, Some(&[1]), Greater);
    test(&[3], 1, Nearest, Some(&[2]), Greater);
    test(&[122, 456], 1, Floor, Some(&[61, 228]), Equal);
    test(&[123, 456], 0, Floor, Some(&[123, 456]), Equal);
    test(&[123, 456], 1, Down, Some(&[61, 228]), Less);
    test(&[123, 455], 1, Floor, Some(&[2147483709, 227]), Less);
    test(&[123, 456], 31, Ceiling, Some(&[913, 0]), Greater);
    test(&[123, 456], 32, Up, Some(&[457]), Greater);
    test(&[123, 456], 100, Down, Some(&[]), Less);
    test(&[256, 456], 8, Exact, Some(&[3355443201, 1]), Equal);
    test(&[u32::MAX, 1], 1, Exact, None, Equal);
    test(&[u32::MAX, u32::MAX], 32, Down, Some(&[u32::MAX]), Less);
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

    test("245", 1, Down, "122", Less);
    test("245", 1, Up, "123", Greater);
    test("245", 1, Floor, "122", Less);
    test("245", 1, Ceiling, "123", Greater);
    test("245", 1, Nearest, "122", Less);

    test("246", 1, Down, "123", Equal);
    test("246", 1, Up, "123", Equal);
    test("246", 1, Floor, "123", Equal);
    test("246", 1, Ceiling, "123", Equal);
    test("246", 1, Nearest, "123", Equal);
    test("246", 1, Exact, "123", Equal);

    test("247", 1, Down, "123", Less);
    test("247", 1, Up, "124", Greater);
    test("247", 1, Floor, "123", Less);
    test("247", 1, Ceiling, "124", Greater);
    test("247", 1, Nearest, "124", Greater);

    test("491", 2, Down, "122", Less);
    test("491", 2, Up, "123", Greater);
    test("491", 2, Floor, "122", Less);
    test("491", 2, Ceiling, "123", Greater);
    test("491", 2, Nearest, "123", Greater);

    test("492", 2, Down, "123", Equal);
    test("492", 2, Up, "123", Equal);
    test("492", 2, Floor, "123", Equal);
    test("492", 2, Ceiling, "123", Equal);
    test("492", 2, Nearest, "123", Equal);
    test("492", 2, Exact, "123", Equal);

    test("493", 2, Down, "123", Less);
    test("493", 2, Up, "124", Greater);
    test("493", 2, Floor, "123", Less);
    test("493", 2, Ceiling, "124", Greater);
    test("493", 2, Nearest, "123", Less);

    test("4127195135", 25, Down, "122", Less);
    test("4127195135", 25, Up, "123", Greater);
    test("4127195135", 25, Floor, "122", Less);
    test("4127195135", 25, Ceiling, "123", Greater);
    test("4127195135", 25, Nearest, "123", Greater);

    test("4127195136", 25, Down, "123", Equal);
    test("4127195136", 25, Up, "123", Equal);
    test("4127195136", 25, Floor, "123", Equal);
    test("4127195136", 25, Ceiling, "123", Equal);
    test("4127195136", 25, Nearest, "123", Equal);
    test("4127195136", 25, Exact, "123", Equal);

    test("4127195137", 25, Down, "123", Less);
    test("4127195137", 25, Up, "124", Greater);
    test("4127195137", 25, Floor, "123", Less);
    test("4127195137", 25, Ceiling, "124", Greater);
    test("4127195137", 25, Nearest, "123", Less);

    test("8254390271", 26, Down, "122", Less);
    test("8254390271", 26, Up, "123", Greater);
    test("8254390271", 26, Floor, "122", Less);
    test("8254390271", 26, Ceiling, "123", Greater);
    test("8254390271", 26, Nearest, "123", Greater);

    test("8254390272", 26, Down, "123", Equal);
    test("8254390272", 26, Up, "123", Equal);
    test("8254390272", 26, Floor, "123", Equal);
    test("8254390272", 26, Ceiling, "123", Equal);
    test("8254390272", 26, Nearest, "123", Equal);
    test("8254390272", 26, Exact, "123", Equal);

    test("8254390273", 26, Down, "123", Less);
    test("8254390273", 26, Up, "124", Greater);
    test("8254390273", 26, Floor, "123", Less);
    test("8254390273", 26, Ceiling, "124", Greater);
    test("8254390273", 26, Nearest, "123", Less);

    test("155921023828072216384094494261247", 100, Down, "122", Less);
    test("155921023828072216384094494261247", 100, Up, "123", Greater);
    test("155921023828072216384094494261247", 100, Floor, "122", Less);
    test(
        "155921023828072216384094494261247",
        100,
        Ceiling,
        "123",
        Greater,
    );
    test(
        "155921023828072216384094494261247",
        100,
        Nearest,
        "123",
        Greater,
    );

    test("155921023828072216384094494261248", 100, Down, "123", Equal);
    test("155921023828072216384094494261248", 100, Up, "123", Equal);
    test(
        "155921023828072216384094494261248",
        100,
        Floor,
        "123",
        Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        Ceiling,
        "123",
        Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        Nearest,
        "123",
        Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        Exact,
        "123",
        Equal,
    );

    test("155921023828072216384094494261249", 100, Down, "123", Less);
    test("155921023828072216384094494261249", 100, Up, "124", Greater);
    test("155921023828072216384094494261249", 100, Floor, "123", Less);
    test(
        "155921023828072216384094494261249",
        100,
        Ceiling,
        "124",
        Greater,
    );
    test(
        "155921023828072216384094494261249",
        100,
        Nearest,
        "123",
        Less,
    );

    test("4294967295", 1, Down, "2147483647", Less);
    test("4294967295", 1, Up, "2147483648", Greater);
    test("4294967295", 1, Floor, "2147483647", Less);
    test("4294967295", 1, Ceiling, "2147483648", Greater);
    test("4294967295", 1, Nearest, "2147483648", Greater);

    test("4294967296", 1, Down, "2147483648", Equal);
    test("4294967296", 1, Up, "2147483648", Equal);
    test("4294967296", 1, Floor, "2147483648", Equal);
    test("4294967296", 1, Ceiling, "2147483648", Equal);
    test("4294967296", 1, Nearest, "2147483648", Equal);
    test("4294967296", 1, Exact, "2147483648", Equal);

    test("4294967297", 1, Down, "2147483648", Less);
    test("4294967297", 1, Up, "2147483649", Greater);
    test("4294967297", 1, Floor, "2147483648", Less);
    test("4294967297", 1, Ceiling, "2147483649", Greater);
    test("4294967297", 1, Nearest, "2147483648", Less);

    test("1000000000000", 0, Down, "1000000000000", Equal);
    test("1000000000000", 0, Up, "1000000000000", Equal);
    test("1000000000000", 0, Floor, "1000000000000", Equal);
    test("1000000000000", 0, Ceiling, "1000000000000", Equal);
    test("1000000000000", 0, Nearest, "1000000000000", Equal);
    test("1000000000000", 0, Exact, "1000000000000", Equal);

    test("7999999999999", 3, Down, "999999999999", Less);
    test("7999999999999", 3, Up, "1000000000000", Greater);
    test("7999999999999", 3, Floor, "999999999999", Less);
    test("7999999999999", 3, Ceiling, "1000000000000", Greater);
    test("7999999999999", 3, Nearest, "1000000000000", Greater);

    test("8000000000000", 3, Down, "1000000000000", Equal);
    test("8000000000000", 3, Up, "1000000000000", Equal);
    test("8000000000000", 3, Floor, "1000000000000", Equal);
    test("8000000000000", 3, Ceiling, "1000000000000", Equal);
    test("8000000000000", 3, Nearest, "1000000000000", Equal);
    test("8000000000000", 3, Exact, "1000000000000", Equal);

    test("8000000000001", 3, Down, "1000000000000", Less);
    test("8000000000001", 3, Up, "1000000000001", Greater);
    test("8000000000001", 3, Floor, "1000000000000", Less);
    test("8000000000001", 3, Ceiling, "1000000000001", Greater);
    test("8000000000001", 3, Nearest, "1000000000000", Less);

    test("16777216000000000000", 24, Down, "1000000000000", Equal);
    test("16777216000000000000", 24, Up, "1000000000000", Equal);
    test("16777216000000000000", 24, Floor, "1000000000000", Equal);
    test("16777216000000000000", 24, Ceiling, "1000000000000", Equal);
    test("16777216000000000000", 24, Nearest, "1000000000000", Equal);
    test("16777216000000000000", 24, Exact, "1000000000000", Equal);

    test("33554432000000000000", 25, Down, "1000000000000", Equal);
    test("33554432000000000000", 25, Up, "1000000000000", Equal);
    test("33554432000000000000", 25, Floor, "1000000000000", Equal);
    test("33554432000000000000", 25, Ceiling, "1000000000000", Equal);
    test("33554432000000000000", 25, Nearest, "1000000000000", Equal);
    test("33554432000000000000", 25, Exact, "1000000000000", Equal);

    test("2147483648000000000000", 31, Down, "1000000000000", Equal);
    test("2147483648000000000000", 31, Up, "1000000000000", Equal);
    test("2147483648000000000000", 31, Floor, "1000000000000", Equal);
    test(
        "2147483648000000000000",
        31,
        Ceiling,
        "1000000000000",
        Equal,
    );
    test(
        "2147483648000000000000",
        31,
        Nearest,
        "1000000000000",
        Equal,
    );
    test("2147483648000000000000", 31, Exact, "1000000000000", Equal);

    test("4294967296000000000000", 32, Down, "1000000000000", Equal);
    test("4294967296000000000000", 32, Up, "1000000000000", Equal);
    test("4294967296000000000000", 32, Floor, "1000000000000", Equal);
    test(
        "4294967296000000000000",
        32,
        Ceiling,
        "1000000000000",
        Equal,
    );
    test(
        "4294967296000000000000",
        32,
        Nearest,
        "1000000000000",
        Equal,
    );
    test("4294967296000000000000", 32, Exact, "1000000000000", Equal);

    test("8589934592000000000000", 33, Down, "1000000000000", Equal);
    test("8589934592000000000000", 33, Up, "1000000000000", Equal);
    test("8589934592000000000000", 33, Floor, "1000000000000", Equal);
    test(
        "8589934592000000000000",
        33,
        Ceiling,
        "1000000000000",
        Equal,
    );
    test(
        "8589934592000000000000",
        33,
        Nearest,
        "1000000000000",
        Equal,
    );
    test("8589934592000000000000", 33, Exact, "1000000000000", Equal);

    test(
        "1267650600228229401496703205376000000000000",
        100,
        Down,
        "1000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Up,
        "1000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Floor,
        "1000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Ceiling,
        "1000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Nearest,
        "1000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Exact,
        "1000000000000",
        Equal,
    );

    test("1000000000000", 10, Down, "976562500", Equal);
    test("1000000000000", 10, Up, "976562500", Equal);
    test("1000000000000", 10, Floor, "976562500", Equal);
    test("1000000000000", 10, Ceiling, "976562500", Equal);
    test("1000000000000", 10, Nearest, "976562500", Equal);
    test("1000000000000", 10, Exact, "976562500", Equal);

    test("980657949", 72, Down, "0", Less);
    test("980657949", 72, Up, "1", Greater);
    test("980657949", 72, Floor, "0", Less);
    test("980657949", 72, Ceiling, "1", Greater);
    test("980657949", 72, Nearest, "0", Less);

    test("4294967295", 31, Down, "1", Less);
    test("4294967295", 31, Up, "2", Greater);
    test("4294967295", 31, Floor, "1", Less);
    test("4294967295", 31, Ceiling, "2", Greater);
    test("4294967295", 31, Nearest, "2", Greater);

    test("4294967295", 32, Down, "0", Less);
    test("4294967295", 32, Up, "1", Greater);
    test("4294967295", 32, Floor, "0", Less);
    test("4294967295", 32, Ceiling, "1", Greater);
    test("4294967295", 32, Nearest, "1", Greater);

    test("4294967296", 32, Down, "1", Equal);
    test("4294967296", 32, Up, "1", Equal);
    test("4294967296", 32, Floor, "1", Equal);
    test("4294967296", 32, Ceiling, "1", Equal);
    test("4294967296", 32, Nearest, "1", Equal);
    test("4294967296", 32, Exact, "1", Equal);

    test("4294967296", 33, Down, "0", Less);
    test("4294967296", 33, Up, "1", Greater);
    test("4294967296", 33, Floor, "0", Less);
    test("4294967296", 33, Ceiling, "1", Greater);
    test("4294967296", 33, Nearest, "0", Less);
}

#[test]
fn test_shr_round_unsigned() {
    apply_fn_to_unsigneds!(test_shr_round_unsigned_helper);
}

macro_rules! shr_round_unsigned_fail_helper {
    ($t:ident) => {
        assert_panic!(Natural::from(123u32).shr_round_assign($t::ONE, Exact));
        assert_panic!(Natural::from(123u32).shr_round_assign($t::exact_from(100), Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shr_round_assign($t::ONE, Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shr_round_assign($t::exact_from(100), Exact));
        assert_panic!(Natural::from(123u32).shr_round($t::ONE, Exact));
        assert_panic!(Natural::from(123u32).shr_round($t::exact_from(100), Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shr_round($t::ONE, Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shr_round($t::exact_from(100), Exact));
        assert_panic!((&Natural::from(123u32)).shr_round($t::ONE, Exact));
        assert_panic!((&Natural::from(123u32)).shr_round($t::exact_from(100), Exact));
        assert_panic!((&Natural::from_str("1000000000001").unwrap()).shr_round($t::ONE, Exact));
        assert_panic!(
            (&Natural::from_str("1000000000001").unwrap()).shr_round($t::exact_from(100), Exact)
        );
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

    test("245", 1, Down, "122", Less);
    test("245", 1, Up, "123", Greater);
    test("245", 1, Floor, "122", Less);
    test("245", 1, Ceiling, "123", Greater);
    test("245", 1, Nearest, "122", Less);

    test("246", 1, Down, "123", Equal);
    test("246", 1, Up, "123", Equal);
    test("246", 1, Floor, "123", Equal);
    test("246", 1, Ceiling, "123", Equal);
    test("246", 1, Nearest, "123", Equal);
    test("246", 1, Exact, "123", Equal);

    test("247", 1, Down, "123", Less);
    test("247", 1, Up, "124", Greater);
    test("247", 1, Floor, "123", Less);
    test("247", 1, Ceiling, "124", Greater);
    test("247", 1, Nearest, "124", Greater);

    test("491", 2, Down, "122", Less);
    test("491", 2, Up, "123", Greater);
    test("491", 2, Floor, "122", Less);
    test("491", 2, Ceiling, "123", Greater);
    test("491", 2, Nearest, "123", Greater);

    test("492", 2, Down, "123", Equal);
    test("492", 2, Up, "123", Equal);
    test("492", 2, Floor, "123", Equal);
    test("492", 2, Ceiling, "123", Equal);
    test("492", 2, Nearest, "123", Equal);
    test("492", 2, Exact, "123", Equal);

    test("493", 2, Down, "123", Less);
    test("493", 2, Up, "124", Greater);
    test("493", 2, Floor, "123", Less);
    test("493", 2, Ceiling, "124", Greater);
    test("493", 2, Nearest, "123", Less);

    test("4127195135", 25, Down, "122", Less);
    test("4127195135", 25, Up, "123", Greater);
    test("4127195135", 25, Floor, "122", Less);
    test("4127195135", 25, Ceiling, "123", Greater);
    test("4127195135", 25, Nearest, "123", Greater);

    test("4127195136", 25, Down, "123", Equal);
    test("4127195136", 25, Up, "123", Equal);
    test("4127195136", 25, Floor, "123", Equal);
    test("4127195136", 25, Ceiling, "123", Equal);
    test("4127195136", 25, Nearest, "123", Equal);
    test("4127195136", 25, Exact, "123", Equal);

    test("4127195137", 25, Down, "123", Less);
    test("4127195137", 25, Up, "124", Greater);
    test("4127195137", 25, Floor, "123", Less);
    test("4127195137", 25, Ceiling, "124", Greater);
    test("4127195137", 25, Nearest, "123", Less);

    test("8254390271", 26, Down, "122", Less);
    test("8254390271", 26, Up, "123", Greater);
    test("8254390271", 26, Floor, "122", Less);
    test("8254390271", 26, Ceiling, "123", Greater);
    test("8254390271", 26, Nearest, "123", Greater);

    test("8254390272", 26, Down, "123", Equal);
    test("8254390272", 26, Up, "123", Equal);
    test("8254390272", 26, Floor, "123", Equal);
    test("8254390272", 26, Ceiling, "123", Equal);
    test("8254390272", 26, Nearest, "123", Equal);
    test("8254390272", 26, Exact, "123", Equal);

    test("8254390273", 26, Down, "123", Less);
    test("8254390273", 26, Up, "124", Greater);
    test("8254390273", 26, Floor, "123", Less);
    test("8254390273", 26, Ceiling, "124", Greater);
    test("8254390273", 26, Nearest, "123", Less);

    test("155921023828072216384094494261247", 100, Down, "122", Less);
    test("155921023828072216384094494261247", 100, Up, "123", Greater);
    test("155921023828072216384094494261247", 100, Floor, "122", Less);
    test(
        "155921023828072216384094494261247",
        100,
        Ceiling,
        "123",
        Greater,
    );
    test(
        "155921023828072216384094494261247",
        100,
        Nearest,
        "123",
        Greater,
    );

    test("155921023828072216384094494261248", 100, Down, "123", Equal);
    test("155921023828072216384094494261248", 100, Up, "123", Equal);
    test(
        "155921023828072216384094494261248",
        100,
        Floor,
        "123",
        Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        Ceiling,
        "123",
        Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        Nearest,
        "123",
        Equal,
    );
    test(
        "155921023828072216384094494261248",
        100,
        Exact,
        "123",
        Equal,
    );

    test("155921023828072216384094494261249", 100, Down, "123", Less);
    test("155921023828072216384094494261249", 100, Up, "124", Greater);
    test("155921023828072216384094494261249", 100, Floor, "123", Less);
    test(
        "155921023828072216384094494261249",
        100,
        Ceiling,
        "124",
        Greater,
    );
    test(
        "155921023828072216384094494261249",
        100,
        Nearest,
        "123",
        Less,
    );

    test("4294967295", 1, Down, "2147483647", Less);
    test("4294967295", 1, Up, "2147483648", Greater);
    test("4294967295", 1, Floor, "2147483647", Less);
    test("4294967295", 1, Ceiling, "2147483648", Greater);
    test("4294967295", 1, Nearest, "2147483648", Greater);

    test("4294967296", 1, Down, "2147483648", Equal);
    test("4294967296", 1, Up, "2147483648", Equal);
    test("4294967296", 1, Floor, "2147483648", Equal);
    test("4294967296", 1, Ceiling, "2147483648", Equal);
    test("4294967296", 1, Nearest, "2147483648", Equal);
    test("4294967296", 1, Exact, "2147483648", Equal);

    test("4294967297", 1, Down, "2147483648", Less);
    test("4294967297", 1, Up, "2147483649", Greater);
    test("4294967297", 1, Floor, "2147483648", Less);
    test("4294967297", 1, Ceiling, "2147483649", Greater);
    test("4294967297", 1, Nearest, "2147483648", Less);

    test("1000000000000", 0, Down, "1000000000000", Equal);
    test("1000000000000", 0, Up, "1000000000000", Equal);
    test("1000000000000", 0, Floor, "1000000000000", Equal);
    test("1000000000000", 0, Ceiling, "1000000000000", Equal);
    test("1000000000000", 0, Nearest, "1000000000000", Equal);
    test("1000000000000", 0, Exact, "1000000000000", Equal);

    test("7999999999999", 3, Down, "999999999999", Less);
    test("7999999999999", 3, Up, "1000000000000", Greater);
    test("7999999999999", 3, Floor, "999999999999", Less);
    test("7999999999999", 3, Ceiling, "1000000000000", Greater);
    test("7999999999999", 3, Nearest, "1000000000000", Greater);

    test("8000000000000", 3, Down, "1000000000000", Equal);
    test("8000000000000", 3, Up, "1000000000000", Equal);
    test("8000000000000", 3, Floor, "1000000000000", Equal);
    test("8000000000000", 3, Ceiling, "1000000000000", Equal);
    test("8000000000000", 3, Nearest, "1000000000000", Equal);
    test("8000000000000", 3, Exact, "1000000000000", Equal);

    test("8000000000001", 3, Down, "1000000000000", Less);
    test("8000000000001", 3, Up, "1000000000001", Greater);
    test("8000000000001", 3, Floor, "1000000000000", Less);
    test("8000000000001", 3, Ceiling, "1000000000001", Greater);
    test("8000000000001", 3, Nearest, "1000000000000", Less);

    test("16777216000000000000", 24, Down, "1000000000000", Equal);
    test("16777216000000000000", 24, Up, "1000000000000", Equal);
    test("16777216000000000000", 24, Floor, "1000000000000", Equal);
    test("16777216000000000000", 24, Ceiling, "1000000000000", Equal);
    test("16777216000000000000", 24, Nearest, "1000000000000", Equal);
    test("16777216000000000000", 24, Exact, "1000000000000", Equal);

    test("33554432000000000000", 25, Down, "1000000000000", Equal);
    test("33554432000000000000", 25, Up, "1000000000000", Equal);
    test("33554432000000000000", 25, Floor, "1000000000000", Equal);
    test("33554432000000000000", 25, Ceiling, "1000000000000", Equal);
    test("33554432000000000000", 25, Nearest, "1000000000000", Equal);
    test("33554432000000000000", 25, Exact, "1000000000000", Equal);

    test("2147483648000000000000", 31, Down, "1000000000000", Equal);
    test("2147483648000000000000", 31, Up, "1000000000000", Equal);
    test("2147483648000000000000", 31, Floor, "1000000000000", Equal);
    test(
        "2147483648000000000000",
        31,
        Ceiling,
        "1000000000000",
        Equal,
    );
    test(
        "2147483648000000000000",
        31,
        Nearest,
        "1000000000000",
        Equal,
    );
    test("2147483648000000000000", 31, Exact, "1000000000000", Equal);

    test("4294967296000000000000", 32, Down, "1000000000000", Equal);
    test("4294967296000000000000", 32, Up, "1000000000000", Equal);
    test("4294967296000000000000", 32, Floor, "1000000000000", Equal);
    test(
        "4294967296000000000000",
        32,
        Ceiling,
        "1000000000000",
        Equal,
    );
    test(
        "4294967296000000000000",
        32,
        Nearest,
        "1000000000000",
        Equal,
    );
    test("4294967296000000000000", 32, Exact, "1000000000000", Equal);

    test("8589934592000000000000", 33, Down, "1000000000000", Equal);
    test("8589934592000000000000", 33, Up, "1000000000000", Equal);
    test("8589934592000000000000", 33, Floor, "1000000000000", Equal);
    test(
        "8589934592000000000000",
        33,
        Ceiling,
        "1000000000000",
        Equal,
    );
    test(
        "8589934592000000000000",
        33,
        Nearest,
        "1000000000000",
        Equal,
    );
    test("8589934592000000000000", 33, Exact, "1000000000000", Equal);

    test(
        "1267650600228229401496703205376000000000000",
        100,
        Down,
        "1000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Up,
        "1000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Floor,
        "1000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Ceiling,
        "1000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Nearest,
        "1000000000000",
        Equal,
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        Exact,
        "1000000000000",
        Equal,
    );

    test("1000000000000", 10, Down, "976562500", Equal);
    test("1000000000000", 10, Up, "976562500", Equal);
    test("1000000000000", 10, Floor, "976562500", Equal);
    test("1000000000000", 10, Ceiling, "976562500", Equal);
    test("1000000000000", 10, Nearest, "976562500", Equal);
    test("1000000000000", 10, Exact, "976562500", Equal);

    test("980657949", 72, Down, "0", Less);
    test("980657949", 72, Up, "1", Greater);
    test("980657949", 72, Floor, "0", Less);
    test("980657949", 72, Ceiling, "1", Greater);
    test("980657949", 72, Nearest, "0", Less);

    test("4294967295", 31, Down, "1", Less);
    test("4294967295", 31, Up, "2", Greater);
    test("4294967295", 31, Floor, "1", Less);
    test("4294967295", 31, Ceiling, "2", Greater);
    test("4294967295", 31, Nearest, "2", Greater);

    test("4294967295", 32, Down, "0", Less);
    test("4294967295", 32, Up, "1", Greater);
    test("4294967295", 32, Floor, "0", Less);
    test("4294967295", 32, Ceiling, "1", Greater);
    test("4294967295", 32, Nearest, "1", Greater);

    test("4294967296", 32, Down, "1", Equal);
    test("4294967296", 32, Up, "1", Equal);
    test("4294967296", 32, Floor, "1", Equal);
    test("4294967296", 32, Ceiling, "1", Equal);
    test("4294967296", 32, Nearest, "1", Equal);
    test("4294967296", 32, Exact, "1", Equal);

    test("4294967296", 33, Down, "0", Less);
    test("4294967296", 33, Up, "1", Greater);
    test("4294967296", 33, Floor, "0", Less);
    test("4294967296", 33, Ceiling, "1", Greater);
    test("4294967296", 33, Nearest, "0", Less);

    test("0", -10, Exact, "0", Equal);
    test("123", -1, Exact, "246", Equal);
    test("123", -2, Exact, "492", Equal);
    test("123", -25, Exact, "4127195136", Equal);
    test("123", -26, Exact, "8254390272", Equal);
    test(
        "123",
        -100,
        Exact,
        "155921023828072216384094494261248",
        Equal,
    );
    test("2147483648", -1, Exact, "4294967296", Equal);
    test("1000000000000", -3, Exact, "8000000000000", Equal);
    test("1000000000000", -24, Exact, "16777216000000000000", Equal);
    test("1000000000000", -25, Exact, "33554432000000000000", Equal);
    test("1000000000000", -31, Exact, "2147483648000000000000", Equal);
    test("1000000000000", -32, Exact, "4294967296000000000000", Equal);
    test("1000000000000", -33, Exact, "8589934592000000000000", Equal);
    test(
        "1000000000000",
        -100,
        Exact,
        "1267650600228229401496703205376000000000000",
        Equal,
    );
}

#[test]
fn test_shr_round_signed() {
    apply_fn_to_signeds!(test_shr_round_signed_helper);
}

macro_rules! shr_round_signed_fail_helper {
    ($t:ident) => {
        assert_panic!(Natural::from(123u32).shr_round_assign($t::ONE, Exact));
        assert_panic!(Natural::from(123u32).shr_round_assign($t::exact_from(100), Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shr_round_assign($t::ONE, Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shr_round_assign($t::exact_from(100), Exact));
        assert_panic!(Natural::from(123u32).shr_round($t::ONE, Exact));
        assert_panic!(Natural::from(123u32).shr_round($t::exact_from(100), Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shr_round($t::ONE, Exact));
        assert_panic!(Natural::from_str("1000000000001")
            .unwrap()
            .shr_round($t::exact_from(100), Exact));
        assert_panic!((&Natural::from(123u32)).shr_round($t::ONE, Exact));
        assert_panic!((&Natural::from(123u32)).shr_round($t::exact_from(100), Exact));
        assert_panic!((&Natural::from_str("1000000000001").unwrap()).shr_round($t::ONE, Exact));
        assert_panic!(
            (&Natural::from_str("1000000000001").unwrap()).shr_round($t::exact_from(100), Exact)
        );
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
            Natural::from_owned_limbs_asc(xs).shr_round(bits, Up),
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
            Natural::from_owned_limbs_asc(xs).shr_round(bits, Nearest),
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
                let (m, o) = (&n).shr_round(bits, Exact);
                assert_eq!(Natural::from_owned_limbs_asc(result_xs), m);
                assert_eq!(m << bits, n);
                assert_eq!(o, Equal);
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
                    assert_eq!(rm, Exact);
                    assert!(!n.divisible_by_power_of_2(bits));
                },
                |(result_xs, o)| {
                    let (m, o_alt) = (&n).shr_round(bits, rm);
                    assert_eq!(Natural::from_owned_limbs_asc(result_xs), m);
                    assert_eq!(o, o_alt);
                    if rm == Exact {
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
            let (n, o_alt) = Natural::from_owned_limbs_asc(old_xs).shr_round(bits, Up);
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
            let (n, o_alt) = Natural::from_owned_limbs_asc(old_xs).shr_round(bits, Nearest);
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
                let (m, o) = (&n).shr_round(bits, Exact);
                assert_eq!(Natural::from_owned_limbs_asc(xs), m);
                assert_eq!(m << bits, n);
                assert_eq!(o, Equal);
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
                if rm == Exact {
                    assert_eq!(m << bits, n);
                }
            } else {
                assert_eq!(rm, Exact);
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
        assert_eq!(n.divisible_by_power_of_2(u.exact_into()), o == Equal);

        assert_eq!((shifted << u).cmp(&n), o);
        match rm {
            Floor | Down => assert_ne!(o, Greater),
            Ceiling | Up => assert_ne!(o, Less),
            Exact => assert_eq!(o, Equal),
            _ => {}
        }
    });

    natural_unsigned_pair_gen_var_4::<T>().test_properties(|(n, u)| {
        let left_shifted = &n << u;
        let no = (n, Equal);
        assert_eq!((&left_shifted).shr_round(u, Down), no);
        assert_eq!((&left_shifted).shr_round(u, Up), no);
        assert_eq!((&left_shifted).shr_round(u, Floor), no);
        assert_eq!((&left_shifted).shr_round(u, Ceiling), no);
        assert_eq!((&left_shifted).shr_round(u, Nearest), no);
        assert_eq!((&left_shifted).shr_round(u, Exact), no);
    });

    natural_unsigned_pair_gen_var_10::<T>().test_properties(|(n, u)| {
        let down = (&n).shr_round(u, Down);
        let up = (&down.0 + Natural::ONE, Greater);
        assert_eq!((&n).shr_round(u, Up), up);
        assert_eq!((&n).shr_round(u, Floor), down);
        assert_eq!((&n).shr_round(u, Ceiling), up);
        let nearest = n.shr_round(u, Nearest);
        assert!(nearest == down || nearest == up);
    });

    unsigned_pair_gen_var_37::<Limb, T>().test_properties(|(u, v)| {
        if let Some(shift) = v.checked_add(T::exact_from(Limb::WIDTH)) {
            assert_eq!(
                Natural::from(u).shr_round(shift, Down),
                (Natural::ZERO, Less)
            );
            assert_eq!(
                Natural::from(u).shr_round(shift, Floor),
                (Natural::ZERO, Less)
            );
            assert_eq!(
                Natural::from(u).shr_round(shift, Up),
                (Natural::ONE, Greater)
            );
            assert_eq!(
                Natural::from(u).shr_round(shift, Ceiling),
                (Natural::ONE, Greater)
            );
            if let Some(extra_shift) = shift.checked_add(T::ONE) {
                assert_eq!(
                    Natural::from(u).shr_round(extra_shift, Nearest),
                    (Natural::ZERO, Less)
                );
            }
        }
    });

    natural_rounding_mode_pair_gen().test_properties(|(n, rm)| {
        assert_eq!((&n).shr_round(T::ZERO, rm), (n, Equal));
    });

    unsigned_rounding_mode_pair_gen::<T>().test_properties(|(u, rm)| {
        assert_eq!(Natural::ZERO.shr_round(u, rm), (Natural::ZERO, Equal));
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
            o == Equal
        );

        if i >= T::ZERO {
            assert_eq!((shifted << i).cmp(&n), o);
        }
        match rm {
            Floor | Down => assert_ne!(o, Greater),
            Ceiling | Up => assert_ne!(o, Less),
            Exact => assert_eq!(o, Equal),
            _ => {}
        }
    });

    natural_rounding_mode_pair_gen().test_properties(|(n, rm)| {
        assert_eq!((&n).shr_round(T::ZERO, rm), (n, Equal));
    });

    signed_rounding_mode_pair_gen::<T>().test_properties(|(i, rm)| {
        assert_eq!(Natural::ZERO.shr_round(i, rm), (Natural::ZERO, Equal));
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
