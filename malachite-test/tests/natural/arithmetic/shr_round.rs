use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShr, DivRound, DivisibleByPowerOfTwo, ShlRound, ShrRound, ShrRoundAssign,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_mode::RoundingMode;
use malachite_nz::natural::arithmetic::shr_round::{
    limbs_shr_exact, limbs_shr_round, limbs_shr_round_nearest, limbs_shr_round_up,
    limbs_vec_shr_exact_in_place, limbs_vec_shr_round_in_place,
    limbs_vec_shr_round_nearest_in_place, limbs_vec_shr_round_up_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_positive_unsigned_and_small_unsigned, pairs_of_signed_and_rounding_mode,
    pairs_of_unsigned_and_rounding_mode, pairs_of_unsigned_vec_and_small_unsigned,
    pairs_of_unsigned_vec_and_small_unsigned_var_1,
    triples_of_unsigned_small_signed_and_rounding_mode_var_1,
    triples_of_unsigned_small_unsigned_and_rounding_mode_var_1,
    triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_rounding_mode, pairs_of_natural_and_small_unsigned,
    pairs_of_natural_and_small_unsigned_var_2,
    triples_of_natural_small_signed_and_rounding_mode_var_2,
    triples_of_natural_small_unsigned_and_rounding_mode_var_1,
};

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
    test(&[123, 455], 1, &[2_147_483_710, 227]);
    test(&[123, 456], 31, &[913, 0]);
    test(&[123, 456], 32, &[457]);
    test(&[123, 456], 100, &[1]);
    test(&[256, 456], 8, &[3_355_443_201, 1]);
    test(&[4_294_967_295, 1], 1, &[0, 1]);
    test(&[4_294_967_295, 4_294_967_295], 32, &[0, 1]);
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
    test(&[123, 455], 1, &[2_147_483_710, 227]);
    test(&[123, 456], 31, &[912, 0]);
    test(&[123, 456], 32, &[456]);
    test(&[123, 456], 100, &[]);
    test(&[256, 456], 8, &[3_355_443_201, 1]);
    test(&[4_294_967_295, 1], 1, &[0, 1]);
    test(&[4_294_967_295, 4_294_967_295], 32, &[0, 1]);
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
    test(&[256, 456], 8, Some(vec![3_355_443_201, 1]));
    test(&[4_294_967_295, 1], 1, None);
    test(&[4_294_967_295, 4_294_967_295], 32, None);
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
        Some(vec![2_147_483_709, 227]),
    );
    test(&[123, 456], 31, RoundingMode::Ceiling, Some(vec![913, 0]));
    test(&[123, 456], 32, RoundingMode::Up, Some(vec![457]));
    test(&[123, 456], 100, RoundingMode::Down, Some(vec![]));
    test(
        &[256, 456],
        8,
        RoundingMode::Exact,
        Some(vec![3_355_443_201, 1]),
    );
    test(&[4_294_967_295, 1], 1, RoundingMode::Exact, None);
    test(
        &[4_294_967_295, 4_294_967_295],
        32,
        RoundingMode::Down,
        Some(vec![4_294_967_295]),
    );
}

#[test]
fn limbs_shr_round_up_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_1,
        |&(ref limbs, bits)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_shr_round_up(limbs, bits)),
                Natural::from_limbs_asc(limbs).shr_round(bits, RoundingMode::Up),
            );
        },
    );
}

#[test]
fn limbs_shr_round_nearest_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, bits)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_shr_round_nearest(limbs, bits)),
                Natural::from_limbs_asc(limbs).shr_round(bits, RoundingMode::Nearest),
            );
        },
    );
}

#[test]
fn limbs_shr_exact_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_1,
        |&(ref limbs, bits)| {
            let n = Natural::from_limbs_asc(limbs);
            if let Some(result_limbs) = limbs_shr_exact(limbs, bits) {
                let m = (&n).shr_round(bits, RoundingMode::Exact);
                assert_eq!(Natural::from_owned_limbs_asc(result_limbs), m);
                assert_eq!(m << bits, n);
            } else {
                assert!(!n.divisible_by_power_of_two(bits));
            }
        },
    );
}

#[test]
fn limbs_shr_round_properties() {
    test_properties(
        triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1,
        |&(ref limbs, bits, rm)| {
            let n = Natural::from_limbs_asc(limbs);
            if let Some(result_limbs) = limbs_shr_round(limbs, bits, rm) {
                let m = (&n).shr_round(bits, rm);
                assert_eq!(Natural::from_owned_limbs_asc(result_limbs), m);
                if rm == RoundingMode::Exact {
                    assert_eq!(m << bits, n);
                }
            } else {
                assert_eq!(rm, RoundingMode::Exact);
                assert!(!n.divisible_by_power_of_two(bits));
            }
        },
    );
}

#[test]
fn limbs_vec_shr_round_up_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_1,
        |&(ref limbs, bits)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_vec_shr_round_up_in_place(&mut limbs, bits);
            let n = Natural::from_limbs_asc(&old_limbs).shr_round(bits, RoundingMode::Up);
            assert_eq!(Natural::from_owned_limbs_asc(limbs), n);
        },
    );
}

#[test]
fn limbs_vec_shr_round_nearest_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, bits)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_vec_shr_round_nearest_in_place(&mut limbs, bits);
            let n = Natural::from_limbs_asc(&old_limbs).shr_round(bits, RoundingMode::Nearest);
            assert_eq!(Natural::from_owned_limbs_asc(limbs), n);
        },
    );
}

#[test]
fn limbs_vec_shr_exact_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_1,
        |&(ref limbs, bits)| {
            let n = Natural::from_limbs_asc(limbs);
            let mut limbs = limbs.to_vec();
            if limbs_vec_shr_exact_in_place(&mut limbs, bits) {
                let m = (&n).shr_round(bits, RoundingMode::Exact);
                assert_eq!(Natural::from_owned_limbs_asc(limbs), m);
                assert_eq!(m << bits, n);
            } else {
                assert!(!n.divisible_by_power_of_two(bits));
            }
        },
    );
}

#[test]
fn limbs_vec_shr_round_in_place_properties() {
    test_properties(
        triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1,
        |&(ref limbs, bits, rm)| {
            let n = Natural::from_limbs_asc(limbs);
            let mut limbs = limbs.to_vec();
            if limbs_vec_shr_round_in_place(&mut limbs, bits, rm) {
                let m = (&n).shr_round(bits, rm);
                assert_eq!(Natural::from_owned_limbs_asc(limbs), m);
                if rm == RoundingMode::Exact {
                    assert_eq!(m << bits, n);
                }
            } else {
                assert_eq!(rm, RoundingMode::Exact);
                assert!(!n.divisible_by_power_of_two(bits));
            }
        },
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
        $shr_round_u_ref_fail_4:ident,
        $shr_round_u_properties:ident
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

        #[test]
        fn $shr_round_u_properties() {
            test_properties(
                triples_of_natural_small_unsigned_and_rounding_mode_var_1::<$t>,
                |&(ref n, u, rm)| {
                    let mut mut_n = n.clone();
                    mut_n.shr_round_assign(u, rm);
                    assert!(mut_n.is_valid());
                    let shifted = mut_n;

                    let shifted_alt = n.shr_round(u, rm);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    let shifted_alt = n.clone().shr_round(u, rm);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    assert!(shifted <= *n);
                    assert_eq!(n.div_round(Natural::ONE << u, rm), shifted);
                },
            );

            test_properties(pairs_of_natural_and_small_unsigned::<$t>, |&(ref n, u)| {
                let left_shifted = n << u;
                assert_eq!((&left_shifted).shr_round(u, RoundingMode::Down), *n);
                assert_eq!((&left_shifted).shr_round(u, RoundingMode::Up), *n);
                assert_eq!((&left_shifted).shr_round(u, RoundingMode::Floor), *n);
                assert_eq!((&left_shifted).shr_round(u, RoundingMode::Ceiling), *n);
                assert_eq!((&left_shifted).shr_round(u, RoundingMode::Nearest), *n);
                assert_eq!((&left_shifted).shr_round(u, RoundingMode::Exact), *n);
            });

            // TODO test using Rationals
            test_properties(
                pairs_of_natural_and_small_unsigned_var_2::<$t>,
                |&(ref n, u)| {
                    let down = n.shr_round(u, RoundingMode::Down);
                    let up = &down + Natural::ONE;
                    assert_eq!(n.shr_round(u, RoundingMode::Up), up);
                    assert_eq!(n.shr_round(u, RoundingMode::Floor), down);
                    assert_eq!(n.shr_round(u, RoundingMode::Ceiling), up);
                    let nearest = n.shr_round(u, RoundingMode::Nearest);
                    assert!(nearest == down || nearest == up);
                },
            );

            test_properties(
                pairs_of_positive_unsigned_and_small_unsigned::<Limb, $t>,
                |&(u, v)| {
                    if let Some(shift) = v.checked_add($t::exact_from(Limb::WIDTH)) {
                        assert_eq!(Natural::from(u).shr_round(shift, RoundingMode::Down), 0);
                        assert_eq!(Natural::from(u).shr_round(shift, RoundingMode::Floor), 0);
                        assert_eq!(Natural::from(u).shr_round(shift, RoundingMode::Up), 1);
                        assert_eq!(Natural::from(u).shr_round(shift, RoundingMode::Ceiling), 1);
                        if let Some(extra_shift) = shift.checked_add(1) {
                            assert_eq!(
                                Natural::from(u).shr_round(extra_shift, RoundingMode::Nearest),
                                0
                            );
                        }
                    }
                },
            );

            #[allow(unknown_lints, identity_op)]
            test_properties(pairs_of_natural_and_rounding_mode, |&(ref n, rm)| {
                assert_eq!(n.shr_round($t::ZERO, rm), *n);
            });

            test_properties(pairs_of_unsigned_and_rounding_mode::<$t>, |&(u, rm)| {
                assert_eq!(Natural::ZERO.shr_round(u, rm), 0);
            });

            test_properties(
                triples_of_unsigned_small_unsigned_and_rounding_mode_var_1::<Limb, $t>,
                |&(n, u, rm)| {
                    assert_eq!(n.shr_round(u, rm), Natural::from(n).shr_round(u, rm));
                },
            );
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
    shr_round_u8_ref_fail_4,
    shr_round_u8_properties
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
    shr_round_u16_ref_fail_4,
    shr_round_u16_properties
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
    shr_round_u32_ref_fail_4,
    shr_round_u32_properties
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
    shr_round_u64_ref_fail_4,
    shr_round_u64_properties
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
    shr_round_usize_ref_fail_4,
    shr_round_usize_properties
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
        $shr_round_i_ref_fail_4:ident,
        $shr_round_i_properties:ident
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

        #[test]
        fn $shr_round_i_properties() {
            test_properties(
                triples_of_natural_small_signed_and_rounding_mode_var_2::<$t>,
                |&(ref n, i, rm)| {
                    let mut mut_n = n.clone();
                    mut_n.shr_round_assign(i, rm);
                    assert!(mut_n.is_valid());
                    let shifted = mut_n;

                    let shifted_alt = n.shr_round(i, rm);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    let shifted_alt = n.clone().shr_round(i, rm);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    if i != $t::MIN {
                        assert_eq!(n.shl_round(-i, rm), shifted);
                    }
                },
            );

            #[allow(unknown_lints, identity_op)]
            test_properties(pairs_of_natural_and_rounding_mode, |&(ref n, rm)| {
                assert_eq!(n.shr_round($t::ZERO, rm), *n);
            });

            test_properties(pairs_of_signed_and_rounding_mode::<$t>, |&(i, rm)| {
                assert_eq!(Natural::ZERO.shr_round(i, rm), 0);
            });

            test_properties(
                triples_of_unsigned_small_signed_and_rounding_mode_var_1::<Limb, $t>,
                |&(n, i, rm)| {
                    if n.arithmetic_checked_shr(i).is_some() {
                        assert_eq!(n.shr_round(i, rm), Natural::from(n).shr_round(i, rm));
                    }
                },
            );
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
    shr_round_i8_ref_fail_4,
    shr_round_i8_properties
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
    shr_round_i16_ref_fail_4,
    shr_round_i16_properties
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
    shr_round_i32_ref_fail_4,
    shr_round_i32_properties
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
    shr_round_i64_ref_fail_4,
    shr_round_i64_properties
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
    shr_round_isize_ref_fail_4,
    shr_round_isize_properties
);
