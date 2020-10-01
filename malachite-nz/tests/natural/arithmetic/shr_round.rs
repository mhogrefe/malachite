use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::shr_round::{
    limbs_shr_exact, limbs_shr_round, limbs_shr_round_nearest, limbs_shr_round_up,
    limbs_vec_shr_exact_in_place, limbs_vec_shr_round_in_place,
    limbs_vec_shr_round_nearest_in_place, limbs_vec_shr_round_up_in_place,
};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_shr_round_up_and_limbs_vec_shr_round_up_in_place() {
    let test = |limbs: &[Limb], bits: u64, out: &[Limb]| {
        assert_eq!(limbs_shr_round_up(limbs, bits), out);

        let mut limbs = limbs.to_vec();
        limbs_vec_shr_round_up_in_place(&mut limbs, bits);
        assert_eq!(limbs, out);
    };
    test(&[1], 0, &[1]);
    test(&[1], 1, &[1]);
    test(&[3], 1, &[2]);
    test(&[122, 456], 1, &[61, 228]);
    test(&[123, 456], 0, &[123, 456]);
    test(&[123, 456], 1, &[62, 228]);
    test(&[123, 455], 1, &[2147483710, 227]);
    test(&[123, 456], 31, &[913, 0]);
    test(&[123, 456], 32, &[457]);
    test(&[123, 456], 100, &[1]);
    test(&[256, 456], 8, &[3355443201, 1]);
    test(&[u32::MAX, 1], 1, &[0, 1]);
    test(&[u32::MAX, u32::MAX], 32, &[0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_shr_round_nearest_and_limbs_vec_shr_round_nearest_in_place() {
    let test = |limbs: &[Limb], bits: u64, out: &[Limb]| {
        assert_eq!(limbs_shr_round_nearest(limbs, bits), out);

        let mut limbs = limbs.to_vec();
        limbs_vec_shr_round_nearest_in_place(&mut limbs, bits);
        assert_eq!(limbs, out);
    };
    test(&[], 0, &[]);
    test(&[], 1, &[]);
    test(&[], 100, &[]);
    test(&[0, 0, 0], 0, &[0, 0, 0]);
    test(&[0, 0, 0], 1, &[0, 0, 0]);
    test(&[0, 0, 0], 100, &[]);
    test(&[1], 0, &[1]);
    test(&[1], 1, &[0]);
    test(&[3], 1, &[2]);
    test(&[122, 456], 1, &[61, 228]);
    test(&[123, 456], 0, &[123, 456]);
    test(&[123, 456], 1, &[62, 228]);
    test(&[123, 455], 1, &[2147483710, 227]);
    test(&[123, 456], 31, &[912, 0]);
    test(&[123, 456], 32, &[456]);
    test(&[123, 456], 100, &[]);
    test(&[256, 456], 8, &[3355443201, 1]);
    test(&[u32::MAX, 1], 1, &[0, 1]);
    test(&[u32::MAX, u32::MAX], 32, &[0, 1]);
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
    let test = |limbs: &[Limb], bits: u64, rm: RoundingMode, out: Option<Vec<Limb>>| {
        assert_eq!(limbs_shr_round(limbs, bits, rm), out);

        let mut limbs = limbs.to_vec();
        if limbs_vec_shr_round_in_place(&mut limbs, bits, rm) {
            assert_eq!(Some(limbs), out);
        } else {
            assert_eq!(None, out)
        }
    };
    test(&[1], 0, RoundingMode::Nearest, Some(vec![1]));
    test(&[1], 1, RoundingMode::Up, Some(vec![1]));
    test(&[3], 1, RoundingMode::Nearest, Some(vec![2]));
    test(&[122, 456], 1, RoundingMode::Floor, Some(vec![61, 228]));
    test(&[123, 456], 0, RoundingMode::Floor, Some(vec![123, 456]));
    test(&[123, 456], 1, RoundingMode::Down, Some(vec![61, 228]));
    test(
        &[123, 455],
        1,
        RoundingMode::Floor,
        Some(vec![2147483709, 227]),
    );
    test(&[123, 456], 31, RoundingMode::Ceiling, Some(vec![913, 0]));
    test(&[123, 456], 32, RoundingMode::Up, Some(vec![457]));
    test(&[123, 456], 100, RoundingMode::Down, Some(vec![]));
    test(
        &[256, 456],
        8,
        RoundingMode::Exact,
        Some(vec![3355443201, 1]),
    );
    test(&[u32::MAX, 1], 1, RoundingMode::Exact, None);
    test(
        &[u32::MAX, u32::MAX],
        32,
        RoundingMode::Down,
        Some(vec![u32::MAX]),
    );
}

macro_rules! tests_and_properties_unsigned {
    (
        $t:ident,
        $test_shr_round_u:ident,
        $shr_round_assign_u_fail_1:ident,
        $shr_round_assign_u_fail_2:ident,
        $shr_round_assign_u_fail_3:ident,
        $shr_round_assign_u_fail_4:ident,
        $shr_round_u_fail_1:ident,
        $shr_round_u_fail_2:ident,
        $shr_round_u_fail_3:ident,
        $shr_round_u_fail_4:ident,
        $shr_round_u_ref_fail_1:ident,
        $shr_round_u_ref_fail_2:ident,
        $shr_round_u_ref_fail_3:ident,
        $shr_round_u_ref_fail_4:ident
    ) => {
        #[test]
        fn $test_shr_round_u() {
            let test = |u, v: $t, rm: RoundingMode, out| {
                let mut n = Natural::from_str(u).unwrap();
                n.shr_round_assign(v, rm);
                assert_eq!(n.to_string(), out);
                assert!(n.is_valid());

                let n = Natural::from_str(u).unwrap().shr_round(v, rm);
                assert_eq!(n.to_string(), out);
                assert!(n.is_valid());

                let n = &Natural::from_str(u).unwrap().shr_round(v, rm);
                assert_eq!(n.to_string(), out);
                assert!(n.is_valid());
            };
            test("0", 0, RoundingMode::Down, "0");
            test("0", 0, RoundingMode::Up, "0");
            test("0", 0, RoundingMode::Floor, "0");
            test("0", 0, RoundingMode::Ceiling, "0");
            test("0", 0, RoundingMode::Nearest, "0");
            test("0", 0, RoundingMode::Exact, "0");

            test("0", 10, RoundingMode::Down, "0");
            test("0", 10, RoundingMode::Up, "0");
            test("0", 10, RoundingMode::Floor, "0");
            test("0", 10, RoundingMode::Ceiling, "0");
            test("0", 10, RoundingMode::Nearest, "0");
            test("0", 10, RoundingMode::Exact, "0");

            test("123", 0, RoundingMode::Down, "123");
            test("123", 0, RoundingMode::Up, "123");
            test("123", 0, RoundingMode::Floor, "123");
            test("123", 0, RoundingMode::Ceiling, "123");
            test("123", 0, RoundingMode::Nearest, "123");
            test("123", 0, RoundingMode::Exact, "123");

            test("245", 1, RoundingMode::Down, "122");
            test("245", 1, RoundingMode::Up, "123");
            test("245", 1, RoundingMode::Floor, "122");
            test("245", 1, RoundingMode::Ceiling, "123");
            test("245", 1, RoundingMode::Nearest, "122");

            test("246", 1, RoundingMode::Down, "123");
            test("246", 1, RoundingMode::Up, "123");
            test("246", 1, RoundingMode::Floor, "123");
            test("246", 1, RoundingMode::Ceiling, "123");
            test("246", 1, RoundingMode::Nearest, "123");
            test("246", 1, RoundingMode::Exact, "123");

            test("247", 1, RoundingMode::Down, "123");
            test("247", 1, RoundingMode::Up, "124");
            test("247", 1, RoundingMode::Floor, "123");
            test("247", 1, RoundingMode::Ceiling, "124");
            test("247", 1, RoundingMode::Nearest, "124");

            test("491", 2, RoundingMode::Down, "122");
            test("491", 2, RoundingMode::Up, "123");
            test("491", 2, RoundingMode::Floor, "122");
            test("491", 2, RoundingMode::Ceiling, "123");
            test("491", 2, RoundingMode::Nearest, "123");

            test("492", 2, RoundingMode::Down, "123");
            test("492", 2, RoundingMode::Up, "123");
            test("492", 2, RoundingMode::Floor, "123");
            test("492", 2, RoundingMode::Ceiling, "123");
            test("492", 2, RoundingMode::Nearest, "123");
            test("492", 2, RoundingMode::Exact, "123");

            test("493", 2, RoundingMode::Down, "123");
            test("493", 2, RoundingMode::Up, "124");
            test("493", 2, RoundingMode::Floor, "123");
            test("493", 2, RoundingMode::Ceiling, "124");
            test("493", 2, RoundingMode::Nearest, "123");

            test("4127195135", 25, RoundingMode::Down, "122");
            test("4127195135", 25, RoundingMode::Up, "123");
            test("4127195135", 25, RoundingMode::Floor, "122");
            test("4127195135", 25, RoundingMode::Ceiling, "123");
            test("4127195135", 25, RoundingMode::Nearest, "123");

            test("4127195136", 25, RoundingMode::Down, "123");
            test("4127195136", 25, RoundingMode::Up, "123");
            test("4127195136", 25, RoundingMode::Floor, "123");
            test("4127195136", 25, RoundingMode::Ceiling, "123");
            test("4127195136", 25, RoundingMode::Nearest, "123");
            test("4127195136", 25, RoundingMode::Exact, "123");

            test("4127195137", 25, RoundingMode::Down, "123");
            test("4127195137", 25, RoundingMode::Up, "124");
            test("4127195137", 25, RoundingMode::Floor, "123");
            test("4127195137", 25, RoundingMode::Ceiling, "124");
            test("4127195137", 25, RoundingMode::Nearest, "123");

            test("8254390271", 26, RoundingMode::Down, "122");
            test("8254390271", 26, RoundingMode::Up, "123");
            test("8254390271", 26, RoundingMode::Floor, "122");
            test("8254390271", 26, RoundingMode::Ceiling, "123");
            test("8254390271", 26, RoundingMode::Nearest, "123");

            test("8254390272", 26, RoundingMode::Down, "123");
            test("8254390272", 26, RoundingMode::Up, "123");
            test("8254390272", 26, RoundingMode::Floor, "123");
            test("8254390272", 26, RoundingMode::Ceiling, "123");
            test("8254390272", 26, RoundingMode::Nearest, "123");
            test("8254390272", 26, RoundingMode::Exact, "123");

            test("8254390273", 26, RoundingMode::Down, "123");
            test("8254390273", 26, RoundingMode::Up, "124");
            test("8254390273", 26, RoundingMode::Floor, "123");
            test("8254390273", 26, RoundingMode::Ceiling, "124");
            test("8254390273", 26, RoundingMode::Nearest, "123");

            test(
                "155921023828072216384094494261247",
                100,
                RoundingMode::Down,
                "122",
            );
            test(
                "155921023828072216384094494261247",
                100,
                RoundingMode::Up,
                "123",
            );
            test(
                "155921023828072216384094494261247",
                100,
                RoundingMode::Floor,
                "122",
            );
            test(
                "155921023828072216384094494261247",
                100,
                RoundingMode::Ceiling,
                "123",
            );
            test(
                "155921023828072216384094494261247",
                100,
                RoundingMode::Nearest,
                "123",
            );

            test(
                "155921023828072216384094494261248",
                100,
                RoundingMode::Down,
                "123",
            );
            test(
                "155921023828072216384094494261248",
                100,
                RoundingMode::Up,
                "123",
            );
            test(
                "155921023828072216384094494261248",
                100,
                RoundingMode::Floor,
                "123",
            );
            test(
                "155921023828072216384094494261248",
                100,
                RoundingMode::Ceiling,
                "123",
            );
            test(
                "155921023828072216384094494261248",
                100,
                RoundingMode::Nearest,
                "123",
            );
            test(
                "155921023828072216384094494261248",
                100,
                RoundingMode::Exact,
                "123",
            );

            test(
                "155921023828072216384094494261249",
                100,
                RoundingMode::Down,
                "123",
            );
            test(
                "155921023828072216384094494261249",
                100,
                RoundingMode::Up,
                "124",
            );
            test(
                "155921023828072216384094494261249",
                100,
                RoundingMode::Floor,
                "123",
            );
            test(
                "155921023828072216384094494261249",
                100,
                RoundingMode::Ceiling,
                "124",
            );
            test(
                "155921023828072216384094494261249",
                100,
                RoundingMode::Nearest,
                "123",
            );

            test("4294967295", 1, RoundingMode::Down, "2147483647");
            test("4294967295", 1, RoundingMode::Up, "2147483648");
            test("4294967295", 1, RoundingMode::Floor, "2147483647");
            test("4294967295", 1, RoundingMode::Ceiling, "2147483648");
            test("4294967295", 1, RoundingMode::Nearest, "2147483648");

            test("4294967296", 1, RoundingMode::Down, "2147483648");
            test("4294967296", 1, RoundingMode::Up, "2147483648");
            test("4294967296", 1, RoundingMode::Floor, "2147483648");
            test("4294967296", 1, RoundingMode::Ceiling, "2147483648");
            test("4294967296", 1, RoundingMode::Nearest, "2147483648");
            test("4294967296", 1, RoundingMode::Exact, "2147483648");

            test("4294967297", 1, RoundingMode::Down, "2147483648");
            test("4294967297", 1, RoundingMode::Up, "2147483649");
            test("4294967297", 1, RoundingMode::Floor, "2147483648");
            test("4294967297", 1, RoundingMode::Ceiling, "2147483649");
            test("4294967297", 1, RoundingMode::Nearest, "2147483648");

            test("1000000000000", 0, RoundingMode::Down, "1000000000000");
            test("1000000000000", 0, RoundingMode::Up, "1000000000000");
            test("1000000000000", 0, RoundingMode::Floor, "1000000000000");
            test("1000000000000", 0, RoundingMode::Ceiling, "1000000000000");
            test("1000000000000", 0, RoundingMode::Nearest, "1000000000000");
            test("1000000000000", 0, RoundingMode::Exact, "1000000000000");

            test("7999999999999", 3, RoundingMode::Down, "999999999999");
            test("7999999999999", 3, RoundingMode::Up, "1000000000000");
            test("7999999999999", 3, RoundingMode::Floor, "999999999999");
            test("7999999999999", 3, RoundingMode::Ceiling, "1000000000000");
            test("7999999999999", 3, RoundingMode::Nearest, "1000000000000");

            test("8000000000000", 3, RoundingMode::Down, "1000000000000");
            test("8000000000000", 3, RoundingMode::Up, "1000000000000");
            test("8000000000000", 3, RoundingMode::Floor, "1000000000000");
            test("8000000000000", 3, RoundingMode::Ceiling, "1000000000000");
            test("8000000000000", 3, RoundingMode::Nearest, "1000000000000");
            test("8000000000000", 3, RoundingMode::Exact, "1000000000000");

            test("8000000000001", 3, RoundingMode::Down, "1000000000000");
            test("8000000000001", 3, RoundingMode::Up, "1000000000001");
            test("8000000000001", 3, RoundingMode::Floor, "1000000000000");
            test("8000000000001", 3, RoundingMode::Ceiling, "1000000000001");
            test("8000000000001", 3, RoundingMode::Nearest, "1000000000000");

            test(
                "16777216000000000000",
                24,
                RoundingMode::Down,
                "1000000000000",
            );
            test(
                "16777216000000000000",
                24,
                RoundingMode::Up,
                "1000000000000",
            );
            test(
                "16777216000000000000",
                24,
                RoundingMode::Floor,
                "1000000000000",
            );
            test(
                "16777216000000000000",
                24,
                RoundingMode::Ceiling,
                "1000000000000",
            );
            test(
                "16777216000000000000",
                24,
                RoundingMode::Nearest,
                "1000000000000",
            );
            test(
                "16777216000000000000",
                24,
                RoundingMode::Exact,
                "1000000000000",
            );

            test(
                "33554432000000000000",
                25,
                RoundingMode::Down,
                "1000000000000",
            );
            test(
                "33554432000000000000",
                25,
                RoundingMode::Up,
                "1000000000000",
            );
            test(
                "33554432000000000000",
                25,
                RoundingMode::Floor,
                "1000000000000",
            );
            test(
                "33554432000000000000",
                25,
                RoundingMode::Ceiling,
                "1000000000000",
            );
            test(
                "33554432000000000000",
                25,
                RoundingMode::Nearest,
                "1000000000000",
            );
            test(
                "33554432000000000000",
                25,
                RoundingMode::Exact,
                "1000000000000",
            );

            test(
                "2147483648000000000000",
                31,
                RoundingMode::Down,
                "1000000000000",
            );
            test(
                "2147483648000000000000",
                31,
                RoundingMode::Up,
                "1000000000000",
            );
            test(
                "2147483648000000000000",
                31,
                RoundingMode::Floor,
                "1000000000000",
            );
            test(
                "2147483648000000000000",
                31,
                RoundingMode::Ceiling,
                "1000000000000",
            );
            test(
                "2147483648000000000000",
                31,
                RoundingMode::Nearest,
                "1000000000000",
            );
            test(
                "2147483648000000000000",
                31,
                RoundingMode::Exact,
                "1000000000000",
            );

            test(
                "4294967296000000000000",
                32,
                RoundingMode::Down,
                "1000000000000",
            );
            test(
                "4294967296000000000000",
                32,
                RoundingMode::Up,
                "1000000000000",
            );
            test(
                "4294967296000000000000",
                32,
                RoundingMode::Floor,
                "1000000000000",
            );
            test(
                "4294967296000000000000",
                32,
                RoundingMode::Ceiling,
                "1000000000000",
            );
            test(
                "4294967296000000000000",
                32,
                RoundingMode::Nearest,
                "1000000000000",
            );
            test(
                "4294967296000000000000",
                32,
                RoundingMode::Exact,
                "1000000000000",
            );

            test(
                "8589934592000000000000",
                33,
                RoundingMode::Down,
                "1000000000000",
            );
            test(
                "8589934592000000000000",
                33,
                RoundingMode::Up,
                "1000000000000",
            );
            test(
                "8589934592000000000000",
                33,
                RoundingMode::Floor,
                "1000000000000",
            );
            test(
                "8589934592000000000000",
                33,
                RoundingMode::Ceiling,
                "1000000000000",
            );
            test(
                "8589934592000000000000",
                33,
                RoundingMode::Nearest,
                "1000000000000",
            );
            test(
                "8589934592000000000000",
                33,
                RoundingMode::Exact,
                "1000000000000",
            );

            test(
                "1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Down,
                "1000000000000",
            );
            test(
                "1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Up,
                "1000000000000",
            );
            test(
                "1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Floor,
                "1000000000000",
            );
            test(
                "1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Ceiling,
                "1000000000000",
            );
            test(
                "1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Nearest,
                "1000000000000",
            );
            test(
                "1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Exact,
                "1000000000000",
            );

            test("1000000000000", 10, RoundingMode::Down, "976562500");
            test("1000000000000", 10, RoundingMode::Up, "976562500");
            test("1000000000000", 10, RoundingMode::Floor, "976562500");
            test("1000000000000", 10, RoundingMode::Ceiling, "976562500");
            test("1000000000000", 10, RoundingMode::Nearest, "976562500");
            test("1000000000000", 10, RoundingMode::Exact, "976562500");

            test("980657949", 72, RoundingMode::Down, "0");
            test("980657949", 72, RoundingMode::Up, "1");
            test("980657949", 72, RoundingMode::Floor, "0");
            test("980657949", 72, RoundingMode::Ceiling, "1");
            test("980657949", 72, RoundingMode::Nearest, "0");

            test("4294967295", 31, RoundingMode::Down, "1");
            test("4294967295", 31, RoundingMode::Up, "2");
            test("4294967295", 31, RoundingMode::Floor, "1");
            test("4294967295", 31, RoundingMode::Ceiling, "2");
            test("4294967295", 31, RoundingMode::Nearest, "2");

            test("4294967295", 32, RoundingMode::Down, "0");
            test("4294967295", 32, RoundingMode::Up, "1");
            test("4294967295", 32, RoundingMode::Floor, "0");
            test("4294967295", 32, RoundingMode::Ceiling, "1");
            test("4294967295", 32, RoundingMode::Nearest, "1");

            test("4294967296", 32, RoundingMode::Down, "1");
            test("4294967296", 32, RoundingMode::Up, "1");
            test("4294967296", 32, RoundingMode::Floor, "1");
            test("4294967296", 32, RoundingMode::Ceiling, "1");
            test("4294967296", 32, RoundingMode::Nearest, "1");
            test("4294967296", 32, RoundingMode::Exact, "1");

            test("4294967296", 33, RoundingMode::Down, "0");
            test("4294967296", 33, RoundingMode::Up, "1");
            test("4294967296", 33, RoundingMode::Floor, "0");
            test("4294967296", 33, RoundingMode::Ceiling, "1");
            test("4294967296", 33, RoundingMode::Nearest, "0");
        }

        #[test]
        #[should_panic]
        fn $shr_round_assign_u_fail_1() {
            Natural::from(123u32).shr_round_assign($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_assign_u_fail_2() {
            Natural::from(123u32).shr_round_assign($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_assign_u_fail_3() {
            Natural::from_str("1000000000001")
                .unwrap()
                .shr_round_assign($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_assign_u_fail_4() {
            Natural::from_str("1000000000001")
                .unwrap()
                .shr_round_assign($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_u_fail_1() {
            Natural::from(123u32).shr_round($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_u_fail_2() {
            Natural::from(123u32).shr_round($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_u_fail_3() {
            Natural::from_str("1000000000001")
                .unwrap()
                .shr_round($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_u_fail_4() {
            Natural::from_str("1000000000001")
                .unwrap()
                .shr_round($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_u_ref_fail_1() {
            (&Natural::from(123u32)).shr_round($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_u_ref_fail_2() {
            (&Natural::from(123u32)).shr_round($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_u_ref_fail_3() {
            (&Natural::from_str("1000000000001").unwrap()).shr_round($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_u_ref_fail_4() {
            (&Natural::from_str("1000000000001").unwrap())
                .shr_round($t::exact_from(100), RoundingMode::Exact);
        }
    };
}
tests_and_properties_unsigned!(
    u8,
    test_shr_round_u8,
    shr_round_assign_u8_fail_1,
    shr_round_assign_u8_fail_2,
    shr_round_assign_u8_fail_3,
    shr_round_assign_u8_fail_4,
    shr_round_u8_fail_1,
    shr_round_u8_fail_2,
    shr_round_u8_fail_3,
    shr_round_u8_fail_4,
    shr_round_u8_ref_fail_1,
    shr_round_u8_ref_fail_2,
    shr_round_u8_ref_fail_3,
    shr_round_u8_ref_fail_4
);
tests_and_properties_unsigned!(
    u16,
    test_shr_round_u16,
    shr_round_assign_u16_fail_1,
    shr_round_assign_u16_fail_2,
    shr_round_assign_u16_fail_3,
    shr_round_assign_u16_fail_4,
    shr_round_u16_fail_1,
    shr_round_u16_fail_2,
    shr_round_u16_fail_3,
    shr_round_u16_fail_4,
    shr_round_u16_ref_fail_1,
    shr_round_u16_ref_fail_2,
    shr_round_u16_ref_fail_3,
    shr_round_u16_ref_fail_4
);
tests_and_properties_unsigned!(
    u32,
    test_shr_round_u32,
    shr_round_assign_u32_fail_1,
    shr_round_assign_u32_fail_2,
    shr_round_assign_u32_fail_3,
    shr_round_assign_u32_fail_4,
    shr_round_u32_fail_1,
    shr_round_u32_fail_2,
    shr_round_u32_fail_3,
    shr_round_u32_fail_4,
    shr_round_u32_ref_fail_1,
    shr_round_u32_ref_fail_2,
    shr_round_u32_ref_fail_3,
    shr_round_u32_ref_fail_4
);
tests_and_properties_unsigned!(
    u64,
    test_shr_round_u64,
    shr_round_assign_u64_fail_1,
    shr_round_assign_u64_fail_2,
    shr_round_assign_u64_fail_3,
    shr_round_assign_u64_fail_4,
    shr_round_u64_fail_1,
    shr_round_u64_fail_2,
    shr_round_u64_fail_3,
    shr_round_u64_fail_4,
    shr_round_u64_ref_fail_1,
    shr_round_u64_ref_fail_2,
    shr_round_u64_ref_fail_3,
    shr_round_u64_ref_fail_4
);
tests_and_properties_unsigned!(
    u128,
    test_shr_round_u128,
    shr_round_assign_u128_fail_1,
    shr_round_assign_u128_fail_2,
    shr_round_assign_u128_fail_3,
    shr_round_assign_u128_fail_4,
    shr_round_u128_fail_1,
    shr_round_u128_fail_2,
    shr_round_u128_fail_3,
    shr_round_u128_fail_4,
    shr_round_u128_ref_fail_1,
    shr_round_u128_ref_fail_2,
    shr_round_u128_ref_fail_3,
    shr_round_u128_ref_fail_4
);
tests_and_properties_unsigned!(
    usize,
    test_shr_round_usize,
    shr_round_assign_usize_fail_1,
    shr_round_assign_usize_fail_2,
    shr_round_assign_usize_fail_3,
    shr_round_assign_usize_fail_4,
    shr_round_usize_fail_1,
    shr_round_usize_fail_2,
    shr_round_usize_fail_3,
    shr_round_usize_fail_4,
    shr_round_usize_ref_fail_1,
    shr_round_usize_ref_fail_2,
    shr_round_usize_ref_fail_3,
    shr_round_usize_ref_fail_4
);

macro_rules! tests_and_properties_signed {
    (
        $t:ident,
        $test_shr_round_i:ident,
        $shr_round_assign_i_fail_1:ident,
        $shr_round_assign_i_fail_2:ident,
        $shr_round_assign_i_fail_3:ident,
        $shr_round_assign_i_fail_4:ident,
        $shr_round_i_fail_1:ident,
        $shr_round_i_fail_2:ident,
        $shr_round_i_fail_3:ident,
        $shr_round_i_fail_4:ident,
        $shr_round_i_ref_fail_1:ident,
        $shr_round_i_ref_fail_2:ident,
        $shr_round_i_ref_fail_3:ident,
        $shr_round_i_ref_fail_4:ident
    ) => {
        #[test]
        fn $test_shr_round_i() {
            let test = |i, j: $t, rm: RoundingMode, out| {
                let mut n = Natural::from_str(i).unwrap();
                n.shr_round_assign(j, rm);
                assert_eq!(n.to_string(), out);
                assert!(n.is_valid());

                let n = Natural::from_str(i).unwrap().shr_round(j, rm);
                assert_eq!(n.to_string(), out);
                assert!(n.is_valid());

                let n = &Natural::from_str(i).unwrap().shr_round(j, rm);
                assert_eq!(n.to_string(), out);
                assert!(n.is_valid());
            };
            test("0", 0, RoundingMode::Down, "0");
            test("0", 0, RoundingMode::Up, "0");
            test("0", 0, RoundingMode::Floor, "0");
            test("0", 0, RoundingMode::Ceiling, "0");
            test("0", 0, RoundingMode::Nearest, "0");
            test("0", 0, RoundingMode::Exact, "0");

            test("0", 10, RoundingMode::Down, "0");
            test("0", 10, RoundingMode::Up, "0");
            test("0", 10, RoundingMode::Floor, "0");
            test("0", 10, RoundingMode::Ceiling, "0");
            test("0", 10, RoundingMode::Nearest, "0");
            test("0", 10, RoundingMode::Exact, "0");

            test("123", 0, RoundingMode::Down, "123");
            test("123", 0, RoundingMode::Up, "123");
            test("123", 0, RoundingMode::Floor, "123");
            test("123", 0, RoundingMode::Ceiling, "123");
            test("123", 0, RoundingMode::Nearest, "123");
            test("123", 0, RoundingMode::Exact, "123");

            test("245", 1, RoundingMode::Down, "122");
            test("245", 1, RoundingMode::Up, "123");
            test("245", 1, RoundingMode::Floor, "122");
            test("245", 1, RoundingMode::Ceiling, "123");
            test("245", 1, RoundingMode::Nearest, "122");

            test("246", 1, RoundingMode::Down, "123");
            test("246", 1, RoundingMode::Up, "123");
            test("246", 1, RoundingMode::Floor, "123");
            test("246", 1, RoundingMode::Ceiling, "123");
            test("246", 1, RoundingMode::Nearest, "123");
            test("246", 1, RoundingMode::Exact, "123");

            test("247", 1, RoundingMode::Down, "123");
            test("247", 1, RoundingMode::Up, "124");
            test("247", 1, RoundingMode::Floor, "123");
            test("247", 1, RoundingMode::Ceiling, "124");
            test("247", 1, RoundingMode::Nearest, "124");

            test("491", 2, RoundingMode::Down, "122");
            test("491", 2, RoundingMode::Up, "123");
            test("491", 2, RoundingMode::Floor, "122");
            test("491", 2, RoundingMode::Ceiling, "123");
            test("491", 2, RoundingMode::Nearest, "123");

            test("492", 2, RoundingMode::Down, "123");
            test("492", 2, RoundingMode::Up, "123");
            test("492", 2, RoundingMode::Floor, "123");
            test("492", 2, RoundingMode::Ceiling, "123");
            test("492", 2, RoundingMode::Nearest, "123");
            test("492", 2, RoundingMode::Exact, "123");

            test("493", 2, RoundingMode::Down, "123");
            test("493", 2, RoundingMode::Up, "124");
            test("493", 2, RoundingMode::Floor, "123");
            test("493", 2, RoundingMode::Ceiling, "124");
            test("493", 2, RoundingMode::Nearest, "123");

            test("4127195135", 25, RoundingMode::Down, "122");
            test("4127195135", 25, RoundingMode::Up, "123");
            test("4127195135", 25, RoundingMode::Floor, "122");
            test("4127195135", 25, RoundingMode::Ceiling, "123");
            test("4127195135", 25, RoundingMode::Nearest, "123");

            test("4127195136", 25, RoundingMode::Down, "123");
            test("4127195136", 25, RoundingMode::Up, "123");
            test("4127195136", 25, RoundingMode::Floor, "123");
            test("4127195136", 25, RoundingMode::Ceiling, "123");
            test("4127195136", 25, RoundingMode::Nearest, "123");
            test("4127195136", 25, RoundingMode::Exact, "123");

            test("4127195137", 25, RoundingMode::Down, "123");
            test("4127195137", 25, RoundingMode::Up, "124");
            test("4127195137", 25, RoundingMode::Floor, "123");
            test("4127195137", 25, RoundingMode::Ceiling, "124");
            test("4127195137", 25, RoundingMode::Nearest, "123");

            test("8254390271", 26, RoundingMode::Down, "122");
            test("8254390271", 26, RoundingMode::Up, "123");
            test("8254390271", 26, RoundingMode::Floor, "122");
            test("8254390271", 26, RoundingMode::Ceiling, "123");
            test("8254390271", 26, RoundingMode::Nearest, "123");

            test("8254390272", 26, RoundingMode::Down, "123");
            test("8254390272", 26, RoundingMode::Up, "123");
            test("8254390272", 26, RoundingMode::Floor, "123");
            test("8254390272", 26, RoundingMode::Ceiling, "123");
            test("8254390272", 26, RoundingMode::Nearest, "123");
            test("8254390272", 26, RoundingMode::Exact, "123");

            test("8254390273", 26, RoundingMode::Down, "123");
            test("8254390273", 26, RoundingMode::Up, "124");
            test("8254390273", 26, RoundingMode::Floor, "123");
            test("8254390273", 26, RoundingMode::Ceiling, "124");
            test("8254390273", 26, RoundingMode::Nearest, "123");

            test(
                "155921023828072216384094494261247",
                100,
                RoundingMode::Down,
                "122",
            );
            test(
                "155921023828072216384094494261247",
                100,
                RoundingMode::Up,
                "123",
            );
            test(
                "155921023828072216384094494261247",
                100,
                RoundingMode::Floor,
                "122",
            );
            test(
                "155921023828072216384094494261247",
                100,
                RoundingMode::Ceiling,
                "123",
            );
            test(
                "155921023828072216384094494261247",
                100,
                RoundingMode::Nearest,
                "123",
            );

            test(
                "155921023828072216384094494261248",
                100,
                RoundingMode::Down,
                "123",
            );
            test(
                "155921023828072216384094494261248",
                100,
                RoundingMode::Up,
                "123",
            );
            test(
                "155921023828072216384094494261248",
                100,
                RoundingMode::Floor,
                "123",
            );
            test(
                "155921023828072216384094494261248",
                100,
                RoundingMode::Ceiling,
                "123",
            );
            test(
                "155921023828072216384094494261248",
                100,
                RoundingMode::Nearest,
                "123",
            );
            test(
                "155921023828072216384094494261248",
                100,
                RoundingMode::Exact,
                "123",
            );

            test(
                "155921023828072216384094494261249",
                100,
                RoundingMode::Down,
                "123",
            );
            test(
                "155921023828072216384094494261249",
                100,
                RoundingMode::Up,
                "124",
            );
            test(
                "155921023828072216384094494261249",
                100,
                RoundingMode::Floor,
                "123",
            );
            test(
                "155921023828072216384094494261249",
                100,
                RoundingMode::Ceiling,
                "124",
            );
            test(
                "155921023828072216384094494261249",
                100,
                RoundingMode::Nearest,
                "123",
            );

            test("4294967295", 1, RoundingMode::Down, "2147483647");
            test("4294967295", 1, RoundingMode::Up, "2147483648");
            test("4294967295", 1, RoundingMode::Floor, "2147483647");
            test("4294967295", 1, RoundingMode::Ceiling, "2147483648");
            test("4294967295", 1, RoundingMode::Nearest, "2147483648");

            test("4294967296", 1, RoundingMode::Down, "2147483648");
            test("4294967296", 1, RoundingMode::Up, "2147483648");
            test("4294967296", 1, RoundingMode::Floor, "2147483648");
            test("4294967296", 1, RoundingMode::Ceiling, "2147483648");
            test("4294967296", 1, RoundingMode::Nearest, "2147483648");
            test("4294967296", 1, RoundingMode::Exact, "2147483648");

            test("4294967297", 1, RoundingMode::Down, "2147483648");
            test("4294967297", 1, RoundingMode::Up, "2147483649");
            test("4294967297", 1, RoundingMode::Floor, "2147483648");
            test("4294967297", 1, RoundingMode::Ceiling, "2147483649");
            test("4294967297", 1, RoundingMode::Nearest, "2147483648");

            test("1000000000000", 0, RoundingMode::Down, "1000000000000");
            test("1000000000000", 0, RoundingMode::Up, "1000000000000");
            test("1000000000000", 0, RoundingMode::Floor, "1000000000000");
            test("1000000000000", 0, RoundingMode::Ceiling, "1000000000000");
            test("1000000000000", 0, RoundingMode::Nearest, "1000000000000");
            test("1000000000000", 0, RoundingMode::Exact, "1000000000000");

            test("7999999999999", 3, RoundingMode::Down, "999999999999");
            test("7999999999999", 3, RoundingMode::Up, "1000000000000");
            test("7999999999999", 3, RoundingMode::Floor, "999999999999");
            test("7999999999999", 3, RoundingMode::Ceiling, "1000000000000");
            test("7999999999999", 3, RoundingMode::Nearest, "1000000000000");

            test("8000000000000", 3, RoundingMode::Down, "1000000000000");
            test("8000000000000", 3, RoundingMode::Up, "1000000000000");
            test("8000000000000", 3, RoundingMode::Floor, "1000000000000");
            test("8000000000000", 3, RoundingMode::Ceiling, "1000000000000");
            test("8000000000000", 3, RoundingMode::Nearest, "1000000000000");
            test("8000000000000", 3, RoundingMode::Exact, "1000000000000");

            test("8000000000001", 3, RoundingMode::Down, "1000000000000");
            test("8000000000001", 3, RoundingMode::Up, "1000000000001");
            test("8000000000001", 3, RoundingMode::Floor, "1000000000000");
            test("8000000000001", 3, RoundingMode::Ceiling, "1000000000001");
            test("8000000000001", 3, RoundingMode::Nearest, "1000000000000");

            test(
                "16777216000000000000",
                24,
                RoundingMode::Down,
                "1000000000000",
            );
            test(
                "16777216000000000000",
                24,
                RoundingMode::Up,
                "1000000000000",
            );
            test(
                "16777216000000000000",
                24,
                RoundingMode::Floor,
                "1000000000000",
            );
            test(
                "16777216000000000000",
                24,
                RoundingMode::Ceiling,
                "1000000000000",
            );
            test(
                "16777216000000000000",
                24,
                RoundingMode::Nearest,
                "1000000000000",
            );
            test(
                "16777216000000000000",
                24,
                RoundingMode::Exact,
                "1000000000000",
            );

            test(
                "33554432000000000000",
                25,
                RoundingMode::Down,
                "1000000000000",
            );
            test(
                "33554432000000000000",
                25,
                RoundingMode::Up,
                "1000000000000",
            );
            test(
                "33554432000000000000",
                25,
                RoundingMode::Floor,
                "1000000000000",
            );
            test(
                "33554432000000000000",
                25,
                RoundingMode::Ceiling,
                "1000000000000",
            );
            test(
                "33554432000000000000",
                25,
                RoundingMode::Nearest,
                "1000000000000",
            );
            test(
                "33554432000000000000",
                25,
                RoundingMode::Exact,
                "1000000000000",
            );

            test(
                "2147483648000000000000",
                31,
                RoundingMode::Down,
                "1000000000000",
            );
            test(
                "2147483648000000000000",
                31,
                RoundingMode::Up,
                "1000000000000",
            );
            test(
                "2147483648000000000000",
                31,
                RoundingMode::Floor,
                "1000000000000",
            );
            test(
                "2147483648000000000000",
                31,
                RoundingMode::Ceiling,
                "1000000000000",
            );
            test(
                "2147483648000000000000",
                31,
                RoundingMode::Nearest,
                "1000000000000",
            );
            test(
                "2147483648000000000000",
                31,
                RoundingMode::Exact,
                "1000000000000",
            );

            test(
                "4294967296000000000000",
                32,
                RoundingMode::Down,
                "1000000000000",
            );
            test(
                "4294967296000000000000",
                32,
                RoundingMode::Up,
                "1000000000000",
            );
            test(
                "4294967296000000000000",
                32,
                RoundingMode::Floor,
                "1000000000000",
            );
            test(
                "4294967296000000000000",
                32,
                RoundingMode::Ceiling,
                "1000000000000",
            );
            test(
                "4294967296000000000000",
                32,
                RoundingMode::Nearest,
                "1000000000000",
            );
            test(
                "4294967296000000000000",
                32,
                RoundingMode::Exact,
                "1000000000000",
            );

            test(
                "8589934592000000000000",
                33,
                RoundingMode::Down,
                "1000000000000",
            );
            test(
                "8589934592000000000000",
                33,
                RoundingMode::Up,
                "1000000000000",
            );
            test(
                "8589934592000000000000",
                33,
                RoundingMode::Floor,
                "1000000000000",
            );
            test(
                "8589934592000000000000",
                33,
                RoundingMode::Ceiling,
                "1000000000000",
            );
            test(
                "8589934592000000000000",
                33,
                RoundingMode::Nearest,
                "1000000000000",
            );
            test(
                "8589934592000000000000",
                33,
                RoundingMode::Exact,
                "1000000000000",
            );

            test(
                "1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Down,
                "1000000000000",
            );
            test(
                "1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Up,
                "1000000000000",
            );
            test(
                "1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Floor,
                "1000000000000",
            );
            test(
                "1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Ceiling,
                "1000000000000",
            );
            test(
                "1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Nearest,
                "1000000000000",
            );
            test(
                "1267650600228229401496703205376000000000000",
                100,
                RoundingMode::Exact,
                "1000000000000",
            );

            test("1000000000000", 10, RoundingMode::Down, "976562500");
            test("1000000000000", 10, RoundingMode::Up, "976562500");
            test("1000000000000", 10, RoundingMode::Floor, "976562500");
            test("1000000000000", 10, RoundingMode::Ceiling, "976562500");
            test("1000000000000", 10, RoundingMode::Nearest, "976562500");
            test("1000000000000", 10, RoundingMode::Exact, "976562500");

            test("980657949", 72, RoundingMode::Down, "0");
            test("980657949", 72, RoundingMode::Up, "1");
            test("980657949", 72, RoundingMode::Floor, "0");
            test("980657949", 72, RoundingMode::Ceiling, "1");
            test("980657949", 72, RoundingMode::Nearest, "0");

            test("4294967295", 31, RoundingMode::Down, "1");
            test("4294967295", 31, RoundingMode::Up, "2");
            test("4294967295", 31, RoundingMode::Floor, "1");
            test("4294967295", 31, RoundingMode::Ceiling, "2");
            test("4294967295", 31, RoundingMode::Nearest, "2");

            test("4294967295", 32, RoundingMode::Down, "0");
            test("4294967295", 32, RoundingMode::Up, "1");
            test("4294967295", 32, RoundingMode::Floor, "0");
            test("4294967295", 32, RoundingMode::Ceiling, "1");
            test("4294967295", 32, RoundingMode::Nearest, "1");

            test("4294967296", 32, RoundingMode::Down, "1");
            test("4294967296", 32, RoundingMode::Up, "1");
            test("4294967296", 32, RoundingMode::Floor, "1");
            test("4294967296", 32, RoundingMode::Ceiling, "1");
            test("4294967296", 32, RoundingMode::Nearest, "1");
            test("4294967296", 32, RoundingMode::Exact, "1");

            test("4294967296", 33, RoundingMode::Down, "0");
            test("4294967296", 33, RoundingMode::Up, "1");
            test("4294967296", 33, RoundingMode::Floor, "0");
            test("4294967296", 33, RoundingMode::Ceiling, "1");
            test("4294967296", 33, RoundingMode::Nearest, "0");

            test("0", -10, RoundingMode::Exact, "0");
            test("123", -1, RoundingMode::Exact, "246");
            test("123", -2, RoundingMode::Exact, "492");
            test("123", -25, RoundingMode::Exact, "4127195136");
            test("123", -26, RoundingMode::Exact, "8254390272");
            test(
                "123",
                -100,
                RoundingMode::Exact,
                "155921023828072216384094494261248",
            );
            test("2147483648", -1, RoundingMode::Exact, "4294967296");
            test("1000000000000", -3, RoundingMode::Exact, "8000000000000");
            test(
                "1000000000000",
                -24,
                RoundingMode::Exact,
                "16777216000000000000",
            );
            test(
                "1000000000000",
                -25,
                RoundingMode::Exact,
                "33554432000000000000",
            );
            test(
                "1000000000000",
                -31,
                RoundingMode::Exact,
                "2147483648000000000000",
            );
            test(
                "1000000000000",
                -32,
                RoundingMode::Exact,
                "4294967296000000000000",
            );
            test(
                "1000000000000",
                -33,
                RoundingMode::Exact,
                "8589934592000000000000",
            );
            test(
                "1000000000000",
                -100,
                RoundingMode::Exact,
                "1267650600228229401496703205376000000000000",
            );
        }

        #[test]
        #[should_panic]
        fn $shr_round_assign_i_fail_1() {
            Natural::from(123u32).shr_round_assign($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_assign_i_fail_2() {
            Natural::from(123u32).shr_round_assign($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_assign_i_fail_3() {
            Natural::from_str("1000000000001")
                .unwrap()
                .shr_round_assign($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_assign_i_fail_4() {
            Natural::from_str("1000000000001")
                .unwrap()
                .shr_round_assign($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_i_fail_1() {
            Natural::from(123u32).shr_round($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_i_fail_2() {
            Natural::from(123u32).shr_round($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_i_fail_3() {
            Natural::from_str("1000000000001")
                .unwrap()
                .shr_round($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_i_fail_4() {
            Natural::from_str("1000000000001")
                .unwrap()
                .shr_round($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_i_ref_fail_1() {
            (&Natural::from(123u32)).shr_round($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_i_ref_fail_2() {
            (&Natural::from(123u32)).shr_round($t::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_i_ref_fail_3() {
            (&Natural::from_str("1000000000001").unwrap()).shr_round($t::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_i_ref_fail_4() {
            (&Natural::from_str("1000000000001").unwrap())
                .shr_round($t::exact_from(100), RoundingMode::Exact);
        }
    };
}
tests_and_properties_signed!(
    i8,
    test_shr_round_i8,
    shr_round_assign_i8_fail_1,
    shr_round_assign_i8_fail_2,
    shr_round_assign_i8_fail_3,
    shr_round_assign_i8_fail_4,
    shr_round_i8_fail_1,
    shr_round_i8_fail_2,
    shr_round_i8_fail_3,
    shr_round_i8_fail_4,
    shr_round_i8_ref_fail_1,
    shr_round_i8_ref_fail_2,
    shr_round_i8_ref_fail_3,
    shr_round_i8_ref_fail_4
);
tests_and_properties_signed!(
    i16,
    test_shr_round_i16,
    shr_round_assign_i16_fail_1,
    shr_round_assign_i16_fail_2,
    shr_round_assign_i16_fail_3,
    shr_round_assign_i16_fail_4,
    shr_round_i16_fail_1,
    shr_round_i16_fail_2,
    shr_round_i16_fail_3,
    shr_round_i16_fail_4,
    shr_round_i16_ref_fail_1,
    shr_round_i16_ref_fail_2,
    shr_round_i16_ref_fail_3,
    shr_round_i16_ref_fail_4
);
tests_and_properties_signed!(
    i32,
    test_shr_round_i32,
    shr_round_assign_i32_fail_1,
    shr_round_assign_i32_fail_2,
    shr_round_assign_i32_fail_3,
    shr_round_assign_i32_fail_4,
    shr_round_i32_fail_1,
    shr_round_i32_fail_2,
    shr_round_i32_fail_3,
    shr_round_i32_fail_4,
    shr_round_i32_ref_fail_1,
    shr_round_i32_ref_fail_2,
    shr_round_i32_ref_fail_3,
    shr_round_i32_ref_fail_4
);
tests_and_properties_signed!(
    i64,
    test_shr_round_i64,
    shr_round_assign_i64_fail_1,
    shr_round_assign_i64_fail_2,
    shr_round_assign_i64_fail_3,
    shr_round_assign_i64_fail_4,
    shr_round_i64_fail_1,
    shr_round_i64_fail_2,
    shr_round_i64_fail_3,
    shr_round_i64_fail_4,
    shr_round_i64_ref_fail_1,
    shr_round_i64_ref_fail_2,
    shr_round_i64_ref_fail_3,
    shr_round_i64_ref_fail_4
);
tests_and_properties_signed!(
    i128,
    test_shr_round_i128,
    shr_round_assign_i128_fail_1,
    shr_round_assign_i128_fail_2,
    shr_round_assign_i128_fail_3,
    shr_round_assign_i128_fail_4,
    shr_round_i128_fail_1,
    shr_round_i128_fail_2,
    shr_round_i128_fail_3,
    shr_round_i128_fail_4,
    shr_round_i128_ref_fail_1,
    shr_round_i128_ref_fail_2,
    shr_round_i128_ref_fail_3,
    shr_round_i128_ref_fail_4
);
tests_and_properties_signed!(
    isize,
    test_shr_round_isize,
    shr_round_assign_isize_fail_1,
    shr_round_assign_isize_fail_2,
    shr_round_assign_isize_fail_3,
    shr_round_assign_isize_fail_4,
    shr_round_isize_fail_1,
    shr_round_isize_fail_2,
    shr_round_isize_fail_3,
    shr_round_isize_fail_4,
    shr_round_isize_ref_fail_1,
    shr_round_isize_ref_fail_2,
    shr_round_isize_ref_fail_3,
    shr_round_isize_ref_fail_4
);
