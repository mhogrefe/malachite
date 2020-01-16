use std::str::FromStr;

use malachite_base::comparison::Max;
use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::round::RoundingMode;
use malachite_nz::natural::arithmetic::shr_u::{
    limbs_shr, limbs_shr_exact, limbs_shr_round, limbs_shr_round_to_nearest, limbs_shr_round_up,
    limbs_shr_to_out, limbs_slice_shr_in_place, limbs_vec_shr_exact_in_place,
    limbs_vec_shr_in_place, limbs_vec_shr_round_in_place, limbs_vec_shr_round_to_nearest_in_place,
    limbs_vec_shr_round_up_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::BigUint;
use rug;

use malachite_test::common::test_properties;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::{
    pairs_of_positive_unsigned_and_small_unsigned, pairs_of_unsigned_and_rounding_mode,
    pairs_of_unsigned_and_small_unsigned, pairs_of_unsigned_vec_and_limb_var_2,
    pairs_of_unsigned_vec_and_small_unsigned, pairs_of_unsigned_vec_and_small_unsigned_var_1,
    triples_of_unsigned_small_unsigned_and_rounding_mode_var_1,
    triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_limb_var_6, unsigneds,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_rounding_mode, pairs_of_natural_and_small_unsigned,
    pairs_of_natural_and_small_unsigned_var_2,
    triples_of_natural_small_unsigned_and_rounding_mode_var_1,
    triples_of_natural_small_unsigned_and_small_unsigned,
};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_shr_and_limbs_vec_shr_in_place() {
    let test = |limbs: &[Limb], bits: u64, out: &[Limb]| {
        assert_eq!(limbs_shr(limbs, bits), out);

        let mut limbs = limbs.to_vec();
        limbs_vec_shr_in_place(&mut limbs, bits);
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
    test(&[3], 1, &[1]);
    test(&[122, 456], 1, &[61, 228]);
    test(&[123, 456], 0, &[123, 456]);
    test(&[123, 456], 1, &[61, 228]);
    test(&[123, 455], 1, &[2_147_483_709, 227]);
    test(&[123, 456], 31, &[912, 0]);
    test(&[123, 456], 32, &[456]);
    test(&[123, 456], 100, &[]);
    test(&[256, 456], 8, &[3_355_443_201, 1]);
    test(&[4_294_967_295, 1], 1, &[4_294_967_295, 0]);
    test(&[4_294_967_295, 4_294_967_295], 32, &[4_294_967_295]);
}

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
fn test_limbs_shr_round_to_nearest_and_limbs_vec_shr_round_to_nearest_in_place() {
    let test = |limbs: &[Limb], bits: u64, out: &[Limb]| {
        assert_eq!(limbs_shr_round_to_nearest(limbs, bits), out);

        let mut limbs = limbs.to_vec();
        limbs_vec_shr_round_to_nearest_in_place(&mut limbs, bits);
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

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_shr_to_out() {
    let test =
        |out_before: &[Limb], limbs_in: &[Limb], bits: Limb, carry: Limb, out_after: &[Limb]| {
            let mut out = out_before.to_vec();
            assert_eq!(limbs_shr_to_out(&mut out, limbs_in, bits), carry);
            assert_eq!(out, out_after);
        };
    test(&[10, 10, 10, 10], &[0, 0, 0], 1, 0, &[0, 0, 0, 10]);
    test(&[10, 10, 10, 10], &[1], 1, 2_147_483_648, &[0, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[3], 1, 2_147_483_648, &[1, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[122, 456], 1, 0, &[61, 228, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        1,
        2_147_483_648,
        &[61, 228, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 455],
        1,
        2_147_483_648,
        &[2_147_483_709, 227, 10, 10],
    );
    test(&[10, 10, 10, 10], &[123, 456], 31, 246, &[912, 0, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[256, 456],
        8,
        0,
        &[3_355_443_201, 1, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[4_294_967_295, 1],
        1,
        2_147_483_648,
        &[4_294_967_295, 0, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[4_294_967_295, 4_294_967_295],
        31,
        4_294_967_294,
        &[4_294_967_295, 1, 10, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shr_to_out_fail_1() {
    limbs_shr_to_out(&mut [10, 10], &[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shr_to_out_fail_2() {
    limbs_shr_to_out(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shr_to_out_fail_3() {
    limbs_shr_to_out(&mut [10, 10, 10], &[123, 456], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shr_to_out_fail_4() {
    limbs_shr_to_out(&mut [10, 10, 10], &[123, 456], 100);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_shr_in_place() {
    let test = |limbs: &[Limb], bits: Limb, carry: Limb, out: &[Limb]| {
        let mut limbs = limbs.to_vec();
        assert_eq!(limbs_slice_shr_in_place(&mut limbs, bits), carry);
        assert_eq!(limbs, out);
    };
    test(&[0, 0, 0], 1, 0, &[0, 0, 0]);
    test(&[1], 1, 2_147_483_648, &[0]);
    test(&[3], 1, 2_147_483_648, &[1]);
    test(&[122, 456], 1, 0, &[61, 228]);
    test(&[123, 456], 1, 2_147_483_648, &[61, 228]);
    test(&[123, 455], 1, 2_147_483_648, &[2_147_483_709, 227]);
    test(&[123, 456], 31, 246, &[912, 0]);
    test(&[256, 456], 8, 0, &[3_355_443_201, 1]);
    test(&[4_294_967_295, 1], 1, 2_147_483_648, &[4_294_967_295, 0]);
    test(
        &[4_294_967_295, 4_294_967_295],
        31,
        4_294_967_294,
        &[4_294_967_295, 1],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_shr_in_place_fail_1() {
    limbs_slice_shr_in_place(&mut [], 1);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_shr_in_place_fail_2() {
    limbs_slice_shr_in_place(&mut [123, 456], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_shr_in_place_fail_3() {
    limbs_slice_shr_in_place(&mut [123, 456], 100);
}

#[test]
fn limbs_shr_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, bits)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_shr(limbs, bits)),
                Natural::from_limbs_asc(limbs) >> bits
            );
        },
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
fn limbs_shr_round_to_nearest_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, bits)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_shr_round_to_nearest(limbs, bits)),
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
                assert_ne!(&n >> bits << bits, n);
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
                assert_ne!(&n >> bits << bits, n);
            }
        },
    );
}

#[test]
fn limbs_shr_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_limb_var_6,
        |&(ref out, ref in_limbs, bits)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            let carry = limbs_shr_to_out(&mut out, in_limbs, bits);
            let n = Natural::from_limbs_asc(in_limbs);
            let m = &n >> bits;
            assert_eq!(carry == 0, &m << bits == n);
            let len = in_limbs.len();
            let mut limbs = m.into_limbs_asc();
            limbs.resize(len, 0);
            let actual_limbs = out[..len].to_vec();
            assert_eq!(limbs, actual_limbs);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_slice_shr_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_limb_var_2,
        |&(ref limbs, bits)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            let carry = limbs_slice_shr_in_place(&mut limbs, bits);
            let n = Natural::from_limbs_asc(&old_limbs);
            let m = &n >> bits;
            assert_eq!(carry == 0, &m << bits == n);
            let mut expected_limbs = m.into_limbs_asc();
            expected_limbs.resize(limbs.len(), 0);
            assert_eq!(limbs, expected_limbs);
        },
    );
}

#[test]
fn limbs_vec_shr_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, bits)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_vec_shr_in_place(&mut limbs, bits);
            let n = Natural::from_limbs_asc(&old_limbs) >> bits;
            assert_eq!(Natural::from_owned_limbs_asc(limbs), n);
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
fn limbs_vec_shr_round_to_nearest_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, bits)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_vec_shr_round_to_nearest_in_place(&mut limbs, bits);
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
                assert_ne!(&n >> bits << bits, n);
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
                assert_ne!(&n >> bits << bits, n);
            }
        },
    );
}

macro_rules! tests_and_properties {
    (
        $t:ident,
        $test_shr_u:ident,
        $shr_u_properties:ident,
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
        $shr_round_u_properties:ident,
        $u:ident,
        $v:ident,
        $out:ident,
        $shl_library_comparison_tests:expr,
        $n:ident,
        $shifted:ident,
        $shl_library_comparison_properties:expr
    ) => {
        #[test]
        fn $test_shr_u() {
            let test = |$u, $v: $t, $out| {
                let mut n = Natural::from_str($u).unwrap();
                n >>= $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = Natural::from_str($u).unwrap() >> $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = &Natural::from_str($u).unwrap() >> $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                $shl_library_comparison_tests
            };
            test("0", 0, "0");
            test("0", 10, "0");
            test("123", 0, "123");
            test("245", 1, "122");
            test("246", 1, "123");
            test("247", 1, "123");
            test("491", 2, "122");
            test("492", 2, "123");
            test("493", 2, "123");
            test("4127195135", 25, "122");
            test("4127195136", 25, "123");
            test("4127195137", 25, "123");
            test("8254390271", 26, "122");
            test("8254390272", 26, "123");
            test("8254390273", 26, "123");
            test("155921023828072216384094494261247", 100, "122");
            test("155921023828072216384094494261248", 100, "123");
            test("155921023828072216384094494261249", 100, "123");
            test("4294967295", 1, "2147483647");
            test("4294967296", 1, "2147483648");
            test("4294967297", 1, "2147483648");
            test("1000000000000", 0, "1000000000000");
            test("7999999999999", 3, "999999999999");
            test("8000000000000", 3, "1000000000000");
            test("8000000000001", 3, "1000000000000");
            test("16777216000000000000", 24, "1000000000000");
            test("33554432000000000000", 25, "1000000000000");
            test("2147483648000000000000", 31, "1000000000000");
            test("4294967296000000000000", 32, "1000000000000");
            test("8589934592000000000000", 33, "1000000000000");
            test(
                "1267650600228229401496703205376000000000000",
                100,
                "1000000000000",
            );
            test("1000000000000", 10, "976562500");
            test("980657949", 72, "0");
            test("4294967295", 31, "1");
            test("4294967295", 32, "0");
            test("4294967296", 32, "1");
            test("4294967296", 33, "0");
        }

        #[test]
        fn $shr_u_properties() {
            test_properties(
                pairs_of_natural_and_small_unsigned::<$t>,
                |&(ref $n, $u)| {
                    let mut mut_n = $n.clone();
                    mut_n >>= $u;
                    assert!(mut_n.is_valid());
                    let $shifted = mut_n;

                    let shifted_alt = $n >> $u;
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, $shifted);

                    let shifted_alt = $n.clone() >> $u;
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, $shifted);

                    assert!($shifted <= *$n);
                    assert_eq!($n.shr_round($u, RoundingMode::Floor), $shifted);

                    $shl_library_comparison_properties

                    if $u < $t::wrapping_from(<$t as PrimitiveUnsigned>::SignedOfEqualWidth::MAX) {
                        let u = <$t as PrimitiveUnsigned>::SignedOfEqualWidth::wrapping_from($u);
                        assert_eq!($n >> u, $shifted);
                        assert_eq!($n << -u, $shifted);
                    }
                },
            );

            test_properties(
                pairs_of_unsigned_and_small_unsigned::<Limb, $t>,
                |&(u, v)| {
                    if let Some(shift) = v.checked_add($t::exact_from(Limb::WIDTH)) {
                        assert_eq!(Natural::from(u) >> shift, 0);
                    }

                    if v < $t::exact_from(Limb::WIDTH) {
                        assert_eq!(u >> v, Natural::from(u) >> v);
                    }
                },
            );

            test_properties(
                triples_of_natural_small_unsigned_and_small_unsigned::<$t>,
                |&(ref n, u, v)| {
                    if let Some(sum) = u.checked_add(v) {
                        assert_eq!(n >> u >> v, n >> sum);
                    }
                },
            );

            #[allow(unknown_lints, identity_op)]
            test_properties(naturals, |n| {
                assert_eq!(n >> $t::ZERO, *n);
            });

            test_properties(unsigneds::<$t>, |&u| {
                assert_eq!(Natural::ZERO >> u, 0);
            });
        }

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
                        assert_eq!(Natural::from(u).shr_round(shift, RoundingMode::Down),
                            0);
                        assert_eq!(Natural::from(u).shr_round(shift, RoundingMode::Floor),
                            0);
                        assert_eq!(Natural::from(u).shr_round(shift, RoundingMode::Up), 1);
                        assert_eq!(Natural::from(u).shr_round(shift, RoundingMode::Ceiling),
                            1);
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
tests_and_properties!(
    u8,
    test_shr_u8,
    shr_u8_properties,
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
    shr_round_u8_properties,
    u,
    v,
    out,
    {},
    n,
    shifted,
    {}
);
tests_and_properties!(
    u16,
    test_shr_u16,
    shr_u16_properties,
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
    shr_round_u16_properties,
    u,
    v,
    out,
    {},
    n,
    shifted,
    {}
);
tests_and_properties!(
    u32,
    test_shr_limb,
    shr_limb_properties,
    test_shr_round_limb,
    shr_round_assign_limb_fail_1,
    shr_round_assign_limb_fail_2,
    shr_round_assign_limb_fail_3,
    shr_round_assign_limb_fail_4,
    shr_round_limb_fail_1,
    shr_round_limb_fail_2,
    shr_round_limb_fail_3,
    shr_round_limb_fail_4,
    shr_round_limb_ref_fail_1,
    shr_round_limb_ref_fail_2,
    shr_round_limb_ref_fail_3,
    shr_round_limb_ref_fail_4,
    shr_round_limb_properties,
    u,
    v,
    out,
    {
        let mut n = rug::Integer::from_str(u).unwrap();
        n >>= v;
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() >> v;
        assert_eq!(n.to_string(), out);

        let n = BigUint::from_str(u).unwrap() >> usize::exact_from(v);
        assert_eq!(n.to_string(), out);

        let n = &BigUint::from_str(u).unwrap() >> usize::exact_from(v);
        assert_eq!(n.to_string(), out);
    },
    n,
    shifted,
    {
        let mut rug_n = natural_to_rug_integer(n);
        rug_n >>= u;
        assert_eq!(rug_integer_to_natural(&rug_n), shifted);

        assert_eq!(
            biguint_to_natural(&(&natural_to_biguint(n) >> usize::exact_from(u))),
            shifted
        );
        assert_eq!(
            biguint_to_natural(&(natural_to_biguint(n) >> usize::exact_from(u))),
            shifted
        );
        assert_eq!(
            rug_integer_to_natural(&(natural_to_rug_integer(n) >> u)),
            shifted
        );
    }
);
tests_and_properties!(
    u64,
    test_shr_u64,
    shr_u64_properties,
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
    shr_round_u64_properties,
    u,
    v,
    out,
    {},
    n,
    shifted,
    {}
);
tests_and_properties!(
    usize,
    test_shr_usize,
    shr_usize_properties,
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
    shr_round_usize_properties,
    u,
    v,
    out,
    {},
    n,
    shifted,
    {}
);
